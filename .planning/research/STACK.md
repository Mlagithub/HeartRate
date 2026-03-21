# Technology Stack

**Project:** Heart Rate Receiver v2.0 - Analytics Enhancement
**Researched:** 2026-03-21
**Confidence:** MEDIUM (WebSearch had limited results; verified core libraries via official sources)

## Recommended Stack Additions

### Rust Backend - Data Analysis

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| `statrs` | 0.18.0 | Statistical distributions, tests | Provides distributions (Normal, Gamma, T) and statistical tests for HRV estimation and exercise detection algorithms |
| `ndarray` | 0.17.2 | N-dimensional arrays | Efficient numerical computing for time series data, enables SIMD and parallel processing via rayon feature |
| `ndarray-stats` | 0.7.0 | Array statistics | Order statistics (percentiles), correlation analysis, deviation measures - needed for heart rate zone calculations |
| `csv` | 1.x | CSV export | Fast CSV writer with Serde support for data export feature |

**Why Rust for analytics:** Compute-heavy operations (rolling statistics, HRV estimation, exercise detection) should run in the Rust backend for performance. The existing `rusqlite` database can be extended with aggregation queries, and statistical functions can process data efficiently without IPC overhead.

### Frontend - Visualization Enhancement

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| `chartjs-plugin-annotation` | 3.1.0 | Zone/threshold annotations | Draw heart rate zones, threshold lines, and labels directly on charts - essential for health data visualization |
| `chartjs-plugin-zoom` | 2.2.0 | Pan and zoom time series | Navigate long history data, zoom into specific sessions - user-requested feature for history analysis |
| `chartjs-adapter-date-fns` | 3.0.0 | Time scale with date-fns | Better time axis handling, locale support, integrates with existing date-fns dependency |
| `simple-statistics` | 7.8.9 | Client-side statistics | Moving averages, percentiles, standard deviation for quick frontend calculations without backend calls |

**Why not replace Chart.js:** Project constraint explicitly states "no chart library change." Chart.js 4.x + plugins provides sufficient capability for health data visualization without migration overhead.

### Data Export

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| `tauri-plugin-fs` | 2.0.0 | File system access | Write exported CSV/JSON files to user-selected locations |
| `tauri-plugin-dialog` | 2.0.0 | Save file dialogs | Native OS save dialog for export feature |
| `serde_json` | 1.0.149 | JSON export | Already in project; use for JSON export format |
| `csv` (Rust) | 1.x | CSV generation | Rust-side CSV generation with Serde serialization |

### Existing Stack (No Changes Required)

| Technology | Version | Purpose |
|------------|---------|---------|
| `chart.js` | 4.x | Real-time heart rate charting |
| `date-fns` | 4.1.0 | Date manipulation for time windows, aggregations |
| `rusqlite` | 0.32 | SQLite for persistent storage, time-range queries |
| `serde` / `serde_json` | 1.x | JSON serialization for IPC and export |

## Alternatives Considered

| Category | Recommended | Alternative | Why Not |
|----------|-------------|-------------|---------|
| Rust Dataframes | Custom queries + ndarray | Polars | Overkill for single-table heart rate data. Polars adds ~10MB binary size for features we won't use (lazy evaluation, SQL queries, multi-table joins). |
| JS Data Analysis | simple-statistics | Arquero 8.x | Arquero is powerful for dataframes but heavy (~200KB). We only need basic statistics (mean, stdev, percentiles) that simple-statistics provides in ~10KB. |
| JS Statistics | simple-statistics | jStat | jStat focuses on distributions; simple-statistics provides more practical summary statistics needed for health analytics. |
| Chart Time Scale | date-fns adapter | Luxon adapter | date-fns already in project; Luxon would add unnecessary dependency. |
| Rust Statistics | statrs + ndarray-stats | Pure ndarray implementations | statrs provides tested, accurate statistical functions. Rolling your own leads to numerical precision issues and edge case bugs. |

## HRV Estimation Approach

**Constraint:** Project specifies HRV "estimation from existing heart rate data" - not real RR-interval calculation.

**Recommended Algorithm:**
1. **SDNN (Standard Deviation of NN intervals):** Calculate from stored RR intervals using ndarray-stats
2. **RMSSD (Root Mean Square of Successive Differences):** Standard HRV metric; can be calculated from RR intervals
3. **Approximation from BPM:** If RR intervals unavailable, estimate HRV from BPM variance over short windows

**Implementation:**
```rust
// In Rust backend using statrs/ndarray-stats
fn calculate_rmssd(rr_intervals: &[f64]) -> f64 {
    let differences: Vec<f64> = rr_intervals
        .windows(2)
        .map(|w| (w[1] - w[0]).powi(2))
        .collect();
    (differences.iter().sum::<f64>() / differences.len() as f64).sqrt()
}
```

## Exercise Detection Approach

**Recommended:** Hybrid approach combining threshold-based detection with manual tagging.

