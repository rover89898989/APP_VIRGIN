# Future Fixes: Frontend (Mobile) Architecture Gaps

## Purpose
This document is a **work queue + implementation cookbook** for the gaps identified in the “Expert Architectural Review: APP_VIRGIN Frontend Guidelines”.

- This file is intentionally **future-facing**.
- It includes **sample code** and **exact file paths** to implement later.
- It does **not** require making any code changes right now.

Related documents:
- `INFRASTRUCTURE_AND_SCALING_GUIDE.md` (service selection, scaling decision trees, migration paths)
- `FUTURE_IMPROVEMENTS_PARETO.md` (high-impact feature roadmap: Shopify, OAuth, WebSockets, Images)

---

## Priority Index

### P0 (Critical – do these first)
1. Error Boundaries (root + nested)
2. Performance & Observability (profiling + budgets + crash reporting)
3. Security Hardening (secure storage + client-side checklist)
4. Offline-First Strategy (React Query persistence + network awareness)

### P1 (High)
5. Deep Linking & Universal Links
6. Push Notifications
7. FlashList migration guide (lists)
8. Analytics abstraction

### P2 (Medium)
9. Feature flags
10. Environment configuration strategy
11. CI/CD + release process notes (mobile)
12. App versioning / OTA update strategy

### P3 (Nice-to-have)
13. Biometrics patterns
14. i18n patterns
15. App icon/splash guidance
16. Code signing + store submission notes

---

## P0.1 Error Boundaries

### Goal
Prevent “white screen” failures and provide:
- A fallback UI
- A retry path
- Logging to a monitoring service

### Recommended files (to add/verify)
- `mobile/src/shared/components/ErrorBoundary.tsx`
- `mobile/App.tsx` (wrap the root)
- Optional: route/screen-level wrappers

### Sample implementation
```ts
// mobile/src/shared/components/ErrorBoundary.tsx
import React, { Component, ErrorInfo, ReactNode } from 'react';
import { View, Text, Pressable } from 'react-native';

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
  onError?: (error: Error, errorInfo: ErrorInfo) => void;
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
    this.props.onError?.(error, errorInfo);
  }

  render() {
    if (this.state.hasError) {
      return (
        this.props.fallback || (
          <View style={{ flex: 1, alignItems: 'center', justifyContent: 'center', padding: 16 }}>
            <Text style={{ fontSize: 18, fontWeight: '600', marginBottom: 8 }}>Something went wrong</Text>
            <Pressable onPress={() => this.setState({ hasError: false, error: null })}>
              <Text style={{ color: '#3B82F6' }}>Try Again</Text>
            </Pressable>
          </View>
        )
      );
    }

    return this.props.children;
  }
}
```

### Root integration (later)
```ts
// mobile/App.tsx
import { ErrorBoundary } from './src/shared/components/ErrorBoundary';

export default function App() {
  return (
    <ErrorBoundary>
      {/* existing providers + app UI */}
    </ErrorBoundary>
  );
}
```

### Verification checklist
- Trigger an intentional error in a screen and confirm:
  - fallback renders
  - retry works
  - error is logged

---

## P0.2 Performance & Observability

### Goals
- Establish a performance budget (bundle size, TTI, frame drops)
- Ensure errors/crashes are observable in production

### Performance budget (initial)
| Metric | Target | Tool |
|--------|--------|------|
| JS Bundle Size | < 500KB initial | Metro analyzer |
| Time to Interactive | < 2s | Flipper / profiling |
| Frame drops | < 5% | React DevTools Profiler |
| Memory leaks | 0 critical | Xcode Instruments / Android Studio |

### Monitoring integration (Sentry example)
**Dependencies (later):**
- `@sentry/react-native`

```ts
// mobile/src/shared/utils/monitoring.ts
import * as Sentry from '@sentry/react-native';

export const setupMonitoring = () => {
  Sentry.init({
    dsn: process.env.EXPO_PUBLIC_SENTRY_DSN,
    tracesSampleRate: 1.0,
    enableAutoSessionTracking: true,
  });
};
```

### App wiring (later)
```ts
// mobile/App.tsx
import { useEffect } from 'react';
import { setupMonitoring } from './src/shared/utils/monitoring';

useEffect(() => {
  setupMonitoring();
}, []);
```

### Recommended performance patterns
- Prefer `@shopify/flash-list` over `FlatList`
- Memoize handlers and heavy computations (`useCallback`, `useMemo`)
- Avoid large re-renders by splitting components
- Use `expo-image` for better image perf

