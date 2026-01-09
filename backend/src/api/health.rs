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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request, Router};
    use axum::routing::get;
    use tower::ServiceExt;

    fn create_test_app() -> Router {
        let config = crate::config::AppConfig {
            host: std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
            port: 8000,
            database_url: None,
            database_required: false,
        };
        let state = crate::AppState {
            config,
            db_pool: None,
        };
        Router::new()
            .route("/health/live", get(live))
            .route("/health/ready", get(ready))
            .with_state(state)
    }

    #[tokio::test]
    async fn test_health_live_returns_ok() {
        let app = create_test_app();
        let response = app
            .oneshot(Request::builder().uri("/health/live").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["status"], "ok");
    }

    #[tokio::test]
    async fn test_health_ready_without_db_returns_ok() {
        let app = create_test_app();
        let response = app
            .oneshot(Request::builder().uri("/health/ready").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["status"], "ready");
        assert_eq!(json["database"], "disabled");
    }

    #[tokio::test]
    async fn test_health_ready_with_required_db_missing_returns_503() {
        let config = crate::config::AppConfig {
            host: std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
            port: 8000,
            database_url: None,
            database_required: true,
        };
        let state = crate::AppState {
            config,
            db_pool: None,
        };
        let app = Router::new()
            .route("/health/ready", get(ready))
            .with_state(state);

        let response = app
            .oneshot(Request::builder().uri("/health/ready").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["status"], "not_ready");
        assert_eq!(json["database"], "missing");
    }
}
