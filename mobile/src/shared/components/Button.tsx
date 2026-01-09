// mobile/src/shared/components/Button.tsx
// SOTA 2026 - Accessible, Animated Button Component

import React from 'react';
import {
  Pressable,
  Text,
  ActivityIndicator,
  StyleSheet,
  type PressableProps,
  type ViewStyle,
} from 'react-native';
import Animated, {
  useAnimatedStyle,
  useSharedValue,
  withSpring,
} from 'react-native-reanimated';
import { colors, components, typography, spacing, borderRadius } from '../theme';

const AnimatedPressable = Animated.createAnimatedComponent(Pressable);

type ButtonVariant = 'primary' | 'secondary' | 'danger' | 'ghost' | 'success';
type ButtonSize = 'sm' | 'md' | 'lg';

interface ButtonProps extends Omit<PressableProps, 'children' | 'style'> {
  children: string;
  variant?: ButtonVariant;
  size?: ButtonSize;
  isLoading?: boolean;
  isDisabled?: boolean;
  leftIcon?: React.ReactNode;
  rightIcon?: React.ReactNode;
  fullWidth?: boolean;
  style?: ViewStyle;
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
  style,
  ...props
}: ButtonProps) {
  const scale = useSharedValue(1);

  const animatedStyle = useAnimatedStyle(() => ({
    transform: [{ scale: scale.value }],
  }));

  const handlePressIn = () => {
    scale.value = withSpring(0.96, { damping: 15, stiffness: 200 });
  };

  const handlePressOut = () => {
    scale.value = withSpring(1, { damping: 15, stiffness: 200 });
  };

  const variantStyles = getVariantStyles(variant, isDisabled);
  const sizeStyles = getSizeStyles(size);

  return (
    <AnimatedPressable
      style={[
        styles.base,
        variantStyles.container,
        sizeStyles.container,
        fullWidth && styles.fullWidth,
        animatedStyle,
        style,
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
        <ActivityIndicator color={variantStyles.textColor} size="small" />
      ) : (
        <>
          {leftIcon}
          <Text
            style={[
              styles.text,
              { color: variantStyles.textColor },
              sizeStyles.text,
              leftIcon ? { marginLeft: spacing[2] } : undefined,
              rightIcon ? { marginRight: spacing[2] } : undefined,
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

const styles = StyleSheet.create({
  base: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    borderRadius: borderRadius.md,
    minHeight: components.button.minTouchTarget,
  },
  fullWidth: {
    width: '100%',
  },
  text: {
    fontFamily: typography.fontFamily.semibold,
    fontSize: typography.body.fontSize,
  },
});

function getVariantStyles(variant: ButtonVariant, isDisabled: boolean) {
  const opacity = isDisabled ? 0.5 : 1;

  const variants = {
    primary: {
      container: { backgroundColor: colors.primary[500], opacity } as ViewStyle,
      textColor: colors.text.inverse,
    },
    secondary: {
      container: {
        backgroundColor: 'transparent',
        borderWidth: 1.5,
        borderColor: colors.border.secondary,
        opacity,
      } as ViewStyle,
      textColor: colors.text.primary,
    },
    danger: {
      container: { backgroundColor: colors.status.error, opacity } as ViewStyle,
      textColor: colors.text.inverse,
    },
    ghost: {
      container: { backgroundColor: 'transparent', opacity } as ViewStyle,
      textColor: colors.primary[500],
    },
    success: {
      container: { backgroundColor: colors.status.success, opacity } as ViewStyle,
      textColor: colors.text.inverse,
    },
  };

  return variants[variant];
}

function getSizeStyles(size: ButtonSize) {
  const sizes = {
    sm: {
      container: {
        height: components.button.height.sm,
        paddingHorizontal: spacing[3],
      } as ViewStyle,
      text: { fontSize: 14 },
    },
    md: {
      container: {
        height: components.button.height.md,
        paddingHorizontal: spacing[4],
      } as ViewStyle,
      text: { fontSize: 16 },
    },
    lg: {
      container: {
        height: components.button.height.lg,
        paddingHorizontal: spacing[6],
      } as ViewStyle,
      text: { fontSize: 18 },
    },
  };

  return sizes[size];
}
