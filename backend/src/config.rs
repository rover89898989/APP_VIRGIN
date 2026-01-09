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
///                                      Default: true when `DATABASE_URL` is set, else false.
///
/// FAILURE MODES:
/// - If `DATABASE_REQUIRED=true` and `DATABASE_URL` is missing, startup fails with a clear error.
/// - This keeps `/health/ready` honest and prevents routing traffic to a server that cannot serve.
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: IpAddr,
    pub port: u16,
    pub database_url: Option<String>,
    pub database_required: bool,
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

        Ok(Self {
            host,
            port,
            database_url,
            database_required,
        })
    }

    pub fn addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}
