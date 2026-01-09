// mobile/src/shared/theme.ts
// SOTA 2026 Design System Tokens

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

  // Accent (for streaks & achievements)
  accent: {
    orange: '#F97316',
    amber: '#F59E0B',
    emerald: '#10B981',
    cyan: '#06B6D4',
    pink: '#EC4899',
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
  fontFamily: {
    regular: 'Inter-Regular',
    medium: 'Inter-Medium',
    semibold: 'Inter-SemiBold',
    bold: 'Inter-Bold',
  },
  
  h1: { fontSize: 32, lineHeight: 40, fontFamily: 'Inter-Bold' },
  h2: { fontSize: 24, lineHeight: 32, fontFamily: 'Inter-SemiBold' },
  h3: { fontSize: 20, lineHeight: 28, fontFamily: 'Inter-SemiBold' },
  h4: { fontSize: 18, lineHeight: 24, fontFamily: 'Inter-Medium' },
  body: { fontSize: 16, lineHeight: 24, fontFamily: 'Inter-Regular' },
  bodySmall: { fontSize: 14, lineHeight: 20, fontFamily: 'Inter-Regular' },
  caption: { fontSize: 12, lineHeight: 16, fontFamily: 'Inter-Regular' },
  overline: { fontSize: 10, lineHeight: 14, fontFamily: 'Inter-Medium' },
  
  // Special: Big Numbers for stats
  stat: { fontSize: 48, lineHeight: 56, fontFamily: 'Inter-Bold' },
  statSmall: { fontSize: 28, lineHeight: 36, fontFamily: 'Inter-Bold' },
} as const;

export const spacing = {
  0: 0,
  1: 4,
  2: 8,
  3: 12,
  4: 16,
  5: 20,
  6: 24,
  8: 32,
  10: 40,
  12: 48,
  16: 64,
  20: 80,
  24: 96,
} as const;

export const borderRadius = {
  none: 0,
  sm: 4,
  md: 8,
  lg: 12,
  xl: 16,
  '2xl': 24,
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
  glow: {
    shadowColor: '#6366F1',
    shadowOffset: { width: 0, height: 0 },
    shadowOpacity: 0.4,
    shadowRadius: 12,
    elevation: 8,
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
    bounce: { damping: 10, stiffness: 180 },
  },
} as const;

export const components = {
  button: {
    height: { sm: 36, md: 44, lg: 52 },
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
  streakBadge: {
    size: { sm: 48, md: 64, lg: 80 },
  },
} as const;
