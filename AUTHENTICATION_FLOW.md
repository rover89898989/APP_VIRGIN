# 🔐 Authentication Flow Diagrams

Comprehensive visual documentation of authentication flows with verbose explanations.

---

## 📊 Overview

This document contains ASCII diagrams showing:
1. **Login Flow** - Web vs Native authentication
2. **Token Refresh Flow** - Automatic token renewal
3. **CSRF Protection** - Double-submit cookie pattern
4. **Request Flow** - Complete request lifecycle with security layers

---

## 1️⃣ Login Flow - Web vs Native

### Web Client Login (httpOnly Cookies)

```
┌─────────────┐                                    ┌─────────────┐
│   Browser   │                                    │   Backend   │
│  (Web App)  │                                    │   (Axum)    │
└──────┬──────┘                                    └──────┬──────┘
       │                                                  │
       │ POST /api/v1/auth/login                         │
       │ Headers: Content-Type: application/json         │
       │ Body: { email, password }                       │
       ├─────────────────────────────────────────────────>│
       │                                                  │
       │                                                  │ 1. Validate email format
       │                                                  │ 2. Query user from database
       │                                                  │ 3. Verify password with Argon2
       │                                                  │ 4. Generate JWT access token (15min)
       │                                                  │ 5. Generate JWT refresh token (7d)
       │                                                  │ 6. Detect client type (no X-Client-Type = web)
       │                                                  │
       │ 200 OK                                           │
       │ Set-Cookie: access_token=xxx; HttpOnly; SameSite=Lax; Secure
       │ Set-Cookie: refresh_token=yyy; HttpOnly; SameSite=Lax; Secure
       │ Body: { success: true, access_token: null }     │
       │<─────────────────────────────────────────────────┤
       │                                                  │
       │ ✅ Tokens stored in httpOnly cookies            │
       │ ✅ JavaScript CANNOT access tokens              │
       │ ✅ XSS attacks CANNOT steal tokens              │
       │ ✅ Browser auto-sends cookies on requests       │
       │                                                  │
```

**Key Security Features:**
- **httpOnly flag**: JavaScript cannot read cookies (XSS protection)
- **SameSite=Lax**: Prevents CSRF on state-changing requests
- **Secure flag**: Only sent over HTTPS (production)
- **No token in response body**: Web clients don't need it

---

### Native Client Login (SecureStore)

```
┌─────────────┐                                    ┌─────────────┐
│   Mobile    │                                    │   Backend   │
│  (React     │                                    │   (Axum)    │
│   Native)   │                                    │             │
└──────┬──────┘                                    └──────┬──────┘
       │                                                  │
       │ POST /api/v1/auth/login                         │
       │ Headers:                                         │
       │   Content-Type: application/json                │
       │   X-Client-Type: native  ← CRITICAL             │
       │ Body: { email, password }                       │
       ├─────────────────────────────────────────────────>│
       │                                                  │
       │                                                  │ 1. Validate email format
       │                                                  │ 2. Query user from database
       │                                                  │ 3. Verify password with Argon2
       │                                                  │ 4. Generate JWT access token (15min)
       │                                                  │ 5. Generate JWT refresh token (7d)
       │                                                  │ 6. Detect X-Client-Type: native
       │                                                  │ 7. Return tokens in response body
       │                                                  │
       │ 200 OK                                           │
       │ Body: {                                          │
       │   success: true,                                 │
       │   access_token: "eyJ...",  ← TOKEN IN BODY      │
       │   refresh_token: "eyJ..."                        │
       │ }                                                │
       │<─────────────────────────────────────────────────┤
       │                                                  │
       │ Store tokens in SecureStore                      │
       │ ┌─────────────────────────────────┐             │
       │ │ await SecureStore.setItemAsync( │             │
       │ │   'access_token',                │             │
       │ │   response.data.access_token     │             │
       │ │ )                                │             │
       │ └─────────────────────────────────┘             │
       │                                                  │
       │ ✅ Tokens stored in hardware-backed keychain    │
       │ ✅ Encrypted at rest                            │
       │ ✅ Requires biometric/PIN to access             │
       │ ✅ Other apps CANNOT access                     │
       │                                                  │
```

**Key Security Features:**
- **X-Client-Type header**: Backend knows to return tokens in body
- **expo-secure-store**: Hardware-backed encryption (iOS Keychain, Android Keystore)
- **No cookies**: Native apps don't use cookies
- **Biometric protection**: OS-level security for token access

