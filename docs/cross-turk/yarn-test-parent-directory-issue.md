# Yarn Test Issue: Parent Directory Name with Hyphens

## Issue Description

When running `yarn test` in a subdirectory of a project where the parent directory name contains hyphens, yarn fails with the error:

```
error ../package.json: Name contains illegal characters
```

## Reproduction Steps

1. **Create project structure with hyphenated parent directory:**

   ```bash
   mkdir -p /path/to/ic-1inch/eth
   cd /path/to/ic-1inch/eth
   ```

2. **Add package.json in eth subdirectory:**

   ```json
   {
     "name": "@1inch/cross-chain-swap",
     "scripts": {
       "test": "FOUNDRY_PROFILE=default forge snapshot --no-match-test \"testFuzz_*\""
     }
   }
   ```

3. **Add package.json in parent directory:**

   ```json
   {
     "name": "some-package"
   }
   ```

4. **Run yarn test from eth subdirectory:**
   ```bash
   cd /path/to/ic-1inch/eth
   yarn test
   ```

## Error Output

```
yarn run v1.22.18
error ../package.json: Name contains illegal characters
info Visit https://yarnpkg.com/en/docs/cli/run for documentation about this command.
```

## Root Cause Analysis

The issue occurs because:

1. **Yarn reads parent package.json**: When running `yarn test` from a subdirectory, yarn attempts to read `../package.json` (parent directory's package.json)

2. **Parent directory name contains hyphen**: The parent directory `ic-1inch` contains a hyphen (`-`)

3. **Yarn considers hyphen illegal**: Yarn treats the parent directory name as a package name and considers hyphens "illegal characters" for package names

## Workarounds

### 1. Clean Repository Clone (Recommended)

```bash
# Clone to directory without hyphens
git clone https://github.com/1inch/cross-chain-swap.git ~/cross-chain-swap
cd ~/cross-chain-swap
yarn test  # ✅ Works
```

### 2. Rename Parent Directory

```bash
# Rename parent directory to remove hyphen
mv /path/to/ic-1inch /path/to/ic1inch
cd /path/to/ic1inch/eth
yarn test  # ✅ Should work
```

### 3. Run Forge Command Directly

```bash
# Bypass yarn entirely
FOUNDRY_PROFILE=default forge snapshot --no-match-test "testFuzz_*"
```

## Technical Details

- **Yarn Version**: 1.22.18
- **Node Version**: (system default)
- **OS**: macOS (darwin 23.4.0)
- **Project**: 1inch cross-chain-swap repository

## Questions for Senior Developer

1. **Is this expected behavior?** Should yarn be reading parent package.json files when running from subdirectories?

2. **Why does yarn treat directory names as package names?** The error suggests yarn is parsing the parent directory name as a package identifier.

3. **Is there a yarn configuration to prevent this?** Can we configure yarn to only read the current directory's package.json?

4. **Best practices for project structure?** Should we avoid hyphens in parent directory names when using yarn workspaces or subdirectories?

5. **Alternative solutions?** Are there yarn flags or configurations to work around this issue?

## Related Files

- `package.json` in eth subdirectory
- Parent directory's `package.json` (if exists)
- Yarn workspace configuration (if applicable)

## Environment

- **Working Directory**: `/Users/stefano/cross-chain-swap` (clean clone)
- **Failing Directory**: `/Users/stefano/Documents/Code/Unite_DeFi/ic-1inch/eth` (original with hyphenated parent)
- **Yarn Version**: 1.22.18
- **Foundry Version**: 1.2.3-stable
