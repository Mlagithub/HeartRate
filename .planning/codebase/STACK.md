# Technology Stack

**Analysis Date:** 2026-03-21

## Languages

**Primary:**
- TypeScript 5.x - Frontend application logic with strict mode enabled
- Rust 2021 Edition - Backend/Tauri application with system-level operations

**Secondary:**
- JavaScript - Build configuration (svelte.config.js, vite.config.ts)
- HTML/CSS - UI templates and styling

## Runtime

**Environment:**
- Node.js - Frontend development and build
- Tauri 2.x - Desktop runtime (Windows, macOS, Linux)
- Tokio 1.x - Async runtime for Rust backend

**Package Manager:**
- npm - Frontend dependency management
- Lockfile: `package-lock.json` (present)
- Cargo - Rust dependency management
- Lockfile: `Cargo.lock` (present)

## Frameworks

**Core:**
- SvelteKit 2.x - Frontend framework with SSG adapter
- Svelte 5.x - UI component framework
- Tauri 2.x - Desktop application framework

**Testing:**
- Rust built-in tests - Backend unit tests (e.g., `src-tauri/src/ble/heart_rate.rs`)
- No frontend test framework detected

**Build/Dev:**
- Vite 6.x - Frontend build tool and dev server
- ESBuild - JavaScript bundling (via Vite)
- svelte-check 4.x - TypeScript/Svelte type checking

## Key Dependencies

**Frontend Critical:**
- `@tauri-apps/api` ^2.0.0 - IPC bridge between frontend and Rust backend
- `chart.js` ^4.4.0 - Heart rate visualization charts
- `date-fns` ^4.1.0 - Date manipulation for timestamps

**Backend Critical:**
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

**Infrastructure:**
- `@sveltejs/adapter-static` ^3.0.0 - Static site generation for Tauri
- `@sveltejs/vite-plugin-svelte` ^5.0.0 - Svelte Vite integration

## Configuration

**Environment:**
- No `.env` files detected
- Configuration via Tauri app data directory

**Build:**
- `svelte.config.js` - SvelteKit configuration with static adapter
- `vite.config.ts` - Vite build configuration, dev server on port 1420
- `tsconfig.json` - TypeScript strict mode configuration
- `src-tauri/tauri.conf.json` - Tauri application configuration
- `src-tauri/Cargo.toml` - Rust dependencies and build profile
- `src-tauri/capabilities/default.json` - Tauri permission system

## Platform Requirements

**Development:**
- Node.js 18+ (for SvelteKit 2)
- Rust toolchain (for Tauri)
- Platform-specific Bluetooth support
- Windows: Windows SDK for Bluetooth

**Production:**
- Desktop deployment via Tauri
- Builds: Windows (MSVC), macOS, Linux
- Output directory: `src-tauri/target/release/`

## IPC Architecture

**Frontend-to-Backend Commands:**
Defined in `/mnt/d/work/heratbeat/src/lib/utils/tauri.ts`:
- `start_scan` / `stop_scan` - BLE device discovery
- `get_devices` - List discovered devices
- `connect_device` / `disconnect_device` - Device connection
- `get_connection_state` / `get_connected_device` - Connection status
- `get_heart_rate_history` / `save_heart_rate` - Data persistence
- `set_alert_settings` / `get_alert_settings` - Alert configuration
- `init_database` - Database initialization

**Backend-to-Frontend Events:**
- `device-discovered` - New BLE device found
- `connection-state-changed` - Connection status update
- `heart-rate-measurement` - Real-time heart rate data

---

*Stack analysis: 2026-03-21*