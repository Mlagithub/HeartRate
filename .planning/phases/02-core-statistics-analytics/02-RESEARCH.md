# Phase 2: Core Statistics & Analytics - Research

**Researched:** 2026-03-21
**Domain:** Time-dimension heart rate statistics with Chart.js trend visualization and SQLite aggregation
**Confidence:** HIGH

## Summary

This phase implements aggregated heart rate statistics across daily/weekly/monthly/yearly time dimensions with trend charts and moving averages. The implementation requires:

1. **Backend aggregation commands** in Rust using SQLite date functions to group records by calendar periods with MIN/MAX/AVG/COUNT
2. **Trend charts** using Chart.js with multiple datasets (raw values + 7-day moving average line)
3. **Tabbed analytics interface** transforming the existing HistoryView into a tabbed layout with History and Statistics tabs

The existing architecture provides a solid foundation: SQLite with timestamp indexes, Chart.js already integrated, established store patterns, and glass-card styling conventions.

**Primary recommendation:** Calculate aggregates in backend Rust handlers using SQLite strftime/date functions, add trend chart with dual datasets (values + moving average), and transform HistoryView into tabbed interface with local tab state.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

#### Page Layout & Tabs
- **D-01:** Two tabs in analytics page — [History] for records list, [Statistics] for aggregations and trends
- **D-02:** Statistics tab layout — summary cards at top (min/max/avg), trend chart below
- **D-03:** Preserve existing History tab functionality — records list, export button, refresh
- **D-04:** Tab navigation at page level — consistent with existing app navigation pattern

#### Time Dimension Navigation
- **D-05:** Pill buttons for time dimension — [Daily] [Weekly] [Monthly] [Yearly], consistent with Phase 1 time window pills
- **D-06:** Weekly view as default — balanced view of patterns without daily noise
- **D-07:** Time dimension selector in Statistics tab header — prominent, always visible

#### Trend Charts
- **D-08:** Line chart for trends — best for continuous heart rate data over time
- **D-09:** Metrics per period — Min BPM, Max BPM, Avg BPM displayed
- **D-10:** Moving average trend line — 7-day window to smooth daily variations
- **D-11:** Record count per period — shown for data quality assessment
- **D-12:** Chart.js with same configuration pattern as HeartRateChart — consistent styling

#### Aggregation Logic
- **D-13:** Aggregate by calendar period — all records within day/week/month/year boundaries grouped together
- **D-14:** Skip empty days in charts — no misleading zeros, cleaner visualization
- **D-15:** Trend indicator comparison — each period compared to previous equivalent period (this week vs last week)
- **D-16:** Calculate aggregates in backend — add aggregation commands to Rust handlers, not frontend computation

### Claude's Discretion
- Exact card styling and spacing for statistics display
- Trend indicator visual design (arrow, percentage, color)
- Empty state for statistics when no data available
- Transition animations between time dimensions

### Deferred Ideas (OUT OF SCOPE)
None — discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| STAT-01 | Daily statistics view showing min, max, avg heart rate with record count | SQLite aggregation by day using `date(timestamp, 'unixepoch')` with MIN/MAX/AVG/COUNT |
| STAT-02 | Weekly statistics view with aggregated metrics and trends | SQLite aggregation by week using `strftime('%Y-%W', timestamp, 'unixepoch')` |
| STAT-03 | Monthly statistics view with aggregated metrics and trends | SQLite aggregation by month using `strftime('%Y-%m', timestamp, 'unixepoch')` |
| STAT-04 | Yearly statistics view with aggregated metrics and trends | SQLite aggregation by year using `strftime('%Y', timestamp, 'unixepoch')` |
| STAT-07 | Time dimension charts showing trends over selected period | Chart.js line chart with multiple datasets (values + moving average), time scale axis |
| PAGE-01 | Transform history page into tabbed interface with "History" and "Statistics" tabs | Svelte reactive state with `activeSubTab` pattern, conditional rendering |
| PAGE-02 | Time dimension selector (daily/weekly/monthly) in statistics tab | Pill button pattern from HeartRateChart.svelte, same styling as D-05 |
| PAGE-04 | Trend analysis charts with moving averages | 7-day moving average as secondary dataset, calculated from aggregated daily values |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Chart.js | 4.5.1 | Trend chart visualization | Already in project, proven for heart rate charts |
| chartjs-adapter-date-fns | 3.0.0 | Time scale support for Chart.js | Required for proper time-based x-axis, integrates with existing date-fns |
| date-fns | 4.1.0 | Date formatting and manipulation | Already in project, used for timestamp formatting |
| rusqlite | 0.32 | SQLite aggregation queries | Already in project, has bundled feature for cross-platform |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| chrono | 0.4 | Rust timestamp handling | Date arithmetic in backend commands |
| serde | 1.0 | JSON serialization for IPC | Return aggregated statistics to frontend |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| chartjs-adapter-date-fns | chartjs-adapter-luxon | date-fns already in project, smaller bundle |
| Backend aggregation | Frontend aggregation | Backend handles large datasets better, keeps frontend lightweight |

