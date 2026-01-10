# 🧪 Testing Guide - APP_VIRGIN Template

**Last Updated:** January 10, 2026  
**Purpose:** Complete testing strategy for backend, mobile, and integration testing

---

## 📋 Table of Contents

1. [Overview](#overview)
2. [Backend Testing](#backend-testing)
3. [Mobile Testing](#mobile-testing)
4. [Integration Testing](#integration-testing)
5. [End-to-End Testing](#end-to-end-testing)
6. [Test Environment Setup](#test-environment-setup)
7. [CI/CD Testing](#cicd-testing)
8. [Troubleshooting](#troubleshooting)

---

## 🎯 Overview

### Testing Philosophy

This template follows a **testing pyramid** approach:

```
        E2E Tests (Few)
       ┌─────────────────┐
      /  Integration     \
     /     Tests          \
    ┌───────────────────────┐
   /   Unit Tests (Many)     \
  /                           \
 ┌─────────────────────────────┐
```

- **Unit Tests**: Fast, isolated, test single functions
- **Integration Tests**: Test component interactions
- **E2E Tests**: Test complete user workflows

### Test Structure

```
APP_VIRGIN/
├── backend/
│   ├── src/
│   │   ├── api/
│   │   │   ├── health.rs      # Health endpoint tests
│   │   │   └── auth.rs         # Auth endpoint tests
│   │   └── features/
│   │       └── users/
│   │           └── domain/     # Domain logic tests
│   └── tests/                 # Integration tests
├── mobile/
│   ├── src/
│   │   └── __tests__/         # Unit tests
│   └── e2e/                   # Detox E2E tests
└── tests/                     # Cross-platform integration tests
```

---

## 🔧 Backend Testing

### Running Tests

```bash
# Run all tests
cd backend
cargo test

# Run specific test file
cargo test health

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_health_live_endpoint

# Run tests in watch mode (requires cargo-watch)
cargo watch -x test
```

### Test Categories

#### 1. Unit Tests (Fast, Isolated)

```rust
// File: backend/src/features/users/domain/entities.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User {
            id: 1,
            email: "test@example.com".to_string(),
        };
        assert_eq!(user.id, 1);
        assert_eq!(user.email, "test@example.com");
    }

    #[test]
    fn test_user_email_validation() {
        // Test email validation logic
        assert!(is_valid_email("user@domain.com"));
        assert!(!is_valid_email("invalid-email"));
    }
}
```

#### 2. Integration Tests (API Endpoints)

```rust
// File: backend/src/api/health.rs
#[cfg(test)]
mod health_tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_health_live_endpoint() {
        let app = create_app().await;
        let server = TestServer::new(app).unwrap();

        let response = server.get("/health/live").await;
        
        assert_eq!(response.status_code(), 200);
        
        let body: serde_json::Value = response.json();
        assert_eq!(body["status"], "ok");
    }

    #[tokio::test]
    async fn test_health_ready_endpoint_without_db() {
        let app = create_app().await;
        let server = TestServer::new(app).unwrap();

        let response = server.get("/health/ready").await;
        
        assert_eq!(response.status_code(), 200);
        
        let body: serde_json::Value = response.json();
        assert_eq!(body["status"], "ok");
        assert_eq!(body["database"], "disabled");
    }
}
```

#### 3. Database Tests (Optional)

```rust
// File: backend/src/features/users/repository.rs
#[cfg(test)]
mod user_repository_tests {
    use super::*;
    use diesel::prelude::*;
    use diesel::result::Error;

    #[tokio::test]
    async fn test_create_user() {
        // Setup test database
        let mut conn = establish_test_connection().await;
        
        let user = NewUser {
            email: "test@example.com".to_string(),
            password_hash: "hash".to_string(),
        };

        let created_user = create_user(&mut conn, &user).await.unwrap();
        
        assert_eq!(created_user.email, "test@example.com");
        assert!(created_user.id > 0);
    }
}
```

### Test Configuration

```toml
# File: backend/Cargo.toml
[dev-dependencies]
tokio-test = "0.4"
axum-test = "15.0"
serde_json = "1.0"
mockall = "0.12"  # For mocking
```

### Mocking External Dependencies

```rust
// File: backend/src/api/auth.rs
#[cfg(test)]
mod auth_tests {
    use super::*;
    use mockall::mock;

    mock! {
        AuthService {}

        impl AuthServiceTrait for AuthService {
            fn generate_token(&self, user_id: i64) -> Result<String, ApiError>;
            fn validate_token(&self, token: &str) -> Result<i64, ApiError>;
        }
    }

    #[tokio::test]
    async fn test_login_success() {
        let mut mock_service = MockAuthService::new();
        
        mock_service
            .expect_generate_token()
            .returning(|_| Ok("fake_token".to_string()));

        // Test login with mocked service
        let result = login(&mock_service, login_request).await;
        assert!(result.is_ok());
    }
}
```

---

## 📱 Mobile Testing

### Unit Testing with Jest

```bash
# Run all tests
cd mobile
npm test

# Run in watch mode
npm test -- --watch

# Run with coverage
npm test -- --coverage

# Run specific test file
npm test -- App.test.tsx
```

#### Test Configuration

```json
// File: mobile/package.json
{
  "scripts": {
    "test": "jest",
    "test:watch": "jest --watch",
    "test:coverage": "jest --coverage"
  },
  "jest": {
    "preset": "jest-expo",
    "setupFilesAfterEnv": ["<rootDir>/jest.setup.js"],
    "collectCoverageFrom": [
      "src/**/*.{ts,tsx}",
      "!src/**/*.d.ts",
      "!src/__tests__/**/*"
    ]
  }
}
```

#### Jest Setup

```javascript
// File: mobile/jest.setup.js
import 'react-native-gesture-handler/jestSetup';

jest.mock('react-native-reanimated', () => {
  const Reanimated = require('react-native-reanimated/mock');
  Reanimated.default.call = () => {};
  return Reanimated;
});

// Silence the warning: Animated: `useNativeDriver` is not supported
jest.mock('react-native/Libraries/Animated/NativeAnimatedHelper');
```

#### Component Unit Tests

```typescript
// File: mobile/src/components/__tests__/Button.test.tsx
import React from 'react';
import { render, fireEvent } from '@testing-library/react-native';
import { Button } from '../Button';

describe('Button Component', () => {
  it('renders correctly', () => {
    const { getByText } = render(
      <Button title="Test Button" onPress={() => {}} />
    );
    
    expect(getByText('Test Button')).toBeTruthy();
  });

  it('calls onPress when pressed', () => {
    const mockOnPress = jest.fn();
    const { getByText } = render(
      <Button title="Test Button" onPress={mockOnPress} />
    );
    
    fireEvent.press(getByText('Test Button'));
    expect(mockOnPress).toHaveBeenCalledTimes(1);
  });

  it('is disabled when loading', () => {
    const mockOnPress = jest.fn();
    const { getByText } = render(
      <Button title="Test Button" onPress={mockOnPress} loading={true} />
    );
    
    fireEvent.press(getByText('Test Button'));
    expect(mockOnPress).not.toHaveBeenCalled();
  });
});
```

#### Hook Testing

```typescript
// File: mobile/src/hooks/__tests__/useApi.test.ts
import { renderHook, act } from '@testing-library/react-hooks';
import { useApi } from '../useApi';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

describe('useApi Hook', () => {
  let queryClient: QueryClient;

  beforeEach(() => {
    queryClient = new QueryClient({
      defaultOptions: {
        queries: { retry: false },
        mutations: { retry: false },
      },
    });
  });

  it('fetches data successfully', async () => {
    const wrapper = ({ children }: { children: React.ReactNode }) => (
      <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
    );

    const { result, waitFor } = renderHook(() => useApi(), { wrapper });

    await waitFor(() => result.current.isSuccess);
    expect(result.current.data).toEqual({ status: 'ok' });
  });

  it('handles errors', async () => {
    const wrapper = ({ children }: { children: React.ReactNode }) => (
      <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
    );

    const { result, waitFor } = renderHook(() => useApi(), { wrapper });

    await waitFor(() => result.current.isError);
    expect(result.current.error).toBeDefined();
  });
});
```

### Integration Testing with Expo

```bash
# Run on simulator/emulator
npm run test:e2e

# Run on specific platform
npm run test:e2e:ios
npm run test:e2e:android
```

#### E2E Test Configuration

```typescript
// File: mobile/e2e/App.e2e.ts
import { device, element, by, expect } from 'detox';

describe('App E2E Tests', () => {
  beforeAll(async () => {
    await device.launchApp();
  });

  beforeEach(async () => {
    await device.reloadReactNative();
  });

  it('should show welcome screen', async () => {
    await expect(element(by.text('🔥'))).toBeVisible();
    await expect(element(by.text('12'))).toBeVisible();
    await expect(element(by.text('Day Streak'))).toBeVisible();
  });

  it('should handle button press', async () => {
    const button = element(by.text('Check In Today'));
    await expect(button).toBeVisible();
    await button.tap();
    
    // Verify expected behavior after button press
    await expect(element(by.text('Checked in!'))).toBeVisible();
  });
});
```

---

## 🔗 Integration Testing

### Cross-Platform Tests

```typescript
// File: tests/integration/auth_flow.test.ts
import { TestServer } from '../utils/test-server';
import { ApiClient } from '../utils/api-client';

describe('Authentication Flow Integration', () => {
  let server: TestServer;
  let apiClient: ApiClient;

  beforeAll(async () => {
    server = new TestServer();
    await server.start();
    apiClient = new ApiClient(server.url);
  });

  afterAll(async () => {
    await server.stop();
  });

  it('should authenticate user end-to-end', async () => {
    // 1. Register user
    const registerResponse = await apiClient.register({
      email: 'test@example.com',
      password: 'password123'
    });
    expect(registerResponse.success).toBe(true);

    // 2. Login user
    const loginResponse = await apiClient.login({
      email: 'test@example.com',
      password: 'password123'
    });
    expect(loginResponse.token).toBeDefined();

    // 3. Access protected endpoint
    const profileResponse = await apiClient.getProfile(loginResponse.token);
    expect(profileResponse.email).toBe('test@example.com');
  });
});
```

### Health Check Integration

```typescript
// File: tests/integration/health_check.test.ts
describe('Health Check Integration', () => {
  it('should verify backend health', async () => {
    const response = await fetch('http://localhost:8000/health/live');
    expect(response.status).toBe(200);
    
    const data = await response.json();
    expect(data.status).toBe('ok');
  });

  it('should verify mobile app loads', async () => {
    // Test if mobile app can reach backend
    const apiClient = new ApiClient('http://localhost:8000');
    const health = await apiClient.health();
    expect(health.status).toBe('ok');
  });
});
```

---

## 🌐 End-to-End Testing

### User Journey Tests

```typescript
// File: tests/e2e/user_journey.test.ts
describe('Complete User Journey', () => {
  it('should handle new user registration and first check-in', async () => {
    // 1. Launch app
    await device.launchApp();
    
    // 2. Register new account
    await element(by.id('register-button')).tap();
    await element(by.id('email-input')).typeText('user@example.com');
    await element(by.id('password-input')).typeText('password123');
    await element(by.id('submit-button')).tap();
    
    // 3. Verify successful registration
    await expect(element(by.text('Welcome!'))).toBeVisible();
    
    // 4. Perform first check-in
    await element(by.id('check-in-button')).tap();
    await expect(element(by.text('13'))).toBeVisible(); // Streak increased
  });
});
```

---

## 🛠 Test Environment Setup

### Backend Test Environment

```bash
# Create test database
createdb app_virgin_test

# Set test environment variables
export DATABASE_URL=postgresql://localhost/app_virgin_test
export JWT_SECRET=test_secret_key
export ENVIRONMENT=test

# Run database migrations for tests
diesel migration run --database-url $DATABASE_URL
```

### Mobile Test Environment

```bash
# Install test dependencies
cd mobile
npm install --save-dev @testing-library/react-native @testing-library/jest-native detox

# Setup Detox for E2E tests
detox init --configuration ios.sim.debug,android.emu.debug

# Build test app
detox build --configuration ios.sim.debug
```

### Test Data Management

```rust
// File: backend/tests/fixtures.rs
use diesel::prelude::*;

pub struct TestFixtures {
    pub pool: PgPool,
}

impl TestFixtures {
    pub async fn setup() -> Self {
        let pool = create_test_pool().await;
        
        // Clean database before each test
        let mut conn = pool.get().unwrap();
        conn.execute("TRUNCATE TABLE users, sessions CASCADE").unwrap();
        
        Self { pool }
    }

    pub async fn create_test_user(&self) -> User {
        let mut conn = self.pool.get().unwrap();
        
        let user = NewUser {
            email: "test@example.com".to_string(),
            password_hash: hash_password("password123"),
        };

        create_user(&mut conn, &user).await.unwrap()
    }
}
```

---

## 🚀 CI/CD Testing

### GitHub Actions Configuration

```yaml
# File: .github/workflows/test.yml
name: Test Suite

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  backend-tests:
    runs-on: ubuntu-latest
    
    services:
      postgres:
        image: postgres:14
        env:
          POSTGRES_PASSWORD: postgres
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Run backend tests
      run: |
        cd backend
        cargo test --verbose
      env:
        DATABASE_URL: postgresql://postgres:postgres@localhost/postgres

  mobile-tests:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
        cache: 'npm'
        cache-dependency-path: mobile/package-lock.json
        
    - name: Install dependencies
      run: |
        cd mobile
        npm ci
        
    - name: Run mobile tests
      run: |
        cd mobile
        npm test -- --coverage
        
    - name: Upload coverage
      uses: codecov/codecov-action@v3
      with:
        directory: mobile/coverage
```

---

## 🐛 Troubleshooting

### Common Test Issues

#### Backend Test Issues

**Issue: Database connection failed**
```bash
# Solution: Check test database setup
psql -h localhost -U postgres -c "CREATE DATABASE app_virgin_test;"

# Verify connection string
echo $DATABASE_URL
```

**Issue: Test hangs on async operations**
```rust
// Solution: Use tokio test macro
#[tokio::test]
async fn test_async_operation() {
    // Test code here
}
```

#### Mobile Test Issues

**Issue: "Cannot find module" errors**
```bash
# Solution: Clear Jest cache
cd mobile
npm test -- --clearCache

# Reinstall dependencies
rm -rf node_modules package-lock.json
npm install
```

**Issue: Detox tests fail to find elements**
```typescript
// Solution: Add test IDs to components
<Button testID="check-in-button" onPress={handleCheckIn}>
  Check In Today
</Button>

// Then in tests:
await element(by.id('check-in-button')).tap();
```

**Issue: Metro bundler conflicts**
```bash
# Solution: Kill existing Metro processes
npx react-native start --reset-cache

# Or use different port
npx react-native start --port 8082
```

### Performance Testing

```bash
# Backend load testing
cd backend
cargo install cargo-flamegraph
cargo flamegraph --bin backend

# Mobile performance profiling
cd mobile
npx react-native-bundle-visualizer
```

### Test Coverage

```bash
# Backend coverage
cd backend
cargo install cargo-tarpaulin
cargo tarpaulin --out Html

# Mobile coverage
cd mobile
npm test -- --coverage --coverageReporters=html
```

---

## 📊 Test Metrics

### Coverage Targets

- **Backend**: 90%+ line coverage
- **Mobile**: 85%+ line coverage
- **Integration**: 100% critical path coverage

### Performance Benchmarks

- **Backend API**: <100ms response time
- **Mobile startup**: <3s cold start
- **E2E tests**: <30s total execution

---

**Happy Testing! 🧪**

Remember: Good tests are investments in future development speed and confidence.
