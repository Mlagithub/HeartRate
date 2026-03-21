# Phase 1: UI Enhancement & Data Export - Context

**Gathered:** 2026-03-21
**Status:** Ready for planning

<domain>
## Phase Boundary

Modernize the heart rate monitoring interface with a minimalist health-focused aesthetic, add configurable fullscreen display options, implement chart X-axis time window controls, and enable data export to CSV/JSON with date range selection via native Tauri dialogs.

</domain>

<decisions>
## Implementation Decisions

### Visual Design Style
- **D-01:** Shift to teal/green health-focused color palette — replace blue (#3b82f6) primary with calming teal/green tones appropriate for health monitoring
- **D-02:** Dark theme as default — darker backgrounds reduce eye strain for extended monitoring sessions
- **D-03:** More spacious/padded layout — increase padding and whitespace for minimalist health aesthetic
- **D-04:** Keep Inter font family — current typography works well; adjust weights for hierarchy

### Fullscreen Configuration
- **D-05:** Replace preset modes with checkbox toggles — individual toggles for "Show Chart" and "Show Stats" instead of 4 preset modes (simple/standard/stats/chart)
- **D-06:** Fullscreen configuration in Settings tab — persistent preferences stored in settings
- **D-07:** BPM display always visible in fullscreen — heart rate value is non-toggleable; chart and stats are optional
- **D-08:** Auto-scale Y-axis to session data — dynamic Y-axis range amplifies small fluctuations (72→75 BPM visible); current HeartRateChart behavior

### Chart X-axis Toggle
- **D-09:** Default view is recent N minutes — on session start, show recent time window, not all data
- **D-10:** Preset time windows: 2m, 5m, 10m, 30m, All — fixed options for time window selection
- **D-11:** Inline pill buttons for time window selection — [2m] [5m] [10m] [30m] [All] buttons in chart header, no dropdown
- **D-12:** Toggle available in both main view and fullscreen — consistent time window control across views

### Export Flow & Location
- **D-13:** Export feature in History tab — logically colocated with data viewing
- **D-14:** Modal dialog for export options — focused panel with format selection, date range, and export button
- **D-15:** Date presets + custom date picker — quick presets (Today, This Week, All Time) alongside custom date range picker
- **D-16:** CSV as default export format — JSON available as alternative option

### Claude's Discretion
- Exact teal/green hex values for color palette
- Specific padding/spacing values for spacious layout
- Export modal visual design
- Transition animations for theme changes

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### UI Components
- `src/app.css` — CSS variables for theming, color palette, spacing, animation keyframes
- `src/lib/components/FullscreenMode.svelte` — Existing fullscreen implementation with modes
- `src/lib/components/HeartRateChart.svelte` — Chart.js configuration, zone backgrounds, Y-axis auto-scaling
- `src/lib/stores/settings.ts` — Settings store pattern, fullscreen mode type definition

### Data & Export
- `src/lib/stores/history.ts` — History store for data access
- `src-tauri/src/storage/database.rs` — SQLite schema for heart rate records

No external specs — requirements fully captured in decisions above.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `FullscreenMode.svelte`: Existing fullscreen component with BPM display, mini-chart, stats — needs checkbox toggle UI instead of preset modes
- `HeartRateChart.svelte`: Chart.js with zone backgrounds, auto-scaling Y-axis, 120-point limit — needs time window toggle
- `settings.ts`: Store pattern with `loadSettings()`, `saveSettings()` — extend for fullscreen preferences
- `app.css`: CSS variable system (`--primary-color`, `--bg-color`, etc.) — update for teal/green palette

### Established Patterns
- Svelte stores with `writable()` and custom methods (`loadSettings`, `saveSettings`)
- Tauri `invoke()` for backend calls
- Glass-card styling with backdrop-filter
- Tab-based navigation (Monitor, History, Settings)

### Integration Points
- Settings tab: Add FullscreenSettings component alongside AlertSettings
- History tab: Add export button that opens ExportModal
- HeartRateChart: Add time window toggle in chart header
- FullscreenMode: Replace mode switch with checkbox-based visibility

</code_context>

<specifics>
## Specific Ideas

- Health apps like Apple Health use clean teal/green palettes — similar aesthetic
- Pill button groups like [2m] [5m] [10m] [30m] [All] are common in fitness apps
- Modal dialogs for export keep the main interface clean

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 01-ui-enhancement-data-export*
*Context gathered: 2026-03-21*