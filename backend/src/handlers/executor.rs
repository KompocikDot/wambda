use std::{collections::HashMap, sync::Arc, time::Duration};

use anyhow::{Context, Result};
use axum::{
    Router,
    body::{Body, to_bytes},
    extract::{Path, Request, State},
    http::header::{AUTHORIZATION, COOKIE},
    response::Response,
    routing::any,
};
use chrono::Utc;
use log::info;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, JoinType, QueryFilter, QuerySelect,
    RelationTrait, value::ChronoUnixTimestampMillis,
};
use tokio::fs::{self, File};
use uuid::Uuid;

use crate::{
    errors::AppError,
    exctractors::ExtractApiKey,
    executor::{ExecutionParams, execute_wasm},
    models::{api_key, function, run},
    schemas::functions::unstringify_envs,
    state::AppState,
};

pub fn executor_router() -> Router<Arc<AppState>> {
    return Router::new().route("/{id}/{*path}", any(execute));
}

async fn execute(
    State(state): State<Arc<AppState>>,
    Path((id, _end_path)): Path<(Uuid, String)>,
    ExtractApiKey(x_api_key): ExtractApiKey,
    req: Request,
) -> Result<Response, AppError> {
    info!("starting function execution {id}");

    let trigger_time = chrono::Utc::now();
    let func: Option<(i32, i32, i32, String)> = function::Entity::find_by_uuid(id)
        .join(JoinType::InnerJoin, function::Relation::User.def())
        .join(JoinType::InnerJoin, api_key::Relation::User.def().rev())
        .filter(api_key::Column::Key.eq(x_api_key))
        .select_only()
        .column(function::Column::Id)
        .column(function::Column::MemoryLimit)
        .column(function::Column::Timeout)
        .column(function::Column::Envs)
        .into_tuple()
        .one(&state.db)
        .await
        .context("could not create query to find api_key matching function")?;

    let (func_id, memory_limit_mb, timeout_s, envs_stringified) = match func {
        Some(func) => func,
        None => return Err(AppError::NotFound),
    };
    let envs: HashMap<String, String> = unstringify_envs(&envs_stringified)
        .into_iter()
        .map(|env| (env.key, env.value))
        .collect();
    let memory_bytes = memory_limit_mb.max(1) as usize * 1024 * 1024;
    let timeout = Duration::from_secs(timeout_s.max(1) as u64);

    info!("function with {id} exists");

    let run_id = Uuid::now_v7();

    let run_dir = format!("/tmp/{id}/{run_id}");
    let workdir_filepath = format!("{run_dir}/data");
    let stdout_filepath = format!("{run_dir}:stdout");
    let stderr_filepath = format!("{run_dir}:stderr");

    fs::create_dir_all(&workdir_filepath).await.context("could not create workdir")?;
    
    let mut stdout = File::options()
        .create(true)
        .read(true)
        .write(true)
        .open(&stdout_filepath)
        .await
        .context("could not create wasi stdout")?;
    info!("created stdout for function {id}");

    let mut stderr = File::options()
        .create(true)
        .read(true)
        .write(true)
        .open(&stderr_filepath)
        .await
        .context("could not create wasi stderr")?;
    info!("created stderr for function {id}");

    let (mut req_parts, req_body) = req.into_parts();

    let guest_path = req_parts.uri.path().splitn(3, '/').nth(2).unwrap_or_default();
    let guest_uri = match req_parts.uri.query() {
        Some(query) => format!("/{guest_path}?{query}"),
        None => format!("/{guest_path}"),
    };
    req_parts.uri = guest_uri.parse().context("could not build guest uri")?;

    req_parts.headers.remove(AUTHORIZATION);
    req_parts.headers.remove(COOKIE);

    let payload_bytes = to_bytes(req_body, usize::MAX)
        .await
        .context("could not read request payload")?;

    state
        .io_executions_bucket
        .put_object(format!("/{id}/{run_id}:payload"), &payload_bytes)
        .await
        .context("could not write payload to bucket")?;

    info!("uploaded function {id} payload into bucket");

    let req = Request::from_parts(req_parts, Body::from(payload_bytes));

    let binary = state.get_binary(id).await?;

    info!("got binary for function {id}");

    let start_time = Utc::now();

    info!("starting function {id} execution");
    let result = execute_wasm(
        state.wasmtime_engine.clone(),
        req,
        &binary,
        ExecutionParams::new(
            stdout_filepath.clone(),
            stderr_filepath.clone(),
            workdir_filepath.clone(),
            envs,
            memory_bytes,
            timeout,
        ),
    )
    .await;

    info!("finished execution of function {id}");

    fs::remove_dir_all(&run_dir)
        .await
        .context("could not remove run directory")?;

    let new_run = run::ActiveModel {
        triggered_at: Set(ChronoUnixTimestampMillis(trigger_time)),
        finish_time: Set(ChronoUnixTimestampMillis(Utc::now())),
        started_at: Set(ChronoUnixTimestampMillis(start_time)),
        uuid: Set(run_id),
        status: Set(if result.is_ok() {
            run::Status::Success
        } else {
            run::Status::Failure
        }),
        function_id: Set(func_id),
        ..Default::default()
    };

    new_run
        .insert(&state.db)
        .await
        .context("could not insert run details into db")?;

    info!("inserted function {id} execution details into db");

    state
        .io_executions_bucket
        .put_object_stream(&mut stdout, format!("/{id}/{run_id}:stdout"))
        .await
        .context("could not write stdout to bucket")?;
    info!("saved function {id} stdout into bucket");
    state
        .io_executions_bucket
        .put_object_stream(&mut stderr, format!("/{id}/{run_id}:stderr"))
        .await
        .context("could not write stderr to bucket")?;
    info!("saved function {id} stderr into bucket");

    fs::remove_file(stdout_filepath)
        .await
        .context("could not remove wasi stdout temp file")?;
    fs::remove_file(stderr_filepath)
        .await
        .context("could not remove wasi stderr temp file")?;

    info!("removed function {id} tmp files");

    let resp = result?;

    let (parts, body) = resp.into_parts();
    let body_bytes = to_bytes(body, usize::MAX)
        .await
        .context("could not read response body")?;

    state
        .io_executions_bucket
        .put_object(format!("/{id}/{run_id}:response"), &body_bytes)
        .await
        .context("could not write response to bucket")?;

    info!("saved response of function {id}");

    Ok(Response::from_parts(parts, Body::from(body_bytes)))
}
