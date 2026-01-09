# Examples

This directory contains working examples that demonstrate the APP_VIRGIN template in action.

## Trivial Example

The **Trivial Example** is a minimal working demonstration that:
- Starts the Rust backend on `http://localhost:8000`
- Runs the Expo web app on `http://localhost:8081`
- Shows "Template Ready" when backend is connected
- Demonstrates the health check flow

### How to Run the Example

1. **Clone APP_VIRGIN to a new directory** (don't modify the template directly):
   ```bash
   # From parent directory of APP_VIRGIN
   robocopy APP_VIRGIN Trivial_Example /E /XD .git node_modules target
   cd Trivial_Example
   git init
   ```

2. **Install dependencies**:
   ```bash
   cd mobile
   npm install
   npx expo install react-dom react-native-web @expo/metro-runtime
   ```

3. **Start backend** (Terminal 1):
   ```bash
   cd backend
   cargo run
   # Should show: backend listening on http://127.0.0.1:8000
   ```

4. **Start mobile web** (Terminal 2):
   ```bash
   cd mobile
   npx expo start --web
   # Opens browser at http://localhost:8081
   ```

5. **Verify**: Browser should show "Template Ready" with green checkmark.

### What the Example Demonstrates

| Feature | How It's Shown |
|---------|----------------|
| **Health Check** | App shows backend status (ok/offline) |
| **React Query** | `useQuery` hook fetches `/health/live` |
| **Platform Detection** | Correct API URL for web/iOS/Android |
| **CORS** | Cross-origin request from 8081 → 8000 |
| **Rate Limiting** | Backend uses tower_governor |

### Files to Study

- `mobile/App.tsx` - Entry point with health check UI
- `mobile/src/api/client.ts` - Type-safe API client with auth
- `backend/src/main.rs` - Server setup with CORS and rate limiting
- `backend/src/api/health.rs` - Health check endpoints
- `backend/src/api/auth.rs` - Cookie-based auth for web

### Security Features Demonstrated

- **httpOnly Cookies** - Web auth tokens immune to XSS
- **SecureStore** - Native auth tokens hardware-backed
- **CORS with Credentials** - Secure cross-origin requests
- **Rate Limiting** - 50 req/sec per IP protection
