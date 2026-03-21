# Phase 1: UI Enhancement & Data Export - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-21
**Phase:** 01-ui-enhancement-data-export
**Areas discussed:** Visual Design Style, Fullscreen Configuration, Chart X-axis Toggle, Export Flow & Location

---

## Visual Design Style

| Option | Description | Selected |
|--------|-------------|----------|
| Shift to teal/green palette | Health/fitness apps often use teal/green. More calming and health-aligned. | ✓ |
| Keep blue, refine minimalism | Current blue is fine. Just refine spacing and typography for cleaner feel. | |
| Blue primary + teal accent | Keep blue but add a subtle teal accent for health elements. | |

**User's choice:** Shift to teal/green palette (Recommended)
**Notes:** Dark default, more spacious layout, keep Inter font.

---

## Fullscreen Configuration Approach

| Option | Description | Selected |
|--------|-------------|----------|
| Checkbox toggles | Replace preset modes with individual checkboxes: Show BPM, Show Chart, Show Stats. More flexible. | ✓ |
| Keep 4 preset modes | Keep the 4 preset modes (simple/standard/stats/chart) as they are. Just add to Settings. | |
| Presets + custom option | Combine both: presets + advanced custom toggle for power users. | |

**User's choice:** Checkbox toggles (Recommended)

---

## Fullscreen Config Location

| Option | Description | Selected |
|--------|-------------|----------|
| Inside fullscreen | Fullscreen is session-specific. No settings clutter. But users can't preset preferences. | |
| Settings tab | Persistent preferences. Users set it once and it applies every time. | ✓ |
| Both | Quick access. Settings icon in fullscreen opens a mini-panel. | |

**User's choice:** Settings tab (Recommended)

---

## BPM Display in Fullscreen

| Option | Description | Selected |
|--------|-------------|----------|
| Always show BPM | Fullscreen always shows the BPM value. Stats and chart are optional extras. | ✓ |
| BPM also toggleable | Maximum flexibility. Could hide everything for a blank screen. | |

**User's choice:** Always show BPM (Recommended)

---

## Auto-scale Y-axis

| Option | Description | Selected |
|--------|-------------|----------|
| Auto-scale to session data | Dynamic Y-axis range based on session data. Amplifies small changes. | ✓ |
| Fixed Y-axis range | Keep fixed range (e.g., 40-200 BPM). User sees absolute position in zones. | |
| User choice in settings | Let user pick: auto-scale or fixed in settings. | |

**User's choice:** Auto-scale to session data (Recommended)

---

## Chart Default View

| Option | Description | Selected |
|--------|-------------|----------|
| Recent N minutes | Focus on recent activity. Most users want to see current trends. | ✓ |
| All session data | See the whole session at once. User can zoom in if needed. | |

**User's choice:** Recent N minutes (Recommended)

---

## Time Window Options

| Option | Description | Selected |
|--------|-------------|----------|
| Presets: 2, 5, 10, 30 min | Common preset windows. Simple selection, covers most use cases. | ✓ |
| Custom input field | User types any value. More flexible but more complex UI. | |
| Presets + custom option | Maximum flexibility but more UI complexity. | |

**User's choice:** Presets: 2, 5, 10, 30 min (Recommended)

---

## Time Window Control Style

| Option | Description | Selected |
|--------|-------------|----------|
| Inline pill buttons | Clean inline UI. Buttons like [2m] [5m] [10m] [All]. No dropdown. | ✓ |
| Dropdown/select | More compact. Opens on click to show options. | |
| Slider control | Slider from 1-60 minutes. Continuous control but harder for exact values. | |

**User's choice:** Inline pill buttons (Recommended)

---

## Chart Toggle Placement

| Option | Description | Selected |
|--------|-------------|----------|
| Main view + fullscreen | Both views show the same heart rate data. Toggle available in both. | ✓ |
| Main view only | Less UI clutter in fullscreen. Only main view has toggle. | |
| Fullscreen only | Fullscreen is for focused monitoring. Toggle only there. | |

**User's choice:** Main view + fullscreen (Recommended)

---

## Export Location

| Option | Description | Selected |
|--------|-------------|----------|
| History tab | History page is where data lives. Export logically belongs there. | ✓ |
| New Export tab | Add a dedicated Export tab. Clear separation from history viewing. | |
| Settings tab | Always accessible. But adds clutter to main monitoring view. | |

**User's choice:** History tab (Recommended)

---

## Export UI Style

| Option | Description | Selected |
|--------|-------------|----------|
| Modal dialog | Button opens a focused panel with format + date range + export. Clear and guided. | ✓ |
| Inline form | Controls embedded in page. No popup, but takes more space. | |
| Split: Quick + options | Quick export with last settings, or open dialog for options. | |

**User's choice:** Modal dialog (Recommended)

---

## Date Range Selection

| Option | Description | Selected |
|--------|-------------|----------|
| Presets + date picker | One-click access to common ranges. Date picker for specific periods. | ✓ |
| Date picker only | Maximum flexibility. User always picks exact dates. | |
| Presets only | Simple preset buttons. No date picker complexity. | |

**User's choice:** Presets (Today, This Week, All Time) + date picker (Recommended)

---

## Export Format Default

| Option | Description | Selected |
|--------|-------------|----------|
| CSV | CSV is more universally useful (Excel, spreadsheets). Better default for most users. | ✓ |
| JSON | JSON is better for developers/programmatic use. Niche default. | |
| Remember last selection | Remember user's last choice. Persistent preference. | |

**User's choice:** CSV (Recommended)

---

## Claude's Discretion

- Exact teal/green hex values for color palette
- Specific padding/spacing values for spacious layout
- Export modal visual design
- Transition animations for theme changes

## Deferred Ideas

None — discussion stayed within phase scope.