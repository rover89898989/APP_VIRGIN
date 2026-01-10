# ⚡ Performance & Optimization Guide - APP_VIRGIN Template

**Last Updated:** January 10, 2026  
**Purpose:** Performance optimization strategies for backend and mobile

---

## 📋 Table of Contents

1. [Backend Performance](#backend-performance)
2. [Mobile Performance](#mobile-performance)
3. [Database Optimization](#database-optimization)
4. [Bundle Size Optimization](#bundle-size-optimization)
5. [Caching Strategies](#caching-strategies)
6. [Monitoring & Profiling](#monitoring--profiling)

---

## 🚀 Backend Performance

### Database Connection Pooling

```rust
// File: backend/src/db.rs
use diesel::r2d2::{Pool, ConnectionManager};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_pool(database_url: &str) -> DbPool {
    Pool::builder()
        .max_size(20)  // Adjust based on load
        .min_idle(Some(5))
        .connection_timeout(Duration::from_secs(30))
        .idle_timeout(Some(Duration::from_secs(600)))
        .max_lifetime(Some(Duration::from_secs(1800)))
        .build(ConnectionManager::new(database_url))
        .expect("Failed to create pool")
}
```

### Async Request Handling

```rust
// File: backend/src/main.rs
use tokio::time::timeout;

pub async fn with_timeout<F, T>(
    duration: Duration,
    future: F,
) -> Result<T, ApiError>
where
    F: Future<Output = Result<T, ApiError>>,
{
    timeout(duration, future)
        .await
        .map_err(|_| ApiError::Timeout)?
}
```

### Response Compression

```rust
// File: backend/src/middleware/compression.rs
use axum::middleware;
use axum::response::Response;

pub async fn compression_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let response = next.run(req).await;
    
    // Compress responses larger than 1KB
    if response.body().size_hint().lower() >= 1024 {
        // Add compression headers and compress body
        Ok(compress_response(response))
    } else {
        Ok(response)
    }
}
```

---

## 📱 Mobile Performance

### Image Optimization

```typescript
// File: mobile/src/components/OptimizedImage.tsx
import React, { memo } from 'react';
import { Image, ImageProps } from 'react-native';

interface OptimizedImageProps extends Omit<ImageProps, 'source'> {
  uri: string;
  width: number;
  height: number;
}

export const OptimizedImage = memo<OptimizedImageProps>(({ 
  uri, 
  width, 
  height, 
  ...props 
}) => {
  return (
    <Image
      {...props}
      source={{ 
        uri: `${uri}?w=${width}&h=${height}&format=webp&quality=80` 
      }}
      style={[props.style, { width, height }]}
      resizeMode="cover"
    />
  );
});
```

### List Virtualization

```typescript
// File: mobile/src/components/VirtualizedList.tsx
import React from 'react';
import { FlatList, FlatListProps } from 'react-native';

interface VirtualizedListProps<T> extends Omit<FlatListProps<T>, 'data'> {
  data: T[];
  renderItem: ({ item, index }: { item: T; index: number }) => React.ReactElement;
}

export function VirtualizedList<T>({ 
  data, 
  renderItem, 
  ...props 
}: VirtualizedListProps<T>) {
  return (
    <FlatList
      data={data}
      renderItem={renderItem}
      keyExtractor={(item, index) => index.toString()}
      getItemLayout={(data, index) => ({
        length: 70, // Approximate item height
        offset: 70 * index,
        index,
      })}
      initialNumToRender={10}
      maxToRenderPerBatch={10}
      windowSize={10}
      removeClippedSubviews={true}
      {...props}
    />
  );
}
```

### State Management Optimization

```typescript
// File: mobile/src/hooks/useOptimizedQuery.ts
import { useQuery } from '@tanstack/react-query';

export const useOptimizedQuery = <T>(
  queryKey: string[],
  queryFn: () => Promise<T>,
  options?: {
    staleTime?: number;
    cacheTime?: number;
    refetchOnWindowFocus?: boolean;
  }
) => {
  return useQuery({
    queryKey,
    queryFn,
    staleTime: options?.staleTime || 5 * 60 * 1000, // 5 minutes
    cacheTime: options?.cacheTime || 30 * 60 * 1000, // 30 minutes
    refetchOnWindowFocus: options?.refetchOnWindowFocus || false,
    retry: 2,
    ...options,
  });
};
```

---

## 🗄 Database Optimization

### Index Strategy

```sql
-- File: backend/migrations/0003_performance_indexes.sql

-- User queries
CREATE INDEX CONCURRENTLY idx_users_email_active ON users(email) WHERE deleted_at IS NULL;
CREATE INDEX CONCURRENTLY idx_users_created_at_desc ON users(created_at DESC);

-- Session queries
CREATE INDEX CONCURRENTLY idx_sessions_user_expires ON sessions(user_id, expires_at);
CREATE INDEX CONCURRENTLY idx_sessions_refresh_token_hash ON sessions(refresh_token_hash);

-- Composite indexes for common queries
CREATE INDEX CONCURRENTLY idx_users_email_verified ON users(email, email_verified) WHERE deleted_at IS NULL;
```

### Query Optimization

```rust
// File: backend/src/features/users/repository.rs
use diesel::prelude::*;
use diesel::sql_types::{Integer, Timestamp};

pub async fn find_active_users_with_pagination(
    page: i64,
    limit: i64,
) -> Result<Vec<User>, ApiError> {
    use crate::schema::users::dsl::*;
    
    let offset = (page - 1) * limit;
    
    let results = users
        .filter(deleted_at.is_null())
        .order(created_at.desc())
        .limit(limit)
        .offset(offset)
        .load::<User>(&mut get_connection()?)?;
    
    Ok(results)
}

// Use raw SQL for complex queries
pub async fn get_user_stats() -> Result<UserStats, ApiError> {
    let stats = sql_query!(
        "SELECT 
            COUNT(*) as total_users,
            COUNT(CASE WHEN created_at > NOW() - INTERVAL '30 days' THEN 1 END) as new_users_30d,
            COUNT(CASE WHEN last_login_at > NOW() - INTERVAL '7 days' THEN 1 END) as active_users_7d
         FROM users 
         WHERE deleted_at IS NULL"
    )
    .get_result(&mut get_connection()?)?;
    
    Ok(stats)
}
```

---

## 📦 Bundle Size Optimization

### Web Bundle Analysis

```bash
# Analyze bundle size
cd mobile
npm run build

# Use webpack-bundle-analyzer
npm install --save-dev webpack-bundle-analyzer
npx webpack-bundle-analyzer dist/static/js/*.js
```

### Tree Shaking Configuration

```javascript
// File: mobile/webpack.config.js
module.exports = {
  mode: 'production',
  optimization: {
    usedExports: true,
    sideEffects: false,
    splitChunks: {
      chunks: 'all',
      cacheGroups: {
        vendor: {
          test: /[\\/]node_modules[\\/]/,
          name: 'vendors',
          chunks: 'all',
        },
      },
    },
  },
};
```

### Dynamic Imports

```typescript
// File: mobile/src/App.tsx
import React, { Suspense, lazy } from 'react';

const LazyComponent = lazy(() => import('./components/HeavyComponent'));

export const App = () => {
  return (
    <Suspense fallback={<LoadingSpinner />}>
      <LazyComponent />
    </Suspense>
  );
};
```

---

## 🗂 Caching Strategies

### Redis Caching

```rust
// File: backend/src/cache.rs
use redis::{Client, Commands};

pub struct CacheService {
    client: Client,
}

impl CacheService {
    pub async fn get<T>(&self, key: &str) -> Result<Option<T>, ApiError>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut conn = self.client.get_async_connection().await?;
        let value: Option<String> = conn.get(key).await?;
        
        match value {
            Some(v) => Ok(Some(serde_json::from_str(&v)?)),
            None => Ok(None),
        }
    }
    
    pub async fn set<T>(&self, key: &str, value: &T, ttl: u64) -> Result<(), ApiError>
    where
        T: serde::Serialize,
    {
        let mut conn = self.client.get_async_connection().await?;
        let serialized = serde_json::to_string(value)?;
        
        conn.set_ex(key, serialized, ttl).await?;
        Ok(())
    }
}
```

### React Query Caching

```typescript
// File: mobile/src/api/cached-client.ts
import { QueryClient } from '@tanstack/react-query';

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 5 * 60 * 1000, // 5 minutes
      cacheTime: 30 * 60 * 1000, // 30 minutes
      refetchOnWindowFocus: false,
      refetchOnReconnect: true,
      retry: (failureCount, error) => {
        if (error.status === 404) return false;
        return failureCount < 3;
      },
    },
  },
});
```

---

## 📊 Monitoring & Profiling

### Performance Metrics

```rust
// File: backend/src/middleware/metrics.rs
use prometheus::{Counter, Histogram, Gauge};

lazy_static! {
    static ref HTTP_REQUESTS_TOTAL: Counter = Counter::new(
        "http_requests_total",
        "Total number of HTTP requests"
    ).unwrap();
    
    static ref HTTP_REQUEST_DURATION: Histogram = Histogram::with_opts(
        prometheus::HistogramOpts::new(
            "http_request_duration_seconds",
            "HTTP request duration"
        ).buckets(vec![0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0])
    ).unwrap();
    
    static ref ACTIVE_CONNECTIONS: Gauge = Gauge::new(
        "active_connections",
        "Number of active connections"
    ).unwrap();
}
```

### Mobile Performance Monitoring

```typescript
// File: mobile/src/utils/performance-monitor.ts
export class PerformanceMonitor {
  static measureRender(componentName: string) {
    const start = performance.now();
    
    return () => {
      const duration = performance.now() - start;
      
      if (__DEV__) {
        console.log(`[PERF] ${componentName}: ${duration.toFixed(2)}ms`);
      }
      
      // Send to monitoring service in production
      if (duration > 100) {
        this.reportSlowRender(componentName, duration);
      }
    };
  }
  
  static measureApiCall<T>(
    apiCall: () => Promise<T>,
    apiName: string
  ): Promise<T> {
    return new Promise(async (resolve, reject) => {
      const start = performance.now();
      
      try {
        const result = await apiCall();
        const duration = performance.now() - start;
        
        this.reportApiMetric(apiName, duration, true);
        resolve(result);
      } catch (error) {
        const duration = performance.now() - start;
        
        this.reportApiMetric(apiName, duration, false);
        reject(error);
      }
    });
  }
  
  private static reportSlowRender(component: string, duration: number) {
    // Send to monitoring service
  }
  
  private static reportApiMetric(api: string, duration: number, success: boolean) {
    // Send to monitoring service
  }
}
```

---

## 🎯 Performance Targets

### Backend Performance

| Metric | Target | Alert Threshold |
|--------|--------|-----------------|
| API Response Time | <100ms (p95) | >200ms |
| Database Query Time | <50ms (p95) | >100ms |
| Memory Usage | <512MB | >1GB |
| CPU Usage | <70% | >85% |
| Error Rate | <0.1% | >1% |

### Mobile Performance

| Metric | Target | Alert Threshold |
|--------|--------|-----------------|
| App Startup Time | <3s | >5s |
| Screen Render Time | <16ms | >100ms |
| Bundle Size | <5MB | >10MB |
| Memory Usage | <200MB | >400MB |
| API Response Time | <500ms | >2s |

---

**Happy Optimizing! ⚡**

Remember: Performance optimization is an ongoing process. Monitor, measure, and improve continuously.
