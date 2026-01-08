# 🏗️ BUILD_FILE — Template Architecture & Organization

> **Purpose**: This document explains **why** this template is organized the way it is, **what** each file and folder does, and **how** the pieces fit together to create a robust, maintainable foundation for new apps.

---

## 📋 Table of Contents

1. [Why This Stack?](#why-this-stack)
2. [High-Level Architecture](#high-level-architecture)
3. [File Structure Explained](#file-structure-explained)
4. [ASCII Logic Tree Diagrams](#ascii-logic-tree-diagrams)
5. [Framework Decisions](#framework-decisions)
6. [Type Safety Flow](#type-safety-flow)
7. [Development Workflow](#development-workflow)
8. [Production Considerations](#production-considerations)

---

## 🎯 Why This Stack?

### React Native + Expo (Mobile Frontend)

**Why Expo over Bare React Native?**
- **Faster onboarding**: No native build setup hell
- **Consistent tooling**: Works the same on iOS/Android/Windows/macOS
- **Over-the-air updates**: Push fixes without app store releases
- **Managed dependencies**: Expo handles native module compatibility
- **Security**: `expo-secure-store` provides hardware-backed keychain access

**Why TypeScript?**
- **Type safety**: Catch bugs at compile time, not runtime
- **AI-friendly**: Copilot/GitHub Copilot work better with typed code
- **Self-documenting**: Types serve as living documentation
- **Refactor safety**: Rename with confidence across the entire codebase

**Why React Query (@tanstack/react-query)?**
- **Caching**: Automatic API response caching (critical for mobile)
- **Loading states**: Built-in isLoading, error, data states
- **Optimistic updates**: Instant UI feedback
- **Background refresh**: Data stays fresh without manual polling

### Rust + Axum (Backend)

**Why Rust?**
- **Performance**: Zero-cost abstractions, memory safety without GC
- **Reliability**: Compiler eliminates entire classes of bugs
- **Type generation**: `ts-rs` generates TypeScript types from Rust structs
- **Modern ergonomics**: Excellent error messages, great tooling

**Why Axum?**
- **Async-first**: Built on Tokio, non-blocking by default
- **Type-safe routing**: Compile-time route validation
- **Middleware ecosystem**: CORS, rate limiting, logging
- **Extractors pattern**: Clean separation of concerns

**Why Diesel?**
- **Type-safe queries**: Compile-time SQL validation
- **Migration system**: Schema changes tracked in code
- **Performance**: Compile-time query optimization

---

## 🏛️ High-Level Architecture

```
┌─────────────────┐    HTTP/JSON    ┌─────────────────┐
│   Mobile App    │ ◄──────────────► │   Rust Backend  │
│  (Expo + TS)    │                  │   (Axum + DB)   │
└─────────────────┘                  └─────────────────┘
         │                                   │
         │                                   ▼
         │                          ┌─────────────────┐
         │                          │ PostgreSQL DB  │
         │                          │   (Diesel ORM) │
         │                          └─────────────────┘
         │
         ▼
┌─────────────────┐
│  Device Storage │
│ (SecureStore)    │
│ Hardware-backed │
└─────────────────┘
```

### Data Flow

1. **User Action** → Mobile component calls hook
2. **Hook** → React Query calls API client
3. **API Client** → Adds JWT token, makes HTTP request
4. **Backend** → Validates JWT, processes request
5. **Database** → Diesel executes type-safe query
6. **Response** → Backend returns JSON
7. **Cache** → React Query caches response
8. **UI** → Component re-renders with fresh data

---

## 📁 File Structure Explained

### Root Level

```
SKELETON_STARTER_v3/
├── 📄 package.json          # Legacy: Original mobile dependencies (kept for reference)
├── 📄 README.md             # Quick start guide
├── 📄 BUILD_FILE.md         # THIS FILE: Architecture explanation
├── 📂 docs/                 # All original documentation (preserved verbatim)
├── 📂 mobile/               # Expo React Native app
└── 📂 backend/              # Rust Axum API (optional)
```

**Why this organization?**
- **Monorepo pattern**: Frontend and backend together, but independently runnable
- **Clear separation**: `mobile/` and `backend/` can be developed in parallel
- **Documentation preserved**: All original `.md` files in `docs/` unchanged

---

### 📂 docs/ — Documentation Archive

```
docs/
├── 📄 EXECUTIVE_SUMMARY.md      # Production audit results, security grade A-
├── 📄 GEMINI_CRITICAL_FIXES.md  # P0 issues found and fixed
├── 📄 GEMINI_FINAL_FIXES.md     # Additional improvements made
└── 📄 LOGIC_TREE_SCHEMATIC.md   # Deep architecture diagrams
```

**Why preserve these?**
- **Historical context**: Shows what was fixed and why
- **Security audit**: Proof of production readiness
- **Learning**: Detailed explanations of best practices
- **Compliance**: Documentation for security reviews

---

### 📂 mobile/ — Expo React Native App

```
mobile/
├── 📄 package.json              # Dependencies and scripts
├── 📄 tsconfig.json             # TypeScript configuration
├── 📄 app.json                  # Expo configuration
├── 📄 App.tsx                   # Root app component
├── 📄 index.ts                  # Entry point
├── 📂 src/                      # Source code
│   └── 📂 api/                  # API layer (infrastructure)
│       ├── 📄 client.ts         # Type-safe HTTP client
│       ├── 📂 types/            # Auto-generated TypeScript types
│       │   ├── 📄 User.ts
│       │   ├── 📄 UserResponse.ts
│       │   ├── 📄 CreateUserRequest.ts
│       │   └── 📄 UpdateUserRequest.ts
│       └── 📄 client_old.ts     # DEPRECATED: Kept for reference
└── 📂 node_modules/             # Dependencies (generated)
```

**Why this structure?**

#### `src/api/` — Infrastructure Layer
- **Single responsibility**: Only handles HTTP communication
- **Reusable**: Same client works for all features
- **Type-safe**: Auto-generated types prevent API drift
- **Secure**: Hardware-backed token storage

#### `src/api/types/` — Generated Types
- **Never edit manually**: Always generated from Rust
- **Type safety**: Compile-time guarantee of API contract
- **Documentation**: Types serve as API documentation
- **Refactor safety**: Changes break compilation, not runtime

#### `package.json` Scripts
```json
{
  "start": "expo start",           // Development server
  "android": "expo start --android", // Android development
  "ios": "expo start --ios",       // iOS development
  "web": "expo start --web",       // Web development
  "type-check": "tsc --noEmit",    // TypeScript validation
  "lint": "echo 'Lint not configured'" // Placeholder for linting
}
```

---

### 📂 backend/ — Rust Axum API

```
backend/
├── 📄 Cargo.toml                 # Rust dependencies and config
├── 📂 src/                       # Source code
│   ├── 📄 main.rs                # Application entry point
│   └── 📂 features/              # Feature-first organization
│       └── 📂 users/             # User feature module
│           └── 📂 infrastructure/ # Database layer
│               └── 📄 repository.rs # User database operations
└── 📂 target/                    # Build artifacts (generated)
```

**Why this structure?**

#### Feature-First Organization (`features/`)
- **Scalable**: Add new features without touching existing code
- **Team-friendly**: Different teams can own different features
- **Deletable**: Remove features by deleting folders
- **Testable**: Each feature can be tested in isolation

#### Clean Architecture Layers
```
features/users/
├── 📂 domain/           # Business logic (not implemented yet)
│   ├── 📄 entities.rs   # Core types
│   └── 📄 services.rs   # Business rules
├── 📂 application/     # Use cases (not implemented yet)
│   └── 📄 commands.rs   # Request/response handling
└── 📂 infrastructure/  # External concerns
    ├── 📄 repository.rs # Database operations
    └── 📄 handlers.rs   # HTTP endpoints (not implemented yet)
```

#### `Cargo.toml` Dependencies
```toml
[dependencies]
axum = "0.7"                    # Web framework
tokio = { version = "1", features = ["full"] }  # Async runtime
diesel = { version = "2.1", features = ["postgres"] }  # ORM
serde = { version = "1.0", features = ["derive"] }    # Serialization
serde_json = "1.0"             # JSON handling
ts-rs = { version = "8.1", features = ["serde-compat"] }  # TypeScript generation
```

---

## 🌳 ASCII Logic Tree Diagrams

### Overall System Architecture

```
                    ┌─────────────────────────────────────┐
                    │           USER INTERACTION            │
                    │         (Mobile App Screen)          │
                    └─────────────────┬───────────────────┘
                                      │
                                      ▼
                    ┌─────────────────────────────────────┐
                    │         PRESENTATION LAYER           │
                    │    (React Components + Hooks)       │
                    │                                     │
                    │  ┌─────────────┐  ┌─────────────┐ │
                    │  │ UserProfile  │  │ UserList    │ │
                    │  │ Component    │  │ Component   │ │
                    │  └─────────────┘  └─────────────┘ │
                    └─────────────────┬───────────────────┘
                                      │
                                      ▼
                    ┌─────────────────────────────────────┐
                    │         APPLICATION LAYER           │
                    │      (React Query Hooks)          │
                    │                                     │
                    │  ┌─────────────┐  ┌─────────────┐ │
                    │  │ useUser()   │  │ useUsers()  │ │
                    │  │ Hook        │  │ Hook        │ │
                    │  └─────────────┘  └─────────────┘ │
                    └─────────────────┬───────────────────┘
                                      │
                                      ▼
                    ┌─────────────────────────────────────┐
                    │        INFRASTRUCTURE LAYER         │
                    │        (API Client)                 │
                    │                                     │
                    │  ┌─────────────┐  ┌─────────────┐ │
                    │  │ userApi     │  │ authApi     │ │
                    │  │ (Mutations) │  │ (Auth)      │ │
                    │  └─────────────┘  └─────────────┘ │
                    └─────────────────┬───────────────────┘
                                      │
                                      ▼
                    ┌─────────────────────────────────────┐
                    │           NETWORK LAYER              │
                    │         (HTTP + JSON)               │
                    │                                     │
                    │  ┌─────────────┐  ┌─────────────┐ │
                    │  │ GET /users  │  │ POST /auth  │ │
                    │  │ /users/:id  │  │ /login      │ │
                    │  └─────────────┘  └─────────────┘ │
                    └─────────────────┬───────────────────┘
                                      │
                                      ▼
                    ┌─────────────────────────────────────┐
                    │          BACKEND API                │
                    │         (Axum + Rust)               │
                    │                                     │
                    │  ┌─────────────┐  ┌─────────────┐ │
                    │  │ User Handler│  │ Auth Handler│ │
                    │  │ (Routes)    │  │ (JWT)       │ │
                    │  └─────────────┘  └─────────────┘ │
                    └─────────────────┬───────────────────┘
                                      │
                                      ▼
                    ┌─────────────────────────────────────┐
                    │         DATABASE LAYER             │
                    │        (Diesel + PostgreSQL)       │
                    │                                     │
                    │  ┌─────────────┐  ┌─────────────┐ │
                    │  │ users table │  │ Query       │ │
                    │  │ (Schema)    │  │ (Type-safe) │ │
                    │  └─────────────┘  └─────────────┘ │
                    └─────────────────────────────────────┘
```

### Type Safety Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                        TYPE GENERATION                           │
│                        (Rust → TypeScript)                      │
└─────────────────────────────────┬───────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────┐
│                    RUST BACKEND                                  │
│                                                                   │
│  #[derive(Serialize, Deserialize, TS)]                           │
│  pub struct User {                                               │
│      pub id: i64,                                                 │
│      pub email: String,                                           │
│      pub name: String,                                           │
│  }                                                               │
└─────────────────────────────────┬───────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────┐
│                    CARGO TEST                                    │
│                  (cargo test)                                    │
│                                                                   │
│  ts-rs generates TypeScript types                                │
│  → mobile/src/api/types/User.ts                                  │
└─────────────────────────────────┬───────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────┐
│                 TYPESCRIPT FRONTEND                              │
│                                                                   │
│  export interface User {                                          │
│      id: number;                                                  │
│      email: string;                                              │
│      name: string;                                               │
│  }                                                               │
└─────────────────────────────────┬───────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────┐
│                    COMPILE-TIME SAFETY                           │
│                                                                   │
│  ❌ If backend changes:                                           │
│     - TypeScript compilation fails                               │
│     - Must update frontend before deployment                     │
│                                                                   │
│  ✅ If types match:                                               │
│     - Zero runtime type errors                                    │
│     - API contract guaranteed                                     │
└─────────────────────────────────────────────────────────────────┘
```

### Data Flow with Caching

```
┌─────────────────────────────────────────────────────────────────┐
│                     COMPONENT                                     │
│                UserProfile({ userId: 123 })                      │
└─────────────────────────────────┬───────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────┐
│                     REACT QUERY                                  │
│                                                                   │
│  const { data: user, isLoading, error } = useUser(123);         │
│                                                                   │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
│  │   Cache?    │  │  Loading?   │  │   Error?    │              │
│  │ Check       │  │ State       │  │ State       │              │
│  └─────────────┘  └─────────────┘  └─────────────┘              │
└─────────────────────────────────┬───────────────────────────────────┘
                                  │ (if cache miss)
                                  ▼
┌─────────────────────────────────────────────────────────────────┐
│                    API CLIENT                                    │
│                                                                   │
│  const response = await apiClient.get(`/api/v1/users/${123}`);   │
│                                                                   │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
│  │   Add JWT   │  │  HTTP Req   │  │  Handle     │              │
│  │   Token     │  │  to Backend │  │  Errors     │              │
│  └─────────────┘  └─────────────┘  └─────────────┘              │
└─────────────────────────────────┬───────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────┐
│                     BACKEND                                      │
│                                                                   │
│  GET /api/v1/users/123 → Validate JWT → Query DB → Return JSON   │
└─────────────────────────────────┬───────────────────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────┐
│                     CACHE UPDATE                                  │
│                                                                   │
│  React Query stores response in cache:                           │
│  - Key: ['user', 123]                                            │
│  - Data: User object                                             │
│  - Stale time: 5 minutes                                         │
│                                                                   │
│  🔄 Background refresh if stale                                  │
│  ⚡ Instant cache hit on subsequent requests                     │
└─────────────────────────────────────────────────────────────────┘
```

---

## ⚙️ Framework Decisions

### Mobile: Expo vs Bare React Native

```
┌─────────────────────┬─────────────────────┬─────────────────────┐
│      CRITERIA       │       EXPO          │    BARE RN         │
├─────────────────────┼─────────────────────┼─────────────────────┤
│ Setup Time          │ ⚡ 5 minutes        │ 🐌 2+ hours        │
│ Native Build Issues │ ✅ Never             │ ❌ Frequently       │
│ OTA Updates         │ ✅ Built-in          │ ❌ Manual           │
│ Native Modules      │ ✅ Managed           │ ❌ Manual config    │
│ Performance         │ ✅ Same              │ ✅ Same            │
│ App Store Review    │ ✅ Standard          │ ✅ Standard        │
│ Learning Curve      │ ✅ Gentle            │ ❌ Steep           │
│ Production Ready    │ ✅ Yes (500k+ apps)  │ ✅ Yes (millions)  │
└─────────────────────┴─────────────────────┴─────────────────────┘
```

**Winner: Expo** for 95% of apps. Choose Bare RN only if you need:
- Custom native modules not in Expo
- Deep integration with native OS features
- Complete control over build process

### Backend: Rust vs Node.js

```
┌─────────────────────┬─────────────────────┬─────────────────────┐
│      CRITERIA       │        RUST          │      NODE.JS       │
├─────────────────────┼─────────────────────┼─────────────────────┤
│ Performance         │ ⚡ 2-10x faster      │ 🐌 Slower          │
│ Memory Usage        │ ✅ Minimal           │ ❌ Higher          │
│ Type Safety         │ ✅ Compile-time      │ ❌ Runtime only    │
│ Concurrency         │ ✅ Fearless          │ ❌ Callback hell   │
│ Learning Curve      │ ❌ Steep             │ ✅ Gentle          │
│ Ecosystem           │ 🔄 Growing fast      │ ✅ Mature          │
│ Deployment          │ ✅ Single binary      │ ❌ Node runtime    │
│ Type Generation     │ ✅ ts-rs built-in    │ ❌ Manual          │
└─────────────────────┴─────────────────────┴─────────────────────┘
```

**Winner: Rust** for APIs where:
- Performance matters (high concurrency)
- Type safety is critical (financial/health data)
- Memory efficiency is important (cost optimization)
- Team is willing to learn Rust

### Database: PostgreSQL vs SQLite

```
┌─────────────────────┬─────────────────────┬─────────────────────┐
│      CRITERIA       │    POSTGRESQL       │      SQLITE         │
├─────────────────────┼─────────────────────┼─────────────────────┤
│ Concurrency         │ ✅ Excellent        │ ❌ Limited         │
│ Scaling             │ ✅ Vertical + Horizontal │ ❌ Single file   │
│ Features            │ ✅ Full SQL + JSON  │ 🔄 Basic SQL       │
│ Transactions        │ ✅ ACID compliant   │ ✅ ACID compliant  │
│ Setup               │ ❌ Separate service  │ ✅ Zero config     │
│ Backup              │ ✅ Professional tools │ ❌ File copy       │
│ Production Ready    │ ✅ Enterprise grade  │ ❌ Small apps only  │
└─────────────────────┴─────────────────────┴─────────────────────┘
```

**Winner: PostgreSQL** for production apps. Use SQLite only for:
- Prototyping
- Small tools
- Embedded applications
- Local-first apps

---

## 🔄 Type Safety Flow

### Step-by-Step Type Generation

```
1. DEFINE IN RUST
   └── backend/src/features/users/domain/entities.rs
       #[derive(Serialize, Deserialize, TS)]
       pub struct User {
           pub id: i64,
           pub email: String,
           pub name: String,
       }

2. GENERATE TYPES
   └── cd backend && cargo test
       → Creates mobile/src/api/types/User.ts
       → TypeScript interface matches Rust struct exactly

3. USE IN FRONTEND
   └── mobile/src/api/client.ts
       import { User } from './types/User';
       
       export function useUser(userId: number) {
           return useQuery({
               queryKey: ['user', userId],
               queryFn: () => fetchUser(userId), // Returns Promise<User>
           });
       }

4. COMPILE-TIME VALIDATION
   └── If Rust changes:
       ❌ TypeScript compilation fails
       ❌ Must update frontend
       ✅ No runtime surprises

5. RUNTIME GUARANTEE
   └── If types compile:
       ✅ API contract guaranteed
       ✅ Zero type-related runtime errors
       ✅ Refactor with confidence
```

### Error Prevention Examples

```
❌ WITHOUT TYPE SAFETY (Runtime Errors)
   // Backend adds field, frontend doesn't know
   const user = await api.getUser(123);
   console.log(user.newField); // undefined - silent failure!

✅ WITH TYPE SAFETY (Compile-Time Errors)
   // Backend adds field, TypeScript catches it
   const user: User = await api.getUser(123);
   console.log(user.newField); // ❌ Compile error: Property 'newField' does not exist
                                 // Must update User type first
```

---

## 🛠️ Development Workflow

### Day 1: New Project Setup

```
1. COPY TEMPLATE
   cp -r SKELETON_STARTER_v3 my-new-app
   cd my-new-app

2. SETUP MOBILE
   cd mobile
   npm install
   npm run start
   → Expo dev server running

3. SETUP BACKEND (Optional)
   cd backend
   cargo check
   cargo test
   → Types generated for mobile

4. VERIFY INTEGRATION
   cd mobile
   npm run type-check
   → Should pass without errors
```

### Day 2: Add New Feature

```
1. DEFINE IN RUST
   // backend/src/features/posts/domain/entities.rs
   #[derive(Serialize, Deserialize, TS)]
   pub struct Post {
       pub id: i64,
       pub title: String,
       pub content: String,
   }

2. GENERATE TYPES
   cd backend && cargo test
   → Creates mobile/src/api/types/Post.ts

3. ADD API CLIENT
   // mobile/src/api/client.ts
   export const postApi = {
       async getAll(): Promise<Post[]> {
           const response = await apiClient.get<Post[]>('/api/v1/posts');
           return response.data;
       }
   };

4. ADD REACT QUERY HOOK
   // mobile/src/api/client.ts
   export function usePosts() {
       return useQuery({
           queryKey: ['posts'],
           queryFn: () => postApi.getAll(),
       });
   }

5. USE IN COMPONENT
   // mobile/src/components/PostList.tsx
   function PostList() {
       const { data: posts, isLoading } = usePosts();
       if (isLoading) return <LoadingSpinner />;
       return posts.map(post => <PostItem key={post.id} post={post} />);
   }
```

### Day 3: Testing & Deployment

```
1. TYPE CHECK
   cd mobile && npm run type-check
   cd backend && cargo check

2. RUN TESTS
   cd mobile && npm test  # (when tests are added)
   cd backend && cargo test

3. LINTING
   cd mobile && npm run lint  # (when configured)
   cd backend && cargo clippy

4. BUILD
   cd mobile && expo build:android  # or expo build:ios
   cd backend && cargo build --release

5. DEPLOY
   → Mobile: App Store/Play Store
   → Backend: Docker/Kubernetes
```

---

## 🚀 Production Considerations

### Security Checklist

```
✅ TOKEN STORAGE
   - Using expo-secure-store (hardware-backed)
   - Never AsyncStorage (unencrypted)

✅ JWT VALIDATION
   - Production secrets only
   - Reject weak/default secrets
   - Token expiration handling

✅ RATE LIMITING
   - 60 requests/minute per IP
   - Prevents brute force attacks
   - Configurable per endpoint

✅ INPUT VALIDATION
   - TypeScript compile-time checks
   - Rust runtime validation
   - SQL injection prevention (Diesel)

✅ HTTPS ONLY
   - Never HTTP in production
   - Certificate pinning (optional)
   - HSTS headers
```

### Performance Checklist

```
✅ CACHING STRATEGY
   - React Query: 5-minute stale time
   - Background refresh
   - Optimistic updates

✅ DATABASE INDEXES
   - Primary keys indexed
   - Foreign keys indexed
   - Query-specific indexes

✅ ASYNC OPERATIONS
   - Never block main thread
   - spawn_blocking for database
   - Async/await throughout

✅ BUNDLE SIZE
   - Tree-shaking enabled
   - Lazy loading routes
   - Optimize images
```

### Monitoring Checklist

```
✅ ERROR TRACKING
   - Sentry integration
   - Crash reporting
   - Performance monitoring

✅ HEALTH CHECKS
   - /health endpoint
   - Database connectivity
   - External service checks

✅ LOGGING
   - Structured logging
   - Request/response logs
   - Error context
```

---

## 🎯 Summary: Why This Template Works

### 1. **Type Safety End-to-End**
- Rust structs → TypeScript interfaces
- Compile-time error prevention
- Zero runtime type surprises

### 2. **Clean Architecture**
- Feature-first organization
- Separation of concerns
- Testable in isolation

### 3. **Modern Tooling**
- Expo for rapid development
- Rust for performance
- React Query for data management

### 4. **Production Ready**
- Security hardened
- Performance optimized
- Monitoring ready

### 5. **Developer Experience**
- Verbose documentation
- Clear file organization
- Repeatable patterns

### 6. **Maintainable**
- Self-documenting code
- Consistent patterns
- Easy onboarding

---

## 📚 Next Steps

1. **Customize for your domain**
   - Add your own features in `backend/src/features/`
   - Update mobile components for your use case

2. **Add tests**
   - Unit tests for business logic
   - Integration tests for API
   - E2E tests for critical flows

3. **Setup CI/CD**
   - GitHub Actions for automated testing
   - Automated deployments
   - Security scanning

4. **Add monitoring**
   - Sentry for error tracking
   - Analytics for user behavior
   - Performance monitoring

---

## 🎉 Conclusion

This template is **more than code**—it's a **philosophy** for building robust, maintainable applications:

- **Type safety** prevents entire classes of bugs
- **Clean architecture** enables team scaling
- **Modern tooling** maximizes developer productivity
- **Production focus** ensures real-world readiness

**Every decision was made with intentionality** and documented here so you can understand not just **what** to build, but **why** it's built this way.

Build something amazing. 🚀