**Installation:**
```bash
npm install chartjs-adapter-date-fns
```

**Version verification:**
- Chart.js: 4.5.1 (verified npm registry 2026-03-21)
- chartjs-adapter-date-fns: 3.0.0 (verified npm registry 2026-03-21)
- date-fns: 4.1.0 (verified npm registry 2026-03-21)
- rusqlite: 0.32 (verified Cargo.toml)
- chrono: 0.4 (verified Cargo.toml)

## Architecture Patterns

### Recommended Project Structure
```
src/
├── lib/
│   ├── components/
│   │   ├── HistoryView.svelte        # Transform to tabbed container
│   │   ├── HistoryTab.svelte         # Extract existing history list
│   │   ├── StatisticsTab.svelte      # New: statistics dashboard
│   │   └── StatisticsChart.svelte    # New: trend chart component
│   └── stores/
│       └── statistics.ts             # New: statistics store
src-tauri/
└── src/
    ├── storage/
    │   └── database.rs               # Add aggregation methods
    └── commands/
        └── handlers.rs               # Add statistics commands
```

### Pattern 1: SQLite Time-Based Aggregation
**What:** Group heart rate records by calendar periods using SQLite date functions
**When to use:** All statistics queries (daily/weekly/monthly/yearly)
**Example:**
```sql
-- Daily aggregation (Source: SQLite date functions documentation)
SELECT
    date(timestamp / 1000, 'unixepoch', 'localtime') as period_date,
    MIN(bpm) as min_bpm,
    MAX(bpm) as max_bpm,
    AVG(bpm) as avg_bpm,
    COUNT(*) as record_count
FROM heart_rate_records
WHERE timestamp >= ? AND timestamp <= ?
GROUP BY period_date
ORDER BY period_date ASC;

-- Weekly aggregation using ISO week number
SELECT
    strftime('%Y-%W', timestamp / 1000, 'unixepoch', 'localtime') as period_week,
    MIN(bpm) as min_bpm,
    MAX(bpm) as max_bpm,
    AVG(bpm) as avg_bpm,
    COUNT(*) as record_count
FROM heart_rate_records
GROUP BY period_week
ORDER BY period_week ASC;

-- Monthly aggregation
SELECT
    strftime('%Y-%m', timestamp / 1000, 'unixepoch', 'localtime') as period_month,
    MIN(bpm) as min_bpm,
    MAX(bpm) as max_bpm,
    AVG(bpm) as avg_bpm,
    COUNT(*) as record_count
FROM heart_rate_records
GROUP BY period_month
ORDER BY period_month ASC;
```

### Pattern 2: Chart.js Time Series with Multiple Datasets
**What:** Line chart with primary data and moving average overlay
**When to use:** Statistics trend chart
**Example:**
```typescript
// Source: Chart.js documentation - multiple datasets and time scale
import { Chart } from 'chart.js';
import 'chartjs-adapter-date-fns';

const chart = new Chart(ctx, {
  type: 'line',
  data: {
    datasets: [
      {
        label: 'Avg BPM',
        data: [{ x: '2026-03-01', y: 72 }, { x: '2026-03-02', y: 75 }], // Date objects or ISO strings
        borderColor: '#14b8a6',
        backgroundColor: 'rgba(20, 184, 166, 0.1)',
        fill: true,
        tension: 0.4,
        pointRadius: 4,
      },
      {
        label: '7-Day Moving Avg',
        data: [{ x: '2026-03-07', y: 73.5 }, { x: '2026-03-08', y: 74.2 }],
        borderColor: '#8b5cf6',
        borderWidth: 2,
        borderDash: [5, 5],
        pointRadius: 0,
        fill: false,
      },
    ],
  },
  options: {
    scales: {
      x: {
        type: 'time',
        time: {
          unit: 'day',
          displayFormats: { day: 'MMM d' },
        },
        grid: { color: 'rgba(51, 65, 85, 0.5)' },
        ticks: { color: '#94a3b8' },
      },
      y: {
        grid: { color: 'rgba(51, 65, 85, 0.5)' },
        ticks: { color: '#94a3b8' },
      },
    },
    plugins: {
      legend: { display: true, position: 'top' },
      tooltip: { mode: 'index', intersect: false },
    },
  },
});
```