---

## P0.3 Security Hardening

### Goal
Make the “secure by default” stance explicit for the frontend.

### Secure storage
**Dependencies (later):**
- `expo-secure-store`

```ts
// mobile/src/shared/utils/secureStorage.ts
import * as SecureStore from 'expo-secure-store';

export const secureStorage = {
  async setToken(token: string) {
    await SecureStore.setItemAsync('auth_token', token);
  },
  async getToken() {
    return await SecureStore.getItemAsync('auth_token');
  },
  async removeToken() {
    await SecureStore.deleteItemAsync('auth_token');
  },
};
```

### API security checklist (frontend)
- [ ] HTTPS only in production
- [ ] Request timeouts (no hanging requests)
- [ ] Token refresh flow (avoid long-lived access tokens)
- [ ] Avoid storing secrets in `AsyncStorage`
- [ ] Input validation on the client (defense in depth)
- [ ] Consider certificate pinning for high-security apps

### Optional: Metro minification hardening (production)
```js
// metro.config.js
module.exports = {
  transformer: {
    minifierConfig: {
      keep_classnames: false,
      keep_fnames: false,
      mangle: { toplevel: true },
    },
  },
};
```

---

## P0.4 Offline-First Strategy

### Goal
Graceful UX under poor connectivity.

### Approach
- Use React Query cache + persistence
- Detect network status
- Design UI states for offline scenarios (offline banner, retry button)

### Network status hook
**Dependencies (later):**
- `@react-native-community/netinfo`

```ts
// mobile/src/shared/hooks/useNetworkStatus.ts
import NetInfo from '@react-native-community/netinfo';
import { useEffect, useState } from 'react';

export const useNetworkStatus = () => {
  const [isConnected, setIsConnected] = useState<boolean | null>(null);

  useEffect(() => {
    const unsubscribe = NetInfo.addEventListener((state) => {
      setIsConnected(state.isConnected);
    });
    return unsubscribe;
  }, []);

  return isConnected;
};
```

### React Query persistence (design note)
React Query persistence can be implemented via:
- `@tanstack/react-query-persist-client`
- a custom persister to storage

**Implementation detail will depend on the React Query major version in `mobile/package.json`.**
When implementing later, we should:
- Confirm whether this repo uses React Query v4 or v5
- Use the matching persistence package and config

---

## P1.5 Deep Linking & Universal Links

### Expo config (example)
```json
// mobile/app.json
{
  "expo": {
    "scheme": "appvirgin",
    "ios": {
      "associatedDomains": ["applinks:yourapp.com"]
    },
    "android": {
      "intentFilters": [
        {
          "action": "VIEW",
          "data": [{ "scheme": "https", "host": "yourapp.com" }],
          "category": ["BROWSABLE", "DEFAULT"]
        }
      ]
    }
  }
}
```

### Route usage (expo-router)
```ts
// app/(tabs)/profile/[userId].tsx
import { useLocalSearchParams } from 'expo-router';

export default function ProfileScreen() {
  const { userId } = useLocalSearchParams<{ userId: string }>();
  // https://yourapp.com/profile/123 => userId === "123"
  return null;
}
```

---

## P1.6 Push Notifications (Expo)

**Dependencies (later):**
- `expo-notifications`

```ts
// mobile/src/shared/hooks/usePushNotifications.ts
import * as Notifications from 'expo-notifications';
import { useEffect } from 'react';

export const usePushNotifications = () => {
  useEffect(() => {
    const register = async () => {
      const { status } = await Notifications.requestPermissionsAsync();
      if (status !== 'granted') return;

      const token = await Notifications.getExpoPushTokenAsync();
      // Send token to backend
      // await fetch(`${API_URL}/users/push-token`, { ... })
      void token;
    };

    void register();
  }, []);
};
```

---

## P1.7 FlashList over FlatList

**Dependency (later):**
- `@shopify/flash-list`

```tsx
import { FlashList } from '@shopify/flash-list';

<FlashList
  data={items}
  renderItem={({ item }) => <ItemCard item={item} />}
  estimatedItemSize={80}
/>
```

---

## P1.8 Analytics Abstraction

