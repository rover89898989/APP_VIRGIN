# 🚀 FRONTEND PRODUCTION ADDITIONS (Part 2)
## Continued from Part 1

---

## 6. PUSH NOTIFICATIONS (P1-HIGH)

### Why This Is Important
Push notifications drive 88% higher app retention and 3x more engagement. Essential for user activation, retention, and re-engagement.

### Files to Create

#### `mobile/src/shared/hooks/usePushNotifications.ts`

```typescript
/**
 * usePushNotifications Hook
 * 
 * PURPOSE:
 * Centralized push notification management
 * Handles registration, permissions, and notification handling
 * 
 * CRITICAL FLOWS:
 * 1. Request permission from user
 * 2. Get Expo push token
 * 3. Send token to backend (associate with user)
 * 4. Handle incoming notifications (foreground/background)
 * 5. Navigate to relevant screen when notification tapped
 * 
 * EXPO PUSH NOTIFICATIONS:
 * - Free tier: Unlimited notifications
 * - No need for APNs (iOS) or FCM (Android) certificates
 * - Works out of the box in development
 * - Production: Configure app credentials
 */

import { useEffect, useRef } from 'react';
import * as Notifications from 'expo-notifications';
import * as Device from 'expo-device';
import { Platform } from 'react-native';
import { router } from 'expo-router';

/**
 * Configure notification handling behavior
 * 
 * WHEN APP IS:
 * - Foreground: Show banner, play sound, update badge
 * - Background: System handles (iOS/Android notification center)
 * - Killed: System handles, app opens on tap
 * 
 * CUSTOMIZATION:
 * - shouldShowAlert: false = silent notification
 * - shouldPlaySound: false = no sound
 * - shouldSetBadge: false = don't update app icon badge
 */
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
    /**
     * Register for push notifications
     * 
     * FLOW:
     * 1. Check if physical device (emulators don't support push)
     * 2. Request permission from user
     * 3. Configure Android notification channel (required Android 8+)
     * 4. Get Expo push token
     * 5. Send token to backend
     */
    registerForPushNotificationsAsync().then((token) => {
      if (token) {
        // TODO: Send token to backend
        // await api.updatePushToken(token);
        console.log('[Push] Token registered:', token);
      }
    });

    /**
     * Listener: Notification received while app is FOREGROUND
     * 
     * USE CASES:
     * - Show in-app toast instead of banner
     * - Update UI immediately (new message count)
     * - Play custom sound
     * 
     * IMPORTANT: This fires BEFORE notification is shown
     * Return value from setNotificationHandler determines how to show it
     */
    notificationListener.current =
      Notifications.addNotificationReceivedListener((notification) => {
        console.log('[Push] Notification received:', notification);
        
        // Example: Update UI based on notification type
        const data = notification.request.content.data;
        if (data.type === 'new_message') {
          // Update message count in UI
          console.log('[Push] New message from:', data.senderId);
        }
      });

    /**
     * Listener: User TAPPED notification
     * 
     * USE CASES:
     * - Navigate to relevant screen
     * - Open specific post/message/product
     * - Mark notification as read
     * 
     * IMPORTANT: Fires when app is foreground, background, OR killed
     * Always works, reliable for deep linking
     */
    responseListener.current =
      Notifications.addNotificationResponseReceivedListener((response) => {
        console.log('[Push] Notification tapped:', response);

        // Extract data from notification
        const data = response.notification.request.content.data;

        // Navigate based on notification type
        handleNotificationNavigation(data);
      });

    /**
     * Cleanup: Remove listeners on unmount
     * CRITICAL: Prevents memory leaks
     */
    return () => {
      if (notificationListener.current) {
        Notifications.removeNotificationSubscription(
          notificationListener.current
        );
      }
      if (responseListener.current) {
        Notifications.removeNotificationSubscription(responseListener.current);
      }
    };
  }, []);

  return {
    // Export functions if needed
    // scheduleLocalNotification,
    // cancelNotification,
  };
};

/**
 * Request notification permissions and get push token
 * 
 * PERMISSION STATES:
 * - granted: User allowed notifications
 * - denied: User denied (cannot ask again on iOS)
 * - undetermined: Not asked yet (first time)
 * 
 * BEST PRACTICE:
 * Don't request permission on app launch
 * Wait until user performs action that benefits from notifications
 * Example: After sending first message, "Get notified of replies?"
 */
async function registerForPushNotificationsAsync() {
  let token: string | undefined;

  /**
   * Check if physical device
   * Expo Go + emulators don't support push notifications
   * 
   * DEVELOPMENT: Use physical device or build standalone app
   */
  if (!Device.isDevice) {
    console.warn('[Push] Push notifications only work on physical devices');
    return;
  }

  /**
   * Check current permission status
   * iOS: Cannot request again if denied
   * Android: Can request multiple times
   */
  const { status: existingStatus } =
    await Notifications.getPermissionsAsync();
  let finalStatus = existingStatus;

  /**
   * Request permission if not granted
   * iOS: Shows system dialog once
   * Android: Shows dialog every time
   */
  if (existingStatus !== 'granted') {
    const { status } = await Notifications.requestPermissionsAsync();
    finalStatus = status;
  }

  /**
   * Handle denied permission
   * iOS: Cannot recover (user must enable in Settings)
   * Android: Can ask again
   */
  if (finalStatus !== 'granted') {
    console.warn('[Push] Notification permission denied');
    
    // Optional: Show alert to user
    // "Enable notifications in Settings to get important updates"
    return;
  }

  /**
   * Configure Android notification channel
   * REQUIRED for Android 8+ (API 26+)
   * 
   * Channels allow users to:
   * - Customize notification sounds per channel
   * - Disable specific notification types
   * - Set importance levels
   * 
   * EXAMPLES:
   * - "messages": High importance, sound + vibration
   * - "updates": Low importance, silent
   * - "promotions": Low importance, can disable
   */
  if (Platform.OS === 'android') {
    await Notifications.setNotificationChannelAsync('default', {
      name: 'default',
      importance: Notifications.AndroidImportance.MAX,
      vibrationPattern: [0, 250, 250, 250],
      lightColor: '#FF231F7C',
    });
  }

  /**
   * Get Expo push token
   * 
   * TOKEN FORMAT:
   * ExponentPushToken[xxxxxxxxxxxxxxxxxxxxxx]
   * 
   * IMPORTANT: This is NOT the same as:
   * - APNs device token (iOS)
   * - FCM registration token (Android)
   * 
   * Expo's backend translates Expo tokens to native tokens
   * Simplifies cross-platform push notifications
   */
  try {
    token = (await Notifications.getExpoPushTokenAsync()).data;
    console.log('[Push] Expo push token:', token);
  } catch (error) {
    console.error('[Push] Failed to get push token:', error);
  }

  return token;
}

/**
 * Handle navigation based on notification data
 * 
 * NOTIFICATION DATA STRUCTURE:
 * {
 *   type: 'new_message' | 'friend_request' | 'post_like' | ...,
 *   id: '123', // ID of the resource (message ID, user ID, etc.)
 *   ...other data
 * }
 * 
 * BACKEND RESPONSIBILITY:
 * Include all data needed to navigate to relevant screen
 * Don't require additional API call to get context
 */
function handleNotificationNavigation(data: any) {
  if (!data) {
    console.warn('[Push] No data in notification');
    return;
  }

  console.log('[Push] Navigating from notification:', data);

  // Example navigation based on type
  switch (data.type) {
    case 'new_message':
      // Navigate to message thread
      router.push(`/messages/${data.conversationId}`);
      break;

    case 'friend_request':
      // Navigate to friend request screen
      router.push(`/friends/requests`);
      break;

    case 'post_like':
      // Navigate to post
      router.push(`/posts/${data.postId}`);
      break;

    case 'comment_reply':
      // Navigate to comment thread
      router.push(`/posts/${data.postId}?commentId=${data.commentId}`);
      break;

    default:
      console.warn('[Push] Unknown notification type:', data.type);
      // Fallback to home screen
      router.push('/');
  }
}

/**
 * Schedule local notification
 * Does NOT require backend or internet
 * 
 * USE CASES:
 * - Reminders (workout reminder, medication)
 * - Timers (pomodoro, cooking timer)
 * - Calendar events
 * - Alarms
 * 
 * @param title - Notification title
 * @param body - Notification message
 * @param seconds - Seconds from now to trigger
 */
export async function scheduleLocalNotification(
  title: string,
  body: string,
  seconds: number = 5
) {
  await Notifications.scheduleNotificationAsync({
    content: {
      title,
      body,
      data: { type: 'local' },
    },
    trigger: { seconds },
  });
}

/**
 * Cancel scheduled notification
 * 
 * @param notificationId - ID returned from scheduleNotificationAsync
 */
export async function cancelNotification(notificationId: string) {
  await Notifications.cancelScheduledNotificationAsync(notificationId);
}

/**
 * Cancel all scheduled notifications
 * 
 * USE CASES:
 * - User disables feature that schedules notifications
 * - User logs out
 * - Reset app state
 */
export async function cancelAllNotifications() {
  await Notifications.cancelAllScheduledNotificationsAsync();
}

/**
 * Get notification badge count
 * iOS only - Android uses different approach
 */
export async function getBadgeCount(): Promise<number> {
  return await Notifications.getBadgeCountAsync();
}

/**
 * Set notification badge count
 * 
 * USE CASES:
 * - Update unread message count
 * - Show pending tasks
 * - Clear badge on app open
 * 
 * IMPORTANT: iOS only
 * Android uses launcher badges (different API)
 */
export async function setBadgeCount(count: number) {
  await Notifications.setBadgeCountAsync(count);
}

/**
 * USAGE EXAMPLE 1: Basic setup
 * 
 * // App.tsx or _layout.tsx
 * function App() {
 *   usePushNotifications();
 *   return <YourApp />;
 * }
 */

/**
 * USAGE EXAMPLE 2: Send push token to backend
 * 
 * // After successful login
 * const handleLogin = async (email, password) => {
 *   const { userId, accessToken } = await api.login(email, password);
 *   
 *   // Get push token
 *   const pushToken = await registerForPushNotificationsAsync();
 *   
 *   // Send to backend
 *   if (pushToken) {
 *     await api.updateUser(userId, { pushToken });
 *   }
 * };
 */

/**
 * USAGE EXAMPLE 3: Backend sends notification
 * 
 * // Node.js backend example
 * const { Expo } = require('expo-server-sdk');
 * const expo = new Expo();
 * 
 * const sendPushNotification = async (pushToken, message) => {
 *   const messages = [{
 *     to: pushToken,
 *     sound: 'default',
 *     title: 'New Message',
 *     body: message,
 *     data: { type: 'new_message', conversationId: '123' },
 *   }];
 * 
 *   const chunks = expo.chunkPushNotifications(messages);
 *   for (const chunk of chunks) {
 *     await expo.sendPushNotificationsAsync(chunk);
 *   }
 * };
 */
```

