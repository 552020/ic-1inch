# Hardhat TypeScript Configuration Issues

## Overview

This document outlines the persistent TypeScript and ESLint configuration issues encountered in the Hardhat project (`src/eth/`) that are blocking development progress.

## Current Status

- ✅ **Task 1.5**: Set up Hardhat project (COMPLETED)
- ✅ **Task 1.6**: Extend React frontend for fusion functionality (COMPLETED)
- ❌ **Task 1.7**: Implement MetaMask integration (BLOCKED by TypeScript errors)

## Issues Encountered

### 1. TypeScript Module Import Errors

**Error**: `Module '"hardhat"' has no exported member 'ethers'`

- **Files affected**: `scripts/deploy.ts`, `scripts/setup-test-env.ts`, `test/FusionEscrow.test.ts`
- **Root cause**: TypeScript can't find the `ethers` export from Hardhat
- **Attempted fixes**:
  - ✅ Fixed `tsconfig.json` to include `typechain-types`
  - ✅ Regenerated typechain types with `npx hardhat typechain`
  - ❌ Issue persists despite proper Hardhat toolbox installation

### 2. Testing Assertion Library Errors

**Error**: `Property 'emit' does not exist on type 'Assertion'`
**Error**: `Property 'revertedWith' does not exist on type 'Assertion'`
**Error**: `Property 'revertedWithCustomError' does not exist on type 'Assertion'`

- **Files affected**: `test/FusionEscrow.test.ts`
- **Root cause**: Chai/Waffle assertion types not properly configured
- **Impact**: All test files have 17+ TypeScript errors

### 3. ESLint Configuration Conflicts

**Error**: ESLint was configured to run using `parserOptions.project` but TypeScript configs don't include the files

- **Root cause**: Global ESLint config (`../../eslint.config.js`) conflicts with local Hardhat project
- **Attempted fixes**:
  - ✅ Created local `eslint.config.js` with CommonJS syntax
  - ✅ Removed deprecated `.eslintignore` and `.eslintrc.js`
  - ✅ Configured proper ignores for `node_modules`, `dist`, `typechain-types`
  - ❌ Still getting parsing errors

### 4. TypeScript Configuration Issues

**Error**: `Property 'getBalance' does not exist on type 'HardhatEthersSigner'`

- **Root cause**: Ethers.js API changes in newer versions
- **Fix applied**: ✅ Changed to `ethers.provider.getBalance(deployer.address)`

## Technical Details

### Current Configuration Files

#### `src/eth/tsconfig.json`

```json
{
  "compilerOptions": {
    "target": "es2020",
    "module": "commonjs",
    "esModuleInterop": true,
    "forceConsistentCasingInFileNames": true,
    "strict": true,
    "skipLibCheck": true,
    "resolveJsonModule": true,
    "outDir": "dist",
    "rootDir": ".",
    "baseUrl": ".",
    "paths": {
      "@/*": ["./*"]
    }
  },
  "include": ["scripts/**/*", "test/**/*", "hardhat.config.cjs"],
  "exclude": ["node_modules", "dist"]
}
```

#### `src/eth/hardhat.config.cjs`

```javascript
const { HardhatUserConfig } = require("hardhat/config");
require("@nomicfoundation/hardhat-toolbox");
require("dotenv").config();

const config = {
  solidity: {
    version: "0.8.28",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200,
      },
    },
  },
  // ... networks, etherscan, gasReporter config
};

module.exports = config;
```

#### `src/eth/eslint.config.js`

```javascript
const js = require("@eslint/js");
const tseslint = require("typescript-eslint");

module.exports = tseslint.config(
  { ignores: ["node_modules/**", "dist/**", "typechain-types/**"] },
  {
    extends: [js.configs.recommended, ...tseslint.configs.recommended],
    files: ["**/*.{ts,tsx}"],
    languageOptions: {
      ecmaVersion: 2020,
      sourceType: "module",
    },
    plugins: {
      "@typescript-eslint": tseslint.plugin,
    },
    rules: {
      "@typescript-eslint/no-unused-vars": "error",
      "@typescript-eslint/no-explicit-any": "warn",
      "prefer-const": "error",
    },
  }
);
```

### Dependencies

- `@nomicfoundation/hardhat-toolbox@6.1.0` ✅ Installed
- `hardhat@2.28.0` ✅ Installed
- `typescript@5.0.0` ✅ Installed

## Blocked Tasks

### Task 1.7: Implement MetaMask Integration

**Status**: BLOCKED
**Reason**: Cannot proceed with frontend integration due to TypeScript compilation errors
**Impact**: Delays entire Team B progress

### Task 1.8: Create basic ETH escrow contract

**Status**: BLOCKED  
**Reason**: Contract tests fail due to assertion library errors
**Impact**: Cannot validate contract functionality

## Recommended Solutions

### For Team A (ICP/Backend Focus)

1. **Continue with backend development** - these issues don't affect ICP canister development
2. **Focus on Tasks 1.4, 2.1, 2.2** - cross-chain identity and escrow functionality

### For Team B (Frontend/Ethereum Focus)

1. **Fix TypeScript configuration** - primary blocker
2. **Resolve testing framework issues** - needed for contract validation
3. **Complete MetaMask integration** - core functionality

## Next Steps for Team B

### Priority 1: Fix TypeScript Configuration

1. **Investigate Hardhat toolbox compatibility** with current TypeScript version
2. **Update assertion library types** for Chai/Waffle
3. **Verify ethers.js version compatibility**

### Priority 2: Resolve ESLint Issues

1. **Test different ESLint configurations**
2. **Consider disabling ESLint temporarily** for development
3. **Create isolated TypeScript config** for Hardhat project

### Priority 3: Validate Contract Functionality

1. **Fix test assertions** once TypeScript is working
2. **Run contract tests** to ensure functionality
3. **Deploy to testnet** for integration testing

## Files Requiring Attention

- `src/eth/scripts/deploy.ts` - ethers import error
- `src/eth/scripts/setup-test-env.ts` - ethers import error
- `src/eth/test/FusionEscrow.test.ts` - 17+ assertion errors
- `src/eth/tsconfig.json` - may need additional configuration
- `src/eth/eslint.config.js` - parsing issues

## Environment Information

- **OS**: macOS 23.4.0
- **Node.js**: v18+ (exact version needed)
- **npm**: Latest
- **Hardhat**: 2.28.0
- **TypeScript**: 5.0.0

## Conclusion

The Hardhat project setup is functionally complete but has persistent TypeScript configuration issues that prevent proper development and testing. These issues need to be resolved before Team B can proceed with Ethereum integration tasks.

**Recommendation**: Team B should prioritize fixing these configuration issues before proceeding with Task 1.7 (MetaMask integration).
