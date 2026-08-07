
use axum::{
    Json,
    extract::{FromRequest, FromRequestParts, Request},
    http::request::Parts,
};
use serde_json::json;
use tower_sessions_cookie_store::Session;
use uuid::Uuid;
use validator::Validate;

use crate::errors::AppError;

pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: serde::de::DeserializeOwned + Validate + 'static,
    S: Send + Sync,
    Json<T>: FromRequest<S>,
    <Json<T> as FromRequest<S>>::Rejection: std::fmt::Display,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await.map_err(|e| {
            let error_payload = json!({
                "error": "Invalid JSON payload",
                "details": e.to_string()
            });

            AppError::InvalidRequest(error_payload)
        })?;

        value.validate().map_err(|e| {
            let error_details = serde_json::to_value(&e).unwrap_or(serde_json::json!({}));
            AppError::InvalidRequest(error_details)
        })?;

        Ok(ValidatedJson(value))
    }
}

#[derive(Clone)]
pub struct AuthUser {
    pub id: i32,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(|_| AppError::Internal(anyhow::anyhow!("Session missing")))?;

        let user_id = session.get("user_id").await.unwrap_or(None);
        match user_id {
            Some(id) => Ok(AuthUser { id }),
            None => Err(AppError::Unauthorized),
        }
    }
}

pub struct ExtractApiKey(pub Uuid);

impl<S> FromRequestParts<S> for ExtractApiKey
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let header_value = parts.headers.get("x-api-key");

        let raw_header = match header_value {
            Some(raw_header) => raw_header,
            None => return Err(AppError::Unauthorized),
        };

        let stringified_header = match raw_header.to_str() {
            Ok(api_key) => api_key.to_string(),
            Err(_) => return Err(AppError::Unauthorized),
        };

        let uuid = Uuid::parse_str(&stringified_header).map_err(|_| AppError::Unauthorized)?;
        Ok(ExtractApiKey(uuid))
    }
}
