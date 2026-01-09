// ==============================================================================
// API CLIENT - TYPE-SAFE HTTP REQUESTS
// ==============================================================================
//
// This file provides a type-safe API client for communicating with the Rust backend.
//
// KEY FEATURES:
// 1. Auto-generated types from Rust (NEVER manually write types!)
// 2. Centralized axios configuration
// 3. Request/response interceptors
// 4. Error handling
// 5. Authentication token management
//
// TYPE GENERATION FLOW:
// 1. Rust backend: #[derive(TS)] on User struct
// 2. cargo test → generates User.ts in this directory
// 3. Import generated types: import { User } from './types/User'
// 4. TypeScript compiler catches any API mismatches
//
// ARCHITECTURE:
// - This is the INFRASTRUCTURE layer for the mobile app
// - Business logic lives in /features/*
// - This file handles HTTP communication only
//
// ==============================================================================

import axios, { AxiosInstance, AxiosError, AxiosRequestConfig } from 'axios';
// SECURITY FIX: Use expo-secure-store instead of AsyncStorage
// AsyncStorage is UNENCRYPTED (XML file on Android, Plist on iOS)
// SecureStore uses iOS Keychain and Android KeyStore (hardware-backed encryption)
// Install: npx expo install expo-secure-store
import { Platform } from 'react-native';

import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';

type SecureStoreModule = typeof import('expo-secure-store');
let secureStoreModule: SecureStoreModule | null = null;

async function getSecureStore(): Promise<SecureStoreModule> {
  if (secureStoreModule) return secureStoreModule;
  const mod = await import('expo-secure-store');
  secureStoreModule = mod;
  return mod;
}

const TokenStorage = {
  async get(): Promise<string | null> {
    if (Platform.OS === 'web') {
      return localStorage.getItem('access_token');
    }

    const SecureStore = await getSecureStore();
    return await SecureStore.getItemAsync('access_token');
  },

  async set(token: string): Promise<void> {
    if (Platform.OS === 'web') {
      localStorage.setItem('access_token', token);
      return;
    }

    const SecureStore = await getSecureStore();
    await SecureStore.setItemAsync('access_token', token);
  },

  async delete(): Promise<void> {
    if (Platform.OS === 'web') {
      localStorage.removeItem('access_token');
      return;
    }

    const SecureStore = await getSecureStore();
    await SecureStore.deleteItemAsync('access_token');
  },
};

// ==============================================================================
// IMPORT AUTO-GENERATED TYPES FROM RUST
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
// ==============================================================================

import { User } from './types/User';
import { UserResponse } from './types/UserResponse';
import { CreateUserRequest } from './types/CreateUserRequest';
import { UpdateUserRequest } from './types/UpdateUserRequest';

// ==============================================================================
// API CONFIGURATION
// ==============================================================================

/**
 * Base API URL
 * 
 * IMPORTANT: process.env does NOT work in React Native!
 * React Native doesn't have Node.js's process.env.
 * 
 * Options for configuration:
 * 1. react-native-config (recommended): Access .env files
 * 2. react-native-dotenv: Babel plugin for .env
 * 3. Hardcode for now, configure later
 * 
 * Environment-specific URLs:
 * - Development (iOS Simulator): http://localhost:8000
 * - Development (Android Emulator): http://10.0.2.2:8000
 * - Development (Physical Device): http://<YOUR_IP>:8000
 * - Production: https://api.yourapp.com
 * 
 * For now, we hardcode the development URL.
 * TODO: Set up react-native-config for proper environment configuration
 */
const API_BASE_URL = __DEV__ 
  ? 'http://localhost:8000'  // Development: local backend
  : 'https://api.yourapp.com';  // Production: replace with your API URL

/**
 * API timeout (milliseconds)
 * 
 * 30 seconds is reasonable for:
 * - Most API calls (< 1 second typically)
 * - Large file uploads (need more time)
 * - Slow mobile networks
 * 
 * If exceeded, request fails with timeout error
 */
const API_TIMEOUT = 30000;

// ==============================================================================
// AXIOS INSTANCE CONFIGURATION
// ==============================================================================
//
// Creates a pre-configured axios instance with:
// - Base URL (don't repeat in every request)
// - Timeout (fail fast if server is down)
// - Headers (JSON content type)
//
// ==============================================================================

const apiClient: AxiosInstance = axios.create({
  baseURL: API_BASE_URL,
  timeout: API_TIMEOUT,
  headers: {
    'Content-Type': 'application/json',
  },
});

