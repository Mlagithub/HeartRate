# Codebase Concerns

**Analysis Date:** 2026-03-21

## Tech Debt

**Missing Frontend Tests:**
- Issue: No test files exist for Svelte frontend components or stores
- Files: `src/lib/**/*.ts`, `src/lib/**/*.svelte`
- Impact: Changes to stores or components can break UI without detection
- Fix approach: Add Vitest with @testing-library/svelte for component tests, unit tests for stores

**Duplicate Session ID Generation:**
- Issue: `generateSessionId()` function defined in both `settings.ts` and `heartRate.ts`
- Files: `src/lib/stores/settings.ts:85-87`, `src/lib/stores/heartRate.ts:44-46`
- Impact: Divergent implementations, potential for inconsistent session IDs
- Fix approach: Extract to shared utility file `src/lib/utils/session.ts`

**Redundant Database Init Command:**
- Issue: `init_database` command exists but database is initialized in `main.rs` setup
- Files: `src-tauri/src/commands/handlers.rs:109-117`
- Impact: Unnecessary code, confusing API surface
- Fix approach: Remove command or add actual re-initialization logic

**Limited Rust Test Coverage:**
- Issue: Only `heart_rate.rs` has unit tests; BLE manager, database, alert logic untested
- Files: `src-tauri/src/ble/manager.rs`, `src-tauri/src/storage/database.rs`, `src-tauri/src/alert/notifier.rs`
- Impact: Core functionality changes can introduce regressions
- Fix approach: Add integration tests for database operations, mock-based tests for BLE manager

## Security Considerations

**Missing Content Security Policy:**
- Risk: CSP set to `null` allows unrestricted script execution
- Files: `src-tauri/tauri.conf.json:24`
- Current mitigation: Application loads local content only
- Recommendations: Configure CSP to restrict script sources, disable eval

**No Input Validation for Alert Thresholds:**
- Risk: User can set `low_threshold > high_threshold`, creating invalid alert state
- Files: `src-tauri/src/storage/database.rs:209-224`, `src/lib/components/AlertSettings.svelte`
- Current mitigation: UI has min/max bounds but no cross-validation
- Recommendations: Add validation in both frontend (before save) and backend (in `set_alert_settings`)

**Shell Permission Enabled:**
- Risk: `shell:allow-open` permission could be exploited to open arbitrary URLs
- Files: `src-tauri/capabilities/default.json:10`
- Current mitigation: Not actively used in codebase
- Recommendations: Remove if not needed, or restrict to specific URL patterns

## Performance Bottlenecks

**Synchronous Database Writes Per Measurement:**
- Problem: Every heart rate measurement triggers immediate database write
- Files: `src/lib/stores/heartRate.ts:62-81`
- Cause: `saveMeasurement` called on every `heart-rate-measurement` event
- Improvement path: Batch writes (e.g., every 10 measurements or 5 seconds), use write-ahead logging

**No History Pagination:**
- Problem: History view loads fixed 50 records with no way to load more
- Files: `src/lib/components/HistoryView.svelte:7`, `src/lib/stores/history.ts:27`
- Cause: No pagination UI or infinite scroll implemented
- Improvement path: Add "Load More" button or infinite scroll with offset-based loading

**Chart Re-renders on Every Store Update:**
- Problem: Chart subscribes to entire store, triggers update on any change
- Files: `src/lib/components/HeartRateChart.svelte:202`
- Cause: `heartRate.subscribe(updateChart)` fires on every measurement
- Improvement path: Debounce updates more aggressively, or subscribe only to history changes

## Fragile Areas

**BLE Connection State Machine:**
- Files: `src-tauri/src/ble/manager.rs:14-27`, `src-tauri/src/ble/manager.rs:333-455`
- Why fragile: Multiple async operations modify shared state, potential race conditions between scan/connect/disconnect
- Safe modification: Add state machine library (e.g., `smol` or explicit state transitions with validation)
- Test coverage: None - high-risk area needs integration tests