```ts
// mobile/src/shared/utils/analytics.ts
interface AnalyticsEvent {
  screen_view: { screen_name: string };
  button_press: { button_id: string; screen: string };
  api_error: { endpoint: string; status: number };
}

export const analytics = {
  track<T extends keyof AnalyticsEvent>(event: T, params: AnalyticsEvent[T]) {
    // Later: wire to Firebase / Mixpanel / Amplitude
    console.log('[Analytics]', event, params);
  },
};
```

---

## P2.9 Feature Flags

```ts
// mobile/src/shared/config/featureFlags.ts
export const featureFlags = {
  newProfileUI: process.env.EXPO_PUBLIC_FF_NEW_PROFILE === 'true',
  betaFeatures: __DEV__,
} as const;
```

---

## P2.10 Environment Configuration

### Suggested `.env` approach (Expo)
- `.env.development`
- `.env.staging`
- `.env.production`

```bash
# .env.development
EXPO_PUBLIC_API_URL=http://localhost:8000
EXPO_PUBLIC_SENTRY_DSN=
```

```ts
// mobile/src/shared/config/env.ts
export const env = {
  apiUrl: process.env.EXPO_PUBLIC_API_URL ?? 'http://localhost:8000',
  sentryDsn: process.env.EXPO_PUBLIC_SENTRY_DSN,
} as const;
```

---

## Appendix: Additional Future Fixes Needed (from `FUTURE_FIXES_NEEDED/`)

This appendix captures additional “ready-to-implement later” code patterns referenced in:

- `FUTURE_FIXES_NEEDED/FRONTEND_PRODUCTION_ADDITIONS.md`
- `FUTURE_FIXES_NEEDED/FRONTEND_PRODUCTION_ADDITIONS_PART2.md`
- `FUTURE_FIXES_NEEDED/COMPREHENSIVE_ARCHITECTURAL_REVIEW.md`

It is included here so that **all frontend production hardening work** is tracked in one place.

---

## A.1 Security: Client Input Validation + Sanitization (P0)

### Goal

- Provide fast client-side validation for better UX.
- Provide baseline sanitization for user-generated text.
- Keep server-side validation as the authoritative control.

### Recommended file (later)

- `mobile/src/shared/utils/inputValidation.ts`

### Sample code (excerpt)

```ts
// mobile/src/shared/utils/inputValidation.ts

export const validateEmail = (email: string): { isValid: boolean; error?: string } => {
  if (!email || email.trim().length === 0) return { isValid: false, error: 'Email is required' };
  if (email.length > 255) return { isValid: false, error: 'Email is too long (max 255 characters)' };

  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  if (!emailRegex.test(email)) return { isValid: false, error: 'Email format is invalid' };

  const [localPart, domainPart] = email.split('@');
  if (localPart.length > 64) {
    return { isValid: false, error: 'Email local part is too long (max 64 characters)' };
  }

  const domainParts = domainPart.split('.');
  if (domainParts.length < 2) return { isValid: false, error: 'Email must have a valid domain' };

  const tld = domainParts[domainParts.length - 1];
  if (tld.length < 2) return { isValid: false, error: 'Email domain extension is too short' };

  return { isValid: true };
};

export const validatePassword = (
  password: string
): { isValid: boolean; error?: string; strength?: 'weak' | 'medium' | 'strong' } => {
  if (!password || password.length === 0) return { isValid: false, error: 'Password is required' };
  if (password.length < 8) {
    return { isValid: false, error: 'Password must be at least 8 characters', strength: 'weak' };
  }

  const hasUppercase = /[A-Z]/.test(password);
  const hasLowercase = /[a-z]/.test(password);
  const hasNumber = /[0-9]/.test(password);
  const hasSpecial = /[!@#$%^&*(),.?":{}|<>]/.test(password);
  const typesCount = [hasUppercase, hasLowercase, hasNumber, hasSpecial].filter(Boolean).length;

  if (typesCount < 3) {
    return {
      isValid: false,
      error: 'Password must contain at least 3 of: uppercase, lowercase, number, special character',
      strength: 'weak',
    };
  }

  const strength: 'weak' | 'medium' | 'strong' =
    password.length >= 12 && typesCount === 4 ? 'strong' : password.length >= 10 ? 'medium' : 'weak';

  return { isValid: true, strength };
};

export const sanitizeString = (input: string): string => {
  if (!input) return '';
  return input
    .replace(/<script\b[^<]*(?:(?!<\/script>)<[^<]*)*<\/script>/gi, '')
    .replace(/<[^>]*>/g, '')
    .replace(/on\w+="[^"]*"/g, '')
    .trim();
};

export const validatePhoneNumber = (
  phone: string
): { isValid: boolean; error?: string } => {
  if (!phone || phone.trim().length === 0) return { isValid: false, error: 'Phone number is required' };
  const e164Regex = /^\+[1-9]\d{1,14}$/;
  if (!e164Regex.test(phone)) {
    return { isValid: false, error: 'Phone number must be in E.164 format (e.g., +14155552671)' };
  }
  return { isValid: true };
};

export const validateURL = (url: string): { isValid: boolean; error?: string } => {
  if (!url || url.trim().length === 0) return { isValid: false, error: 'URL is required' };
  try {
    const parsedUrl = new URL(url);
    if (process.env.NODE_ENV === 'production' && parsedUrl.protocol !== 'https:') {
      return { isValid: false, error: 'Only HTTPS URLs are allowed' };
    }
    const blockedProtocols = ['javascript:', 'data:', 'file:'];
    if (blockedProtocols.includes(parsedUrl.protocol)) {
      return { isValid: false, error: 'URL protocol is not allowed' };
    }
    return { isValid: true };
  } catch {
    return { isValid: false, error: 'Invalid URL format' };
  }
};
```

