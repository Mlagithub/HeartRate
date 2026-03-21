# Phase 3: Advanced Analytics & Exercise Tracking - Context

**Gathered:** 2026-03-21
**Status:** Ready for planning

<domain>
## Phase Boundary

Provide exercise detection and HRV insights — manual exercise tagging with activity types, automatic threshold-based detection with inline confirmation, exercise vs resting heart rate analysis, and HRV estimation from BPM variance with clear labeling.

</domain>

<decisions>
## Implementation Decisions

### Manual Exercise Tagging
- **D-01:** Tag from History records — click session in records list to expand and tag
- **D-02:** 5 preset activity types — Running, Cycling, Swimming, Gym, Other
- **D-03:** Tag by session_id — one exercise tag per BLE session (not time ranges)
- **D-04:** Store exercise tags in database — add exercise_type column or join table

### Automatic Exercise Detection
- **D-05:** Threshold-based algorithm — sustained elevated HR above personal baseline for N minutes
- **D-06:** 7 days baseline required — detection activates only after 7+ days of data for personal baseline
- **D-07:** Inline prompt in History — "Was this exercise?" with confirm/dismiss buttons on detected sessions
- **D-08:** Confidence threshold — show detection only when confidence >= threshold (Claude's discretion on exact value)

### HRV Estimation Display
- **D-09:** BPM variance estimation — calculate from existing heart rate data, no RR-interval storage
- **D-10:** RMSSD-style single score — one HRV number in standard format
- **D-11:** "(estimated)" badge/label — clear indication that HRV is derived from BPM, not RR-intervals
- **D-12:** Display in Statistics tab — alongside other aggregated metrics

### Exercise Analysis Views
- **D-13:** Third [Exercise] tab in analytics — History/Statistics/Exercise tab navigation
- **D-14:** Comparison metrics — Avg exercise HR vs resting HR, Time in HR zones, Trend by exercise type
- **D-15:** Personal resting baseline — compare exercise against user's own resting heart rate average
- **D-16:** Chart.js for visualizations — consistent with existing chart patterns

### Claude's Discretion
- Exact threshold values for detection algorithm
- Confidence score display format
- HR zones calculation (percentage of max HR or fixed ranges)
- Exercise tab empty state design

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### UI Components
- `src/lib/components/HistoryView.svelte` — Tabbed analytics with History/Statistics, add Exercise tab
- `src/lib/components/StatisticsTab.svelte` — Statistics display pattern, add HRV metric
- `src/lib/components/HeartRateChart.svelte` — Chart.js pattern, zone backgrounds

### Data & Backend
- `src/lib/stores/statistics.ts` — Statistics store pattern, extend for exercise data
- `src-tauri/src/storage/database.rs` — SQLite schema, add exercise tagging storage
- `src-tauri/src/commands/handlers.rs` — Command pattern, add exercise-related commands

### Prior Phase Context
- `.planning/phases/02-core-statistics-analytics/02-CONTEXT.md` — Statistics patterns, D-01 through D-16
- `.planning/phases/01-ui-enhancement-data-export/01-CONTEXT.md` — UI patterns, D-01 through D-16

No external specs — requirements fully captured in decisions above.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `HistoryView.svelte`: Tab container with History/Statistics — add Exercise tab
- `StatisticsTab.svelte`: Summary cards pattern — reuse for exercise comparison
- `HeartRateChart.svelte`: Chart.js with zone backgrounds — reuse for exercise HR zones
- `statistics.ts` store: Aggregation loading — extend for exercise-specific queries
- `database.rs`: PeriodStats pattern — add exercise-related storage and queries

### Established Patterns
- Svelte stores with writable() and async load methods
- Tauri invoke() for backend commands
- Pill buttons for tab/selection UI
- Chart.js with time scale
- Teal/green color palette (#14b8a6)

### Integration Points
- HistoryView: Add Exercise tab to tab navigation
- Database: Add exercise_type column to heart_rate_records or exercise_tags table
- Backend: Add tag_exercise_session, detect_exercise, get_exercise_stats commands
- StatisticsTab: Add HRV estimation display

</code_context>

<specifics>
## Specific Ideas

- Exercise detection "Was this exercise?" prompts similar to activity recognition in fitness apps
- Activity type pills (Running/Cycling/Swimming/Gym/Other) consistent with time dimension pills
- HRV score with "(estimated)" badge clearly distinguishes from RR-interval based HRV

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 03-advanced-analytics-exercise-tracking*
*Context gathered: 2026-03-21*