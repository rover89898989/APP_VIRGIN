// ==============================================================================
// USER REPOSITORY - DATABASE QUERIES
// ==============================================================================
//
// This module handles all database operations for users.
// Uses Diesel ORM with PostgreSQL.
//
// CRITICAL PERFORMANCE FIX:
// - Diesel is SYNCHRONOUS (blocking I/O)
// - Axum/Tokio is ASYNCHRONOUS (non-blocking)
// - Running Diesel queries directly BLOCKS the async runtime
// - This causes the entire API to hang if DB is slow
// 
// SOLUTION: tokio::task::spawn_blocking
// - Offloads blocking work to dedicated thread pool
// - Async runtime stays responsive
// - Health checks pass, but requests still process
//
// ==============================================================================

use crate::DbPool;
use crate::features::users::domain::entities::{User, CreateUserRequest, UpdateUserRequest, UserError};
use crate::api::ApiError;
use crate::api::password;
use crate::schema::users;
use diesel::prelude::*;
use chrono::Utc;

// ==============================================================================
// USER REPOSITORY
// ==============================================================================

/// Get user by ID
///
/// PERFORMANCE FIX: Uses spawn_blocking to prevent blocking the async runtime.
///
/// # Why spawn_blocking?
/// - Diesel queries are synchronous (blocking)
/// - Axum handlers are async (non-blocking)
/// - Running blocking code in async context = bad performance
/// - spawn_blocking moves work to separate thread pool
/// 
/// # Arguments
/// * `pool` - Database connection pool
/// * `user_id` - User's unique identifier
/// 
/// # Returns
/// * `Ok(User)` - User data
/// * `Err(ApiError)` - User not found or database error
/// 
/// # Example
/// ```
/// async fn handler(
///     State(pool): State<DbPool>,
///     Path(id): Path<i64>
/// ) -> Result<Json<User>, ApiError> {
///     let user = get_user_by_id(pool, id).await?;
///     Ok(Json(user))
/// }
/// ```
pub async fn get_user_by_id(
    pool: DbPool,
    user_id: i64,
) -> Result<User, ApiError> {
    tokio::task::spawn_blocking(move || {
        let mut conn = pool.get()
            .map_err(|e| {
                tracing::error!("Failed to get DB connection: {}", e);
                ApiError::InternalError("Database connection failed".to_string())
            })?;
        
        users::table
            .find(user_id)
            .first::<User>(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    ApiError::NotFound(format!("User {} not found", user_id))
                }
                _ => {
                    tracing::error!("Database query error: {}", e);
                    ApiError::InternalError("Database query failed".to_string())
                }
            })
    })
    .await
    .map_err(|e| {
        tracing::error!("Thread panic in database query: {}", e);
        ApiError::InternalError("Database query panicked".to_string())
    })?
}

/// Create new user
///
/// PERFORMANCE FIX: Uses spawn_blocking for database insert.
pub async fn create_user(
    pool: DbPool,
    data: CreateUserRequest,
) -> Result<User, ApiError> {
    // Validate email before hitting database
    crate::features::users::domain::validate_email(&data.email)
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;
    
    // Hash password before database insert
    let password_hash = password::hash_password(&data.password)?;
    
    tokio::task::spawn_blocking(move || {
        let mut conn = pool.get()
            .map_err(|e| {
                tracing::error!("Failed to get DB connection: {}", e);
                ApiError::InternalError("Database connection failed".to_string())
            })?;
        
        diesel::insert_into(users::table)
            .values((
                users::email.eq(&data.email),
                users::password_hash.eq(&password_hash),
                users::name.eq(&data.name),
            ))
            .get_result::<User>(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation, _
                ) => {
                    ApiError::Conflict("Email already exists".to_string())
                }
                _ => {
                    tracing::error!("Database insert error: {}", e);
                    ApiError::InternalError("Database insert failed".to_string())
                }
            })
    })
    .await
    .map_err(|e| {
        tracing::error!("Thread panic in database insert: {}", e);
        ApiError::InternalError("Database insert panicked".to_string())
    })?
}

/// Update user
///
/// PERFORMANCE FIX: Uses spawn_blocking for database update.
pub async fn update_user(
    pool: DbPool,
    user_id: i64,
    data: UpdateUserRequest,
) -> Result<User, ApiError> {
    // Validate email if provided
    if let Some(ref email) = data.email {
        crate::features::users::domain::validate_email(email)
            .map_err(|e| ApiError::BadRequest(e.to_string()))?;
    }
    
    tokio::task::spawn_blocking(move || {
        let mut conn = pool.get()
            .map_err(|e| {
                tracing::error!("Failed to get DB connection: {}", e);
                ApiError::InternalError("Database connection failed".to_string())
            })?;
        
        let now = Utc::now();
        
        // Build dynamic update query
        let target = users::table.find(user_id);
        
        // Update fields that are provided
        let updated_rows = if let (Some(email), Some(name)) = (&data.email, &data.name) {
            diesel::update(target)
                .set((
                    users::email.eq(email),
                    users::name.eq(name),
                    users::updated_at.eq(now),
                ))
                .execute(&mut conn)
        } else if let Some(email) = &data.email {
            diesel::update(target)
                .set((
                    users::email.eq(email),
                    users::updated_at.eq(now),
                ))
                .execute(&mut conn)
        } else if let Some(name) = &data.name {
            diesel::update(target)
                .set((
                    users::name.eq(name),
                    users::updated_at.eq(now),
                ))
                .execute(&mut conn)
        } else {
            // No fields to update
            Ok(0)
        }
        .map_err(|e| {
            tracing::error!("Database update error: {}", e);
            ApiError::InternalError("Database update failed".to_string())
        })?;
        
        if updated_rows == 0 {
            return Err(ApiError::NotFound(format!("User {} not found", user_id)));
        }
        
        // Fetch the updated user
        users::table
            .find(user_id)
            .first::<User>(&mut conn)
            .map_err(|e| {
                tracing::error!("Failed to fetch updated user: {}", e);
                ApiError::InternalError("Database query failed".to_string())
            })
    })
    .await
    .map_err(|e| {
        tracing::error!("Thread panic in database update: {}", e);
        ApiError::InternalError("Database update panicked".to_string())
    })?
}

