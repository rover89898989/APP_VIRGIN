// ==============================================================================
// AUTHENTICATION API - SECURE TOKEN HANDLING
// ==============================================================================
//
// This module handles authentication with SECURE token storage.
//
// SECURITY MODEL:
// ---------------
// - Native apps (iOS/Android): Tokens stored in SecureStore (hardware-backed)
// - Web apps: Tokens stored in httpOnly cookies (immune to XSS)
//
// WHY httpOnly COOKIES FOR WEB:
// -----------------------------
// localStorage/sessionStorage are vulnerable to XSS (Cross-Site Scripting):
//   - If an attacker injects JavaScript, they can steal tokens with:
//     `localStorage.getItem('access_token')`
//   - This gives them full access to the user's account
//
// httpOnly cookies CANNOT be read by JavaScript:
//   - The browser automatically sends them with requests
//   - Even if XSS occurs, the attacker cannot extract the token
//   - This is the INDUSTRY STANDARD for web authentication in 2026
//
// TRADEOFFS:
// ----------
// - Cookies require CORS credentials mode
// - Cookies don't transfer between web and native apps
// - CSRF protection is recommended (noted for future enhancement)
//
// ==============================================================================

use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::env;

use crate::AppState;
use super::jwt::{generate_token_pair, generate_access_token, validate_refresh_token, TokenPair};

// ==============================================================================
// COOKIE CONFIGURATION
// ==============================================================================
//
// These constants define how authentication cookies behave.
// They are security-critical and should not be changed without review.
//
// ==============================================================================

/// Cookie name for the access token.
/// Using a prefix like `__Host-` would add additional security but requires HTTPS.
const ACCESS_TOKEN_COOKIE_NAME: &str = "access_token";

/// Cookie name for the refresh token.
const REFRESH_TOKEN_COOKIE_NAME: &str = "refresh_token";

/// Access token cookie max age in seconds (15 minutes).
/// Short-lived tokens reduce the window of exposure if somehow compromised.
const ACCESS_TOKEN_MAX_AGE_SECONDS: i64 = 900; // 15 minutes

/// Refresh token cookie max age in seconds (7 days).
const REFRESH_TOKEN_MAX_AGE_SECONDS: i64 = 604800; // 7 days

/// Check if we're running in production (requires Secure cookies)
fn is_production() -> bool {
    env::var("ENVIRONMENT")
        .map(|v| v.to_lowercase() == "production" || v.to_lowercase() == "prod")
        .unwrap_or(false)
}

// ==============================================================================
// REQUEST/RESPONSE TYPES
// ==============================================================================

/// Login request payload.
/// 
/// SECURITY NOTE:
/// - Password is transmitted over HTTPS (TLS) in production
/// - Never log passwords or include them in error messages
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Login response payload.
/// 
/// NOTE: For web clients, the access token is set as an httpOnly cookie.
/// For native clients (detected via X-Client-Type header), tokens are in the body.
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    /// Access token - only populated for native clients
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// Refresh token - only populated for native clients
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    /// Seconds until access token expires
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i64>,
}

/// Refresh token request payload
#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

/// Refresh token response payload
#[derive(Debug, Serialize)]
pub struct RefreshResponse {
    pub success: bool,
    pub access_token: String,
    pub expires_in: i64,
}

// ==============================================================================
// LOGIN ENDPOINT
// ==============================================================================
//
// POST /api/v1/auth/login
//
// This endpoint authenticates a user and returns a token.
// The token delivery method depends on the client type:
//
// WEB CLIENTS:
//   - Token is set as an httpOnly cookie
//   - Cookie is automatically sent with subsequent requests
//   - JavaScript CANNOT read this cookie (XSS protection)
//
// NATIVE CLIENTS (iOS/Android):
//   - Token is returned in the response body
//   - Client stores it in SecureStore (hardware-backed encryption)
//   - Detected via `X-Client-Type: native` header
//
// ==============================================================================

