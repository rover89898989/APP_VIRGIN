# Frontend Design Guidelines Policy
## SOTA 2026 React Native + Rust Monorepo

> **Version:** 1.0.0  
> **Last Updated:** January 2026  
> **Scope:** Mobile frontend architecture, design system, and AI-assisted development workflow

---

## Table of Contents

1. [Architecture Overview](#1-architecture-overview)
2. [AI Context Injection](#2-ai-context-injection)
3. [File Structure & Naming](#3-file-structure--naming)
4. [Design System](#4-design-system)
5. [Component Architecture](#5-component-architecture)
6. [State Management](#6-state-management)
7. [UI State Handling](#7-ui-state-handling)
8. [Forms & Validation](#8-forms--validation)
9. [Navigation](#9-navigation)
10. [Accessibility](#10-accessibility)
11. [Animations](#11-animations)
12. [Testing](#12-testing)
13. [Development Workflow](#13-development-workflow)
14. [Checklist & Quick Reference](#14-checklist--quick-reference)

---

## 1. Architecture Overview

### Tech Stack

| Layer | Technology | Notes |
|-------|------------|-------|
| **Framework** | React Native (Expo) | Managed workflow |
| **Language** | TypeScript | Strict mode enabled |
| **Styling** | NativeWind (Tailwind CSS) | Primary styling solution |
| **Server State** | React Query (`@tanstack/react-query`) | Caching, sync, mutations |
| **Local State** | React Hooks (`useState`, `useReducer`) | UI-only state |
| **Forms** | React Hook Form + Zod | Validation & type safety |
| **Navigation** | Expo Router | File-based routing |
| **Animations** | React Native Reanimated 3 | 60fps animations |
| **Backend** | Rust (Axum) | Types auto-generated |

### Core Principles

1. **Feature-First Architecture** — Code organized by domain, not by type
2. **Type Safety End-to-End** — Rust types → TypeScript types (auto-generated)
3. **Mobile-First UX** — Touch targets, safe areas, keyboard handling
4. **State Machine Thinking** — Every screen handles all 5+ UI states
5. **Design System Compliance** — No hardcoded colors, spacing, or typography

---

## 2. AI Context Injection

### Purpose

Prevent AI assistants from generating incompatible code (HTML/CSS, generic React). Force compliance with React Native primitives and your monorepo structure.

### The SOTA Context Prompt

Save as `.windsurfrules` in project root OR paste before any design/code generation session:

```markdown
You are an expert Mobile Architect working in a SOTA 2026 Monorepo.

## Tech Stack
- **Frontend:** React Native (Expo), TypeScript, React Query (@tanstack/react-query)
- **Styling:** NativeWind (Tailwind classes on RN primitives)
- **Forms:** React Hook Form + Zod
- **Navigation:** Expo Router (file-based)
- **Animations:** React Native Reanimated 3
- **Backend:** Rust (Axum). Types are auto-generated.

## Hard Constraints (NEVER violate)
1. **NEVER** use HTML tags (`div`, `span`, `h1`, `button`, `input`).
   - USE: `View`, `Text`, `Pressable`, `TextInput`, `SafeAreaView`, `ScrollView`
2. **NEVER** mock types manually. Import from `@/api/types/`.
3. **NEVER** hardcode colors or spacing. Use `theme.ts` constants.
4. **NEVER** use inline styles for reusable values. Use NativeWind classes.
5. **NEVER** ignore loading/error states. Handle ALL UI states explicitly.

## File Structure
- Shared atoms/molecules: `mobile/src/shared/components/`
- Feature screens: `mobile/src/features/[feature]/screens/`
- Feature components: `mobile/src/features/[feature]/components/`
- API hooks: `mobile/src/api/client.ts`
- Types: `mobile/src/api/types/`
- Theme: `mobile/src/shared/theme.ts`
- Navigation: `mobile/src/app/` (Expo Router)

## Data Fetching Pattern
- Reads: Custom hooks using `useQuery` (e.g., `useUser`, `useProducts`)
- Writes: Custom hooks using `useMutation` (e.g., `useUpdateUser`)
- Always handle: `isLoading`, `isError`, `isSuccess`, `isFetching` (background)

## Component Requirements
- Minimum touch target: 48x48px
- Always wrap screens in `SafeAreaView`
- Always handle keyboard with `KeyboardAvoidingView`
- Support dark mode via theme tokens
- Include accessibility props (`accessibilityLabel`, `accessibilityRole`)
```

---

## 3. File Structure & Naming

### Directory Structure

```
mobile/
├── src/
│   ├── app/                          # Expo Router (file-based navigation)
│   │   ├── (auth)/                   # Auth group (login, register)
│   │   ├── (tabs)/                   # Main tab navigation
│   │   ├── _layout.tsx               # Root layout
│   │   └── index.tsx                 # Entry redirect
│   │
│   ├── api/
│   │   ├── client.ts                 # React Query hooks (useUser, etc.)
│   │   ├── fetcher.ts                # Base fetch wrapper
│   │   └── types/                    # Auto-generated from Rust
│   │       ├── user.ts
│   │       └── index.ts
│   │
│   ├── features/
│   │   ├── auth/
│   │   │   ├── screens/
│   │   │   │   ├── LoginScreen.tsx
│   │   │   │   └── RegisterScreen.tsx
│   │   │   ├── components/
│   │   │   │   └── AuthForm.tsx
│   │   │   └── hooks/
│   │   │       └── useAuthFlow.ts
│   │   │
│   │   ├── users/
│   │   │   ├── screens/
│   │   │   │   ├── ProfileScreen.tsx
│   │   │   │   └── SettingsScreen.tsx
│   │   │   └── components/
│   │   │       └── AvatarUploader.tsx
│   │   │
│   │   └── [feature]/                # Pattern for new features
│   │
│   ├── shared/
│   │   ├── components/               # Atoms & Molecules
│   │   │   ├── Button.tsx
│   │   │   ├── TextInput.tsx
│   │   │   ├── Card.tsx
│   │   │   ├── Skeleton.tsx
│   │   │   ├── ErrorState.tsx
│   │   │   └── EmptyState.tsx
│   │   │
│   │   ├── hooks/
│   │   │   ├── useKeyboard.ts
│   │   │   └── useTheme.ts
│   │   │
│   │   ├── theme.ts                  # Design tokens
│   │   ├── theme.dark.ts             # Dark mode overrides
│   │   └── utils/
│   │       └── accessibility.ts
│   │
│   └── types/
│       └── navigation.ts             # Navigation type definitions
│
├── assets/
├── app.json
├── tailwind.config.js                # NativeWind config
└── package.json
```

### Naming Conventions

| Type | Convention | Example |
|------|------------|---------|
| **Screens** | PascalCase + `Screen` suffix | `ProfileScreen.tsx` |
| **Components** | PascalCase | `Button.tsx`, `UserCard.tsx` |
| **Hooks** | camelCase + `use` prefix | `useUser.ts`, `useAuthFlow.ts` |
| **Types** | PascalCase | `User`, `UpdateUserRequest` |
| **Constants** | SCREAMING_SNAKE_CASE | `API_BASE_URL` |
| **Theme tokens** | camelCase nested objects | `colors.primary.500` |

---

## 4. Design System

### Theme Structure (`mobile/src/shared/theme.ts`)

```typescript
// mobile/src/shared/theme.ts

export const colors = {
  // Brand
  primary: {
    50: '#EEF2FF',
    100: '#E0E7FF',
    200: '#C7D2FE',
    300: '#A5B4FC',
    400: '#818CF8',
    500: '#6366F1',  // Main primary
    600: '#4F46E5',
    700: '#4338CA',
    800: '#3730A3',
    900: '#312E81',
  },
  
  // Semantic
  background: {
    primary: '#FFFFFF',
    secondary: '#F9FAFB',
    tertiary: '#F3F4F6',
  },
  
  surface: {
    primary: '#FFFFFF',
    elevated: '#FFFFFF',
  },
  
  text: {
    primary: '#111827',
    secondary: '#6B7280',
    tertiary: '#9CA3AF',
    inverse: '#FFFFFF',
  },
  
  border: {
    primary: '#E5E7EB',
    secondary: '#D1D5DB',
    focus: '#6366F1',
  },
  
  status: {
    success: '#10B981',
    warning: '#F59E0B',
    error: '#EF4444',
    info: '#3B82F6',
  },
} as const;

export const darkColors = {
  background: {
    primary: '#0F172A',
    secondary: '#1E293B',
    tertiary: '#334155',
  },
  surface: {
    primary: '#1E293B',
    elevated: '#334155',
  },
  text: {
    primary: '#F9FAFB',
    secondary: '#9CA3AF',
    tertiary: '#6B7280',
    inverse: '#111827',
  },
  border: {
    primary: '#334155',
    secondary: '#475569',
    focus: '#818CF8',
  },
} as const;

export const typography = {
  // Font families
  fontFamily: {
    regular: 'Inter-Regular',
    medium: 'Inter-Medium',
    semibold: 'Inter-SemiBold',
    bold: 'Inter-Bold',
  },
  
  // Type scale (size / lineHeight)
  h1: { fontSize: 32, lineHeight: 40, fontFamily: 'Inter-Bold' },
  h2: { fontSize: 24, lineHeight: 32, fontFamily: 'Inter-SemiBold' },
  h3: { fontSize: 20, lineHeight: 28, fontFamily: 'Inter-SemiBold' },
  h4: { fontSize: 18, lineHeight: 24, fontFamily: 'Inter-Medium' },
  body: { fontSize: 16, lineHeight: 24, fontFamily: 'Inter-Regular' },
  bodySmall: { fontSize: 14, lineHeight: 20, fontFamily: 'Inter-Regular' },
  caption: { fontSize: 12, lineHeight: 16, fontFamily: 'Inter-Regular' },
  overline: { fontSize: 10, lineHeight: 14, fontFamily: 'Inter-Medium' },
} as const;

export const spacing = {
  0: 0,
  1: 4,    // 4px
  2: 8,    // 8px
  3: 12,   // 12px
  4: 16,   // 16px - Base unit
  5: 20,   // 20px
  6: 24,   // 24px
  8: 32,   // 32px
  10: 40,  // 40px
  12: 48,  // 48px - Min touch target
  16: 64,  // 64px
  20: 80,  // 80px
  24: 96,  // 96px
} as const;

export const borderRadius = {
  none: 0,
  sm: 4,
  md: 8,
  lg: 12,
  xl: 16,
  full: 9999,
} as const;

export const shadows = {
  none: {},
  sm: {
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 1 },
    shadowOpacity: 0.05,
    shadowRadius: 2,
    elevation: 1,
  },
  md: {
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 3,
  },
  lg: {
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 4 },
    shadowOpacity: 0.15,
    shadowRadius: 8,
    elevation: 6,
  },
} as const;

export const animation = {
  duration: {
    instant: 100,
    fast: 200,
    normal: 300,
    slow: 500,
  },
  easing: {
    default: 'ease-out',
    spring: { damping: 15, stiffness: 150 },
  },
} as const;

// Component-specific tokens
export const components = {
  button: {
    height: {
      sm: 36,
      md: 44,
      lg: 52,
    },
    minTouchTarget: 48,
  },
  input: {
    height: 48,
    paddingHorizontal: 16,
  },
  card: {
    padding: 16,
    borderRadius: 12,
  },
} as const;
```

### NativeWind Configuration (`tailwind.config.js`)

```javascript
// tailwind.config.js
const { colors, spacing, borderRadius } = require('./src/shared/theme');

module.exports = {
  content: ['./src/**/*.{js,jsx,ts,tsx}'],
  presets: [require('nativewind/preset')],
  theme: {
    extend: {
      colors: {
        primary: colors.primary,
        background: colors.background,
        surface: colors.surface,
        text: colors.text,
        border: colors.border,
        status: colors.status,
      },
      spacing: Object.fromEntries(
        Object.entries(spacing).map(([k, v]) => [k, `${v}px`])
      ),
      borderRadius: Object.fromEntries(
        Object.entries(borderRadius).map(([k, v]) => [k, `${v}px`])
      ),
    },
  },
  plugins: [],
};
```

---

## 5. Component Architecture

### Atomic Design Hierarchy

```
Atoms       → Button, TextInput, Text, Icon, Avatar, Skeleton
Molecules   → Card, ListItem, SearchBar, FormField
Organisms   → Header, TabBar, UserCard, FormSection
Templates   → ScreenLayout, AuthLayout, TabLayout
Screens     → LoginScreen, ProfileScreen, SettingsScreen
```

### Shared Component Template

```typescript
// mobile/src/shared/components/Button.tsx

import React from 'react';
import {
  Pressable,
  Text,
  ActivityIndicator,
  type PressableProps,
} from 'react-native';
import Animated, {
  useAnimatedStyle,
  useSharedValue,
  withSpring,
} from 'react-native-reanimated';
import { colors, components, typography } from '../theme';

const AnimatedPressable = Animated.createAnimatedComponent(Pressable);

type ButtonVariant = 'primary' | 'secondary' | 'danger' | 'ghost';
type ButtonSize = 'sm' | 'md' | 'lg';

interface ButtonProps extends Omit<PressableProps, 'children'> {
  children: string;
  variant?: ButtonVariant;
  size?: ButtonSize;
  isLoading?: boolean;
  isDisabled?: boolean;
  leftIcon?: React.ReactNode;
  rightIcon?: React.ReactNode;
  fullWidth?: boolean;
}

export function Button({
  children,
  variant = 'primary',
  size = 'md',
  isLoading = false,
  isDisabled = false,
  leftIcon,
  rightIcon,
  fullWidth = false,
  ...props
}: ButtonProps) {
  const scale = useSharedValue(1);
  
  const animatedStyle = useAnimatedStyle(() => ({
    transform: [{ scale: scale.value }],
  }));
  
  const handlePressIn = () => {
    scale.value = withSpring(0.97);
  };
  
  const handlePressOut = () => {
    scale.value = withSpring(1);
  };
  
  const variantStyles = getVariantStyles(variant, isDisabled);
  const sizeStyles = getSizeStyles(size);
  
  return (
    <AnimatedPressable
      style={[
        {
          flexDirection: 'row',
          alignItems: 'center',
          justifyContent: 'center',
          borderRadius: 8,
          minHeight: components.button.minTouchTarget,
          ...variantStyles.container,
          ...sizeStyles.container,
          ...(fullWidth && { width: '100%' }),
        },
        animatedStyle,
      ]}
      onPressIn={handlePressIn}
      onPressOut={handlePressOut}
      disabled={isDisabled || isLoading}
      accessibilityRole="button"
      accessibilityState={{ disabled: isDisabled, busy: isLoading }}
      accessibilityLabel={children}
      {...props}
    >
      {isLoading ? (
        <ActivityIndicator color={variantStyles.text.color} />
      ) : (
        <>
          {leftIcon}
          <Text
            style={[
              typography.body,
              { fontFamily: typography.fontFamily.semibold },
              variantStyles.text,
              sizeStyles.text,
              leftIcon && { marginLeft: 8 },
              rightIcon && { marginRight: 8 },
            ]}
          >
            {children}
          </Text>
          {rightIcon}
        </>
      )}
    </AnimatedPressable>
  );
}

function getVariantStyles(variant: ButtonVariant, isDisabled: boolean) {
  const opacity = isDisabled ? 0.5 : 1;
  
  const styles = {
    primary: {
      container: { backgroundColor: colors.primary[500], opacity },
      text: { color: colors.text.inverse },
    },
    secondary: {
      container: {
        backgroundColor: 'transparent',
        borderWidth: 1,
        borderColor: colors.border.primary,
        opacity,
      },
      text: { color: colors.text.primary },
    },
    danger: {
      container: { backgroundColor: colors.status.error, opacity },
      text: { color: colors.text.inverse },
    },
    ghost: {
      container: { backgroundColor: 'transparent', opacity },
      text: { color: colors.primary[500] },
    },
  };
  
  return styles[variant];
}

function getSizeStyles(size: ButtonSize) {
  const styles = {
    sm: {
      container: {
        height: components.button.height.sm,
        paddingHorizontal: 12,
      },
      text: { fontSize: 14 },
    },
    md: {
      container: {
        height: components.button.height.md,
        paddingHorizontal: 16,
      },
      text: { fontSize: 16 },
    },
    lg: {
      container: {
        height: components.button.height.lg,
        paddingHorizontal: 24,
      },
      text: { fontSize: 18 },
    },
  };
  
  return styles[size];
}
```

---

## 6. State Management

### Server State (React Query)

```typescript
// mobile/src/api/client.ts

import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import type { User, UpdateUserRequest } from './types';
import { fetcher } from './fetcher';

// Query Keys Factory
export const queryKeys = {
  all: ['api'] as const,
  users: () => [...queryKeys.all, 'users'] as const,
  user: (id: string) => [...queryKeys.users(), id] as const,
  currentUser: () => [...queryKeys.users(), 'me'] as const,
};

// Queries
export function useUser() {
  return useQuery({
    queryKey: queryKeys.currentUser(),
    queryFn: () => fetcher<User>('/api/users/me'),
    staleTime: 5 * 60 * 1000, // 5 minutes
    gcTime: 30 * 60 * 1000,   // 30 minutes
  });
}

// Mutations
export function useUpdateUser() {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: (data: UpdateUserRequest) =>
      fetcher<User>('/api/users/me', {
        method: 'PATCH',
        body: JSON.stringify(data),
      }),
    onSuccess: (updatedUser) => {
      queryClient.setQueryData(queryKeys.currentUser(), updatedUser);
    },
  });
}

export function useLogout() {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: () => fetcher('/api/auth/logout', { method: 'POST' }),
    onSuccess: () => {
      queryClient.clear();
    },
  });
}
```

### Local State Patterns

```typescript
// UI State - useState
const [isModalVisible, setIsModalVisible] = useState(false);

// Complex UI State - useReducer
const [formState, dispatch] = useReducer(formReducer, initialState);

// Derived State - useMemo
const filteredItems = useMemo(
  () => items.filter(item => item.active),
  [items]
);
```

---

## 7. UI State Handling

### The 6 Essential UI States

Every screen/component that fetches data MUST handle all applicable states:

| State | React Query | Visual Treatment |
|-------|-------------|------------------|
| **Loading** | `isLoading && !data` | Skeleton placeholders |
| **Error** | `isError` | Error message + Retry button |
| **Empty** | `isSuccess && data.length === 0` | Friendly empty state + CTA |
| **Success** | `isSuccess && data` | Full content view |
| **Partial** | `isSuccess && !data.field` | Graceful degradation |
| **Refreshing** | `isFetching && data` | Subtle indicator (not blocking) |

### State Component Templates

```typescript
// mobile/src/shared/components/Skeleton.tsx
import { View } from 'react-native';
import Animated, {
  useAnimatedStyle,
  withRepeat,
  withTiming,
  useSharedValue,
} from 'react-native-reanimated';
import { useEffect } from 'react';
import { colors } from '../theme';

interface SkeletonProps {
  width: number | string;
  height: number;
  borderRadius?: number;
}

export function Skeleton({ width, height, borderRadius = 4 }: SkeletonProps) {
  const opacity = useSharedValue(0.3);
  
  useEffect(() => {
    opacity.value = withRepeat(
      withTiming(0.7, { duration: 800 }),
      -1,
      true
    );
  }, []);
  
  const animatedStyle = useAnimatedStyle(() => ({
    opacity: opacity.value,
  }));
  
  return (
    <Animated.View
      style={[
        {
          width,
          height,
          borderRadius,
          backgroundColor: colors.background.tertiary,
        },
        animatedStyle,
      ]}
    />
  );
}

// mobile/src/shared/components/ErrorState.tsx
import { View, Text } from 'react-native';
import { Button } from './Button';
import { colors, spacing, typography } from '../theme';

interface ErrorStateProps {
  title?: string;
  message?: string;
  onRetry?: () => void;
}

export function ErrorState({
  title = 'Something went wrong',
  message = 'Please try again later.',
  onRetry,
}: ErrorStateProps) {
  return (
    <View
      style={{
        flex: 1,
        alignItems: 'center',
        justifyContent: 'center',
        padding: spacing[6],
      }}
      accessibilityRole="alert"
    >
      <Text
        style={[typography.h3, { color: colors.text.primary, marginBottom: spacing[2] }]}
      >
        {title}
      </Text>
      <Text
        style={[
          typography.body,
          { color: colors.text.secondary, textAlign: 'center', marginBottom: spacing[6] },
        ]}
      >
        {message}
      </Text>
      {onRetry && (
        <Button onPress={onRetry} variant="primary">
          Try Again
        </Button>
      )}
    </View>
  );
}

// mobile/src/shared/components/EmptyState.tsx
import { View, Text } from 'react-native';
import { Button } from './Button';
import { colors, spacing, typography } from '../theme';

interface EmptyStateProps {
  icon?: React.ReactNode;
  title: string;
  message: string;
  actionLabel?: string;
  onAction?: () => void;
}

export function EmptyState({
  icon,
  title,
  message,
  actionLabel,
  onAction,
}: EmptyStateProps) {
  return (
    <View
      style={{
        flex: 1,
        alignItems: 'center',
        justifyContent: 'center',
        padding: spacing[6],
      }}
    >
      {icon && <View style={{ marginBottom: spacing[4] }}>{icon}</View>}
      <Text
        style={[typography.h3, { color: colors.text.primary, marginBottom: spacing[2] }]}
      >
        {title}
      </Text>
      <Text
        style={[
          typography.body,
          { color: colors.text.secondary, textAlign: 'center', marginBottom: spacing[6] },
        ]}
      >
        {message}
      </Text>
      {actionLabel && onAction && (
        <Button onPress={onAction} variant="primary">
          {actionLabel}
        </Button>
      )}
    </View>
  );
}
```

### Screen Implementation Pattern

```typescript
// mobile/src/features/users/screens/ProfileScreen.tsx

import React from 'react';
import { ScrollView, View, Text, RefreshControl } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import { useUser, useUpdateUser } from '@/api/client';
import { Button, TextInput, Skeleton, ErrorState } from '@/shared/components';
import { colors, spacing, typography } from '@/shared/theme';
import type { User } from '@/api/types';

export function ProfileScreen() {
  const { data: user, isLoading, isError, error, refetch, isFetching } = useUser();
  const updateUser = useUpdateUser();

  // Loading State
  if (isLoading) {
    return (
      <SafeAreaView style={{ flex: 1, backgroundColor: colors.background.primary }}>
        <ProfileSkeleton />
      </SafeAreaView>
    );
  }

  // Error State
  if (isError) {
    return (
      <SafeAreaView style={{ flex: 1, backgroundColor: colors.background.primary }}>
        <ErrorState
          title="Couldn't load profile"
          message={error?.message || 'Please check your connection and try again.'}
          onRetry={refetch}
        />
      </SafeAreaView>
    );
  }

  // Success State (with Partial handling)
  return (
    <SafeAreaView style={{ flex: 1, backgroundColor: colors.background.primary }}>
      <ScrollView
        contentContainerStyle={{ padding: spacing[4] }}
        refreshControl={
          <RefreshControl refreshing={isFetching} onRefresh={refetch} />
        }
      >
        <ProfileForm user={user!} onSubmit={updateUser.mutate} />
      </ScrollView>
    </SafeAreaView>
  );
}

function ProfileSkeleton() {
  return (
    <View style={{ padding: spacing[4], gap: spacing[4] }}>
      <View style={{ alignItems: 'center', marginBottom: spacing[4] }}>
        <Skeleton width={96} height={96} borderRadius={48} />
        <Skeleton width={150} height={24} style={{ marginTop: spacing[3] }} />
      </View>
      <Skeleton width="100%" height={48} borderRadius={8} />
      <Skeleton width="100%" height={48} borderRadius={8} />
      <Skeleton width="100%" height={48} borderRadius={8} />
    </View>
  );
}
```

---

## 8. Forms & Validation

### Form Architecture (React Hook Form + Zod)

```typescript
// mobile/src/features/users/components/ProfileForm.tsx

import React from 'react';
import { View, KeyboardAvoidingView, Platform } from 'react-native';
import { useForm, Controller } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { Button, TextInput, FormField } from '@/shared/components';
import { spacing } from '@/shared/theme';
import type { User, UpdateUserRequest } from '@/api/types';

// Validation Schema
const profileSchema = z.object({
  name: z
    .string()
    .min(2, 'Name must be at least 2 characters')
    .max(50, 'Name must be under 50 characters'),
  email: z
    .string()
    .email('Please enter a valid email'),
});

type ProfileFormData = z.infer<typeof profileSchema>;

interface ProfileFormProps {
  user: User;
  onSubmit: (data: UpdateUserRequest) => void;
  isSubmitting?: boolean;
}

export function ProfileForm({ user, onSubmit, isSubmitting }: ProfileFormProps) {
  const {
    control,
    handleSubmit,
    formState: { errors, isDirty },
  } = useForm<ProfileFormData>({
    resolver: zodResolver(profileSchema),
    defaultValues: {
      name: user.name ?? '',
      email: user.email,
    },
  });

  return (
    <KeyboardAvoidingView
      behavior={Platform.OS === 'ios' ? 'padding' : 'height'}
      style={{ flex: 1 }}
    >
      <View style={{ gap: spacing[4] }}>
        <Controller
          control={control}
          name="name"
          render={({ field: { onChange, onBlur, value } }) => (
            <FormField label="Full Name" error={errors.name?.message}>
              <TextInput
                value={value}
                onChangeText={onChange}
                onBlur={onBlur}
                placeholder="Enter your name"
                autoCapitalize="words"
                autoComplete="name"
                accessibilityLabel="Full name input"
              />
            </FormField>
          )}
        />

        <Controller
          control={control}
          name="email"
          render={({ field: { onChange, onBlur, value } }) => (
            <FormField label="Email" error={errors.email?.message}>
              <TextInput
                value={value}
                onChangeText={onChange}
                onBlur={onBlur}
                placeholder="Enter your email"
                keyboardType="email-address"
                autoCapitalize="none"
                autoComplete="email"
                accessibilityLabel="Email input"
              />
            </FormField>
          )}
        />

        <Button
          onPress={handleSubmit(onSubmit)}
          isLoading={isSubmitting}
          isDisabled={!isDirty}
          fullWidth
        >
          Save Changes
        </Button>
      </View>
    </KeyboardAvoidingView>
  );
}
```

---

## 9. Navigation

### Expo Router Structure

```typescript
// mobile/src/app/_layout.tsx
import { Stack } from 'expo-router';
import { QueryClientProvider } from '@tanstack/react-query';
import { SafeAreaProvider } from 'react-native-safe-area-context';
import { queryClient } from '@/api/queryClient';

export default function RootLayout() {
  return (
    <QueryClientProvider client={queryClient}>
      <SafeAreaProvider>
        <Stack screenOptions={{ headerShown: false }}>
          <Stack.Screen name="(auth)" />
          <Stack.Screen name="(tabs)" />
        </Stack>
      </SafeAreaProvider>
    </QueryClientProvider>
  );
}

// mobile/src/app/(tabs)/_layout.tsx
import { Tabs } from 'expo-router';
import { Home, User, Settings } from 'lucide-react-native';
import { colors } from '@/shared/theme';

export default function TabLayout() {
  return (
    <Tabs
      screenOptions={{
        tabBarActiveTintColor: colors.primary[500],
        tabBarInactiveTintColor: colors.text.tertiary,
        headerShown: false,
      }}
    >
      <Tabs.Screen
        name="index"
        options={{
          title: 'Home',
          tabBarIcon: ({ color, size }) => <Home color={color} size={size} />,
        }}
      />
      <Tabs.Screen
        name="profile"
        options={{
          title: 'Profile',
          tabBarIcon: ({ color, size }) => <User color={color} size={size} />,
        }}
      />
      <Tabs.Screen
        name="settings"
        options={{
          title: 'Settings',
          tabBarIcon: ({ color, size }) => <Settings color={color} size={size} />,
        }}
      />
    </Tabs>
  );
}
```

### Navigation Type Safety

```typescript
// mobile/src/types/navigation.ts
import type { NavigatorScreenParams } from '@react-navigation/native';

export type RootStackParamList = {
  '(auth)': NavigatorScreenParams<AuthStackParamList>;
  '(tabs)': NavigatorScreenParams<TabParamList>;
};

export type AuthStackParamList = {
  login: undefined;
  register: undefined;
  'forgot-password': { email?: string };
};

export type TabParamList = {
  index: undefined;
  profile: undefined;
  settings: undefined;
};
```

---

## 10. Accessibility

### Required Accessibility Props

| Component | Required Props |
|-----------|----------------|
| **Pressable/Button** | `accessibilityRole="button"`, `accessibilityLabel`, `accessibilityState` |
| **TextInput** | `accessibilityLabel`, `accessibilityHint` (if not obvious) |
| **Images** | `accessibilityLabel` (descriptive) or `accessibilityIgnoresInvertColors` |
| **Icons (decorative)** | `accessibilityElementsHidden={true}` |
| **Icons (functional)** | `accessibilityRole`, `accessibilityLabel` |
| **Lists** | `accessibilityRole="list"` on container |
| **Errors/Alerts** | `accessibilityRole="alert"`, `accessibilityLiveRegion="assertive"` |

### Accessibility Checklist Per Component

```typescript
// Example: Accessible Button
<Pressable
  accessibilityRole="button"
  accessibilityLabel="Submit form"
  accessibilityHint="Double tap to submit your changes"
  accessibilityState={{
    disabled: isDisabled,
    busy: isLoading,
    selected: isSelected,
  }}
  accessible={true}
>
  {/* content */}
</Pressable>
```

### Screen Reader Testing Commands

```bash
# iOS Simulator
# Enable: Settings > Accessibility > VoiceOver
# Or: Cmd + F5 in Simulator

# Android Emulator  
# Enable: Settings > Accessibility > TalkBack
```

---

## 11. Animations

### Animation Standards

| Animation Type | Duration | Easing |
|---------------|----------|--------|
| **Micro-interactions** | 100-200ms | ease-out |
| **Page transitions** | 250-350ms | ease-in-out |
| **Modal/sheet entry** | 300ms | spring |
| **Loading states** | 800ms loop | linear |

### Reanimated Patterns

```typescript
// Fade In on Mount
import Animated, { FadeIn, FadeOut } from 'react-native-reanimated';

<Animated.View entering={FadeIn.duration(300)} exiting={FadeOut.duration(200)}>
  {/* content */}
</Animated.View>

// Press Scale Animation
const scale = useSharedValue(1);

const animatedStyle = useAnimatedStyle(() => ({
  transform: [{ scale: scale.value }],
}));

const handlePressIn = () => {
  scale.value = withSpring(0.95, { damping: 15, stiffness: 150 });
};

const handlePressOut = () => {
  scale.value = withSpring(1, { damping: 15, stiffness: 150 });
};
```

---

## 12. Testing

### Testing Stack

| Type | Tool | Location |
|------|------|----------|
| **Unit** | Jest | `*.test.ts` |
| **Component** | React Native Testing Library | `*.test.tsx` |
| **Integration** | Detox | `e2e/` |
| **Snapshot** | Jest | `__snapshots__/` |

### Component Test Template

```typescript
// mobile/src/shared/components/__tests__/Button.test.tsx

import React from 'react';
import { render, fireEvent, screen } from '@testing-library/react-native';
import { Button } from '../Button';

describe('Button', () => {
  it('renders label correctly', () => {
    render(<Button>Click me</Button>);
    expect(screen.getByText('Click me')).toBeTruthy();
  });

  it('calls onPress when pressed', () => {
    const onPress = jest.fn();
    render(<Button onPress={onPress}>Click me</Button>);
    
    fireEvent.press(screen.getByRole('button'));
    expect(onPress).toHaveBeenCalledTimes(1);
  });

  it('shows loading indicator when isLoading', () => {
    render(<Button isLoading>Click me</Button>);
    expect(screen.getByRole('button')).toHaveAccessibilityState({ busy: true });
  });

  it('is disabled when isDisabled', () => {
    const onPress = jest.fn();
    render(<Button isDisabled onPress={onPress}>Click me</Button>);
    
    fireEvent.press(screen.getByRole('button'));
    expect(onPress).not.toHaveBeenCalled();
  });
});
```

### Testing Commands

```bash
# Run all tests
npm run test

# Run with coverage
npm run test -- --coverage

# Run specific file
npm run test -- Button.test.tsx

# Run e2e tests
npm run e2e:ios
npm run e2e:android
```

---

## 13. Development Workflow

### Phase-by-Phase Implementation

```
┌─────────────────────────────────────────────────────────────────┐
│  PHASE 1: Context Setup                                         │
│  ├─ Create/update .windsurfrules                                │
│  └─ Paste SOTA Context prompt before AI sessions                │
├─────────────────────────────────────────────────────────────────┤
│  PHASE 2: Feature PRD                                           │
│  ├─ Define screens needed                                       │
│  ├─ List user stories                                           │
│  └─ Identify backend endpoints required                         │
├─────────────────────────────────────────────────────────────────┤
│  PHASE 3: Design System                                         │
│  ├─ Generate theme.ts with all tokens                           │
│  ├─ Configure tailwind.config.js                                │
│  └─ Create dark mode variant if needed                          │
├─────────────────────────────────────────────────────────────────┤
│  PHASE 4: State Definition                                      │
│  ├─ Map all 6 UI states for each screen                         │
│  └─ Design skeleton/error/empty components                      │
├─────────────────────────────────────────────────────────────────┤
│  PHASE 5: Component Generation                                  │
│  ├─ Build Atoms first (Button, TextInput, etc.)                 │
│  ├─ Build Molecules (Cards, FormFields)                         │
│  ├─ Build Screens last (using atoms/molecules)                  │
│  └─ Write tests alongside components                            │
├─────────────────────────────────────────────────────────────────┤
│  PHASE 6: Iteration & Polish                                    │
│  ├─ Test in simulator (iOS + Android)                           │
│  ├─ Check keyboard handling                                     │
│  ├─ Verify safe areas                                           │
│  ├─ Run accessibility audit                                     │
│  └─ Performance profiling                                       │
└─────────────────────────────────────────────────────────────────┘
```

### Prompting Patterns for AI

**Bad Prompt:**
> "Create a profile page"

**Good Prompt:**
> "Generate ProfileScreen.tsx in mobile/src/features/users/screens/. Use useUser hook from @/api/client. Handle isLoading with Skeleton, isError with ErrorState + retry. Import User type from @/api/types. Use theme.ts spacing. Wrap in SafeAreaView."

### Review Checklist Before Merge

- [ ] All UI states handled (loading, error, empty, success, partial, refreshing)
- [ ] Accessibility props on all interactive elements
- [ ] No hardcoded colors/spacing (uses theme.ts)
- [ ] Touch targets minimum 48px
- [ ] Keyboard handling with KeyboardAvoidingView
- [ ] SafeAreaView wrapper on screens
- [ ] Types imported from @/api/types (not manually defined)
- [ ] Tests written for new components
- [ ] Works on both iOS and Android
- [ ] Dark mode compatible

---

## 14. Checklist & Quick Reference

### DO ✅

- Use React Native primitives (`View`, `Text`, `Pressable`)
- Import types from `@/api/types/`
- Use `theme.ts` for all design tokens
- Handle all 6 UI states
- Wrap screens in `SafeAreaView`
- Add accessibility props
- Use React Query for server state
- Use React Hook Form + Zod for forms
- Test components

### DON'T ❌

- Use HTML tags (`div`, `span`, `button`)
- Mock types manually
- Hardcode colors, spacing, or typography
- Return `null` for loading states
- Ignore keyboard handling
- Skip accessibility
- Use `any` type
- Put business logic in components

### Quick Component Checklist

```
□ Imports from theme.ts
□ TypeScript props interface
□ Default prop values
□ accessibilityRole
□ accessibilityLabel  
□ accessibilityState (if stateful)
□ Minimum 48px touch target
□ Handles disabled state
□ Handles loading state (if applicable)
□ Uses Reanimated for animations
□ Unit test exists
```

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | Jan 2026 | Initial release |

---

*This document is the source of truth for all frontend development in this monorepo. All AI-assisted code generation must comply with these guidelines.*