### Pattern 3: Svelte Tab Component State
**What:** Local tab state with reactive switching
**When to use:** HistoryView transformation into tabbed interface
**Example:**
```svelte
<!-- Pattern from existing +page.svelte tab navigation -->
<script lang="ts">
  let activeSubTab: 'history' | 'statistics' = 'history';
  let selectedDimension: 'daily' | 'weekly' | 'monthly' | 'yearly' = 'weekly';
</script>

<div class="analytics-view glass-card">
  <!-- Sub-tabs within History page -->
  <div class="sub-tabs">
    <button
      class="sub-tab"
      class:active={activeSubTab === 'history'}
      on:click={() => activeSubTab = 'history'}
    >
      History
    </button>
    <button
      class="sub-tab"
      class:active={activeSubTab === 'statistics'}
      on:click={() => activeSubTab = 'statistics'}
    >
      Statistics
    </button>
  </div>

  {#if activeSubTab === 'history'}
    <HistoryTab />
  {:else}
    <StatisticsTab bind:dimension={selectedDimension} />
  {/if}
</div>
```

### Anti-Patterns to Avoid
- **Frontend aggregation of large datasets:** Fetching all records and computing stats in JavaScript is inefficient and blocks the UI. Always use backend SQLite aggregation.
- **Missing date adapter:** Chart.js time scale requires an adapter. Without `chartjs-adapter-date-fns`, the time axis will fail silently or show incorrect labels.
- **String labels for time axis:** Using formatted string labels (e.g., "Mar 21") instead of Date objects prevents proper time scale zooming and tooltip formatting.
- **Zero-filling empty periods:** Don't insert zero values for days with no data. Skip them entirely for cleaner charts and accurate moving averages.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Date grouping in SQLite | Custom timestamp parsing in Rust | SQLite `strftime()` and `date()` functions | Built-in, optimized, handles edge cases like week boundaries |
| Chart time axis | Manual label formatting | Chart.js time scale with date-fns adapter | Proper zooming, pan, and localized formatting |
| Moving average calculation | Custom array reduction in frontend | Simple arithmetic on aggregated backend data | Consistent calculation, no frontend complexity |
| Tab state management | Complex router setup | Svelte reactive variable (`let activeTab`) | Simple, matches existing app pattern |

**Key insight:** The aggregation logic belongs in SQLite, not application code. SQLite's date functions handle calendar boundaries correctly (e.g., week transitions, month boundaries) which are notoriously error-prone to implement manually.

## Common Pitfalls

### Pitfall 1: Millisecond vs Second Timestamp Mismatch
**What goes wrong:** SQLite `unixepoch` expects seconds, but the database stores milliseconds. Queries return NULL or incorrect dates.
**Why it happens:** The project stores `timestamp: i64` as milliseconds from JavaScript `Date.now()`, but SQLite date functions expect Unix epoch in seconds.
**How to avoid:** Divide by 1000 in SQL: `date(timestamp / 1000, 'unixepoch')` or store in seconds.
**Warning signs:** NULL values in period columns, dates showing as 1970.

### Pitfall 2: Missing Chart.js Date Adapter
**What goes wrong:** Time scale configuration silently fails, charts show no data or incorrect axis labels.
**Why it happens:** Chart.js 3+ requires an external date library adapter for time scales. Without it, the time scale falls back to category scale.
**How to avoid:** Install and import `chartjs-adapter-date-fns` before creating charts with `type: 'time'` axis.
**Warning signs:** Console warnings about missing adapter, x-axis showing indices instead of dates.

### Pitfall 3: Moving Average Window Edge Cases
**What goes wrong:** Moving average line starts from day 1 instead of day 7, showing misleading early values.
**Why it happens:** Applying moving average without waiting for enough data points creates incomplete averages.
**How to avoid:** Only start the moving average line from the 7th data point onward, or pre-fill with null values for the first 6 points.
**Warning signs:** Moving average line extending to chart left edge, unusual early values.

### Pitfall 4: Timezone Inconsistency
**What goes wrong:** Daily statistics group by UTC day instead of local day, causing records near midnight to appear in wrong period.
**Why it happens:** SQLite `date()` without `'localtime'` modifier uses UTC.
**How to avoid:** Always include `'localtime'` modifier: `date(timestamp / 1000, 'unixepoch', 'localtime')`.
**Warning signs:** User sees records from yesterday showing in today's statistics.

## Code Examples

