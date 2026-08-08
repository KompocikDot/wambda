use std::sync::Arc;

use anyhow::Context;
use axum::{Json, Router, extract::State, routing::get};
use sea_orm::{ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait};

use crate::{
    errors::AppError,
    exctractors::AuthUser,
    models::{function, run, user},
    schemas::run::{RunResponse},
    state::AppState,
};

pub fn runs_router() -> Router<Arc<AppState>> {
    return Router::new().route("/", get(list_runs));
}

#[axum::debug_handler]
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
