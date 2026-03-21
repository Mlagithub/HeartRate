# Research Summary: Heart Rate Analytics Enhancement

**Project:** Heart Rate Receiver v2.0
**Synthesized:** 2026-03-21
**Sources:** STACK.md, FEATURES.md, ARCHITECTURE.md, PITFALLS.md

---

## Executive Summary

This research synthesis covers the enhancement of an existing Tauri-based heart rate monitoring desktop application with analytics capabilities. The project builds on a working foundation: real-time BLE heart rate monitoring, Chart.js visualization, SQLite storage, and configurable alerts are already implemented. The target features are data export, time-dimension statistics (daily/weekly/monthly), HRV estimation, and automatic exercise detection.

The recommended approach uses a **SQL-first architecture** where SQLite window functions handle aggregations, Rust computes complex algorithms (HRV estimation, exercise detection), and the frontend focuses on visualization. This avoids the common pitfall of fetching large datasets for JavaScript computation. Key additions include `statrs` and `ndarray-stats` for Rust statistics, Chart.js plugins for annotations and zoom, and Tauri file system plugins for export.

**Critical finding:** RR-interval data IS available from the BLE Heart Rate Measurement characteristic but is currently parsed and discarded. This means HRV could be accurate rather than estimated if we store RR-intervals during the implementation. This is a strategic decision point for the roadmap.

Major risks include: misleading users with estimated HRV (mitigation: clear labeling), chart performance degradation during long sessions (mitigation: decimation from the start), and static exercise detection thresholds failing across user physiology (mitigation: personalized baselines).

---

## Key Findings

### From STACK.md

**Core Technology Additions:**

| Technology | Purpose | Rationale |
|------------|---------|-----------|
| `statrs` 0.18.0 | Statistical distributions | HRV estimation, exercise detection algorithms |
| `ndarray` + `ndarray-stats` | Numerical computing | Percentile calculations, time series analysis |
| `csv` 1.x | CSV export | Streaming export for large datasets |
| `chartjs-plugin-annotation` 3.1.0 | Zone annotations | Draw heart rate zones on charts |
| `chartjs-plugin-zoom` 2.2.0 | Pan/zoom navigation | History analysis feature |
| `simple-statistics` 7.8.9 | Client-side stats | Quick frontend calculations |
| `tauri-plugin-fs` + `tauri-plugin-dialog` | File export | Native save dialogs, file writing |

**Critical Stack Decision:** Chart.js is retained (project constraint: "no chart library change"). Plugins extend it for health data visualization needs.

**HRV Algorithm:** RMSSD and SDNN calculated from RR-intervals (if stored) or estimated from BPM variance (less accurate, must be labeled).

### From FEATURES.md

**Must Have (Phase 1):**
- Data export (CSV/JSON) - table stakes for health apps
- Daily statistics - basic analytics expectation
- Configurable chart X-axis range - user control

**Should Have (Phase 2):**
- Weekly/Monthly aggregations
- Exercise vs resting analysis
- Auto-scaling Y-axis

**Differentiators (Phase 3):**
- HRV estimation - clearly labeled as estimated
- Automatic exercise detection - dual mode (manual + auto)

**Key Discovery:** BLE implementation already parses RR-intervals (`heart_rate.rs` lines 62-69) but does NOT store them. Adding RR-interval storage would enable accurate HRV instead of estimation.

**Anti-Features (Do NOT Build):**
- Mobile platform support
- Real RR-interval based HRV (requires investigation - may be possible)
- Cloud sync/backup
- Multi-user accounts
- Social features
- AI coaching

### From ARCHITECTURE.md

**Data Flow Pattern:**
```
Frontend Request (dimension, date_range)
    |
    v
Backend Command (get_statistics)
    |
    v
SQLite Aggregation Query (window functions)
    |
    v
Return Aggregated Results (not raw data)
    |
    v
Frontend Store (cache results)
    |
    v
Chart.js Rendering
```

**Where Calculations Live:**
- **Backend (Rust):** Daily/weekly/monthly aggregations, moving averages, HRV estimation, exercise detection, export generation, percentile calculations
- **Frontend (TypeScript):** Real-time session stats, Chart.js data transformations, UI-derived state, color coding

