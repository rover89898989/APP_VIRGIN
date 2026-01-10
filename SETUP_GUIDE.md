# 🚀 Setup Guide - APP_VIRGIN Template

Complete setup instructions for getting the template running locally.

---

## 📋 Prerequisites

- **Node.js** 18+ and npm
- **Rust** 1.70+ and Cargo
- **PostgreSQL** 14+ (optional - only if using database features)
- **Diesel CLI** (optional - for database migrations)

```bash
# Install Diesel CLI (if using database)
cargo install diesel_cli --no-default-features --features postgres
```

---

## ⚡ Quick Start (5 minutes)

### 1. Clone and Install

```bash
# Clone the template
git clone https://github.com/YOUR_USERNAME/APP_VIRGIN.git my-app
cd my-app

# Install mobile dependencies
cd mobile
npm install
cd ..
```

### 2. Configure Environment

```bash
# Backend configuration
cd backend
cp .env.example .env
# Edit .env and set JWT_SECRET (required)
cd ..

# Mobile configuration
cd mobile
cp .env.example .env
# Edit .env if needed (defaults work for local dev)
cd ..
```

### 3. Start Backend (Terminal 1)

```bash
cd backend
cargo run
# Backend starts at http://localhost:8000
```

### 4. Start Mobile (Terminal 2)

```bash
cd mobile
npm start
# Press 'w' for web browser
# Or scan QR code with Expo Go app
```

### 5. Verify

- Mobile app shows: 🔥 12 Day Streak demo
- Backend health: `curl http://localhost:8000/health/live`
- Should return: `{"status":"ok"}`

---

## 🔧 Detailed Setup

### Backend Configuration

#### Required Environment Variables

Edit `backend/.env`:

```bash
# REQUIRED: Generate with: openssl rand -base64 32
JWT_SECRET=your-32-byte-random-secret-here

# REQUIRED: Set to production when deploying
ENVIRONMENT=development

# REQUIRED: Add your frontend URLs
ALLOWED_ORIGINS=http://localhost:8081,http://localhost:19006
```

#### Optional: Database Setup

If using database features:

```bash
# 1. Create PostgreSQL database
createdb myapp

# 2. Set DATABASE_URL in .env
DATABASE_URL=postgres://postgres:password@localhost:5432/myapp

# 3. Run migrations
cd backend
diesel migration run
```

### Mobile Configuration

Edit `mobile/.env`:

```bash
# Backend API URL
EXPO_PUBLIC_API_URL=http://localhost:8000

# For Android Emulator
# EXPO_PUBLIC_API_URL=http://10.0.2.2:8000

# For physical devices on same network
# EXPO_PUBLIC_API_URL=http://YOUR_IP:8000
```

---

## ✅ Verification Commands

### Backend

```bash
cd backend

# Check compilation
cargo check

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run with logging
RUST_LOG=debug cargo run
```

### Mobile

```bash
cd mobile

# Type checking
npm run type-check

# Linting
npm run lint

# Format code
npm run format
```

---

## 🔐 Security Checklist

Before deploying to production:

### Backend
- [ ] Set `ENVIRONMENT=production` in `.env`
- [ ] Generate secure `JWT_SECRET` (32+ bytes)
- [ ] Update `ALLOWED_ORIGINS` with production domains
- [ ] Enable database SSL if using PostgreSQL
- [ ] Run migrations: `diesel migration run`
- [ ] Set up reverse proxy (nginx/Caddy) for HTTPS

### Mobile
- [ ] Update `EXPO_PUBLIC_API_URL` to production API
- [ ] Add Sentry DSN for error tracking
- [ ] Test on physical devices
- [ ] Build production bundle: `eas build`

---

## 🐛 Troubleshooting

### Development Environment Issues

**Node.js Version Compatibility**
```bash
# Issue: Node.js v24+ causes Expo CLI errors
# Symptoms: "Body has already been read", bundler stuck

# Check your Node.js version:
node --version

# Recommended versions:
# - Node.js 18 LTS (most stable with Expo)
# - Node.js 20 LTS (good compatibility)
# - Node.js 22 (mostly works, some issues)
# - Node.js 24 (avoid for now - compatibility issues)

# Switch versions using nvm (recommended):
nvm install 18
nvm use 18
npm install
npx expo start --web
```

**PowerShell Command Issues**
```bash
# Issue: PowerShell doesn't support && operator
# Error: "The token '&&' is not a valid statement separator"

# Instead of:
cd "path" && npm run web

# Use separate commands:
cd "path"
npm run web

# Or use semicolon:
cd "path"; npm run web
```

**Port Already in Use**
```bash
# Issue: Metro bundler port 8081 already occupied
# Error: "Metro bundler port in use"

# Solution 1: Kill process on port 8081
netstat -ano | findstr :8081
taskkill /F /PID <PID_FROM_PREVIOUS_COMMAND>

# Solution 2: Use different port
npx expo start --web --port 8082

# Solution 3: Clear and restart (most reliable)
npx expo start --web --clear
```

**Package Installation Issues**
```bash
# Issue: Corrupted node_modules or lock file
# Symptoms: Various import errors, missing modules

# Complete clean install:
cd mobile
rm -rf node_modules
rm package-lock.json
npm cache clean --force
npm install

# If still issues, check for platform-specific packages:
npm install --platform=win32
```

### Backend won't start

**Error: "JWT_SECRET not set"**
```bash
# Solution: Set in backend/.env
JWT_SECRET=$(openssl rand -base64 32)
```