### SQLite Aggregation Function (Rust)
```rust
// Source: SQLite date functions + rusqlite patterns
// Add to src-tauri/src/storage/database.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodStats {
    pub period: String,        // ISO date string (YYYY-MM-DD, YYYY-WW, YYYY-MM, YYYY)
    pub min_bpm: u16,
    pub max_bpm: u16,
    pub avg_bpm: f64,
    pub record_count: i64,
}

impl Database {
    /// Get aggregated statistics by time dimension
    pub fn get_statistics(
        &self,
        dimension: &str,  // "daily", "weekly", "monthly", "yearly"
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> SqliteResult<Vec<PeriodStats>> {
        let conn = self.get_conn()?;

        let period_expr = match dimension {
            "daily" => "date(timestamp / 1000, 'unixepoch', 'localtime')",
            "weekly" => "strftime('%Y-%W', timestamp / 1000, 'unixepoch', 'localtime')",
            "monthly" => "strftime('%Y-%m', timestamp / 1000, 'unixepoch', 'localtime')",
            "yearly" => "strftime('%Y', timestamp / 1000, 'unixepoch', 'localtime')",
            _ => return Err(rusqlite::Error::InvalidParameterName("Invalid dimension".into())),
        };

        let sql = format!(
            "SELECT
                {} as period,
                MIN(bpm) as min_bpm,
                MAX(bpm) as max_bpm,
                AVG(bpm) as avg_bpm,
                COUNT(*) as record_count
             FROM heart_rate_records
             WHERE (?1 IS NULL OR timestamp >= ?1)
               AND (?2 IS NULL OR timestamp <= ?2)
             GROUP BY period
             ORDER BY period ASC",
            period_expr
        );

        let mut stmt = conn.prepare(&sql)?;
        let stats = stmt
            .query_map(params![start_time, end_time], |row| {
                Ok(PeriodStats {
                    period: row.get(0)?,
                    min_bpm: row.get::<_, i64>(1)? as u16,
                    max_bpm: row.get::<_, i64>(2)? as u16,
                    avg_bpm: row.get(3)?,
                    record_count: row.get(4)?,
                })
            })?
            .collect::<SqliteResult<Vec<_>>>()?;

        Ok(stats)
    }
}
```

### Tauri Command for Statistics
```rust
// Source: Existing command patterns in handlers.rs
// Add to src-tauri/src/commands/handlers.rs

#[tauri::command]
pub async fn get_heart_rate_statistics(
    db: State<'_, Database>,
    dimension: String,
    start_time: Option<i64>,
    end_time: Option<i64>,
) -> Result<Vec<PeriodStats>, String> {
    db.get_statistics(&dimension, start_time, end_time)
        .map_err(|e| format!("Failed to get statistics: {}", e))
}
```

### Statistics Store (TypeScript)
```typescript
// Source: Existing history.ts store pattern
// Create src/lib/stores/statistics.ts

import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface PeriodStats {
  period: string;
  min_bpm: number;
  max_bpm: number;
  avg_bpm: number;
  record_count: number;
}

export interface StatisticsState {
  stats: PeriodStats[];
  isLoading: boolean;
  error: string | null;
  dimension: 'daily' | 'weekly' | 'monthly' | 'yearly';
}

function createStatisticsStore() {
  const initialState: StatisticsState = {
    stats: [],
    isLoading: false,
    error: null,
    dimension: 'weekly',
  };

  const { subscribe, update, set } = writable<StatisticsState>(initialState);

  async function loadStatistics(
    dimension: 'daily' | 'weekly' | 'monthly' | 'yearly',
    startTime?: number,
    endTime?: number
  ) {
    update((state) => ({ ...state, isLoading: true, error: null, dimension }));
    try {
      const stats = await invoke<PeriodStats[]>('get_heart_rate_statistics', {
        dimension,
        startTime,
        endTime,
      });
      update((state) => ({ ...state, stats, isLoading: false }));
    } catch (error) {
      update((state) => ({
        ...state,
        isLoading: false,
        error: String(error),
      }));
    }
  }

  return {
    subscribe,
    loadStatistics,
    reset: () => set(initialState),
  };
}

export const statistics = createStatisticsStore();
```