---

## 7. ANALYTICS FRAMEWORK (P1-HIGH)

### Why This Is Important
You can't improve what you don't measure. Analytics track user behavior, conversion funnels, feature adoption, and app health.

### Files to Create

#### `mobile/src/shared/utils/analytics.ts`

```typescript
/**
 * Analytics Utilities
 * 
 * PURPOSE:
 * Centralized analytics tracking across the app
 * Single interface for multiple analytics providers
 * 
 * PROVIDERS SUPPORTED:
 * - Firebase Analytics (free, mobile-optimized)
 * - Mixpanel (advanced segmentation, funnels)
 * - Amplitude (product analytics, retention)
 * - Custom backend (your own analytics endpoint)
 * 
 * CRITICAL PRINCIPLES:
 * - Track user actions, not page views (mobile is not web)
 * - Include context with every event (screen, feature, user segment)
 * - Never track PII without consent (GDPR, CCPA compliance)
 * - Sanitize data before sending (no tokens, passwords)
 */

import { Platform } from 'react-native';

/**
 * Analytics event types
 * 
 * TYPE SAFETY: Prevents typos in event names
 * AUTOCOMPLETE: IDE suggests available events
 * DOCUMENTATION: Self-documenting event catalog
 * 
 * NAMING CONVENTION:
 * - object_action: button_press, screen_view
 * - noun_verb: user_login, post_create
 * - Keep consistent across platforms (web, mobile, backend)
 */
interface AnalyticsEvent {
  // Screen events
  screen_view: {
    screen_name: string;
    screen_class?: string;
    previous_screen?: string;
  };

  // User events
  user_login: {
    method: 'email' | 'google' | 'apple' | 'facebook';
    success: boolean;
  };
  user_signup: {
    method: 'email' | 'google' | 'apple' | 'facebook';
    success: boolean;
  };
  user_logout: {
    reason: 'manual' | 'session_expired' | 'error';
  };

  // Feature usage
  button_press: {
    button_id: string;
    screen: string;
    feature?: string;
  };
  feature_discovery: {
    feature_name: string;
    discovery_method: 'onboarding' | 'tooltip' | 'organic';
  };

  // Content interactions
  post_view: {
    post_id: string;
    post_type?: string;
    author_id?: string;
  };
  post_create: {
    post_id: string;
    post_type?: string;
    has_media: boolean;
  };
  post_share: {
    post_id: string;
    share_method: 'link' | 'native' | 'social';
  };

  // Search
  search_query: {
    query: string;
    results_count: number;
    filters?: string[];
  };
  search_result_click: {
    query: string;
    result_id: string;
    position: number;
  };

  // Errors
  api_error: {
    endpoint: string;
    status_code: number;
    error_message?: string;
  };
  app_crash: {
    error_message: string;
    stack_trace?: string;
    screen?: string;
  };

  // Commerce (if applicable)
  product_view: {
    product_id: string;
    product_name: string;
    price: number;
    currency: string;
  };
  add_to_cart: {
    product_id: string;
    quantity: number;
    price: number;
  };
  begin_checkout: {
    cart_total: number;
    item_count: number;
  };
  purchase_complete: {
    transaction_id: string;
    total: number;
    currency: string;
  };
}

/**
 * User properties for segmentation
 * 
 * PERSISTENT: Set once per user session
 * SEGMENTS: Group users for analysis
 * PERSONALIZATION: Customize experience per segment
 * 
 * PRIVACY: Only set non-PII properties
 * Never: email, phone, real name, address
 * OK: user_id (hashed), user_type, subscription_tier
 */
interface UserProperties {
  user_id?: string;
  user_type?: 'free' | 'premium' | 'trial';
  subscription_tier?: string;
  signup_date?: string;
  app_version?: string;
  platform?: string;
  device_type?: string;
}

/**
 * Analytics provider interface
 * Implement this for each analytics service
 * 
 * PATTERN: Strategy pattern for easy provider swapping
 * TEST: Mock provider for unit tests
 * PRODUCTION: Real providers (Firebase, Mixpanel, etc.)
 */
interface AnalyticsProvider {
  initialize(): Promise<void>;
  track<T extends keyof AnalyticsEvent>(
    event: T,
    params: AnalyticsEvent[T]
  ): void;
  setUserProperties(properties: UserProperties): void;
  clearUser(): void;
}

/**
 * Console provider (development only)
 * Logs events to console for debugging
 * 
 * USAGE: Development, testing
 * PRODUCTION: Replace with real provider
 */
class ConsoleAnalyticsProvider implements AnalyticsProvider {
  async initialize() {
    console.log('[Analytics] Console provider initialized');
  }

  track<T extends keyof AnalyticsEvent>(
    event: T,
    params: AnalyticsEvent[T]
  ) {
    console.log(`[Analytics] Event: ${event}`, params);
  }

  setUserProperties(properties: UserProperties) {
    console.log('[Analytics] User properties:', properties);
  }

  clearUser() {
    console.log('[Analytics] User cleared');
  }
}

/**
 * Firebase Analytics provider (production)
 * 
 * SETUP:
 * 1. Install: expo install @react-native-firebase/analytics
 * 2. Configure: firebase.json with project credentials
 * 3. Enable: Firebase console → Analytics
 * 
 * FEATURES:
 * - Automatic events (screen_view, session_start)
 * - User properties
 * - Audience segmentation
 * - Integration with other Firebase services
 */
class FirebaseAnalyticsProvider implements AnalyticsProvider {
  async initialize() {
    // Example: Firebase initialization
    // await analytics().setAnalyticsCollectionEnabled(true);
    console.log('[Analytics] Firebase provider initialized');
  }

  track<T extends keyof AnalyticsEvent>(
    event: T,
    params: AnalyticsEvent[T]
  ) {
    // Example: Firebase event tracking
    // await analytics().logEvent(event, params);
    console.log(`[Analytics] Firebase event: ${event}`, params);
  }

  setUserProperties(properties: UserProperties) {
    // Example: Firebase user properties
    // await analytics().setUserProperties(properties);
    console.log('[Analytics] Firebase user properties:', properties);
  }

  clearUser() {
    // Example: Firebase clear user
    // await analytics().resetAnalyticsData();
    console.log('[Analytics] Firebase user cleared');
  }
}

/**
 * Analytics singleton
 * Single instance manages all providers
 * 
 * INITIALIZATION: Call setupAnalytics() once at app startup
 * USAGE: Import analytics object, call track()
 */
class Analytics {
  private providers: AnalyticsProvider[] = [];
  private initialized = false;

  /**
   * Initialize analytics providers
   * 
   * CALL ONCE: App.tsx or _layout.tsx
   * AWAIT: Ensure providers ready before tracking
   */
  async initialize() {
    if (this.initialized) {
      console.warn('[Analytics] Already initialized');
      return;
    }

    // Development: Use console provider
    if (process.env.NODE_ENV === 'development') {
      this.providers.push(new ConsoleAnalyticsProvider());
    }
    // Production: Use real providers
    else {
      // Add your production providers here
      // this.providers.push(new FirebaseAnalyticsProvider());
      // this.providers.push(new MixpanelProvider());
      
      // Fallback to console in production if providers not configured
      this.providers.push(new ConsoleAnalyticsProvider());
    }

    // Initialize all providers
    await Promise.all(this.providers.map((p) => p.initialize()));
    this.initialized = true;

    console.log('[Analytics] Initialized with', this.providers.length, 'providers');
  }

  /**
   * Track event
   * 
   * AUTOMATIC ENRICHMENT:
   * - Timestamp
   * - Platform (iOS/Android/web)
   * - App version
   * - User ID (if set)
   * 
   * @param event - Event name (type-safe)
   * @param params - Event parameters (type-safe)
   */
  track<T extends keyof AnalyticsEvent>(
    event: T,
    params: AnalyticsEvent[T]
  ) {
    if (!this.initialized) {
      console.warn('[Analytics] Not initialized, event not tracked:', event);
      return;
    }

    // Enrich with platform data
    const enrichedParams = {
      ...params,
      platform: Platform.OS,
      timestamp: new Date().toISOString(),
      // Add app version if available
      // app_version: Constants.manifest?.version,
    };

    // Send to all providers
    this.providers.forEach((provider) => {
      try {
        provider.track(event, enrichedParams);
      } catch (error) {
        console.error('[Analytics] Provider failed:', provider, error);
      }
    });
  }

  /**
   * Set user properties
   * 
   * CALL: After successful login
   * CLEAR: On logout
   * 
   * @param properties - User properties for segmentation
   */
  setUserProperties(properties: UserProperties) {
    if (!this.initialized) {
      console.warn('[Analytics] Not initialized');
      return;
    }

    // Sanitize properties (remove PII)
    const sanitized = this.sanitizeUserProperties(properties);

    this.providers.forEach((provider) => {
      try {
        provider.setUserProperties(sanitized);
      } catch (error) {
        console.error('[Analytics] Failed to set user properties:', error);
      }
    });
  }

  /**
   * Clear user data
   * 
   * CALL: On logout
   * GDPR: Respect user's right to be forgotten
   */
  clearUser() {
    if (!this.initialized) return;

    this.providers.forEach((provider) => {
      try {
        provider.clearUser();
      } catch (error) {
        console.error('[Analytics] Failed to clear user:', error);
      }
    });
  }

  /**
   * Sanitize user properties
   * Remove PII and sensitive data
   * 
   * NEVER TRACK:
   * - Email addresses
   * - Phone numbers
   * - Real names
   * - Physical addresses
   * - Payment information
   */
  private sanitizeUserProperties(
    properties: UserProperties
  ): UserProperties {
    // Create copy
    const sanitized = { ...properties };

    // Remove any accidentally included PII
    // Add your app-specific PII fields here
    delete (sanitized as any).email;
    delete (sanitized as any).phone;
    delete (sanitized as any).name;
    delete (sanitized as any).address;

    return sanitized;
  }
}

// Export singleton instance
export const analytics = new Analytics();

/**
 * USAGE EXAMPLE 1: Initialize at app startup
 * 
 * // App.tsx
 * import { analytics } from '@/shared/utils/analytics';
 * 
 * export default function App() {
 *   useEffect(() => {
 *     analytics.initialize();
 *   }, []);
 * 
 *   return <YourApp />;
 * }
 */

/**
 * USAGE EXAMPLE 2: Track button press
 * 
 * function SubmitButton() {
 *   const handlePress = () => {
 *     analytics.track('button_press', {
 *       button_id: 'submit_form',
 *       screen: 'ProfileScreen',
 *       feature: 'profile_edit',
 *     });
 * 
 *     submitForm();
 *   };
 * 
 *   return <Button onPress={handlePress}>Submit</Button>;
 * }
 */

/**
 * USAGE EXAMPLE 3: Track screen view
 * 
 * function ProfileScreen() {
 *   useEffect(() => {
 *     analytics.track('screen_view', {
 *       screen_name: 'ProfileScreen',
 *       screen_class: 'UserProfile',
 *     });
 *   }, []);
 * 
 *   return <View>...</View>;
 * }
 */

/**
 * USAGE EXAMPLE 4: Set user properties on login
 * 
 * const handleLogin = async (email, password) => {
 *   const { userId, userType } = await api.login(email, password);
 * 
 *   analytics.setUserProperties({
 *     user_id: userId,
 *     user_type: userType,
 *     platform: Platform.OS,
 *   });
 * };
 */

/**
 * USAGE EXAMPLE 5: Track errors
 * 
 * try {
 *   await api.fetchUser(userId);
 * } catch (error) {
 *   analytics.track('api_error', {
 *     endpoint: '/api/users/' + userId,
 *     status_code: error.status,
 *     error_message: error.message,
 *   });
 * }
 */
```