// ==============================================================================
// REQUEST INTERCEPTOR
// ==============================================================================
//
// Runs BEFORE every request.
// Attaches authentication token from ENCRYPTED storage.
//
// SECURITY FIX:
// - OLD: AsyncStorage (UNENCRYPTED - XML/Plist files)
// - NEW: SecureStore (iOS Keychain, Android KeyStore, hardware-backed)
// - Stolen phone with AsyncStorage = stolen session
// - Stolen phone with SecureStore = tokens still protected
//
// FLOW:
// 1. User logs in → JWT token stored in SecureStore
// 2. User makes API request
// 3. This interceptor retrieves token from secure storage
// 4. Adds: Authorization: Bearer <token>
// 5. Backend validates token
// 6. Request proceeds if valid, fails if invalid/expired
//
// ==============================================================================

apiClient.interceptors.request.use(
  async (config) => {
    // CRITICAL: SecureStore failure is a catastrophic OS-level issue
    // Possible causes:
    // - Device lock/encryption disabled
    // - OS keychain corruption  
    // - Permission denied
    // 
    // We must NOT send partial auth requests - fail fast
    try {
      // Use encrypted hardware-backed storage
      // iOS: Keychain (Touch ID/Face ID protected)
      // Android: KeyStore (hardware security module when available)
      const token = await TokenStorage.get();
      
      if (token) {
        // Add Authorization header if token exists
        // Format: "Bearer <token>" (industry standard JWT)
        config.headers.Authorization = `Bearer ${token}`;
        
        // Log request in development (helpful for debugging)
        if (__DEV__) {
          console.log(`📤 ${config.method?.toUpperCase()} ${config.url}`);
        }
      }
    } catch (error) {
      // SecureStore failure is CRITICAL - don't continue
      // This prevents sending requests with missing/partial auth
      console.error('❌ CRITICAL: Storage access failed:', error);
      return Promise.reject(new Error('Storage access failed - cannot authenticate'));
    }
    
    return config;
  },
  (error) => {
    // Request failed before being sent (rare)
    // Example: Network offline before request
    console.error('❌ Request error:', error);
    return Promise.reject(error);
  }
);

// ==============================================================================
// RESPONSE INTERCEPTOR
// ==============================================================================
//
// Runs AFTER every response.
// Handles errors globally (don't repeat in every component).
//
// ERROR HANDLING STRATEGY:
// - 401 Unauthorized → Token expired, redirect to login
// - 403 Forbidden → User doesn't have permission
// - 404 Not Found → Resource doesn't exist
// - 422 Validation Error → Invalid input
// - 500 Server Error → Backend crashed
// - Network Error → No internet or server down
//
// ==============================================================================

apiClient.interceptors.response.use(
  (response) => {
    // Success response (2xx status code)
    // Log in development for debugging
    if (__DEV__) {
      console.log(`✅ ${response.config.method?.toUpperCase()} ${response.config.url} - ${response.status}`);
    }
    
    return response;
  },
  async (error: AxiosError) => {
    // Error response (4xx, 5xx status codes) or network error
    
    if (error.response) {
      // Server responded with error status code
      const status = error.response.status;
      const url = error.config?.url;
      
      if (__DEV__) {
        console.error(`❌ ${error.config?.method?.toUpperCase()} ${url} - ${status}`);
      }
      
      // Handle specific error codes
      switch (status) {
        case 401:
          // Unauthorized: Token expired or invalid
          // Clear token and redirect to login
          // ✅ FIX: Platform-aware token deletion
          await TokenStorage.delete();
          // TODO: Trigger navigation to login screen
          // navigationRef.current?.navigate('Login');
          break;
          
        case 403:
          // Forbidden: User doesn't have permission
          // Show error message
          console.error('Permission denied');
          break;
          
        case 404:
          // Not Found: Resource doesn't exist
          // Could be: user deleted, URL typo, etc.
          console.error('Resource not found');
          break;
          
        case 422:
          // Validation Error: Invalid input
          // Backend returned validation errors (email format, etc.)
          console.error('Validation error:', error.response.data);
          break;
          
        case 500:
        case 502:
        case 503:
          // Server Error: Backend crashed or unavailable
          // Show friendly error message to user
          console.error('Server error, please try again later');
          break;
      }
    } else if (error.request) {
      // Request made but no response received
      // Causes: No internet, server down, timeout
      console.error('Network error: No response from server');
      console.error('Check internet connection or server status');
    } else {
      // Error setting up request (rare)
      console.error('Request setup error:', error.message);
    }
    
    // Re-throw error for component to handle
    // Component can show user-friendly error message
    return Promise.reject(error);
  }
);

