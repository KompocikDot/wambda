use std::sync::Arc;

use anyhow::{Context, Result};
use axum::{
    Json, Router, extract::State, http::StatusCode, middleware::from_extractor, routing::get,
};
use axum_typed_multipart::TypedMultipart;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{
    errors::AppError,
    exctractors::AuthUser,
    models::function::{self, Entity as Function},
    schemas::functions::{CreateFunctionPayload, CreateFunctionResponse, ListFunctionsResponse},
    state::AppState,
};

pub fn functions_router() -> Router<Arc<AppState>> {
    return Router::new()
        .route("/", get(list_functions).post(create_function))
        .layer(from_extractor::<AuthUser>());
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

    state
        .blob_bucket
        .put_object(uuid.to_string(), &payload.wasm.contents)
        .await
        .context("could not write wasm to bucket")?;

    Ok((StatusCode::CREATED, Json(CreateFunctionResponse { uuid })))
}

#[axum::debug_handler]
async fn list_functions(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
) -> Result<(StatusCode, Json<ListFunctionsResponse>), AppError> {
    let functions = Function::find()
        .filter(function::COLUMN.owner_id.eq(user.id))
        .all(&state.db)
        .await
        .context("could not retrieve functions")?;

    Ok((StatusCode::OK, Json(functions.into())))
}
