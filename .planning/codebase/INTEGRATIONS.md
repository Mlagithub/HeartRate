# External Integrations

**Analysis Date:** 2026-03-21

## APIs & External Services

**Bluetooth Low Energy (BLE):**
- Service: Heart Rate Monitors (standard Bluetooth Heart Rate Profile)
- SDK/Client: `btleplug` 0.11 (Rust cross-platform BLE library)
- Protocol: Standard Heart Rate Service UUID `0x180D`
- Characteristic: Heart Rate Measurement UUID `0x2A37`
- Data format: 8-bit or 16-bit BPM, optional RR intervals, sensor contact, energy expended
- Implementation: `/mnt/d/work/heratbeat/src-tauri/src/ble/heart_rate.rs`

## Data Storage

**Databases:**
- SQLite (via rusqlite 0.32 with bundled feature)
  - Connection: Local file at `{app_data_dir}/heart_rate.db`
  - Location: Configured in `/mnt/d/work/heratbeat/src-tauri/src/main.rs`
  - Tables: `heart_rate_records`, `alert_settings`
  - Indexes: `idx_timestamp`, `idx_session_id`
  - Implementation: `/mnt/d/work/heratbeat/src-tauri/src/storage/database.rs`

**File Storage:**
- Tauri app data directory for database file
- Path resolution: `app_handle.path().app_data_dir()`
- No cloud storage - fully local/offline

**Caching:**
- In-memory cache: BLE device discovery cache in `BleManagerInner.discovered_devices`
- No external caching service

## Authentication & Identity

**Auth Provider:**
- None - Desktop application with no user authentication
- Session identification: Generated UUID-like session IDs (format: `session_{timestamp}_{random}`)
- Implementation: `/mnt/d/work/heratbeat/src/lib/stores/settings.ts`

## Monitoring & Observability

**Error Tracking:**
- None - No external error tracking service

**Logs:**
- Rust logging via `log` and `env_logger` crates
- Log level: Configurable via `RUST_LOG` environment variable (defaults to "info")
- Output: Standard output/stderr
- Implementation: `/mnt/d/work/heratbeat/src-tauri/src/main.rs`

**Frontend Logging:**
- Console-based logging via `console.debug`, `console.error`
- No structured logging framework

## CI/CD & Deployment

**Hosting:**
- Desktop deployment only (not a web application)
- Tauri handles native installer generation

**CI Pipeline:**
- None detected - No CI configuration files found

## Environment Configuration

**Required env vars:**
- None - Application runs without environment configuration
- Optional: `RUST_LOG` for log level control

**Secrets location:**
- None - No secrets or API keys required

## Webhooks & Callbacks

**Incoming:**
- None - No external webhook endpoints

**Outgoing:**
- None - No outgoing webhooks or callbacks

## Hardware Integration

**Bluetooth Devices:**
- Heart Rate Monitors supporting standard Bluetooth Heart Rate Profile
- Auto-discovery via BLE scanning
- Subscription to heart rate measurement notifications
- Real-time data streaming via Tauri events

**Device Requirements:**
- Computer with Bluetooth Low Energy support
- Heart rate monitor advertising Heart Rate Service UUID (0x180D)

## System Notifications

**Notification System:**
- Platform: OS-native via `tauri-plugin-notification`
- Use cases:
  - High heart rate alerts (above threshold)
  - Low heart rate alerts (below threshold)
- Cooldown: 60 seconds between same-type alerts
- Implementation: `/mnt/d/work/heratbeat/src-tauri/src/alert/notifier.rs`

**Permissions:**
- Managed via Tauri capabilities in `/mnt/d/work/heratbeat/src-tauri/capabilities/default.json`
- Required: `notification:default`, `notification:allow-is-permission-granted`, `notification:allow-request-permission`, `notification:allow-notify`

## Plugin Ecosystem

**Tauri Plugins Used:**
| Plugin | Version | Purpose |
|--------|---------|---------|
| tauri-plugin-shell | 2 | Open external URLs/files |
| tauri-plugin-notification | 2 | System notifications |
| tauri-plugin-store | 2 | Persistent settings storage |

---

*Integration audit: 2026-03-21*