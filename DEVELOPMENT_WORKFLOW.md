# 🔄 Development Workflow Guide - APP_VIRGIN Template

**Last Updated:** January 10, 2026  
**Purpose:** Complete development workflow, from setup to deployment

---

## 📋 Table of Contents

1. [Daily Development Workflow](#daily-development-workflow)
2. [Hot Reload & Development Servers](#hot-reload--development-servers)
3. [Debugging Setup](#debugging-setup)
4. [Code Generation Workflow](#code-generation-workflow)
5. [Git Workflow](#git-workflow)
6. [Environment Management](#environment-management)
7. [Performance Monitoring](#performance-monitoring)
8. [Common Development Patterns](#common-development-patterns)

---

## 🚀 Daily Development Workflow

### Morning Setup (5 minutes)

```bash
# 1. Start backend (Terminal 1)
cd backend
cargo run
# Backend starts at http://localhost:8000

# 2. Start mobile (Terminal 2)
cd mobile
npm start
# Press 'w' for web or scan QR for Expo Go

# 3. Verify everything works
curl http://localhost:8000/health/live
# Should return: {"status":"ok"}

# 4. Open browser to http://localhost:8081
# Should see: 🔥 12 Day Streak demo
```

### Development Loop

```bash
# Backend changes → Auto-reload with cargo-watch
cargo watch -x run

# Mobile changes → Hot reload in Expo
# Save file → Auto-refresh in browser/app

# Type generation → After Rust changes
cd backend
cargo test generate_typescript_types
```

### End of Day

```bash
# 1. Run tests to ensure nothing broke
cd backend && cargo test
cd mobile && npm test

# 2. Commit changes
git add .
git commit -m "feat: describe the change"

# 3. Push if working on feature branch
git push origin feature-branch-name
```

---

## 🔥 Hot Reload & Development Servers

### Backend Hot Reload

```bash
# Install cargo-watch if not already installed
cargo install cargo-watch

# Start with auto-reload
cd backend
cargo watch -x run

# Watch specific patterns
cargo watch -x 'run --bin backend'

# Custom watch commands
cargo watch -s 'cargo check && cargo run'
```

**What triggers reload:**
- Any `.rs` file change
- `.env` file changes
- Cargo.toml changes

**Reload time:** ~2-3 seconds for small changes

### Mobile Hot Reload

```bash
# Start Expo development server
cd mobile
npm start

# Options:
# Press 'w' → Open web browser
# Press 'a' → Open Android emulator
# Press 'i' → Open iOS simulator
# Press 'r' → Manual reload
# Press 'd' → Open debugger
```

**Hot Reload Features:**
- **Fast Refresh**: Component state preserved for function components
- **Instant Refresh**: Full app reload for class components or certain changes
- **Error Recovery**: Automatic recovery from syntax errors

**Hot Reload Limitations:**
- Changes to `App.tsx` entry point require full reload
- Native module changes require restart
- Environment variable changes require restart

### Development Server URLs

| Service | URL | Purpose |
|---------|-----|---------|
| Backend API | http://localhost:8000 | REST API endpoints |
| Mobile Web | http://localhost:8081 | Expo web app |
| Expo DevTools | http://localhost:8081 | React DevTools, logs |
| Health Check | http://localhost:8000/health/live | Service status |

---

## 🐛 Debugging Setup

### Backend Debugging

#### VS Code Configuration

```json
// File: .vscode/launch.json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug Backend",
      "cargo": {
        "args": [
          "build",
          "--bin=backend"
        ],
        "filter": {
          "name": "backend",
          "kind": "bin"
        }
      },
      "args": [],
      "cwd": "${workspaceFolder}/backend"
    }
  ]
}
```

#### Logging Setup

```rust
// File: backend/src/main.rs
use tracing::{info, warn, error, debug};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("Starting backend server");
    
    // Your app code here
    
    debug!("Debug information");
    warn!("Warning message");
    error!("Error occurred");
}
```

#### Environment Variables for Debugging

```bash
# Enable verbose logging
RUST_LOG=debug cargo run

# Enable SQL query logging
RUST_LOG=diesel=debug cargo run

# Custom log levels
RUST_LOG=backend=debug,axum=info cargo run
```

### Mobile Debugging

#### React DevTools

```bash
# Install React DevTools
npm install -g react-devtools

# Start DevTools
react-devtools

# Or use browser DevTools at http://localhost:8081
```

#### Flipper (Advanced Debugging)

```bash
# Install Flipper
npm install -g flipper

# Add Flipper to mobile app
cd mobile
npm install react-native-flipper
```

#### Remote Debugging

```bash
# In Expo app, shake device or press Ctrl+D
# Select "Open Debugger" or press 'd' in terminal

# Opens Chrome DevTools at:
# http://localhost:8081/debugger-ui/
```

#### Console Logging

```typescript
// File: mobile/src/utils/logger.ts
export const logger = {
  info: (message: string, data?: any) => {
    console.log(`[INFO] ${message}`, data);
  },
  error: (message: string, error?: any) => {
    console.error(`[ERROR] ${message}`, error);
  },
  debug: (message: string, data?: any) => {
    if (__DEV__) {
      console.log(`[DEBUG] ${message}`, data);
    }
  }
};

// Usage in components
import { logger } from '../utils/logger';

const MyComponent = () => {
  const handlePress = () => {
    logger.info('Button pressed', { timestamp: Date.now() });
  };
  
  return <Button onPress={handlePress}>Press me</Button>;
};
```

---

## 🔧 Code Generation Workflow

### TypeScript Type Generation

#### 1. Define Rust Types

```rust
// File: backend/src/features/users/domain/entities.rs
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
}
```

#### 2. Generate TypeScript Types

```bash
# Run the generation test
cd backend
cargo test generate_typescript_types

# Generated file location:
# backend/src/domain/types/User.ts
```

#### 3. Sync to Mobile (Optional Automation)

```powershell
# File: scripts/sync-types.ps1
# Sync generated types to mobile app

$backendTypes = "backend/src/domain/types"
$mobileTypes = "mobile/src/api/types"

# Create mobile types directory if not exists
if (!(Test-Path $mobileTypes)) {
    New-Item -ItemType Directory -Path $mobileTypes
}

# Copy all generated TypeScript files
Copy-Item -Path "$backendTypes/*.ts" -Destination $mobileTypes -Force

Write-Host "Types synced successfully!"
```

#### 4. Use in Mobile App

```typescript
// File: mobile/src/api/client.ts
import { User, CreateUserRequest } from './types/User';

export const apiClient = {
  createUser: async (request: CreateUserRequest): Promise<User> => {
    const response = await fetch('/api/v1/users', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(request),
    });
    
    return response.json();
  }
};
```

### API Client Generation

```typescript
// File: scripts/generate-api-client.ts
// Auto-generate API client from OpenAPI spec

import { generateApi } from 'swagger-typescript-api';

const result = await generateApi({
  name: 'ApiClient',
  output: './mobile/src/api/generated.ts',
  url: 'http://localhost:8000/api-docs/openapi.json',
  httpClientType: 'axios',
  generateClient: true,
  generateRouteTypes: true,
});

console.log('API client generated!');
```

---

## 🌿 Git Workflow

### Branch Strategy

```bash
# Main branches
main          # Production-ready code
develop       # Integration branch

# Feature branches
feature/user-auth
feature/health-check
feature/mobile-ui

# Release branches
release/v1.0.0

# Hotfix branches
hotfix/critical-bug-fix
```

### Commit Message Convention

```bash
# Format: <type>(<scope>): <description>

# Types
feat:     New feature
fix:      Bug fix
docs:     Documentation
style:    Code formatting
refactor: Code refactoring
test:     Adding tests
chore:    Maintenance tasks

# Examples
feat(auth): add JWT token refresh
fix(api): resolve CORS error
docs(readme): update setup instructions
test(backend): add integration tests
```

### Daily Git Workflow

```bash
# 1. Start new feature
git checkout develop
git pull origin develop
git checkout -b feature/new-feature

# 2. Work on feature (make commits)
git add .
git commit -m "feat: implement new feature"

# 3. Keep branch updated
git checkout develop
git pull origin develop
git checkout feature/new-feature
git merge develop

# 4. Finish feature
git checkout develop
git merge feature/new-feature
git push origin develop

# 5. Clean up
git branch -d feature/new-feature
```

### Pre-commit Hooks

```bash
# Install husky for git hooks
cd mobile
npm install --save-dev husky

# Setup pre-commit hook
npx husky add .husky/pre-commit "npm run lint && npm run type-check"

# Make it executable
chmod +x .husky/pre-commit
```

---

## 🌍 Environment Management

### Development Environments

```bash
# Local Development
ENVIRONMENT=development
DATABASE_URL=postgresql://localhost/app_virgin_dev
JWT_SECRET=dev_secret_key
ALLOWED_ORIGINS=http://localhost:8081,http://localhost:19006

# Staging
ENVIRONMENT=staging
DATABASE_URL=postgresql://staging-db/app_virgin_staging
JWT_SECRET=staging_secret_key
ALLOWED_ORIGINS=https://staging.app.com

# Production
ENVIRONMENT=production
DATABASE_URL=postgresql://prod-db/app_virgin_prod
JWT_SECRET=super_secure_production_secret
ALLOWED_ORIGINS=https://app.com
```

### Environment-Specific Configurations

```typescript
// File: mobile/src/config/environment.ts
const config = {
  development: {
    apiUrl: 'http://localhost:8000',
    enableLogging: true,
    enableDebugMenu: true,
  },
  staging: {
    apiUrl: 'https://staging-api.app.com',
    enableLogging: true,
    enableDebugMenu: false,
  },
  production: {
    apiUrl: 'https://api.app.com',
    enableLogging: false,
    enableDebugMenu: false,
  },
};

const currentEnv = __DEV__ ? 'development' : 'production';
export const environment = config[currentEnv];
```

### Secret Management

```bash
# Never commit secrets to git
# Use .env files (already in .gitignore)

# Backend secrets
cp backend/.env.example backend/.env
# Edit backend/.env with real secrets

# Mobile secrets
cp mobile/.env.example mobile/.env
# Edit mobile/.env with real secrets

# For production, use environment variables or secret management service
```

---

## 📊 Performance Monitoring

### Backend Monitoring

```rust
// File: backend/src/middleware/logging.rs
use axum::middleware;
use tracing::info;

pub async fn logging_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();
    
    let response = next.run(req).await;
    
    let duration = start.elapsed();
    info!(
        "Request: {} {} - Status: {} - Duration: {:?}",
        method,
        uri,
        response.status(),
        duration
    );
    
    Ok(response)
}
```

### Mobile Performance Monitoring

```typescript
// File: mobile/src/utils/performance.ts
export const performance = {
  measureComponentRender: (componentName: string) => {
    const startTime = performance.now();
    
    return () => {
      const endTime = performance.now();
      const renderTime = endTime - startTime;
      
      if (__DEV__) {
        console.log(`[PERF] ${componentName} rendered in ${renderTime.toFixed(2)}ms`);
      }
    };
  },
  
  measureApiCall: async <T>(
    apiCall: () => Promise<T>,
    apiName: string
  ): Promise<T> => {
    const startTime = performance.now();
    
    try {
      const result = await apiCall();
      const duration = performance.now() - startTime;
      
      if (__DEV__) {
        console.log(`[PERF] ${apiName} completed in ${duration.toFixed(2)}ms`);
      }
      
      return result;
    } catch (error) {
      const duration = performance.now() - startTime;
      console.error(`[PERF] ${apiName} failed after ${duration.toFixed(2)}ms`, error);
      throw error;
    }
  }
};

// Usage in components
const MyComponent = () => {
  const endMeasure = performance.measureComponentRender('MyComponent');
  
  useEffect(() => {
    endMeasure();
  });
  
  const handleApiCall = async () => {
    const data = await performance.measureApiCall(
      () => apiClient.getData(),
      'getData'
    );
  };
};
```

### Memory Leak Detection

```typescript
// File: mobile/src/utils/memory-leak-detector.ts
export const MemoryLeakDetector = {
  trackComponent: (componentName: string) => {
    if (__DEV__) {
      const componentCount = (global as any).__componentCounts || {};
      componentCount[componentName] = (componentCount[componentName] || 0) + 1;
      (global as any).__componentCounts = componentCount;
      
      console.log(`[MEMORY] ${componentName} instances: ${componentCount[componentName]}`);
    }
  },
  
  untrackComponent: (componentName: string) => {
    if (__DEV__) {
      const componentCount = (global as any).__componentCounts || {};
      componentCount[componentName] = Math.max(0, (componentCount[componentName] || 0) - 1);
      (global as any).__componentCounts = componentCount;
      
      console.log(`[MEMORY] ${componentName} instances: ${componentCount[componentName]}`);
    }
  }
};
```

---

## 🏗 Common Development Patterns

### Adding New API Endpoints

#### 1. Backend Implementation

```rust
// File: backend/src/features/posts/api.rs
use axum::{Json, extract::Path};
use serde_json::Value;

pub async fn get_post(Path(id): Path<i64>) -> Result<Json<Value>, ApiError> {
    // Fetch post from database
    let post = posts::repository::find_by_id(id).await?;
    
    Ok(Json(serde_json::to_value(post)?))
}

pub async fn create_post(
    Json(payload): Json<CreatePostRequest>
) -> Result<Json<Value>, ApiError> {
    // Create new post
    let post = posts::repository::create(payload).await?;
    
    Ok(Json(serde_json::to_value(post)?))
}

// Register routes
pub fn post_routes() -> Router {
    Router::new()
        .route("/posts/:id", get(get_post))
        .route("/posts", post(create_post))
}
```

#### 2. Type Generation

```rust
// File: backend/src/features/posts/domain/entities.rs
#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}
```

#### 3. Frontend Integration

```typescript
// File: mobile/src/api/posts.ts
import { Post, CreatePostRequest } from './types/Post';
import { apiClient } from './client';

export const postsApi = {
  getPost: async (id: number): Promise<Post> => {
    return apiClient.get(`/posts/${id}`);
  },
  
  createPost: async (post: CreatePostRequest): Promise<Post> => {
    return apiClient.post('/posts', post);
  }
};
```

#### 4. React Query Integration

```typescript
// File: mobile/src/hooks/usePosts.ts
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { postsApi } from '../api/posts';

export const usePost = (id: number) => {
  return useQuery({
    queryKey: ['post', id],
    queryFn: () => postsApi.getPost(id),
  });
};

export const useCreatePost = () => {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: postsApi.createPost,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['posts'] });
    },
  });
};
```

### Adding New Screens

#### 1. Screen Component

```typescript
// File: mobile/src/screens/PostDetailScreen.tsx
import React from 'react';
import { View, Text, StyleSheet } from 'react-native';
import { usePost } from '../hooks/usePosts';

interface PostDetailScreenProps {
  postId: number;
}

export const PostDetailScreen: React.FC<PostDetailScreenProps> = ({ postId }) => {
  const { data: post, isLoading, error } = usePost(postId);
  
  if (isLoading) return <Text>Loading...</Text>;
  if (error) return <Text>Error: {error.message}</Text>;
  
  return (
    <View style={styles.container}>
      <Text style={styles.title}>{post?.title}</Text>
      <Text style={styles.content}>{post?.content}</Text>
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    padding: 16,
  },
  title: {
    fontSize: 24,
    fontWeight: 'bold',
    marginBottom: 16,
  },
  content: {
    fontSize: 16,
    lineHeight: 24,
  },
});
```

#### 2. Navigation Integration

```typescript
// File: mobile/src/navigation/AppNavigator.tsx
import { PostDetailScreen } from '../screens/PostDetailScreen';

const Stack = createNativeStackNavigator();

export const AppNavigator = () => {
  return (
    <Stack.Navigator>
      <Stack.Screen name="Home" component={HomeScreen} />
      <Stack.Screen 
        name="PostDetail" 
        component={PostDetailScreen}
        options={({ route }) => ({ 
          title: route.params.postId.toString() 
        })}
      />
    </Stack.Navigator>
  );
};
```

### Error Handling Patterns

#### Backend Error Handling

```rust
// File: backend/src/api/mod.rs
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    DatabaseError(diesel::result::Error),
    NotFound,
    Unauthorized,
    ValidationError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::DatabaseError(err) => {
                tracing::error!("Database error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            ApiError::ValidationError(msg) => (StatusCode::BAD_REQUEST, &msg),
        };

        let body = json!({
            "error": error_message,
            "status": status.as_u16()
        });

        (status, Json(body)).into_response()
    }
}
```

#### Frontend Error Handling

```typescript
// File: mobile/src/components/ErrorBoundary.tsx
import React, { Component, ErrorInfo, ReactNode } from 'react';

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
}

interface State {
  hasError: boolean;
  error: Error | null;
}

export class ErrorBoundary extends Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = { hasError: false, error: null };
  }

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('ErrorBoundary caught:', error, errorInfo);
    
    // Send to error reporting service
    if (!__DEV__) {
      // Sentry.captureException(error);
    }
  }

  render() {
    if (this.state.hasError) {
      return this.props.fallback || (
        <View style={styles.container}>
          <Text>Something went wrong</Text>
          {__DEV__ && <Text>{this.state.error?.message}</Text>}
        </View>
      );
    }

    return this.props.children;
  }
}
```

---

## 🎯 Development Best Practices

### Code Organization

1. **Feature-first structure**: Group related files together
2. **Type safety first**: Generate types, don't write them manually
3. **Error handling**: Handle errors at every boundary
4. **Testing**: Write tests alongside code
5. **Documentation**: Document complex decisions

### Performance Tips

1. **Lazy loading**: Load code and data on demand
2. **Caching**: Use React Query for API caching
3. **Bundle size**: Monitor and optimize bundle size
4. **Memory leaks**: Track component mount/unmount
5. **Database queries**: Use indexes and query optimization

### Security Practices

1. **Environment variables**: Never commit secrets
2. **Input validation**: Validate all inputs
3. **Authentication**: Use proper token management
4. **HTTPS**: Always use HTTPS in production
5. **Dependencies**: Keep dependencies updated

---

**Happy Development! 🚀**

This workflow ensures consistent, efficient development across the entire stack.