// ==============================================================================
// API SERVICE FUNCTIONS
// ==============================================================================
//
// Type-safe API calls using generated types from Rust.
// Each function:
// 1. Takes typed parameters
// 2. Makes HTTP request
// 3. Returns typed response
// 4. TypeScript catches type errors at compile-time
//
// ARCHITECTURE FIX:
// - OLD: Mixed imperative (userApi.getById) with declarative (useUser)
// - NEW: Enforce "hook-only" pattern for reads
// - Reads (GET): Internal functions → Only hooks exported
// - Mutations (POST/PUT/DELETE): Exported in userApi
// - Why? Prevents bypassing React Query cache
//
// ==============================================================================

// ==============================================================================
// INTERNAL FETCHERS (Do not export - use hooks instead)
// ==============================================================================
//
// These are internal functions used by React Query hooks.
// Never call these directly from components - always use the hooks.
// 
// Why?
// - Calling fetchUser() directly bypasses cache
// - UI shows stale data
// - Multiple components fetch same data
// - Network waste, inconsistent state
//
// Always use: useUser() hook instead
//
// ==============================================================================

/**
 * Internal: Fetch user by ID
 * 
 * DO NOT USE DIRECTLY - Use useUser() hook instead!
 * This is internal to React Query and should never be called from components.
 */
const fetchUser = async (userId: number): Promise<UserResponse> => {
  const response = await apiClient.get<UserResponse>(`/api/v1/users/${userId}`);
  return response.data;
};

// ==============================================================================
// MUTATION API (Exported - these modify data)
// ==============================================================================
//
// These are exported because mutations should be callable from anywhere:
// - Event handlers
// - Thunks
// - Background tasks
// - Hooks
//
// Mutations don't cache, so there's no cache bypass risk.
//
// ==============================================================================

/**
 * User API service (Mutations Only)
 * 
 * For reads (GET), use hooks instead:
 * - ✅ useUser(id) - NOT userApi.getById(id)
 * 
 * For mutations (POST/PUT/DELETE), use this API:
 * - ✅ userApi.create(data)
 * - ✅ userApi.update(id, data)  
 * - ✅ userApi.delete(id)
 */
export const userApi = {
  /**
   * Create new user (registration)
   * 
   * @param data - User registration data
   * @returns Promise<UserResponse> - Created user
   * @throws AxiosError if email already exists (409) or validation fails (422)
   * 
   * Example:
   * ```typescript
   * const newUser = await userApi.create({
   *   email: 'user@example.com',
   *   password: 'SecurePass123!',
   *   name: 'Alice Smith',
   * });
   * ```
   * 
   * Type safety:
   * - CreateUserRequest type is auto-generated from Rust
   * - TypeScript forces you to provide required fields
   * - Can't send invalid fields (caught at compile-time)
   */
  async create(data: CreateUserRequest): Promise<UserResponse> {
    const response = await apiClient.post<UserResponse>('/api/v1/users', data);
    return response.data;
  },

  /**
   * Update user profile
   * 
   * @param userId - User's unique identifier
   * @param data - Fields to update (all optional)
   * @returns Promise<UserResponse> - Updated user
   * @throws AxiosError if user not found (404) or validation fails (422)
   * 
   * Example:
   * ```typescript
   * const updated = await userApi.update(123, {
   *   name: 'Alice Johnson', // Only update name
   * });
   * ```
   * 
   * Type safety:
   * - UpdateUserRequest has all optional fields
   * - Only send fields that changed (efficient)
   * - TypeScript prevents sending invalid fields
   */
  async update(userId: number, data: UpdateUserRequest): Promise<UserResponse> {
    const response = await apiClient.put<UserResponse>(`/api/v1/users/${userId}`, data);
    return response.data;
  },

  /**
   * Delete user account
   * 
   * @param userId - User's unique identifier
   * @throws AxiosError if user not found (404) or unauthorized (401)
   * 
   * Example:
   * ```typescript
   * await userApi.delete(123);
   * // Account deleted, user logged out
   * ```
   * 
   * Note: Backend implements soft-delete (user marked deleted, not removed)
   */
  async delete(userId: number): Promise<void> {
    await apiClient.delete(`/api/v1/users/${userId}`);
  },
};

/**
 * Authentication API service
 * 
 * Login, logout, token refresh.
 */
