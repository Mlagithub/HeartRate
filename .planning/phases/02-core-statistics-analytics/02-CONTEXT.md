# Phase 2: Core Statistics & Analytics - Context

**Gathered:** 2026-03-21
**Status:** Ready for planning

<domain>
## Phase Boundary

Deliver time-dimension heart rate insights through aggregated statistics (daily/weekly/monthly/yearly) with trend analysis in a unified tabbed analytics interface. Users can view summary statistics, trend charts, and time-based comparisons.

</domain>

<decisions>
## Implementation Decisions

### Page Layout & Tabs
- **D-01:** Two tabs in analytics page — [History] for records list, [Statistics] for aggregations and trends
- **D-02:** Statistics tab layout — summary cards at top (min/max/avg), trend chart below
- **D-03:** Preserve existing History tab functionality — records list, export button, refresh
- **D-04:** Tab navigation at page level — consistent with existing app navigation pattern

### Time Dimension Navigation
- **D-05:** Pill buttons for time dimension — [Daily] [Weekly] [Monthly] [Yearly], consistent with Phase 1 time window pills
- **D-06:** Weekly view as default — balanced view of patterns without daily noise
- **D-07:** Time dimension selector in Statistics tab header — prominent, always visible

### Trend Charts
- **D-08:** Line chart for trends — best for continuous heart rate data over time
- **D-09:** Metrics per period — Min BPM, Max BPM, Avg BPM displayed
- **D-10:** Moving average trend line — 7-day window to smooth daily variations
- **D-11:** Record count per period — shown for data quality assessment
- **D-12:** Chart.js with same configuration pattern as HeartRateChart — consistent styling

### Aggregation Logic
- **D-13:** Aggregate by calendar period — all records within day/week/month/year boundaries grouped together
- **D-14:** Skip empty days in charts — no misleading zeros, cleaner visualization
- **D-15:** Trend indicator comparison — each period compared to previous equivalent period (this week vs last week)
- **D-16:** Calculate aggregates in backend — add aggregation commands to Rust handlers, not frontend computation

### Claude's Discretion
- Exact card styling and spacing for statistics display
- Trend indicator visual design (arrow, percentage, color)
- Empty state for statistics when no data available
- Transition animations between time dimensions

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### UI Components
- `src/lib/components/HistoryView.svelte` — Existing history page to transform into tabbed interface
- `src/lib/components/HeartRateChart.svelte` — Chart.js pattern, time window pills, teal color palette
- `src/app.css` — CSS variables, teal/green health palette (#14b8a6), card styling

### Data & Backend
- `src/lib/stores/history.ts` — History store pattern, invoke() for backend calls
- `src-tauri/src/storage/database.rs` — SQLite schema, get_history_range() for time-based queries
- `src-tauri/src/commands/handlers.rs` — Command pattern for Tauri IPC

### Prior Phase Context
- `.planning/phases/01-ui-enhancement-data-export/01-CONTEXT.md` — Established patterns, decisions D-01 through D-16

No external specs — requirements fully captured in decisions above.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `HistoryView.svelte`: Current history page — transform to tabbed layout, extract History tab content
- `HeartRateChart.svelte`: Chart.js with time window pills — reuse pill button pattern for dimension selector
- `history.ts` store: `loadHistory()` pattern — extend with statistics loading methods
- `database.rs`: `get_history_range()` — foundation for time-based queries, needs aggregation functions

### Established Patterns
- Svelte stores with writable() and async load methods
- Tauri invoke() for backend commands
- Glass-card styling with backdrop-filter
- Tab-based navigation (Monitor, History, Settings)
- Pill buttons for selection UI (from Phase 1)
- Teal/green color palette (#14b8a6)

### Integration Points
- HistoryView: Add tab container, extract History tab content, create Statistics tab
- Statistics tab: Add time dimension pills, statistics cards, trend chart
- Backend: Add aggregation commands (get_daily_stats, get_weekly_stats, etc.)
- Database: Add aggregation queries with MIN/MAX/AVG/COUNT

</code_context>

<specifics>
## Specific Ideas

- Health apps like Apple Health show summary cards with key metrics at top — similar dashboard layout
- Time dimension pills [Daily][Weekly][Monthly][Yearly] mirror the chart time window pills from Phase 1
- Trend indicators (↑ ↓) with percentage change are common in fitness tracking apps

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 02-core-statistics-analytics*
*Context gathered: 2026-03-21*