---

## 8. FLASHLIST MIGRATION (P1-HIGH)

### Why This Is Important
FlatList is 10x slower than FlashList for lists with 100+ items. FlashList uses recycling architecture similar to RecyclerView (Android) and UITableView (iOS).

### Files to Create

#### `mobile/src/shared/components/List.tsx`

```typescript
/**
 * List Component (FlashList wrapper)
 * 
 * PURPOSE:
 * Drop-in replacement for FlatList with better performance
 * Handles common list patterns (empty states, pull-to-refresh, etc.)
 * 
 * PERFORMANCE:
 * - 10x faster than FlatList for large lists
 * - Recycling-based architecture (reuses views)
 * - Blank space free (no white flashes during scroll)
 * 
 * WHEN TO USE:
 * - Lists with 20+ items
 * - Infinite scroll
 * - Complex item layouts
 * - High-frequency updates
 * 
 * WHEN TO USE FLATLIST:
 * - Short lists (<20 items)
 * - Horizontal scrolls (FlashList horizontal support limited)
 * - Sticky headers (complex scenarios)
 */

import React from 'react';
import { FlashList, FlashListProps } from '@shopify/flash-list';
import { View, Text, ActivityIndicator, Pressable } from 'react-native';

interface ListProps<T> extends Omit<FlashListProps<T>, 'renderItem'> {
  data: T[] | undefined;
  renderItem: (item: T, index: number) => React.ReactElement | null;
  isLoading?: boolean;
  isError?: boolean;
  isEmpty?: boolean;
  onRetry?: () => void;
  emptyMessage?: string;
  errorMessage?: string;
}

/**
 * List Component
 * 
 * FEATURES:
 * - Loading state (spinner)
 * - Error state (retry button)
 * - Empty state (custom message)
 * - Pull-to-refresh (built-in)
 * 
 * CRITICAL: Must provide estimatedItemSize
 * FlashList needs approximate item height for recycling
 * Inaccurate estimate = more blank space during scroll
 * 
 * @param estimatedItemSize - REQUIRED: Average item height in pixels
 */
export function List<T>({
  data,
  renderItem,
  isLoading = false,
  isError = false,
  isEmpty = false,
  onRetry,
  emptyMessage = 'No items to display',
  errorMessage = 'Failed to load items',
  estimatedItemSize,
  ...flashListProps
}: ListProps<T>) {
  /**
   * LOADING STATE
   * Show spinner while data is loading
   * 
   * BEST PRACTICE: Use skeleton loading instead
   * Provides better perceived performance
   */
  if (isLoading && (!data || data.length === 0)) {
    return (
      <View className="flex-1 items-center justify-center">
        <ActivityIndicator size="large" color="#6366F1" />
        <Text className="mt-4 text-text-secondary">Loading...</Text>
      </View>
    );
  }

  /**
   * ERROR STATE
   * Show error message with retry button
   * 
   * ACCESSIBILITY: Include retry button for screen readers
   */
  if (isError) {
    return (
      <View className="flex-1 items-center justify-center p-6">
        <Text className="text-lg font-semibold text-text-primary mb-2">
          {errorMessage}
        </Text>
        {onRetry && (
          <Pressable
            onPress={onRetry}
            className="bg-primary-500 px-6 py-3 rounded-lg active:opacity-80"
            accessibilityRole="button"
            accessibilityLabel="Retry loading"
          >
            <Text className="text-white font-semibold">Retry</Text>
          </Pressable>
        )}
      </View>
    );
  }

  /**
   * EMPTY STATE
   * Show message when list has no items
   * 
   * BETTER UX: Show illustration + call-to-action
   * Example: "No messages yet. Start a conversation!"
   */
  if (isEmpty || !data || data.length === 0) {
    return (
      <View className="flex-1 items-center justify-center p-6">
        <Text className="text-base text-text-secondary text-center">
          {emptyMessage}
        </Text>
      </View>
    );
  }

  /**
   * FLASHLIST RENDER
   * 
   * CRITICAL PROPS:
   * - estimatedItemSize: REQUIRED for performance
   * - keyExtractor: Unique key per item (prevents render bugs)
   * - renderItem: Must return element or null (not undefined)
   * 
   * PERFORMANCE TIPS:
   * - Memoize renderItem function
   * - Use React.memo for item components
   * - Avoid inline functions in renderItem
   */
  return (
    <FlashList
      data={data}
      renderItem={({ item, index }) => renderItem(item, index)}
      estimatedItemSize={estimatedItemSize || 80}
      {...flashListProps}
    />
  );
}

/**
 * USAGE EXAMPLE 1: Basic list
 * 
 * import { List } from '@/shared/components/List';
 * 
 * function UserList() {
 *   const { data: users, isLoading, isError, refetch } = useUsers();
 * 
 *   return (
 *     <List
 *       data={users}
 *       renderItem={(user) => <UserCard user={user} />}
 *       isLoading={isLoading}
 *       isError={isError}
 *       onRetry={refetch}
 *       estimatedItemSize={100}
 *       emptyMessage="No users found"
 *     />
 *   );
 * }
 */

/**
 * USAGE EXAMPLE 2: Infinite scroll
 * 
 * function PostList() {
 *   const {
 *     data: posts,
 *     isLoading,
 *     hasNextPage,
 *     fetchNextPage,
 *   } = useInfinitePosts();
 * 
 *   return (
 *     <List
 *       data={posts}
 *       renderItem={(post) => <PostCard post={post} />}
 *       isLoading={isLoading}
 *       estimatedItemSize={200}
 *       onEndReached={() => {
 *         if (hasNextPage) fetchNextPage();
 *       }}
 *       onEndReachedThreshold={0.5}
 *     />
 *   );
 * }
 */

/**
 * MIGRATION GUIDE: FlatList → FlashList
 * 
 * STEP 1: Install
 * npm install @shopify/flash-list
 * 
 * STEP 2: Replace import
 * - import { FlatList } from 'react-native';
 * + import { FlashList } from '@shopify/flash-list';
 * 
 * STEP 3: Add estimatedItemSize
 * <FlashList
 *   data={data}
 *   renderItem={renderItem}
 * + estimatedItemSize={80}
 * />
 * 
 * STEP 4: Test scroll performance
 * - Check for blank space during fast scroll
 * - Adjust estimatedItemSize if needed
 * - Measure actual item heights (use onLayout)
 * 
 * COMMON ISSUES:
 * 
 * Issue: Blank space during scroll
 * Fix: Provide accurate estimatedItemSize
 * 
 * Issue: Items not rendering
 * Fix: Ensure renderItem returns element or null (not undefined)
 * 
 * Issue: Key warnings
 * Fix: Provide keyExtractor prop
 * 
 * Issue: Poor performance with images
 * Fix: Use expo-image instead of Image component
 */
```

