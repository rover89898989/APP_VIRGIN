use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::schema::users;

/// User entity - maps to the `users` database table.
/// 
/// Derives:
/// - Queryable: Can be queried from the database
/// - Identifiable: Has an `id` field for lookups
/// - Selectable: For type-safe column selection
/// - TS: Generates TypeScript types
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable, TS)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[ts(export)]
pub struct User {
    pub id: i64,
    pub email: String,
    #[serde(skip_serializing)] // Never send password hash to client
    #[ts(skip)]
    pub password_hash: String,
    pub name: String,
    pub is_active: bool,
    #[ts(type = "string")]
    pub created_at: DateTime<Utc>,
    #[ts(type = "string")]
    pub updated_at: DateTime<Utc>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct UserResponse {
    pub id: i64,
    pub user: User,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub name: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserError {
    InvalidEmail,
}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::InvalidEmail => write!(f, "invalid email"),
        }
    }
}

impl std::error::Error for UserError {}
