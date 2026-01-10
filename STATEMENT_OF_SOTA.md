# STATEMENT OF SOTA (2026)

This document is the **Statement of SOTA** for this template.

It exists to:
- **Prove** (in plain language) why these choices are robust, secure, and maintainable.
- Explain the **principles** and **tradeoffs** that shape the template.
- Provide a shared reference for audits and reviews (especially where **sensitive user data** is involved).

Related document:
- `INFRASTRUCTURE_AND_SCALING_GUIDE.md` (service selection, scaling decision trees, migration paths)
- `FUTURE_IMPROVEMENTS_PARETO.md` (high-impact feature roadmap: Shopify, OAuth, WebSockets, Images)

---

## 1) What “SOTA 2026” means for this template

SOTA here is not “trendy.” It is:

- **Reliable defaults** over cleverness.
- **Security-by-construction** over “remember to do the right thing.”
- **Clear boundaries** over architecture astronautics.
- **Portable development** across Windows/macOS, while staying **mobile-first**.

### Non-negotiable goals

- **Primary platforms**: Android + iOS.
- **Secondary platforms**: Windows/macOS via **Expo Web** (browser runtime).
- **Sensitive data**: private user information is treated with appropriate security measures.
- **Template quality**: predictable, repeatable, boringly reliable.

---

## 2) High-level architecture

This is a monorepo designed for **one product** with two independently runnable parts:

- `mobile/` : Expo (React Native + TypeScript)
- `backend/`: Rust (Axum + Diesel + Postgres)

Root-level `package.json` is intentionally an **orchestrator** so developers can run common tasks without installing conflicting dependencies.

### System diagram

```
                ┌───────────────────────────────────────┐
                │              Developers               │
                │     (Windows / macOS / Linux)         │
                └───────────────────┬───────────────────┘
                                    │
                                    ▼
                      ┌─────────────────────────┐
                      │      Repo Root           │
                      │  package.json (runner)   │
                      └───────────┬─────────────┘
                                  │
                 ┌────────────────┴─────────────────┐
                 │                                  │
                 ▼                                  ▼
     ┌─────────────────────────┐        ┌─────────────────────────┐
     │        mobile/           │        │        backend/          │
     │ Expo + React Native + TS │        │ Rust + Axum + Diesel     │
     │ Android/iOS first        │        │ Postgres optional/req    │
     └───────────┬─────────────┘        └───────────┬─────────────┘
                 │ HTTP/JSON                          │ SQL
                 ▼                                    ▼
     ┌─────────────────────────┐        ┌─────────────────────────┐
     │  API Client + ReactQuery │        │       PostgreSQL         │
     │  Token storage adapter   │        │   (Encrypted at rest*)   │
     └─────────────────────────┘        └─────────────────────────┘

     * Typically provided by the hosting platform (managed Postgres / disk encryption).
```

---

## 3) Why Expo (React Native) for 2026 mobile-first templates

### Why Expo

- **Fast onboarding**: fewer native build edge-cases.
- **Cross-platform dev ergonomics**: consistent tooling across Windows/macOS.
- **One codebase**: Android + iOS first, and a web runtime for desktop.
- **Mature ecosystem**: common device capabilities have stable packages.

### Desktop requirement without needless complexity

We intentionally support Windows/macOS via:

- **Expo Web** (`npm run web`) → runs as a web app in the browser.

We do **not** default to packaging native desktop apps (Electron/Tauri/RN-Windows/RN-macOS) because:

- It increases CI matrix size and native edge-cases.
- It increases long-term maintenance burden.
- Most “desktop support” requirements are satisfied by a web runtime.

This is a deliberate SOTA tradeoff:

- **Default path**: web runtime (robust, low complexity)
- **Upgrade path**: add packaging later only when it pays rent

---

## 4) Why TypeScript end-to-end

TypeScript is chosen because:

- **Refactor safety** at scale.
- **API contract clarity** between app layers.
- **Tooling & ergonomics** in 2026 are excellent (autocomplete, static analysis).

For a template, the ability for new developers to safely change code is more valuable than micro-optimizations.

---

## 5) Data fetching, caching, and correctness

### Why React Query (@tanstack/react-query)

The template uses React Query because it enforces a correct data lifecycle:

- deduping requests
- caching
- background refresh
- consistent loading/error states

### The “Hook-Only for Reads” rule

Reads (GET) should be exposed via hooks only.

- Reduces “cache bypass” mistakes.
- Makes UI state consistent across screens.

Mutations are exposed as imperative functions (or mutation hooks), because they do not represent cached state.

