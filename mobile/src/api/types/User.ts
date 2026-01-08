// ==============================================================================
// AUTO-GENERATED TYPES FROM RUST BACKEND
// ==============================================================================
//
// CRITICAL: These types are generated from Rust backend
// Location: backend/src/features/users/domain/entities.rs
//
// When backend changes:
// 1. Run: cd backend && cargo test
// 2. Types regenerate automatically
// 3. TypeScript compiler catches breaking changes
// 4. Fix errors before deploying
//
// This prevents:
// - Runtime errors from API changes
// - Manual type synchronization (error-prone)
// - Guessing what fields exist
//
// TEMPORARY PLACEHOLDER:
// - Replace with actual generated types from Rust backend
// - Do NOT manually edit these types in production
//
// ==============================================================================

export interface User {
  id: number;
  email: string;
  name: string;
  // Add more fields as generated from Rust
}

export interface UserResponse {
  user: User;
  id: number; // Added for compatibility with existing code
  // Add response fields as generated from Rust
}

export interface CreateUserRequest {
  email: string;
  password: string;
  name: string;
  // Add more fields as generated from Rust
}

export interface UpdateUserRequest {
  email?: string;
  name?: string;
  // Add more fields as generated from Rust
}
