// mobile/src/features/streaks/components/AchievementCard.tsx
// SOTA 2026 - Achievement Badge with Progress

import React from 'react';
import { View, Text, StyleSheet } from 'react-native';
import Animated, { FadeInUp } from 'react-native-reanimated';
import { colors, typography, spacing, borderRadius, shadows } from '../../../shared/theme';
import type { Achievement, AchievementIcon } from '../../../api/types';

interface AchievementCardProps {
  achievement: Achievement;
  index?: number;
}

const ICON_MAP: Record<AchievementIcon, string> = {
  flame: '🔥',
  star: '⭐',
  trophy: '🏆',
  rocket: '🚀',
  crown: '👑',
  lightning: '⚡',
  medal: '🏅',
  diamond: '💎',
};

export function AchievementCard({ achievement, index = 0 }: AchievementCardProps) {
  const isUnlocked = achievement.unlockedAt !== null;
  const progress = Math.min(achievement.progress / achievement.requirement, 1);

  return (
    <Animated.View
      entering={FadeInUp.delay(index * 100).springify()}
      style={[
        styles.card,
        !isUnlocked && styles.lockedCard,
      ]}
      accessibilityRole="button"
      accessibilityLabel={`${achievement.title}: ${isUnlocked ? 'Unlocked' : `${Math.round(progress * 100)}% complete`}`}
    >
      <View style={[styles.iconContainer, !isUnlocked && styles.lockedIcon]}>
        <Text style={styles.icon}>{ICON_MAP[achievement.icon]}</Text>
        {!isUnlocked && <View style={styles.lockOverlay}><Text>🔒</Text></View>}
      </View>

      <View style={styles.content}>
        <Text style={[styles.title, !isUnlocked && styles.lockedText]}>
          {achievement.title}
        </Text>
        <Text style={styles.description} numberOfLines={1}>
          {achievement.description}
        </Text>

        {!isUnlocked && (
          <View style={styles.progressContainer}>
            <View style={styles.progressTrack}>
              <View style={[styles.progressFill, { width: `${progress * 100}%` }]} />
            </View>
            <Text style={styles.progressText}>
              {achievement.progress}/{achievement.requirement}
            </Text>
          </View>
        )}
      </View>

      {isUnlocked && (
        <View style={styles.unlockedBadge}>
          <Text style={styles.unlockedText}>✓</Text>
        </View>
      )}
    </Animated.View>
  );
}

const styles = StyleSheet.create({
  card: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: colors.surface.elevated,
    borderRadius: borderRadius.lg,
    padding: spacing[3],
    marginBottom: spacing[3],
    ...shadows.sm,
  },
  lockedCard: {
    opacity: 0.7,
  },
  iconContainer: {
    width: 48,
    height: 48,
    borderRadius: 24,
    backgroundColor: colors.primary[100],
    alignItems: 'center',
    justifyContent: 'center',
    marginRight: spacing[3],
  },
  lockedIcon: {
    backgroundColor: colors.background.tertiary,
  },
  icon: {
    fontSize: 24,
  },
  lockOverlay: {
    position: 'absolute',
    bottom: -4,
    right: -4,
  },
  content: {
    flex: 1,
  },
  title: {
    ...typography.bodySmall,
    fontFamily: typography.fontFamily.semibold,
    color: colors.text.primary,
  },
  lockedText: {
    color: colors.text.secondary,
  },
  description: {
    ...typography.caption,
    color: colors.text.tertiary,
    marginTop: 2,
  },
  progressContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    marginTop: spacing[2],
    gap: spacing[2],
  },
  progressTrack: {
    flex: 1,
    height: 4,
    backgroundColor: colors.background.tertiary,
    borderRadius: 2,
    overflow: 'hidden',
  },
  progressFill: {
    height: '100%',
    backgroundColor: colors.primary[500],
    borderRadius: 2,
  },
  progressText: {
    ...typography.caption,
    color: colors.text.tertiary,
    minWidth: 40,
    textAlign: 'right',
  },
  unlockedBadge: {
    width: 24,
    height: 24,
    borderRadius: 12,
    backgroundColor: colors.status.success,
    alignItems: 'center',
    justifyContent: 'center',
  },
  unlockedText: {
    color: colors.text.inverse,
    fontSize: 14,
    fontFamily: typography.fontFamily.bold,
  },
});
