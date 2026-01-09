# Frontend-Backend Integration Guide
## SOTA 2026 Monorepo — Type-Safe End-to-End Development

> **Purpose**: Step-by-step guide for connecting frontend features to the Rust backend while maintaining full type safety.

---

## Table of Contents

1. [Overview](#1-overview)
2. [Type Generation Flow](#2-type-generation-flow)
3. [Adding a New API Endpoint](#3-adding-a-new-api-endpoint)
4. [Creating Frontend Hooks](#4-creating-frontend-hooks)
5. [Building a New Feature](#5-building-a-new-feature)
6. [Error Handling](#6-error-handling)
7. [Testing the Integration](#7-testing-the-integration)
8. [Checklist](#8-checklist)

---

## 1. Overview

### The Golden Rule

```
Rust Types → TypeScript Types → React Query Hooks → Components
     ↑                                                    ↓
     └────────── NEVER manually duplicate types ──────────┘
```

### Architecture Summary

| Layer | Technology | Location |
|-------|------------|----------|
| **Database** | PostgreSQL + Diesel | `backend/src/db.rs` |
| **API** | Rust + Axum | `backend/src/features/*/` |
| **Type Bridge** | ts-rs | Auto-generates `.ts` files |
| **Data Layer** | React Query | `mobile/src/api/client.ts` |
| **UI** | React Native | `mobile/src/features/*/` |

---

## 2. Type Generation Flow

### How Types Flow from Rust to TypeScript

```
┌─────────────────────────────────────────────────────────────────┐
│ BACKEND (Rust)                                                  │
│                                                                 │
│   #[derive(Serialize, Deserialize, TS)]                         │
│   #[ts(export)]                                                 │
│   pub struct User {                                             │
│       pub id: i32,                                              │
│       pub email: String,                                        │
│       pub name: Option<String>,                                 │
│   }                                                             │
│                                                                 │
│   $ cargo test  ──────────────────────────────────────────┐     │
└───────────────────────────────────────────────────────────│─────┘
                                                            │
                                                            ▼
┌─────────────────────────────────────────────────────────────────┐
│ GENERATED (mobile/src/api/types/)                               │
│                                                                 │
│   // User.ts (AUTO-GENERATED - DO NOT EDIT)                     │
│   export interface User {                                       │
│       id: number;                                               │
│       email: string;                                            │
│       name: string | null;                                      │
│   }                                                             │
└─────────────────────────────────────────────────────────────────┘
                                                            │
                                                            ▼
┌─────────────────────────────────────────────────────────────────┐
│ FRONTEND (React Native)                                         │
│                                                                 │
│   import type { User } from '@/api/types';                      │
│                                                                 │
│   function useUser(id: number) {                                │
│       return useQuery<User>({ ... });                           │
│   }                                                             │
└─────────────────────────────────────────────────────────────────┘
```

### Commands

```bash
# Generate TypeScript types from Rust
cd backend && cargo test

# Types appear in:
# mobile/src/api/types/*.ts
```

---

## 3. Adding a New API Endpoint

### Step 1: Define the Rust Types

```rust
// backend/src/features/products/domain/entities.rs

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../mobile/src/api/types/")]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price_cents: i32,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "../mobile/src/api/types/")]
pub struct CreateProductRequest {
    pub name: String,
    pub price_cents: i32,
    pub description: Option<String>,
}
```

### Step 2: Create the Axum Handler

```rust
// backend/src/features/products/api/handlers.rs

use axum::{extract::Path, Json};
use crate::features::products::domain::entities::{Product, CreateProductRequest};

pub async fn get_product(Path(id): Path<i32>) -> Json<Product> {
    // Database query here
    Json(product)
}

pub async fn create_product(Json(req): Json<CreateProductRequest>) -> Json<Product> {
    // Database insert here
    Json(new_product)
}
```

### Step 3: Register the Route

```rust
// backend/src/features/products/api/routes.rs

use axum::{routing::{get, post}, Router};
use super::handlers::{get_product, create_product};

pub fn product_routes() -> Router {
    Router::new()
        .route("/products/:id", get(get_product))
        .route("/products", post(create_product))
}
```

### Step 4: Generate Types

```bash
cd backend && cargo test
# Check: mobile/src/api/types/Product.ts exists
```

---

## 4. Creating Frontend Hooks

### Step 1: Add Types to Barrel Export

```typescript
// mobile/src/api/types/index.ts

export * from './User';
export * from './Product';  // Add new type
```

### Step 2: Create Query Keys Factory

```typescript
// mobile/src/api/client.ts

export const queryKeys = {
  all: ['api'] as const,
  
  // Existing
  users: () => [...queryKeys.all, 'users'] as const,
  user: (id: number) => [...queryKeys.users(), id] as const,
  
  // New: Products
  products: () => [...queryKeys.all, 'products'] as const,
  product: (id: number) => [...queryKeys.products(), id] as const,
  productsByCategory: (category: string) => [...queryKeys.products(), 'category', category] as const,
};
```

### Step 3: Create the Hook

```typescript
// mobile/src/api/client.ts

import type { Product, CreateProductRequest } from './types';

// Internal fetcher (not exported)
const fetchProduct = async (id: number): Promise<Product> => {
  const response = await apiClient.get<Product>(`/api/v1/products/${id}`);
  return response.data;
};

// Exported hook (the ONLY way to fetch products)
export function useProduct(id: number) {
  return useQuery({
    queryKey: queryKeys.product(id),
    queryFn: () => fetchProduct(id),
    staleTime: 5 * 60 * 1000,
  });
}

// Mutation hook
export function useCreateProduct() {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: async (data: CreateProductRequest): Promise<Product> => {
      const response = await apiClient.post<Product>('/api/v1/products', data);
      return response.data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.products() });
    },
  });
}
```

---

## 5. Building a New Feature

### Directory Structure

```
mobile/src/features/products/
├── components/
│   ├── ProductCard.tsx
│   ├── ProductList.tsx
│   └── index.ts
├── screens/
│   ├── ProductListScreen.tsx
│   ├── ProductDetailScreen.tsx
│   └── index.ts
├── hooks/
│   └── useProductFilters.ts  (local UI state only)
└── index.ts
```

### Feature Template

```typescript
// mobile/src/features/products/screens/ProductListScreen.tsx

import React from 'react';
import { View, FlatList, RefreshControl } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';

// API hooks from centralized client
import { useProducts } from '@/api/client';

// Shared components from design system
import { ErrorState, Skeleton } from '@/shared/components';
import { colors, spacing } from '@/shared/theme';

// Feature-specific components
import { ProductCard } from '../components';

export function ProductListScreen() {
  const { data, isLoading, isError, error, refetch, isFetching } = useProducts();

  // STATE 1: Loading
  if (isLoading) {
    return <ProductListSkeleton />;
  }

  // STATE 2: Error
  if (isError) {
    return (
      <ErrorState
        title="Couldn't load products"
        message={error?.message}
        onRetry={refetch}
      />
    );
  }

  // STATE 3: Empty
  if (!data || data.length === 0) {
    return <EmptyState message="No products yet" />;
  }

  // STATE 4: Success
  return (
    <SafeAreaView style={{ flex: 1, backgroundColor: colors.background.primary }}>
      <FlatList
        data={data}
        keyExtractor={(item) => item.id.toString()}
        renderItem={({ item }) => <ProductCard product={item} />}
        refreshControl={
          <RefreshControl refreshing={isFetching} onRefresh={refetch} />
        }
        contentContainerStyle={{ padding: spacing[4] }}
      />
    </SafeAreaView>
  );
}
```

---

## 6. Error Handling

### Backend Errors

```rust
// backend/src/features/common/errors.rs

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS)]
#[ts(export, export_to = "../mobile/src/api/types/")]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = match self.code.as_str() {
            "NOT_FOUND" => StatusCode::NOT_FOUND,
            "UNAUTHORIZED" => StatusCode::UNAUTHORIZED,
            "VALIDATION_ERROR" => StatusCode::UNPROCESSABLE_ENTITY,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(self)).into_response()
    }
}
```

### Frontend Error Handling

```typescript
// mobile/src/api/client.ts (interceptor)

apiClient.interceptors.response.use(
  (response) => response,
  async (error: AxiosError<ApiError>) => {
    const apiError = error.response?.data;
    
    // Log for debugging
    console.error('API Error:', apiError);
    
    // Handle specific error codes
    if (apiError?.code === 'UNAUTHORIZED') {
      await TokenStorage.delete();
      // Navigate to login
    }
    
    return Promise.reject(error);
  }
);
```

### Component-Level Error Display

```typescript
// In your screen
const { error } = useProduct(id);

// error.response?.data is typed as ApiError
<ErrorState
  title={error?.response?.data?.code || 'Error'}
  message={error?.response?.data?.message || 'Something went wrong'}
/>
```

---

## 7. Testing the Integration

### Backend Test

```bash
# Start backend
cd backend && cargo run

# Test endpoint
curl http://localhost:8000/api/v1/products/1
```

### Frontend Test

```typescript
// mobile/src/features/products/__tests__/useProduct.test.ts

import { renderHook, waitFor } from '@testing-library/react-native';
import { useProduct } from '@/api/client';
import { wrapper } from '@/test/utils';

describe('useProduct', () => {
  it('fetches product by id', async () => {
    const { result } = renderHook(() => useProduct(1), { wrapper });
    
    await waitFor(() => expect(result.current.isSuccess).toBe(true));
    
    expect(result.current.data).toEqual({
      id: 1,
      name: 'Test Product',
      price_cents: 1000,
    });
  });
});
```

### Type Safety Test

```bash
# This should catch any type mismatches
cd mobile && npm run type-check
```

---

## 8. Checklist

### Adding a New Feature

- [ ] **Backend**
  - [ ] Define Rust structs with `#[derive(TS)]`
  - [ ] Add `#[ts(export)]` attribute
  - [ ] Create handler functions
  - [ ] Register routes
  - [ ] Run `cargo test` to generate types

- [ ] **Type Bridge**
  - [ ] Verify `.ts` files generated in `mobile/src/api/types/`
  - [ ] Add to barrel export (`index.ts`)
  - [ ] Run `npm run type-check` in mobile

- [ ] **Frontend Data Layer**
  - [ ] Add query keys to factory
  - [ ] Create internal fetcher function
  - [ ] Create exported hook (`useXxx`)
  - [ ] Create mutation hook if needed (`useCreateXxx`, `useUpdateXxx`)

- [ ] **Frontend UI**
  - [ ] Create feature folder structure
  - [ ] Build components using shared design system
  - [ ] Handle all 6 UI states
  - [ ] Add accessibility props
  - [ ] Write tests

### Before Merging

- [ ] Types match between backend and frontend (run `type-check`)
- [ ] No hardcoded API URLs (use `apiClient`)
- [ ] No manual type definitions (use generated types)
- [ ] Error states handled gracefully
- [ ] Loading states show skeletons
- [ ] Tests pass

---

## Quick Reference

### File Locations

| What | Where |
|------|-------|
| Rust entities | `backend/src/features/[feature]/domain/entities.rs` |
| Rust handlers | `backend/src/features/[feature]/api/handlers.rs` |
| Generated types | `mobile/src/api/types/*.ts` |
| API hooks | `mobile/src/api/client.ts` |
| Feature screens | `mobile/src/features/[feature]/screens/` |
| Shared components | `mobile/src/shared/components/` |
| Design tokens | `mobile/src/shared/theme.ts` |

### Commands

```bash
# Generate types from Rust
cd backend && cargo test

# Type check frontend
cd mobile && npm run type-check

# Start backend
cd backend && cargo run

# Start mobile (web)
cd mobile && npm run web

# Start mobile (native)
cd mobile && npm start
```

---

*This guide ensures type safety flows from database to UI. Never skip steps or manually create types.*
