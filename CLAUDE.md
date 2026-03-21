<!-- GSD:project-start source:PROJECT.md -->
## Project

**Heart Rate Receiver v2.0**

A desktop heart rate monitoring application with real-time visualization, comprehensive data analysis, and clean health-focused UI. Built with Tauri 2.x (Rust backend + Svelte frontend).

This version focuses on two major enhancements: UI refinement with a minimalist health aesthetic, and robust data recording, statistics, and analysis capabilities.

**Core Value:** **Clear, actionable heart rate insights at a glance.** Users should instantly understand their heart rate patterns—whether resting, exercising, or tracking long-term trends—without navigating complex interfaces.

### Constraints

- **Platform:** Desktop only (Windows, macOS, Linux) via Tauri
- **Timeline:** 1-2 months for complete delivery
- **Tech Stack:** Existing Tauri 2.x + Svelte 5 + Rust + chart.js (no chart library change)
- **BLE:** Current btleplug implementation, Heart Rate Service standard profile
<!-- GSD:project-end -->

<!-- GSD:stack-start source:codebase/STACK.md -->
## Technology Stack

## Languages
- TypeScript 5.x - Frontend application logic with strict mode enabled
- Rust 2021 Edition - Backend/Tauri application with system-level operations
- JavaScript - Build configuration (svelte.config.js, vite.config.ts)
- HTML/CSS - UI templates and styling
## Runtime
- Node.js - Frontend development and build
- Tauri 2.x - Desktop runtime (Windows, macOS, Linux)
- Tokio 1.x - Async runtime for Rust backend
- npm - Frontend dependency management
- Lockfile: `package-lock.json` (present)
- Cargo - Rust dependency management
- Lockfile: `Cargo.lock` (present)
## Frameworks
- SvelteKit 2.x - Frontend framework with SSG adapter
- Svelte 5.x - UI component framework
- Tauri 2.x - Desktop application framework
- Rust built-in tests - Backend unit tests (e.g., `src-tauri/src/ble/heart_rate.rs`)
- No frontend test framework detected
- Vite 6.x - Frontend build tool and dev server
- ESBuild - JavaScript bundling (via Vite)
- svelte-check 4.x - TypeScript/Svelte type checking
## Key Dependencies
- `@tauri-apps/api` ^2.0.0 - IPC bridge between frontend and Rust backend
- `chart.js` ^4.4.0 - Heart rate visualization charts
- `date-fns` ^4.1.0 - Date manipulation for timestamps
- `tauri` 2 - Desktop framework core
- `tauri-plugin-notification` 2 - System notifications for heart rate alerts
- `tauri-plugin-store` 2 - Persistent key-value storage
- `tauri-plugin-shell` 2 - Shell command execution
- `btleplug` 0.11 - Cross-platform Bluetooth Low Energy
- `rusqlite` 0.32 - SQLite database (bundled) for heart rate history
- `tokio` 1 - Async runtime with full feature set
- `serde` 1 / `serde_json` 1 - JSON serialization for IPC
- `chrono` 0.4 - Timestamp handling with serde support
- `uuid` 1 - Unique identifiers
- `anyhow` 1 / `thiserror` 1 - Error handling
- `log` 0.4 / `env_logger` 0.11 - Logging infrastructure
- `@sveltejs/adapter-static` ^3.0.0 - Static site generation for Tauri
- `@sveltejs/vite-plugin-svelte` ^5.0.0 - Svelte Vite integration
## Configuration
- No `.env` files detected
- Configuration via Tauri app data directory
- `svelte.config.js` - SvelteKit configuration with static adapter
- `vite.config.ts` - Vite build configuration, dev server on port 1420
- `tsconfig.json` - TypeScript strict mode configuration
- `src-tauri/tauri.conf.json` - Tauri application configuration
- `src-tauri/Cargo.toml` - Rust dependencies and build profile
- `src-tauri/capabilities/default.json` - Tauri permission system
## Platform Requirements
- Node.js 18+ (for SvelteKit 2)
- Rust toolchain (for Tauri)
- Platform-specific Bluetooth support
- Windows: Windows SDK for Bluetooth
- Desktop deployment via Tauri
- Builds: Windows (MSVC), macOS, Linux
- Output directory: `src-tauri/target/release/`
## IPC Architecture
- `start_scan` / `stop_scan` - BLE device discovery
- `get_devices` - List discovered devices
- `connect_device` / `disconnect_device` - Device connection
- `get_connection_state` / `get_connected_device` - Connection status
- `get_heart_rate_history` / `save_heart_rate` - Data persistence
- `set_alert_settings` / `get_alert_settings` - Alert configuration
- `init_database` - Database initialization
- `device-discovered` - New BLE device found
- `connection-state-changed` - Connection status update
- `heart-rate-measurement` - Real-time heart rate data
<!-- GSD:stack-end -->

