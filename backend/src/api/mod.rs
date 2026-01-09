mod health;

pub use health::{live, ready};

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use axum::routing::get;
use axum::Router;

use crate::AppState;

/// Canonical API error type.
///
/// CONTRACT:
/// - Errors returned from handlers MUST be converted into a stable HTTP status + JSON body.
/// - Error messages must be safe for clients (no secrets, no tokens, no PHI).
/// - Internal diagnostics should be logged separately (with redaction policy).
///
/// This keeps the "failure surface" consistent across the app, which makes:
/// - mobile clients easier to implement
/// - audits easier (HIPAA / PHI handling)
/// - future maintenance safer
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("bad request")]
    BadRequest(String),

    #[error("unauthorized")]
    Unauthorized(String),

    #[error("forbidden")]
    Forbidden(String),

    #[error("not found")]
    NotFound(String),

    #[error("conflict")]
    Conflict(String),

    #[error("service unavailable")]
    ServiceUnavailable(String),

    #[error("internal error")]
    InternalError(String),
}

#[derive(Debug, Serialize)]
struct ApiErrorBody {
    error: String,
}

impl ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn public_message(&self) -> String {
        // NOTE:
        // Keep this message client-safe.
        // If you add error variants that wrap other error types, do NOT `format!("{err}")`
        // unless you're certain it cannot contain secrets/PHI.
        match self {
            ApiError::BadRequest(msg)
            | ApiError::Unauthorized(msg)
            | ApiError::Forbidden(msg)
            | ApiError::NotFound(msg)
            | ApiError::Conflict(msg)
            | ApiError::ServiceUnavailable(msg)
            | ApiError::InternalError(msg) => msg.clone(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        (status, Json(ApiErrorBody {
            error: self.public_message(),
        }))
            .into_response()
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/health/live", get(live))
        .route("/health/ready", get(ready))
}
