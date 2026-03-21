# Phase 1: UI Enhancement & Data Export - Research

**Researched:** 2026-03-21
**Domain:** Tauri 2.x Desktop Application (UI Design, File Export, Chart.js Configuration)
**Confidence:** HIGH

## Summary

Phase 1 modernizes the heart rate monitoring interface with a minimalist health-focused aesthetic (teal/green palette, dark theme default, spacious layout), replaces fullscreen preset modes with checkbox-based toggles, adds chart X-axis time window controls (2m/5m/10m/30m/All), and enables data export to CSV/JSON with native Tauri file dialogs.

**Primary recommendation:** Install `@tauri-apps/plugin-dialog` and `@tauri-apps/plugin-fs` for export functionality; use `chartjs-adapter-date-fns` for time-based X-axis configuration; update CSS variables for teal/green health palette while preserving zone colors.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Shift to teal/green health-focused color palette - replace blue (#3b82f6) primary with calming teal/green tones appropriate for health monitoring
- **D-02:** Dark theme as default - darker backgrounds reduce eye strain for extended monitoring sessions
- **D-03:** More spacious/padded layout - increase padding and whitespace for minimalist health aesthetic
- **D-04:** Keep Inter font family - current typography works well; adjust weights for hierarchy
- **D-05:** Replace preset modes with checkbox toggles - individual toggles for "Show Chart" and "Show Stats" instead of 4 preset modes (simple/standard/stats/chart)
- **D-06:** Fullscreen configuration in Settings tab - persistent preferences stored in settings
- **D-07:** BPM display always visible in fullscreen - heart rate value is non-toggleable; chart and stats are optional
- **D-08:** Auto-scale Y-axis to session data - dynamic Y-axis range amplifies small fluctuations (72 to 75 BPM visible); current HeartRateChart behavior
- **D-09:** Default view is recent N minutes - on session start, show recent time window, not all data
- **D-10:** Preset time windows: 2m, 5m, 10m, 30m, All - fixed options for time window selection
- **D-11:** Inline pill buttons for time window selection - [2m] [5m] [10m] [30m] [All] buttons in chart header, no dropdown
- **D-12:** Toggle available in both main view and fullscreen - consistent time window control across views
- **D-13:** Export feature in History tab - logically colocated with data viewing
- **D-14:** Modal dialog for export options - focused panel with format selection, date range, and export button
- **D-15:** Date presets + custom date picker - quick presets (Today, This Week, All Time) alongside custom date range picker
- **D-16:** CSV as default export format - JSON available as alternative option

### Claude's Discretion
- Exact teal/green hex values for color palette
- Specific padding/spacing values for spacious layout
- Export modal visual design
- Transition animations for theme changes

### Deferred Ideas (OUT OF SCOPE)
None - discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| UI-01 | Minimalist health-focused visual design with clean typography, ample whitespace, and calming color palette | CSS variable system in `app.css`; teal/green palette recommendations; existing Inter font retained |
| UI-02 | Full-screen mode with auto-scaling Y-axis that amplifies small heart rate fluctuations | Existing `HeartRateChart.svelte` already implements auto-scaling Y-axis via `calculateYAxisRange()` |
| UI-03 | Configurable full-screen UI elements (heart rate value, curve, session stats, status info) configurable in main settings | Settings store pattern in `settings.ts`; checkbox toggle pattern replaces preset modes |
| UI-04 | Heart rate curve X-axis toggle: recent N minutes (customizable by user) vs all data | Chart.js time scale with `min`/`max` bounds; `chartjs-adapter-date-fns` required |
| EXP-01 | Export heart rate data to CSV format with timestamp, BPM, session ID columns | Tauri dialog + fs plugins; backend `get_history_range()` query exists |
| EXP-02 | Export heart rate data to JSON format | Same export infrastructure as CSV; JSON serialization via `serde_json` |
| EXP-03 | Date range selection for exports (all time, specific period) | Date picker component; date-fns for date manipulation; backend query supports time range |
| EXP-04 | Native file save dialog via Tauri | `@tauri-apps/plugin-dialog` v2.6.0 with `save()` function |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `@tauri-apps/plugin-dialog` | 2.6.0 | Native file save dialogs | Official Tauri plugin for desktop file dialogs |
| `@tauri-apps/plugin-fs` | 2.4.5 | File system write operations | Official Tauri plugin for writing exported files |
| `chartjs-adapter-date-fns` | 3.0.0 | Time scale support for Chart.js | Required for time-based X-axis configuration |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `date-fns` | 4.1.0 (installed) | Date manipulation for exports and time windows | Already installed; used for timestamp formatting |
| `chart.js` | 4.5.1 (installed) | Chart rendering with time scale | Already installed; add time scale adapter |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Tauri dialog plugin | Web-based download | Native dialogs feel more desktop-appropriate; better UX |
| chartjs-adapter-date-fns | chartjs-adapter-luxon | date-fns already installed; smaller bundle |
| Custom export backend | Frontend-only blob download | Backend handles large datasets better; streaming support |

**Installation:**
```bash
npm install @tauri-apps/plugin-dialog @tauri-apps/plugin-fs chartjs-adapter-date-fns
```

**Rust additions (Cargo.toml):**
```toml
tauri-plugin-dialog = "2"
tauri-plugin-fs = "2"
```

**Rust initialization (lib.rs):**
```rust
.plugin(tauri_plugin_dialog::init())
.plugin(tauri_plugin_fs::init())
```

**Version verification:** Verified via npm registry on 2026-03-21:
- `chart.js`: 4.5.1
- `chartjs-adapter-date-fns`: 3.0.0
- `@tauri-apps/plugin-dialog`: 2.6.0
- `@tauri-apps/plugin-fs`: 2.4.5
- `date-fns`: 4.1.0

## Architecture Patterns

### Recommended Project Structure
```
src/
├── lib/
│   ├── components/
│   │   ├── ExportModal.svelte      # NEW: Export options modal
│   │   ├── FullscreenSettings.svelte # NEW: Checkbox toggles for fullscreen
│   │   ├── HeartRateChart.svelte   # MODIFY: Add time window toggle
│   │   ├── HistoryView.svelte      # MODIFY: Add export button
│   │   └── FullscreenMode.svelte   # MODIFY: Checkbox-based visibility
│   └── stores/
│       └── settings.ts             # MODIFY: Add fullscreen preferences
src-tauri/
└── src/
    ├── commands/
    │   └── handlers.rs             # MODIFY: Add export data command
    └── storage/
        └── database.rs             # Existing: get_history_range() available
```

### Pattern 1: Tauri Dialog + FS Export Flow
**What:** Use native save dialog to get file path, then write content via fs plugin
**When to use:** All file export operations
**Example:**
```typescript
// Source: https://v2.tauri.app/plugin/dialog/
import { save } from '@tauri-apps/plugin-dialog';
import { writeTextFile } from '@tauri-apps/plugin-fs';

async function exportData(records: HeartRateRecord[], format: 'csv' | 'json') {
  const extension = format === 'csv' ? 'csv' : 'json';
  const path = await save({
    filters: [{
      name: format.toUpperCase(),
      extensions: [extension]
    }],
    defaultPath: `heart-rate-export.${extension}`
  });

  if (!path) return; // User cancelled

  const content = format === 'csv'
    ? recordsToCsv(records)
    : JSON.stringify(records, null, 2);

  await writeTextFile(path, content);
}
```

### Pattern 2: Chart.js Time Scale Configuration
**What:** Configure X-axis with time scale and dynamic min/max for time windows
**When to use:** Implementing time window toggle (2m/5m/10m/30m/All)
**Example:**
```typescript
// Source: https://www.chartjs.org/docs/latest/axes/cartesian/time.html
import 'chartjs-adapter-date-fns';

// Chart configuration
{
  type: 'line',
  data: {
    datasets: [{
      data: history.map(h => ({ x: h.timestamp, y: h.bpm }))
    }]
  },
  options: {
    scales: {
      x: {
        type: 'time',
        time: {
          unit: 'minute',
          displayFormats: { minute: 'HH:mm:ss' }
        },
        min: timeWindowStart,  // Dynamic based on selected window
        max: Date.now()
      }
    }
  }
}
```

### Pattern 3: Fullscreen Checkbox Settings
**What:** Replace preset modes with individual boolean toggles
**When to use:** Fullscreen configuration in Settings tab
**Example:**
```typescript
// settings.ts - Updated interface
export interface FullscreenPreferences {
  showChart: boolean;
  showStats: boolean;
  // BPM always visible (D-07)
}

// FullscreenSettings.svelte
<div class="fullscreen-settings">
  <label class="toggle-setting">
    <span>Show Chart</span>
    <input type="checkbox" bind:checked={preferences.showChart} />
  </label>
  <label class="toggle-setting">
    <span>Show Stats</span>
    <input type="checkbox" bind:checked={preferences.showStats} />
  </label>
</div>
```

### Pattern 4: Time Window Pill Buttons
**What:** Inline pill buttons for time window selection
**When to use:** Chart X-axis toggle in main view and fullscreen
**Example:**
```svelte
<div class="time-window-pills">
  {#each ['2m', '5m', '10m', '30m', 'All'] as window}
    <button
      class="pill"
      class:active={selectedWindow === window}
      on:click={() => setTimeWindow(window)}
    >
      {window}
    </button>
  {/each}
</div>

<style>
  .time-window-pills {
    display: flex;
    gap: 8px;
  }
  .pill {
    padding: 6px 12px;
    border-radius: 16px;
    background: var(--card-bg);
    border: 1px solid var(--border-color);
    color: var(--text-secondary);
    font-size: 12px;
    font-weight: 500;
  }
  .pill.active {
    background: var(--primary-color);
    color: white;
    border-color: var(--primary-color);
  }
</style>
```

### Anti-Patterns to Avoid
- **Fetching all data then filtering client-side:** Use backend `get_history_range()` with time bounds
- **Storing fullscreen mode as string enum:** Use boolean preferences for granular control
- **Using Date objects in Chart.js data:** Use timestamps (numbers) for time scale; adapter handles conversion
- **Hardcoding time windows:** Make window durations configurable via constants

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| File save dialog | Custom modal with path input | `@tauri-apps/plugin-dialog` | Native OS dialog, handles permissions, user familiarity |
| File writing | XMLHttpRequest or fetch blob | `@tauri-apps/plugin-fs` | Proper file system access, streaming support |
| Time scale in Chart.js | Custom X-axis formatting | `chartjs-adapter-date-fns` | Handles timezone, formatting, zoom/pan compatibility |
| CSV generation | String concatenation | Template literal with proper escaping | Handle commas, quotes, newlines in data |
| Date range picker | Custom calendar component | HTML `<input type="date">` + date-fns | Native browser support, accessibility |

**Key insight:** Tauri plugins handle platform differences (Windows/macOS/Linux) automatically; custom solutions would need per-platform testing.

## Common Pitfalls

### Pitfall 1: Time Zone Handling in Exports
**What goes wrong:** Exported timestamps appear in wrong timezone, confusing users
**Why it happens:** Storing local time instead of UTC, or not specifying timezone in export
**How to avoid:** Store timestamps as UTC milliseconds (existing pattern); export with ISO 8601 format; include timezone note in export file
**Warning signs:** Users report "wrong time" in exported data

### Pitfall 2: Large Export Performance
**What goes wrong:** UI freezes when exporting thousands of records
**Why it happens:** Generating CSV/JSON string in memory before writing
**How to avoid:** For large datasets (>10k records), implement streaming export via Rust backend using `csv` crate; show progress indicator
**Warning signs:** Export takes >5 seconds, UI becomes unresponsive

### Pitfall 3: Chart Time Scale Migration
**What goes wrong:** Existing chart breaks when adding time scale adapter
**Why it happens:** Data format changes from `labels: string[]` to `data: {x, y}[]`
**How to avoid:** Migrate chart data to `{x: timestamp, y: bpm}` format; test with existing zone background plugin
**Warning signs:** Chart shows "Invalid date" labels, data points disappear

### Pitfall 4: Fullscreen State Persistence
**What goes wrong:** Fullscreen preferences reset on app restart
**Why it happens:** Settings not persisted to backend storage
**How to avoid:** Extend settings store to include `fullscreenPreferences`; consider using `tauri-plugin-store` for persistence or extend SQLite schema
**Warning signs:** Preferences lost after closing app

### Pitfall 5: Export Modal Accessibility
**What goes wrong:** Modal cannot be closed with keyboard, traps focus incorrectly
**Why it happens:** Missing ARIA attributes, improper focus management
**How to avoid:** Use `aria-modal="true"`, `role="dialog"`, trap focus within modal, handle Escape key
**Warning signs:** Tab navigation escapes modal, screen reader doesn't announce modal

## Code Examples

Verified patterns from official sources:

### Tauri Save Dialog with Filters
```typescript
// Source: https://v2.tauri.app/plugin/dialog/
import { save } from '@tauri-apps/plugin-dialog';

const path = await save({
  filters: [
    { name: 'CSV', extensions: ['csv'] },
    { name: 'JSON', extensions: ['json'] }
  ],
  defaultPath: 'heart-rate-export'
});
// Returns: string path or null if cancelled
```

### Tauri File Write
```typescript
// Source: https://v2.tauri.app/plugin/file-system/
import { writeTextFile } from '@tauri-apps/plugin-fs';

await writeTextFile(path, content);
// No baseDir needed when using absolute path from save dialog
```

### Chart.js Time Scale with Dynamic Window
```typescript
// Source: https://www.chartjs.org/docs/latest/axes/cartesian/time.html
import 'chartjs-adapter-date-fns';

const timeWindows = {
  '2m': 2 * 60 * 1000,
  '5m': 5 * 60 * 1000,
  '10m': 10 * 60 * 1000,
  '30m': 30 * 60 * 1000,
  'All': Infinity
};

function getTimeWindowBounds(window: string) {
  const now = Date.now();
  const duration = timeWindows[window];
  return {
    min: duration === Infinity ? undefined : now - duration,
    max: now
  };
}
```

### CSV Generation from Records
```typescript
function recordsToCsv(records: HeartRateRecord[]): string {
  const headers = ['timestamp', 'bpm', 'session_id', 'sensor_contact'];
  const rows = records.map(r => [
    new Date(r.timestamp).toISOString(),
    r.bpm,
    r.session_id,
    r.sensor_contact ?? ''
  ].join(','));

  return [headers.join(','), ...rows].join('\n');
}
```

### Health-Focused Teal Color Palette
```css
/* Recommended teal/green palette for health monitoring */
:root {
  /* Primary - Teal */
  --primary-color: #14b8a6;      /* teal-500 */
  --primary-hover: #0d9488;      /* teal-600 */
  --primary-light: rgba(20, 184, 166, 0.15);

  /* Glow effects */
  --glow-primary: 0 0 20px rgba(20, 184, 166, 0.3);

  /* Zone colors - KEEP EXISTING */
  --zone-rest: #94a3b8;
  --zone-fat-burn: #22c55e;      /* Green for healthy zone */
  --zone-cardio: #f59e0b;
  --zone-peak: #ef4444;
  --zone-extreme: #dc2626;
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Preset fullscreen modes | Checkbox toggles | Phase 1 | More granular user control |
| Blue primary color | Teal/green health palette | Phase 1 | Health-focused aesthetic |
| Fixed 120-point chart limit | Time window toggle | Phase 1 | User control over data visibility |
| No export capability | Native file export | Phase 1 | Data portability |

**Deprecated/outdated:**
- `FullscreenMode` type (`'simple' | 'standard' | 'stats' | 'chart'`): Replace with `FullscreenPreferences` interface
- Hardcoded `MAX_POINTS = 120`: Replace with time window calculation

## Open Questions

1. **Time window default on session start**
   - What we know: D-09 says "recent N minutes" is default
   - What's unclear: What is "N"? 2 minutes? 5 minutes?
   - Recommendation: Start with 5 minutes as default; make configurable in future

2. **Export date picker UI**
   - What we know: D-15 specifies presets (Today, This Week, All Time) + custom picker
   - What's unclear: Which date picker library? Native HTML or custom?
   - Recommendation: Use native `<input type="date">` for simplicity; already works across browsers

3. **Fullscreen preferences persistence**
   - What we know: D-06 says "stored in settings"
   - What's unclear: Via tauri-plugin-store or extend SQLite schema?
   - Recommendation: Use existing settings store pattern; consider tauri-plugin-store for consistency with alert settings

4. **Transition animations for theme changes**
   - What we know: Claude's discretion
   - What's unclear: Duration, easing function
   - Recommendation: 0.3s ease-out transition on CSS variables; use `transition: background-color 0.3s ease, color 0.3s ease`

## Sources

### Primary (HIGH confidence)
- [Tauri Dialog Plugin](https://v2.tauri.app/plugin/dialog/) - Save dialog API, filters, configuration
- [Tauri File System Plugin](https://v2.tauri.app/plugin/file-system/) - writeTextFile API
- [Chart.js Time Scale](https://www.chartjs.org/docs/latest/axes/cartesian/time.html) - Time axis configuration, min/max bounds
- [chartjs-adapter-date-fns](https://github.com/chartjs/chartjs-adapter-date-fns) - Installation and usage

### Secondary (MEDIUM confidence)
- Codebase analysis: `src/app.css`, `HeartRateChart.svelte`, `FullscreenMode.svelte`, `settings.ts`, `history.ts`, `database.rs` - Existing patterns and structure

### Tertiary (LOW confidence)
- Health app color palette recommendations - Standard teal/green patterns from Tailwind CSS color system; not from official health app documentation

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Tauri and Chart.js plugins verified from official docs; versions confirmed via npm registry
- Architecture: HIGH - Patterns match existing codebase conventions; reusable components identified
- Pitfalls: HIGH - Based on Chart.js documentation and common Tauri plugin patterns
- Color palette: MEDIUM - Standard Tailwind teal colors; not verified against specific health app design systems

**Research date:** 2026-03-21
**Valid until:** 30 days (stable APIs for Tauri 2.x and Chart.js 4.x)