# Architecture Patterns: Analytics Integration

**Domain:** Heart rate analytics for desktop Tauri application
**Researched:** 2026-03-21
**Confidence:** HIGH (based on existing codebase analysis + SQLite capabilities + established patterns)

## Recommended Architecture

### High-Level Component Diagram

```
+------------------+     IPC Commands      +---------------------+
|                  | --------------------> |                     |
|   Frontend       |                       |   Backend (Rust)    |
|   (Svelte/TS)    | <-------------------- |                     |
|                  |     Events/Results    |                     |
+------------------+                       +---------------------+
        |                                          |
        v                                          v
+------------------+                       +---------------------+
|   UI Stores      |                       |   Analytics Module  |
|   - stats        |                       |   - aggregations    |
|   - history      |                       |   - HRV estimation  |
|   - trends       |                       |   - exercise detect |
+------------------+                       +---------------------+
        |                                          |
        v                                          v
+------------------+                       +---------------------+
|   Chart.js       |                       |   SQLite Database   |
|   Visualizations |                       |   + Window Functions|
+------------------+                       +---------------------+
```

### Component Boundaries

| Component | Responsibility | Communicates With | Location |
|-----------|---------------|-------------------|----------|
| **Analytics Module** | Statistical calculations, aggregations, HRV estimation, exercise detection | Database, Commands layer | `src-tauri/src/analytics/` |
| **Statistics Store** | Frontend state for analytics data, caching, UI-derived calculations | Backend via IPC, UI components | `src/lib/stores/statistics.ts` |
| **Analytics Commands** | Tauri command handlers for analytics requests | Analytics Module, Frontend | `src-tauri/src/commands/analytics.rs` |
| **Export Service** | CSV/JSON generation, file writing | Analytics Module, OS filesystem | `src-tauri/src/export/` |
| **Statistics View** | UI for analytics display, charts, time dimension selectors | Statistics Store | `src/lib/components/StatisticsView.svelte` |
| **Enhanced Database** | Window function queries, time-based aggregations | Analytics Module | `src-tauri/src/storage/database.rs` |

---

## Where Calculations Live: Decision Matrix

### Backend (Rust) - Heavy Lifting

| Calculation | Why Backend | Implementation |
|-------------|-------------|----------------|
| Daily/Weekly/Monthly aggregations | Large dataset scans, SQL efficiency | SQLite window functions + aggregate queries |
| Moving averages | Window-based computation | SQLite `ROWS BETWEEN N PRECEDING AND CURRENT ROW` |
| Trend analysis | Statistical calculations on historical data | Rust algorithm, database query support |
| HRV estimation | Complex algorithm, needs consistent results | Rust implementation in analytics module |
| Exercise detection | Pattern recognition, configurable thresholds | Rust state machine |
| Export generation | File I/O, memory efficiency for large exports | Rust streaming to file |
| Percentile calculations | Requires full dataset or approximation | SQLite PERCENTILE or Rust computation |

### Frontend (Svelte/TypeScript) - Light Touch

| Calculation | Why Frontend | Implementation |
|-------------|-------------|----------------|
| Real-time session stats | Already implemented, low latency need | Existing `heartRate` store |
| Chart data transformations | Chart.js format needs, reactive updates | Store derived values |
| UI-specific derived state | Component-scoped, changes with UI | Svelte derived stores |
| Time range filtering (small sets) | Immediate UI feedback | Client-side array filtering |
| Color coding, zone classification | Pure UI concern | Component logic |

### Rationale

**SQL-first for aggregations:** SQLite window functions provide efficient computation without pulling all data into memory. For a desktop app with potentially months of heart rate data, computing `AVG(bpm) OVER (PARTITION BY date(timestamp))` in SQL is orders of magnitude faster than fetching all rows and computing in JavaScript.

**Rust for algorithms:** HRV estimation and exercise detection involve non-trivial algorithms. Rust ensures:
- Consistent floating-point results across platforms
- No GC pauses during computation
- Testable in isolation from UI
- Reusable for future features (notifications, background analysis)

