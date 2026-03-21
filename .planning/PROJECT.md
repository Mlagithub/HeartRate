# Heart Rate Receiver v2.0

## What This Is

A desktop heart rate monitoring application with real-time visualization, comprehensive data analysis, and clean health-focused UI. Built with Tauri 2.x (Rust backend + Svelte frontend).

This version focuses on two major enhancements: UI refinement with a minimalist health aesthetic, and robust data recording, statistics, and analysis capabilities.

## Core Value

**Clear, actionable heart rate insights at a glance.** Users should instantly understand their heart rate patterns—whether resting, exercising, or tracking long-term trends—without navigating complex interfaces.

## Requirements

### Validated

- ✓ Real-time heart rate display from BLE monitors — existing
- ✓ Device scanning and connection management — existing
- ✓ SQLite-based history storage — existing
- ✓ Configurable heart rate alerts with thresholds — existing
- ✓ Chart.js visualization of current session — existing
- ✓ Full-screen mode for monitoring — existing

### Active

**UI Enhancement:**
- [ ] Minimalist health-focused visual design
- [ ] Full-screen mode with auto-scaling Y-axis to amplify heart rate fluctuations
- [ ] Configurable full-screen UI elements (heart rate value, curve, session stats, status info) in main settings
- [ ] Heart rate curve X-axis toggle: recent N minutes (customizable) vs all data

**Data Analysis:**
- [ ] Time-dimension statistics: daily/weekly/monthly/yearly aggregations
- [ ] Exercise vs resting heart rate analysis
- [ ] HRV (Heart Rate Variability) estimation from existing heart rate data
- [ ] Data export to CSV/JSON formats
- [ ] Dual exercise detection modes: manual tagging + intelligent auto-detection

**Statistics Page:**
- [ ] Transform history page into data statistics & analysis page
- [ ] Tabbed interface: history records + statistics analysis
- [ ] Time dimension charts, exercise type comparison, trend analysis

### Out of Scope

- Mobile platform support — desktop-only for this version
- Real RR-interval based HRV calculation — using estimation from existing data
- Cloud sync/backup — local-only storage
- Multi-user/accounts — single-user desktop app

## Context

**Existing Architecture:**
- Tauri 2.x hybrid: Rust backend for BLE/database/alerts, Svelte frontend for UI
- Event-driven IPC between frontend and backend
- SQLite for persistent storage
- btleplug for cross-platform BLE communication
- chart.js for visualization

**Technical Debt to Address:**
- Missing frontend tests — add Vitest with testing-library/svelte
- Duplicate session ID generation — consolidate to shared utility
- Synchronous database writes per measurement — consider batching
- No history pagination — needed for statistics page

**Known Concerns:**
- BLE connection state machine is fragile — add integration tests
- In-memory session statistics lost on refresh — persist to storage
- No automatic reconnection on disconnect — consider adding

## Constraints

- **Platform:** Desktop only (Windows, macOS, Linux) via Tauri
- **Timeline:** 1-2 months for complete delivery
- **Tech Stack:** Existing Tauri 2.x + Svelte 5 + Rust + chart.js (no chart library change)
- **BLE:** Current btleplug implementation, Heart Rate Service standard profile

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Minimalist health UI style | Clean aesthetic suits health monitoring, reduces visual noise | — Pending |
| Auto-scaling Y-axis in fullscreen | Amplifies small fluctuations (72→75 BPM) for better visibility | — Pending |
| HRV estimation from existing data | Avoids BLE protocol upgrade, provides basic insights | — Pending |
| Dual exercise detection | Flexibility: manual for accuracy, auto for convenience | — Pending |
| Merged history + stats tabs | Unified analytics view, reduces navigation friction | — Pending |
| X-axis range toggle in chart | Direct manipulation, no settings menu digging | — Pending |

---

*Last updated: 2026-03-21 after initialization*

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd:transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd:complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state