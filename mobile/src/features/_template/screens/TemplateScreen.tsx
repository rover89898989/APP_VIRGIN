// mobile/src/features/_template/screens/TemplateScreen.tsx
// Rename: TemplateScreen → YourFeatureScreen
//
// This template demonstrates all 6 UI states.
// Replace useTemplateData with your actual API hook.

import React from 'react';
import { View, Text, FlatList, RefreshControl, StyleSheet } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';

// Import your API hook from client.ts
// import { useYourData } from '../../../api/client';

// Shared components
import { Button, ErrorState, Skeleton } from '../../../shared/components';
import { colors, typography, spacing } from '../../../shared/theme';

// Feature components
import { TemplateCard } from '../components';

// =============================================================================
// MOCK DATA - Replace with real API hook
// =============================================================================
const useMockData = () => {
  // Simulate different states for testing
  // Change these to test different UI states
  const isLoading = false;
  const isError = false;
  const isFetching = false;
  const error: Error | null = null;
  
  const data = [
    { id: '1', title: 'Item One', subtitle: 'Description' },
    { id: '2', title: 'Item Two', subtitle: 'Description' },
    { id: '3', title: 'Item Three', subtitle: 'Description' },
  ];
  
  const refetch = () => console.log('Refetching...');
  
  return { data, isLoading, isError, isFetching, error, refetch };
};

export function TemplateScreen() {
  // Replace useMockData with your real hook:
  // const { data, isLoading, isError, isFetching, error, refetch } = useYourData();
  const { data, isLoading, isError, isFetching, error, refetch } = useMockData();

  // =========================================================================
  // STATE 1: Loading
  // =========================================================================
  if (isLoading) {
    return (
      <SafeAreaView style={styles.container}>
        <View style={styles.content}>
          <Skeleton width="100%" height={80} style={{ marginBottom: spacing[3] }} />
          <Skeleton width="100%" height={80} style={{ marginBottom: spacing[3] }} />
          <Skeleton width="100%" height={80} />
        </View>
      </SafeAreaView>
    );
  }

  // =========================================================================
  // STATE 2: Error
  // =========================================================================
  if (isError) {
    return (
      <SafeAreaView style={styles.container}>
        <ErrorState
          title="Something went wrong"
          message={(error as Error | null)?.message || 'Please try again'}
          onRetry={refetch}
        />
      </SafeAreaView>
    );
  }

  // =========================================================================
  // STATE 3: Empty
  // =========================================================================
  if (!data || data.length === 0) {
    return (
      <SafeAreaView style={styles.container}>
        <View style={styles.emptyState}>
          <Text style={styles.emptyEmoji}>📭</Text>
          <Text style={styles.emptyTitle}>Nothing here yet</Text>
          <Text style={styles.emptyMessage}>
            Add your first item to get started.
          </Text>
          <Button onPress={() => {}} variant="primary">
            Add Item
          </Button>
        </View>
      </SafeAreaView>
    );
  }

  // =========================================================================
  // STATE 4, 5, 6: Success / Partial / Refreshing
  // =========================================================================
  return (
    <SafeAreaView style={styles.container}>
      <FlatList
        data={data}
        keyExtractor={(item) => item.id}
        renderItem={({ item }) => (
          <TemplateCard
            title={item.title}
            subtitle={item.subtitle}
            onPress={() => console.log('Pressed:', item.id)}
          />
        )}
        contentContainerStyle={styles.content}
        refreshControl={
          <RefreshControl
            refreshing={isFetching && !isLoading}
            onRefresh={refetch}
            tintColor={colors.primary[500]}
          />
        }
        ListHeaderComponent={
          <Text style={styles.header}>Template Feature</Text>
        }
      />
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background.primary,
  },
  content: {
    padding: spacing[4],
  },
  header: {
    ...typography.h2,
    color: colors.text.primary,
    marginBottom: spacing[4],
  },
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
    ...typography.h3,
    color: colors.text.primary,
    marginBottom: spacing[2],
  },
  emptyMessage: {
    ...typography.body,
    color: colors.text.secondary,
    textAlign: 'center',
    marginBottom: spacing[6],
  },
});