**Frontend for presentation:** Chart.js transformations and UI state belong in the frontend. The frontend receives pre-aggregated data and focuses on rendering.

---

## Data Flow

### 1. Time-Dimension Statistics Flow

```
User selects "Weekly" tab
        |
        v
Frontend invokes get_statistics command
        |  Parameters: { dimension: "weekly", start_date, end_date }
        v
Backend Analytics Module
        |
        +---> Database query with window functions
        |     SELECT
        |       date(timestamp, 'weekday 0') as week_start,
        |       AVG(bpm), MIN(bpm), MAX(bpm), COUNT(*)
        |     FROM heart_rate_records
        |     WHERE timestamp BETWEEN ? AND ?
        |     GROUP BY date(timestamp, 'weekday 0')
        |
        v
Return WeeklyStatistics[] to frontend
        |
        v
Frontend Statistics Store caches result
        |
        v
Chart component renders
```

### 2. HRV Estimation Flow

```
User requests HRV analysis
        |
        v
Frontend invokes get_hrv_analysis command
        |  Parameters: { session_id } or { time_range }
        v
Backend Analytics Module
        |
        +---> Fetch heart rate data from database
        |
        +---> Apply HRV estimation algorithm:
        |     - Estimate RR intervals from BPM (60000 / bpm)
        |     - Calculate RMSSD: sqrt(mean(squared_diffs))
        |     - Calculate SDNN: std_dev(rr_intervals)
        |     - Calculate pNN50: percent diffs > 50ms
        |
        v
Return HrvAnalysis { rmssd, sdnn, pnn50, confidence }
        |
        v
Frontend displays with confidence indicator
```

**Note:** HRV from BPM is an estimation. True HRV requires RR intervals from BLE Heart Rate Measurement characteristic. The estimation provides basic insights but has limitations.

### 3. Exercise Detection Flow

```
Two modes:

MANUAL MODE:
User tags session as "Exercise" -> Backend stores tag with session

AUTO MODE:
Session completes or user requests detection
        |
        v
Backend Analytics Module
        |
        +---> Fetch session BPM data
        |
        +---> Apply detection algorithm:
        |     - Calculate session avg vs user's baseline
        |     - Detect sustained elevated periods (>10 min above threshold)
        |     - Check for recovery patterns
        |     - Consider time of day patterns
        |
        v
Return ExerciseDetection { detected: bool, confidence, suggested_type }
        |
        v
Frontend shows suggestion, user confirms or corrects
```

### 4. Export Flow

```
User clicks "Export" -> Selects format (CSV/JSON), date range
        |
        v
Frontend invokes export_data command
        |  Parameters: { format: "csv", start_date, end_date }
        v
Backend Export Service
        |
        +---> Query database with streaming cursor
        |
        +---> Write to temp file with streaming
        |     CSV: header + row-by-row
        |     JSON: streaming array format
        |
        +----- Return file path to frontend
        |
        v
Frontend triggers download via Tauri shell.open()
```

---

## Patterns to Follow

### Pattern 1: SQL-First Aggregations

**What:** Push aggregation logic to SQLite using window functions instead of fetching raw data.

**When:** Any time-based statistics, moving calculations, comparisons.

**Example:**
```sql
-- Daily statistics with SQLite window functions
SELECT
    date(timestamp / 1000, 'unixepoch') as day,
    COUNT(*) as record_count,
    AVG(bpm) as avg_bpm,
    MIN(bpm) as min_bpm,
    MAX(bpm) as max_bpm,
    -- Moving average over 7 days
    AVG(bpm) OVER (
        ORDER BY date(timestamp / 1000, 'unixepoch')
        ROWS BETWEEN 6 PRECEDING AND CURRENT ROW
    ) as moving_avg_7d
FROM heart_rate_records
GROUP BY date(timestamp / 1000, 'unixepoch')
ORDER BY day DESC;
```

**Why:** Avoids transferring large datasets to frontend. SQLite is optimized for these queries. Results are cached in frontend store.

### Pattern 2: Analytics Module Structure

**What:** Dedicated Rust module for analytics, separate from database and commands.

