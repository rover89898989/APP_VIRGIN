// ==============================================================================
// BACKEND MAIN ENTRY POINT
// ==============================================================================
//
// This is the main entry point for the Rust backend API server.
//
// ARCHITECTURE:
// - Axum web framework (async, fast, production-ready)
// - Diesel ORM with PostgreSQL
// - Clean Architecture (API → Domain → Infrastructure)
// - Type generation for TypeScript frontend
//
// FEATURES:
// - Health checks
// - User management
// - JWT authentication
// - Rate limiting
// - CORS
//
// ==============================================================================

mod api;
mod config;
mod db;
mod features;

use axum::http::Method;
use axum::routing::get;
use axum::Router;
use config::AppConfig;
use tower::limit::ConcurrencyLimitLayer;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tracing::info;
use tracing_subscriber::EnvFilter;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};

pub type DbPool = db::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db_pool: Option<DbPool>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = match AppConfig::from_env() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("Configuration error: {err}");
            std::process::exit(1);
        }
    };

    let db_pool = match (&config.database_url, config.database_required) {
        (Some(url), _) => match db::create_pool(url) {
            Ok(pool) => Some(pool),
            Err(err) => {
                eprintln!("Database pool error: {err}");
                std::process::exit(1);
            }
        },
        (None, true) => {
            eprintln!("Configuration error: DATABASE_REQUIRED=true but DATABASE_URL is missing");
            std::process::exit(1);
        }
        (None, false) => None,
    };

    let state = AppState {
        config: config.clone(),
        db_pool,
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    let governor_conf = GovernorConfigBuilder::default()
        .per_second(50)
        .burst_size(100)
        .finish()
        .expect("governor config");

    let app = Router::new()
        .nest("/api/v1", api::routes())
        .route("/health/live", get(api::live))
        .route("/health/ready", get(api::ready))
        .layer(GovernorLayer::new(governor_conf))
        .layer(cors)
        .layer(ConcurrencyLimitLayer::new(256))
        .layer(CompressionLayer::new())
        .with_state(state);

    let listener = match tokio::net::TcpListener::bind(config.addr()).await {
        Ok(l) => l,
        Err(err) => {
            eprintln!("Failed to bind to {}: {err}", config.addr());
            std::process::exit(1);
        }
    };

    info!("backend listening on http://{}", config.addr());

    if let Err(err) = axum::serve(listener, app).await {
        eprintln!("Server error: {err}");
        std::process::exit(1);
    }
}