/// Delete user (soft delete)
///
/// PERFORMANCE FIX: Uses spawn_blocking for database update.
pub async fn delete_user(
    pool: DbPool,
    user_id: i64,
) -> Result<(), ApiError> {
    tokio::task::spawn_blocking(move || {
        let mut conn = pool.get()
            .map_err(|e| {
                tracing::error!("Failed to get DB connection: {}", e);
                ApiError::InternalError("Database connection failed".to_string())
            })?;
        
        let now = Utc::now();
        
        let updated_rows = diesel::update(users::table.find(user_id))
            .set((
                users::is_active.eq(false),
                users::updated_at.eq(now),
            ))
            .execute(&mut conn)
            .map_err(|e| {
                tracing::error!("Database delete error: {}", e);
                ApiError::InternalError("Database delete failed".to_string())
            })?;
        
        if updated_rows == 0 {
            return Err(ApiError::NotFound(format!("User {} not found", user_id)));
        }
        
        Ok(())
    })
    .await
    .map_err(|e| {
        tracing::error!("Thread panic in database delete: {}", e);
        ApiError::InternalError("Database delete panicked".to_string())
    })?
}

/// Get user by email (for authentication)
pub async fn get_user_by_email(
    pool: DbPool,
    email: String,
) -> Result<User, ApiError> {
    tokio::task::spawn_blocking(move || {
        let mut conn = pool.get()
            .map_err(|e| {
                tracing::error!("Failed to get DB connection: {}", e);
                ApiError::InternalError("Database connection failed".to_string())
            })?;
        
        users::table
            .filter(users::email.eq(&email))
            .filter(users::is_active.eq(true))
            .first::<User>(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    ApiError::NotFound("User not found".to_string())
                }
                _ => {
                    tracing::error!("Database query error: {}", e);
                    ApiError::InternalError("Database query failed".to_string())
                }
            })
    })
    .await
    .map_err(|e| {
        tracing::error!("Thread panic in database query: {}", e);
        ApiError::InternalError("Database query panicked".to_string())
    })?
}

// ==============================================================================
// PERFORMANCE COMPARISON
// ==============================================================================
//
// ❌ BAD (Blocks async runtime):
// ```rust
// pub async fn get_user(pool: State<DbPool>, id: i64) -> Result<User, ApiError> {
//     let mut conn = pool.get()?;  // BLOCKS all other requests!
//     users::table.find(id).first(&mut conn)?  // BLOCKS all other requests!
// }
// ```
//
// ✅ GOOD (Non-blocking):
// ```rust
// pub async fn get_user(pool: DbPool, id: i64) -> Result<User, ApiError> {
//     tokio::task::spawn_blocking(move || {
//         let mut conn = pool.get()?;  // Blocks only this thread
//         users::table.find(id).first(&mut conn)?  // Blocks only this thread
//     }).await??  // Await the spawned task
// }
// ```
//
// IMPACT:
// - With blocking: 1 slow query = entire API hangs
// - With spawn_blocking: 1 slow query = only that request is slow
// - Health check remains responsive even if database is slow
//
// ==============================================================================

// ==============================================================================
// WHEN TO USE spawn_blocking
// ==============================================================================
//
// USE spawn_blocking for:
// - ✅ Database queries (Diesel, rusqlite, etc.)
// - ✅ File I/O (reading/writing files)
// - ✅ CPU-intensive work (heavy computation)
// - ✅ Blocking APIs (non-async libraries)
//
// DON'T use spawn_blocking for:
// - ❌ Quick validation (< 1ms)
// - ❌ In-memory operations
// - ❌ Already-async code (async fn, .await)
// - ❌ Trivial work (overhead > work time)
//
// RULE OF THUMB:
// If it blocks for > 10 microseconds, use spawn_blocking.
//
// ==============================================================================

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    // NOTE: These are examples - actual tests require database setup
    
    #[tokio::test]
    async fn test_get_user_doesnt_block_runtime() {
        // This test would verify that getting a user doesn't block
        // other async tasks from running
        // Would require setting up test database
    }
    
    #[tokio::test]
    async fn test_concurrent_queries() {
        // This test would verify that multiple queries can run
        // concurrently without blocking each other
        // Would require setting up test database
    }
}
