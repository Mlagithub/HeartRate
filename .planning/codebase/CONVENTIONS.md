# Coding Conventions

**Analysis Date:** 2026-03-21

## Naming Patterns

**Files:**
- Svelte components: PascalCase with `.svelte` extension (e.g., `HeartRateDisplay.svelte`)
- TypeScript modules: camelCase with `.ts` extension (e.g., `heartRate.ts`)
- Index/barrel files: `index.ts`
- Rust modules: snake_case (e.g., `heart_rate.rs`)

**TypeScript Functions:**
- camelCase for function names: `createDeviceStore()`, `startScan()`
- Async functions prefixed with action verbs: `loadSettings()`, `initListeners()`
- Factory functions use `create` prefix: `createHeartRateStore()`

**TypeScript Variables:**
- camelCase for variables and parameters: `deviceMap`, `sessionId`
- UPPER_SNAKE_CASE for constants: not observed (inline values used)

**TypeScript Types:**
- PascalCase for interfaces and types: `DeviceInfo`, `HeartRateState`, `ConnectionState`
- Union types for state enums: `type ConnectionState = 'Disconnected' | 'Connecting' | 'Connected' | ...`

**Rust Naming:**
- snake_case for functions, variables, modules: `start_scan()`, `get_connection_state()`
- PascalCase for structs, enums, traits: `DeviceInfo`, `ConnectionState`, `BleManager`
- SCREAMING_SNAKE_CASE for constants: `HEART_RATE_SERVICE_UUID`, `ALERT_COOLDOWN_SECS`

**Cross-Language Property Names:**
- Properties that serialize to/from Rust use snake_case: `low_threshold`, `high_threshold`, `supports_heart_rate`
- Frontend-only properties use camelCase: `isScanning`, `maxHistoryLength`

## Code Style

**Formatting:**
- No explicit formatter config (no `.prettierrc` detected)
- TypeScript uses 2-space indentation
- Svelte components use 2-space indentation
- Rust follows standard `cargo fmt` conventions (4-space indentation)

**Linting:**
- No ESLint configuration detected
- TypeScript compiler provides type checking
- `svelte-check` for Svelte validation

**TypeScript Strict Mode:**
```json
// tsconfig.json
{
  "compilerOptions": {
    "strict": true,
    "forceConsistentCasingInFileNames": true,
    "noImplicitAny": true
  }
}
```

## Import Organization

**Order:**
1. External packages (Svelte, Tauri APIs)
2. Internal aliases (`$lib/...`)
3. Relative imports

**Pattern (TypeScript):**
```typescript
import { writable } from 'svelte/store';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';

import HeartRateDisplay from '$lib/components/HeartRateDisplay.svelte';
import { heartRate } from '$lib/stores/heartRate';
```

**Pattern (Rust):**
```rust
use crate::ble::{DeviceInfo, ConnectionState};
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use tauri::{AppHandle, Emitter};
```

**Path Aliases:**
- `$lib` maps to `src/lib/` (SvelteKit standard alias)

## Error Handling

**TypeScript Patterns:**
- Try-catch with console.error for logging:
```typescript
try {
  const device = await invoke<DeviceInfo>('get_connected_device');
} catch (e) {
  console.error('Failed to get connected device:', e);
}
```
- Error state in stores:
```typescript
interface HistoryState {
  records: HeartRateRecord[];
  isLoading: boolean;
  error: string | null;
}
```

**Rust Patterns:**
- `Result<T, String>` for Tauri commands:
```rust
pub async fn start_scan(...) -> Result<(), String> {
    // ...
    .map_err(|e| format!("Failed to start scan: {}", e))
}
```
- `anyhow::Result` for internal operations (dependency available)
- `thiserror` for custom error types (dependency available)
- Log errors before returning:
```rust
log::error!("Failed to connect: {}", e);
return Err(format!("Failed to connect: {}", e));
```

## Logging

**TypeScript:**
- Uses `console.error()` and `console.debug()` for development logging
- No structured logging framework

**Rust:**
- Uses `log` crate with `env_logger`
- Log levels: `log::info!()`, `log::error!()`, `log::debug!()`, `log::warn!()`
- Default filter level: `info`
- Initialization in `main.rs`:
```rust
env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
    .init();
```

## Comments

**When to Comment:**
- Doc comments for public Rust APIs using `///`
- Inline comments for non-obvious logic (e.g., performance optimizations)

**Rust Doc Comments:**
```rust
/// BLE Manager state
pub struct BleManagerInner {
    /// Bluetooth adapter
    adapter: Option<Adapter>,
    /// Currently connected peripheral
    connected_peripheral: Option<Peripheral>,
}
```

**TypeScript JSDoc:**
- Not extensively used
- Type annotations serve as documentation

## Function Design

**Size:** Functions generally under 30 lines; complex operations split into helper functions

**Parameters:**
- TypeScript: Object parameters for multiple args when calling Tauri:
```typescript
await invoke('save_heart_rate', { measurement, sessionId });
```
- Rust: Multiple parameters with clear types:
```rust
pub async fn save_heart_rate(
    db: State<'_, Database>,
    measurement: HeartRateMeasurement,
    session_id: String,
) -> Result<i64, String>
```

**Return Values:**
- Async functions return `Promise<T>`
- Store factory functions return object with `subscribe` plus custom methods

## Module Design

**Exports:**
- Named exports preferred over default exports for utilities
- Default exports for Svelte components
- Barrel files for component aggregation

**Barrel Files:**
```typescript
// src/lib/components/index.ts
export { default as HeartRateDisplay } from './HeartRateDisplay.svelte';
export { default as HeartRateChart } from './HeartRateChart.svelte';
```

```typescript
// src/lib/stores/index.ts
export { heartRate } from './heartRate';
export { device } from './device';
```

**Rust Module Pattern:**
```rust
// src/ble/mod.rs
pub mod device;
pub mod heart_rate;
pub mod manager;

pub use device::*;
pub use heart_rate::*;
pub use manager::*;
```

## Store Pattern (Svelte)

**Factory Function Pattern:**
```typescript
function createHeartRateStore() {
  const { subscribe, update, set } = writable<HeartRateState>(initialState);

  return {
    subscribe,
    initListener: async () => { /* ... */ },
    cleanup: () => { /* ... */ },
    reset: () => set(initialState),
  };
}

export const heartRate = createHeartRateStore();
```

**Lifecycle:**
- `initListeners()` called in `onMount()`
- `cleanup()` returned from `onMount()` for automatic cleanup
- `reset()` for manual state reset

## Component Pattern (Svelte)

**Structure:**
```svelte
<script lang="ts">
  // Imports
  // Props (export let)
  // Reactive declarations ($:)
  // Functions
</script>

<!-- Template -->

<style>
  /* Scoped styles */
</style>
```

**Reactive Declarations:**
```typescript
$: currentBpm = $heartRate.current?.bpm ?? null;
$: isConnected = $device.connectionState === 'Connected';
```

## CSS Conventions

**CSS Variables:**
- Defined in `:root` in `src/app.css`
- Theme variables: `--primary-color`, `--bg-color`, `--text-primary`, etc.
- Status colors: `--success-color`, `--danger-color`, `--warning-color`
- Component-specific values passed via style props

**Utility Classes:**
- `.glass-card` - Card with blur backdrop
- `.btn-primary`, `.btn-secondary`, `.btn-danger` - Button variants

---

*Convention analysis: 2026-03-21*