### Notes

- `sanitizeString()` is intentionally basic.
- For rich-text / HTML contexts, prefer a vetted sanitizer (platform-dependent).

---

## A.2 Offline-First: Full React Query Persistence Setup (P0)

### Goal

- Persist query cache across app restarts.
- Prefer cached reads when offline.
- Sync/refetch when connectivity returns.

### Recommended files (later)

- `mobile/src/api/queryClient.ts`
- `mobile/src/shared/hooks/useNetworkStatus.ts`

### Dependencies (later)

- `@react-native-async-storage/async-storage`
- `@react-native-community/netinfo`

React Query persistence options (choose based on your React Query version):

- v4-ish approach: `@tanstack/query-async-storage-persister`
- v5-ish approach: `@tanstack/react-query-persist-client` + a persister

### Sample config (from production additions, adjust for your React Query major)

```ts
// mobile/src/api/queryClient.ts
import { QueryClient } from '@tanstack/react-query';
import { createAsyncStoragePersister } from '@tanstack/query-async-storage-persister';
import AsyncStorage from '@react-native-async-storage/async-storage';

const asyncStoragePersister = createAsyncStoragePersister({
  storage: AsyncStorage,
});

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      cacheTime: 1000 * 60 * 60 * 24,
      staleTime: 1000 * 60 * 5,
      retry: 2,
      retryDelay: (attemptIndex) => Math.min(1000 * 2 ** attemptIndex, 30000),
      networkMode: 'offlineFirst',
      refetchOnWindowFocus: true,
      refetchOnReconnect: true,
      refetchOnMount: true,
      structuralSharing: true,
    },
    mutations: {
      retry: false,
      networkMode: 'online',
    },
  },
});

export { asyncStoragePersister };
```

### Wiring (later)

```ts
// mobile/App.tsx (or expo-router root)
import { PersistQueryClientProvider } from '@tanstack/react-query-persist-client';
import { queryClient, asyncStoragePersister } from './src/api/queryClient';

<PersistQueryClientProvider
  client={queryClient}
  persistOptions={{ persister: asyncStoragePersister }}
>
  {/* app */}
</PersistQueryClientProvider>
```

### Network status hook (richer variant)

```ts
// mobile/src/shared/hooks/useNetworkStatus.ts
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
    const unsubscribe = NetInfo.addEventListener((state: NetInfoState) => {
      setNetworkStatus({
        isConnected: state.isConnected,
        isInternetReachable: state.isInternetReachable,
        type: state.type,
        details: state.details,
      });
    });
    return () => unsubscribe();
  }, []);

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
    isOnline: networkStatus.isConnected === true,
    isOffline: networkStatus.isConnected === false,
    isUnknown: networkStatus.isConnected === null,
  };
};
```

### Important note

- React Query v5 uses `gcTime` rather than `cacheTime`. When implementing, align with the version in `mobile/package.json`.

---

## A.3 Deep Linking: Utilities + Server Well-Known Files (P1)

### Goal

- Support:
  - Universal Links (iOS)
  - App Links (Android)
  - Custom scheme (always works)

### Recommended files (later)

- `mobile/app.json` (update)
- `mobile/src/shared/utils/linking.ts`

