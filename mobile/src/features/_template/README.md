# Feature Template

Copy this folder to create a new feature.

## Usage

```bash
# Copy the template
cp -r mobile/src/features/_template mobile/src/features/your-feature-name
```

## Structure

```
your-feature-name/
├── components/          # Feature-specific components
│   ├── YourComponent.tsx
│   └── index.ts         # Barrel export
├── screens/             # Screen components
│   ├── YourScreen.tsx
│   └── index.ts         # Barrel export
├── hooks/               # Local UI state hooks (optional)
│   └── useYourHook.ts
└── index.ts             # Feature barrel export
```

## Checklist

- [ ] Rename all `Template` references to your feature name
- [ ] Add API hooks to `mobile/src/api/client.ts`
- [ ] Add types to `mobile/src/api/types/index.ts`
- [ ] Update navigation to include new screens
- [ ] Handle all 6 UI states in screens
