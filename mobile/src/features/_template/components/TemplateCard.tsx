// mobile/src/features/_template/components/TemplateCard.tsx
// Rename: TemplateCard → YourFeatureCard

import React from 'react';
import { View, Text, StyleSheet, Pressable } from 'react-native';
import { colors, typography, spacing, borderRadius, shadows } from '../../../shared/theme';

interface TemplateCardProps {
  title: string;
  subtitle?: string;
  onPress?: () => void;
}

export function TemplateCard({ title, subtitle, onPress }: TemplateCardProps) {
  return (
    <Pressable
      style={styles.card}
      onPress={onPress}
      accessibilityRole="button"
      accessibilityLabel={title}
    >
      <Text style={styles.title}>{title}</Text>
      {subtitle && <Text style={styles.subtitle}>{subtitle}</Text>}
    </Pressable>
  );
}

const styles = StyleSheet.create({
  card: {
    backgroundColor: colors.surface.elevated,
    borderRadius: borderRadius.lg,
    padding: spacing[4],
    marginBottom: spacing[3],
    ...shadows.sm,
  },
  title: {
    ...typography.body,
    fontFamily: typography.fontFamily.semibold,
    color: colors.text.primary,
  },
  subtitle: {
    ...typography.bodySmall,
    color: colors.text.secondary,
    marginTop: spacing[1],
  },
});
