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
mod schema;

#[allow(unused_imports)] // Required for into_make_service_with_connect_info
use axum::extract::ConnectInfo;
use axum::http::{header, Method};
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use config::AppConfig;
use tower::limit::ConcurrencyLimitLayer;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tracing::info;
use tracing_subscriber::EnvFilter;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

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

    // ==========================================================================
    // CORS CONFIGURATION FOR SECURE COOKIE-BASED AUTH
    // ==========================================================================
    //
    // Origins are configured via ALLOWED_ORIGINS environment variable.
    // In production, this MUST be set to your actual domain(s).
    // In development, defaults to localhost origins.
    //
    // ==========================================================================
    let allowed_origins: Vec<axum::http::HeaderValue> = config
        .allowed_origins
        .iter()
        .filter_map(|origin| origin.parse().ok())
        .collect();

    if allowed_origins.is_empty() {
        eprintln!("Warning: No valid CORS origins configured");
    }

    let allowed_headers = [
        header::CONTENT_TYPE,
        header::AUTHORIZATION,
        header::ACCEPT,
        header::HeaderName::from_static("x-client-type"),
    ];

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(allowed_headers)
        .allow_origin(allowed_origins)
        .allow_credentials(true);

    // ==========================================================================
    // RATE LIMITING CONFIGURATION
    // ==========================================================================
    //
    // Two rate limiters:
    // 1. General API: 50 req/sec, burst 100 (for normal endpoints)
    // 2. Auth endpoints: 5 req/min, burst 10 (prevent brute force)
    //
    // ==========================================================================
    
    // General rate limiter for most endpoints
    let general_governor = GovernorConfigBuilder::default()
        .per_second(50)
        .burst_size(100)
        .finish()
        .expect("general governor config");

    // Strict rate limiter for auth endpoints (prevent brute force)
    let auth_governor = GovernorConfigBuilder::default()
        .per_second(1) // 1 request per second sustained
        .burst_size(5) // Allow burst of 5 attempts
        .finish()
        .expect("auth governor config");

    // Auth routes with stricter rate limiting
    let auth_routes = Router::new()
        .route("/auth/login", axum::routing::post(api::login))
        .route("/auth/logout", axum::routing::post(api::logout))
        .route("/auth/refresh", axum::routing::post(api::refresh))
        .layer(GovernorLayer::new(auth_governor));

    let app = Router::new()
        .nest("/api/v1", api::routes().merge(auth_routes))
        .route("/health/live", get(api::live))
        .route("/health/ready", get(api::ready))
        .layer(TraceLayer::new_for_http()) // Request/response logging
        .layer(GovernorLayer::new(general_governor))
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

    // Graceful shutdown handling
    let shutdown_signal = async {
        let ctrl_c = async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {
                info!("Received Ctrl+C, starting graceful shutdown...");
            },
            _ = terminate => {
                info!("Received SIGTERM, starting graceful shutdown...");
            },
        }
    };

    // Use into_make_service_with_connect_info for rate limiter to extract peer IP
    let server = axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal);

    if let Err(err) = server.await {
        eprintln!("Server error: {err}");
        std::process::exit(1);
    }

    info!("Server shutdown complete");
}