---

## 2️⃣ Token Refresh Flow

### Automatic Token Refresh on 401

```
┌─────────────┐                                    ┌─────────────┐
│   Client    │                                    │   Backend   │
│  (Any Type) │                                    │   (Axum)    │
└──────┬──────┘                                    └──────┬──────┘
       │                                                  │
       │ GET /api/v1/users/123                           │
       │ Authorization: Bearer <expired_access_token>    │
       ├─────────────────────────────────────────────────>│
       │                                                  │
       │                                                  │ 1. Validate JWT signature
       │                                                  │ 2. Check expiration
       │                                                  │ 3. Token expired!
       │                                                  │
       │ 401 Unauthorized                                 │
       │ { error: "Token expired" }                      │
       │<─────────────────────────────────────────────────┤
       │                                                  │
       │ ⚠️ Access token expired!                        │
       │ 🔄 Axios interceptor catches 401                │
       │                                                  │
       │ ┌────────────────────────────────────┐          │
       │ │ 1. Queue original request           │          │
       │ │ 2. Check if refresh already running │          │
       │ │ 3. If not, start refresh            │          │
       │ └────────────────────────────────────┘          │
       │                                                  │
       │ POST /api/v1/auth/refresh                       │
       │ [Web] Cookie: refresh_token=yyy                 │
       │ [Native] Body: { refresh_token: "yyy" }         │
       ├─────────────────────────────────────────────────>│
       │                                                  │
       │                                                  │ 1. Extract refresh token
       │                                                  │ 2. Validate JWT signature
       │                                                  │ 3. Check expiration
       │                                                  │ 4. Verify token type = refresh
       │                                                  │ 5. Generate NEW access token
       │                                                  │ 6. Generate NEW refresh token
       │                                                  │
       │ 200 OK                                           │
       │ [Web] Set-Cookie: access_token=new_xxx          │
       │ [Native] Body: { access_token: "new_xxx" }      │
       │<─────────────────────────────────────────────────┤
       │                                                  │
       │ ✅ New access token received                    │
       │ 🔄 Retry all queued requests                    │
       │                                                  │
       │ GET /api/v1/users/123 (RETRY)                   │
       │ Authorization: Bearer <new_access_token>        │
       ├─────────────────────────────────────────────────>│
       │                                                  │
       │                                                  │ ✅ Token valid
       │                                                  │ ✅ Return user data
       │                                                  │
       │ 200 OK                                           │
       │ { id: 123, email: "user@example.com" }         │
       │<─────────────────────────────────────────────────┤
       │                                                  │
```

**Key Features:**
- **Automatic**: User never sees "session expired" errors
- **Request queuing**: Multiple 401s trigger only ONE refresh
- **Transparent**: Original request succeeds after refresh
- **Secure**: Refresh token has longer expiry (7 days vs 15 minutes)

---

## 3️⃣ CSRF Protection - Double Submit Cookie

### How CSRF Attacks Work (Without Protection)

```
┌─────────────┐                  ┌─────────────┐                  ┌─────────────┐
│  Attacker   │                  │   Victim    │                  │   Backend   │
│   Website   │                  │   Browser   │                  │   (Axum)    │
└──────┬──────┘                  └──────┬──────┘                  └──────┬──────┘
       │                                │                                │
       │ User visits attacker.com       │                                │
       │ <form action="yourapp.com/transfer" method="POST">             │
       │   <input name="to" value="attacker">                           │
       │   <input name="amount" value="1000">                           │
       │ </form>                        │                                │
       │ <script>document.forms[0].submit()</script>                    │
       ├───────────────────────────────>│                                │
       │                                │                                │
       │                                │ POST /transfer                 │
       │                                │ Cookie: session=valid_cookie   │
       │                                │ Body: { to: "attacker", amount: 1000 }
       │                                ├───────────────────────────────>│
       │                                │                                │
       │                                │                                │ ❌ Accepts request!
       │                                │                                │ ❌ Cookie is valid
       │                                │                                │ ❌ No way to detect
       │                                │                                │    it's from attacker
       │                                │                                │
       │                                │ 200 OK                         │
       │                                │ { success: true }              │
       │                                │<───────────────────────────────┤
       │                                │                                │
       │ 💰 Money transferred!          │                                │
```

---

### CSRF Protection with Double-Submit Cookie

