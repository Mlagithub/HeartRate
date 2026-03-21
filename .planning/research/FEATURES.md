# Feature Landscape

**Domain:** Heart rate monitoring and health analytics for desktop applications
**Researched:** 2026-03-21
**Confidence:** HIGH (based on existing codebase analysis, HRV research from NCBI, and health app feature patterns)

## Table Stakes

Features users expect. Missing = product feels incomplete.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| **Real-time BPM display** | Core purpose of the app | Low | Already implemented with zone coloring |
| **Heart rate zones** | Standard fitness concept, users expect zone-based feedback | Low | Already implemented with 5 zones |
| **Session statistics (min/avg/max)** | Basic insight from any monitoring session | Low | Already implemented in-session |
| **History of past sessions** | Users expect to review past data | Low | Already implemented with pagination |
| **Configurable alerts** | Safety feature for heart rate thresholds | Low | Already implemented |
| **Device connection management** | Essential for BLE monitors | Low | Already implemented |
| **Chart visualization** | Visual representation of heart rate over time | Low | Already implemented with Chart.js |
| **Data export (CSV/JSON)** | Users own their data and expect to export it | Medium | Target feature - standard in all health apps |
| **Time-based statistics** | Daily/weekly/monthly aggregations | Medium | Target feature - basic analytics expectation |
| **Fullscreen mode** | Workout/distance monitoring use case | Medium | Implemented but needs enhancement |

## Differentiators

Features that set product apart. Not expected, but valued.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **HRV estimation from BPM** | Provides health insights without protocol upgrade | Medium | Target feature - limited accuracy but valuable if clearly labeled |
| **Automatic exercise detection** | Convenience: no manual tagging required | High | Target feature - dual mode (manual + auto) offers flexibility |
| **Auto-scaling Y-axis** | Amplifies small fluctuations for better visibility | Low | Target feature - differentiates from fixed-scale apps |
| **Configurable X-axis range** | User control over time window displayed | Low | Target feature - recent N minutes vs all data |
| **Exercise vs resting analysis** | Separates active from passive heart rate | Medium | Target feature - insight into fitness patterns |
| **Minimalist health-focused UI** | Reduces visual noise, premium feel | Medium | Target feature - aesthetic differentiation |
| **Cross-platform desktop app** | Works on Windows, macOS, Linux | Low | Existing - differentiates from mobile-only competitors |
| **Local-only data storage** | Privacy: no cloud dependency | Low | Existing - differentiates from cloud-required apps |

## Anti-Features

Features to explicitly NOT build.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| **Mobile platform support** | Scope creep, different UX patterns, BLE implementation differences | Focus on desktop excellence; consider mobile as separate future project |
| **Real RR-interval based HRV** | Requires BLE protocol change, different hardware support, major refactor | Use BPM-based estimation with clear accuracy labeling; document limitation |
| **Cloud sync/backup** | Privacy concerns, infrastructure cost, single-user desktop focus | Export functionality for user-controlled backup; local SQLite database |
| **Multi-user accounts** | Desktop app paradigm, adds authentication complexity | Single-user assumption; data separation at OS level if needed |
| **Social features (sharing, challenges)** | Not core value proposition, privacy-first positioning | Export for external sharing; no in-app social |
| **AI coaching/recommendations** | Over-engineering, liability concerns, requires extensive data | Show data clearly; let users draw conclusions |
| **Real-time continuous HRV** | HRV needs minimum 60 seconds data, not meaningful per-beat | On-demand session-based HRV analysis |
| **Sleep tracking** | Different use case, requires overnight monitoring, battery considerations | Focus on active monitoring; sleep is separate product territory |

## Feature Dependencies

```
Real-time BPM display
    |
    +---> Session statistics (min/avg/max) [requires BPM stream]
    |
    +---> Heart rate zones [requires BPM]
    |
    +---> Chart visualization [requires BPM stream]
    |
    +---> Fullscreen mode [requires chart + BPM display]

History storage
    |
    +---> Data export [requires stored history]
    |
    +---> Time-based statistics [requires historical data]
    |        |
    |        +---> Daily aggregations [requires date-indexed data]
    |        +---> Weekly/Monthly/Yearly [extends daily]
    |
    +---> Exercise vs resting analysis [requires time-tagged data]
    |        |
    |        +---> Automatic exercise detection [requires baseline + patterns]
    |
    +---> HRV estimation [requires BPM data, better with RR-intervals]

BPM-based estimation
    |
    +---> HRV from BPM [requires multi-beat data, estimation algorithm]
           |
           +---> Accuracy depends on: measurement frequency, session duration, artifact filtering
```

