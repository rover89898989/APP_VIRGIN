// ==============================================================================
// JWT TOKEN MANAGEMENT
// ==============================================================================
//
// Handles JWT token generation and validation for authentication.
//
// SECURITY MODEL:
// - Access tokens: Short-lived (15 min), used for API requests
// - Refresh tokens: Long-lived (7 days), used only to get new access tokens
// - Tokens signed with HS256 (symmetric) - use RS256 for multi-service setups
//
// ==============================================================================

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::env;

use super::ApiError;

// ==============================================================================
// CONFIGURATION
// ==============================================================================

/// Get JWT secret from environment variable.
/// CRITICAL: This MUST be set in production. Use a strong random secret (32+ bytes).
fn get_jwt_secret() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| {
        if cfg!(debug_assertions) {
            // Development only - NEVER use this in production
            eprintln!("⚠️  WARNING: Using default JWT_SECRET. Set JWT_SECRET env var in production!");
            "DEVELOPMENT_ONLY_SECRET_CHANGE_IN_PRODUCTION_32bytes".to_string()
        } else {
            panic!("JWT_SECRET environment variable must be set in production");
        }
    })
}

/// Access token validity duration
const ACCESS_TOKEN_DURATION_MINUTES: i64 = 15;

/// Refresh token validity duration
const REFRESH_TOKEN_DURATION_DAYS: i64 = 7;

// ==============================================================================
// TOKEN CLAIMS
// ==============================================================================

/// Claims embedded in the JWT token.
/// 
/// Standard claims:
/// - `sub`: Subject (user ID)
/// - `exp`: Expiration time (Unix timestamp)
/// - `iat`: Issued at (Unix timestamp)
/// 
/// Custom claims:
/// - `email`: User's email (for convenience, avoid DB lookup)
/// - `token_type`: "access" or "refresh" (prevent refresh token misuse)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,        // User ID as string
    pub email: String,      // User email
    pub token_type: String, // "access" or "refresh"
    pub exp: i64,           // Expiration (Unix timestamp)
    pub iat: i64,           // Issued at (Unix timestamp)
    pub jti: String,        // JWT ID (for revocation)
}

impl Claims {
    /// Create new access token claims
    pub fn new_access(user_id: i64, email: &str) -> Self {
        let now = Utc::now();
        let exp = now + Duration::minutes(ACCESS_TOKEN_DURATION_MINUTES);
        
        Self {
            sub: user_id.to_string(),
            email: email.to_string(),
            token_type: "access".to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            jti: uuid::Uuid::new_v4().to_string(),
        }
    }
    
    /// Create new refresh token claims
    pub fn new_refresh(user_id: i64, email: &str) -> Self {
        let now = Utc::now();
        let exp = now + Duration::days(REFRESH_TOKEN_DURATION_DAYS);
        
        Self {
            sub: user_id.to_string(),
            email: email.to_string(),
            token_type: "refresh".to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            jti: uuid::Uuid::new_v4().to_string(),
        }
    }
    
    /// Get user ID from claims
    pub fn user_id(&self) -> Result<i64, ApiError> {
        self.sub.parse::<i64>()
            .map_err(|_| ApiError::Unauthorized("Invalid token subject".to_string()))
    }
    
    /// Check if this is an access token
    pub fn is_access_token(&self) -> bool {
        self.token_type == "access"
    }
    
    /// Check if this is a refresh token
    pub fn is_refresh_token(&self) -> bool {
        self.token_type == "refresh"
    }
}

// ==============================================================================
// TOKEN GENERATION
// ==============================================================================

/// Token pair returned after successful authentication
#[derive(Debug, Serialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64, // Seconds until access token expires
}

