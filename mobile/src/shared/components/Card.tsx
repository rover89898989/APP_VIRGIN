// mobile/src/shared/components/Card.tsx
// SOTA 2026 - Elevated Card Container

import React from 'react';
import { View, StyleSheet, type ViewStyle, type ViewProps } from 'react-native';
import { colors, spacing, borderRadius, shadows } from '../theme';

type CardVariant = 'elevated' | 'outlined' | 'filled';

interface CardProps extends ViewProps {
  children: React.ReactNode;
  variant?: CardVariant;
  padding?: keyof typeof spacing;
  style?: ViewStyle;
}

export function Card({
  children,
  variant = 'elevated',
  padding = 4,
  style,
  ...props
}: CardProps) {
  const variantStyles = getVariantStyles(variant);

  return (
    <View
      style={[
        styles.base,
        variantStyles,
        { padding: spacing[padding] },
        style,
      ]}
      {...props}
    >
      {children}
    </View>
  );
}

const styles = StyleSheet.create({
  base: {
    borderRadius: borderRadius.lg,
    overflow: 'hidden',
  },
});

function getVariantStyles(variant: CardVariant): ViewStyle {
  switch (variant) {
    case 'elevated':
      return {
        backgroundColor: colors.surface.elevated,
        ...shadows.md,
      };
    case 'outlined':
      return {
        backgroundColor: colors.surface.primary,
        borderWidth: 1,
        borderColor: colors.border.primary,
      };
    case 'filled':
      return {
        backgroundColor: colors.background.secondary,
      };
    default:
      return {};
  }
}