export const authApi = {
  /**
   * Login with email and password
   * 
   * @param email - User's email
   * @param password - User's password
   * @returns Promise<LoginResponse> - JWT tokens and user data
   * @throws AxiosError if credentials invalid (401)
   * 
   * Flow:
   * 1. Send email + password to backend
   * 2. Backend validates credentials
   * 3. Backend returns JWT access token + refresh token
   * 4. Token automatically stored in SecureStore (hardware-backed)
   * 5. Future requests use token in Authorization header
   * 
   * Example:
   * ```typescript
   * const { access_token, user } = await authApi.login(
   *   'user@example.com',
   *   'password123'
   * );
   * // Token already stored in SecureStore - no manual storage needed!
   * // Navigate to home screen
   * ```
   */
  async login(email: string, password: string) {
    const response = await apiClient.post('/api/v1/auth/login', {
      email,
      password,
    });
    
    // ✅ FIX: Automatically store token in SecureStore
    // This ensures tokens are always stored securely and consistently
    // UI doesn't need to remember to store the token
    if (response.data.access_token) {
      await TokenStorage.set(response.data.access_token);
    }
    
    return response.data;
  },

  /**
   * Logout
   * 
   * Clears tokens and invalidates session on backend.
   * Token automatically deleted from SecureStore.
   * 
   * Example:
   * ```typescript
   * await authApi.logout();
   * // Token already deleted from SecureStore
   * navigationRef.current?.navigate('Login');
   * ```
   */
  async logout() {
    await apiClient.post('/api/v1/auth/logout');
    // ✅ FIX: Platform-aware token deletion
    await TokenStorage.delete();
  },
};

// Export configured axios instance for advanced usage
export default apiClient;

// ==============================================================================
// TYPE-SAFE API HOOKS (React Query)
// ==============================================================================
//
// React Query hooks for data fetching with caching.
// 
// Benefits:
// - Automatic caching (don't re-fetch if data is fresh)
// - Loading states (no manual isLoading flags)
// - Error handling (automatic retry on failure)
// - Optimistic updates (instant UI feedback)
//
// ==============================================================================

/**
 * Hook to fetch user by ID
 * 
 * Uses React Query for caching and loading states.
 * 
 * Example:
 * ```typescript
 * function UserProfile({ userId }: { userId: number }) {
 *   const { data: user, isLoading, error } = useUser(userId);
 *   
 *   if (isLoading) return <LoadingSpinner />;
 *   if (error) return <ErrorMessage error={error} />;
 *   
 *   return (
 *     <View>
 *       <Text>{user?.name}</Text>
 *     </View>
 *   );
 * }
 * ```
 */
/**
 * Hook to fetch user by ID (USE THIS, not userApi.getById!)
 * 
 * This is the ONLY way to fetch user data from components.
 * Do not call fetchUser() or userApi.getById() directly.
 * 
 * Why hooks only?
 * - React Query cache is only accessible through hooks
 * - Calling fetchUser() directly bypasses cache
 * - Results in stale UI, duplicate requests, wasted bandwidth
 * 
 * Example:
 * ```typescript
 * function UserProfile({ userId }: { userId: number }) {
 *   const { data: user, isLoading, error } = useUser(userId);
 *   
 *   if (isLoading) return <LoadingSpinner />;
 *   if (error) return <ErrorMessage error={error} />;
 *   if (!user) return <Text>User not found</Text>;
 *   
 *   return (
 *     <View>
 *       <Text>{user.name}</Text>
 *       <Text>{user.email}</Text>
 *     </View>
 *   );
 * }
 * ```
 * 
 * Caching:
 * - Data cached for 5 minutes
 * - Background refresh if stale
 * - Instant display from cache
 * - Multiple components share same cache
 */

export function useUser(userId: number) {
  return useQuery({
    queryKey: ['user', userId],
    queryFn: () => fetchUser(userId),  // ✅ Uses internal fetcher
    staleTime: 5 * 60 * 1000, // Data fresh for 5 minutes (critical for mobile)
    retry: 3, // Retry 3 times on failure
  });
}

/**
 * Hook to update user
 * 
 * Uses React Query mutation for optimistic updates.
 * 
 * Example:
 * ```typescript
 * function EditProfile({ userId }: { userId: number }) {
 *   const updateMutation = useUpdateUser();
 *   
 *   const handleSubmit = (data: UpdateUserRequest) => {
 *     updateMutation.mutate(
 *       { userId, data },
 *       {
 *         onSuccess: () => {
 *           Alert.alert('Success', 'Profile updated!');
 *         },
 *         onError: (error) => {
 *           Alert.alert('Error', 'Failed to update profile');
 *         },
 *       }
 *     );
 *   };
 *   
 *   return (
 *     <Form onSubmit={handleSubmit} isLoading={updateMutation.isLoading} />
 *   );
 * }
 * ```
 */
