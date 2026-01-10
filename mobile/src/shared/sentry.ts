// ==============================================================================
// SENTRY ERROR TRACKING SETUP
// ==============================================================================
//
// SETUP INSTRUCTIONS:
// 1. Install: npx expo install @sentry/react-native
// 2. Create account at https://sentry.io
// 3. Get DSN from project settings
// 4. Set EXPO_PUBLIC_SENTRY_DSN in .env
//
// ==============================================================================

import { Platform } from 'react-native';

// Sentry configuration
const SENTRY_DSN = process.env.EXPO_PUBLIC_SENTRY_DSN || '';

// Initialize Sentry (call this in App.tsx before anything else)
export async function initSentry(): Promise<void> {
  if (__DEV__) {
    console.log('Sentry disabled in development');
    return;
  }

  if (!SENTRY_DSN) {
    console.warn('SENTRY_DSN not configured - error tracking disabled');
    return;
  }

  try {
    // Dynamic import to avoid bundling in dev
    const Sentry = await import('@sentry/react-native');
    
    Sentry.init({
      dsn: SENTRY_DSN,
      environment: __DEV__ ? 'development' : 'production',
      enableAutoSessionTracking: true,
      sessionTrackingIntervalMillis: 30000,
      // Capture 100% of transactions for performance monitoring
      tracesSampleRate: 1.0,
      // Don't send events in development
      enabled: !__DEV__,
      // Attach user info to events
      beforeSend(event) {
        // Scrub sensitive data before sending
        if (event.request?.headers) {
          delete event.request.headers['Authorization'];
          delete event.request.headers['Cookie'];
        }
        return event;
      },
    });
    
    console.log('Sentry initialized');
  } catch (error) {
    console.warn('Failed to initialize Sentry:', error);
  }
}

// Capture exception manually
export function captureException(error: Error, context?: Record<string, unknown>): void {
  if (__DEV__) {
    console.error('Sentry.captureException (dev):', error, context);
    return;
  }

  try {
    // Dynamic import
    import('@sentry/react-native').then((Sentry) => {
      Sentry.captureException(error, {
        extra: context,
      });
    });
  } catch {
    console.error('Failed to capture exception:', error);
  }
}

// Capture message
export function captureMessage(message: string, level: 'info' | 'warning' | 'error' = 'info'): void {
  if (__DEV__) {
    console.log(`Sentry.captureMessage (dev) [${level}]:`, message);
    return;
  }

  try {
    import('@sentry/react-native').then((Sentry) => {
      Sentry.captureMessage(message, level);
    });
  } catch {
    console.error('Failed to capture message:', message);
  }
}

// Set user context
export function setUser(user: { id: string; email?: string } | null): void {
  if (__DEV__) return;

  try {
    import('@sentry/react-native').then((Sentry) => {
      Sentry.setUser(user);
    });
  } catch {
    // Ignore
  }
}
