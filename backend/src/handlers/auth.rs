use std::sync::Arc;

use anyhow::{Context, anyhow};
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, patch, post},
};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{Set, Unchanged},
    EntityTrait, QueryFilter, QuerySelect,
};
use serde_json::json;
use tower_sessions_cookie_store::Session;
use uuid::Uuid;

use crate::{
    errors::AppError,
    exctractors::{AuthUser, ValidatedJson},
    models::{
        api_key::{self, ActiveModel as ApiKeyModel, Entity as ApiKey},
        user::{self, Entity as User},
    },
    schemas::auth::{
        ChangeApiKeyResponse, ChangePasswordPayload, GetApiKeyResponse, LoginUserPayload,
        LoginUserResponse, RegisterUserPayload, RegisterUserResponse,
    },
    state::AppState,
};

pub fn auth_router() -> Router<Arc<AppState>> {
    return Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .route("/logout", post(logout))
        .route("/change-password", patch(change_password))
        .route("/api-key", get(get_api_key).patch(regenerate_api_key));
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

    let new_api_key = ApiKeyModel {
        key: Set(Uuid::now_v7()),
        owner_id: Set(user.id),
        ..Default::default()
    };

    new_api_key
        .insert(&state.db)
        .await
        .context("could not insert api key into the db")?;

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

async fn regenerate_api_key(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
) -> Result<Json<ChangeApiKeyResponse>, AppError> {
    let uuid = Uuid::now_v7();
    let model = ApiKeyModel {
        key: Set(uuid),
        owner_id: Unchanged(user.id),
        ..Default::default()
    };

    ApiKey::update(model)
        .exec(&state.db)
        .await
        .context("could not update user api key")?;

    Ok(Json(ChangeApiKeyResponse { api_key: uuid }))
}

async fn get_api_key(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
) -> Result<Json<GetApiKeyResponse>, AppError> {
    let db_api_key = api_key::Entity::find()
        .filter(api_key::Entity::COLUMN.owner_id.eq(user.id))
        .select_only()
        .column(api_key::Column::Key)
        .into_tuple()
        .one(&state.db)
        .await
        .context("could not fetch api key due to db error")?;

    match db_api_key {
        Some(api_key) => Ok(Json(GetApiKeyResponse { api_key })),
        None => Err(AppError::Internal(anyhow!(
            "could not fetch user={} api key",
            user.id
        ))),
    }
}
