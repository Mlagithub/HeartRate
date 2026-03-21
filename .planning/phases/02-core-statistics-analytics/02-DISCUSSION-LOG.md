# Phase 2: Core Statistics & Analytics - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-21
**Phase:** 02-core-statistics-analytics
**Areas discussed:** Page Layout & Tabs, Time Dimension Nav, Trend Charts, Aggregation Logic

---

## Page Layout & Tabs

| Option | Description | Selected |
|--------|-------------|----------|
| Two tabs: History + Statistics | Single analytics page with tabbed interface. Statistics shows aggregations and charts. | ✓ |
| Three tabs: History + Statistics + Trends | More separation but more navigation between views. | |

| Option | Description | Selected |
|--------|-------------|----------|
| Cards on top, chart below | Summary cards at top (min/max/avg), trend chart below. Classic dashboard layout. | ✓ |
| Chart dominant, stats secondary | Trend chart prominent, summary stats as overlay or sidebar. | |

**User's choice:** Two tabs with cards on top, chart below
**Notes:** Consistent with health app patterns like Apple Health

---

## Time Dimension Navigation

| Option | Description | Selected |
|--------|-------------|----------|
| Pill buttons | [Daily] [Weekly] [Monthly] [Yearly] — consistent with Phase 1 time window pills. | ✓ |
| Dropdown selector | Takes less space but requires click to see options. | |

| Option | Description | Selected |
|--------|-------------|----------|
| Daily view | Most relevant for users checking recent patterns. | |
| Weekly view | Balanced view of patterns without daily noise. Good for trend overview. | ✓ |

**User's choice:** Pill buttons with Weekly view default
**Notes:** Matches established UI pattern from Phase 1

---

## Trend Charts

| Option | Description | Selected |
|--------|-------------|----------|
| Line chart | Best for showing trends over time. Smooth lines connect data points. | ✓ |
| Bar chart | Clear discrete values per period. Good for comparing absolute values. | |

| Metric | Selected |
|--------|----------|
| Min / Max / Avg BPM | ✓ |
| Moving average trend line | ✓ |
| Record count | ✓ |

| Option | Description | Selected |
|--------|-------------|----------|
| 7-day moving average | Smooths daily noise while showing week-to-week changes. | ✓ |
| 14-day moving average | More smoothing, slower to respond. | |

**User's choice:** Line chart with all metrics, 7-day MA
**Notes:** Comprehensive metrics display for health insights

---

## Aggregation Logic

| Option | Description | Selected |
|--------|-------------|----------|
| By calendar period | All records within day/week/month/year boundaries grouped. Natural for health tracking. | ✓ |
| By session first, then period | Group by BLE session, then aggregate sessions. More complex. | |

| Option | Description | Selected |
|--------|-------------|----------|
| Skip empty days | Charts skip days with no data. Cleaner visualization, no misleading zeros. | ✓ |
| Show gaps for empty days | Charts show gap or null marker. Shows data availability. | |

| Option | Description | Selected |
|--------|-------------|----------|
| vs Previous period | Each period compared to previous equivalent (week vs previous week). | ✓ |
| vs Overall average | Compare against user's overall average. | |

**User's choice:** Calendar period grouping, skip empty days, vs previous period comparison
**Notes:** Simplest implementation with clear progress indicators

---

## Claude's Discretion

- Exact card styling and spacing for statistics display
- Trend indicator visual design (arrow, percentage, color)
- Empty state for statistics when no data available
- Transition animations between time dimensions

## Deferred Ideas

None — discussion stayed within phase scope.