```
┌─────────────┐                  ┌─────────────┐                  ┌─────────────┐
│  Legitimate │                  │   User      │                  │   Backend   │
│   Website   │                  │   Browser   │                  │   (Axum)    │
└──────┬──────┘                  └──────┬──────┘                  └──────┬──────┘
       │                                │                                │
       │ 1. Get CSRF token              │                                │
       │ GET /api/v1/csrf               │                                │
       ├───────────────────────────────>│                                │
       │                                ├───────────────────────────────>│
       │                                │                                │
       │                                │                                │ Generate random token
       │                                │                                │ (32 bytes, hex encoded)
       │                                │                                │
       │                                │ 200 OK                         │
       │                                │ Set-Cookie: csrf_token=abc123  │
       │                                │   (NOT HttpOnly - JS can read) │
       │                                │ Body: { csrf_token: "abc123" } │
       │                                │<───────────────────────────────┤
       │                                │                                │
       │ 2. JavaScript reads token      │                                │
       │    from response body          │                                │
       │                                │                                │
       │ 3. Make state-changing request │                                │
       │ POST /api/v1/transfer          │                                │
       │ Cookie: csrf_token=abc123      │ ← Browser auto-sends          │
       │ X-CSRF-Token: abc123           │ ← JavaScript sets header      │
       │ Body: { to: "friend", amount: 100 }                            │
       ├───────────────────────────────>│                                │
       │                                ├───────────────────────────────>│
       │                                │                                │
       │                                │                                │ 1. Extract cookie: abc123
       │                                │                                │ 2. Extract header: abc123
       │                                │                                │ 3. Compare (constant-time)
       │                                │                                │ 4. ✅ Match! Proceed
       │                                │                                │
       │                                │ 200 OK                         │
       │                                │<───────────────────────────────┤
       │                                │                                │
```

---

### CSRF Attack Blocked

```
┌─────────────┐                  ┌─────────────┐                  ┌─────────────┐
│  Attacker   │                  │   Victim    │                  │   Backend   │
│   Website   │                  │   Browser   │                  │   (Axum)    │
└──────┬──────┘                  └──────┬──────┘                  └──────┬──────┘
       │                                │                                │
       │ Attacker tries CSRF            │                                │
       │ <form action="yourapp.com/transfer">                           │
       │   <input name="to" value="attacker">                           │
       │ </form>                        │                                │
       ├───────────────────────────────>│                                │
       │                                │                                │
       │                                │ POST /transfer                 │
       │                                │ Cookie: csrf_token=abc123      │
       │                                │ X-CSRF-Token: (MISSING!)       │
       │                                ├───────────────────────────────>│
       │                                │                                │
       │                                │                                │ 1. Extract cookie: abc123
       │                                │                                │ 2. Extract header: (none)
       │                                │                                │ 3. ❌ Mismatch!
       │                                │                                │
       │                                │ 403 Forbidden                  │
       │                                │ { error: "CSRF token invalid" }│
       │                                │<───────────────────────────────┤
       │                                │                                │
       │ ❌ Attack blocked!             │                                │
```

**Why This Works:**
1. **Browser auto-sends cookie** - Attacker can trigger this
2. **Browser CANNOT set custom headers** - Same-origin policy prevents this
3. **Attacker cannot read cookie** - Same-origin policy prevents this
4. **Without header, request fails** - Backend rejects mismatched tokens

**Implementation Details:**
- CSRF token is NOT httpOnly (JavaScript must read it)
- Token is random, unpredictable (32 bytes)
- Comparison uses constant-time algorithm (timing attack prevention)
- Native clients skip CSRF (they use Bearer tokens, not cookies)

---

## 4️⃣ Complete Request Flow with Security Layers

