use std::sync::Arc;

use anyhow::Context;
use axum::{
    Json, Router,
    body::Body,
    extract::{Path, State},
    http::{HeaderValue, header},
    response::{IntoResponse, Response},
    routing::get,
};
use sea_orm::{
    ColumnTrait, EntityTrait, ExprTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
    sea_query::Expr,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    errors::AppError,
    exctractors::AuthUser,
    models::{function, run, user},
    schemas::run::{RunDetailsResponse, RunResponse},
    state::AppState,
};

pub fn runs_router() -> Router<Arc<AppState>> {
    return Router::new()
        .route("/", get(list_runs))
        .route("/{uuid}/details", get(get_run_details))
        .route("/{uuid}/files/{file}", get(get_run_file_content));
}

async fn list_runs(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
) -> Result<Json<Vec<RunResponse>>, AppError> {
    let runs = run::Entity::find()
        .find_also_related(function::Entity)
        .join(JoinType::InnerJoin, function::Relation::User.def())
        .filter(user::Column::Id.eq(user.id))
        .all(&state.db)
        .await
        .context("could not fetch runs for user")?;

    Ok(Json(runs.into_iter().map(Into::into).collect()))
}

async fn get_run_details(
    State(state): State<Arc<AppState>>,
    _user: AuthUser,
    Path(run_id): Path<Uuid>,
) -> Result<Json<RunDetailsResponse>, AppError> {
    let duration: Option<u32> = run::Entity::find_by_uuid(run_id)
        .expr_as(
            Expr::col(run::Column::FinishTime).sub(Expr::col(run::Column::TriggeredAt)),
            "duration",
        )
        .into_tuple()
        .one(&state.db)
        .await
        .context("could not get run details")?;

    match duration {
        Some(val) => Ok(Json(RunDetailsResponse { duration: val })),
        None => Err(AppError::NotFound),
    }
}

async fn get_run_file_content(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
    Path((run_id, file)): Path<(Uuid, String)>,
) -> Result<Response, AppError> {
    let allowed_filetypes = vec!["stdout", "stderr", "payload", "response"];
    if !allowed_filetypes.contains(&file.as_str()) {
        return Err(AppError::InvalidRequest(
            json!({"description": "invalid filetype, allowed: ['stdout', 'stderr', 'payload', 'response']"}),
        ));
    }

    let function_uuid: Option<uuid::Uuid> = run::Entity::find_by_uuid(run_id)
        .join(JoinType::InnerJoin, run::Relation::Function.def())
        .join(JoinType::InnerJoin, function::Relation::User.def())
        .filter(user::Column::Id.eq(user.id))
        .select_only()
        .column(function::Column::Uuid)
        .into_tuple()
        .one(&state.db)
        .await
        .context("could not fetch run by uuid to check user ownership")?;

    let uuid = match function_uuid {
        Some(v) => v,
        None => return Err(AppError::Unauthorized),
    };

    let dupa = state
        .io_executions_bucket
        .get_object_stream(format!("/{uuid}/{run_id}:{file}"))
        .await
        .context("could not get run file content")?;

    let body = Body::from_stream(dupa.bytes);

    Ok((
        [(header::CONTENT_TYPE, HeaderValue::from_static("text/plain"))],
        body,
    )
        .into_response())
}