### Data flow diagram

```
 UI Component
    │
    ▼
 React Query Hook (useX)
    │
    ▼
 API Client (axios)
    │   adds Authorization header (token adapter)
    ▼
 Backend HTTP Handler (Axum)
    │
    ├── validate input + auth
    │
    └── call infrastructure
          │
          ▼
        Diesel (blocking)  ──>  spawn_blocking  ──>  Postgres
```

---

## 6) Backend: why Rust + Axum + Diesel (and how we keep it safe)

### Why Rust

- **Memory safety** without GC.
- **Predictable performance**.
- **Strong type system** that prevents entire classes of bugs.
- Excellent fit for high-integrity code where sensitive data is involved.

### Why Axum

- Async-first, simple mental model.
- Strong ecosystem and middleware patterns.
- Minimal surface area compared to heavier frameworks.

### Why Diesel (with explicit async boundary)

Diesel is synchronous. That is not a problem if we treat it as a boundary:

- **Rule**: All Diesel work occurs in `spawn_blocking`.
- **Benefit**: Tokio runtime remains responsive under load.

This is a reliability choice. It avoids the most common failure mode:

- “DB is slow → entire async server hangs.”

### Honest health checks

We implement:

- `/health/live` : process is alive
- `/health/ready`: safe to receive traffic
  - returns **503** when DB is required and missing/down

Health check design is part of SOTA because:

- it prevents load balancers from routing traffic to a broken instance
- it is observable and automatable

#### Health check decision diagram

```
              /health/ready
                    │
                    ▼
           Is DB required?
            ├───────────────┐
            │               │
           yes             no
            │               │
            ▼               ▼
     Is DB configured?     200 OK
        ├───────┐
        │       │
       no      yes
        │       │
        ▼       ▼
     503      DB probe
             ├───────┐
             │       │
            ok      fail
             │       │
             ▼       ▼
           200      503
```

---

## 7) Sensitive data posture (security by construction)

This template assumes that user data may be sensitive and requires protection.

### Core security principle

**Sensitive data must never be accidentally leaked by default.**

That means:

- **No sensitive data in logs** (client or server).
- **No sensitive data persisted client-side by default**.
- **Transport security** is mandatory in production.
- **Explicit boundaries** for storage, network, and database.

### Data classification model

- **Public**: app version, non-sensitive UI strings
- **Internal**: account metadata, preferences
- **Sensitive**: private user information, personal data, credentials

### Sensitive data flow (ideal)

```
 User enters private data
          │
          ▼
  Mobile UI state (in-memory)
          │
          ▼
  POST /api/v1/resource  (TLS)
          │
          ▼
  Backend validates + stores
          │
          ▼
  DB (encrypted at rest via platform)
          │
          ▼
  Client reads via hooks (no persistence by default)
```

### Token storage policy (XSS-immune by design)

**UPDATE 2026-01-09**: We eliminated the XSS vulnerability in web token storage.

#### The Problem with localStorage

The original approach stored tokens in `localStorage` for web:
```javascript
// ❌ VULNERABLE: Any XSS can steal this
localStorage.setItem('access_token', token);
```

If an attacker injects JavaScript (via XSS), they can:
1. Read the token: `localStorage.getItem('access_token')`
2. Send it to their server
3. Impersonate the user indefinitely

#### The Solution: httpOnly Cookies

We now use **httpOnly cookies** for web authentication:

```
Set-Cookie: access_token=<token>; HttpOnly; SameSite=Lax; Path=/; Max-Age=3600
```

This is **immune to XSS** because:
- `HttpOnly` flag prevents JavaScript from reading the cookie
- Browser automatically sends cookie with requests
- Even if XSS occurs, attacker cannot extract the token

#### Platform-Specific Security

| Platform | Storage | Security Level | XSS Risk |
|----------|---------|----------------|----------|
| **iOS** | Keychain (SecureStore) | Hardware-backed | None (no JS) |
| **Android** | KeyStore (SecureStore) | Hardware-backed | None (no JS) |
| **Web** | httpOnly Cookie | Browser-managed | **Immune** |

#### Implementation Details

**Backend** (`backend/src/api/auth.rs`):
- `/api/v1/auth/login` - Sets httpOnly cookie
- `/api/v1/auth/logout` - Clears cookie (Max-Age=0)
- `extract_token_from_request()` - Checks cookie OR Bearer header

**Frontend** (`mobile/src/api/client.ts`):
- `withCredentials: true` on axios (sends cookies cross-origin)
- `TokenStorage` returns null for web (tokens managed by browser)
- Native apps still use SecureStore + Authorization header