**When:** Non-trivial calculations that don't belong in SQL.

**Example:**
```rust
// src-tauri/src/analytics/mod.rs
pub mod hrv;
pub mod exercise;
pub mod statistics;

pub struct AnalyticsEngine {
    db: Arc<Database>,
}

impl AnalyticsEngine {
    pub fn calculate_hrv(&self, records: &[HeartRateRecord]) -> HrvResult {
        // Convert BPM to estimated RR intervals
        let rr_intervals: Vec<f64> = records.iter()
            .map(|r| 60000.0 / r.bpm as f64)
            .collect();

        hrv::calculate_rmssd(&rr_intervals)
    }

    pub fn detect_exercise(&self, session: &Session) -> ExerciseDetection {
        exercise::detect(session)
    }
}
```

### Pattern 3: Reactive Statistics Store

**What:** Frontend store that caches analytics results and invalidates appropriately.

**When:** Any analytics data displayed in UI.

**Example:**
```typescript
// src/lib/stores/statistics.ts
interface StatisticsState {
  daily: DailyStats[] | null;
  weekly: WeeklyStats[] | null;
  monthly: MonthlyStats[] | null;
  hrv: HrvAnalysis | null;
  isLoading: boolean;
  lastFetched: number | null;
}

function createStatisticsStore() {
  const { subscribe, update, set } = writable<StatisticsState>(initialState);

  async function fetchDailyStats(startDate: Date, endDate: Date) {
    update(s => ({ ...s, isLoading: true }));
    const result = await invoke<DailyStats[]>('get_daily_statistics', {
      startDate: startDate.getTime(),
      endDate: endDate.getTime()
    });
    update(s => ({ ...s, daily: result, isLoading: false, lastFetched: Date.now() }));
  }

  return { subscribe, fetchDailyStats, fetchWeeklyStats, fetchHrvAnalysis };
}
```

### Pattern 4: Export with Streaming

**What:** Stream large exports to file instead of building in memory.

**When:** Exporting more than 1000 records.

**Example:**
```rust
// src-tauri/src/export/mod.rs
pub fn export_to_csv(db: &Database, path: &Path, query: ExportQuery) -> Result<PathBuf> {
    let mut file = File::create(path)?;
    writeln!(file, "timestamp,bpm,session_id")?;

    // Stream records in batches
    let mut offset = 0;
    const BATCH_SIZE: i64 = 1000;

    loop {
        let records = db.get_history_range_batch(query.start, query.end, BATCH_SIZE, offset)?;
        if records.is_empty() { break; }

        for record in &records {
            writeln!(file, "{},{},{}", record.timestamp, record.bpm, record.session_id)?;
        }
        offset += BATCH_SIZE;
    }

    Ok(path.to_path_buf())
}
```

---

## Anti-Patterns to Avoid

### Anti-Pattern 1: Fetch-All-Then-Compute

**What:** Fetching all raw heart rate data to frontend and computing statistics in JavaScript.

**Why bad:**
- Memory pressure on frontend (JS heap)
- Network transfer overhead (IPC serialization)
- Slower than SQL-native aggregation
- Duplicated computation on each view

**Instead:** Use SQLite window functions and aggregate queries. Fetch only results.

### Anti-Pattern 2: Real-Time HRV Calculation

**What:** Computing HRV on every heart rate measurement in real-time.

**Why bad:**
- HRV requires sufficient data (minimum 60 seconds, ideally 5 minutes)
- Real-time updates would be noisy and misleading
- Unnecessary computation for display

**Instead:** Compute HRV on-demand when user views analysis, or batch-compute after session ends.

### Anti-Pattern 3: Storing Computed Analytics

**What:** Pre-computing and storing all analytics results in database.

**Why bad:**
- Stale data if computation changes
- Storage overhead
- Complex invalidation logic

**Instead:** Compute on-demand with caching in frontend store. Consider materialized views only for very expensive aggregations that are frequently accessed.

### Anti-Pattern 4: Single Monolithic Analytics Command

**What:** One `get_analytics` command that returns all statistics at once.