**Error: "Database connection failed"**
```bash
# Solution: Either disable database or fix connection
# Option 1: Disable database
DATABASE_REQUIRED=false

# Option 2: Fix PostgreSQL connection
# Check PostgreSQL is running:
pg_isready
```

### Mobile won't build

**Error: "Cannot find module 'babel-preset-expo'"**
```bash
# Solution: Clean install
cd mobile
rm -rf node_modules
npm install
```

**Error: "Metro bundler port in use"**
```bash
# Solution: Kill existing process
npx expo start --clear
```

**CRITICAL: Node.js v24 Compatibility Issue**
```bash
# Symptoms:
# - "Body has already been read" error
# - Bundler stuck at 85.2%
# - babel-preset-expo version mismatch warning
# - Blank white screen in browser

# Solution 1: Clear cache (recommended first)
cd mobile
npx expo start --web --clear

# Solution 2: Update babel-preset-expo (if Solution 1 fails)
npm install babel-preset-expo@~54.0.9

# Solution 3: Use Node.js 18-22 (if all else fails)
# Install Node.js 18 LTS using nvm or official installer
nvm install 18
nvm use 18
npm install
npx expo start --web

# Verification:
# Should see "Metro waiting on exp://127.0.0.1:8081"
# Web bundling should reach 100%
# Browser should show 🔥 12 Day Streak demo
```

**Error: "Expo CLI compatibility warning"**
```bash
# Warning: "The following packages should be updated"
# Usually safe to ignore during development
# If needed, update specific package:
npm install babel-preset-expo@~54.0.9
```

**Error: Blank white screen after bundling completes**
```bash
# Symptoms:
# - Bundling reaches 100%
# - Browser opens but shows blank screen
# - No errors in console

# Solutions:
# 1. Check browser console (F12) for JavaScript errors
# 2. Clear browser cache and hard refresh (Ctrl+F5)
# 3. Restart with clear cache:
npx expo start --web --clear
# 4. Check if ErrorBoundary caught an error (should show error UI)
# 5. Verify all imports in App.tsx are correct
```

### API requests fail

**CORS errors in browser console**
```bash
# Solution: Add frontend URL to ALLOWED_ORIGINS in backend/.env
ALLOWED_ORIGINS=http://localhost:8081,http://YOUR_URL
```

**401 Unauthorized errors**
```bash
# Solution: Get CSRF token first
curl http://localhost:8000/api/v1/csrf
# Then include X-CSRF-Token header in requests
```

---

## 🔍 Quick Diagnostic Commands

When something goes wrong, run these commands first:

```bash
# 1. Check environment versions
node --version
npm --version
rustc --version
cargo --version

# 2. Check if ports are in use
netstat -ano | findstr :8081  # Expo web
netstat -ano | findstr :8000  # Backend

# 3. Test backend health
curl http://localhost:8000/health/live

# 4. Check package versions
cd mobile
npm list expo
npm list babel-preset-expo

# 5. Clear all caches (nuclear option)
cd mobile
npx expo start --web --clear
npm cache clean --force
rm -rf .expo
```

**Expected Healthy State:**
- Node.js: 18.x, 20.x, or 22.x (avoid 24.x for now)
- Expo web: `http://localhost:8081` shows 🔥 12 Day Streak
- Backend: `curl http://localhost:8000/health/live` returns `{"status":"ok"}`
- No port conflicts
- Bundling reaches 100% without errors

---

## 📚 Next Steps

1. **Read the docs**: Check `STATEMENT_OF_SOTA.md` for architecture details
2. **Explore examples**: See `examples/` for working code samples
3. **Customize**: Start building in `mobile/src/` and `backend/src/features/`
4. **Deploy**: Follow deployment guides in `docs/`

---

## 🎯 Development Workflow

### Adding a New Feature

1. **Backend**: Create feature module in `backend/src/features/your_feature/`
2. **Define types**: Add Rust structs with `#[ts(export)]` in `domain/entities.rs`
3. **Generate types**: Run `cargo test` to generate TypeScript types
4. **Frontend**: Use generated types from `mobile/src/api/types/`
5. **Test**: Write tests for both backend and frontend

### Type Generation Pipeline

```bash
# 1. Define Rust types
# backend/src/features/users/domain/entities.rs
#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct User {
    pub id: i64,
    pub email: String,
}

# 2. Generate TypeScript types
cd backend
cargo test generate_typescript_types

# 3. Sync to mobile (optional script)
./scripts/sync-types.ps1

# 4. Use in frontend
import { User } from './api/types/User';
```

---

## 🔄 Keeping Template Updated

If the template receives updates:

```bash
cd my-app
git remote add template https://github.com/YOUR_USERNAME/APP_VIRGIN.git
git fetch template
git merge template/main
# Resolve any conflicts
```

---

## 💡 Tips

- **Hot reload**: Both backend (with `cargo-watch`) and mobile support hot reload
- **Debugging**: Use `RUST_LOG=debug` for verbose backend logs
- **Testing**: Run `cargo test` and `npm test` frequently
- **Type safety**: Let TypeScript catch errors before runtime

---

## 📞 Support

- **Issues**: Check `PRODUCTION_REVIEW.md` for known issues
- **Architecture**: See `BUILD_FILE.md` for system design
- **Security**: See `STATEMENT_OF_SOTA.md` for security details

---

**Ready to build! 🚀**
