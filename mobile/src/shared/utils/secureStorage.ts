import * as SecureStore from 'expo-secure-store';
import { Platform } from 'react-native';

// Secure storage utilities - NEVER use AsyncStorage for sensitive data
export const secureStorage = {
  // Authentication tokens
  async setToken(token: string): Promise<void> {
    if (Platform.OS === 'web') {
      // Web: Use httpOnly cookies (handled by backend)
      console.warn('Token storage on web should use httpOnly cookies');
      return;
    }
    
    await SecureStore.setItemAsync('auth_token', token, {
      requireAuthentication: true, // Require biometric/passcode on iOS
    });
  },

  async getToken(): Promise<string | null> {
    if (Platform.OS === 'web') {
      // Web: Token is in httpOnly cookie, handled by backend
      return null;
    }
    
    return await SecureStore.getItemAsync('auth_token');
  },

  async removeToken(): Promise<void> {
    if (Platform.OS === 'web') {
      // Web: Call logout endpoint to clear cookie
      await fetch('/api/v1/auth/logout', { method: 'POST' });
      return;
    }
    
    await SecureStore.deleteItemAsync('auth_token');
  },

  // Refresh tokens
  async setRefreshToken(token: string): Promise<void> {
    if (Platform.OS === 'web') {
      console.warn('Refresh token storage on web should use httpOnly cookies');
      return;
    }
    
    await SecureStore.setItemAsync('refresh_token', token, {
      requireAuthentication: true,
    });
  },

  async getRefreshToken(): Promise<string | null> {
    if (Platform.OS === 'web') {
      return null;
    }
    
    return await SecureStore.getItemAsync('refresh_token');
  },

  async removeRefreshToken(): Promise<void> {
    if (Platform.OS === 'web') {
      return;
    }
    
    await SecureStore.deleteItemAsync('refresh_token');
  },

  // Biometric authentication status
  async setBiometricEnabled(enabled: boolean): Promise<void> {
    if (Platform.OS === 'web') return;
    
    await SecureStore.setItemAsync('biometric_enabled', JSON.stringify(enabled));
  },

  async isBiometricEnabled(): Promise<boolean> {
    if (Platform.OS === 'web') return false;
    
    const enabled = await SecureStore.getItemAsync('biometric_enabled');
    return enabled ? JSON.parse(enabled) : false;
  },

  // Clear all secure data (for logout)
  async clearAll(): Promise<void> {
    if (Platform.OS === 'web') {
      await fetch('/api/v1/auth/logout', { method: 'POST' });
      return;
    }
    
    await Promise.all([
      SecureStore.deleteItemAsync('auth_token'),
      SecureStore.deleteItemAsync('refresh_token'),
      SecureStore.deleteItemAsync('biometric_enabled'),
    ]);
  },
};

// Token validation utilities
export const tokenUtils = {
  isTokenExpired(token: string): boolean {
    try {
      const payload = JSON.parse(atob(token.split('.')[1]));
      return payload.exp * 1000 < Date.now();
    } catch {
      return true; // Invalid token
    }
  },

  getTokenExpiration(token: string): Date | null {
    try {
      const payload = JSON.parse(atob(token.split('.')[1]));
      return new Date(payload.exp * 1000);
    } catch {
      return null;
    }
  },

  shouldRefreshToken(token: string, bufferMinutes: number = 5): boolean {
    const expiration = this.getTokenExpiration(token);
    if (!expiration) return true;
    
    const now = new Date();
    const bufferTime = new Date(expiration.getTime() - bufferMinutes * 60 * 1000);
    
    return now >= bufferTime;
  },
};
