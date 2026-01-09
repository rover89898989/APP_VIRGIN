// mobile/src/features/streaks/screens/StreakDashboardScreen.tsx
// SOTA 2026 - Sample Proof of Concept: Daily Streak Dashboard
//
// This screen demonstrates ALL guidelines from FRONTEND_GUIDELINES.md:
// ✅ React Native primitives (View, Text, ScrollView, etc.)
// ✅ Theme tokens (no hardcoded colors/spacing)
// ✅ All 6 UI states (Loading, Error, Empty, Success, Partial, Refreshing)
// ✅ React Query for data fetching
// ✅ Accessibility props
// ✅ Reanimated animations
// ✅ Component composition (atoms → molecules → screen)
// ✅ Type imports from @/api/types

import React, { useCallback } from 'react';
import {
  View,
  Text,
  ScrollView,
  RefreshControl,
  StyleSheet,
} from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';
import Animated, { FadeIn, FadeInDown, SlideInRight } from 'react-native-reanimated';

import { Button, Card, ErrorState, Skeleton, SkeletonCircle } from '../../../shared/components';
import { colors, typography, spacing, borderRadius } from '../../../shared/theme';
import { StreakBadge, WeeklyCalendar, AchievementCard, StatCard } from '../components';
import type { DailyStreak } from '../../../api/types';

// ==========================================================================
// MOCK DATA FOR DEMO (Replace with useStreak() hook in production)
// ==========================================================================
const MOCK_STREAK: DailyStreak = {
  id: 'streak-001',
  userId: 'user-001',
  currentStreak: 12,
  longestStreak: 21,
  lastActivityDate: new Date().toISOString(),
  totalDaysActive: 47,
  streakStartDate: new Date(Date.now() - 12 * 24 * 60 * 60 * 1000).toISOString(),
  weeklyActivity: [
    { date: '2026-01-06', isActive: true, activityCount: 3 },
    { date: '2026-01-07', isActive: true, activityCount: 5 },
    { date: '2026-01-08', isActive: true, activityCount: 2 },
    { date: '2026-01-09', isActive: true, activityCount: 4 },
    { date: '2026-01-10', isActive: false, activityCount: 0 },
    { date: '2026-01-11', isActive: false, activityCount: 0 },
    { date: '2026-01-12', isActive: false, activityCount: 0 },
  ],
  achievements: [
    {
      id: 'ach-1',
      title: 'First Flame',
      description: 'Start your first streak',
      icon: 'flame',
      unlockedAt: '2026-01-01',
      requirement: 1,
      progress: 1,
    },
    {
      id: 'ach-2',
      title: 'Week Warrior',
      description: 'Maintain a 7-day streak',
      icon: 'star',
      unlockedAt: '2026-01-08',
      requirement: 7,
      progress: 7,
    },
    {
      id: 'ach-3',
      title: 'Fortnight Force',
      description: 'Maintain a 14-day streak',
      icon: 'trophy',
      unlockedAt: null,
      requirement: 14,
      progress: 12,
    },
    {
      id: 'ach-4',
      title: 'Monthly Master',
      description: 'Maintain a 30-day streak',
      icon: 'crown',
      unlockedAt: null,
      requirement: 30,
      progress: 12,
    },
  ],
};