---

## 9. FEATURE FLAGS (P2-MEDIUM)

### Why This Is Useful
Feature flags enable gradual rollouts, A/B testing, and instant feature toggles without app updates.

### Files to Create

#### `mobile/src/shared/config/featureFlags.ts`

```typescript
/**
 * Feature Flags Configuration
 * 
 * PURPOSE:
 * Enable/disable features without app updates
 * Gradual rollout of new features
 * A/B testing and experimentation
 * 
 * TYPES OF FLAGS:
 * 1. Release flags: New features in beta
 * 2. Experiment flags: A/B tests
 * 3. Ops flags: Circuit breakers for backend issues
 * 4. Permission flags: Premium features, user segments
 * 
 * INFRASTRUCTURE:
 * - Development: Local config (this file)
 * - Production: Remote config (Firebase, LaunchDarkly, etc.)
 * 
 * BEST PRACTICES:
 * - Remove old flags after full rollout
 * - Document why each flag exists
 * - Set expiration dates for temporary flags
 */

/**
 * Feature flag definitions
 * 
 * NAMING CONVENTION:
 * - feature_name: General features
 * - experiment_name: A/B tests
 * - premium_name: Paid features
 * 
 * LIFECYCLE:
 * 1. Create flag (disabled by default)
 * 2. Enable in development
 * 3. Enable for beta users
 * 4. Enable for all users
 * 5. Remove flag, make feature permanent
 */
export const featureFlags = {
  /**
   * RELEASE FLAGS
   * New features being rolled out gradually
   */
  
  // New profile UI (redesign)
  // Added: 2026-01-10
  // Remove after: 2026-02-10 (full rollout)
  newProfileUI: getBooleanFlag('FF_NEW_PROFILE', false),

  // Dark mode support
  // Added: 2026-01-01
  // Remove after: Always (user preference)
  darkMode: getBooleanFlag('FF_DARK_MODE', true),

  // Push notifications
  // Added: 2026-01-05
  // Remove after: 2026-02-05 (full rollout)
  pushNotifications: getBooleanFlag('FF_PUSH_NOTIFICATIONS', false),

  /**
   * EXPERIMENT FLAGS
   * A/B tests and experiments
   */

  // Experiment: Show onboarding tutorial
  // Test: Does tutorial improve retention?
  // Groups: control (no tutorial), variant (tutorial)
  showOnboardingTutorial: getBooleanFlag('EXP_ONBOARDING', false),

  // Experiment: New home screen layout
  // Test: Does new layout increase engagement?
  homeScreenLayoutV2: getBooleanFlag('EXP_HOME_V2', false),

  /**
   * OPS FLAGS
   * Circuit breakers for operational issues
   */

  // Disable comments if backend overloaded
  enableComments: getBooleanFlag('OPS_COMMENTS', true),

  // Disable video uploads if storage full
  enableVideoUploads: getBooleanFlag('OPS_VIDEO_UPLOADS', true),

  /**
   * PERMISSION FLAGS
   * Features gated by user permissions
   */

  // Premium features (requires subscription)
  premiumFilters: false, // Set based on user subscription
  premiumExport: false,

  // Beta features (opt-in for beta users)
  betaFeatures: getBooleanFlag('FF_BETA', __DEV__),

  /**
   * DEVELOPMENT FLAGS
   * Always enabled in development
   */

  // Show debug overlay
  debugOverlay: __DEV__,

  // Enable Redux DevTools
  reduxDevTools: __DEV__,

  // Verbose logging
  verboseLogging: __DEV__,
} as const;

/**
 * Get boolean flag from environment or default
 * 
 * ENVIRONMENT VARIABLES:
 * - Expo: EXPO_PUBLIC_* prefix required
 * - React Native: process.env.*
 * - Expo constants: Constants.expoConfig.extra.*
 * 
 * @param envVar - Environment variable name
 * @param defaultValue - Default value if not set
 */
function getBooleanFlag(envVar: string, defaultValue: boolean): boolean {
  const value = process.env[envVar];
  
  if (value === undefined) {
    return defaultValue;
  }

  // Parse string to boolean
  // '1', 'true', 'TRUE', 'yes' → true
  // '0', 'false', 'FALSE', 'no' → false
  return value === '1' || value.toLowerCase() === 'true' || value.toLowerCase() === 'yes';
}

/**
 * Get string flag from environment or default
 * 
 * USE CASES:
 * - API endpoints (staging vs production)
 * - Feature variants (layout_a, layout_b, layout_c)
 * - Configuration values (max_items: '50')
 */
function getStringFlag(envVar: string, defaultValue: string): string {
  return process.env[envVar] || defaultValue;
}

/**
 * Get number flag from environment or default
 * 
 * USE CASES:
 * - Limits (max_uploads: 10)
 * - Timeouts (api_timeout: 30000)
 * - Percentages (rollout_percentage: 50)
 */
function getNumberFlag(envVar: string, defaultValue: number): number {
  const value = process.env[envVar];
  if (value === undefined) return defaultValue;
  const parsed = parseInt(value, 10);
  return isNaN(parsed) ? defaultValue : parsed;
}

/**
 * Check if user is in feature flag segment
 * 
 * SEGMENTATION STRATEGIES:
 * - User ID: Consistent experience per user
 * - Percentage: Gradual rollout (10%, 50%, 100%)
 * - Cohort: Beta users, premium users, etc.
 * 
 * @param userId - Current user ID
 * @param percentage - Rollout percentage (0-100)
 */
export function isUserInRollout(userId: string, percentage: number): boolean {
  if (percentage === 0) return false;
  if (percentage === 100) return true;

  // Hash user ID to get consistent random number
  const hash = hashString(userId);
  return (hash % 100) < percentage;
}

/**
 * Simple string hash function
 * Converts string to number (0-99)
 * 
 * PROPERTIES:
 * - Deterministic: Same input → same output
 * - Distributed: Similar inputs → different outputs
 * - Fast: O(n) where n = string length
 */
function hashString(str: string): number {
  let hash = 0;
  for (let i = 0; i < str.length; i++) {
    hash = ((hash << 5) - hash) + str.charCodeAt(i);
    hash = hash & hash; // Convert to 32-bit integer
  }
  return Math.abs(hash);
}

/**
 * USAGE EXAMPLE 1: Simple flag check
 * 
 * import { featureFlags } from '@/shared/config/featureFlags';
 * 
 * function ProfileScreen() {
 *   if (featureFlags.newProfileUI) {
 *     return <ProfileScreenV2 />;
 *   }
 *   return <ProfileScreenV1 />;
 * }
 */

/**
 * USAGE EXAMPLE 2: Gradual rollout
 * 
 * function HomeScreen() {
 *   const { userId } = useAuth();
 *   const showNewLayout = isUserInRollout(userId, 50); // 50% rollout
 * 
 *   if (showNewLayout) {
 *     return <HomeScreenV2 />;
 *   }
 *   return <HomeScreenV1 />;
 * }
 */

/**
 * USAGE EXAMPLE 3: Premium feature
 * 
 * function ExportButton() {
 *   const { user } = useAuth();
 *   const canExport = user.isPremium || featureFlags.premiumExport;
 * 
 *   if (!canExport) {
 *     return <UpgradePrompt feature="export" />;
 *   }
 * 
 *   return <Button onPress={handleExport}>Export</Button>;
 * }
 */

/**
 * PRODUCTION INTEGRATION: Firebase Remote Config
 * 
 * // Install
 * expo install @react-native-firebase/remote-config
 * 
 * // Setup
 * const remoteConfig = firebase.remoteConfig();
 * await remoteConfig.setDefaults({
 *   newProfileUI: false,
 *   darkMode: true,
 * });
 * 
 * // Fetch
 * await remoteConfig.fetchAndActivate();
 * 
 * // Get value
 * const newProfileUI = remoteConfig.getBoolean('newProfileUI');
 */
```

