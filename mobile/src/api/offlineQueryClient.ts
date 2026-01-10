import { QueryClient, useQuery } from '@tanstack/react-query';

// Offline-first Query Client configuration
export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      // Cache for 24 hours (v5 uses gcTime instead of cacheTime)
      gcTime: 1000 * 60 * 60 * 24,
      
      // Consider data fresh for 5 minutes
      staleTime: 1000 * 60 * 5,
      
      // Retry failed requests 2 times
      retry: (failureCount, error: any) => {
        // Don't retry on 4xx errors (client errors)
        if (error?.status >= 400 && error?.status < 500) {
          return false;
        }
        // Retry up to 2 times for other errors
        return failureCount < 2;
      },
      
      // Retry delay with exponential backoff
      retryDelay: (attemptIndex) => Math.min(1000 * 2 ** attemptIndex, 30000),
      
      // Enable offline-first behavior
      networkMode: 'offlineFirst',
      
      // Refetch on window focus (web) or app foreground (native)
      refetchOnWindowFocus: true,
      
      // Refetch on reconnect
      refetchOnReconnect: true,
      
      // Don't refetch on mount if data is fresh
      refetchOnMount: false,
    },
    
    mutations: {
      // Retry mutations once
      retry: 1,
      
      // Network mode for mutations
      networkMode: 'online',
    },
  },
});
      
      // Retry failed requests 2 times
      retry: (failureCount, error: any) => {
        // Don't retry on 4xx errors (client errors)
        if (error?.status >= 400 && error?.status < 500) {
          return false;
        }
        // Retry up to 2 times for other errors
        return failureCount < 2;
      },
      
      // Retry delay with exponential backoff
      retryDelay: (attemptIndex) => Math.min(1000 * 2 ** attemptIndex, 30000),
      
      // Enable offline-first behavior
      networkMode: 'offlineFirst',
      
      // Refetch on window focus (web) or app foreground (native)
      refetchOnWindowFocus: true,
      
      // Refetch on reconnect
      refetchOnReconnect: true,
      
      // Don't refetch on mount if data is fresh
      refetchOnMount: false,
    },
    
    mutations: {
      // Retry mutations once
      retry: 1,
      
      // Network mode for mutations
      networkMode: 'online',
    },
  },
});

// Export persister for use in React Query Devtools
export { asyncStoragePersister };

// Offline-aware query hook
export const useOfflineQuery = (
  queryKey: string[],
  queryFn: () => Promise<any>,
  options?: {
    enabled?: boolean;
    onSuccess?: (data: any) => void;
    onError?: (error: any) => void;
  }
) => {
  return queryClient.useQuery({
    queryKey,
    queryFn,
    ...options,
    // Always enabled for offline-first
    enabled: options?.enabled !== false,
  });
};

// Prefetching utility for offline preparation
export const prefetchForOffline = async (
  queryKey: string[],
  queryFn: () => Promise<any>
) => {
  await queryClient.prefetchQuery({
    queryKey,
    queryFn,
    staleTime: 1000 * 60 * 60, // 1 hour
  });
};

// Invalidate and refetch utility
export const invalidateAndRefetch = async (queryKey: string[]) => {
  await queryClient.invalidateQueries({ queryKey });
  await queryClient.refetchQueries({ queryKey });
};

// Clear all cache (for logout)
export const clearQueryCache = () => {
  queryClient.clear();
  // Also clear AsyncStorage
  AsyncStorage.removeItem('react-query-cache');
};
