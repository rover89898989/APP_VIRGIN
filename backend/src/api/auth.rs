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
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::AppState;

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

/// Cookie max age in seconds (1 hour).
/// Short-lived tokens reduce the window of exposure if somehow compromised.
/// Users will need to re-login after this time.
const ACCESS_TOKEN_MAX_AGE_SECONDS: i64 = 3600;

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
/// NOTE: The actual token is NOT included in the response body for web clients.
/// Instead, it's set as an httpOnly cookie that the browser manages automatically.
/// 
/// For native clients, we include the token in the response body because:
/// - Native apps can't use cookies across app boundaries
/// - Native apps use SecureStore which is hardware-backed and secure
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    /// Token is only populated for native clients (detected via User-Agent or header)
    /// Web clients receive the token via httpOnly cookie instead
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
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
    Json(request): Json<LoginRequest>,
) -> Response {
    // ==========================================================================
    // STUB IMPLEMENTATION
    // ==========================================================================
    // In a real app, you would:
    // 1. Look up the user by email in the database
    // 2. Verify the password hash using argon2 or bcrypt
    // 3. Generate a JWT or opaque token
    // 4. Return the token via cookie (web) or body (native)
    //
    // For this template, we accept any login and return a dummy token.
    // ==========================================================================

    // Validate input (basic check - real app would do more)
    if request.email.is_empty() || request.password.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(LoginResponse {
                success: false,
                message: "Email and password are required".to_string(),
                access_token: None,
            }),
        )
            .into_response();
    }

    // Generate a dummy token (in production, use a proper JWT library)
    // This is just for demonstration - NEVER use this in production!
    let token = format!("dummy_token_for_{}", request.email);

    // ==========================================================================
    // BUILD THE RESPONSE WITH httpOnly COOKIE
    // ==========================================================================
    //
    // Cookie attributes explained:
    //
    // - HttpOnly: JavaScript CANNOT read this cookie (XSS protection)
    // - Secure: Cookie only sent over HTTPS (enable in production)
    // - SameSite=Lax: Protects against CSRF for most cases
    // - Path=/: Cookie sent for all paths on this domain
    // - Max-Age: Cookie expires after this many seconds
    //
    // ==========================================================================

    let cookie_value = build_auth_cookie(&token, false);

    let response_body = LoginResponse {
        success: true,
        message: "Login successful".to_string(),
        // For web clients, token is in cookie, not body
        // Native clients would check X-Client-Type header and get token in body
        access_token: None,
    };

    // Build response with Set-Cookie header
    (
        StatusCode::OK,
        [(header::SET_COOKIE, cookie_value)],
        Json(response_body),
    )
        .into_response()
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
    // ==========================================================================
    // CLEAR THE COOKIE
    // ==========================================================================
    //
    // To delete a cookie, we set it with:
    // - Empty value
    // - Max-Age=0 (expire immediately)
    //
    // The browser will remove it from storage.
    //
    // ==========================================================================

    let cookie_value = build_auth_cookie("", true);

    (
        StatusCode::OK,
        [(header::SET_COOKIE, cookie_value)],
        Json(serde_json::json!({
            "success": true,
            "message": "Logged out successfully"
        })),
    )
        .into_response()
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
/// - `Secure`: Only send over HTTPS (commented out for local dev)
///
/// # Security Notes
/// - In production, ALWAYS enable the Secure flag
/// - Consider using `__Host-` prefix for additional security
/// - SameSite=Strict would be more secure but breaks some OAuth flows
fn build_auth_cookie(token: &str, clear: bool) -> String {
    let max_age = if clear { 0 } else { ACCESS_TOKEN_MAX_AGE_SECONDS };

    // ==========================================================================
    // COOKIE FORMAT
    // ==========================================================================
    //
    // Format: name=value; attribute1; attribute2; ...
    //
    // We build this manually for clarity. In production, consider using
    // a cookie library like `cookie` crate for proper escaping.
    //
    // ==========================================================================

    format!(
        "{}={}; HttpOnly; SameSite=Lax; Path=/; Max-Age={}",
        ACCESS_TOKEN_COOKIE_NAME,
        token,
        max_age
    )
    // ==========================================================================
    // PRODUCTION NOTE: Add "; Secure" flag when deploying to HTTPS
    // ==========================================================================
    // In production, uncomment this to require HTTPS:
    // format!(
    //     "{}={}; HttpOnly; Secure; SameSite=Lax; Path=/; Max-Age={}",
    //     ACCESS_TOKEN_COOKIE_NAME,
    //     token,
    //     max_age
    // )
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
