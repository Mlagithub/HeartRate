# Codebase Structure

**Analysis Date:** 2026-03-21

## Directory Layout

```
/mnt/d/work/heratbeat/
├── src/                      # Frontend source (Svelte/TypeScript)
│   ├── lib/                  # Shared library code
│   │   ├── components/       # Svelte UI components
│   │   ├── stores/           # Svelte reactive stores
│   │   └── utils/            # Utility functions
│   └── routes/               # SvelteKit routes (pages)
├── src-tauri/                # Rust backend source
│   ├── src/                  # Rust source code
│   │   ├── alert/            # Alert/notification system
│   │   ├── ble/              # Bluetooth Low Energy module
│   │   ├── commands/         # Tauri IPC command handlers
│   │   └── storage/          # Database operations
│   ├── capabilities/         # Tauri security capabilities
│   ├── icons/                # Application icons
│   └── tauri.conf.json       # Tauri configuration
├── .planning/                # Planning documents (this directory)
├── package.json              # Node.js dependencies
├── svelte.config.js          # SvelteKit configuration
├── tsconfig.json             # TypeScript configuration
└── vite.config.ts            # Vite build configuration
```

## Directory Purposes

**src/lib/components:**
- Purpose: Reusable Svelte UI components
- Contains: `.svelte` files for UI elements
- Key files: `HeartRateDisplay.svelte`, `DeviceScanner.svelte`, `HeartRateChart.svelte`

**src/lib/stores:**
- Purpose: Reactive state management
- Contains: TypeScript store factory functions
- Key files: `heartRate.ts`, `device.ts`, `settings.ts`, `history.ts`

**src/lib/utils:**
- Purpose: Helper functions and API wrappers
- Contains: TypeScript utility modules
- Key files: `tauri.ts` (Tauri command wrappers)

**src/routes:**
- Purpose: SvelteKit page routing
- Contains: `+page.svelte` (main page), `+layout.svelte` (root layout)
- Key files: `+page.svelte` (application shell with tabs)

**src-tauri/src/ble:**
- Purpose: Bluetooth Low Energy operations
- Contains: Device discovery, connection, heart rate parsing
- Key files: `manager.rs` (BLE state machine), `heart_rate.rs` (parsing), `device.rs` (types)

**src-tauri/src/commands:**
- Purpose: Tauri IPC command handlers
- Contains: `#[tauri::command]` functions callable from frontend
- Key files: `handlers.rs` (all command implementations)

**src-tauri/src/storage:**
- Purpose: Data persistence
- Contains: SQLite database operations
- Key files: `database.rs` (all database operations and types)

**src-tauri/src/alert:**
- Purpose: Heart rate alert notifications
- Contains: Threshold checking, notification sending
- Key files: `notifier.rs` (alert logic with cooldown)

## Key File Locations

**Entry Points:**
- `src-tauri/src/main.rs`: Rust application entry point
- `src/routes/+page.svelte`: Main UI page
- `src/routes/+layout.svelte`: Root layout component

**Configuration:**
- `src-tauri/tauri.conf.json`: Tauri app configuration (window, plugins, build)
- `package.json`: Node.js dependencies and scripts
- `tsconfig.json`: TypeScript compiler options
- `svelte.config.js`: SvelteKit adapter configuration
- `vite.config.ts`: Vite dev server and build config

**Core Logic:**
- `src-tauri/src/ble/manager.rs`: BLE connection and scanning logic
- `src-tauri/src/storage/database.rs`: SQLite operations
- `src-tauri/src/alert/notifier.rs`: Alert threshold checking

**Frontend State:**
- `src/lib/stores/heartRate.ts`: Heart rate measurements and history
- `src/lib/stores/device.ts`: BLE device discovery and connection state
- `src/lib/stores/settings.ts`: Alert settings and preferences
- `src/lib/stores/history.ts`: Historical records from database

**Testing:**
- `src-tauri/src/ble/heart_rate.rs`: Contains `#[cfg(test)]` unit tests for parsing

## Naming Conventions

**Files:**
- Svelte components: PascalCase with `.svelte` extension (e.g., `HeartRateDisplay.svelte`)
- TypeScript stores: camelCase with `.ts` extension (e.g., `heartRate.ts`)
- Rust modules: snake_case with `.rs` extension (e.g., `heart_rate.rs`)
- Rust module directories: lowercase (e.g., `ble/`, `storage/`)

**Directories:**
- Feature-based: lowercase, descriptive (e.g., `components/`, `stores/`, `ble/`)

**Type/Struct Names:**
- TypeScript interfaces: PascalCase (e.g., `HeartRateMeasurement`, `DeviceInfo`)
- Rust structs/enums: PascalCase (e.g., `BleManager`, `ConnectionState`)

**Functions:**
- TypeScript: camelCase (e.g., `startScan`, `initListeners`)
- Rust: snake_case (e.g., `start_scan`, `connect_device`)

## Where to Add New Code

**New Feature:**
- Frontend component: `src/lib/components/[ComponentName].svelte`
- Export from: `src/lib/components/index.ts`
- Backend command: `src-tauri/src/commands/handlers.rs`
- Register in: `src-tauri/src/main.rs` (invoke_handler)

**New Store:**
- Implementation: `src/lib/stores/[storeName].ts`
- Export from: `src/lib/stores/index.ts`

**New BLE Feature:**
- Implementation: `src-tauri/src/ble/` (add to existing or new file)
- Export from: `src-tauri/src/ble/mod.rs`
- Command handler: `src-tauri/src/commands/handlers.rs`

**New Database Table/Operation:**
- Schema: Add CREATE TABLE to `src-tauri/src/storage/database.rs` constructor
- Operations: Add methods to `Database` struct in same file

**New Alert Type:**
- Logic: `src-tauri/src/alert/notifier.rs`
- Settings: Update `AlertSettings` struct in `src-tauri/src/storage/database.rs`

**New UI Tab/Page:**
- Component: `src/lib/components/[ViewName].svelte`
- Add to tabs: `src/routes/+page.svelte` (nav and conditional rendering)

## Special Directories

**.svelte-kit:**
- Purpose: SvelteKit generated files
- Generated: Yes (auto-generated by SvelteKit)
- Committed: No (in .gitignore)

**src-tauri/target:**
- Purpose: Rust build artifacts
- Generated: Yes (cargo build output)
- Committed: No (in .gitignore)

**node_modules:**
- Purpose: Node.js dependencies
- Generated: Yes (npm install)
- Committed: No (in .gitignore)

**.planning:**
- Purpose: GSD planning documents
- Generated: Yes (by GSD commands)
- Committed: Yes (tracked in git)

## Import Path Aliases

**TypeScript/Svelte:**
- `$lib/` maps to `src/lib/`
- Example: `import { heartRate } from '$lib/stores/heartRate';`

**Rust:**
- `crate::` for internal module references
- Example: `use crate::ble::BleManager;`

---

*Structure analysis: 2026-03-21*