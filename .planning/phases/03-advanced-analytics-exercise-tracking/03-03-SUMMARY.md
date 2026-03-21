---
phase: 03-advanced-analytics-exercise-tracking
plan: 03
subsystem: frontend
tags: [exercise, store, ui, tagging, detection]
dependencies:
  requires: [03-01, 03-02]
  provides: [exercise-store, tagging-ui, detection-prompts]
  affects: [HistoryView]
tech-stack:
  added: []
  patterns: [svelte-store, tauri-invoke, activity-pills, detection-prompt]
key-files:
  created:
    - src/lib/stores/exercise.ts
  modified:
    - src/lib/stores/index.ts
    - src/lib/components/HistoryView.svelte
decisions:
  - ACTIVITY_TYPES constant with 5 preset types per D-02
  - Detection confidence threshold >= 0.5 for showing prompt per D-08
  - Sessions list replaces individual records in History view
metrics:
  duration: 8m
  completed: 2026-03-21
  tasks: 2
  files: 3
---

# Phase 03 Plan 03: Exercise Tagging Store and UI Summary

## One-liner

Created exercise store with tagging/detection methods and added expandable session rows with activity type selection and auto-detection prompts to the History view.

## Completed Tasks

| Task | Name | Commit | Files |
|------|------|--------|-------|
| 1 | Create exercise store with tagging and detection methods | dfe5db6 | exercise.ts, index.ts |
| 2 | Add exercise tagging UI to History records list | f65633b | HistoryView.svelte |

## Implementation Details

### Task 1: Exercise Store

Created `src/lib/stores/exercise.ts` following the existing store pattern:

- **Interfaces**: `ExerciseTag`, `SessionInfo`, `DetectionResult`, `ExerciseState`
- **Constants**: `ACTIVITY_TYPES` array with 5 preset types (Running, Cycling, Swimming, Gym, Other)
- **Methods**: `loadSessions()`, `tagExercise()`, `detectExercise()`, `runDetectionForAll()`
- **State**: sessions array, detections Map, isLoading, error

### Task 2: History View UI

Modified `HistoryView.svelte` to show sessions with exercise tagging:

- **Sessions list**: Replaced individual records with grouped sessions
- **Expandable rows**: Click session to expand/collapse tagging UI
- **Activity pills**: 5 type selector matching existing pill button styling
- **Detection prompt**: Shows "Was this exercise?" for auto-detected sessions with confidence >= 0.5
- **Exercise badge**: Displays on tagged sessions

## Deviations from Plan

None - plan executed exactly as written.

## Verification

- svelte-check passes with 0 errors
- Exercise store follows existing statistics.ts pattern
- Activity types match D-02 exactly
- Detection confidence threshold >= 0.5 per D-08
- Tagging UI matches existing pill button styling

## Known Stubs

- Backend commands (`get_sessions_list`, `tag_exercise_session`, `detect_exercise_session`) not yet implemented - called via invoke but will fail until Phase 03-01 backend is complete

## Self-Check: PASSED

- [x] exercise.ts exists with all interfaces and methods
- [x] Exported from stores/index.ts
- [x] HistoryView.svelte has sessions list with tagging UI
- [x] Commits dfe5db6 and f65633b exist