pub async fn login(
    State(_state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<LoginRequest>,
) -> Response {
    // ==========================================================================
    // INPUT VALIDATION
    // ==========================================================================
    if request.email.is_empty() || request.password.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(LoginResponse {
                success: false,
                message: "Email and password are required".to_string(),
                access_token: None,
                refresh_token: None,
                expires_in: None,
            }),
        )
            .into_response();
    }

    // ==========================================================================
    // DATABASE LOOKUP & PASSWORD VERIFICATION
    // ==========================================================================
    // TODO: Replace with actual database lookup when migrations are complete
    // 
    // In production:
    // 1. Look up user by email in database
    // 2. Verify password against stored hash using password::verify_password()
    // 3. Return 401 if user not found or password mismatch
    //
    // For now, we use a demo user for testing the JWT flow
    // ==========================================================================
    
    let demo_user_id: i64 = 1;
    let demo_email = &request.email;
    
    // TODO: Uncomment when database is ready
    // let user = match get_user_by_email(&state.db_pool, &request.email).await {
    //     Ok(u) => u,
    //     Err(_) => return unauthorized_response("Invalid email or password"),
    // };
    // 
    // if !password::verify_password(&request.password, &user.password_hash)? {
    //     return unauthorized_response("Invalid email or password");
    // }

    // ==========================================================================
    // GENERATE JWT TOKENS
    // ==========================================================================
    let token_pair = match generate_token_pair(demo_user_id, demo_email) {
        Ok(pair) => pair,
        Err(e) => {
            tracing::error!("Failed to generate tokens: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(LoginResponse {
                    success: false,
                    message: "Authentication failed".to_string(),
                    access_token: None,
                    refresh_token: None,
                    expires_in: None,
                }),
            )
                .into_response();
        }
    };

    // ==========================================================================
    // DETECT CLIENT TYPE (WEB vs NATIVE)
    // ==========================================================================
    let is_native_client = headers
        .get("X-Client-Type")
        .map(|v| v.to_str().unwrap_or("").to_lowercase() == "native")
        .unwrap_or(false);

    // ==========================================================================
    // BUILD RESPONSE BASED ON CLIENT TYPE
    // ==========================================================================
    if is_native_client {
        // Native clients: Return tokens in response body
        // They will store in SecureStore (hardware-backed encryption)
        (
            StatusCode::OK,
            Json(LoginResponse {
                success: true,
                message: "Login successful".to_string(),
                access_token: Some(token_pair.access_token),
                refresh_token: Some(token_pair.refresh_token),
                expires_in: Some(token_pair.expires_in),
            }),
        )
            .into_response()
    } else {
        // Web clients: Set httpOnly cookies (immune to XSS)
        let access_cookie = build_auth_cookie(&token_pair.access_token, false);
        let refresh_cookie = build_refresh_cookie(&token_pair.refresh_token, false);
        
        (
            StatusCode::OK,
            [
                (header::SET_COOKIE, access_cookie),
                (header::SET_COOKIE, refresh_cookie),
            ],
            Json(LoginResponse {
                success: true,
                message: "Login successful".to_string(),
                access_token: None, // In cookie, not body
                refresh_token: None, // In cookie, not body
                expires_in: Some(token_pair.expires_in),
            }),
        )
            .into_response()
    }
}

// ==============================================================================
// LOGOUT ENDPOINT
// ==============================================================================
//
// POST /api/v1/auth/logout
//
// Clears the authentication cookie by setting it to expire immediately.
// The browser will delete the cookie and stop sending it with requests.
//
// ==============================================================================

pub async fn logout() -> Response {
    // Clear both access and refresh cookies
    let access_cookie = build_auth_cookie("", true);
    let refresh_cookie = build_refresh_cookie("", true);

    (
        StatusCode::OK,
        [
            (header::SET_COOKIE, access_cookie),
            (header::SET_COOKIE, refresh_cookie),
        ],
        Json(serde_json::json!({
            "success": true,
            "message": "Logged out successfully"
        })),
    )
        .into_response()
}

// ==============================================================================
// REFRESH TOKEN ENDPOINT
// ==============================================================================
//
// POST /api/v1/auth/refresh
//
// Uses the refresh token to obtain a new access token without re-authenticating.
// This allows short-lived access tokens while maintaining user sessions.
//
// ==============================================================================

