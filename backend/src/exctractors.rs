use axum::{
    Json,
    extract::{FromRequest, Request},
};
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
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("JSON Parse error: {}", e)))?;

        value.validate().map_err(|e| {
            let error_details = serde_json::to_value(&e).unwrap_or(serde_json::json!({}));
            AppError::InvalidRequest(error_details)
        })?;

        Ok(ValidatedJson(value))
    }
}