**Algorithm (Backend):**
1. **Resting HR Baseline:** Calculate rolling minimum over 5-minute windows during detected low-activity periods
2. **Exercise Detection Thresholds:**
   - Light: 50-60% of HR reserve
   - Moderate: 60-70% of HR reserve
   - Vigorous: 70-85% of HR reserve
   - Maximum: 85%+ of HR reserve
3. **HR Reserve Calculation:** Use Karvonen formula: `HRR = HRmax - HRresting`

**Data Model Extension:**
```sql
-- Add to existing heart_rate_records table
ALTER TABLE heart_rate_records ADD COLUMN activity_type TEXT;
ALTER TABLE heart_rate_records ADD COLUMN estimated_intensity INTEGER; -- 0-100
```

## Time-Series Aggregation Strategy

**Database Layer (SQLite):**
```sql
-- Daily aggregation
SELECT date(timestamp/1000, 'unixepoch') as day,
       AVG(bpm) as avg_bpm,
       MIN(bpm) as min_bpm,
       MAX(bpm) as max_bpm,
       COUNT(*) as count
FROM heart_rate_records
GROUP BY day
ORDER BY day;

-- Weekly using SQLite's date functions
SELECT strftime('%Y-%W', timestamp/1000, 'unixepoch') as week, ...
```

**Frontend Layer (date-fns):**
- `startOfDay`, `endOfDay`, `startOfWeek`, `startOfMonth`, `startOfYear` for time boundaries
- `differenceInMinutes`, `differenceInHours` for duration calculations
- `format` with locale for display

## Installation

```toml
# Cargo.toml additions
[dependencies]
statrs = "0.18"
ndarray = { version = "0.17", features = ["rayon"] }
ndarray-stats = "0.7"
csv = "1"

# For file export
tauri-plugin-fs = "2"
tauri-plugin-dialog = "2"
```

```bash
# npm additions
npm install simple-statistics chartjs-plugin-annotation chartjs-plugin-zoom chartjs-adapter-date-fns papaparse
npm install -D @types/papaparse
```

## Sources

### Verified HIGH Confidence
- [ndarray docs](https://docs.rs/ndarray/latest/ndarray/) - Current version 0.17.2, verified 2026-03-21
- [ndarray-stats docs](https://docs.rs/ndarray-stats/latest/ndarray_stats/) - Current version 0.7.0, verified 2026-03-21
- [statrs docs](https://docs.rs/statrs/latest/statrs/) - Current version 0.18.0, verified 2026-03-21
- [serde_json docs](https://docs.rs/serde_json/latest/serde_json/) - Current version 1.0.149, verified 2026-03-21
- [date-fns releases](https://github.com/date-fns/date-fns/releases) - Current version 4.1.0, verified 2026-03-21
- [chartjs-plugin-annotation](https://github.com/chartjs/chartjs-plugin-annotation) - Current version 3.1.0, verified 2026-03-21
- [chartjs-plugin-zoom](https://github.com/chartjs/chartjs-plugin-zoom) - Current version 2.2.0, verified 2026-03-21
- [chartjs-adapter-date-fns](https://github.com/chartjs/chartjs-adapter-date-fns) - Current version 3.0.0, verified 2026-03-21
- [Papa Parse](https://www.papaparse.com/docs) - Current version 5, verified 2026-03-21
- [tauri-plugin-fs](https://github.com/tauri-apps/tauri-plugin-fs) - Current version 2.0.0, verified 2026-03-21
- [tauri-plugin-dialog](https://github.com/tauri-apps/tauri-plugin-dialog) - Current version 2.0.0, verified 2026-03-21

### MEDIUM Confidence (Official sources, version inferred)
- [simple-statistics](https://github.com/simple-statistics/simple-statistics) - Version 7.8.9, official GitHub
- [Arquero](https://github.com/uwdata/arquero) - Version 8.0.3, official GitHub
- [Polars](https://github.com/pola-rs/polars) - Version 1.39.3, official GitHub
- [rust-csv](https://github.com/BurntSushi/rust-csv) - Official GitHub (version from crates.io ~1.3)

### LOW Confidence (Algorithm approaches - training data)
- HRV estimation approaches (RMSSD, SDNN from RR intervals)
- Exercise detection threshold algorithms
- Heart rate zone calculation (Karvonen formula)

## Implementation Notes

### Phase 1: Statistics Backend
1. Add `statrs` and `ndarray-stats` to Cargo.toml
2. Create `src-tauri/src/analytics/` module with:
   - `statistics.rs` - Basic statistics (mean, stdev, percentiles)
   - `hrv.rs` - HRV estimation from RR intervals
   - `zones.rs` - Heart rate zone calculations
3. Extend database schema with aggregation views

### Phase 2: Enhanced Visualization
1. Add Chart.js plugins for annotations and zoom
2. Implement zone visualization using chartjs-plugin-annotation
3. Add time-range navigation with chartjs-plugin-zoom

### Phase 3: Export & Analytics UI
1. Implement CSV export using rust-csv + tauri-plugin-fs
2. Add simple-statistics for frontend calculations
3. Create statistics dashboard with time-dimension charts

---

*Research completed: 2026-03-21*