pub async fn refresh(
    headers: HeaderMap,
    body: Option<Json<RefreshRequest>>,
) -> Response {
    // ==========================================================================
    // EXTRACT REFRESH TOKEN
    // ==========================================================================
    // Check body first (native clients), then cookie (web clients)
    
    let refresh_token = if let Some(Json(req)) = body {
        // Native client: token in request body
        Some(req.refresh_token)
    } else {
        // Web client: token in cookie
        extract_refresh_token_from_cookie(&headers)
    };
    
    let refresh_token = match refresh_token {
        Some(t) if !t.is_empty() => t,
        _ => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "success": false,
                    "message": "Refresh token required"
                })),
            )
                .into_response();
        }
    };

    // ==========================================================================
    // VALIDATE REFRESH TOKEN
    // ==========================================================================
    let claims = match validate_refresh_token(&refresh_token) {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "success": false,
                    "message": "Invalid or expired refresh token"
                })),
            )
                .into_response();
        }
    };

    // ==========================================================================
    // GENERATE NEW ACCESS TOKEN
    // ==========================================================================
    let user_id = match claims.user_id() {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "success": false,
                    "message": "Invalid token claims"
                })),
            )
                .into_response();
        }
    };

    let new_access_token = match generate_access_token(user_id, &claims.email) {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Failed to generate access token: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "success": false,
                    "message": "Token generation failed"
                })),
            )
                .into_response();
        }
    };

    // ==========================================================================
    // DETECT CLIENT TYPE AND RESPOND
    // ==========================================================================
    let is_native_client = headers
        .get("X-Client-Type")
        .map(|v| v.to_str().unwrap_or("").to_lowercase() == "native")
        .unwrap_or(false);

    if is_native_client {
        // Native: return token in body
        (
            StatusCode::OK,
            Json(RefreshResponse {
                success: true,
                access_token: new_access_token,
                expires_in: 900, // 15 minutes
            }),
        )
            .into_response()
    } else {
        // Web: set new cookie
        let cookie = build_auth_cookie(&new_access_token, false);
        (
            StatusCode::OK,
            [(header::SET_COOKIE, cookie)],
            Json(serde_json::json!({
                "success": true,
                "expires_in": 900
            })),
        )
            .into_response()
    }
}

/// Extract refresh token from cookie header
fn extract_refresh_token_from_cookie(headers: &HeaderMap) -> Option<String> {
    if let Some(cookie_header) = headers.get(header::COOKIE) {
        if let Ok(cookies_str) = cookie_header.to_str() {
            for cookie in cookies_str.split(';') {
                let cookie = cookie.trim();
                if let Some(value) = cookie.strip_prefix(&format!("{}=", REFRESH_TOKEN_COOKIE_NAME)) {
                    if !value.is_empty() {
                        return Some(value.to_string());
                    }
                }
            }
        }
    }
    None
}

// ==============================================================================
// HELPER FUNCTIONS
// ==============================================================================

/// Builds the Set-Cookie header value for the access token.
///
/// # Arguments
/// * `token` - The token value (or empty string for logout)
/// * `clear` - If true, sets Max-Age=0 to delete the cookie
///
/// # Cookie Attributes
/// - `HttpOnly`: Prevents JavaScript access (XSS protection)
/// - `SameSite=Lax`: Prevents CSRF for most requests
/// - `Path=/`: Cookie valid for all routes
/// - `Secure`: Only send over HTTPS (auto-enabled in production)
fn build_auth_cookie(token: &str, clear: bool) -> String {
    let max_age = if clear { 0 } else { ACCESS_TOKEN_MAX_AGE_SECONDS };
    let secure_flag = if is_production() { "; Secure" } else { "" };

    format!(
        "{}={}; HttpOnly; SameSite=Lax; Path=/; Max-Age={}{}",
        ACCESS_TOKEN_COOKIE_NAME,
        token,
        max_age,
        secure_flag
    )
}