**Why bad:**
- Over-fetching when user only needs one dimension
- Slow response time
- Memory overhead

**Instead:** Separate commands for each analytics type:
- `get_daily_statistics`
- `get_weekly_statistics`
- `get_hrv_analysis`
- `detect_exercise`

---

## Scalability Considerations

| Concern | At 1K records | At 100K records | At 1M records |
|---------|---------------|-----------------|---------------|
| Daily aggregation query | < 10ms | < 50ms | < 200ms |
| Weekly aggregation query | < 10ms | < 30ms | < 100ms |
| HRV calculation (single session) | < 5ms | < 5ms | < 5ms |
| Full data export | Instant | ~100ms | ~1-2 seconds |
| Database size | < 100KB | ~10MB | ~100MB |
| Frontend memory (cached stats) | Minimal | Minimal | Minimal |

**Key insight:** SQL aggregations scale well. The frontend only holds aggregated results, not raw data. This architecture remains performant even with years of heart rate data.

---

## Module Structure

### New Backend Modules

```
src-tauri/src/
  analytics/
    mod.rs           # AnalyticsEngine struct, exports
    hrv.rs           # HRV estimation algorithms
    exercise.rs      # Exercise detection logic
    statistics.rs    # Time-dimension aggregations
  export/
    mod.rs           # Export service
    csv.rs           # CSV formatter
    json.rs          # JSON formatter
  commands/
    analytics.rs     # Analytics command handlers (new)
    handlers.rs      # Existing handlers (extend)
```

### New Frontend Modules

```
src/lib/
  stores/
    statistics.ts    # Analytics state management
  components/
    StatisticsView.svelte     # Main statistics page
    TimeDimensionTabs.svelte  # Daily/Weekly/Monthly tabs
    HrvAnalysis.svelte        # HRV display component
    ExerciseDetection.svelte  # Exercise tagging/detection UI
    ExportButton.svelte       # Export trigger component
```

---

## Build Order Implications

Based on dependencies, recommended build sequence:

### Phase 1: Foundation (No dependencies)
1. **Database schema enhancements** - Add indexes for time-based queries, potentially sessions table
2. **Analytics module skeleton** - Basic structure with placeholder functions
3. **Statistics store frontend** - Empty store with loading states

### Phase 2: Core Analytics (Depends on Phase 1)
4. **Daily statistics** - SQL queries + command + store integration + basic chart
5. **Weekly/Monthly aggregations** - Extends daily pattern
6. **Export functionality** - Independent, but uses same queries

### Phase 3: Advanced Features (Depends on Phase 2)
7. **HRV estimation** - Uses existing data, new algorithm module
8. **Exercise detection** - Requires statistics baseline, new detection logic
9. **Enhanced visualizations** - Depends on all analytics data

---

## Integration Points with Existing Architecture

### Event System (No changes needed)
- Existing `heart-rate-measurement` event continues for real-time display
- Analytics computed on-demand, not event-driven

### Database (Minimal changes)
- Existing schema works for analytics
- Add indexes: `CREATE INDEX IF NOT EXISTS idx_date ON heart_rate_records(date(timestamp/1000, 'unixepoch'))`
- Optional: Add `sessions` table if manual exercise tagging needs persistence

### Commands (Extension)
- Existing commands unchanged
- New `analytics.rs` for analytics-specific commands
- Register in `main.rs` alongside existing commands

### Stores (Extension)
- Existing stores unchanged
- New `statistics.ts` store for analytics
- No coupling between statistics store and existing stores

---

## Sources

- SQLite Window Functions Documentation - https://www.sqlite.org/windowfunctions.html (HIGH confidence)
- HRV Time-Domain Measures (RMSSD, SDNN, pNN50) - https://pmc.ncbi.nlm.nih.gov/articles/PMC5624990/ (HIGH confidence, peer-reviewed)
- Existing codebase analysis - `/mnt/d/work/heratbeat/src-tauri/src/` (HIGH confidence, direct observation)
- Tauri IPC patterns - Inferred from existing `commands/handlers.rs` (HIGH confidence, working code)