<!-- GSD:conventions-start source:CONVENTIONS.md -->
## Conventions

## Naming Patterns
- Svelte components: PascalCase with `.svelte` extension (e.g., `HeartRateDisplay.svelte`)
- TypeScript modules: camelCase with `.ts` extension (e.g., `heartRate.ts`)
- Index/barrel files: `index.ts`
- Rust modules: snake_case (e.g., `heart_rate.rs`)
- camelCase for function names: `createDeviceStore()`, `startScan()`
- Async functions prefixed with action verbs: `loadSettings()`, `initListeners()`
- Factory functions use `create` prefix: `createHeartRateStore()`
- camelCase for variables and parameters: `deviceMap`, `sessionId`
- UPPER_SNAKE_CASE for constants: not observed (inline values used)
- PascalCase for interfaces and types: `DeviceInfo`, `HeartRateState`, `ConnectionState`
- Union types for state enums: `type ConnectionState = 'Disconnected' | 'Connecting' | 'Connected' | ...`
- snake_case for functions, variables, modules: `start_scan()`, `get_connection_state()`
- PascalCase for structs, enums, traits: `DeviceInfo`, `ConnectionState`, `BleManager`
- SCREAMING_SNAKE_CASE for constants: `HEART_RATE_SERVICE_UUID`, `ALERT_COOLDOWN_SECS`
- Properties that serialize to/from Rust use snake_case: `low_threshold`, `high_threshold`, `supports_heart_rate`
- Frontend-only properties use camelCase: `isScanning`, `maxHistoryLength`
## Code Style
- No explicit formatter config (no `.prettierrc` detected)
- TypeScript uses 2-space indentation
- Svelte components use 2-space indentation
- Rust follows standard `cargo fmt` conventions (4-space indentation)
- No ESLint configuration detected
- TypeScript compiler provides type checking
- `svelte-check` for Svelte validation
## Import Organization
- `$lib` maps to `src/lib/` (SvelteKit standard alias)
## Error Handling
- Try-catch with console.error for logging:
- Error state in stores:
- `Result<T, String>` for Tauri commands:
- `anyhow::Result` for internal operations (dependency available)
- `thiserror` for custom error types (dependency available)
- Log errors before returning:
## Logging
- Uses `console.error()` and `console.debug()` for development logging
- No structured logging framework
- Uses `log` crate with `env_logger`
- Log levels: `log::info!()`, `log::error!()`, `log::debug!()`, `log::warn!()`
- Default filter level: `info`
- Initialization in `main.rs`:
## Comments
- Doc comments for public Rust APIs using `///`
- Inline comments for non-obvious logic (e.g., performance optimizations)
- Not extensively used
- Type annotations serve as documentation
## Function Design
- TypeScript: Object parameters for multiple args when calling Tauri:
- Rust: Multiple parameters with clear types:
- Async functions return `Promise<T>`
- Store factory functions return object with `subscribe` plus custom methods
## Module Design
- Named exports preferred over default exports for utilities
- Default exports for Svelte components
- Barrel files for component aggregation
## Store Pattern (Svelte)
- `initListeners()` called in `onMount()`
- `cleanup()` returned from `onMount()` for automatic cleanup
- `reset()` for manual state reset
## Component Pattern (Svelte)
## CSS Conventions
- Defined in `:root` in `src/app.css`
- Theme variables: `--primary-color`, `--bg-color`, `--text-primary`, etc.
- Status colors: `--success-color`, `--danger-color`, `--warning-color`
- Component-specific values passed via style props
- `.glass-card` - Card with blur backdrop
- `.btn-primary`, `.btn-secondary`, `.btn-danger` - Button variants
<!-- GSD:conventions-end -->

<!-- GSD:architecture-start source:ARCHITECTURE.md -->
## Architecture

