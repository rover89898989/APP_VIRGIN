use serde::{Deserialize, Serialize};
use ts_rs::TS;

// These structs are used for TypeScript type generation via ts-rs.
// They will be used by API handlers once the database schema is set up.
#[allow(dead_code)]

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub name: String,
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
