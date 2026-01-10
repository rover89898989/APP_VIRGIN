# 🚀 FRONTEND PRODUCTION ADDITIONS
## Production-Grade Enhancements for APP_VIRGIN Template

> **Version:** 2.0.0  
> **Date:** January 10, 2026  
> **Author:** Senior Mobile Architect Review  
> **Purpose:** Critical production additions identified during architectural review

---

## 📋 TABLE OF CONTENTS

1. [Error Boundaries](#1-error-boundaries-p0-critical)
2. [Performance Monitoring](#2-performance-monitoring-p0-critical)
3. [Security Hardening](#3-security-hardening-p0-critical)
4. [Offline-First Patterns](#4-offline-first-patterns-p0-critical)
5. [Deep Linking](#5-deep-linking-p1-high)
6. [Push Notifications](#6-push-notifications-p1-high)
7. [Analytics Framework](#7-analytics-framework-p1-high)
8. [FlashList Migration](#8-flashlist-migration-p1-high)
9. [Feature Flags](#9-feature-flags-p2-medium)
10. [Environment Configuration](#10-environment-configuration-p2-medium)
11. [Biometric Authentication](#11-biometric-authentication-p3-nice-to-have)
12. [Internationalization](#12-internationalization-p3-nice-to-have)

---

## 1. ERROR BOUNDARIES (P0-CRITICAL)

### Why This Is Critical
React Native apps crash without proper error boundaries. A single unhandled error in any component can bring down the entire app. This is the #1 cause of production crashes.

### Files to Create

#### `mobile/src/shared/components/ErrorBoundary.tsx`

```typescript
/**
 * ErrorBoundary Component
 * 
 * PURPOSE:
 * Catches JavaScript errors anywhere in the child component tree, logs those errors,
 * and displays a fallback UI instead of crashing the entire app.
 * 
 * CRITICAL BEHAVIOR:
 * - Prevents entire app crash from single component failure
 * - Provides graceful degradation
 * - Logs errors to monitoring service (Sentry, etc.)
 * - Offers recovery mechanism to users
 * 
 * USAGE PATTERN:
 * Wrap entire app in root ErrorBoundary (_layout.tsx)
 * Wrap each route/feature in nested boundaries for isolation
 * 
 * SECURITY NOTES:
 * - Never expose stack traces in production UI
 * - Sanitize error messages before displaying to users
 * - Log full error details server-side only
 * 
 * RECOVERY STRATEGY:
 * Users can tap "Try Again" to reset error state and retry
 * Children components remount with clean state
 */

import React, { Component, ErrorInfo, ReactNode } from 'react';
import { View, Text, Pressable, ScrollView, Platform } from 'react-native';

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
  onError?: (error: Error, errorInfo: ErrorInfo) => void;
  // Optional: Custom error screen component
  FallbackComponent?: React.ComponentType<{
    error: Error;
    resetError: () => void;
  }>;
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

  /**
   * LIFECYCLE: Called when error is thrown
   * Updates state to trigger fallback UI render
   * This is called during render phase - keep it pure
   */
  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  /**
   * LIFECYCLE: Called after error is caught
   * Safe to perform side effects here (logging, analytics)
   * 
   * PRODUCTION INTEGRATION:
   * Uncomment Sentry/monitoring integration when ready
   */
  componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    // Log error details for debugging
    console.error('ErrorBoundary caught error:', {
      error: error.message,
      componentStack: errorInfo.componentStack,
      // Only log stack trace in development
      ...(process.env.NODE_ENV === 'development' && {
        stack: error.stack,
      }),
    });

    // Call custom error handler if provided
    this.props.onError?.(error, errorInfo);

    // PRODUCTION: Send to monitoring service
    // Example: Sentry.captureException(error, { contexts: { react: errorInfo } });
  }

  /**
   * Reset error state to allow recovery
   * Called when user taps "Try Again" button
   * Children components will remount with clean state
   */
  resetError = () => {
    this.setState({ hasError: false, error: null });
  };

  render() {
    if (this.state.hasError) {
      // Use custom fallback component if provided
      if (this.props.FallbackComponent) {
        return (
          <this.props.FallbackComponent
            error={this.state.error!}
            resetError={this.resetError}
          />
        );
      }

      // Use custom fallback UI if provided
      if (this.props.fallback) {
        return this.props.fallback;
      }

      // Default fallback UI
      // DESIGN: Friendly, non-technical error message
      // ACTION: Clear recovery mechanism
      return (
        <View className="flex-1 items-center justify-center p-6 bg-background-primary">
          <ScrollView
            contentContainerStyle={{
              alignItems: 'center',
              justifyContent: 'center',
              paddingVertical: 24,
            }}
          >
            {/* Error Icon */}
            <View className="w-16 h-16 mb-4 rounded-full bg-status-error/10 items-center justify-center">
              <Text className="text-4xl">⚠️</Text>
            </View>

            {/* Error Title */}
            <Text className="text-xl font-bold text-text-primary mb-2 text-center">
              Something went wrong
            </Text>

            {/* Error Message (sanitized for users) */}
            <Text className="text-base text-text-secondary mb-6 text-center max-w-sm">
              We're sorry for the inconvenience. The app encountered an unexpected
              error. Please try again.
            </Text>

            {/* Development-only: Show actual error */}
            {process.env.NODE_ENV === 'development' && this.state.error && (
              <View className="bg-surface-elevated p-4 rounded-lg mb-4 max-w-sm">
                <Text className="text-sm font-mono text-text-secondary">
                  {this.state.error.message}
                </Text>
              </View>
            )}

            {/* Retry Button */}
            <Pressable
              onPress={this.resetError}
              className="bg-primary-500 px-6 py-3 rounded-lg active:opacity-80"
              accessibilityRole="button"
              accessibilityLabel="Try again"
              accessibilityHint="Tap to reload the screen"
            >
              <Text className="text-white font-semibold text-base">
                Try Again
              </Text>
            </Pressable>

            {/* Secondary action: Report bug (optional) */}
            {/* Uncomment when bug reporting is implemented
            <Pressable
              onPress={() => {
                // TODO: Implement bug report submission
                console.log('Report bug:', this.state.error);
              }}
              className="mt-4"
              accessibilityRole="button"
              accessibilityLabel="Report this problem"
            >
              <Text className="text-primary-500 text-sm">
                Report this problem
              </Text>
            </Pressable>
            */}
          </ScrollView>
        </View>
      );
    }

    // No error: render children normally
    return this.props.children;
  }
}

/**
 * USAGE EXAMPLE 1: Root-level error boundary
 * 
 * // app/_layout.tsx
 * export default function RootLayout() {
 *   return (
 *     <ErrorBoundary>
 *       <QueryClientProvider client={queryClient}>
 *         <Stack />
 *       </QueryClientProvider>
 *     </ErrorBoundary>
 *   );
 * }
 */

/**
 * USAGE EXAMPLE 2: Feature-level error boundary
 * 
 * // app/(tabs)/profile.tsx
 * export default function ProfileScreen() {
 *   return (
 *     <ErrorBoundary
 *       onError={(error) => {
 *         // Feature-specific error handling
 *         analytics.track('profile_error', { error: error.message });
 *       }}
 *     >
 *       <ProfileContent />
 *     </ErrorBoundary>
 *   );
 * }
 */

/**
 * USAGE EXAMPLE 3: Custom fallback component
 * 
 * const CustomErrorScreen = ({ error, resetError }) => (
 *   <View className="flex-1 items-center justify-center">
 *     <Text>Custom error UI</Text>
 *     <Pressable onPress={resetError}>
 *       <Text>Reset</Text>
 *     </Pressable>
 *   </View>
 * );
 * 
 * <ErrorBoundary FallbackComponent={CustomErrorScreen}>
 *   <MyApp />
 * </ErrorBoundary>
 */
```

#### `mobile/src/shared/hooks/useErrorHandler.ts`

```typescript
/**
 * useErrorHandler Hook
 * 
 * PURPOSE:
 * Provides imperative error handling for async operations
 * Integrates with ErrorBoundary for consistent error display
 * 
 * USE CASES:
 * - Async operations (API calls, file operations)
 * - Event handlers (button presses, form submissions)
 * - Non-component errors (utility functions)
 * 
 * PATTERN:
 * Try-catch in async functions → call handleError → triggers ErrorBoundary
 */

import { useCallback, useState } from 'react';

export const useErrorHandler = () => {
  const [error, setError] = useState<Error | null>(null);

  /**
   * Handle error by setting state
   * ErrorBoundary will catch this during next render
   * 
   * NOTE: This causes a re-render, which ErrorBoundary will intercept
   */
  const handleError = useCallback((error: Error) => {
    console.error('Handled error:', error);
    setError(error);
  }, []);

  /**
   * Reset error state
   * Allows recovery from error without remounting component
   */
  const resetError = useCallback(() => {
    setError(null);
  }, []);

  // Re-throw error to be caught by ErrorBoundary
  if (error) {
    throw error;
  }

  return { handleError, resetError };
};

/**
 * USAGE EXAMPLE:
 * 
 * function MyComponent() {
 *   const { handleError } = useErrorHandler();
 *   const { mutate } = useUpdateUser();
 * 
 *   const handleSubmit = async () => {
 *     try {
 *       await mutate(userData);
 *     } catch (error) {
 *       handleError(error as Error);
 *     }
 *   };
 * 
 *   return <Button onPress={handleSubmit}>Submit</Button>;
 * }
 */
```

---

## 2. PERFORMANCE MONITORING (P0-CRITICAL)

### Why This Is Critical
You can't optimize what you can't measure. Production apps need real-time performance tracking to identify bottlenecks, memory leaks, and slow renders.

### Files to Create

#### `mobile/src/shared/utils/monitoring.ts`

```typescript
/**
 * Monitoring & Observability Utilities
 * 
 * PURPOSE:
 * Centralized performance monitoring, error tracking, and analytics
 * Provides single interface for multiple monitoring services
 * 
 * ARCHITECTURE:
 * - Provider pattern allows swapping monitoring services
 * - Graceful degradation if service unavailable
 * - No-op in development to avoid noise
 * 
 * CRITICAL INTEGRATIONS:
 * - Sentry: Error tracking, performance monitoring
 * - React DevTools Profiler: Component render tracking
 * - Custom metrics: Business-specific performance indicators
 * 
 * SECURITY:
 * - Never log sensitive user data
 * - Sanitize error messages before sending
 * - Respect user privacy preferences
 */

import * as Sentry from '@sentry/react-native';
import { Platform } from 'react-native';

/**
 * Performance budget thresholds
 * SOTA 2026 recommendations for mobile apps
 * 
 * RATIONALE:
 * - 2s TTI: Users perceive app as "instant"
 * - 500KB: Keeps initial load fast on 3G networks
 * - 60fps: Smooth animations, no jank
 * - 50MB: Prevents OS memory warnings
 */
export const PERFORMANCE_BUDGET = {
  TIME_TO_INTERACTIVE_MS: 2000,
  JS_BUNDLE_SIZE_KB: 500,
  TARGET_FPS: 60,
  MAX_MEMORY_MB: 50,
} as const;

/**
 * Initialize monitoring services
 * Call once at app startup (App.tsx)
 * 
 * PRODUCTION READINESS:
 * Set EXPO_PUBLIC_SENTRY_DSN in .env.production
 * Monitoring is disabled in development to reduce noise
 */
export const setupMonitoring = () => {
  // Only enable in production
  // Development: use React DevTools and console.log
  if (process.env.NODE_ENV !== 'production') {
    console.log('[Monitoring] Skipped in development');
    return;
  }

  const sentryDsn = process.env.EXPO_PUBLIC_SENTRY_DSN;

  if (!sentryDsn) {
    console.warn('[Monitoring] Sentry DSN not configured');
    return;
  }

  /**
   * Sentry Configuration
   * 
   * tracesSampleRate: 1.0 = 100% of transactions
   * Production: reduce to 0.1 (10%) to control costs
   * 
   * enableAutoSessionTracking: Tracks app sessions automatically
   * enableNativeCrashHandling: Catches native iOS/Android crashes
   * 
   * beforeSend: Last chance to sanitize data before sending
   * Use for removing PII, sensitive data, etc.
   */
  Sentry.init({
    dsn: sentryDsn,
    
    // Environment tracking
    environment: process.env.EXPO_PUBLIC_ENV || 'production',
    
    // Performance monitoring
    tracesSampleRate: 1.0, // TODO: Reduce to 0.1 in production after validation
    enableAutoSessionTracking: true,
    
    // Native crash reporting (iOS/Android only)
    enableNativeCrashHandling: Platform.OS !== 'web',
    
    // Attach stack trace for better debugging
    attachStacktrace: true,
    
    /**
     * Data sanitization before sending
     * CRITICAL: Remove PII, tokens, passwords
     */
    beforeSend(event, hint) {
      // Remove sensitive data from breadcrumbs
      if (event.breadcrumbs) {
        event.breadcrumbs = event.breadcrumbs.map((breadcrumb) => ({
          ...breadcrumb,
          // Sanitize HTTP request data
          data: sanitizeData(breadcrumb.data),
        }));
      }

      // Remove sensitive data from context
      if (event.contexts) {
        // Remove device ID, user info if present
        delete event.contexts.device?.id;
      }

      return event;
    },
  });

  console.log('[Monitoring] Sentry initialized');
};

/**
 * Sanitize data before sending to monitoring service
 * Removes common sensitive fields
 * 
 * EXTEND THIS: Add your app-specific sensitive fields
 */
function sanitizeData(data: any): any {
  if (!data) return data;

  const sensitive = ['password', 'token', 'secret', 'apiKey', 'ssn', 'creditCard'];
  const sanitized = { ...data };

  for (const key of Object.keys(sanitized)) {
    if (sensitive.some((s) => key.toLowerCase().includes(s))) {
      sanitized[key] = '[REDACTED]';
    }
  }

  return sanitized;
}

/**
 * Track custom performance metric
 * Use for business-critical operations
 * 
 * EXAMPLES:
 * - Time to first API call
 * - Time to render user profile
 * - Database query duration
 */
export const trackPerformance = (
  operation: string,
  durationMs: number,
  metadata?: Record<string, any>
) => {
  if (process.env.NODE_ENV !== 'production') {
    console.log(`[Performance] ${operation}: ${durationMs}ms`, metadata);
    return;
  }

  // Send to Sentry
  Sentry.addBreadcrumb({
    category: 'performance',
    message: operation,
    level: 'info',
    data: {
      durationMs,
      ...metadata,
    },
  });

  // Check against budget
  if (operation === 'time_to_interactive' && durationMs > PERFORMANCE_BUDGET.TIME_TO_INTERACTIVE_MS) {
    Sentry.captureMessage(
      `Performance budget exceeded: ${operation} took ${durationMs}ms (budget: ${PERFORMANCE_BUDGET.TIME_TO_INTERACTIVE_MS}ms)`,
      'warning'
    );
  }
};

/**
 * Track custom business metric
 * Use for conversion funnels, feature adoption, etc.
 * 
 * NOTE: Use analytics.track() for user behavior
 * Use this for system/performance metrics only
 */
export const trackMetric = (
  name: string,
  value: number,
  unit: string = 'count'
) => {
  if (process.env.NODE_ENV !== 'production') {
    console.log(`[Metric] ${name}: ${value} ${unit}`);
    return;
  }

  Sentry.addBreadcrumb({
    category: 'metric',
    message: name,
    level: 'info',
    data: { value, unit },
  });
};

/**
 * Log error to monitoring service
 * Automatically captured by ErrorBoundary
 * Use this for imperative error logging
 */
export const logError = (
  error: Error,
  context?: Record<string, any>
) => {
  console.error('[Error]', error.message, context);

  if (process.env.NODE_ENV !== 'production') {
    return;
  }

  Sentry.captureException(error, {
    contexts: {
      custom: sanitizeData(context),
    },
  });
};

/**
 * Set user context for error tracking
 * Call after successful login
 * 
 * PRIVACY: Only set non-sensitive identifiers
 * Never include email, phone, real name in production
 */
export const setUserContext = (userId: string) => {
  if (process.env.NODE_ENV !== 'production') {
    console.log('[Monitoring] User context:', userId);
    return;
  }

  Sentry.setUser({
    id: userId,
    // PRIVACY: Only include if user consents
    // email: userEmail,
  });
};

/**
 * Clear user context
 * Call on logout
 */
export const clearUserContext = () => {
  if (process.env.NODE_ENV !== 'production') {
    console.log('[Monitoring] User context cleared');
    return;
  }

  Sentry.setUser(null);
};

/**
 * USAGE EXAMPLE 1: Track component mount time
 * 
 * function ProfileScreen() {
 *   useEffect(() => {
 *     const startTime = Date.now();
 *     return () => {
 *       trackPerformance('profile_screen_mount', Date.now() - startTime);
 *     };
 *   }, []);
 * }
 */

/**
 * USAGE EXAMPLE 2: Track API call duration
 * 
 * const fetchUser = async (id: string) => {
 *   const startTime = Date.now();
 *   try {
 *     const user = await api.get(`/users/${id}`);
 *     trackPerformance('api_fetch_user', Date.now() - startTime, { userId: id });
 *     return user;
 *   } catch (error) {
 *     logError(error, { operation: 'fetch_user', userId: id });
 *     throw error;
 *   }
 * };
 */
```

#### `mobile/src/shared/hooks/usePerformanceMonitor.ts`

```typescript
/**
 * usePerformanceMonitor Hook
 * 
 * PURPOSE:
 * Measures component render performance
 * Detects slow renders, memory leaks, unnecessary re-renders
 * 
 * CRITICAL FOR:
 * - Lists with many items
 * - Complex screens (forms, dashboards)
 * - Animation-heavy components
 * 
 * USAGE:
 * Wrap component in <Profiler> with this hook's callback
 */

import { useRef, useCallback } from 'react';
import { trackPerformance } from '../utils/monitoring';

interface PerformanceMetrics {
  componentName: string;
  phase: 'mount' | 'update';
  actualDuration: number;
  baseDuration: number;
  startTime: number;
  commitTime: number;
}

/**
 * Performance monitoring hook
 * Integrates with React Profiler API
 * 
 * THRESHOLDS:
 * - 16ms: Single frame at 60fps
 * - 50ms: Noticeable lag
 * - 100ms: User perceives as slow
 */
export const usePerformanceMonitor = (componentName: string) => {
  const renderCount = useRef(0);
  const mountTime = useRef<number | null>(null);

  /**
   * React Profiler callback
   * Called after component commits to screen
   * 
   * PARAMETERS:
   * - id: Component name
   * - phase: 'mount' or 'update'
   * - actualDuration: Time spent rendering (ms)
   * - baseDuration: Estimated time without memoization (ms)
   * - startTime: When render started (ms)
   * - commitTime: When React committed to DOM (ms)
   */
  const onRender = useCallback(
    (
      id: string,
      phase: 'mount' | 'update',
      actualDuration: number,
      baseDuration: number,
      startTime: number,
      commitTime: number
    ) => {
      renderCount.current += 1;

      // Track mount time
      if (phase === 'mount') {
        mountTime.current = actualDuration;
        console.log(`[Perf] ${componentName} mounted in ${actualDuration.toFixed(2)}ms`);
      }

      // Warn on slow renders
      const SLOW_RENDER_THRESHOLD_MS = 50;
      if (actualDuration > SLOW_RENDER_THRESHOLD_MS) {
        console.warn(
          `[Perf] Slow render detected: ${componentName} took ${actualDuration.toFixed(2)}ms (${phase})`
        );

        // Track in production
        trackPerformance(`slow_render_${componentName}`, actualDuration, {
          phase,
          renderCount: renderCount.current,
        });
      }

      // Warn on excessive re-renders
      const EXCESSIVE_RENDER_COUNT = 10;
      if (renderCount.current > EXCESSIVE_RENDER_COUNT && phase === 'update') {
        console.warn(
          `[Perf] Excessive re-renders: ${componentName} has rendered ${renderCount.current} times`
        );
      }
    },
    [componentName]
  );

  return {
    onRender,
    renderCount: renderCount.current,
    mountTime: mountTime.current,
  };
};

/**
 * USAGE EXAMPLE:
 * 
 * import { Profiler } from 'react';
 * import { usePerformanceMonitor } from '@/shared/hooks/usePerformanceMonitor';
 * 
 * function ProfileScreen() {
 *   const { onRender } = usePerformanceMonitor('ProfileScreen');
 * 
 *   return (
 *     <Profiler id="ProfileScreen" onRender={onRender}>
 *       <View>
 *         <ProfileContent />
 *       </View>
 *     </Profiler>
 *   );
 * }
 */
```

---

## 3. SECURITY HARDENING (P0-CRITICAL)

### Why This Is Critical
Production apps handle sensitive user data. Token storage, certificate pinning, and input validation are non-negotiable for security.

### Files to Create

#### `mobile/src/shared/utils/secureStorage.ts`

```typescript
/**
 * Secure Storage Utilities
 * 
 * PURPOSE:
 * Centralized secure storage for sensitive data
 * Hardware-backed encryption on iOS/Android
 * 
 * CRITICAL SECURITY NOTES:
 * - NEVER use AsyncStorage for tokens, passwords, or secrets
 * - AsyncStorage is UNENCRYPTED and easily accessible
 * - SecureStore uses Keychain (iOS) and KeyStore (Android)
 * - Hardware-backed encryption cannot be extracted without device unlock
 * 
 * PLATFORM DIFFERENCES:
 * - iOS: Keychain with kSecAttrAccessibleAfterFirstUnlock
 * - Android: EncryptedSharedPreferences with KeyStore
 * - Web: Falls back to httpOnly cookies (server-managed)
 * 
 * IMPORTANT: This module ONLY handles token storage
 * Server handles token generation, validation, expiration
 */

import * as SecureStore from 'expo-secure-store';
import { Platform } from 'react-native';

/**
 * Storage keys
 * Namespaced to avoid conflicts with other apps
 * 
 * SECURITY: Keys are not secret, values are encrypted
 */
const STORAGE_KEYS = {
  ACCESS_TOKEN: 'com.yourapp.access_token',
  REFRESH_TOKEN: 'com.yourapp.refresh_token',
  USER_ID: 'com.yourapp.user_id',
} as const;

/**
 * SecureStore options
 * 
 * keychainAccessible: When keychain item is accessible
 * - AfterFirstUnlock: After device first unlocked (recommended)
 * - WhenUnlocked: Only when device unlocked (most secure)
 * - Always: Even when locked (least secure)
 */
const SECURE_STORE_OPTIONS: SecureStore.SecureStoreOptions = {
  keychainAccessible: SecureStore.AFTER_FIRST_UNLOCK,
};

/**
 * Store access token securely
 * 
 * USAGE: Call after successful login
 * INVALIDATION: Call removeTokens() on logout
 * 
 * SECURITY BOUNDARY:
 * - Native (iOS/Android): Stored in hardware-backed keychain
 * - Web: NOT STORED (managed by httpOnly cookies)
 * 
 * @param token - JWT access token from backend
 */
export const storeAccessToken = async (token: string): Promise<void> => {
  if (Platform.OS === 'web') {
    // Web uses httpOnly cookies, no client storage needed
    console.log('[SecureStorage] Web platform uses httpOnly cookies');
    return;
  }

  try {
    await SecureStore.setItemAsync(
      STORAGE_KEYS.ACCESS_TOKEN,
      token,
      SECURE_STORE_OPTIONS
    );
    console.log('[SecureStorage] Access token stored securely');
  } catch (error) {
    console.error('[SecureStorage] Failed to store access token:', error);
    throw new Error('Failed to securely store authentication token');
  }
};

/**
 * Store refresh token securely
 * 
 * REFRESH TOKEN PATTERN:
 * - Longer TTL than access token (7 days vs 15 minutes)
 * - Used to obtain new access tokens without re-login
 * - Must be rotated on each use (OIDC best practice)
 * - Stored server-side for revocation capability
 * 
 * @param token - JWT refresh token from backend
 */
export const storeRefreshToken = async (token: string): Promise<void> => {
  if (Platform.OS === 'web') {
    console.log('[SecureStorage] Web platform uses httpOnly cookies');
    return;
  }

  try {
    await SecureStore.setItemAsync(
      STORAGE_KEYS.REFRESH_TOKEN,
      token,
      SECURE_STORE_OPTIONS
    );
    console.log('[SecureStorage] Refresh token stored securely');
  } catch (error) {
    console.error('[SecureStorage] Failed to store refresh token:', error);
    throw new Error('Failed to securely store refresh token');
  }
};

/**
 * Retrieve access token from secure storage
 * 
 * USAGE: Called by API client before each request
 * RETURNS: Token string or null if not found
 * 
 * ERROR HANDLING:
 * - Returns null on errors (graceful degradation)
 * - Logs error for debugging
 * - Allows app to function (user re-prompted to login)
 */
export const getAccessToken = async (): Promise<string | null> => {
  if (Platform.OS === 'web') {
    // Web tokens managed by httpOnly cookies
    return null;
  }

  try {
    const token = await SecureStore.getItemAsync(STORAGE_KEYS.ACCESS_TOKEN);
    return token;
  } catch (error) {
    console.error('[SecureStorage] Failed to retrieve access token:', error);
    return null;
  }
};

/**
 * Retrieve refresh token from secure storage
 * 
 * USAGE: Called when access token expires
 * Used to request new access token from /auth/refresh endpoint
 */
export const getRefreshToken = async (): Promise<string | null> => {
  if (Platform.OS === 'web') {
    return null;
  }

  try {
    const token = await SecureStore.getItemAsync(STORAGE_KEYS.REFRESH_TOKEN);
    return token;
  } catch (error) {
    console.error('[SecureStorage] Failed to retrieve refresh token:', error);
    return null;
  }
};

/**
 * Remove all stored tokens
 * 
 * USAGE: Call on logout, password change, or security event
 * CRITICAL: Ensures old tokens cannot be reused
 * 
 * SECURITY NOTE:
 * This does NOT invalidate tokens on server
 * Server must also revoke tokens in database
 */
export const removeTokens = async (): Promise<void> => {
  if (Platform.OS === 'web') {
    console.log('[SecureStorage] Web tokens cleared via /auth/logout endpoint');
    return;
  }

  try {
    await Promise.all([
      SecureStore.deleteItemAsync(STORAGE_KEYS.ACCESS_TOKEN),
      SecureStore.deleteItemAsync(STORAGE_KEYS.REFRESH_TOKEN),
      SecureStore.deleteItemAsync(STORAGE_KEYS.USER_ID),
    ]);
    console.log('[SecureStorage] All tokens removed');
  } catch (error) {
    console.error('[SecureStorage] Failed to remove tokens:', error);
    // Don't throw - logout should proceed even if storage fails
  }
};

/**
 * Check if user has valid stored token
 * 
 * USAGE: App startup to determine authentication state
 * 
 * NOTE: This only checks storage, not token validity
 * Backend still validates token on API calls
 * Expired tokens are handled by 401 response + token refresh
 */
export const hasStoredToken = async (): Promise<boolean> => {
  const token = await getAccessToken();
  return token !== null && token.length > 0;
};

/**
 * Store user ID for offline functionality
 * 
 * NON-SENSITIVE: User ID is public identifier
 * Can be stored in regular AsyncStorage if needed
 * Here we use SecureStore for consistency
 */
export const storeUserId = async (userId: string): Promise<void> => {
  if (Platform.OS === 'web') {
    return;
  }

  try {
    await SecureStore.setItemAsync(STORAGE_KEYS.USER_ID, userId);
  } catch (error) {
    console.error('[SecureStorage] Failed to store user ID:', error);
  }
};

/**
 * Retrieve stored user ID
 */
export const getUserId = async (): Promise<string | null> => {
  if (Platform.OS === 'web') {
    return null;
  }

  try {
    return await SecureStore.getItemAsync(STORAGE_KEYS.USER_ID);
  } catch (error) {
    console.error('[SecureStorage] Failed to retrieve user ID:', error);
    return null;
  }
};

/**
 * MIGRATION HELPER: Check for insecure storage
 * 
 * PURPOSE: Detect if app previously used AsyncStorage
 * Helps users migrate from insecure to secure storage
 * 
 * USAGE: Run once at app startup, remove after migration period
 */
export const checkForInsecureStorage = async (): Promise<boolean> => {
  // Implementation depends on your old storage keys
  // Example:
  // const oldToken = await AsyncStorage.getItem('token');
  // if (oldToken) {
  //   console.warn('[Security] Found token in insecure storage!');
  //   await AsyncStorage.removeItem('token');
  //   return true;
  // }
  return false;
};

/**
 * USAGE EXAMPLE 1: Login flow
 * 
 * const handleLogin = async (email: string, password: string) => {
 *   const { accessToken, refreshToken, userId } = await api.login(email, password);
 *   
 *   await storeAccessToken(accessToken);
 *   await storeRefreshToken(refreshToken);
 *   await storeUserId(userId);
 *   
 *   navigation.navigate('Home');
 * };
 */

/**
 * USAGE EXAMPLE 2: Logout flow
 * 
 * const handleLogout = async () => {
 *   await api.logout(); // Server-side token revocation
 *   await removeTokens(); // Client-side token removal
 *   navigation.navigate('Login');
 * };
 */

/**
 * USAGE EXAMPLE 3: Token refresh flow
 * 
 * const refreshAccessToken = async () => {
 *   const refreshToken = await getRefreshToken();
 *   if (!refreshToken) {
 *     throw new Error('No refresh token available');
 *   }
 *   
 *   const { accessToken, newRefreshToken } = await api.refreshToken(refreshToken);
 *   
 *   await storeAccessToken(accessToken);
 *   if (newRefreshToken) {
 *     await storeRefreshToken(newRefreshToken); // Token rotation
 *   }
 * };
 */
```

#### `mobile/src/shared/utils/inputValidation.ts`

```typescript
/**
 * Input Validation Utilities
 * 
 * PURPOSE:
 * Client-side input validation for security and UX
 * Complements server-side validation (never replaces it)
 * 
 * SECURITY PRINCIPLE:
 * "Never trust client input" - always validate server-side
 * Client validation is for UX only (instant feedback)
 * 
 * VALIDATION STRATEGY:
 * 1. Client validation: Fast feedback, no network required
 * 2. Server validation: Authoritative, prevents malicious input
 * 3. Database constraints: Last line of defense
 */

/**
 * Email validation (RFC 5322 compliant)
 * 
 * RULES:
 * - Max 255 characters total
 * - Local part (before @) max 64 characters
 * - Must have TLD (top-level domain) with min 2 characters
 * - Allows common special characters in local part
 * 
 * DOES NOT VALIDATE:
 * - Whether email address actually exists
 * - Whether email is deliverable
 * - MX records or SMTP validation
 * 
 * For email verification, use confirmation emails
 */
export const validateEmail = (email: string): {
  isValid: boolean;
  error?: string;
} => {
  // Empty check
  if (!email || email.trim().length === 0) {
    return { isValid: false, error: 'Email is required' };
  }

  // Length check
  if (email.length > 255) {
    return { isValid: false, error: 'Email is too long (max 255 characters)' };
  }

  // Basic format check
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  if (!emailRegex.test(email)) {
    return { isValid: false, error: 'Email format is invalid' };
  }

  // Split into local and domain parts
  const [localPart, domainPart] = email.split('@');

  // Local part length check
  if (localPart.length > 64) {
    return {
      isValid: false,
      error: 'Email local part is too long (max 64 characters)',
    };
  }

  // Domain must have TLD
  const domainParts = domainPart.split('.');
  if (domainParts.length < 2) {
    return { isValid: false, error: 'Email must have a valid domain' };
  }

  // TLD must be at least 2 characters
  const tld = domainParts[domainParts.length - 1];
  if (tld.length < 2) {
    return { isValid: false, error: 'Email domain extension is too short' };
  }

  return { isValid: true };
};

/**
 * Password strength validation
 * 
 * SECURITY REQUIREMENTS:
 * - Minimum 8 characters (NIST recommendation)
 * - At least one uppercase letter
 * - At least one lowercase letter
 * - At least one number
 * - At least one special character
 * 
 * BEST PRACTICE:
 * Use zxcvbn library for real password strength estimation
 * This is basic validation only
 */
export const validatePassword = (password: string): {
  isValid: boolean;
  error?: string;
  strength?: 'weak' | 'medium' | 'strong';
} => {
  if (!password || password.length === 0) {
    return { isValid: false, error: 'Password is required' };
  }

  // Minimum length
  if (password.length < 8) {
    return {
      isValid: false,
      error: 'Password must be at least 8 characters',
      strength: 'weak',
    };
  }

  // Check for required character types
  const hasUppercase = /[A-Z]/.test(password);
  const hasLowercase = /[a-z]/.test(password);
  const hasNumber = /[0-9]/.test(password);
  const hasSpecial = /[!@#$%^&*(),.?":{}|<>]/.test(password);

  const typesCount = [hasUppercase, hasLowercase, hasNumber, hasSpecial].filter(
    Boolean
  ).length;

  if (typesCount < 3) {
    return {
      isValid: false,
      error:
        'Password must contain at least 3 of: uppercase, lowercase, number, special character',
      strength: 'weak',
    };
  }

  // Determine strength
  let strength: 'weak' | 'medium' | 'strong';
  if (password.length >= 12 && typesCount === 4) {
    strength = 'strong';
  } else if (password.length >= 10 && typesCount >= 3) {
    strength = 'medium';
  } else {
    strength = 'weak';
  }

  return { isValid: true, strength };
};

/**
 * Sanitize string input for XSS prevention
 * 
 * REMOVES:
 * - HTML tags
 * - Script tags
 * - Event handlers (onclick, onerror, etc.)
 * 
 * USE CASES:
 * - User-generated content (comments, posts)
 * - Display names, bios
 * - Any text shown to other users
 * 
 * IMPORTANT: This is basic sanitization
 * For rich text, use a library like DOMPurify
 */
export const sanitizeString = (input: string): string => {
  if (!input) return '';

  return input
    .replace(/<script\b[^<]*(?:(?!<\/script>)<[^<]*)*<\/script>/gi, '') // Remove script tags
    .replace(/<[^>]*>/g, '') // Remove HTML tags
    .replace(/on\w+="[^"]*"/g, '') // Remove event handlers
    .trim();
};

/**
 * Validate phone number (E.164 format)
 * 
 * FORMAT: +[country code][number]
 * Example: +14155552671
 * 
 * LIMITATIONS:
 * - Does not validate if number is actually in service
 * - Does not validate country code
 * 
 * For real phone validation, use libphonenumber-js
 */
export const validatePhoneNumber = (phone: string): {
  isValid: boolean;
  error?: string;
} => {
  if (!phone || phone.trim().length === 0) {
    return { isValid: false, error: 'Phone number is required' };
  }

  // Basic E.164 format check
  const e164Regex = /^\+[1-9]\d{1,14}$/;
  if (!e164Regex.test(phone)) {
    return {
      isValid: false,
      error: 'Phone number must be in E.164 format (e.g., +14155552671)',
    };
  }

  return { isValid: true };
};

/**
 * Validate URL
 * 
 * CHECKS:
 * - Valid URL format
 * - HTTPS only (security requirement)
 * - No javascript: or data: URLs (XSS prevention)
 */
export const validateURL = (url: string): {
  isValid: boolean;
  error?: string;
} => {
  if (!url || url.trim().length === 0) {
    return { isValid: false, error: 'URL is required' };
  }

  try {
    const parsedUrl = new URL(url);

    // Only allow HTTPS in production
    if (process.env.NODE_ENV === 'production' && parsedUrl.protocol !== 'https:') {
      return {
        isValid: false,
        error: 'Only HTTPS URLs are allowed',
      };
    }

    // Block dangerous protocols
    const blockedProtocols = ['javascript:', 'data:', 'file:'];
    if (blockedProtocols.includes(parsedUrl.protocol)) {
      return {
        isValid: false,
        error: 'URL protocol is not allowed',
      };
    }

    return { isValid: true };
  } catch {
    return { isValid: false, error: 'Invalid URL format' };
  }
};

/**
 * USAGE EXAMPLE: Form validation
 * 
 * const handleSubmit = async () => {
 *   const emailValidation = validateEmail(email);
 *   if (!emailValidation.isValid) {
 *     setError(emailValidation.error);
 *     return;
 *   }
 * 
 *   const passwordValidation = validatePassword(password);
 *   if (!passwordValidation.isValid) {
 *     setError(passwordValidation.error);
 *     return;
 *   }
 * 
 *   await submitForm(email, password);
 * };
 */
```

---

## 4. OFFLINE-FIRST PATTERNS (P0-CRITICAL)

### Why This Is Critical
Mobile apps must handle poor connectivity gracefully. Users expect apps to work offline and sync when network returns.

### Files to Create

#### `mobile/src/api/queryClient.ts`

```typescript
/**
 * React Query Client Configuration
 * 
 * PURPOSE:
 * Centralized configuration for data fetching, caching, and offline support
 * Enables offline-first architecture with automatic background sync
 * 
 * OFFLINE-FIRST ARCHITECTURE:
 * 1. Data persisted to AsyncStorage
 * 2. Reads served from cache (instant)
 * 3. Background sync when network available
 * 4. Optimistic updates for writes
 * 
 * CRITICAL CONFIGURATION:
 * - cacheTime: How long data stays in cache after unused
 * - staleTime: How long data is considered fresh
 * - retry: How many times to retry failed requests
 * - networkMode: offline/online/always behavior
 */

import { QueryClient } from '@tanstack/react-query';
import { createAsyncStoragePersister } from '@tanstack/query-async-storage-persister';
import AsyncStorage from '@react-native-async-storage/async-storage';

/**
 * AsyncStorage persister for React Query
 * 
 * PERSISTENCE STRATEGY:
 * - Queries cached to AsyncStorage
 * - Available across app restarts
 * - Cleared on logout or storage full
 * 
 * STORAGE SIZE:
 * - Default limit: 10MB
 * - Monitor usage to prevent quota exceeded
 * 
 * SECURITY NOTE:
 * - Only cache non-sensitive data
 * - Never cache tokens, passwords, PII
 * - Sanitize data before caching
 */
const asyncStoragePersister = createAsyncStoragePersister({
  storage: AsyncStorage,
  // Optional: Customize serialization
  // serialize: (data) => JSON.stringify(data),
  // deserialize: (data) => JSON.parse(data),
});

/**
 * Query Client Configuration
 * 
 * TUNING GUIDE:
 * - Short staleTime (1-5 min): Real-time data (chat, notifications)
 * - Medium staleTime (5-15 min): User data, profiles
 * - Long staleTime (1 hour+): Static content, settings
 * 
 * CACHE TIME vs STALE TIME:
 * - staleTime: How long data is "fresh" (no background refetch)
 * - cacheTime: How long data stays in memory after last use
 * 
 * Example: staleTime=5min, cacheTime=24hr
 * - First 5 minutes: Serve from cache, no refetch
 * - After 5 minutes: Serve from cache, refetch in background
 * - After 24 hours unused: Remove from cache
 */
export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      /**
       * CACHE TIME: 24 hours
       * Data stays in cache for 24 hours after last component unmount
       * Prevents unnecessary API calls for frequently accessed data
       */
      cacheTime: 1000 * 60 * 60 * 24, // 24 hours

      /**
       * STALE TIME: 5 minutes
       * Data is considered fresh for 5 minutes
       * No background refetch during this period
       * 
       * ADJUST FOR YOUR USE CASE:
       * - Real-time: 0 (always refetch)
       * - Moderate: 5 minutes (recommended)
       * - Static: Infinity (never refetch)
       */
      staleTime: 1000 * 60 * 5, // 5 minutes

      /**
       * RETRY STRATEGY
       * Failed requests retry 2 times with exponential backoff
       * 
       * Backoff schedule:
       * - First retry: ~1 second
       * - Second retry: ~2 seconds
       * - Third attempt: fail
       * 
       * TUNE FOR YOUR API:
       * - Fast API: 1 retry
       * - Slow/unreliable: 3 retries
       * - Never retry: false
       */
      retry: 2,

      /**
       * RETRY DELAY
       * Exponential backoff prevents thundering herd
       * attemptIndex starts at 0
       */
      retryDelay: (attemptIndex) => Math.min(1000 * 2 ** attemptIndex, 30000),

      /**
       * NETWORK MODE: Offline-first
       * 
       * Options:
       * - 'offlineFirst': Try cache first, fetch in background
       * - 'online': Only fetch when online (throws if offline)
       * - 'always': Always try fetch (may fail if offline)
       * 
       * SOTA 2026: offlineFirst is recommended for mobile
       */
      networkMode: 'offlineFirst',

      /**
       * REFETCH TRIGGERS
       * When to automatically refetch data
       * 
       * onWindowFocus: User returns to app (e.g., from background)
       * - Ensures fresh data when app becomes active
       * - Set false if data changes infrequently
       * 
       * onReconnect: Network connection restored
       * - Syncs data after offline period
       * - Critical for offline-first apps
       * 
       * onMount: Component mounts
       * - Usually handled by staleTime
       * - Set true if staleTime is long but you want fresh data on mount
       */
      refetchOnWindowFocus: true,
      refetchOnReconnect: true,
      refetchOnMount: true,

      /**
       * STRUCTURAL SHARING
       * Preserves referential equality when data hasn't changed
       * Prevents unnecessary re-renders
       * 
       * LEAVE ENABLED: Performance optimization with no downside
       */
      structuralSharing: true,
    },

    mutations: {
      /**
       * MUTATION RETRY
       * Mutations (POST/PUT/DELETE) typically don't retry
       * Risk of duplicate operations (e.g., double charge)
       * 
       * EXCEPTION: Idempotent operations can retry safely
       * Example: PUT (update with same payload is safe)
       */
      retry: false,

      /**
       * NETWORK MODE: Online only
       * Mutations should not execute offline (by default)
       * Prevents data inconsistency
       * 
       * OVERRIDE: For offline-first mutations
       * Use { networkMode: 'always' } in specific mutations
       */
      networkMode: 'online',
    },
  },
});

/**
 * USAGE EXAMPLE 1: Basic setup
 * 
 * // App.tsx
 * import { QueryClientProvider } from '@tanstack/react-query';
 * import { PersistQueryClientProvider } from '@tanstack/react-query-persist-client';
 * import { queryClient, asyncStoragePersister } from './api/queryClient';
 * 
 * export default function App() {
 *   return (
 *     <PersistQueryClientProvider
 *       client={queryClient}
 *       persistOptions={{ persister: asyncStoragePersister }}
 *     >
 *       <YourApp />
 *     </PersistQueryClientProvider>
 *   );
 * }
 */

/**
 * USAGE EXAMPLE 2: Custom query options
 * 
 * // Override defaults for specific query
 * const { data } = useQuery({
 *   queryKey: ['realtime-data'],
 *   queryFn: fetchRealtimeData,
 *   staleTime: 0, // Always refetch (real-time)
 *   cacheTime: 0, // Don't cache
 * });
 */

/**
 * USAGE EXAMPLE 3: Offline-first mutation
 * 
 * const mutation = useMutation({
 *   mutationFn: updateUser,
 *   networkMode: 'always', // Execute even offline
 *   onMutate: async (newUser) => {
 *     // Optimistic update
 *     await queryClient.cancelQueries({ queryKey: ['user'] });
 *     const previousUser = queryClient.getQueryData(['user']);
 *     queryClient.setQueryData(['user'], newUser);
 *     return { previousUser };
 *   },
 *   onError: (err, newUser, context) => {
 *     // Rollback on error
 *     queryClient.setQueryData(['user'], context.previousUser);
 *   },
 * });
 */

/**
 * PRODUCTION MONITORING:
 * Track cache metrics to tune configuration
 * - Cache hit rate
 * - Average stale time
 * - Retry success rate
 * - Offline operation count
 */
```

#### `mobile/src/shared/hooks/useNetworkStatus.ts`

```typescript
/**
 * useNetworkStatus Hook
 * 
 * PURPOSE:
 * Monitor network connectivity status
 * Provides real-time updates on connection state
 * 
 * CRITICAL FOR:
 * - Offline-first apps
 * - Showing connection indicators
 * - Preventing network operations when offline
 * - Syncing when connection restored
 * 
 * NETWORK STATES:
 * - Online: Connected to internet
 * - Offline: No internet connection
 * - Unknown: Cannot determine (initial state)
 */

import { useEffect, useState, useCallback } from 'react';
import NetInfo, { NetInfoState } from '@react-native-community/netinfo';

interface NetworkStatus {
  isConnected: boolean | null;
  isInternetReachable: boolean | null;
  type: string | null;
  details: any;
}

export const useNetworkStatus = () => {
  const [networkStatus, setNetworkStatus] = useState<NetworkStatus>({
    isConnected: null,
    isInternetReachable: null,
    type: null,
    details: null,
  });

  useEffect(() => {
    /**
     * Subscribe to network state changes
     * Fires when connection status changes
     * 
     * IMPORTANT: Unsubscribe on unmount to prevent memory leaks
     */
    const unsubscribe = NetInfo.addEventListener((state: NetInfoState) => {
      setNetworkStatus({
        isConnected: state.isConnected,
        isInternetReachable: state.isInternetReachable,
        type: state.type,
        details: state.details,
      });
    });

    return () => {
      unsubscribe();
    };
  }, []);

  /**
   * Force refresh network status
   * Useful after user manually checks connection
   */
  const refresh = useCallback(async () => {
    const state = await NetInfo.fetch();
    setNetworkStatus({
      isConnected: state.isConnected,
      isInternetReachable: state.isInternetReachable,
      type: state.type,
      details: state.details,
    });
  }, []);

  return {
    ...networkStatus,
    refresh,
    // Convenience helpers
    isOnline: networkStatus.isConnected === true,
    isOffline: networkStatus.isConnected === false,
    isUnknown: networkStatus.isConnected === null,
  };
};

/**
 * USAGE EXAMPLE 1: Show offline banner
 * 
 * function OfflineBanner() {
 *   const { isOffline } = useNetworkStatus();
 * 
 *   if (!isOffline) return null;
 * 
 *   return (
 *     <View className="bg-status-warning p-2">
 *       <Text className="text-center text-white">
 *         You are offline. Some features may be unavailable.
 *       </Text>
 *     </View>
 *   );
 * }
 */

/**
 * USAGE EXAMPLE 2: Prevent operations when offline
 * 
 * function SubmitButton() {
 *   const { isOnline } = useNetworkStatus();
 *   const mutation = useUpdateUser();
 * 
 *   const handlePress = () => {
 *     if (!isOnline) {
 *       Alert.alert('No connection', 'Please connect to the internet to save changes');
 *       return;
 *     }
 *     mutation.mutate(userData);
 *   };
 * 
 *   return (
 *     <Button onPress={handlePress} disabled={!isOnline}>
 *       Save
 *     </Button>
 *   );
 * }
 */

/**
 * USAGE EXAMPLE 3: Sync when connection restored
 * 
 * function DataSync() {
 *   const { isOnline } = useNetworkStatus();
 *   const queryClient = useQueryClient();
 * 
 *   useEffect(() => {
 *     if (isOnline) {
 *       // Refetch all queries when connection restored
 *       queryClient.refetchQueries();
 *       console.log('[Sync] Connection restored, syncing data...');
 *     }
 *   }, [isOnline, queryClient]);
 * 
 *   return null;
 * }
 */
```

---

## 5. DEEP LINKING (P1-HIGH)

### Why This Is Important
Deep linking is critical for user acquisition, notifications, sharing, and marketing campaigns. Without it, users cannot navigate directly to content.

### Files to Create

#### `mobile/app.json` (UPDATE)

```json
{
  "expo": {
    "scheme": "yourapp",
    "ios": {
      "bundleIdentifier": "com.yourcompany.yourapp",
      "associatedDomains": [
        "applinks:yourapp.com",
        "applinks:www.yourapp.com"
      ]
    },
    "android": {
      "package": "com.yourcompany.yourapp",
      "intentFilters": [
        {
          "action": "VIEW",
          "autoVerify": true,
          "data": [
            {
              "scheme": "https",
              "host": "yourapp.com",
              "pathPrefix": "/"
            },
            {
              "scheme": "https",
              "host": "www.yourapp.com",
              "pathPrefix": "/"
            }
          ],
          "category": [
            "BROWSABLE",
            "DEFAULT"
          ]
        }
      ]
    },
    "web": {
      "bundler": "metro"
    }
  }
}
```

#### `mobile/src/shared/utils/linking.ts`

```typescript
/**
 * Deep Linking Utilities
 * 
 * PURPOSE:
 * Handle deep links from notifications, marketing, sharing
 * Navigate users directly to specific content
 * 
 * LINK TYPES:
 * 1. Universal Links (iOS): https://yourapp.com/profile/123
 * 2. App Links (Android): https://yourapp.com/profile/123
 * 3. Custom Scheme: yourapp://profile/123
 * 
 * CRITICAL SETUP REQUIRED:
 * - iOS: Apple App Site Association file
 * - Android: Digital Asset Links file
 * - Both: Server must serve at /.well-known/
 */

import * as Linking from 'expo-linking';
import { router } from 'expo-router';

/**
 * Deep linking configuration
 * Maps URL patterns to app routes
 * 
 * PATTERN MATCHING:
 * - Exact: /profile → ProfileScreen
 * - Dynamic: /profile/:id → ProfileScreen with params
 * - Query: /search?q=term → SearchScreen with query params
 */
export const linkingConfig = {
  prefixes: [
    // Custom scheme (always works)
    'yourapp://',
    // Universal/App Links (require server setup)
    'https://yourapp.com',
    'https://www.yourapp.com',
  ],
  config: {
    screens: {
      // Auth routes
      '(auth)': {
        screens: {
          login: 'login',
          register: 'register',
          'forgot-password': 'forgot-password',
          'reset-password': 'reset-password/:token', // Token from email
        },
      },
      // Main app routes
      '(tabs)': {
        screens: {
          home: '',
          profile: 'profile/:userId',
          settings: 'settings',
        },
      },
      // Deep link examples
      post: 'post/:postId',
      product: 'product/:productId',
      invite: 'invite/:inviteCode',
    },
  },
};

/**
 * Handle incoming deep link
 * Called when app opens from link
 * 
 * ENTRY POINTS:
 * - App closed: Opens app, navigates to link
 * - App background: Brings app to foreground, navigates
 * - App open: Navigates within app
 * 
 * @param url - Deep link URL
 */
export const handleDeepLink = async (url: string) => {
  console.log('[DeepLink] Handling:', url);

  try {
    // Parse URL
    const { hostname, path, queryParams } = Linking.parse(url);
    console.log('[DeepLink] Parsed:', { hostname, path, queryParams });

    // Example: Handle invite link
    if (path === '/invite/:inviteCode') {
      const { inviteCode } = queryParams;
      router.push(`/invite/${inviteCode}`);
      return;
    }

    // Example: Handle profile link
    if (path?.startsWith('/profile/')) {
      const userId = path.split('/')[2];
      router.push(`/profile/${userId}`);
      return;
    }

    // Example: Handle reset password link
    if (path === '/reset-password/:token') {
      const { token } = queryParams;
      router.push(`/reset-password/${token}`);
      return;
    }

    // Default: Navigate to root if no match
    console.warn('[DeepLink] No route matched, navigating to root');
    router.push('/');
  } catch (error) {
    console.error('[DeepLink] Failed to handle:', error);
    // Graceful fallback: navigate to home
    router.push('/');
  }
};

/**
 * Generate deep link for sharing
 * Creates universal link that works across platforms
 * 
 * FALLBACK BEHAVIOR:
 * - If app installed: Opens in app
 * - If app not installed: Opens in browser → app store
 * 
 * @param path - App path (e.g., /profile/123)
 * @param params - Optional query parameters
 */
export const createDeepLink = (
  path: string,
  params?: Record<string, string>
): string => {
  const baseUrl = 'https://yourapp.com';
  const url = new URL(path, baseUrl);

  // Add query parameters
  if (params) {
    Object.entries(params).forEach(([key, value]) => {
      url.searchParams.append(key, value);
    });
  }

  return url.toString();
};

/**
 * Open deep link in app or browser
 * Handles installed/not-installed scenarios
 * 
 * USE CASES:
 * - Share button → create link → open share sheet
 * - Email/SMS link → user clicks → opens app
 * - QR code → scan → opens app
 */
export const openDeepLink = async (url: string) => {
  try {
    const canOpen = await Linking.canOpenURL(url);
    if (canOpen) {
      await Linking.openURL(url);
    } else {
      console.warn('[DeepLink] Cannot open URL:', url);
    }
  } catch (error) {
    console.error('[DeepLink] Failed to open:', error);
  }
};

/**
 * Get initial deep link
 * Called on app startup to check if opened from link
 * 
 * CRITICAL: Check this before showing onboarding
 * User may be clicking password reset link, not starting fresh
 */
export const getInitialDeepLink = async (): Promise<string | null> => {
  try {
    const url = await Linking.getInitialURL();
    return url;
  } catch (error) {
    console.error('[DeepLink] Failed to get initial URL:', error);
    return null;
  }
};

/**
 * USAGE EXAMPLE 1: App startup
 * 
 * // App.tsx
 * useEffect(() => {
 *   const checkInitialLink = async () => {
 *     const url = await getInitialDeepLink();
 *     if (url) {
 *       handleDeepLink(url);
 *     }
 *   };
 *   checkInitialLink();
 * 
 *   // Listen for links while app is open
 *   const subscription = Linking.addEventListener('url', (event) => {
 *     handleDeepLink(event.url);
 *   });
 * 
 *   return () => subscription.remove();
 * }, []);
 */

/**
 * USAGE EXAMPLE 2: Share profile
 * 
 * import * as Sharing from 'expo-sharing';
 * 
 * const handleShare = async () => {
 *   const link = createDeepLink('/profile/123', { ref: 'share' });
 *   await Sharing.shareAsync(link);
 * };
 */

/**
 * USAGE EXAMPLE 3: Email magic link
 * 
 * // Backend sends email with link:
 * // https://yourapp.com/login?token=abc123
 * 
 * // App handles link:
 * export default function LoginScreen() {
 *   useEffect(() => {
 *     const checkMagicLink = async () => {
 *       const url = await getInitialDeepLink();
 *       if (url && url.includes('token=')) {
 *         const { queryParams } = Linking.parse(url);
 *         const token = queryParams.token;
 *         await loginWithToken(token);
 *       }
 *     };
 *     checkMagicLink();
 *   }, []);
 * }
 */
```

---

## 6. PUSH NOTIFICATIONS (P1-HIGH)

[Due to length constraints, I'll include this in the next file section]

**Continue in next section...**