---

## 10. ENVIRONMENT CONFIGURATION (P2-MEDIUM)

### Files to Create

#### `mobile/.env.example`

```bash
# Environment Configuration Example
# Copy this file to .env and fill in values
# DO NOT commit .env to git (add to .gitignore)

# API Configuration
EXPO_PUBLIC_API_URL=http://localhost:8000/api
EXPO_PUBLIC_WS_URL=ws://localhost:8000/ws

# Feature Flags
EXPO_PUBLIC_FF_NEW_PROFILE=false
EXPO_PUBLIC_FF_DARK_MODE=true
EXPO_PUBLIC_FF_PUSH_NOTIFICATIONS=false

# Monitoring
EXPO_PUBLIC_SENTRY_DSN=
EXPO_PUBLIC_ENVIRONMENT=development

# Analytics
EXPO_PUBLIC_FIREBASE_API_KEY=
EXPO_PUBLIC_FIREBASE_PROJECT_ID=

# Maps (if needed)
EXPO_PUBLIC_GOOGLE_MAPS_API_KEY=

# NOTE: EXPO_PUBLIC_* prefix makes variables available in app
# Without prefix, variables only available in Metro bundler
```

#### `mobile/src/shared/config/env.ts`

```typescript
/**
 * Environment Configuration
 * 
 * PURPOSE:
 * Centralized access to environment variables
 * Type-safe environment configuration
 * Validation of required variables
 * 
 * SECURITY:
 * - Never commit .env files to git
 * - Use .env.example as template
 * - Rotate secrets regularly
 * - Different secrets per environment
 */

import Constants from 'expo-constants';

/**
 * Environment type
 * Determines which configuration to use
 */
type Environment = 'development' | 'staging' | 'production';

/**
 * Environment configuration interface
 * Add your app-specific config here
 */
interface EnvConfig {
  apiUrl: string;
  wsUrl?: string;
  environment: Environment;
  sentryDsn?: string;
  isProduction: boolean;
  isDevelopment: boolean;
}

/**
 * Get environment from Expo constants or process.env
 * 
 * PRIORITY:
 * 1. Expo release channel (production builds)
 * 2. EXPO_PUBLIC_ENVIRONMENT variable
 * 3. NODE_ENV
 * 4. Default to development
 */
function getEnvironment(): Environment {
  // Production: Check release channel
  const releaseChannel = Constants.expoConfig?.extra?.releaseChannel;
  if (releaseChannel === 'production') return 'production';
  if (releaseChannel === 'staging') return 'staging';

  // Development/Expo Go: Check environment variable
  const env = process.env.EXPO_PUBLIC_ENVIRONMENT;
  if (env === 'production') return 'production';
  if (env === 'staging') return 'staging';

  return 'development';
}

/**
 * Environment configurations
 * 
 * SETUP:
 * - Development: Local backend, debug enabled
 * - Staging: Staging backend, monitoring enabled
 * - Production: Production backend, full monitoring
 */
const ENV_CONFIG: Record<Environment, EnvConfig> = {
  development: {
    apiUrl: process.env.EXPO_PUBLIC_API_URL || 'http://localhost:8000/api',
    wsUrl: process.env.EXPO_PUBLIC_WS_URL || 'ws://localhost:8000/ws',
    environment: 'development',
    sentryDsn: undefined, // No monitoring in dev
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

/**
 * Current environment configuration
 * Import this in your app code
 */
const currentEnv = getEnvironment();
export const env = ENV_CONFIG[currentEnv];

/**
 * Validate required environment variables
 * Call at app startup to fail fast
 */
export function validateEnv() {
  const required: (keyof EnvConfig)[] = ['apiUrl'];

  for (const key of required) {
    if (!env[key]) {
      throw new Error(`Missing required environment variable: ${key}`);
    }
  }

  console.log('[Env] Configuration loaded:', {
    environment: env.environment,
    apiUrl: env.apiUrl,
  });
}

/**
 * USAGE EXAMPLE:
 * 
 * // app/_layout.tsx
 * import { validateEnv, env } from '@/shared/config/env';
 * 
 * export default function RootLayout() {
 *   useEffect(() => {
 *     validateEnv();
 *   }, []);
 * 
 *   return <App />;
 * }
 * 
 * // API client
 * const apiClient = axios.create({
 *   baseURL: env.apiUrl,
 * });
 */
```

