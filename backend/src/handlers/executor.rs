use std::{collections::HashMap, sync::Arc};

use anyhow::{Context, Result};
use axum::{
    Router,
    extract::{Path, Request, State},
    response::Response,
    routing::any,
};
use sea_orm::{ColumnTrait, JoinType, QueryFilter, QuerySelect, RelationTrait, SelectExt};
use tokio::fs::{self, File};
use uuid::Uuid;

use crate::{
    errors::AppError,
    exctractors::ExtractApiKey,
    executor::{ExecutionParams, execute_wasm},
    models::{api_key, function},
    state::AppState,
};

pub fn executor_router() -> Router<Arc<AppState>> {
    return Router::new().route("/{id}/{*path}", any(execute));
}

async fn execute(
    State(state): State<Arc<AppState>>,
    Path((id, end_path)): Path<(Uuid, String)>,
    ExtractApiKey(x_api_key): ExtractApiKey,
    req: Request,
) -> Result<Response, AppError> {
    if !function::Entity::find_by_uuid(id)
        .join(JoinType::InnerJoin, function::Relation::User.def())
        .join(JoinType::InnerJoin, api_key::Relation::User.def().rev())
        .filter(api_key::Column::Key.eq(x_api_key))
        .exists(&state.db)
        .await
        .context("could not create query to find api_key matching function")?
    {
        return Err(AppError::NotFound);
    }

    let run_id = Uuid::now_v7();
    let path = format!("/tmp/{id}");

    if !fs::try_exists(&path)
        .await
        .context("could not check if tmp path exists")?
    {
        fs::create_dir(path)
            .await
            .context("could not create tmp directory for run")?;
    }

    let stdout_filepath = format!("/tmp/{id}/{run_id}:stdout");
    let stderr_filepath = format!("/tmp/{id}/{run_id}:stderr");

    let mut stdout = File::options()
        .create(true)
        .read(true)
        .write(true)
        .open(&stdout_filepath)
        .await
        .context("could not create wasi stdout")?;

    let mut stderr = File::options()
        .create(true)
        .read(true)
        .write(true)
        .open(&stderr_filepath)
        .await
        .context("could not create wasi stderr")?;

    // TODO: Fix that path to get corrrect binary based on payload
    let binary = state
        .blob_bucket
        .get_object(id.to_string())
        .await
        .context("could not read wasm binary from bucket")?;

    let resp = execute_wasm(
        state.wasmtime_engine.clone(),
        req,
        binary.as_slice(),
        ExecutionParams::new(
            stdout_filepath.clone(),
            stderr_filepath.clone(),
            HashMap::new(),
        ),
    )
    .await?;

    state
        .io_executions_bucket
        .put_object_stream(&mut stdout, format!("/{id}/{run_id}:stdout"))
        .await
        .context("could not write stdout to bucket")?;
    state
        .io_executions_bucket
        .put_object_stream(&mut stderr, format!("/{id}/{run_id}:stderr"))
        .await
        .context("could not write stderr to bucket")?;

    fs::remove_file(stdout_filepath)
        .await
        .context("could not remove wasi stdout temp file")?;
    fs::remove_file(stderr_filepath)
        .await
        .context("could not remove wasi stderr temp file")?;

    Ok(resp)
}