```
┌─────────────┐
│   Client    │
│  (Browser/  │
│   Mobile)   │
└──────┬──────┘
       │
       │ HTTP Request
       │ ┌────────────────────────────────────────────────┐
       │ │ GET /api/v1/users/123                          │
       │ │ Authorization: Bearer <token>                  │
       │ │ X-CSRF-Token: abc123                           │
       │ │ Cookie: csrf_token=abc123                      │
       │ └────────────────────────────────────────────────┘
       │
       ▼
┌──────────────────────────────────────────────────────────┐
│                    BACKEND LAYERS                        │
│                                                          │
│  ┌────────────────────────────────────────────────────┐ │
│  │ 1. CORS Layer                                      │ │
│  │    - Check Origin header                           │ │
│  │    - Verify against ALLOWED_ORIGINS                │ │
│  │    - Reject if not allowed                         │ │
│  └────────────────────────────────────────────────────┘ │
│                         │                                │
│                         ▼                                │
│  ┌────────────────────────────────────────────────────┐ │
│  │ 2. Rate Limiting Layer (tower_governor)            │ │
│  │    - Extract client IP                             │ │
│  │    - Check request count                           │ │
│  │    - Auth endpoints: 1 req/sec, burst 5            │ │
│  │    - General endpoints: 50 req/sec, burst 100      │ │
│  │    - Return 429 if exceeded                        │ │
│  └────────────────────────────────────────────────────┘ │
│                         │                                │
│                         ▼                                │
│  ┌────────────────────────────────────────────────────┐ │
│  │ 3. Trace Layer (tower_http)                        │ │
│  │    - Log request method, path, headers             │ │
│  │    - Start timer                                   │ │
│  │    - Continue to next layer                        │ │
│  └────────────────────────────────────────────────────┘ │
│                         │                                │
│                         ▼                                │
│  ┌────────────────────────────────────────────────────┐ │
│  │ 4. CSRF Middleware (custom)                        │ │
│  │    - Skip if GET/HEAD/OPTIONS                      │ │
│  │    - Skip if X-Client-Type: native                 │ │
│  │    - Extract csrf_token from cookie                │ │
│  │    - Extract X-CSRF-Token from header              │ │
│  │    - Compare (constant-time)                       │ │
│  │    - Return 403 if mismatch                        │ │
│  └────────────────────────────────────────────────────┘ │
│                         │                                │
│                         ▼                                │
│  ┌────────────────────────────────────────────────────┐ │
│  │ 5. Route Handler                                   │ │
│  │    - Extract Authorization header                  │ │
│  │    - Validate JWT signature                        │ │
│  │    - Check expiration                              │ │
│  │    - Extract user_id from claims                   │ │
│  │    - Call business logic                           │ │
│  └────────────────────────────────────────────────────┘ │
│                         │                                │
│                         ▼                                │
│  ┌────────────────────────────────────────────────────┐ │
│  │ 6. Database Layer (Diesel + spawn_blocking)        │ │
│  │    - Get connection from pool                      │ │
│  │    - Execute query in blocking thread              │ │
│  │    - Return result                                 │ │
│  └────────────────────────────────────────────────────┘ │
│                         │                                │
│                         ▼                                │
│  ┌────────────────────────────────────────────────────┐ │
│  │ 7. Response                                        │ │
│  │    - Serialize to JSON                             │ │
│  │    - Add headers                                   │ │
│  │    - Log response time                             │ │
│  │    - Return to client                              │ │
│  └────────────────────────────────────────────────────┘ │
│                                                          │
└──────────────────────────────────────────────────────────┘
       │
       │ HTTP Response
       │ ┌────────────────────────────────────────────────┐
       │ │ 200 OK                                         │
       │ │ Content-Type: application/json                 │
       │ │ { id: 123, email: "user@example.com" }        │
       │ └────────────────────────────────────────────────┘
       ▼
┌──────────────┐
│   Client     │
└──────────────┘
```

**Security Layers Explained:**

1. **CORS** - Prevents unauthorized origins from making requests
2. **Rate Limiting** - Prevents brute force and DoS attacks
3. **Trace Layer** - Audit trail for security investigations
4. **CSRF** - Prevents cross-site request forgery
5. **JWT Validation** - Ensures request is authenticated
6. **Database** - Async-safe blocking prevents runtime issues
7. **Response** - Structured error handling, no sensitive data leaks

---

## 🔒 Security Principles

### Defense in Depth
Multiple layers of security ensure that if one layer fails, others protect the system.

### Fail Secure
All security checks default to DENY. Explicit allow is required.

### Least Privilege
Tokens have minimal permissions and short expiry times.

### Audit Trail
All requests are logged for security investigations.

### Constant-Time Operations
CSRF and password comparisons use constant-time algorithms to prevent timing attacks.

---

## 📚 References

- **JWT Best Practices**: RFC 8725
- **CSRF Protection**: OWASP CSRF Prevention Cheat Sheet
- **Argon2**: RFC 9106
- **Rate Limiting**: OWASP API Security Top 10

---

**Last Updated:** January 9, 2026