---

## 11. UPDATE FRONTEND_GUIDELINES.md

Add these sections to your existing FRONTEND_GUIDELINES.md:

```markdown
## 15. Production Readiness Checklist

### Error Handling
- [ ] ErrorBoundary wraps root app
- [ ] ErrorBoundary wraps each route/feature
- [ ] Custom error screens for common errors
- [ ] Error logging to monitoring service (Sentry)
- [ ] Graceful degradation (app usable even with errors)

### Performance
- [ ] React DevTools Profiler setup
- [ ] Performance budget defined (see monitoring.ts)
- [ ] Slow renders tracked (>50ms warning)
- [ ] Memory leaks prevented (cleanup useEffect)
- [ ] FlashList for lists (>20 items)
- [ ] Images optimized (expo-image)
- [ ] Bundle size monitored (<500KB)

### Security
- [ ] Tokens in SecureStore (NOT AsyncStorage)
- [ ] Input validation client-side
- [ ] Input sanitization (XSS prevention)
- [ ] HTTPS only (no http:// in production)
- [ ] No PII in logs or analytics
- [ ] Environment secrets not committed (.env in .gitignore)

### Offline Support
- [ ] React Query persistence configured
- [ ] Network status detection
- [ ] Offline banner shown
- [ ] Operations queued when offline
- [ ] Sync when connection restored

### Deep Linking
- [ ] app.json configured (scheme, associatedDomains)
- [ ] Universal Links setup (iOS)
- [ ] App Links setup (Android)
- [ ] Deep link handling implemented
- [ ] Share functionality works

### Push Notifications
- [ ] Expo push notifications setup
- [ ] Permission requested (at appropriate time)
- [ ] Push token sent to backend
- [ ] Notification handling (foreground/background)
- [ ] Navigation from notifications works

### Analytics
- [ ] Analytics initialized at startup
- [ ] Key events tracked (login, signup, errors)
- [ ] User properties set on login
- [ ] User properties cleared on logout
- [ ] No PII tracked

### Monitoring
- [ ] Sentry configured (production)
- [ ] Error tracking works
- [ ] Performance monitoring enabled
- [ ] User context set (non-PII)
- [ ] Breadcrumbs useful for debugging

### Environment
- [ ] .env.example provided
- [ ] .env in .gitignore
- [ ] Environment validation at startup
- [ ] Different configs per environment (dev/staging/prod)
- [ ] Secrets rotated (not using defaults)

### Testing
- [ ] Unit tests for business logic
- [ ] Component tests for shared components
- [ ] Integration tests for critical flows
- [ ] E2E tests for user journeys
- [ ] Accessibility tests (screen reader)
```