## Pattern Overview
- Rust backend for native functionality (BLE, database, alerts)
- Svelte frontend for UI rendering
- Event-driven communication between frontend and backend via Tauri IPC
- Reactive state management using Svelte stores
## Layers
- Purpose: User interface rendering and user interaction handling
- Location: `src/`
- Contains: Svelte components, TypeScript stores, utility functions
- Depends on: Tauri API (`@tauri-apps/api`)
- Used by: User interactions
- Purpose: Native platform access, BLE communication, data persistence
- Location: `src-tauri/src/`
- Contains: BLE manager, database operations, alert system, Tauri commands
- Depends on: Tauri runtime, btleplug, rusqlite, tokio
- Used by: Frontend via Tauri IPC commands
- Purpose: Persistent storage of heart rate records and settings
- Location: `src-tauri/src/storage/`
- Contains: SQLite database operations, data models
- Depends on: rusqlite
- Used by: Backend commands layer
## Data Flow
- Frontend uses custom Svelte stores (`src/lib/stores/`)
- Each store manages its own Tauri event listeners
- Backend state managed via Tauri managed state (`State<T>` in commands)
- BleManager uses `Arc<RwLock<BleManagerInner>>` for thread-safe state
## Key Abstractions
- Purpose: Central manager for all Bluetooth Low Energy operations
- Examples: `src-tauri/src/ble/manager.rs`
- Pattern: Singleton managed by Tauri, wraps btleplug adapter with async state
- Purpose: Reactive state management for frontend
- Examples: `src/lib/stores/heartRate.ts`, `src/lib/stores/device.ts`
- Pattern: Factory function returning Svelte store with custom methods
- Purpose: RPC-style function callable from frontend
- Examples: `src-tauri/src/commands/handlers.rs`
- Pattern: `#[tauri::command]` async functions with `State` injection
- Purpose: SQLite-based persistent storage
- Examples: `src-tauri/src/storage/database.rs`
- Pattern: Mutex-wrapped connection for thread-safe access
## Entry Points
- Location: `src-tauri/src/main.rs`
- Triggers: OS launches application executable
- Responsibilities: Initialize Tauri, plugins, database, register commands
- Location: `src/routes/+page.svelte`
- Triggers: SvelteKit routing after app initialization
- Responsibilities: Mount components, initialize store listeners, load settings
- Location: `src-tauri/src/ble/manager.rs:142` (`start_scan`)
- Triggers: Frontend invokes `start_scan` command
- Responsibilities: Initialize adapter, start discovery, emit events
- Location: `src-tauri/src/ble/manager.rs:427` (spawned task)
- Triggers: BLE peripheral sends notification
- Responsibilities: Parse measurement, emit event, check alerts
## Error Handling
- Rust: `Result<T, String>` for command return types
- Rust: `anyhow::Result` internally, converted to String for Tauri
- TypeScript: try/catch with error state in stores
- TypeScript: Console logging for debug, error state for UI feedback
- Backend uses `log` crate with `env_logger`
- Frontend uses `console.debug`/`console.error`
## Cross-Cutting Concerns
- Heart rate parsing validates data length and flags (`src-tauri/src/ble/heart_rate.rs`)
- Settings validated via TypeScript types and Rust serde deserialization
- Backend: `tokio` async runtime, `Arc<RwLock<T>>` for shared state
- Frontend: Single-threaded, reactive updates via Svelte stores
- Database: `Mutex<Connection>` for thread-safe SQLite access
- Backend emits events via `AppHandle::emit()`
- Frontend listens via `listen<T>()` from `@tauri-apps/api/event`
- Event types: `device-discovered`, `connection-state-changed`, `heart-rate-measurement`
<!-- GSD:architecture-end -->

<!-- GSD:workflow-start source:GSD defaults -->
## GSD Workflow Enforcement

Before using Edit, Write, or other file-changing tools, start work through a GSD command so planning artifacts and execution context stay in sync.

Use these entry points:
- `/gsd:quick` for small fixes, doc updates, and ad-hoc tasks
- `/gsd:debug` for investigation and bug fixing
- `/gsd:execute-phase` for planned phase work

Do not make direct repo edits outside a GSD workflow unless the user explicitly asks to bypass it.
<!-- GSD:workflow-end -->



<!-- GSD:profile-start -->
## Developer Profile

> Profile not yet configured. Run `/gsd:profile-user` to generate your developer profile.
> This section is managed by `generate-claude-profile` -- do not edit manually.
<!-- GSD:profile-end -->