/// Builds the Set-Cookie header value for the refresh token.
///
/// Similar to access token but with longer expiry and restricted path.
fn build_refresh_cookie(token: &str, clear: bool) -> String {
    let max_age = if clear { 0 } else { REFRESH_TOKEN_MAX_AGE_SECONDS };
    let secure_flag = if is_production() { "; Secure" } else { "" };

    format!(
        "{}={}; HttpOnly; SameSite=Lax; Path=/api/v1/auth; Max-Age={}{}",
        REFRESH_TOKEN_COOKIE_NAME,
        token,
        max_age,
        secure_flag
    )
}

// ==============================================================================
// MIDDLEWARE: EXTRACT TOKEN FROM COOKIE
// ==============================================================================
//
// This function extracts the access token from the request.
// It checks both:
// 1. Cookie (for web clients)
// 2. Authorization header (for native clients)
//
// This allows the same endpoints to work for both web and native.
//
// ==============================================================================

/// Extracts the access token from the request.
/// 
/// Priority:
/// 1. Authorization: Bearer <token> header (native clients)
/// 2. access_token cookie (web clients)
/// 
/// Returns None if no token is found.
#[allow(dead_code)] // Will be used by auth middleware when protected routes are added
pub fn extract_token_from_request(headers: &axum::http::HeaderMap) -> Option<String> {
    // ==========================================================================
    // CHECK AUTHORIZATION HEADER FIRST (Native clients)
    // ==========================================================================
    //
    // Native apps send: Authorization: Bearer <token>
    // This is the standard for mobile apps using SecureStore.
    //
    // ==========================================================================

    if let Some(auth_header) = headers.get(header::AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                return Some(token.to_string());
            }
        }
    }

    // ==========================================================================
    // CHECK COOKIE (Web clients)
    // ==========================================================================
    //
    // Web browsers automatically send cookies with requests.
    // We parse the Cookie header to find our access_token.
    //
    // ==========================================================================

    if let Some(cookie_header) = headers.get(header::COOKIE) {
        if let Ok(cookies_str) = cookie_header.to_str() {
            // Parse cookies (simple implementation - production should use cookie crate)
            for cookie in cookies_str.split(';') {
                let cookie = cookie.trim();
                if let Some(value) = cookie.strip_prefix(&format!("{}=", ACCESS_TOKEN_COOKIE_NAME)) {
                    if !value.is_empty() {
                        return Some(value.to_string());
                    }
                }
            }
        }
    }

    None
}

// ==============================================================================
// TESTS
// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn test_build_auth_cookie_sets_httponly() {
        let cookie = build_auth_cookie("test_token", false);
        assert!(cookie.contains("HttpOnly"), "Cookie must be HttpOnly for XSS protection");
    }

    #[test]
    fn test_build_auth_cookie_sets_samesite() {
        let cookie = build_auth_cookie("test_token", false);
        assert!(cookie.contains("SameSite=Lax"), "Cookie should have SameSite for CSRF protection");
    }

    #[test]
    fn test_build_auth_cookie_clear_sets_zero_max_age() {
        let cookie = build_auth_cookie("", true);
        assert!(cookie.contains("Max-Age=0"), "Clear cookie must expire immediately");
    }

    #[test]
    fn test_extract_token_from_bearer_header() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_static("Bearer my_secret_token"),
        );
        let token = extract_token_from_request(&headers);
        assert_eq!(token, Some("my_secret_token".to_string()));
    }

    #[test]
    fn test_extract_token_from_cookie() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(
            header::COOKIE,
            HeaderValue::from_static("access_token=cookie_token; other=value"),
        );
        let token = extract_token_from_request(&headers);
        assert_eq!(token, Some("cookie_token".to_string()));
    }

    #[test]
    fn test_bearer_header_takes_priority_over_cookie() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_static("Bearer header_token"),
        );
        headers.insert(
            header::COOKIE,
            HeaderValue::from_static("access_token=cookie_token"),
        );
        let token = extract_token_from_request(&headers);
        // Bearer header should take priority (for native clients)
        assert_eq!(token, Some("header_token".to_string()));
    }
}