/// Generate a new access/refresh token pair for a user.
/// 
/// # Arguments
/// * `user_id` - The user's database ID
/// * `email` - The user's email address
/// 
/// # Returns
/// * `Ok(TokenPair)` - Access and refresh tokens
/// * `Err(ApiError)` - Token generation failed
pub fn generate_token_pair(user_id: i64, email: &str) -> Result<TokenPair, ApiError> {
    let secret = get_jwt_secret();
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());
    
    // Generate access token
    let access_claims = Claims::new_access(user_id, email);
    let access_token = encode(&Header::default(), &access_claims, &encoding_key)
        .map_err(|e| {
            tracing::error!("Failed to generate access token: {}", e);
            ApiError::InternalError("Token generation failed".to_string())
        })?;
    
    // Generate refresh token
    let refresh_claims = Claims::new_refresh(user_id, email);
    let refresh_token = encode(&Header::default(), &refresh_claims, &encoding_key)
        .map_err(|e| {
            tracing::error!("Failed to generate refresh token: {}", e);
            ApiError::InternalError("Token generation failed".to_string())
        })?;
    
    Ok(TokenPair {
        access_token,
        refresh_token,
        expires_in: ACCESS_TOKEN_DURATION_MINUTES * 60, // Convert to seconds
    })
}

/// Generate only an access token (used during refresh)
pub fn generate_access_token(user_id: i64, email: &str) -> Result<String, ApiError> {
    let secret = get_jwt_secret();
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());
    
    let claims = Claims::new_access(user_id, email);
    encode(&Header::default(), &claims, &encoding_key)
        .map_err(|e| {
            tracing::error!("Failed to generate access token: {}", e);
            ApiError::InternalError("Token generation failed".to_string())
        })
}

// ==============================================================================
// TOKEN VALIDATION
// ==============================================================================

/// Validate and decode a JWT token.
/// 
/// # Arguments
/// * `token` - The JWT token string
/// 
/// # Returns
/// * `Ok(Claims)` - Valid token, returns claims
/// * `Err(ApiError)` - Invalid, expired, or malformed token
pub fn validate_token(token: &str) -> Result<Claims, ApiError> {
    let secret = get_jwt_secret();
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    
    let validation = Validation::default();
    
    let token_data: TokenData<Claims> = decode(token, &decoding_key, &validation)
        .map_err(|e| {
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    ApiError::Unauthorized("Token expired".to_string())
                }
                jsonwebtoken::errors::ErrorKind::InvalidToken => {
                    ApiError::Unauthorized("Invalid token".to_string())
                }
                _ => {
                    tracing::warn!("Token validation failed: {}", e);
                    ApiError::Unauthorized("Token validation failed".to_string())
                }
            }
        })?;
    
    Ok(token_data.claims)
}

/// Validate an access token specifically.
/// Rejects refresh tokens used as access tokens.
pub fn validate_access_token(token: &str) -> Result<Claims, ApiError> {
    let claims = validate_token(token)?;
    
    if !claims.is_access_token() {
        return Err(ApiError::Unauthorized("Invalid token type".to_string()));
    }
    
    Ok(claims)
}

/// Validate a refresh token specifically.
/// Rejects access tokens used as refresh tokens.
pub fn validate_refresh_token(token: &str) -> Result<Claims, ApiError> {
    let claims = validate_token(token)?;
    
    if !claims.is_refresh_token() {
        return Err(ApiError::Unauthorized("Invalid token type".to_string()));
    }
    
    Ok(claims)
}

// ==============================================================================
// TESTS
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_and_validate_token_pair() {
        let pair = generate_token_pair(123, "test@example.com").unwrap();
        
        // Validate access token
        let access_claims = validate_access_token(&pair.access_token).unwrap();
        assert_eq!(access_claims.sub, "123");
        assert_eq!(access_claims.email, "test@example.com");
        assert!(access_claims.is_access_token());
        
        // Validate refresh token
        let refresh_claims = validate_refresh_token(&pair.refresh_token).unwrap();
        assert_eq!(refresh_claims.sub, "123");
        assert!(refresh_claims.is_refresh_token());
    }
    
    #[test]
    fn test_access_token_rejected_as_refresh() {
        let pair = generate_token_pair(123, "test@example.com").unwrap();
        
        // Access token should fail when validated as refresh token
        let result = validate_refresh_token(&pair.access_token);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_refresh_token_rejected_as_access() {
        let pair = generate_token_pair(123, "test@example.com").unwrap();
        
        // Refresh token should fail when validated as access token
        let result = validate_access_token(&pair.refresh_token);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_invalid_token_rejected() {
        let result = validate_token("invalid.token.here");
        assert!(result.is_err());
    }
}
