# Roadmap: Heart Rate Receiver v2.0

**Created:** 2026-03-21
**Granularity:** Coarse (3-5 phases)
**Core Value:** Clear, actionable heart rate insights at a glance

---

## Overview

Enhancement project for existing Tauri-based heart rate monitoring application. Building on working foundation (BLE, visualization, storage, alerts) to add UI refinement and comprehensive data analysis capabilities.

**Total v1 Requirements:** 23
**Coverage:** 23/23 mapped (100%)

---

## Phases

- [ ] **Phase 1: UI Enhancement & Data Export** - Modernize visual design and enable data portability
- [ ] **Phase 2: Core Statistics & Analytics** - Deliver time-dimension heart rate insights
- [ ] **Phase 3: Advanced Analytics & Exercise Tracking** - Provide exercise detection and HRV insights

---

## Phase Details

### Phase 1: UI Enhancement & Data Export

**Goal:** Users see their heart rate data in a clean, health-focused interface and can export it for external analysis.

**Depends on:** Nothing (foundation phase)

**Requirements:** UI-01, UI-02, UI-03, UI-04, EXP-01, EXP-02, EXP-03, EXP-04

**Success Criteria** (what must be TRUE):
1. User sees a minimalist health-focused interface with clean typography and calming colors when viewing heart rate data
2. User can toggle chart X-axis between recent N minutes (configurable) and all session data directly in the main view
3. User can configure which elements appear in fullscreen mode (heart rate value, curve, session stats, status info) via settings
4. User can export heart rate data to CSV or JSON file using native file save dialog
5. User can select date range for exports (all time or specific period)

**Plans:** 4 plans

Plans:
- [ ] 01-01-PLAN.md - UI Theme & Layout (UI-01, UI-02): Teal/green color palette, increased spacing, dark theme default
- [ ] 01-02-PLAN.md - Fullscreen Preferences (UI-03): Checkbox toggles for chart/stats visibility in fullscreen mode
- [ ] 01-03-PLAN.md - Chart Time Window Toggle (UI-04): Time window pills [2m] [5m] [10m] [30m] [All] for chart X-axis
- [ ] 01-04-PLAN.md - Data Export Feature (EXP-01, EXP-02, EXP-03, EXP-04): Export modal, CSV/JSON format, date range selection, native dialogs

---

### Phase 2: Core Statistics & Analytics

**Goal:** Users can analyze their heart rate patterns across different time dimensions with aggregated statistics.

**Depends on:** Phase 1 (UI foundation in place)

**Requirements:** STAT-01, STAT-02, STAT-03, STAT-04, STAT-07, PAGE-01, PAGE-02, PAGE-04

**Success Criteria** (what must be TRUE):
1. User can view daily heart rate statistics showing min, max, avg BPM with record count
2. User can view weekly, monthly, and yearly aggregated statistics with trend indicators
3. User can navigate between History and Statistics tabs in the unified analytics page
4. User can switch time dimension (daily/weekly/monthly) within the statistics tab
5. User can see trend analysis charts with moving averages over selected periods

**Plans:** TBD

---

### Phase 3: Advanced Analytics & Exercise Tracking

**Goal:** Users gain insights into exercise patterns and heart rate variability without manual data entry overhead.

**Depends on:** Phase 2 (statistics foundation in place)

**Requirements:** STAT-05, STAT-06, STAT-08, EX-01, EX-02, EX-03, PAGE-03

**Success Criteria** (what must be TRUE):
1. User can manually tag any session as exercise with optional activity type
2. User sees automatic exercise detection with confidence indicator and can confirm or dismiss
3. User can view exercise vs resting heart rate analysis with comparison charts
4. User can see HRV estimation with clear labeling that indicates it is estimated from BPM (not RR-intervals)
5. User can compare heart rate patterns across different exercise types

**Plans:** TBD

---

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. UI Enhancement & Data Export | 0/4 | Planned | - |
| 2. Core Statistics & Analytics | 0/3 | Not started | - |
| 3. Advanced Analytics & Exercise Tracking | 0/3 | Not started | - |

---

## Dependency Graph

```
Phase 1 (UI/Export)
    |
    v
Phase 2 (Statistics)
    |
    v
Phase 3 (Advanced/Exercise)
```

**Phase 1 Plan Dependencies:**
```
        01-01 (UI Theme & Layout)
              |
       +------+------+
       |             |
       v             v
   01-02         01-03
(Fullscreen)  (Time Window)
       |             |
       +------+------+
              |
              v
          01-04 (Data Export)
```

---

## Research Flags

Phases requiring additional research during planning:

**Phase 3:** HRV implementation decision point
- Option A: Store RR-intervals for accurate HRV (more storage, better accuracy)
- Option B: Estimate from BPM variance (current plan, must label clearly)
- Research recommendation: Store RR-intervals since BLE already parses them

**Phase 3:** Exercise detection algorithm
- Threshold-based vs. ML-based detection needs design during planning
- Requires personalized baseline (3-7 days of data)

---

*Roadmap created: 2026-03-21*
*Phase 1 planned: 2026-03-21*