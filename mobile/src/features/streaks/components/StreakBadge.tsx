// mobile/src/features/streaks/components/StreakBadge.tsx
// SOTA 2026 - Animated Streak Counter with Fire Effect

import React, { useEffect } from 'react';
import { View, Text, StyleSheet } from 'react-native';
import Animated, {
  useAnimatedStyle,
  useSharedValue,
  withRepeat,
  withSequence,
  withTiming,
  withSpring,
  Easing,
} from 'react-native-reanimated';
import { colors, typography, spacing, shadows, borderRadius } from '../../../shared/theme';

interface StreakBadgeProps {
  count: number;
  size?: 'sm' | 'md' | 'lg';
  animated?: boolean;
}

export function StreakBadge({ count, size = 'md', animated = true }: StreakBadgeProps) {
  const scale = useSharedValue(1);
  const rotation = useSharedValue(0);
  const glowOpacity = useSharedValue(0.4);

  useEffect(() => {
    if (animated && count > 0) {
      // Pulse animation
      scale.value = withRepeat(
        withSequence(
          withTiming(1.05, { duration: 1000, easing: Easing.inOut(Easing.ease) }),
          withTiming(1, { duration: 1000, easing: Easing.inOut(Easing.ease) })
        ),
        -1,
        true
      );

      // Subtle rotation wobble
      rotation.value = withRepeat(
        withSequence(
          withTiming(-2, { duration: 800 }),
          withTiming(2, { duration: 800 })
        ),
        -1,
        true
      );

      // Glow pulse
      glowOpacity.value = withRepeat(
        withSequence(
          withTiming(0.7, { duration: 1200 }),
          withTiming(0.3, { duration: 1200 })
        ),
        -1,
        true
      );
    }
  }, [animated, count, scale, rotation, glowOpacity]);

  const animatedStyle = useAnimatedStyle(() => ({
    transform: [
      { scale: scale.value },
      { rotate: `${rotation.value}deg` },
    ],
  }));

  const glowStyle = useAnimatedStyle(() => ({
    opacity: glowOpacity.value,
  }));

  const sizeConfig = getSizeConfig(size);
  const isOnFire = count >= 7;
  const fireColor = isOnFire ? colors.accent.orange : colors.accent.amber;

  return (
    <View style={styles.container}>
      {/* Glow effect */}
      {animated && count > 0 && (
        <Animated.View
          style={[
            styles.glow,
            {
              width: sizeConfig.glowSize,
              height: sizeConfig.glowSize,
              backgroundColor: fireColor,
              borderRadius: sizeConfig.glowSize / 2,
            },
            glowStyle,
          ]}
        />
      )}

      <Animated.View
        style={[
          styles.badge,
          {
            width: sizeConfig.size,
            height: sizeConfig.size,
            borderRadius: sizeConfig.size / 2,
            backgroundColor: count > 0 ? fireColor : colors.background.tertiary,
          },
          count > 0 && shadows.glow,
          animatedStyle,
        ]}
        accessibilityRole="text"
        accessibilityLabel={`${count} day streak`}
      >
        <Text style={styles.fireEmoji}>{count > 0 ? '🔥' : '💤'}</Text>
        <Text
          style={[
            styles.count,
            { fontSize: sizeConfig.fontSize },
            count === 0 && { color: colors.text.tertiary },
          ]}
        >
          {count}
        </Text>
      </Animated.View>

      {isOnFire && (
        <View style={styles.fireLabel}>
          <Text style={styles.fireLabelText}>ON FIRE!</Text>
        </View>
      )}
    </View>
  );
}

function getSizeConfig(size: 'sm' | 'md' | 'lg') {
  const configs = {
    sm: { size: 64, fontSize: 20, glowSize: 80 },
    md: { size: 96, fontSize: 28, glowSize: 120 },
    lg: { size: 128, fontSize: 40, glowSize: 160 },
  };
  return configs[size];
}

const styles = StyleSheet.create({
  container: {
    alignItems: 'center',
    justifyContent: 'center',
  },
  glow: {
    position: 'absolute',
  },
  badge: {
    alignItems: 'center',
    justifyContent: 'center',
  },
  fireEmoji: {
    fontSize: 24,
    marginBottom: 2,
  },
  count: {
    ...typography.statSmall,
    color: colors.text.inverse,
  },
  fireLabel: {
    marginTop: spacing[2],
    backgroundColor: colors.accent.orange,
    paddingHorizontal: spacing[3],
    paddingVertical: spacing[1],
    borderRadius: borderRadius.full,
  },
  fireLabelText: {
    ...typography.overline,
    color: colors.text.inverse,
    fontFamily: typography.fontFamily.bold,
    letterSpacing: 1,
  },
});
