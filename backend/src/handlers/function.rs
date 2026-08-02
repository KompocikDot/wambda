use std::sync::Arc;

use anyhow::{Context, Result, anyhow};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware::from_extractor,
    routing::{get, post},
};
use axum_typed_multipart::TypedMultipart;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, QueryFilter, QuerySelect};
use uuid::Uuid;
use wasmtime::{Config, Engine};

use crate::{
    errors::AppError,
    exctractors::AuthUser,
    models::{
        function::{self, Entity as Function, MainPageFunctionQuery},
        run,
    },
    schemas::functions::{
        CreateFunctionPayload, CreateFunctionResponse, MainPageFunctionsResponse,
        SingleFunctionResponse,
    },
    state::AppState,
};

pub fn functions_router() -> Router<Arc<AppState>> {
    return Router::new()
        .route("/", get(list_functions))
        .route("/", post(create_function))
        .route("/{function_id}", get(get_function).delete(delete_function))
        .layer(from_extractor::<AuthUser>());
}

async fn precopmile_wasm(engine: Arc<Engine>, bytes: &[u8]) -> Result<Vec<u8>, AppError> {
    let compiled_module = engine
        .precompile_module(bytes)
        .map_err(|e| AppError::Internal(anyhow!("could not precompile wasm: {e}")))?;

    Ok(compiled_module)
}

async fn create_function(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
    payload: TypedMultipart<CreateFunctionPayload>,
) -> Result<(StatusCode, Json<CreateFunctionResponse>), AppError> {
    let uuid = Uuid::now_v7();

    let new_fn = function::ActiveModel {
        name: Set(payload.name.clone()),
        uuid: Set(uuid),
        language: Set(payload.language.clone().into()),
        trigger: Set(payload.trigger.clone().into()),
        memory_limit: Set(payload.memory_limit),
        timeout: Set(payload.timeout),
        envs: Set(payload.envs_stringified.clone()),
        owner_id: Set(user.id),
        ..Default::default()
    };

    new_fn
        .insert(&state.db)
        .await
        .context("could not insert function in the db")?;

    let wasm = precopmile_wasm(state.wasmtime_engine.clone(), &payload.wasm.contents).await?;

    state
        .blob_bucket
        .put_object(uuid.to_string(), wasm.as_slice())
        .await
        .context("could not write wasm to bucket")?;

    Ok((StatusCode::CREATED, Json(CreateFunctionResponse { uuid })))
}

async fn get_function(
    State(state): State<Arc<AppState>>,
    Path(function_id): Path<Uuid>,
    user: AuthUser,
) -> Result<(StatusCode, Json<SingleFunctionResponse>), AppError> {
    let function = Function::find_by_uuid(function_id)
        .filter(function::COLUMN.owner_id.eq(user.id))
        .one(&state.db)
        .await
        .context("could not get function")?;

    match function {
        Some(f) => Ok((StatusCode::OK, Json(f.into()))),
        None => Err(AppError::NotFound),
    }
}

async fn list_functions(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
) -> Result<(StatusCode, Json<MainPageFunctionsResponse>), AppError> {
    let functions = Function::find()
        .filter(function::COLUMN.owner_id.eq(user.id))
        .left_join(run::Entity)
        .group_by(function::Column::Id)
        .into_partial_model::<MainPageFunctionQuery>()
        .all(&state.db)
        .await
        .context("could not retrieve functions")?;

    Ok((StatusCode::OK, Json(functions.into())))
}