## MVP Recommendation

### Phase 1: Foundation (Must Have)
1. **Data export (CSV/JSON)** - Table stakes, enables user data ownership
2. **Daily statistics** - Basic analytics expectation
3. **Configurable chart X-axis range** - User control enhancement

### Phase 2: Core Analytics (Should Have)
4. **Weekly/Monthly aggregations** - Extends daily pattern
5. **Exercise vs resting analysis** - Insight differentiation
6. **Auto-scaling Y-axis** - Visualization enhancement

### Phase 3: Advanced Features (Could Have)
7. **HRV estimation** - Differentiator, clearly labeled as estimated
8. **Automatic exercise detection** - Convenience feature

### Defer (Won't Have This Version)
- Mobile platform support
- RR-interval based HRV (requires BLE protocol investigation)
- Cloud sync
- Multi-user support

## Complexity Assessment

| Feature | Backend Complexity | Frontend Complexity | Database Impact |
|---------|-------------------|---------------------|-----------------|
| Data export | Medium (streaming, file I/O) | Low (trigger button) | None (read-only) |
| Daily statistics | Medium (SQL aggregations) | Medium (charts, caching) | Add date index |
| Weekly/Monthly stats | Low (extends daily) | Low (tab switching) | None |
| Exercise vs resting | Medium (classification logic) | Medium (visualization) | Add session type field |
| HRV estimation | High (algorithm, accuracy) | Medium (display, confidence) | None (computed) |
| Auto exercise detection | High (ML/rules, personalization) | Medium (confirmation UI) | Add detection state |
| Auto-scaling Y-axis | Low | Low (chart config) | None |
| Configurable X-axis | Low | Low (range selector) | None |
| Minimalist UI | Low | Medium (design iteration) | None |

## Existing Implementation Status

| Feature | Status | Location |
|---------|--------|----------|
| Real-time BPM | Implemented | `HeartRateDisplay.svelte`, `heartRate.ts` store |
| Heart rate zones | Implemented | `HeartRateDisplay.svelte` (hardcoded thresholds) |
| Session stats | Implemented | `heartRate.ts` store (in-memory only) |
| History view | Implemented | `HistoryView.svelte`, `history.ts` store |
| Alerts | Implemented | `AlertSettings.svelte`, `database.rs` |
| Chart | Implemented | `HeartRateChart.svelte`, Chart.js |
| Fullscreen | Implemented | `FullscreenMode.svelte` (needs enhancement) |
| BLE connection | Implemented | `device.ts`, `manager.rs` |
| SQLite storage | Implemented | `database.rs` |
| RR-interval parsing | Implemented but NOT stored | `heart_rate.rs` line 62-69 |

## Key Insight: RR-Interval Availability

**Critical finding:** The BLE Heart Rate Measurement characteristic provides RR-intervals (line 62-69 of `src-tauri/src/ble/heart_rate.rs`), but they are currently **parsed but discarded** - not stored in database.

**Implications:**
1. **For HRV feature:** Could upgrade to real RR-interval based HRV by storing RR data
2. **Database change required:** Add `rr_intervals` column to `heart_rate_records` table
3. **Accuracy trade-off:** BPM estimation is less accurate than RR-interval HRV
4. **Recommendation:** Investigate storing RR-intervals during Data Analysis phase if HRV accuracy is prioritized

## Sources

| Source | Confidence | Information Used |
|--------|------------|------------------|
| NCBI PMC Article on HRV | HIGH | HRV calculation methods (RMSSD, SDNN), RR-interval requirements |
| Codebase analysis (`src-tauri/src/ble/heart_rate.rs`) | HIGH | RR-interval parsing already implemented |
| Codebase analysis (`src-tauri/src/storage/database.rs`) | HIGH | Current schema, what's stored vs discarded |
| Apple Health documentation | MEDIUM | Standard health app features, data visualization patterns |
| PROJECT.md | HIGH | Active requirements, out of scope items |

---

*Research completed: 2026-03-21*