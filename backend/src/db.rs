use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::RunQueryDsl;

/// Diesel connection pool type.
///
/// NOTE:
/// - Diesel is synchronous (blocking I/O).
/// - Any calls that touch the database should be executed in `spawn_blocking` when called
///   from async handlers, to avoid blocking Tokio's async runtime.
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Create a Diesel connection pool.
///
/// FAILURE MODES:
/// - Returns an error string suitable for a startup failure.
pub fn create_pool(database_url: &str) -> Result<DbPool, String> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .map_err(|e| format!("failed to create database pool: {e}"))
}

/// Lightweight database liveness probe.
///
/// PURPOSE:
/// - Used by `/health/ready` to decide if the server is safe to receive traffic.
///
/// IMPORTANT:
/// - This is a blocking operation.
/// - Call it from `spawn_blocking` in async contexts.
pub fn check_database(pool: &DbPool) -> Result<(), String> {
    let mut conn = pool
        .get()
        .map_err(|e| format!("failed to get database connection from pool: {e}"))?;

    diesel::sql_query("SELECT 1")
        .execute(&mut conn)
        .map_err(|e| format!("database health query failed: {e}"))?;

    Ok(())
}
