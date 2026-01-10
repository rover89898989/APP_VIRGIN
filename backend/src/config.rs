use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

/// Application configuration.
///
/// CONTRACT:
/// - Values are read once at startup.
/// - Invalid or missing required values cause a fail-fast startup error.
/// - This prevents "half-configured" deployments (critical for production safety).
///
/// ENVIRONMENT VARIABLES:
/// - `BACKEND_HOST` (optional)         : IP to bind. Default `127.0.0.1`.
/// - `BACKEND_PORT` (optional)         : Port to bind. Default `8000`.
/// - `DATABASE_URL` (optional)         : Postgres connection string.
/// - `DATABASE_REQUIRED` (optional)    : If true, missing DB is a startup error.
/// - `ALLOWED_ORIGINS` (optional)      : Comma-separated list of allowed CORS origins.
/// - `ENVIRONMENT` (optional)          : "production" or "development". Affects security settings.
/// - `JWT_SECRET` (required in prod)   : Secret key for JWT signing.
///
/// FAILURE MODES:
/// - If `DATABASE_REQUIRED=true` and `DATABASE_URL` is missing, startup fails with a clear error.
/// - If `ENVIRONMENT=production` and `ALLOWED_ORIGINS` is missing, startup fails.
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: IpAddr,
    pub port: u16,
    pub database_url: Option<String>,
    pub database_required: bool,
    pub allowed_origins: Vec<String>,
    pub environment: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, String> {
        let host = env::var("BACKEND_HOST")
            .ok()
            .and_then(|v| v.parse::<IpAddr>().ok())
            .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST));

        let port = env::var("BACKEND_PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(8000);

        let database_url = env::var("DATABASE_URL").ok().filter(|v| !v.trim().is_empty());

        let database_required = env::var("DATABASE_REQUIRED")
            .ok()
            .and_then(|v| match v.to_lowercase().as_str() {
                "1" | "true" | "yes" => Some(true),
                "0" | "false" | "no" => Some(false),
                _ => None,
            })
            .unwrap_or(database_url.is_some());

        if database_required && database_url.is_none() {
            return Err("DATABASE_REQUIRED=true but DATABASE_URL is missing".to_string());
        }

        // Environment detection
        let environment = env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "development".to_string())
            .to_lowercase();
        
        let is_production = environment == "production" || environment == "prod";

        // CORS origins - comma-separated list
        let allowed_origins = env::var("ALLOWED_ORIGINS")
            .ok()
            .map(|v| {
                v.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_else(|| {
                if is_production {
                    Vec::new() // Will fail validation below
                } else {
                    // Development defaults
                    vec![
                        "http://localhost:8081".to_string(),
                        "http://localhost:19006".to_string(),
                        "http://127.0.0.1:8081".to_string(),
                        "http://10.0.2.2:8081".to_string(),
                    ]
                }
            });

        // Validate production requirements
        if is_production {
            if allowed_origins.is_empty() {
                return Err("ALLOWED_ORIGINS must be set in production".to_string());
            }
            if env::var("JWT_SECRET").is_err() {
                return Err("JWT_SECRET must be set in production".to_string());
            }
        }

        Ok(Self {
            host,
            port,
            database_url,
            database_required,
            allowed_origins,
            environment,
        })
    }

    pub fn addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }

    pub fn is_production(&self) -> bool {
        self.environment == "production" || self.environment == "prod"
    }
}
