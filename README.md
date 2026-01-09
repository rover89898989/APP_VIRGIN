# 🎯 Production-Ready App Template

A **robust, well-organized, reusable monorepo template** for building modern apps with:
- **Expo + TypeScript** mobile app
- **Rust (Axum) backend** (optional)
- **Type-safe API** with auto-generated TypeScript types from Rust
- **SOTA 2026 best practices** with verbose documentation

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

## 🚀 Quick Start

### 1. Install Dependencies

```bash
# From project root
cd mobile && npm install
```

### 2. Start Backend (Terminal 1)

```bash
npm run backend:run
# Backend starts at http://localhost:8000
```

### 3. Start Mobile (Terminal 2)

```bash
npm run mobile:web   # Web browser
# OR
npm run mobile:start # Expo Go (iOS/Android)
```

### 4. Verify

- Mobile shows "Template Ready" with green checkmark
- Backend health: `curl http://localhost:8000/health/live`

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

### ✅ Security-Hardened

- **XSS-Immune Web Auth** - httpOnly cookies for web (JavaScript cannot steal tokens)
- **Secure Native Storage** - `expo-secure-store` for iOS/Android (hardware-backed)
- **Rate limiting** - 50 req/sec per IP with burst protection
- **CORS with credentials** - Prevents unauthorized cross-origin requests
- **Platform-aware auth** - Different secure mechanisms for web vs native

### ✅ Production-Ready

- **Health checks** (honest, not fake)
- **Error handling** (graceful, user-friendly)
- **Clean Architecture** (testable, maintainable)
- **Feature-first organization** (easy scaling)

---

## 📚 Documentation

- **`examples/`** - Working examples demonstrating the template
- **`STATEMENT_OF_SOTA.md`** - Detailed rationale ("why") for all architecture/security choices, with ASCII diagrams
- **`BUILD_FILE.md`** - Complete architecture explanation, framework decisions, and ASCII diagrams
- **`docs/`** - Original documentation archive (preserved verbatim)
  - `EXECUTIVE_SUMMARY.md` - Complete overview and audit results
  - `GEMINI_CRITICAL_FIXES.md` - Critical issues found and fixed
  - `GEMINI_FINAL_FIXES.md` - Additional improvements
  - `LOGIC_TREE_SCHEMATIC.md` - Architecture deep dive

---

## 🎯 Usage

### For New Projects

1. **Copy this template** to your new project folder
2. **Run `npm install`** in `mobile/`
3. **Run `cargo check`** in `backend/` (if using backend)
4. **Customize** in `mobile/src/` and `backend/src/`

### Type Generation Workflow

1. **Define types in Rust** (`backend/src/features/users/domain/entities.rs`)
2. **Run `cargo test`** in `backend/`
3. **Types auto-generate** in `mobile/src/api/types/`
4. **Use in frontend** with full TypeScript safety

---

## 🏆 Why This Template

- **Robust**: Compiles cleanly, type-safe, production-hardened
- **Safe**: Security audited, A- grade, 0 critical vulnerabilities
- **Maintainable**: Clean architecture, feature-first, well-documented
- **Reusable**: Copy-paste ready for every new app
- **Educational**: Every decision explained in verbose comments

---

## 📞 Support

- **SOTA Proof**: See `STATEMENT_OF_SOTA.md` for the full “why” behind the template (security posture, tradeoffs, and diagrams)
- **Architecture**: See `BUILD_FILE.md` for complete system explanation
- **Docs**: See `docs/` for detailed explanations
- **Issues**: Check `EXECUTIVE_SUMMARY.md` for known fixes
- **Deep Dive**: See `LOGIC_TREE_SCHEMATIC.md` for architecture diagrams

---

## 🎉 Deploy with Confidence

This template is **production-ready** and **security-audited**. Build your next app on a solid foundation. 🚀
