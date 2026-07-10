use std::sync::Arc;

use anyhow::{Context, Result};
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{delete, patch},
};
use sea_orm::{ActiveValue::Set, EntityTrait};
use tower_sessions::Session;

use crate::{
    errors::AppError,
    exctractors::AuthUser,
    models::user::{self, Entity as UserModel},
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
