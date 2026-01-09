# Contributing to APP_VIRGIN

Thank you for contributing! This document explains how to work with this codebase.

## Quick Start

```bash
# Clone and setup
git clone https://github.com/rover89898989/APP_VIRGIN.git
cd APP_VIRGIN

# Backend
cd backend
cargo check   # Verify Rust compiles
cargo test    # Run 15 tests

# Mobile
cd ../mobile
npm install
npm run type-check  # TypeScript validation
```

## Running the App

**Terminal 1 - Backend:**
```bash
cd backend
cargo run
# Listens on http://localhost:8000
```

**Terminal 2 - Mobile:**
```bash
cd mobile
npx expo start --web
# Opens http://localhost:8081
```

## Code Style

### Rust (Backend)
- Run `cargo fmt` before committing
- Run `cargo clippy` for lints
- All public functions need doc comments
- Use `spawn_blocking` for any Diesel operations

### TypeScript (Mobile)
- Run `npm run lint` before committing
- Run `npm run type-check` to verify types
- Use React Query hooks for data fetching
- Never store tokens in localStorage (use cookies/SecureStore)

## Adding New Features

### Backend Route
1. Create handler in `backend/src/api/` or `backend/src/features/`
2. Add route in `backend/src/api/mod.rs`
3. Add tests in the same file with `#[cfg(test)]`

### Frontend API Call
1. Add types in `mobile/src/api/types/` (or generate from Rust)
2. Add hook in `mobile/src/api/client.ts`
3. Use the hook in your component

## Testing

```bash
# Backend tests
cd backend
cargo test

# Type checking
cd mobile
npm run type-check
```

## Security Guidelines

- **Never** store tokens in localStorage/sessionStorage
- **Always** use httpOnly cookies for web auth
- **Always** use SecureStore for native auth
- **Never** log sensitive data or tokens
- **Always** validate input on backend

## Pull Request Process

1. Create a feature branch
2. Make your changes
3. Run all tests (`cargo test`, `npm run type-check`)
4. Update documentation if needed
5. Submit PR with clear description

## Questions?

See `STATEMENT_OF_SOTA.md` for architecture decisions and security rationale.