#### Why This is SOTA 2026

- **Industry standard** for web authentication
- **Zero JavaScript token access** eliminates entire attack class
- **No localStorage/sessionStorage** for sensitive data
- **Backwards compatible** with native apps (Bearer header still works)

#### CSRF Protection

Cross-Site Request Forgery (CSRF) is mitigated by our cookie configuration:

```
Set-Cookie: access_token=<token>; HttpOnly; SameSite=Lax; Path=/; Max-Age=3600
```

**`SameSite=Lax`** provides protection by:
- Blocking cookies on cross-origin POST/PUT/DELETE requests
- Only allowing cookies on same-site requests or top-level navigation GET
- Preventing malicious sites from making authenticated requests

**When to upgrade to CSRF tokens:**
- If you need `SameSite=None` (third-party cookie scenarios)
- If you support very old browsers without SameSite support
- If your security audit specifically requires explicit CSRF tokens

**Upgrade path** (if needed):
1. Generate CSRF token on login, store in separate non-httpOnly cookie
2. Frontend reads CSRF cookie, sends in `X-CSRF-Token` header
3. Backend validates header matches cookie on state-changing requests

---

## 8) Operational safety and reliability

### Fail-fast configuration

Startup validates config and exits with a clear error when unsafe to run.

This prevents:

- “server runs but is misconfigured”
- “health checks lie”

### Structured errors

We define a canonical `ApiError` contract:

- stable status codes
- JSON bodies
- client-safe messages (no secrets, no sensitive data)

This improves:

- client reliability (predictable handling)
- auditability
- maintainability

### Minimal observability stance

SOTA does not require a massive monitoring stack in the template.

It requires:

- deterministic startup behavior
- honest health checks
- consistent error surfaces
- redaction policy for logs

You can add Sentry/OTel later, but you should not need them to have a safe baseline.

---

## 9) Maintainability principles (how we keep this clean)

### Principle: enforce good patterns by making bad ones difficult

- Root orchestrator prevents wrong installs.
- Hook-only reads prevent cache bypass.
- Config module prevents env sprawl.
- Health checks make “readiness” explicit.

### Principle: comments explain “why”, not “what”

This repository prefers **contract comments**:

- purpose
- invariants
- failure modes
- security notes

Over time, this reduces stale comment rot while keeping the “why” durable.

---

## 10) Known tradeoffs (explicitly acknowledged)

- **Short-lived tokens only**: No refresh tokens by default.
  - Users re-login after 1 hour (configurable in `ACCESS_TOKEN_MAX_AGE_SECONDS`)
  - Reduces complexity and attack surface
  - Upgrade path documented below
- **Diesel synchronous**: requires `spawn_blocking` discipline.
  - We enforce via code patterns and examples.

SOTA is not pretending these don't exist; it documents them and contains them.

### Refresh Token Upgrade Path

If you need longer sessions without re-login, implement refresh tokens:

**Backend changes:**
1. Generate refresh token on login (longer TTL, e.g., 7 days)
2. Store refresh token hash in database (for revocation)
3. Add `/api/v1/auth/refresh` endpoint that:
   - Validates refresh token
   - Issues new access token
   - Optionally rotates refresh token

**Frontend changes:**
1. Store refresh token in httpOnly cookie (separate from access token)
2. On 401 response, call `/auth/refresh` before retrying
3. If refresh fails, redirect to login

**Security considerations:**
- Refresh tokens should be rotated on each use
- Store only hashed refresh tokens in database
- Implement token revocation for logout/password change

---

## 11) “Prove it” checklist (what should always remain true)

- `cargo check` succeeds.
- Backend serves:
  - `/health/live` returns 200.
  - `/health/ready` returns:
    - 200 when safe
    - 503 when DB is required but missing/down
- Mobile app runs:
  - Android, iOS
  - Web runtime for Windows/macOS
- No sensitive data appears in:
  - logs
  - client persistent storage by default

---

## 12) Upgrade paths (only when needed)

- **Desktop packaging**: Electron/Tauri wrapper around Expo Web build.
- **Long-lived sessions on web**: introduce refresh tokens via httpOnly cookies.
- **Stronger DB crypto needs**: add envelope encryption for specific sensitive columns.

These are intentionally not defaults because they add complexity.

---

## Bottom line

This template is designed to be the “best default” in 2026:

- mobile-first
- cross-platform dev ergonomics
- secure handling assumptions for sensitive user data
- explicit, honest runtime health and configuration
- maintainable patterns that prevent common failure modes