export function useUpdateUser() {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: ({ userId, data }: { userId: number; data: UpdateUserRequest }) =>
      userApi.update(userId, data),
    onSuccess: (updatedUser) => {
      // Invalidate cache so next query fetches fresh data
      queryClient.invalidateQueries({ queryKey: ['user', updatedUser.id] });
    },
  });
}

/**
 * Hook to create new user
 * 
 * Uses React Query mutation for user registration.
 * 
 * Example:
 * ```typescript
 * function RegisterScreen() {
 *   const createMutation = useCreateUser();
 *   
 *   const handleRegister = (data: CreateUserRequest) => {
 *     createMutation.mutate(
 *       data,
 *       {
 *         onSuccess: (newUser) => {
 *           Alert.alert('Success', 'Account created!');
 *           navigation.navigate('Home');
 *         },
 *         onError: (error) => {
 *           if (error.response?.status === 409) {
 *             Alert.alert('Error', 'Email already exists');
 *           } else {
 *             Alert.alert('Error', 'Failed to create account');
 *           }
 *         },
 *       }
 *     );
 *   };
 *   
 *   return (
 *     <RegistrationForm onSubmit={handleRegister} isLoading={createMutation.isLoading} />
 *   );
 * }
 * ```
 */
export function useCreateUser() {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: (data: CreateUserRequest) => userApi.create(data),
    onSuccess: () => {
      // Invalidate all user queries so lists refresh
      queryClient.invalidateQueries({ queryKey: ['users'] });
    },
  });
}

/**
 * Hook to delete user
 * 
 * Uses React Query mutation for account deletion.
 * 
 * Example:
 * ```typescript
 * function AccountSettings({ userId }: { userId: number }) {
 *   const deleteMutation = useDeleteUser();
 *   
 *   const handleDelete = () => {
 *     Alert.alert(
 *       'Delete Account',
 *       'Are you sure? This cannot be undone.',
 *       [
 *         { text: 'Cancel', style: 'cancel' },
 *         {
 *           text: 'Delete',
 *           style: 'destructive',
 *           onPress: () => {
 *             deleteMutation.mutate(
 *               userId,
 *               {
 *                 onSuccess: () => {
 *                   Alert.alert('Success', 'Account deleted');
 *                   // Log out and navigate to login
 *                   authApi.logout();
 *                   navigation.navigate('Login');
 *                 },
 *                 onError: () => {
 *                   Alert.alert('Error', 'Failed to delete account');
 *                 },
 *               }
 *             );
 *           },
 *         },
 *       ]
 *     );
 *   };
 *   
 *   return (
 *     <Button
 *       title="Delete Account"
 *       onPress={handleDelete}
 *       loading={deleteMutation.isLoading}
 *       color="red"
 *     />
 *   );
 * }
 * ```
 */
export function useDeleteUser() {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: (userId: number) => userApi.delete(userId),
    onSuccess: (_, userId) => {
      // Remove this specific user from cache
      queryClient.removeQueries({ queryKey: ['user', userId] });
      // Invalidate user list
      queryClient.invalidateQueries({ queryKey: ['users'] });
    },
  });
}

// ==============================================================================
// NOTES FOR PRODUCTION
// ==============================================================================
//
// Before deploying to production:
//
// 1. Environment Variables:
//    - Use react-native-config or similar
//    - Different API_URL for dev/staging/production
//    - Never hardcode production URLs
//
// 2. Security:
//    - Use HTTPS (not HTTP) in production
//    - Enable certificate pinning (prevent MITM attacks)
//    - Rotate JWT secrets regularly
//    - Implement token refresh logic
//
// 3. Error Handling:
//    - Show user-friendly error messages (not technical)
//    - Log errors to external service (Sentry, Bugsnag)
//    - Handle offline mode gracefully
//    - Retry failed requests with exponential backoff
//
// 4. Performance:
//    - Cache responses aggressively (reduce API calls)
//    - Use pagination for large lists
//    - Compress requests (gzip)
//    - Monitor API latency
//
// 5. Testing:
//    - Mock API responses in tests
//    - Test error handling paths
//    - Test offline behavior
//    - Integration tests with real backend
//
// ==============================================================================
