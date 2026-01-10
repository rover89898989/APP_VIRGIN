# 📱 Platform-Specific Issues Guide - APP_VIRGIN Template

**Last Updated:** January 10, 2026  
**Purpose:** Platform-specific development issues and solutions

---

## 📋 Table of Contents

1. [iOS vs Android Differences](#ios-vs-android-differences)
2. [Simulator vs Emulator Issues](#simulator-vs-emulator-issues)
3. [Physical Device Testing](#physical-device-testing)
4. [Platform-Specific APIs](#platform-specific-apis)
5. [Build Configurations](#build-configurations)
6. [Common Platform Issues](#common-platform-issues)

---

## 🍎 iOS vs Android Differences

### URL Handling

```typescript
// File: mobile/src/utils/platform-api.ts
import { Platform } from 'react-native';

export const getApiBaseUrl = (): string => {
  if (__DEV__) {
    if (Platform.OS === 'ios') {
      // iOS Simulator uses localhost
      return 'http://localhost:8000';
    } else if (Platform.OS === 'android') {
      // Android Emulator uses 10.0.2.2
      return 'http://10.0.2.2:8000';
    } else {
      // Web/other platforms
      return 'http://localhost:8000';
    }
  } else {
    // Production URL
    return 'https://api.yourdomain.com';
  }
};
```

### Storage Differences

```typescript
// File: mobile/src/utils/storage.ts
import { Platform } from 'react-native';
import * as SecureStore from 'expo-secure-store';

export class TokenStorage {
  static async setToken(token: string): Promise<void> {
    if (Platform.OS === 'web') {
      // Web: Use httpOnly cookies (handled by backend)
      // Tokens are stored automatically by browser
      return;
    } else {
      // Native: Use SecureStore
      await SecureStore.setItemAsync('auth_token', token);
    }
  }
  
  static async getToken(): Promise<string | null> {
    if (Platform.OS === 'web') {
      // Web: Token is in httpOnly cookie
      // Backend handles token extraction
      return null;
    } else {
      // Native: Get from SecureStore
      return await SecureStore.getItemAsync('auth_token');
    }
  }
  
  static async removeToken(): Promise<void> {
    if (Platform.OS === 'web') {
      // Web: Call logout endpoint to clear cookie
      await fetch('/api/v1/auth/logout', { method: 'POST' });
    } else {
      // Native: Remove from SecureStore
      await SecureStore.deleteItemAsync('auth_token');
    }
  }
}
```

### Platform-Specific Components

```typescript
// File: mobile/src/components/PlatformButton.tsx
import React from 'react';
import { TouchableOpacity, Text, Platform, StyleSheet } from 'react-native';

interface PlatformButtonProps {
  title: string;
  onPress: () => void;
}

export const PlatformButton: React.FC<PlatformButtonProps> = ({ title, onPress }) => {
  const buttonStyle = [
    styles.button,
    Platform.OS === 'ios' ? styles.iosButton : styles.androidButton,
  ];
  
  const textStyle = [
    styles.text,
    Platform.OS === 'ios' ? styles.iosText : styles.androidText,
  ];
  
  return (
    <TouchableOpacity style={buttonStyle} onPress={onPress}>
      <Text style={textStyle}>{title}</Text>
    </TouchableOpacity>
  );
};

const styles = StyleSheet.create({
  button: {
    paddingVertical: 12,
    paddingHorizontal: 24,
    borderRadius: 8,
    alignItems: 'center',
    justifyContent: 'center',
  },
  iosButton: {
    backgroundColor: '#007AFF',
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
  },
  androidButton: {
    backgroundColor: '#2196F3',
    elevation: 4,
  },
  text: {
    fontSize: 16,
    fontWeight: '600',
  },
  iosText: {
    color: '#FFFFFF',
  },
  androidText: {
    color: '#FFFFFF',
  },
});
```

---

## 🖥 Simulator vs Emulator Issues

### Network Connectivity

#### iOS Simulator

```bash
# iOS Simulator network troubleshooting
# 1. Check if localhost is accessible
curl http://localhost:8000/health/live

# 2. If not working, try 127.0.0.1
curl http://127.0.0.1:8000/health/live

# 3. Reset network settings in Simulator
# Device → Erase All Content and Settings
```

#### Android Emulator

```bash
# Android Emulator network troubleshooting
# 1. Check if 10.0.2.2 works (host machine IP)
curl http://10.0.2.2:8000/health/live

# 2. If not working, try 10.0.2.3
curl http://10.0.2.3:8000/health/live

# 3. Restart emulator with network settings
emulator -avd <AVD_NAME> -dns-server 8.8.8.8
```

### Platform-Specific Debugging

```typescript
// File: mobile/src/utils/platform-debug.ts
import { Platform } from 'react-native';

export const PlatformDebugger = {
  logNetworkRequest: (url: string, method: string) => {
    if (__DEV__) {
      const platform = Platform.OS;
      const resolvedUrl = platform === 'android' 
        ? url.replace('localhost', '10.0.2.2')
        : url;
      
      console.log(`[${platform.toUpperCase()}] ${method} ${resolvedUrl}`);
    }
  },
  
  logPlatformInfo: () => {
    if (__DEV__) {
      console.log('Platform Info:', {
        OS: Platform.OS,
        Version: Platform.Version,
        isPad: Platform.isPad,
        isTV: Platform.isTV,
        Constants: Platform.Constants,
      });
    }
  },
};
```

---

## 📱 Physical Device Testing

### iOS Physical Device Setup

```bash
# 1. Install Xcode Command Line Tools
xcode-select --install

# 2. Check device connection
xcrun xctrace list devices

# 3. Trust development certificate
# Settings → General → VPN & Device Management → Trust Developer

# 4. Enable developer mode on device (iOS 16+)
# Settings → Privacy & Security → Developer Mode
```

### Android Physical Device Setup

```bash
# 1. Enable USB debugging
# Settings → About Phone → Tap "Build number" 7 times
# Settings → Developer options → USB debugging

# 2. Check device connection
adb devices

# 3. If device not listed, restart adb server
adb kill-server
adb start-server
adb devices

# 4. Install app on device
cd mobile
npx expo run:android
```

### Device-Specific Configuration

```typescript
// File: mobile/src/config/device-config.ts
import { Platform, Dimensions } from 'react-native';

export const DeviceConfig = {
  isTablet: () => {
    const { width, height } = Dimensions.get('window');
    const aspectRatio = width / height;
    
    return Platform.OS === 'ios' 
      ? Platform.isPad 
      : aspectRatio > 1.6; // Android tablet heuristic
  },
  
  getLayoutBreakpoints: () => ({
    small: Platform.OS === 'ios' ? 375 : 360,
    medium: Platform.OS === 'ios' ? 414 : 480,
    large: Platform.OS === 'ios' ? 768 : 600,
  }),
  
  getSafeAreaInsets: () => {
    // Platform-specific safe area handling
    if (Platform.OS === 'ios') {
      return {
        top: 44, // Status bar + notch
        bottom: 34, // Home indicator
      };
    } else {
      return {
        top: 24, // Status bar
        bottom: 0,
      };
    }
  },
};
```

---

## 🔧 Platform-Specific APIs

### iOS-Specific Features

```typescript
// File: mobile/src/utils/ios-features.ts
import { Platform } from 'react-native';

export const IOSFeatures = {
  // Face ID / Touch ID
  authenticate: async (): Promise<boolean> => {
    if (Platform.OS !== 'ios') return false;
    
    try {
      const LocalAuthentication = require('expo-local-authentication');
      const compatible = await LocalAuthentication.isDeviceAvailableAsync();
      
      if (compatible) {
        const result = await LocalAuthentication.authenticateAsync({
          promptMessage: 'Authenticate to continue',
        });
        return result.success;
      }
    } catch (error) {
      console.error('Authentication failed:', error);
    }
    
    return false;
  },
  
  // iOS-specific permissions
  requestPermissions: async () => {
    if (Platform.OS === 'ios') {
      const { Permissions } = require('expo-permissions');
      
      // Request iOS-specific permissions
      await Permissions.askAsync(Permissions.CAMERA);
      await Permissions.askAsync(Permissions.PHOTOS);
    }
  },
};
```

### Android-Specific Features

```typescript
// File: mobile/src/utils/android-features.ts
import { Platform } from 'react-native';

export const AndroidFeatures = {
  // Android permissions
  requestPermissions: async () => {
    if (Platform.OS === 'android') {
      const { Permissions } = require('expo-permissions');
      
      // Request Android-specific permissions
      await Permissions.askAsync(Permissions.CAMERA);
      await Permissions.askAsync(Permissions.ANDROID_RECORD_AUDIO);
    }
  },
  
  // Android-specific UI
  getStatusBarHeight: (): number => {
    if (Platform.OS === 'android') {
      return Platform.Version >= 23 ? 24 : 25;
    }
    return 0;
  },
};
```

---

## 🏗 Build Configurations

### iOS Build Configuration

```json
// File: mobile/app.json (iOS specific)
{
  "expo": {
    "ios": {
      "bundleIdentifier": "com.yourcompany.appvirgin",
      "buildNumber": "1.0.0",
      "supportsTablet": true,
      "infoPlist": {
        "NSCameraUsageDescription": "This app uses the camera for profile photos",
        "NSPhotoLibraryUsageDescription": "This app accesses photos for profile images",
        "NSFaceIDUsageDescription": "This app uses Face ID for authentication"
      },
      "config": {
        "usesNonExemptEncryption": false
      }
    }
  }
}
```

### Android Build Configuration

```json
// File: mobile/app.json (Android specific)
{
  "expo": {
    "android": {
      "package": "com.yourcompany.appvirgin",
      "versionCode": 1,
      "compileSdkVersion": 33,
      "targetSdkVersion": 33,
      "adaptiveIcon": {
        "foregroundImage": "./assets/adaptive-icon.png",
        "backgroundColor": "#FFFFFF"
      },
      "permissions": [
        "CAMERA",
        "READ_EXTERNAL_STORAGE",
        "WRITE_EXTERNAL_STORAGE"
      ]
    }
  }
}
```

### Platform-Specific Build Scripts

```bash
#!/bin/bash
# File: scripts/build-platform.sh

PLATFORM=$1
ENVIRONMENT=$2

case $PLATFORM in
  "ios")
    echo "Building for iOS..."
    cd mobile
    if [ "$ENVIRONMENT" = "production" ]; then
      eas build --platform ios --profile production
    else
      eas build --platform ios --profile preview
    fi
    ;;
    
  "android")
    echo "Building for Android..."
    cd mobile
    if [ "$ENVIRONMENT" = "production" ]; then
      eas build --platform android --profile production
    else
      eas build --platform android --profile preview
    fi
    ;;
    
  "web")
    echo "Building for Web..."
    cd mobile
    npm run build
    ;;
    
  *)
    echo "Usage: $0 [ios|android|web] [development|staging|production]"
    exit 1
    ;;
esac
```

---

## 🐛 Common Platform Issues

### iOS Issues

#### Status Bar Overlap

```typescript
// File: mobile/src/components/SafeAreaView.tsx
import React from 'react';
import { View, StyleSheet, Platform } from 'react-native';
import { SafeAreaView } from 'react-native-safe-area-context';

interface SafeAreaViewProps {
  children: React.ReactNode;
}

export const CustomSafeAreaView: React.FC<SafeAreaViewProps> = ({ children }) => {
  if (Platform.OS === 'ios') {
    return (
      <SafeAreaView style={styles.container} edges={['top', 'left', 'right']}>
        {children}
      </SafeAreaView>
    );
  }
  
  return <View style={styles.container}>{children}</View>;
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
});
```

#### Keyboard Handling

```typescript
// File: mobile/src/hooks/useKeyboard.ts
import { useState, useEffect } from 'react';
import { Keyboard, Platform } from 'react-native';

export const useKeyboard = () => {
  const [keyboardHeight, setKeyboardHeight] = useState(0);
  
  useEffect(() => {
    const keyboardShowListener = Keyboard.addListener(
      Platform.OS === 'ios' ? 'keyboardWillShow' : 'keyboardDidShow',
      (event) => {
        setKeyboardHeight(event.endCoordinates.height);
      }
    );
    
    const keyboardHideListener = Keyboard.addListener(
      Platform.OS === 'ios' ? 'keyboardWillHide' : 'keyboardDidHide',
      () => {
        setKeyboardHeight(0);
      }
    );
    
    return () => {
      keyboardShowListener.remove();
      keyboardHideListener.remove();
    };
  }, []);
  
  return { keyboardHeight };
};
```

### Android Issues

#### Back Button Handling

```typescript
// File: mobile/src/hooks/useBackButton.ts
import { useEffect } from 'react';
import { BackHandler, Platform } from 'react-native';

export const useBackButton = (handler: () => boolean) => {
  useEffect(() => {
    if (Platform.OS === 'android') {
      const backHandler = BackHandler.addEventListener('hardwareBackPress', handler);
      
      return () => backHandler.remove();
    }
  }, [handler]);
};
```

#### Permission Handling

```typescript
// File: mobile/src/utils/permissions.ts
import { Platform } from 'react-native';
import { Permissions } from 'expo-permissions';

export const requestPermission = async (permission: string): Promise<boolean> => {
  if (Platform.OS === 'android') {
    // Android requires explicit permission requests
    const { status } = await Permissions.askAsync(permission);
    return status === 'granted';
  } else {
    // iOS permissions are handled differently
    const { status } = await Permissions.askAsync(permission);
    return status === 'granted';
  }
};
```

### Cross-Platform Issues

#### Font Rendering

```typescript
// File: mobile/src/theme/fonts.ts
import { Platform } from 'react-native';

export const Fonts = {
  // Platform-specific font families
  regular: Platform.select({
    ios: 'SF Pro Text',
    android: 'Roboto',
    default: 'System',
  }),
  
  medium: Platform.select({
    ios: 'SF Pro Text Medium',
    android: 'Roboto Medium',
    default: 'System',
  }),
  
  // Platform-specific font sizes
  adjustSize: (size: number) => {
    if (Platform.OS === 'android') {
      return size * 1.1; // Android fonts often appear smaller
    }
    return size;
  },
};
```

#### Gesture Handling

```typescript
// File: mobile/src/components/GestureHandler.tsx
import React from 'react';
import { PanGestureHandler, State } from 'react-native-gesture-handler';
import { Platform } from 'react-native';

export const PlatformGestureHandler: React.FC = ({ children }) => {
  const onGestureEvent = (event: any) => {
    if (Platform.OS === 'ios') {
      // iOS gesture handling
      // iOS has different gesture recognition patterns
    } else {
      // Android gesture handling
      // Android gestures may need different thresholds
    }
  };
  
  return (
    <PanGestureHandler onGestureEvent={onGestureEvent}>
      {children}
    </PanGestureHandler>
  );
};
```

---

## 🧪 Platform Testing Checklist

### iOS Testing

- [ ] Test on iPhone (various screen sizes)
- [ ] Test on iPad (if supported)
- [ ] Test Face ID / Touch ID authentication
- [ ] Test push notifications
- [ ] Test background app refresh
- [ ] Test memory management (low memory warnings)
- [ ] Test network connectivity changes

### Android Testing

- [ ] Test on various Android versions (API 24-33)
- [ ] Test on different screen densities
- [ ] Test fingerprint authentication
- [ ] Test push notifications
- [ ] Test background services
- [ ] Test memory management
- [ ] Test network connectivity changes

### Web Testing

- [ ] Test on Chrome, Firefox, Safari
- [ ] Test responsive design
- [ ] Test keyboard navigation
- [ ] Test touch vs mouse interactions
- [ ] Test browser storage limitations
- [ ] Test CORS policies

---

## 📚 Platform Resources

### Documentation Links

- [React Native Platform Specific Code](https://reactnative.dev/docs/platform-specific-code)
- [Expo Platform Differences](https://docs.expo.dev/workflow/platform/)
- [iOS Human Interface Guidelines](https://developer.apple.com/design/human-interface-guidelines/)
- [Android Material Design](https://material.io/design/)

### Debugging Tools

- **iOS**: Xcode Console, Instruments
- **Android**: Android Studio Logcat, Layout Inspector
- **Web**: Chrome DevTools, React DevTools

---

**Happy Cross-Platform Development! 📱**

Remember: Test early, test often, and test on real devices.