**Module Structure:**
```
src-tauri/src/
  analytics/
    mod.rs
    hrv.rs
    exercise.rs
    statistics.rs
  export/
    mod.rs
    csv.rs
    json.rs
  commands/
    analytics.rs (new)

src/lib/
  stores/
    statistics.ts (new)
  components/
    StatisticsView.svelte (new)
    TimeDimensionTabs.svelte (new)
    HrvAnalysis.svelte (new)
```

**Anti-Patterns to Avoid:**
1. Fetch-all-then-compute (use SQL aggregations)
2. Real-time HRV calculation (compute on-demand)
3. Storing computed analytics (compute on-demand with caching)
4. Single monolithic analytics command (separate by type)

### From PITFALLS.md

**Critical Pitfalls:**

| Pitfall | Consequence | Prevention |
|---------|-------------|------------|
| HRV estimation without RR-intervals | Misleading health metrics | Label as "estimated", consider storing RR data |
| Unweighted averages in statistics | Skewed results | Time-weight aggregations, show data coverage |
| Static exercise thresholds | False positives/negatives | Personalized baselines after 3-7 days |
| Chart performance degradation | UI freezes on long sessions | Decimation from the start, disable animations |

**Current Code Issues Identified:**
1. `HeartRateChart.svelte`: 250ms throttle creates new arrays on each update; no decimation
2. `heart_rate.ts`: Session stats stored only in memory, lost on refresh
3. `HeartRateChart.svelte`: Auto Y-axis hides trend information
4. RR-intervals parsed but discarded in `heart_rate.rs`

**Phase-Specific Warnings:**
- HRV implementation: Must label as estimated if using BPM, or store RR-intervals for accuracy
- Exercise detection: Require baseline period before enabling auto-detection
- Statistics: Implement outlier filtering and time-weighting from the start
- Chart optimization: Decimation is essential, not optional

---

## Implications for Roadmap

### Recommended Phase Structure

**Phase 1: Foundation & Data Export** (3-5 days)

Rationale: Export is table stakes with no dependencies on other features. Database indexes needed for statistics. Statistics store establishes patterns.

Delivers:
- CSV/JSON data export with streaming
- Database indexes for time-based queries
- Statistics store skeleton with loading states

Features from FEATURES.md:
- Data export (CSV/JSON)

Pitfalls to avoid:
- Pitfall #9: Timezone handling - use ISO 8601, store UTC

**Phase 2: Core Statistics** (5-7 days)

Rationale: Daily statistics are the foundation for weekly/monthly. SQL-first approach established here is reused for all aggregations.

Delivers:
- Daily statistics with SQL aggregations
- Weekly/Monthly aggregations (extends daily pattern)
- Time-dimension UI tabs
- Statistics visualization

Features from FEATURES.md:
- Daily statistics
- Weekly/Monthly aggregations
- Configurable X-axis range
- Auto-scaling Y-axis

Pitfalls to avoid:
- Pitfall #2: Time-weighted averages, show data coverage
- Pitfall #5: Artifact/outlier filtering
- Pitfall #8: Offer both fixed and dynamic Y-axis modes

**Phase 3: Chart Enhancement** (2-3 days)

Rationale: Performance optimization must happen before advanced features add more data. Decimation is easier to implement correctly from the start.

Delivers:
- Chart decimation for long sessions
- Configurable X-axis range
- Fixed/dynamic Y-axis toggle
- Chart.js annotation and zoom plugins

Features from FEATURES.md:
- Configurable chart X-axis range
- Auto-scaling Y-axis with fixed mode option

Pitfalls to avoid:
- Pitfall #4: Chart performance degradation - decimation essential
- Pitfall #6: Fullscreen platform testing

**Phase 4: Advanced Analytics** (5-7 days)

Rationale: HRV and exercise detection depend on statistics being working. Personalization requires baseline data collection.

Delivers:
- HRV estimation (or accurate HRV if RR-intervals stored)
- Exercise detection with personalization
- Exercise vs resting analysis

