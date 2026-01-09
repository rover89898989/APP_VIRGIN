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

#[allow(unused_imports)] // Required for into_make_service_with_connect_info
use axum::extract::ConnectInfo;
use axum::http::Method;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
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

    // ==========================================================================
    // CORS CONFIGURATION FOR SECURE COOKIE-BASED AUTH
    // ==========================================================================
    //
    // IMPORTANT: When using httpOnly cookies for authentication, we must:
    // 1. Set `allow_credentials(true)` - allows cookies to be sent cross-origin
    // 2. Specify exact origins - `Any` origin is NOT allowed with credentials
    // 3. The frontend must use `withCredentials: true` on requests
    //
    // WHY NOT `Any` ORIGIN?
    // - The browser security model forbids `Access-Control-Allow-Origin: *` 
    //   when credentials (cookies) are involved
    // - This prevents malicious sites from making authenticated requests
    //
    // DEVELOPMENT vs PRODUCTION:
    // - Development: Allow localhost origins (multiple ports for Expo)
    // - Production: Replace with your actual domain(s)
    //
    // ==========================================================================
    let allowed_origins = [
        "http://localhost:8081".parse().unwrap(),  // Expo web
        "http://localhost:19006".parse().unwrap(), // Expo web (alt port)
        "http://127.0.0.1:8081".parse().unwrap(),  // Expo web (IP)
        "http://10.0.2.2:8081".parse().unwrap(),   // Android emulator
        // Production: Add your domain here
        // "https://yourapp.com".parse().unwrap(),
    ];

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_origin(allowed_origins)
        // CRITICAL: This enables cookies to be sent cross-origin
        // Without this, the browser will NOT include cookies in requests
        .allow_credentials(true);

    // ==========================================================================
    // RATE LIMITING CONFIGURATION
    // ==========================================================================
    //
    // Protects the API from abuse and denial-of-service attacks.
    //
    // CONFIGURATION:
    // - per_second(50): Allows 50 requests per second per IP
    // - burst_size(100): Allows bursts of up to 100 requests
    //
    // CRITICAL REQUIREMENT:
    // The rate limiter needs the client's IP address to work. This requires:
    // 
    //   app.into_make_service_with_connect_info::<SocketAddr>()
    //
    // instead of the usual:
    //
    //   app.into_make_service()
    //
    // Without this, you'll get "Unable To Extract Key!" errors because
    // tower_governor cannot determine the client's IP address.
    //
    // This is configured at the bottom of this file in axum::serve().
    //
    // PRODUCTION NOTES:
    // - If behind a reverse proxy, use SmartIpKeyExtractor instead
    // - Adjust limits based on your traffic patterns
    // - Consider different limits for different endpoints
    //
    // ==========================================================================
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

    // Use into_make_service_with_connect_info for rate limiter to extract peer IP
    if let Err(err) = axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await {
        eprintln!("Server error: {err}");
        std::process::exit(1);
    }
}
