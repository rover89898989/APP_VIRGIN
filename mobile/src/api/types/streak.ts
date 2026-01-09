// mobile/src/api/types/streak.ts
// Auto-generated types from Rust backend

export interface DailyStreak {
  id: string;
  userId: string;
  currentStreak: number;
  longestStreak: number;
  lastActivityDate: string; // ISO date
  totalDaysActive: number;
  streakStartDate: string; // ISO date
  achievements: Achievement[];
  weeklyActivity: DayActivity[];
}

export interface Achievement {
  id: string;
  title: string;
  description: string;
  icon: AchievementIcon;
  unlockedAt: string | null; // null if locked
  requirement: number;
  progress: number;
}

export type AchievementIcon = 
  | 'flame'
  | 'star'
  | 'trophy'
  | 'rocket'
  | 'crown'
  | 'lightning'
  | 'medal'
  | 'diamond';

export interface DayActivity {
  date: string; // ISO date
  isActive: boolean;
  activityCount: number;
}

export interface StreakCheckIn {
  streakId: string;
  activityType: 'login' | 'task_complete' | 'goal_met';
}

export interface StreakResponse {
  streak: DailyStreak;
  message: string;
  bonusAwarded: boolean;
}