### Sample linking utility (excerpt)

```ts
// mobile/src/shared/utils/linking.ts
import * as Linking from 'expo-linking';
import { router } from 'expo-router';

export const linkingConfig = {
  prefixes: ['yourapp://', 'https://yourapp.com', 'https://www.yourapp.com'],
  config: {
    screens: {
      '(auth)': {
        screens: {
          login: 'login',
          register: 'register',
          'forgot-password': 'forgot-password',
          'reset-password': 'reset-password/:token',
        },
      },
      '(tabs)': {
        screens: {
          home: '',
          profile: 'profile/:userId',
          settings: 'settings',
        },
      },
      post: 'post/:postId',
      product: 'product/:productId',
      invite: 'invite/:inviteCode',
    },
  },
};

export const createDeepLink = (path: string) => Linking.createURL(path);

export const openDeepLink = async (url: string) => {
  const parsed = Linking.parse(url);
  if (!parsed.path) return;
  router.push('/' + parsed.path);
};
```

### Required server setup (later)

- iOS requires an Apple App Site Association file at:
  - `https://yourapp.com/.well-known/apple-app-site-association`
- Android requires a Digital Asset Links file at:
  - `https://yourapp.com/.well-known/assetlinks.json`

---

## A.4 Feature Flags: Rollouts + Experiments + Ops Circuit Breakers (P2)

### Goal

- Gradually roll out risky UI/features.
- Run experiments.
- Disable features during backend incidents.

### Recommended file (later)

- `mobile/src/shared/config/featureFlags.ts`

### Sample feature flags (excerpt)

```ts
// mobile/src/shared/config/featureFlags.ts
export const featureFlags = {
  newProfileUI: getBooleanFlag('FF_NEW_PROFILE', false),
  pushNotifications: getBooleanFlag('FF_PUSH_NOTIFICATIONS', false),
  enableComments: getBooleanFlag('OPS_COMMENTS', true),
  betaFeatures: getBooleanFlag('FF_BETA', __DEV__),
} as const;

function getBooleanFlag(envVar: string, defaultValue: boolean): boolean {
  const value = process.env[envVar];
  if (value === undefined) return defaultValue;
  return value === '1' || value.toLowerCase() === 'true' || value.toLowerCase() === 'yes';
}

export function isUserInRollout(userId: string, percentage: number): boolean {
  if (percentage === 0) return false;
  if (percentage === 100) return true;
  const hash = hashString(userId);
  return (hash % 100) < percentage;
}

function hashString(str: string): number {
  let hash = 0;
  for (let i = 0; i < str.length; i++) {
    hash = ((hash << 5) - hash) + str.charCodeAt(i);
    hash = hash & hash;
  }
  return Math.abs(hash);
}
```

---

## A.5 Environment Configuration: Full Pattern + Validation (P2)

### Goal

- Make `dev` / `staging` / `production` explicit.
- Validate required variables at startup.
- Keep secrets out of git.

### Recommended files (later)

- `mobile/.env.example`
- `mobile/src/shared/config/env.ts`

### `.env.example` template (excerpt)

```bash
EXPO_PUBLIC_API_URL=http://localhost:8000/api
EXPO_PUBLIC_WS_URL=ws://localhost:8000/ws

EXPO_PUBLIC_FF_NEW_PROFILE=false
EXPO_PUBLIC_FF_DARK_MODE=true
EXPO_PUBLIC_FF_PUSH_NOTIFICATIONS=false

EXPO_PUBLIC_SENTRY_DSN=
EXPO_PUBLIC_ENVIRONMENT=development
```

### Full env module pattern (excerpt)

