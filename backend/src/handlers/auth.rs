use std::sync::Arc;

use anyhow::{Context, Result, anyhow};
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{patch, post},
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
use serde_json::json;
use tower_sessions_cookie_store::Session;

use crate::{
    errors::AppError,
    exctractors::{AuthUser, ValidatedJson},
    models::user::{self, Entity as User},
    schemas::auth::{
        ChangePasswordPayload, LoginUserPayload, LoginUserResponse, RegisterUserPayload,
        RegisterUserResponse,
    },
    state::AppState,
};

pub fn auth_router() -> Router<Arc<AppState>> {
    return Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .route("/logout", post(logout))
        .route("/change-password", patch(change_password));
}

async fn register_user(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<RegisterUserPayload>,
) -> Result<(StatusCode, Json<RegisterUserResponse>), AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .context("Failed to create hash")?
        .to_string();

    let new_user = user::ActiveModel {
        full_name: Set(payload.full_name),
        email: Set(payload.email),
        password: Set(password_hash),
        ..Default::default()
    };

    let user = new_user
        .insert(&state.db)
        .await
        .context("could not insert user into the db")?;

    return Ok((
        StatusCode::CREATED,
        Json(RegisterUserResponse {
            email: user.email,
            full_name: user.full_name,
        }),
    ));
}

async fn login_user(
    State(state): State<Arc<AppState>>,
    session: Session,
    ValidatedJson(payload): ValidatedJson<LoginUserPayload>,
) -> Result<(StatusCode, Json<LoginUserResponse>), AppError> {
    let user = User::find_by_email(payload.email)
        .one(&state.db)
        .await
        .map_err(|db_err| {
            tracing::error!("get user error: {:?}", db_err);
            AppError::Internal(anyhow!("user get database error"))
        })?
        .ok_or_else(|| AppError::InvalidRequest(json!({"global": "Invalid email or password."})))?;

    let passwords_identical = Argon2::default().verify_password(
        payload.password.as_ref(),
        &PasswordHash::new(&user.password).context("")?,
    );

    if passwords_identical.is_err() {
        return Err(AppError::InvalidRequest(
            json!({"global": "invalid email or password"}),
        ));
    }

    session
        .insert_value("user_id", serde_json::json!(user.id))
        .await
        .context("could not insert user_id into session")?;

    return Ok((
        StatusCode::OK,
        Json(LoginUserResponse {
            email: user.email,
            full_name: user.full_name,
        }),
    ));
}

async fn logout(_: AuthUser, session: Session) -> Result<StatusCode, AppError> {
    session
        .delete()
        .await
        .context("could not remove session on logout")?;

    return Ok(StatusCode::OK);
}

async fn change_password(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
    Json(payload): Json<ChangePasswordPayload>,
) -> Result<StatusCode, AppError> {
    let user = User::find_by_id(user.id)
        .one(&state.db)
        .await
        .map_err(|db_err| {
            tracing::error!("get user error: {:?}", db_err);
            AppError::Internal(anyhow!("user get database error"))
        })?
        .ok_or_else(|| AppError::Internal(anyhow!("can't get user to update password")))?;

    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let passwords_identical = argon2.verify_password(
        payload.current_password.as_ref(),
        &PasswordHash::new(&user.password).context("")?,
    );

    if passwords_identical.is_err() {
        return Err(AppError::InvalidRequest(
            json!({"global": "sent password do not match current one"}),
        ));
    }

    let new_hashed_password = argon2
        .hash_password(payload.new_password.as_bytes(), &salt)
        .context("Failed to create hash")?
        .to_string();

    let model = user::ActiveModel {
        id: Set(user.id),
        password: Set(new_hashed_password),
        ..Default::default()
    };

    User::update(model)
        .exec(&state.db)
        .await
        .context("could not set new user password")?;

    Ok(StatusCode::NO_CONTENT)
}
