// ==============================================================================
// CSRF PROTECTION
// ==============================================================================
//
// Cross-Site Request Forgery (CSRF) protection for state-changing requests.
//
// STRATEGY: Double Submit Cookie Pattern
// 1. Server generates a random CSRF token
// 2. Token is sent in both a cookie AND expected in a header
// 3. Attackers can trigger requests but can't read/set the header
// 4. If cookie value != header value, request is rejected
//
// WHY THIS WORKS:
// - Attacker's site can trigger POST requests to our API
// - Browser automatically includes our cookies (if credentials mode)
// - BUT attacker can't read our cookies (same-origin policy)
// - AND attacker can't set custom headers on cross-origin requests
// - So attacker can't provide the matching X-CSRF-Token header
//
// ==============================================================================

use axum::{
    extract::Request,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use rand::Rng;
use std::env;

/// Cookie name for CSRF token
const CSRF_COOKIE_NAME: &str = "csrf_token";

/// Header name for CSRF token
const CSRF_HEADER_NAME: &str = "x-csrf-token";

/// CSRF token length in bytes (32 bytes = 256 bits)
const CSRF_TOKEN_LENGTH: usize = 32;

/// Generate a cryptographically secure random CSRF token
pub fn generate_csrf_token() -> String {
    let mut rng = rand::thread_rng();
    let token: Vec<u8> = (0..CSRF_TOKEN_LENGTH).map(|_| rng.gen()).collect();
    hex::encode(token)
}

/// Build CSRF cookie value
pub fn build_csrf_cookie(token: &str) -> String {
    let is_production = env::var("ENVIRONMENT")
        .map(|v| v.to_lowercase() == "production" || v.to_lowercase() == "prod")
        .unwrap_or(false);
    
    let secure_flag = if is_production { "; Secure" } else { "" };
    
    // Note: This cookie is NOT HttpOnly because JavaScript needs to read it
    // to include in the X-CSRF-Token header
    format!(
        "{}={}; SameSite=Lax; Path=/{}",
        CSRF_COOKIE_NAME,
        token,
        secure_flag
    )
}

/// CSRF validation middleware
///
/// Validates CSRF token for state-changing requests (POST, PUT, DELETE, PATCH).
/// GET and HEAD requests are exempt (they should be idempotent).
///
/// # How it works
/// 1. Extract CSRF token from cookie
/// 2. Extract CSRF token from X-CSRF-Token header
/// 3. Compare them (constant-time comparison)
/// 4. Reject if they don't match
pub async fn csrf_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().clone();
    
    // Skip CSRF check for safe methods (GET, HEAD, OPTIONS)
    if method == axum::http::Method::GET 
        || method == axum::http::Method::HEAD 
        || method == axum::http::Method::OPTIONS 
    {
        return next.run(request).await;
    }
    
    // Skip CSRF check for native clients (they use Bearer tokens, not cookies)
    if let Some(client_type) = headers.get("x-client-type") {
        if client_type.to_str().unwrap_or("").to_lowercase() == "native" {
            return next.run(request).await;
        }
    }
    
    // Extract CSRF token from cookie
    let cookie_token = extract_csrf_from_cookie(&headers);
    
    // Extract CSRF token from header
    let header_token = headers
        .get(CSRF_HEADER_NAME)
        .and_then(|v| v.to_str().ok())
        .map(String::from);
    
    // Validate tokens match
    match (cookie_token, header_token) {
        (Some(cookie), Some(header)) if constant_time_eq(&cookie, &header) => {
            // Tokens match - proceed
            next.run(request).await
        }
        (None, _) => {
            // No cookie token - might be first request, generate one
            // For now, reject - client should fetch a CSRF token first
            tracing::warn!("CSRF validation failed: no cookie token");
            (
                StatusCode::FORBIDDEN,
                axum::Json(serde_json::json!({
                    "error": "CSRF token missing. Fetch /api/v1/csrf first."
                })),
            )
                .into_response()
        }
        _ => {
            // Tokens don't match or header missing
            tracing::warn!("CSRF validation failed: token mismatch");
            (
                StatusCode::FORBIDDEN,
                axum::Json(serde_json::json!({
                    "error": "CSRF token invalid"
                })),
            )
                .into_response()
        }
    }
}

/// Extract CSRF token from cookie header
fn extract_csrf_from_cookie(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::COOKIE)?
        .to_str()
        .ok()?
        .split(';')
        .find_map(|cookie| {
            let cookie = cookie.trim();
            cookie
                .strip_prefix(&format!("{}=", CSRF_COOKIE_NAME))
                .filter(|v| !v.is_empty())
                .map(String::from)
        })
}

/// Constant-time string comparison to prevent timing attacks
fn constant_time_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let mut result = 0u8;
    for (x, y) in a.bytes().zip(b.bytes()) {
        result |= x ^ y;
    }
    result == 0
}

/// Handler to get a new CSRF token
///
/// GET /api/v1/csrf
///
/// Returns a CSRF token in both:
/// 1. Response body (for JavaScript to read)
/// 2. Set-Cookie header (for browser to store)
pub async fn get_csrf_token() -> Response {
    let token = generate_csrf_token();
    let cookie = build_csrf_cookie(&token);
    
    (
        StatusCode::OK,
        [(header::SET_COOKIE, HeaderValue::from_str(&cookie).unwrap())],
        axum::Json(serde_json::json!({
            "csrf_token": token
        })),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_csrf_token_length() {
        let token = generate_csrf_token();
        // 32 bytes = 64 hex characters
        assert_eq!(token.len(), 64);
    }
    
    #[test]
    fn test_generate_csrf_token_unique() {
        let token1 = generate_csrf_token();
        let token2 = generate_csrf_token();
        assert_ne!(token1, token2);
    }
    
    #[test]
    fn test_constant_time_eq_same() {
        assert!(constant_time_eq("abc123", "abc123"));
    }
    
    #[test]
    fn test_constant_time_eq_different() {
        assert!(!constant_time_eq("abc123", "abc124"));
    }
    
    #[test]
    fn test_constant_time_eq_different_length() {
        assert!(!constant_time_eq("abc", "abcd"));
    }
}