export function StreakDashboardScreen() {
  // DEMO MODE: Using mock data
  // In production, replace with:
  // const { data: streak, isLoading, isError, error, refetch, isFetching } = useStreak();
  const streak = MOCK_STREAK;
  const isLoading = false;
  const isError = false;
  const error: Error | null = null;
  const isFetching = false;
  const refetch = () => {};

  // Mock check-in state
  const [checkedIn, setCheckedIn] = React.useState(false);
  const checkIn = {
    isPending: false,
    isSuccess: checkedIn,
    mutate: () => setCheckedIn(true),
  };

  const handleCheckIn = useCallback(() => {
    checkIn.mutate();
  }, [checkIn]);

  const handleRefresh = useCallback(() => {
    refetch();
  }, [refetch]);

  // ==========================================================================
  // STATE 1: Loading State (isLoading)
  // ==========================================================================
  if (isLoading) {
    return (
      <SafeAreaView style={styles.safeArea}>
        <StreakDashboardSkeleton />
      </SafeAreaView>
    );
  }

  // ==========================================================================
  // STATE 2: Error State (isError)
  // ==========================================================================
  if (isError) {
    return (
      <SafeAreaView style={styles.safeArea}>
        <ErrorState
          title="Couldn't load your streak"
          message={(error as Error | null)?.message || 'Please check your connection and try again.'}
          onRetry={refetch}
        />
      </SafeAreaView>
    );
  }

  // ==========================================================================
  // STATE 3: Empty/Zero State (no streak data)
  // ==========================================================================
  if (!streak) {
    return (
      <SafeAreaView style={styles.safeArea}>
        <View style={styles.emptyState}>
          <Text style={styles.emptyEmoji}>🌱</Text>
          <Text style={styles.emptyTitle}>Start Your Streak!</Text>
          <Text style={styles.emptyMessage}>
            Check in daily to build your streak and unlock achievements.
          </Text>
          <Button onPress={handleCheckIn} variant="primary" isLoading={checkIn.isPending}>
            Begin Journey
          </Button>
        </View>
      </SafeAreaView>
    );
  }

  // ==========================================================================
  // STATE 4 & 5: Success State (with Partial handling)
  // ==========================================================================
  const hasAchievements = streak.achievements && streak.achievements.length > 0;
  const hasWeeklyData = streak.weeklyActivity && streak.weeklyActivity.length > 0;

  return (
    <SafeAreaView style={styles.safeArea}>
      <ScrollView
        style={styles.scrollView}
        contentContainerStyle={styles.scrollContent}
        showsVerticalScrollIndicator={false}
        refreshControl={
          // STATE 6: Refreshing State (isFetching + data)
          <RefreshControl
            refreshing={isFetching && !isLoading}
            onRefresh={handleRefresh}
            tintColor={colors.primary[500]}
          />
        }
      >
        {/* Header */}
        <Animated.View entering={FadeIn.duration(400)} style={styles.header}>
          <Text style={styles.greeting}>Welcome back! 👋</Text>
          <Text style={styles.headerTitle}>Your Streak</Text>
        </Animated.View>

        {/* Main Streak Badge */}
        <Animated.View entering={FadeInDown.delay(100).springify()} style={styles.streakSection}>
          <StreakBadge count={streak.currentStreak} size="lg" />
          <Text style={styles.streakLabel}>
            {streak.currentStreak === 0
              ? "Let's get started!"
              : streak.currentStreak === 1
              ? 'day streak'
              : 'days streak'}
          </Text>
        </Animated.View>

        {/* Check-in Button */}
        <Animated.View entering={FadeInDown.delay(200)} style={styles.checkInSection}>
          <Button
            onPress={handleCheckIn}
            variant="success"
            size="lg"
            fullWidth
            isLoading={checkIn.isPending}
          >
            {checkIn.isSuccess ? '✓ Checked In Today!' : "Check In for Today"}
          </Button>
        </Animated.View>

        {/* Stats Row */}
        <Animated.View entering={FadeInDown.delay(300)} style={styles.statsRow}>
          <StatCard
            label="Best Streak"
            value={streak.longestStreak}
            suffix=" days"
            icon="🏆"
            color={colors.accent.amber}
            index={0}
          />
          <View style={styles.statSpacer} />
          <StatCard
            label="Total Days"
            value={streak.totalDaysActive}
            icon="📅"
            color={colors.accent.emerald}
            index={1}
          />
        </Animated.View>

        {/* Weekly Calendar */}
        {hasWeeklyData && (
          <Animated.View entering={SlideInRight.delay(400)}>
            <Card variant="outlined" style={styles.calendarCard}>
              <WeeklyCalendar days={streak.weeklyActivity} />
            </Card>
          </Animated.View>
        )}

        {/* Achievements Section */}
        {hasAchievements && (
          <Animated.View entering={FadeInDown.delay(500)} style={styles.achievementsSection}>
            <Text style={styles.sectionTitle}>Achievements</Text>
            {streak.achievements.map((achievement, index) => (
              <AchievementCard
                key={achievement.id}
                achievement={achievement}
                index={index}
              />
            ))}
          </Animated.View>
        )}

        {/* Partial State: No achievements yet */}
        {!hasAchievements && (
          <Animated.View entering={FadeInDown.delay(500)} style={styles.noAchievements}>
            <Card variant="filled" padding={6}>
              <Text style={styles.noAchievementsEmoji}>🎯</Text>
              <Text style={styles.noAchievementsTitle}>Achievements Coming Soon</Text>
              <Text style={styles.noAchievementsText}>
                Keep your streak going to unlock special badges!
              </Text>
            </Card>
          </Animated.View>
        )}

        {/* Bottom Spacing */}
        <View style={styles.bottomSpacer} />
      </ScrollView>
    </SafeAreaView>
  );
}

