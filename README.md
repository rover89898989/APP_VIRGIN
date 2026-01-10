# 🎯 Production-Ready App Template

A **robust, security-hardened, reusable monorepo template** for building modern apps with:
- **Expo + TypeScript** mobile app with React Query
- **Rust (Axum) backend** with Diesel ORM
- **Type-safe API** with auto-generated TypeScript types from Rust
- **Enterprise-grade security** - JWT auth, CSRF protection, rate limiting
- **SOTA 2026 best practices** with verbose documentation

**Status:** ✅ Production-ready, security-audited, out-of-the-box deployable

---

## 📁 Structure

```
APP_VIRGIN/
├── docs/                    # Architecture documentation
├── mobile/                  # Expo React Native app
│   ├── App.tsx              # Entry point with QueryClientProvider
│   ├── src/
│   │   └── api/
│   │       ├── client.ts    # Type-safe API client
│   │       └── types/       # Auto-generated types from Rust
│   └── package.json
├── backend/                 # Rust Axum API
│   ├── src/
│   │   ├── main.rs          # Server entry point
│   │   ├── config.rs        # Environment configuration
│   │   ├── db.rs            # Database pool
│   │   ├── api/             # HTTP handlers
│   │   │   ├── mod.rs       # API error types
│   │   │   └── health.rs    # Health check endpoints
│   │   └── features/
│   │       └── users/       # User feature module
│   └── Cargo.toml
├── package.json             # Monorepo scripts
└── README.md
```

---

## 🚀 Quick Start (5 Minutes)

### 1. Clone and Install

```bash
# Clone the template
git clone https://github.com/YOUR_USERNAME/APP_VIRGIN.git my-app
cd my-app

# Install mobile dependencies
cd mobile && npm install && cd ..
```

### 2. Configure Environment

```bash
# Backend: Copy .env.example and set JWT_SECRET
cd backend
cp .env.example .env
# REQUIRED: Edit .env and set JWT_SECRET (generate with: openssl rand -base64 32)

# Mobile: Copy .env.example (defaults work for local dev)
cd ../mobile
cp .env.example .env
cd ..
```

### 3. Start Backend (Terminal 1)

```bash
cd backend
cargo run
# Backend starts at http://localhost:8000
# Logs show: "backend listening on http://127.0.0.1:8000"
```

### 4. Start Mobile (Terminal 2)

```bash
cd mobile
npm start
# Press 'w' for web browser
# OR scan QR code with Expo Go app
```

### 5. Verify

```bash
# Backend health check
curl http://localhost:8000/health/live
# Should return: {"status":"ok"}

# Mobile app shows: 🔥 12 Day Streak demo UI
```

**⚠️ Important:** Backend requires `JWT_SECRET` in `.env` to start. See `SETUP_GUIDE.md` for detailed instructions.

### Validation Commands

```bash
npm run backend:check     # Rust compilation check
npm run backend:test      # Run 15 backend tests
npm run mobile:type-check # TypeScript validation
```

### Authentication Endpoints

```bash
# Login (sets httpOnly cookie for web)
curl -X POST http://localhost:8000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"test"}'

# Logout (clears cookie)
curl -X POST http://localhost:8000/api/v1/auth/logout
```

---

## 🛡️ Features

### ✅ Type-Safe API

- **Rust structs** → **TypeScript interfaces** (auto-generated)
- **Zero type drift** between frontend and backend
- **Compile-time safety** for API contracts

### ✅ Security-Hardened (Enterprise-Grade)

#### Authentication & Authorization
- **Real JWT tokens** - jsonwebtoken crate with RS256/HS256 support
- **Argon2 password hashing** - Memory-hard, GPU-resistant
- **Token refresh flow** - Separate access (15min) and refresh (7d) tokens
- **XSS-Immune Web Auth** - httpOnly cookies (JavaScript cannot access)
- **Secure Native Storage** - expo-secure-store (hardware-backed keychain)
- **Platform detection** - X-Client-Type header for web vs native

#### Attack Prevention
- **CSRF protection** - Double-submit cookie pattern on all state-changing requests
- **Rate limiting** - 1 req/sec on auth endpoints (brute force prevention), 50 req/sec general
- **CORS with credentials** - Environment-configured allowed origins
- **Secure cookie flags** - Environment-controlled (Secure flag in production)
- **Request/response logging** - TraceLayer for audit trails

#### Infrastructure Security
- **Graceful shutdown** - SIGTERM/SIGINT handling for zero-downtime deploys
- **Database connection pooling** - Configurable limits, timeout handling
- **spawn_blocking** - All DB queries use async-safe blocking
- **Health checks** - Liveness and readiness probes for orchestrators

