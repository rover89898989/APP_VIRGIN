// mobile/src/features/streaks/components/WeeklyCalendar.tsx
// SOTA 2026 - Visual Weekly Activity Calendar

import React from 'react';
import { View, Text, StyleSheet } from 'react-native';
import Animated, { FadeIn } from 'react-native-reanimated';
import { colors, typography, spacing, borderRadius } from '../../../shared/theme';
import type { DayActivity } from '../../../api/types';

interface WeeklyCalendarProps {
  days: DayActivity[];
}

const DAY_LABELS = ['M', 'T', 'W', 'T', 'F', 'S', 'S'];

export function WeeklyCalendar({ days }: WeeklyCalendarProps) {
  const today = new Date().getDay();
  const todayIndex = today === 0 ? 6 : today - 1; // Convert Sun=0 to index 6

  return (
    <View style={styles.container}>
      <Text style={styles.title}>This Week</Text>
      <View style={styles.calendar}>
        {DAY_LABELS.map((label, index) => {
          const dayData = days[index];
          const isActive = dayData?.isActive ?? false;
          const isToday = index === todayIndex;
          const isFuture = index > todayIndex;

          return (
            <Animated.View
              key={index}
              entering={FadeIn.delay(index * 50)}
              style={styles.dayColumn}
            >
              <Text
                style={[
                  styles.dayLabel,
                  isToday && styles.todayLabel,
                ]}
              >
                {label}
              </Text>
              <View
                style={[
                  styles.dayDot,
                  isActive && styles.activeDot,
                  isToday && styles.todayDot,
                  isFuture && styles.futureDot,
                ]}
                accessibilityRole="image"
                accessibilityLabel={`${DAY_LABELS[index]}: ${isActive ? 'Active' : 'Inactive'}`}
              >
                {isActive && <Text style={styles.checkmark}>✓</Text>}
              </View>
            </Animated.View>
          );
        })}
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    paddingVertical: spacing[4],
  },
  title: {
    ...typography.caption,
    color: colors.text.secondary,
    textTransform: 'uppercase',
    letterSpacing: 1,
    marginBottom: spacing[3],
    textAlign: 'center',
  },
  calendar: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    paddingHorizontal: spacing[2],
  },
  dayColumn: {
    alignItems: 'center',
    gap: spacing[2],
  },
  dayLabel: {
    ...typography.caption,
    color: colors.text.tertiary,
    fontFamily: typography.fontFamily.medium,
  },
  todayLabel: {
    color: colors.primary[500],
    fontFamily: typography.fontFamily.bold,
  },
  dayDot: {
    width: 36,
    height: 36,
    borderRadius: 18,
    backgroundColor: colors.background.tertiary,
    alignItems: 'center',
    justifyContent: 'center',
    borderWidth: 2,
    borderColor: 'transparent',
  },
  activeDot: {
    backgroundColor: colors.status.success,
    borderColor: colors.status.success,
  },
  todayDot: {
    borderColor: colors.primary[500],
    borderWidth: 2,
  },
  futureDot: {
    backgroundColor: colors.background.secondary,
    opacity: 0.5,
  },
  checkmark: {
    color: colors.text.inverse,
    fontSize: 16,
    fontFamily: typography.fontFamily.bold,
  },
});
