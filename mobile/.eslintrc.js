// ==============================================================================
// ESLINT CONFIGURATION
// ==============================================================================
//
// This config enforces code quality and catches common errors.
// Run with: npm run lint
//
// ==============================================================================

module.exports = {
  root: true,
  extends: [
    'expo',
    'prettier',
  ],
  plugins: ['prettier'],
  rules: {
    // ==========================================================================
    // SECURITY RULES
    // ==========================================================================
    
    // Prevent console.log in production (use proper logging)
    'no-console': ['warn', { allow: ['warn', 'error'] }],
    
    // ==========================================================================
    // CODE QUALITY
    // ==========================================================================
    
    // Enforce prettier formatting
    'prettier/prettier': 'error',
    
    // Prevent unused variables (except those prefixed with _)
    '@typescript-eslint/no-unused-vars': ['error', { 
      argsIgnorePattern: '^_',
      varsIgnorePattern: '^_',
    }],
    
    // Require explicit return types on functions
    '@typescript-eslint/explicit-function-return-type': 'off',
    
    // Allow any type (sometimes needed for external libs)
    '@typescript-eslint/no-explicit-any': 'warn',
  },
  ignorePatterns: [
    'node_modules/',
    '.expo/',
    'dist/',
    'web-build/',
  ],
};
