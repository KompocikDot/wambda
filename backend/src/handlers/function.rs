use std::sync::Arc;

use anyhow::{Context, Result, anyhow};
use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Path, State},
    http::StatusCode,
    middleware::from_extractor,
    routing::{get, post},
};
use axum_typed_multipart::TypedMultipart;
use s3::serde_types::ObjectIdentifier;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, QueryFilter, QuerySelect};
use tokio::task;
use uuid::Uuid;
use wasmtime::Engine;

use crate::{
    errors::AppError,
    exctractors::AuthUser,
    models::{
        function::{self, Entity as Function, MainPageFunctionQuery},
        run,
    },
    schemas::functions::{
        CreateFunctionPayload, CreateFunctionResponse, MainPageFunctionsResponse,
        SingleFunctionResponse, UpdateFunctionPayload,
    },
    state::AppState,
};

pub fn functions_router() -> Router<Arc<AppState>> {
    return Router::new()
        .route("/", post(create_function))
        .route_layer(DefaultBodyLimit::max(1024 * 1024 * 512)) // 512MB limit for routes above it
        .route("/", get(list_functions))
        .route(
            "/{function_id}",
            get(get_function)
                .delete(delete_function)
                .patch(update_function),
        )
        .layer(from_extractor::<AuthUser>());
}

async fn precopmile_wasm(engine: Arc<Engine>, bytes: &[u8]) -> Result<Vec<u8>, AppError> {
    let bytes = bytes.to_vec();
    let compiled_module = task::spawn_blocking(move || {
        // Generates native ELF machine code bytes (.cwasm)
        engine.precompile_component(&bytes)
    })
    .await
    .context("could not compile wasm")?
    .map_err(|e| AppError::Internal(anyhow!("could not precompile wasm: {e}")))?;

    Ok(compiled_module)
}

async fn create_function(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
    payload: TypedMultipart<CreateFunctionPayload>,
) -> Result<(StatusCode, Json<CreateFunctionResponse>), AppError> {
    let uuid = Uuid::now_v7();

    let wasm_hash = sha256::digest(payload.wasm.contents.as_ref());

    let new_fn = function::ActiveModel {
        name: Set(payload.name.clone()),
        uuid: Set(uuid),
        language: Set(payload.language.clone().into()),
        trigger: Set(payload.trigger.clone().into()),
        memory_limit: Set(payload.memory_limit),
        timeout: Set(payload.timeout),
        envs: Set(payload.envs_stringified.clone()),
        owner_id: Set(user.id),
        wasm_hash: Set(wasm_hash),
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

async fn delete_function(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
    Path(function_id): Path<Uuid>,
) -> Result<(), AppError> {
    let function_instance: Option<function::Model> = Function::find()
        .filter(Function::COLUMN.owner_id.eq(user.id))
        .filter(Function::COLUMN.uuid.eq(function_id))
        .one(&state.db)
        .await
        .context("could not fetch function to delete it")?;

    let function_id = match function_instance {
        Some(func) => func.id,
        None => return Err(AppError::NotFound),
    };

    let run_ids: Vec<i32> = run::Entity::find()
        .filter(run::COLUMN.function_id.eq(function_id))
        .select_only()
        .column(run::Column::Id)
        .into_tuple()
        .all(&state.db)
        .await
        .context("could not retrieve run ids to later remove them")?;

    for chunk in run_ids.chunks(500) {
        let delete_objects = chunk
            .iter()
            .flat_map(|id| {
                [
                    ObjectIdentifier::new(format!("/{function_id}/{id}:stdout")),
                    ObjectIdentifier::new(format!("/{function_id}/{id}:stderr")),
                ]
            })
            .collect::<Vec<ObjectIdentifier>>();

        state
            .io_executions_bucket
            .delete_objects(delete_objects)
            .await
            .context("could not delete io-execution buckets")?;
    }

    state
        .blob_bucket
        .delete_object(function_id.to_string())
        .await
        .context("could not delete function binary from s3")?;

    Function::delete_by_id(function_id)
        .exec(&state.db)
        .await
        .context("could not delete function from db")?;

    Ok(())
}

async fn update_function(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
    Path(function_id): Path<Uuid>,
    payload: TypedMultipart<CreateFunctionPayload>,
) -> Result<StatusCode, AppError> {
    let fn_to_update: Option<i32> = function::Entity::find_by_uuid(function_id)
        .select_only()
        .column(function::Column::Id)
        .into_tuple()
        .one(&state.db)
        .await
        .context("could not get function to update")?;

    let fn_owner_id = match fn_to_update {
        Some(id) => id,
        None => return Err(AppError::NotFound),
    };

    if fn_owner_id != user.id {
        return Err(AppError::Unauthorized);
    }

    let update_function = function::ActiveModel {
        name: Set(payload.name.clone()),
        language: Set(payload.language.clone().into()),
        trigger: Set(payload.trigger.clone().into()),
        memory_limit: Set(payload.memory_limit),
        timeout: Set(payload.timeout),
        envs: Set(payload.envs_stringified.clone()),
        ..Default::default()
    };

    update_function
        .update(&state.db)
        .await
        .context("could not update function")?;

    Ok(StatusCode::OK)
}