```ts
// mobile/src/shared/config/env.ts
import Constants from 'expo-constants';

type Environment = 'development' | 'staging' | 'production';

interface EnvConfig {
  apiUrl: string;
  wsUrl?: string;
  environment: Environment;
  sentryDsn?: string;
  isProduction: boolean;
  isDevelopment: boolean;
}

function getEnvironment(): Environment {
  const releaseChannel = Constants.expoConfig?.extra?.releaseChannel;
  if (releaseChannel === 'production') return 'production';
  if (releaseChannel === 'staging') return 'staging';

  const env = process.env.EXPO_PUBLIC_ENVIRONMENT;
  if (env === 'production') return 'production';
  if (env === 'staging') return 'staging';
  return 'development';
}

const ENV_CONFIG: Record<Environment, EnvConfig> = {
  development: {
    apiUrl: process.env.EXPO_PUBLIC_API_URL || 'http://localhost:8000/api',
    wsUrl: process.env.EXPO_PUBLIC_WS_URL || 'ws://localhost:8000/ws',
    environment: 'development',
    sentryDsn: undefined,
    isProduction: false,
    isDevelopment: true,
  },
  staging: {
    apiUrl: process.env.EXPO_PUBLIC_API_URL || 'https://staging-api.yourapp.com',
    wsUrl: process.env.EXPO_PUBLIC_WS_URL || 'wss://staging-api.yourapp.com/ws',
    environment: 'staging',
    sentryDsn: process.env.EXPO_PUBLIC_SENTRY_DSN,
    isProduction: false,
    isDevelopment: false,
  },
  production: {
    apiUrl: process.env.EXPO_PUBLIC_API_URL || 'https://api.yourapp.com',
    wsUrl: process.env.EXPO_PUBLIC_WS_URL || 'wss://api.yourapp.com/ws',
    environment: 'production',
    sentryDsn: process.env.EXPO_PUBLIC_SENTRY_DSN,
    isProduction: true,
    isDevelopment: false,
  },
};

const currentEnv = getEnvironment();
export const env = ENV_CONFIG[currentEnv];

export function validateEnv() {
  const required: (keyof EnvConfig)[] = ['apiUrl'];
  for (const key of required) {
    if (!env[key]) throw new Error(`Missing required environment variable: ${key}`);
  }
}
```

---

## A.6 Deep Linking: Handling + Sharing Patterns (P1)

### Goal

- Handle initial link on cold start.
- Handle links received while app is already open.
- Provide safe helpers for:
  - sharing profile/post links
  - password reset links
  - invites

### Recommended file (later)

- `mobile/src/shared/utils/linking.ts`

### Sample code (excerpt)

```ts
// mobile/src/shared/utils/linking.ts
import * as Linking from 'expo-linking';
import { router } from 'expo-router';

export const handleDeepLink = async (url: string) => {
  try {
    const { path, queryParams } = Linking.parse(url);

    if (path?.startsWith('/profile/')) {
      const userId = path.split('/')[2];
      router.push(`/profile/${userId}`);
      return;
    }

    if (path?.startsWith('/invite/')) {
      const inviteCode = path.split('/')[2];
      router.push(`/invite/${inviteCode}`);
      return;
    }

    if (path?.startsWith('/reset-password/')) {
      const token = path.split('/')[2] ?? (queryParams?.token as string | undefined);
      if (token) {
        router.push(`/reset-password/${token}`);
        return;
      }
    }

    router.push('/');
  } catch {
    router.push('/');
  }
};

export const createDeepLink = (path: string, params?: Record<string, string>): string => {
  const baseUrl = 'https://yourapp.com';
  const url = new URL(path, baseUrl);
  if (params) {
    Object.entries(params).forEach(([key, value]) => url.searchParams.append(key, value));
  }
  return url.toString();
};

export const openDeepLink = async (url: string) => {
  const canOpen = await Linking.canOpenURL(url);
  if (canOpen) await Linking.openURL(url);
};

export const getInitialDeepLink = async (): Promise<string | null> => {
  try {
    return await Linking.getInitialURL();
  } catch {
    return null;
  }
};
```

### App startup wiring (later)

```ts
// App.tsx (or expo-router root layout)
import * as Linking from 'expo-linking';
import { handleDeepLink, getInitialDeepLink } from '@/shared/utils/linking';

useEffect(() => {
  const run = async () => {
    const url = await getInitialDeepLink();
    if (url) await handleDeepLink(url);
  };
  void run();

  const sub = Linking.addEventListener('url', (event) => {
    void handleDeepLink(event.url);
  });

  return () => sub.remove();
}, []);
```

---

## A.7 Push Notifications: Recommended Patterns (P1)

### Goal

- Centralize registration + listeners.
- Ensure navigation works on tap.
- Avoid memory leaks (unsubscribe listeners).

### Recommended file (later)

- `mobile/src/shared/hooks/usePushNotifications.ts`

### Notes / constraints

- Push tokens require a physical device for real testing.
- Android requires a notification channel for Android 8+.

### Sample code (excerpt)