### Trend Chart Component
```svelte
<!-- Source: Existing HeartRateChart.svelte pattern + Chart.js time scale -->
<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { statistics } from '$lib/stores/statistics';
  import Chart from 'chart.js/auto';
  import 'chartjs-adapter-date-fns';

  let canvas: HTMLCanvasElement;
  let chart: Chart | null = null;

  function calculateMovingAverage(data: { x: string; y: number }[], window: number = 7) {
    return data.map((point, i, arr) => {
      if (i < window - 1) return null;
      const sum = arr.slice(i - window + 1, i + 1).reduce((acc, p) => acc + p.y, 0);
      return { x: point.x, y: sum / window };
    }).filter(Boolean);
  }

  function initChart() {
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    chart = new Chart(ctx, {
      type: 'line',
      data: {
        datasets: [
          {
            label: 'Average BPM',
            data: [],
            borderColor: '#14b8a6',
            backgroundColor: 'rgba(20, 184, 166, 0.1)',
            fill: true,
            tension: 0.4,
            pointRadius: 4,
          },
          {
            label: '7-Day Moving Avg',
            data: [],
            borderColor: '#8b5cf6',
            borderWidth: 2,
            borderDash: [5, 5],
            pointRadius: 0,
            fill: false,
          },
        ],
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        scales: {
          x: {
            type: 'time',
            time: { unit: 'day', displayFormats: { day: 'MMM d' } },
            grid: { color: 'rgba(51, 65, 85, 0.5)' },
            ticks: { color: '#94a3b8', maxTicksLimit: 10 },
          },
          y: {
            grid: { color: 'rgba(51, 65, 85, 0.5)' },
            ticks: { color: '#94a3b8' },
          },
        },
        plugins: {
          legend: { display: true, labels: { color: '#94a3b8' } },
          tooltip: { mode: 'index', intersect: false },
        },
      },
    });
  }

  function updateChart() {
    if (!chart || $statistics.stats.length === 0) return;

    const avgData = $statistics.stats.map(s => ({ x: s.period, y: s.avg_bpm }));
    const movingAvgData = calculateMovingAverage(avgData);

    chart.data.datasets[0].data = avgData;
    chart.data.datasets[1].data = movingAvgData;
    chart.update();
  }

  let unsubscribe: (() => void) | null = null;

  onMount(() => {
    initChart();
    unsubscribe = statistics.subscribe(updateChart);
  });

  onDestroy(() => {
    if (unsubscribe) unsubscribe();
    if (chart) chart.destroy();
  });
</script>

<div class="chart-wrapper">
  <canvas bind:this={canvas}></canvas>
</div>

<style>
  .chart-wrapper {
    height: 300px;
    position: relative;
  }
</style>
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| String labels for chart x-axis | Chart.js time scale with date adapter | Chart.js 3.0+ | Proper zoom, pan, localized formatting |
| Frontend data aggregation | Backend SQLite aggregation | Best practice | Handles large datasets, consistent results |
| Custom date parsing | SQLite date functions | SQLite 3.38+ | Built-in timezone handling, calendar-aware |

**Deprecated/outdated:**
- Moment.js: Use date-fns instead (smaller, immutable, tree-shakeable)
- Chart.js category scale for time data: Use time scale with adapter

## Open Questions

1. **Moving Average Start Point**
   - What we know: D-10 specifies 7-day window
   - What's unclear: Should moving average line start from day 1 (with partial window) or day 7 (full window only)?
   - Recommendation: Start from day 7 with full window for accuracy, show null for first 6 points

2. **Statistics Empty State**
   - What we know: Need empty state for no data
   - What's unclear: Exact messaging and visual design (Claude's discretion)
   - Recommendation: Match existing HistoryView empty state pattern with statistics-specific messaging

## Validation Architecture

> Skipped per workflow.nyquist_validation = false in config.json

## Sources

### Primary (HIGH confidence)
- SQLite Date Functions (https://www.sqlite.org/lang_datefunc.html) - strftime, date, datetime functions verified
- Chart.js Line Chart Documentation (https://www.chartjs.org/docs/latest/charts/line.html) - Multiple datasets, styling options
- Chart.js Time Scale (https://www.chartjs.org/docs/latest/axes/cartesian/time.html) - Time axis configuration, adapter requirement
- chartjs-adapter-date-fns README (https://github.com/chartjs/chartjs-adapter-date-fns) - Installation and configuration

### Secondary (MEDIUM confidence)
- Existing codebase patterns (HeartRateChart.svelte, HistoryView.svelte, history.ts, database.rs) - Verified architecture patterns

### Tertiary (LOW confidence)
- None - all findings verified with official documentation

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - All libraries already in project or verified npm versions
- Architecture: HIGH - Patterns match existing codebase conventions
- Pitfalls: HIGH - Based on documented SQLite/Chart.js behavior and existing project patterns

**Research date:** 2026-03-21
**Valid until:** 30 days - Stable libraries (Chart.js 4.x, SQLite 3.x, Svelte 5.x)