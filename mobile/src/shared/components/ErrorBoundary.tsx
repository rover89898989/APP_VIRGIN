import React, { Component, ErrorInfo, ReactNode } from 'react';
import { View, Text, Pressable, ScrollView } from 'react-native';

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
  onError?: (error: Error, errorInfo: ErrorInfo) => void;
}

interface State {
  hasError: boolean;
  error: Error | null;
  errorInfo: ErrorInfo | null;
}

export class ErrorBoundary extends Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = { hasError: false, error: null, errorInfo: null };
  }

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error, errorInfo: null };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    // Log to monitoring service (Sentry, etc.)
    console.error('ErrorBoundary caught:', error, errorInfo);
    
    // In production, send to error reporting service
    if (!__DEV__) {
      // Example: Sentry.captureException(error, { contexts: { react: { componentStack: errorInfo.componentStack } } });
    }
    
    this.props.onError?.(error, errorInfo);
    this.setState({ errorInfo });
  }

  handleRetry = () => {
    this.setState({ hasError: false, error: null, errorInfo: null });
  };

  render() {
    if (this.state.hasError) {
      if (this.props.fallback) {
        return this.props.fallback;
      }

      return (
        <ScrollView className="flex-1 bg-background">
          <View className="flex-1 items-center justify-center p-6 min-h-screen">
            <View className="bg-red-50 dark:bg-red-900/20 rounded-2xl p-6 w-full max-w-sm">
              <Text className="text-2xl font-bold text-red-600 dark:text-red-400 mb-2 text-center">
                Something went wrong
              </Text>
              
              <Text className="text-gray-600 dark:text-gray-300 mb-6 text-center leading-relaxed">
                We encountered an unexpected error. This has been reported to our team.
              </Text>

              {__DEV__ && this.state.error && (
                <View className="bg-gray-100 dark:bg-gray-800 rounded-lg p-4 mb-6">
                  <Text className="text-xs font-mono text-gray-800 dark:text-gray-200 mb-2">
                    Error: {this.state.error.message}
                  </Text>
                  {this.state.errorInfo && (
                    <Text className="text-xs font-mono text-gray-600 dark:text-gray-400">
                      Component Stack: {this.state.errorInfo.componentStack}
                    </Text>
                  )}
                </View>
              )}

              <Pressable
                onPress={this.handleRetry}
                className="bg-primary-500 hover:bg-primary-600 rounded-xl px-6 py-3 items-center"
              >
                <Text className="text-white font-semibold">Try Again</Text>
              </Pressable>

              {!__DEV__ && (
                <Pressable
                  onPress={() => {
                    // In production, you might want to contact support
                    // Linking.openURL('mailto:support@yourapp.com');
                  }}
                  className="mt-4"
                >
                  <Text className="text-primary-500 text-center">Contact Support</Text>
                </Pressable>
              )}
            </View>
          </View>
        </ScrollView>
      );
    }

    return this.props.children;
  }
}

// HOC for wrapping components
export const withErrorBoundary = <P extends object>(
  Component: React.ComponentType<P>,
  errorBoundaryProps?: Omit<Props, 'children'>
) => {
  const WrappedComponent = (props: P) => (
    <ErrorBoundary {...errorBoundaryProps}>
      <Component {...props} />
    </ErrorBoundary>
  );

  WrappedComponent.displayName = `withErrorBoundary(${Component.displayName || Component.name})`;
  
  return WrappedComponent;
};
