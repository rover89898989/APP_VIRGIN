# APP_VIRGIN Template - Improvement Roadmap

**Last Updated**: 2026-01-09

## Priority Order (High → Low)

### P1: Critical for Developer Experience

- [x] **1. Android Emulator URL Handling** ✅
  - API client uses `localhost:8000` which doesn't work on Android Emulator
  - Android Emulator requires `10.0.2.2:8000`
  - Impact: Android devs blocked immediately on first run
  - Fix: Added Platform detection in `getApiBaseUrl()` function

- [x] **2. Add Working Integration Test** ✅
  - Added 3 tests for health endpoints in `backend/src/api/health.rs`
  - Tests: `/health/live` returns 200, `/health/ready` returns 200 (no DB), 503 (DB required but missing)
  - Run with: `cargo test`

### P2: Code Quality

- [ ] **3. Clean Up Unused Code Warnings**
  - Repository stubs trigger 20+ warnings
  - Prefix unused params with `_` or use `#[allow(dead_code)]`
  - Impact: Noisy build output obscures real issues

- [ ] **4. Add .env.example File**
  - Document expected environment variables
  - `BACKEND_HOST`, `BACKEND_PORT`, `DATABASE_URL`, `DATABASE_REQUIRED`
  - Impact: New devs don't know what to configure

### P3: Documentation

- [ ] **5. Document Rate Limiter Configuration**
  - `into_make_service_with_connect_info::<SocketAddr>()` is required
  - Not obvious from tower_governor docs
  - Impact: We debugged this for 10+ minutes

- [ ] **6. Add CONTRIBUTING.md**
  - How to run tests
  - How to add new features
  - Code style expectations

### P4: Future Enhancements (Not Blocking)

- [ ] **7. Add ESLint/Prettier Config for Mobile**
  - Currently just echoes "not configured"
  - Impact: Code style inconsistencies over time

- [ ] **8. Add Refresh Token Support (Optional)**
  - Currently uses short-lived tokens only
  - Better UX for long sessions
  - Impact: Users re-login frequently on web

---

## Completed

- [x] Fix duplicate health routes in api/mod.rs
- [x] Add UserResponse struct to Rust entities
- [x] Wire up App.tsx with QueryClientProvider and health check
- [x] Fix CORS: add allow_headers for browser requests
- [x] Fix rate limiter: use into_make_service_with_connect_info
- [x] **Android Emulator URL Handling** - Platform-aware `getApiBaseUrl()`
- [x] **Integration Tests** - 3 health endpoint tests + 6 auth tests
- [x] **XSS-Immune Web Auth** - httpOnly cookies for web, SecureStore for native
  - Backend: `/api/v1/auth/login` and `/api/v1/auth/logout` endpoints
  - Backend: CORS with credentials and specific origins
  - Frontend: `withCredentials: true` on axios
  - Frontend: Platform-aware TokenStorage (no localStorage for web)

---

## Session Notes

### 2026-01-09
- Template validated end-to-end (backend + mobile web)
- Found and fixed: CORS, rate limiter, duplicate routes
- Created Trivial_Example_Template for testing
- Critical assessment completed