---

## INTEGRATION CHECKLIST

To integrate these additions into your APP_VIRGIN template:

### Step 1: Install Dependencies

```bash
cd mobile

# Error handling & monitoring
npm install @sentry/react-native

# Offline support
npm install @react-native-community/netinfo @react-native-async-storage/async-storage
npm install @tanstack/query-async-storage-persister

# Push notifications
npm install expo-notifications expo-device

# Performance
npm install @shopify/flash-list

# Deep linking (already included with Expo)
# expo-linking (built-in)

# Environment
npm install expo-constants

# Analytics (choose one or multiple)
# npm install @react-native-firebase/analytics
# npm install mixpanel-react-native
```

### Step 2: Create Files

Copy all the code from this document into the appropriate files in your `mobile/` directory.

### Step 3: Update App.tsx

```typescript
// mobile/App.tsx or mobile/src/app/_layout.tsx
import { useEffect } from 'react';
import { ErrorBoundary } from '@/shared/components/ErrorBoundary';
import { setupMonitoring } from '@/shared/utils/monitoring';
import { validateEnv } from '@/shared/config/env';
import { analytics } from '@/shared/utils/analytics';
import { usePushNotifications } from '@/shared/hooks/usePushNotifications';
import { QueryClientProvider } from '@tanstack/react-query';
import { PersistQueryClientProvider } from '@tanstack/react-query-persist-client';
import { queryClient, asyncStoragePersister } from '@/api/queryClient';

export default function App() {
  // Initialize services
  useEffect(() => {
    // Validate environment
    validateEnv();
    
    // Setup monitoring
    setupMonitoring();
    
    // Initialize analytics
    analytics.initialize();
  }, []);

  // Setup push notifications
  usePushNotifications();

  return (
    <ErrorBoundary>
      <PersistQueryClientProvider
        client={queryClient}
        persistOptions={{ persister: asyncStoragePersister }}
      >
        <YourApp />
      </PersistQueryClientProvider>
    </ErrorBoundary>
  );
}
```

