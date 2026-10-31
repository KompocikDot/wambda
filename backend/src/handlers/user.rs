use std::sync::Arc;

use anyhow::{Context, Result};
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{delete, patch},
};
use sea_orm::{ActiveValue::Set, EntityTrait, QueryFilter, QuerySelect};
use tower_sessions::Session;
use uuid::Uuid;

use crate::{
    errors::AppError,
    exctractors::AuthUser,
    handlers::function::purge_function_objects,
    models::{
        function, run,
        user::{self, Entity as UserModel},
    },
    schemas::user::ChangeAccountDetailsPayload,
    state::AppState,
};

pub fn user_router() -> Router<Arc<AppState>> {
    return Router::new()
        .route("/", delete(delete_user))
        .route("/", patch(change_account_details));
}

async fn delete_user(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
    session: Session,
) -> Result<StatusCode, AppError> {
    let functions: Vec<(i32, Uuid)> = function::Entity::find()
        .filter(function::COLUMN.owner_id.eq(user.id))
        .select_only()
        .column(function::Column::Id)
        .column(function::Column::Uuid)
        .into_tuple()
        .all(&state.db)
        .await
        .context("could not fetch user functions")?;

    for (function_db_id, function_uuid) in functions {
        let run_uuids: Vec<Uuid> = run::Entity::find()
            .filter(run::COLUMN.function_id.eq(function_db_id))
            .select_only()
            .column(run::Column::Uuid)
            .into_tuple()
            .all(&state.db)
            .await
            .context("could not fetch function runs")?;

        purge_function_objects(&state, function_uuid, &run_uuids).await?;
    }

    UserModel::delete_by_id(user.id)
        .exec(&state.db)
        .await
        .context("could not delete user")?;

    session
        .delete()
        .await
        .context("could not remove session on user delete")?;

    Ok(StatusCode::NO_CONTENT)
}
async fn change_account_details(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
    Json(payload): Json<ChangeAccountDetailsPayload>,
) -> Result<StatusCode, AppError> {
    let model = user::ActiveModel {
        id: Set(user.id),
        full_name: Set(payload.full_name),
        email: Set(payload.email),
        ..Default::default()
    };

    UserModel::update(model)
        .exec(&state.db)
        .await
        .context("could not update user details")?;

    Ok(StatusCode::NO_CONTENT)
}