```ts
// mobile/src/shared/hooks/usePushNotifications.ts
import { useEffect, useRef } from 'react';
import * as Notifications from 'expo-notifications';
import * as Device from 'expo-device';
import { Platform } from 'react-native';
import { router } from 'expo-router';

Notifications.setNotificationHandler({
  handleNotification: async () => ({
    shouldShowAlert: true,
    shouldPlaySound: true,
    shouldSetBadge: true,
  }),
});

export const usePushNotifications = () => {
  const notificationListener = useRef<Notifications.Subscription>();
  const responseListener = useRef<Notifications.Subscription>();

  useEffect(() => {
    const register = async () => {
      if (!Device.isDevice) return;

      const { status: existingStatus } = await Notifications.getPermissionsAsync();
      const { status } =
        existingStatus === 'granted' ? { status: existingStatus } : await Notifications.requestPermissionsAsync();
      if (status !== 'granted') return;

      if (Platform.OS === 'android') {
        await Notifications.setNotificationChannelAsync('default', {
          name: 'default',
          importance: Notifications.AndroidImportance.MAX,
        });
      }

      const token = (await Notifications.getExpoPushTokenAsync()).data;
      // TODO: send `token` to backend
      void token;
    };

    void register();

    notificationListener.current = Notifications.addNotificationReceivedListener(() => {
      // TODO: update UI (badge counts, query invalidation, etc.)
    });

    responseListener.current = Notifications.addNotificationResponseReceivedListener((response) => {
      const data = response.notification.request.content.data as any;
      if (data?.type === 'new_message' && data?.conversationId) {
        router.push(`/messages/${data.conversationId}`);
      }
    });

    return () => {
      if (notificationListener.current) {
        Notifications.removeNotificationSubscription(notificationListener.current);
      }
      if (responseListener.current) {
        Notifications.removeNotificationSubscription(responseListener.current);
      }
    };
  }, []);
};
```

---

## A.8 Production Readiness Checklist (Frontend) (P0/P1)

This is a doc-only checklist to prevent regressions and ensure the “template” remains production-grade.

### Error Handling

- [ ] Root `ErrorBoundary` wraps app
- [ ] Feature-level boundaries for high-risk screens
- [ ] Errors logged to monitoring service
- [ ] Fallback UX provides a recovery action

### Performance

- [ ] Profiling workflow documented (React DevTools / Flipper)
- [ ] Performance budget defined
- [ ] Lists use `FlashList` for 20+ items
- [ ] Cleanup in `useEffect` prevents leaks

### Security

- [ ] Tokens stored securely (`SecureStore` on native; httpOnly cookies on web)
- [ ] No tokens/PII in logs, analytics, or monitoring metadata
- [ ] HTTPS-only in production builds
- [ ] Client-side validation and sanitization exist (UX + defense-in-depth)

### Offline support

- [ ] React Query persistence configured
- [ ] Network status detection present
- [ ] Offline banner and retry affordances exist
- [ ] Sync/refetch on reconnect

### Deep linking

- [ ] `app.json` configured (scheme, associatedDomains, intentFilters)
- [ ] `.well-known` files served by your domain
- [ ] Initial link handling works
- [ ] In-app link listener works

### Push notifications

- [ ] Permission request strategy (not always at launch)
- [ ] Push token stored + sent to backend
- [ ] Notification tap navigation works

---

## A.9 Integration Checklist (when implementing later)

### Dependencies to install (later)

```bash
# monitoring
npm install @sentry/react-native

# offline support
npm install @react-native-community/netinfo @react-native-async-storage/async-storage

# react query persistence (pick based on react-query major)
npm install @tanstack/query-async-storage-persister
# or
npm install @tanstack/react-query-persist-client

# push notifications
npm install expo-notifications expo-device

# performance
npm install @shopify/flash-list
```

### Root wiring checklist (later)

- [ ] `validateEnv()` called early
- [ ] monitoring setup called early
- [ ] analytics setup called early
- [ ] `PersistQueryClientProvider` wraps the app
- [ ] `ErrorBoundary` wraps the app

---

## Notes for Later Implementation Session

### Implementation order recommendation
1. Confirm React Query version + persistence strategy
2. Wire root `ErrorBoundary`
3. Add `monitoring.ts` + environment variables
4. Add offline network status + UI indicators
5. Add deep links, push notifications, analytics, feature flags

### When we implement later
- We will make changes **incrementally**, with `git diff` review between steps.
- We will keep dependencies minimal and aligned with existing Expo SDK versions.