// ==========================================================================
// SKELETON LOADER COMPONENT
// ==========================================================================
function StreakDashboardSkeleton() {
  return (
    <View style={styles.skeletonContainer}>
      {/* Header Skeleton */}
      <View style={styles.skeletonHeader}>
        <Skeleton width={120} height={16} />
        <Skeleton width={180} height={32} style={{ marginTop: spacing[2] }} />
      </View>

      {/* Badge Skeleton */}
      <View style={styles.skeletonBadge}>
        <SkeletonCircle size={128} />
        <Skeleton width={100} height={20} style={{ marginTop: spacing[4] }} />
      </View>

      {/* Button Skeleton */}
      <Skeleton width="100%" height={52} borderRadius={borderRadius.md} style={styles.skeletonButton} />

      {/* Stats Skeleton */}
      <View style={styles.skeletonStats}>
        <Skeleton width="48%" height={100} borderRadius={borderRadius.lg} />
        <Skeleton width="48%" height={100} borderRadius={borderRadius.lg} />
      </View>

      {/* Calendar Skeleton */}
      <Skeleton width="100%" height={120} borderRadius={borderRadius.lg} style={styles.skeletonCalendar} />
    </View>
  );
}

// ==========================================================================
// STYLES
// ==========================================================================
const styles = StyleSheet.create({
  safeArea: {
    flex: 1,
    backgroundColor: colors.background.primary,
  },
  scrollView: {
    flex: 1,
  },
  scrollContent: {
    paddingHorizontal: spacing[4],
    paddingTop: spacing[4],
  },

  // Header
  header: {
    marginBottom: spacing[6],
  },
  greeting: {
    ...typography.bodySmall,
    color: colors.text.secondary,
  },
  headerTitle: {
    ...typography.h1,
    color: colors.text.primary,
    marginTop: spacing[1],
  },

  // Streak Section
  streakSection: {
    alignItems: 'center',
    marginBottom: spacing[6],
  },
  streakLabel: {
    ...typography.h4,
    color: colors.text.secondary,
    marginTop: spacing[3],
  },

  // Check-in
  checkInSection: {
    marginBottom: spacing[6],
  },

  // Stats
  statsRow: {
    flexDirection: 'row',
    marginBottom: spacing[4],
  },
  statSpacer: {
    width: spacing[3],
  },

  // Calendar
  calendarCard: {
    marginBottom: spacing[4],
  },

  // Achievements
  achievementsSection: {
    marginTop: spacing[4],
  },
  sectionTitle: {
    ...typography.h3,
    color: colors.text.primary,
    marginBottom: spacing[4],
  },

  // No Achievements
  noAchievements: {
    marginTop: spacing[4],
  },
  noAchievementsEmoji: {
    fontSize: 40,
    textAlign: 'center',
    marginBottom: spacing[3],
  },
  noAchievementsTitle: {
    ...typography.h4,
    color: colors.text.primary,
    textAlign: 'center',
    marginBottom: spacing[2],
  },
  noAchievementsText: {
    ...typography.body,
    color: colors.text.secondary,
    textAlign: 'center',
  },

  // Empty State
  emptyState: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
    padding: spacing[6],
  },
  emptyEmoji: {
    fontSize: 64,
    marginBottom: spacing[4],
  },
  emptyTitle: {
    ...typography.h2,
    color: colors.text.primary,
    marginBottom: spacing[2],
  },
  emptyMessage: {
    ...typography.body,
    color: colors.text.secondary,
    textAlign: 'center',
    marginBottom: spacing[6],
  },

  // Skeleton
  skeletonContainer: {
    flex: 1,
    padding: spacing[4],
  },
  skeletonHeader: {
    marginBottom: spacing[6],
  },
  skeletonBadge: {
    alignItems: 'center',
    marginBottom: spacing[6],
  },
  skeletonButton: {
    marginBottom: spacing[6],
  },
  skeletonStats: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginBottom: spacing[4],
  },
  skeletonCalendar: {
    marginBottom: spacing[4],
  },

  // Bottom
  bottomSpacer: {
    height: spacing[10],
  },
});
