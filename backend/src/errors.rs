use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
struct ErrorPayload {
    msg: String, // Human readable message
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<Value>, // Optional details - for example for validation errors
    error: String, // Status code -> text message
}

pub enum AppError {
    Internal(anyhow::Error),
    InvalidRequest(Value),
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg, details) = match self {
            AppError::Internal(inner_err) => {
                tracing::error!("Internal server error: {:?}", inner_err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal server error occured".to_string(),
                    None,
                )
            }
            AppError::InvalidRequest(validation_err) => (
                StatusCode::BAD_REQUEST,
                "Invalid request data".to_string(),
                Some(validation_err),
            ),
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "Resource not found".to_string(),
                None,
            ),
        };

        let error_string = status
            .canonical_reason()
            .unwrap_or("Unknown error")
            .to_string();

        let body = Json(ErrorPayload {
            msg,
            details,
            error: error_string,
        });

        return (status, body).into_response();
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err)
    }
}
