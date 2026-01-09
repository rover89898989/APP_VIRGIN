// mobile/src/features/streaks/components/StatCard.tsx
// SOTA 2026 - Animated Statistic Display Card

import React, { useEffect } from 'react';
import { View, Text, StyleSheet } from 'react-native';
import Animated, {
  useAnimatedStyle,
  useSharedValue,
  withTiming,
  Easing,
  FadeInUp,
} from 'react-native-reanimated';
import { colors, typography, spacing, borderRadius, shadows } from '../../../shared/theme';

interface StatCardProps {
  label: string;
  value: number;
  suffix?: string;
  icon: string;
  color?: string;
  index?: number;
}

export function StatCard({
  label,
  value,
  suffix = '',
  icon,
  color = colors.primary[500],
  index = 0,
}: StatCardProps) {
  const displayValue = useSharedValue(0);

  useEffect(() => {
    displayValue.value = withTiming(value, {
      duration: 1000,
      easing: Easing.out(Easing.cubic),
    });
  }, [value, displayValue]);

  const animatedTextStyle = useAnimatedStyle(() => ({
    opacity: 1,
  }));

  return (
    <Animated.View
      entering={FadeInUp.delay(index * 100).springify()}
      style={[styles.card, { borderLeftColor: color }]}
    >
      <View style={styles.header}>
        <Text style={styles.icon}>{icon}</Text>
        <Text style={styles.label}>{label}</Text>
      </View>
      <Animated.View style={animatedTextStyle}>
        <Text style={[styles.value, { color }]}>
          {value}
          {suffix && <Text style={styles.suffix}>{suffix}</Text>}
        </Text>
      </Animated.View>
    </Animated.View>
  );
}

const styles = StyleSheet.create({
  card: {
    flex: 1,
    backgroundColor: colors.surface.elevated,
    borderRadius: borderRadius.lg,
    padding: spacing[4],
    borderLeftWidth: 4,
    ...shadows.sm,
  },
  header: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: spacing[2],
  },
  icon: {
    fontSize: 16,
    marginRight: spacing[2],
  },
  label: {
    ...typography.caption,
    color: colors.text.secondary,
    textTransform: 'uppercase',
    letterSpacing: 0.5,
  },
  value: {
    ...typography.statSmall,
  },
  suffix: {
    ...typography.body,
    color: colors.text.secondary,
  },
});