Features from FEATURES.md:
- HRV estimation
- Automatic exercise detection
- Exercise vs resting analysis

Pitfalls to avoid:
- Pitfall #1: Label HRV clearly as estimated OR store RR-intervals
- Pitfall #3: Require 3-7 day baseline before auto-detection

**Phase 5: Polish & Refinement** (2-3 days)

Rationale: Final integration, edge case handling, fullscreen enhancement.

Delivers:
- Session persistence across refresh
- Personalized heart rate zones
- Cross-platform fullscreen testing

Features from FEATURES.md:
- Personalized zone thresholds

Pitfalls to avoid:
- Pitfall #7: Session stats persistence
- Pitfall #10: Configurable zone thresholds

### Research Flags

**Needs `/gsd:research-phase` during planning:**
- Phase 4 (HRV implementation): Decision point on storing RR-intervals for accurate HRV vs. estimation from BPM
- Phase 4 (Exercise detection): Research ML-based vs. rule-based detection approaches

**Standard patterns (skip additional research):**
- Phase 1 (Export): Well-documented streaming file generation
- Phase 2 (Statistics): SQLite window functions are standard
- Phase 3 (Chart optimization): Chart.js performance patterns are documented

---

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Core libraries verified from official sources; versions confirmed |
| Features | HIGH | Based on codebase analysis, health app patterns, NCBI research |
| Architecture | HIGH | SQL-first pattern is proven; existing codebase provides template |
| Pitfalls | HIGH | Sources include peer-reviewed research (NCBI), official docs (Chart.js), and direct codebase analysis |

**Gaps Requiring Attention:**

1. **RR-interval storage decision:** BLE code parses RR-intervals but doesn't store them. Must decide early whether to:
   - Store RR-intervals for accurate HRV (schema change, more storage)
   - Use BPM estimation only (limited accuracy, must label clearly)

2. **Exercise detection approach:** Research recommends hybrid (auto + manual), but specific algorithm (threshold-based vs. ML-based) needs design during Phase 4 planning.

3. **User baseline calculation:** Exercise detection requires personalized resting HR. Need to determine:
   - How many days of data before baseline is reliable
   - How to handle users with irregular patterns
   - Whether to ask for manual input or auto-detect

---

## Decision Points for Requirements Phase

1. **HRV Accuracy vs. Estimation**
   - Option A: Store RR-intervals, compute accurate HRV (more storage, better accuracy)
   - Option B: Estimate from BPM variance (less storage, must label as estimated)
   - Recommendation: Store RR-intervals - the BLE code already parses them

2. **Exercise Detection Mode**
   - Option A: Manual tagging only (simple, user control)
   - Option B: Auto-detection only (convenient, potential errors)
   - Option C: Hybrid with user confirmation (balanced)
   - Recommendation: Hybrid approach (from PITFALLS.md)

3. **Chart Y-Axis Strategy**
   - Option A: Fixed physiological range (40-180 BPM) for consistency
   - Option B: Dynamic auto-scaling for detail
   - Option C: User toggle between modes
   - Recommendation: User toggle (from PITFALLS.md)

---

## Sources

### High Confidence

- NCBI PMC Article on HRV Measurement (https://pmc.ncbi.nlm.nih.gov/articles/PMC5624990/) - HRV methodology
- Chart.js Performance Documentation - Optimization strategies
- SQLite Window Functions Documentation - Aggregation patterns
- ndarray, ndarray-stats, statrs documentation - Verified versions
- Chart.js plugin documentation - Verified versions
- Tauri plugin documentation - Verified versions
- Codebase analysis (src-tauri/src/, src/lib/) - Current implementation details

### Medium Confidence

- Whoop HRV Guide - Practical HRV measurement approach
- Apple Health feature patterns - Health app expectations
- simple-statistics documentation - Client-side statistics

### Requiring Validation

- Exercise detection thresholds (Karvonen formula) - Needs user testing
- HRV estimation accuracy from BPM - Needs validation against RR-interval data
- Personal baseline establishment period - Needs user testing

---

*Synthesis completed: 2026-03-21*