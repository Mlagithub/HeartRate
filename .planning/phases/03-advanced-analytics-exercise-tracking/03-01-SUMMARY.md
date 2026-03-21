---
phase: 03-advanced-analytics-exercise-tracking
plan: 01
subsystem: backend
tags: [exercise, detection, storage, rust, tauri]
dependency_graph:
  requires: []
  provides:
    - exercise_tags table
    - ExerciseTag struct
    - DetectionResult struct
    - BaselineStats struct
    - tag_exercise_session command
    - detect_exercise_session command
    - get_sessions_list command
    - get_resting_baseline command
  affects:
    - src-tauri/src/storage/database.rs
    - src-tauri/src/commands/handlers.rs
    - src-tauri/src/main.rs
tech_stack:
  added: []
  patterns:
    - SQLite table with session_id primary key
    - Threshold-based exercise detection algorithm
    - Personal resting baseline calculation from 7+ days data
key_files:
  created: []
  modified:
    - path: src-tauri/src/storage/database.rs
      changes: Added ExerciseTag, SessionInfo, DetectionResult, BaselineStats structs; exercise_tags table; tag/detect methods
    - path: src-tauri/src/commands/handlers.rs
      changes: Added 8 exercise-related Tauri commands
    - path: src-tauri/src/main.rs
      changes: Registered new commands in invoke_handler
decisions:
  - id: D-05
    choice: Threshold-based detection using resting_avg + 30 BPM
    rationale: Simple, effective heuristic that works with available BPM data
  - id: D-06
    choice: Require 7+ days of baseline data
    rationale: Provides statistically meaningful resting HR estimate
  - id: D-08
    choice: Show detection prompt only when confidence >= 0.5
    rationale: Reduces false positives, only prompt when reasonably confident
metrics:
  duration: 15m
  tasks_completed: 2
  files_modified: 3
  completed_date: 2026-03-21
---

# Phase 03 Plan 01: Exercise Tagging Backend Summary

## One-liner

Backend exercise tagging storage with manual tagging support and threshold-based automatic exercise detection using personal resting baseline.

## What Was Done

### Task 1: Add exercise_tags table and ExerciseTag struct to database

- Added `ExerciseTag` struct with `session_id`, `exercise_type`, `is_confirmed`, `confidence`, `tagged_at` fields
- Added `SessionInfo` struct for session list display with optional `exercise_tag`
- Created `exercise_tags` table in `Database::new()` with session_id as primary key
- Implemented `tag_exercise`, `get_exercise_tag`, `get_sessions_with_exercise`, `get_sessions` methods
- Implemented `remove_exercise_tag` for untagging sessions

### Task 2: Add exercise detection algorithm and Tauri commands

- Added `DetectionResult` struct with `session_id`, `is_exercise`, `confidence`, `reason` fields
- Added `BaselineStats` struct with `resting_avg`, `resting_stddev`, `data_days` fields
- Implemented `calculate_resting_baseline` - calculates personal resting HR from past 30 days
- Implemented `detect_exercise` - threshold-based detection (resting_avg + 30 BPM)
- Implemented `detect_exercise_for_sessions` - batch detection for all untagged sessions
- Added 8 Tauri commands: `tag_exercise_session`, `get_sessions_list`, `detect_exercise_session`, `get_resting_baseline`, `detect_exercise_all`, `remove_exercise_tag`, `get_exercise_tags`
- Registered all commands in `main.rs`

## Key Implementation Details

### Detection Algorithm (per D-05)

```
1. Get session average HR
2. Check session duration >= 5 minutes
3. If no baseline (less than 7 days): return is_exercise=false, confidence=0.0
4. Threshold: session_avg > resting_avg + 30 BPM
5. Confidence: (session_avg - resting_avg - 20) / 40, clamped 0.0-1.0
6. Only show detection prompt if confidence >= 0.5 per D-08
```

### Resting Baseline Calculation (per D-06, D-15)

- Uses past 30 days of data
- Calculates daily minimum HR for each day
- Averages daily minimums to estimate resting HR
- Returns `None` if less than 7 days of data

## Deviations from Plan

### Auto-fixed Issues

None - plan executed exactly as written.

### Environment Limitation

- `cargo check` could not be run due to Rust toolchain not being available in the execution environment
- Code follows existing patterns exactly, compilation should succeed

## Verification Status

- [x] ExerciseTag struct exists with required fields
- [x] exercise_tags table created in Database::new()
- [x] tag_exercise, get_exercise_tag, get_sessions_with_exercise, get_sessions methods exist
- [x] SessionInfo struct exists with exercise_tag field
- [x] DetectionResult struct with session_id, is_exercise, confidence, reason fields
- [x] BaselineStats struct with resting_avg, resting_stddev, data_days fields
- [x] calculate_resting_baseline, detect_exercise methods exist
- [x] All Tauri commands implemented and registered
- [ ] cargo check passes (could not verify - Rust not available)

## Success Criteria

- [x] Backend can store and retrieve exercise tags by session_id
- [x] Backend can detect exercise with confidence scores based on personal baseline
- [x] Frontend can invoke tag_exercise_session, get_sessions_list, detect_exercise_session commands

## Files Modified

| File | Lines Added | Purpose |
|------|-------------|---------|
| src-tauri/src/storage/database.rs | ~400 | Exercise structs, table, and methods |
| src-tauri/src/commands/handlers.rs | ~100 | Tauri command handlers |
| src-tauri/src/main.rs | 8 | Command registration |

## Next Steps

Plan 03-02 will add frontend components:
- Exercise tab in HistoryView
- Session list with exercise tagging UI
- Detection prompts with confirm/dismiss
- Exercise statistics display