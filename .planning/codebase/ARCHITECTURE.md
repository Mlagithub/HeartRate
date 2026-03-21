# Architecture

**Analysis Date:** 2026-03-21

## Pattern Overview

**Overall:** Tauri Desktop Application (Hybrid Architecture)

**Key Characteristics:**
- Rust backend for native functionality (BLE, database, alerts)
- Svelte frontend for UI rendering
- Event-driven communication between frontend and backend via Tauri IPC
- Reactive state management using Svelte stores

## Layers

**Frontend Layer (Svelte/TypeScript):**
- Purpose: User interface rendering and user interaction handling
- Location: `src/`
- Contains: Svelte components, TypeScript stores, utility functions
- Depends on: Tauri API (`@tauri-apps/api`)
- Used by: User interactions

**Backend Layer (Rust/Tauri):**
- Purpose: Native platform access, BLE communication, data persistence
- Location: `src-tauri/src/`
- Contains: BLE manager, database operations, alert system, Tauri commands
- Depends on: Tauri runtime, btleplug, rusqlite, tokio
- Used by: Frontend via Tauri IPC commands

**Data Layer:**
- Purpose: Persistent storage of heart rate records and settings
- Location: `src-tauri/src/storage/`
- Contains: SQLite database operations, data models
- Depends on: rusqlite
- Used by: Backend commands layer

## Data Flow

**Heart Rate Monitoring Flow:**

1. User clicks "Scan" button in frontend
2. Frontend invokes `start_scan` Tauri command
3. Rust BleManager starts BLE scan via btleplug
4. Devices discovered emit `device-discovered` events to frontend
5. User selects device, frontend invokes `connect_device`
6. BleManager connects, subscribes to heart rate characteristic
7. Heart rate notifications emit `heart-rate-measurement` events
8. Frontend store receives event, updates UI, saves to database
9. Alert checker evaluates thresholds, sends notifications if needed

**State Management:**
- Frontend uses custom Svelte stores (`src/lib/stores/`)
- Each store manages its own Tauri event listeners
- Backend state managed via Tauri managed state (`State<T>` in commands)
- BleManager uses `Arc<RwLock<BleManagerInner>>` for thread-safe state

## Key Abstractions

**BleManager:**
- Purpose: Central manager for all Bluetooth Low Energy operations
- Examples: `src-tauri/src/ble/manager.rs`
- Pattern: Singleton managed by Tauri, wraps btleplug adapter with async state

**Store Pattern:**
- Purpose: Reactive state management for frontend
- Examples: `src/lib/stores/heartRate.ts`, `src/lib/stores/device.ts`
- Pattern: Factory function returning Svelte store with custom methods

**Tauri Command:**
- Purpose: RPC-style function callable from frontend
- Examples: `src-tauri/src/commands/handlers.rs`
- Pattern: `#[tauri::command]` async functions with `State` injection

**Database:**
- Purpose: SQLite-based persistent storage
- Examples: `src-tauri/src/storage/database.rs`
- Pattern: Mutex-wrapped connection for thread-safe access

## Entry Points

**Application Entry:**
- Location: `src-tauri/src/main.rs`
- Triggers: OS launches application executable
- Responsibilities: Initialize Tauri, plugins, database, register commands

**Frontend Entry:**
- Location: `src/routes/+page.svelte`
- Triggers: SvelteKit routing after app initialization
- Responsibilities: Mount components, initialize store listeners, load settings

**BLE Scan Entry:**
- Location: `src-tauri/src/ble/manager.rs:142` (`start_scan`)
- Triggers: Frontend invokes `start_scan` command
- Responsibilities: Initialize adapter, start discovery, emit events

**Heart Rate Notification Entry:**
- Location: `src-tauri/src/ble/manager.rs:427` (spawned task)
- Triggers: BLE peripheral sends notification
- Responsibilities: Parse measurement, emit event, check alerts

## Error Handling

**Strategy:** Result types with string errors for Tauri commands

**Patterns:**
- Rust: `Result<T, String>` for command return types
- Rust: `anyhow::Result` internally, converted to String for Tauri
- TypeScript: try/catch with error state in stores
- TypeScript: Console logging for debug, error state for UI feedback

**Logging:**
- Backend uses `log` crate with `env_logger`
- Frontend uses `console.debug`/`console.error`

## Cross-Cutting Concerns

**Logging:** `env_logger` in Rust with `RUST_LOG=info` default; console in TypeScript

**Validation:**
- Heart rate parsing validates data length and flags (`src-tauri/src/ble/heart_rate.rs`)
- Settings validated via TypeScript types and Rust serde deserialization

**Authentication:** None (local desktop application, no user authentication required)

**Concurrency:**
- Backend: `tokio` async runtime, `Arc<RwLock<T>>` for shared state
- Frontend: Single-threaded, reactive updates via Svelte stores
- Database: `Mutex<Connection>` for thread-safe SQLite access

**Event System:**
- Backend emits events via `AppHandle::emit()`
- Frontend listens via `listen<T>()` from `@tauri-apps/api/event`
- Event types: `device-discovered`, `connection-state-changed`, `heart-rate-measurement`

---

*Architecture analysis: 2026-03-21*