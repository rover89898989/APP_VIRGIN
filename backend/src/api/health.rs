use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;

use crate::db;
use crate::AppState;

#[derive(Debug, Serialize)]
struct LiveResponse {
    status: &'static str,
}

pub async fn live() -> impl IntoResponse {
    (StatusCode::OK, Json(LiveResponse { status: "ok" }))
}

#[derive(Debug, Serialize)]
struct ReadyResponse {
    status: &'static str,
    database: &'static str,
}

pub async fn ready(State(state): State<AppState>) -> impl IntoResponse {
    match &state.db_pool {
        Some(pool) => {
            let pool = pool.clone();
            match tokio::task::spawn_blocking(move || db::check_database(&pool)).await {
                Ok(Ok(())) => (
                    StatusCode::OK,
                    Json(ReadyResponse {
                        status: "ready",
                        database: "ok",
                    }),
                ),
                Ok(Err(_)) | Err(_) => (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(ReadyResponse {
                        status: "not_ready",
                        database: "down",
                    }),
                ),
            }
        }
        None if state.config.database_required => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ReadyResponse {
                status: "not_ready",
                database: "missing",
            }),
        ),
        None => (
            StatusCode::OK,
            Json(ReadyResponse {
                status: "ready",
                database: "disabled",
            }),
        ),
    }
}
