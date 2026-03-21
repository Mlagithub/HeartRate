# Phase 3: Advanced Analytics & Exercise Tracking - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-21
**Phase:** 03-advanced-analytics-exercise-tracking
**Areas discussed:** Manual Exercise Tagging, Automatic Exercise Detection, HRV Estimation Display, Exercise Analysis Views

---

## Manual Exercise Tagging

| Option | Description | Selected |
|--------|-------------|----------|
| From History records | Click session in History to expand and tag. Natural fit. | ✓ |
| From Settings tab | Dedicated section. More formal but less discoverable. | |

| Option | Description | Selected |
|--------|-------------|----------|
| 5 preset types | Running, Cycling, Swimming, Gym, Other. Simple, covers most cases. | ✓ |
| Free-form text | User types custom names. Flexible but less structured. | |

| Option | Description | Selected |
|--------|-------------|----------|
| By session_id | Tag all records with same session_id. One tag per BLE session. | ✓ |
| By time range | Select specific range within session. More granular but complex. | |

**User's choice:** Tag from History records, 5 preset types, by session_id
**Notes:** Simple, discoverable approach that fits existing History UI

---

## Automatic Exercise Detection

| Option | Description | Selected |
|--------|-------------|----------|
| Threshold-based | Simple rules: sustained elevated HR above baseline. Easy to tune. | ✓ |
| ML-based | Sophisticated pattern recognition. Requires training data. | |

| Option | Description | Selected |
|--------|-------------|----------|
| 7 days baseline | Require 7+ days of data before activation. More accurate. | ✓ |
| Immediate detection | Start immediately with default thresholds. Faster but less personalized. | |

| Option | Description | Selected |
|--------|-------------|----------|
| Inline prompt in History | "Was this exercise?" prompt in records. Minimal UI. | ✓ |
| Notifications panel | Separate panel for pending detections. More prominent. | |

**User's choice:** Threshold-based, 7 days baseline, inline prompt
**Notes:** Pragmatic approach that prioritizes user control and simplicity

---

## HRV Estimation Display

| Option | Description | Selected |
|--------|-------------|----------|
| BPM variance estimation | Calculate from existing data. Simpler, no schema change. | ✓ |
| RR-interval based | More accurate but requires BLE verification and schema change. | |

| Option | Description | Selected |
|--------|-------------|----------|
| RMSSD-style score | Single HRV number. Standard format. | ✓ |
| Multiple HRV metrics | SDNN, pNN50, etc. More comprehensive but complex. | |

| Option | Description | Selected |
|--------|-------------|----------|
| Badge/label next to value | "(estimated)" badge. Clear, always visible. | ✓ |
| Tooltip disclaimer | Show in info panel. Cleaner but less obvious. | |

**User's choice:** BPM variance, RMSSD-style score, badge/label
**Notes:** Aligns with ROADMAP decision to use BPM estimation with clear labeling

---

## Exercise Analysis Views

| Option | Description | Selected |
|--------|-------------|----------|
| Third tab in analytics | History/Statistics/Exercise tabs. Consistent navigation. | ✓ |
| Within Statistics tab | Exercise comparison as filter. Less navigation. | |

| Metric | Selected |
|--------|----------|
| Avg exercise HR vs resting HR | ✓ |
| Time in HR zones | ✓ |
| Trend by exercise type | ✓ |

| Option | Description | Selected |
|--------|-------------|----------|
| vs Personal resting baseline | Compare against user's own resting average. Personalized. | ✓ |
| vs Population averages | Compare against general averages. Broader context. | |

**User's choice:** Third Exercise tab, all three metrics, personal baseline
**Notes:** Comprehensive exercise analytics with personalized comparisons

---

## Claude's Discretion

- Exact threshold values for detection algorithm
- Confidence score display format
- HR zones calculation (percentage of max HR or fixed ranges)
- Exercise tab empty state design

## Deferred Ideas

None — discussion stayed within phase scope.