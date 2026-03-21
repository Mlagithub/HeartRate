# Testing Patterns

**Analysis Date:** 2026-03-21

## Test Framework

**Runner:**
- Not configured
- No test framework detected in the project

**Assertion Library:**
- None

**Run Commands:**
- No test scripts defined in `package.json`
- Available scripts:
```bash
npm run dev       # Development server
npm run build     # Production build
npm run preview   # Preview production build
npm run check     # Type checking with svelte-check
npm run tauri     # Tauri CLI commands
```

## Test File Organization

**Location:**
- None - no test files exist in the project

**Expected Pattern (not yet implemented):**
```
src/
├── lib/
│   ├── components/
│   │   ├── HeartRateDisplay.svelte
│   │   └── HeartRateDisplay.test.ts  (co-located pattern recommended)
│   └── stores/
│       ├── heartRate.ts
│       └── heartRate.test.ts
```

## Test Structure

**Suite Organization:**
- Not applicable - no tests exist

**Recommended Pattern:**
```typescript
import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';

describe('HeartRateDisplay', () => {
  beforeEach(() => {
    // Setup
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('displays current BPM when connected', () => {
    // Test implementation
  });
});
```

## Mocking

**Framework:** Not configured

**Recommended Patterns:**

**Tauri API Mocks:**
```typescript
// __mocks__/@tauri-apps/api/core.ts
export const invoke = vi.fn();
```

```typescript
// __mocks__/@tauri-apps/api/event.ts
export const listen = vi.fn(() => Promise.resolve(() => {}));
export type UnlistenFn = () => void;
```

**What to Mock:**
- `@tauri-apps/api/core` - `invoke()` for backend calls
- `@tauri-apps/api/event` - `listen()` for event subscriptions
- BLE adapter and peripherals (Rust side)

**What NOT to Mock:**
- Svelte stores (test actual store behavior)
- Utility functions (test actual implementation)
- Data transformations

## Fixtures and Factories

**Test Data:**
- Not implemented

**Recommended Pattern:**
```typescript
// test/fixtures/heartRate.ts
export function createHeartRateMeasurement(overrides = {}) {
  return {
    bpm: 72,
    sensor_contact: true,
    energy_expended: null,
    rr_intervals: [],
    timestamp: Date.now(),
    ...overrides,
  };
}

export function createDeviceInfo(overrides = {}) {
  return {
    id: 'test-device-id',
    name: 'Test Heart Rate Monitor',
    rssi: -60,
    supports_heart_rate: true,
    ...overrides,
  };
}
```

**Location:**
- Recommended: `src/test/fixtures/` or co-located `test/` directory

## Coverage

**Requirements:** None enforced

**Coverage Tool:** Not configured

**Recommended Setup:**
```bash
# Install vitest
npm install -D vitest @vitest/coverage-v8 @testing-library/svelte jsdom

# Add to package.json scripts
"test": "vitest",
"test:coverage": "vitest run --coverage"
```

## Test Types

**Unit Tests:**
- Scope: Individual functions, stores, utilities
- Approach: Test store factory functions, data transformations, pure functions
- Not implemented

**Integration Tests:**
- Scope: Component + store interactions
- Approach: Render components with mocked Tauri APIs
- Not implemented

**E2E Tests:**
- Framework: Not used
- Potential tools: Playwright, WebdriverIO, or Tauri's built-in testing
- Not implemented

## Backend Testing (Rust)

**Framework:** Not configured

**Recommended Setup:**
```toml
# Cargo.toml
[dev-dependencies]
tokio-test = "0.4"
```

**Recommended Pattern:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse_heart_rate_measurement() {
        // Test implementation
    }
}
```

## Common Patterns

**Async Testing (Recommended):**
```typescript
import { waitFor } from '@testing-library/svelte';

it('updates state after connection', async () => {
  const { container } = render(DeviceScanner);

  await waitFor(() => {
    expect(screen.getByText('Connected')).toBeInTheDocument();
  });
});
```

**Error Testing (Recommended):**
```typescript
it('handles connection failure gracefully', async () => {
  vi.mocked(invoke).mockRejectedValue(new Error('Connection failed'));

  // Trigger connection
  // Verify error state is set
});
```

**Store Testing (Recommended):**
```typescript
import { get } from 'svelte/store';
import { heartRate } from '$lib/stores/heartRate';

describe('heartRate store', () => {
  beforeEach(() => {
    heartRate.reset();
  });

  it('adds measurement and updates stats', () => {
    heartRate.addMeasurement({ bpm: 72, timestamp: Date.now() });
    const state = get(heartRate);

    expect(state.current?.bpm).toBe(72);
    expect(state.stats.count).toBe(1);
  });
});
```

## Test Coverage Gaps

**Untested Areas:**
- All Svelte components in `src/lib/components/`
- All stores in `src/lib/stores/`
- Utility functions in `src/lib/utils/tauri.ts`
- All Rust modules in `src-tauri/src/`

**Priority Areas for Testing:**
1. **High Priority:**
   - `src/lib/stores/heartRate.ts` - Core business logic
   - `src-tauri/src/ble/heart_rate.rs` - Heart rate parsing
   - `src-tauri/src/storage/database.rs` - Data persistence

2. **Medium Priority:**
   - `src/lib/stores/device.ts` - Device connection state
   - `src/lib/components/HeartRateDisplay.svelte` - Main UI component

3. **Lower Priority:**
   - Visual components with minimal logic
   - Configuration code

## Recommendations

**Immediate Actions:**
1. Add Vitest as test framework
2. Configure `vitest.config.ts`
3. Add Tauri API mocks
4. Create test fixtures for common data types
5. Add test scripts to `package.json`

**Suggested Vitest Config:**
```typescript
// vitest.config.ts
import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';

export default defineConfig({
  plugins: [sveltekit()],
  test: {
    include: ['src/**/*.{test,spec}.{js,ts}'],
    globals: true,
    environment: 'jsdom',
  },
});
```

---

*Testing analysis: 2026-03-21*