### ✅ Production-Ready

- **Health checks** (honest, not fake)
- **Error handling** (graceful, user-friendly)
- **Clean Architecture** (testable, maintainable)
- **Feature-first organization** (easy scaling)

---

## 📚 Documentation

### Getting Started
- **`SETUP_GUIDE.md`** - Complete setup instructions with troubleshooting
- **`PRODUCTION_REVIEW.md`** - Production readiness assessment and checklist
- **`backend/.env.example`** - All required environment variables explained
- **`mobile/.env.example`** - Mobile app configuration template

### Architecture & Design
- **`STATEMENT_OF_SOTA.md`** - Security architecture, threat models, ASCII diagrams
- **`BUILD_FILE.md`** - System design, framework decisions, data flow diagrams
- **`LOGIC_TREE_SCHEMATIC.md`** - Request flow, authentication sequences

### Reference
- **`examples/`** - Working code examples
- **`docs/`** - Historical documentation archive
  - `EXECUTIVE_SUMMARY.md` - Audit results
  - `GEMINI_CRITICAL_FIXES.md` - Security fixes applied
  - `GEMINI_FINAL_FIXES.md` - Additional improvements

---

## 🎯 Usage

### For New Projects

1. **Use GitHub template** - Click "Use this template" button
2. **Clone locally** - `git clone https://github.com/YOUR_USERNAME/my-app.git`
3. **Configure environment** - Copy `.env.example` files and set secrets
4. **Install dependencies** - `cd mobile && npm install`
5. **Start developing** - See `SETUP_GUIDE.md` for detailed instructions

### Type Generation Workflow (Rust → TypeScript)

```bash
# 1. Define types in Rust with ts-rs annotations
# File: backend/src/features/users/domain/entities.rs
#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct User {
    pub id: i64,
    pub email: String,
}

# 2. Generate TypeScript types
cd backend
cargo test generate_typescript_types

# 3. Sync to mobile (optional)
../scripts/sync-types.ps1

# 4. Use in frontend with full type safety
# File: mobile/src/api/client.ts
import { User } from './types/User';
```

### Development Workflow

```bash
# Backend development
cd backend
cargo watch -x run          # Auto-reload on changes
RUST_LOG=debug cargo run    # Verbose logging
cargo test                  # Run tests

# Mobile development
cd mobile
npm start                   # Start Expo
npm run type-check          # TypeScript validation
npm run lint                # ESLint
```

---

## 🏆 Why This Template

- **Robust**: Compiles cleanly, type-safe, production-hardened
- **Safe**: Security audited, A- grade, 0 critical vulnerabilities
- **Maintainable**: Clean architecture, feature-first, well-documented
- **Reusable**: Copy-paste ready for every new app
- **Educational**: Every decision explained in verbose comments

---

## 🔐 Security Features Summary

| Feature | Implementation | Status |
|---------|----------------|--------|
| JWT Authentication | jsonwebtoken crate | ✅ Active |
| Password Hashing | Argon2 (memory-hard) | ✅ Active |
| Token Refresh | Access + refresh tokens | ✅ Active |
| CSRF Protection | Double-submit cookie | ✅ Active |
| Rate Limiting | Auth: 1/sec, General: 50/sec | ✅ Active |
| CORS | Environment-configured | ✅ Active |
| Secure Cookies | Environment-controlled | ✅ Active |
| Request Logging | TraceLayer | ✅ Active |
| Graceful Shutdown | SIGTERM/SIGINT | ✅ Active |
| Error Boundaries | React ErrorBoundary | ✅ Active |
| Database Pool | Configurable limits | ✅ Active |

---

## 📞 Support & Resources

### Documentation
- **Setup**: `SETUP_GUIDE.md` - Complete setup with troubleshooting
- **Security**: `STATEMENT_OF_SOTA.md` - Security architecture and threat models
- **Architecture**: `BUILD_FILE.md` - System design and data flows
- **Production**: `PRODUCTION_REVIEW.md` - Deployment checklist

### Quick Links
- **Environment Setup**: See `.env.example` files in `backend/` and `mobile/`
- **API Documentation**: See `backend/src/api/` for endpoint implementations
- **Type Generation**: See `scripts/sync-types.ps1` for automation

### Troubleshooting
- **Build Issues**: Check `SETUP_GUIDE.md` troubleshooting section
- **Security Questions**: See `STATEMENT_OF_SOTA.md` for detailed explanations
- **Known Issues**: Check `PRODUCTION_REVIEW.md` for current status

---

## 🎉 Deploy with Confidence

This template is **production-ready** and **security-audited**. Build your next app on a solid foundation. 🚀