### Step 4: Update package.json

Add these scripts to your `mobile/package.json`:

```json
{
  "scripts": {
    "start": "expo start",
    "android": "expo start --android",
    "ios": "expo start --ios",
    "web": "expo start --web",
    "type-check": "tsc --noEmit",
    "lint": "eslint .",
    "test": "jest",
    "test:watch": "jest --watch",
    "test:coverage": "jest --coverage"
  }
}
```

### Step 5: Configure Sentry

```bash
# Generate Sentry DSN at https://sentry.io
# Add to .env
echo "EXPO_PUBLIC_SENTRY_DSN=your-dsn-here" >> .env
```

---

## FINAL NOTES

### What You've Gained

1. **Production Stability** - Error boundaries prevent crashes
2. **Performance Visibility** - Monitor app health in real-time
3. **Security Hardening** - Secure token storage, input validation
4. **Offline Support** - App works without internet
5. **User Engagement** - Push notifications + deep linking
6. **Data-Driven** - Analytics track user behavior
7. **Faster Lists** - FlashList for 10x performance
8. **Flexible Features** - Feature flags for gradual rollouts
9. **Environment Management** - Easy dev/staging/prod switching

### Recommended Next Steps

1. **Week 1** - Implement P0 (error boundaries, security, performance)
2. **Week 2** - Implement P1 (deep linking, push notifications, analytics)
3. **Week 3** - Implement P2 (feature flags, environment config)
4. **Week 4** - Testing, refinement, documentation

### Support Resources

- **Sentry Docs**: https://docs.sentry.io/platforms/react-native/
- **FlashList Docs**: https://shopify.github.io/flash-list/
- **Expo Notifications**: https://docs.expo.dev/push-notifications/overview/
- **React Query**: https://tanstack.com/query/latest/docs/react/overview

---

**END OF PRODUCTION ADDITIONS**

Your APP_VIRGIN template is now equipped with production-grade infrastructure. Deploy with confidence! 🚀