**Notification Stream Lifecycle:**
- Files: `src-tauri/src/ble/manager.rs:427-452`
- Why fragile: Notification handler spawned in tokio task; if task panics, heart rate updates stop silently
- Safe modification: Add error recovery, restart notification stream on failure, health check mechanism
- Test coverage: None

**In-Memory Session Statistics:**
- Files: `src/lib/stores/heartRate.ts:41-43`
- Why fragile: `sessionStats` and `totalBpm` are module-level variables; lost on page refresh or navigation
- Safe modification: Persist to localStorage or sync with database
- Test coverage: None

**Connection Error State Not Persisted:**
- Files: `src-tauri/src/ble/manager.rs:364-367`
- Why fragile: Error state emitted but not stored; UI may not reflect error after re-render
- Safe modification: Store error state in BleManagerInner and expose via command
- Test coverage: None

## Scaling Limits

**Database Size Growth:**
- Current capacity: No limits, grows indefinitely with heart rate records
- Limit: Could cause slow queries and storage issues over months of use
- Scaling path: Add automatic cleanup (e.g., keep last 30 days), or data archival feature

**History Query Performance:**
- Current capacity: Indexed by timestamp, but no query optimization for large datasets
- Limit: `get_history` with limit/offset performs poorly with large offsets
- Scaling path: Use cursor-based pagination instead of offset, add compound indexes

**Chart Data Points:**
- Current capacity: Limited to 100 points in memory (`maxHistoryLength: 100`)
- Limit: Fixed buffer size may not suit all use cases; long sessions lose early data
- Scaling path: Make configurable, or implement dynamic windowing based on session duration

## Dependencies at Risk

**btleplug BLE Library:**
- Risk: Active maintenance but BLE is platform-specific; Windows/macOS/Linux behavior may differ
- Impact: Connection failures, device discovery issues on specific platforms
- Migration plan: None currently - monitor issues, consider abstraction layer

**chart.js:**
- Risk: Large bundle size (~70KB minified), may be overkill for simple line chart
- Impact: Slower initial load, unnecessary complexity
- Migration plan: Consider lighter alternatives (uPlot, lightweight-charts) if bundle size becomes issue

## Missing Critical Features

**Automatic Reconnection:**
- Problem: When device disconnects unexpectedly, no automatic reconnection attempt
- Files: `src-tauri/src/ble/manager.rs:446-452`
- Blocks: Seamless user experience during brief connection drops

**History Data Export:**
- Problem: No way to export heart rate data (CSV, JSON)
- Files: `src/lib/components/HistoryView.svelte`
- Blocks: Users cannot backup or analyze data externally

**Clear History Functionality:**
- Problem: `clear_all` exists in database but not exposed to UI
- Files: `src-tauri/src/storage/database.rs:181-185`
- Blocks: Users cannot manage their data storage

**Historical Data Visualization:**
- Problem: History tab shows only list view; no chart for historical data
- Files: `src/lib/components/HistoryView.svelte`
- Blocks: Users cannot see trends over time from past sessions

## Test Coverage Gaps

**BLE Manager Logic:**
- What's not tested: Scan lifecycle, connection handling, notification subscription, error recovery
- Files: `src-tauri/src/ble/manager.rs`
- Risk: Connection bugs may not be caught until runtime
- Priority: High

**Database Operations:**
- What's not tested: CRUD operations, concurrent access, error handling
- Files: `src-tauri/src/storage/database.rs`
- Risk: Data corruption or loss on edge cases
- Priority: High

**Alert Notification Logic:**
- What's not tested: Threshold comparison, cooldown timer, notification triggering
- Files: `src-tauri/src/alert/notifier.rs`
- Risk: Alerts may not fire when expected
- Priority: Medium

**Frontend Stores:**
- What's not tested: State updates, listener lifecycle, error handling
- Files: `src/lib/stores/*.ts`
- Risk: State management bugs affect entire UI
- Priority: Medium

**Svelte Components:**
- What's not tested: Rendering, user interactions, event handling
- Files: `src/lib/components/*.svelte`
- Risk: UI regressions may go undetected
- Priority: Low

---

*Concerns audit: 2026-03-21*