---
phase: 03-advanced-analytics-exercise-tracking
plan: 02
subsystem: backend
tags: [hrv, exercise-statistics, analytics]
requires: [03-01]
provides: [hrv-estimation, exercise-statistics, exercise-type-comparison]
affects: [database, commands, main]
tech_stack:
  added:
    - HRV variance estimation from BPM data
    - Exercise vs resting heart rate comparison
    - Heart rate zone calculation
    - Exercise type aggregation statistics
  patterns:
    - Variance-based HRV approximation
    - SQLite aggregation with JOINs
    - Confidence scoring based on data quantity
key_files:
  created: []
  modified:
    - path: src-tauri/src/storage/database.rs
      changes: Added HRVResult, HeartRateZones, ExerciseStats, ExerciseTypeStats structs and calculate_hrv, get_exercise_statistics, get_exercise_type_statistics methods
    - path: src-tauri/src/commands/handlers.rs
      changes: Added get_hrv_estimate, get_exercise_statistics, get_exercise_type_statistics commands
    - path: src-tauri/src/main.rs
      changes: Registered new commands in invoke_handler
decisions:
  - id: D-09
    summary: BPM variance estimation for HRV instead of RR-intervals
  - id: D-10
    summary: RMSSD-style single score from variance
  - id: D-11
    summary: is_estimated always true for HRV results
  - id: D-14
    summary: Exercise vs resting HR comparison with zones
  - id: D-15
    summary: Personal resting baseline from non-exercise sessions
metrics:
  duration: 5m
  tasks_completed: 2
  files_modified: 3
  completed_date: 2026-03-21
---

# Phase 03 Plan 02: HRV Estimation and Exercise Statistics Summary

## One-liner

Added backend commands for HRV estimation from BPM variance and exercise statistics aggregation for exercise vs resting comparison and exercise type analysis.

## Changes

### Task 1: HRV Estimation from BPM Variance

Added HRVResult struct and calculate_hrv method using variance-based approximation:
- Variance calculation: `AVG(bpm*bpm) - AVG(bpm)^2`
- RMSSD approximation: `sqrt(variance) * 10` (scaling factor to ms range)
- Confidence levels: high (>100 points), medium (50-100), low (<50)
- is_estimated always true per D-11

### Task 2: Exercise Statistics Commands

Added exercise statistics aggregation:
- ExerciseStats: avg_exercise_hr, avg_resting_hr, exercise_sessions, total_exercise_minutes, hr_zones
- HeartRateZones: zone1-5 percentages based on max HR (180 for age 40 default)
- ExerciseTypeStats: per-type aggregation with session_count, avg_hr, max_hr, min_hr, total_minutes

## Deviations from Plan

### Implementation Completed Ahead of Schedule

**Found during:** Plan execution
**Issue:** All planned functionality for Plan 03-02 was already implemented in Plan 03-01 commits (2e250b8, 64c9f2c)
**Resolution:** Verified commits contain required structs, methods, and commands. No additional work needed.
**Commits:**
- 2e250b8: feat(03-01): add exercise tagging storage and detection structs
- 64c9f2c: feat(03-01): add exercise detection commands and Tauri handlers

## Verification

- HRVResult struct with hrv_value, is_estimated=true, confidence, data_points fields
- calculate_hrv method uses variance formula per D-09
- get_hrv_estimate command exists and registered in main.rs
- ExerciseStats, HeartRateZones, ExerciseTypeStats structs defined
- get_exercise_statistics and get_exercise_type_statistics methods exist
- Commands registered in main.rs

## Key Interfaces

```rust
// HRV estimation
pub struct HRVResult {
    pub hrv_value: f64,        // Estimated RMSSD-style value (ms)
    pub is_estimated: bool,    // Always true per D-11
    pub confidence: String,    // "low", "medium", "high"
    pub data_points: i64,
    pub period_start: i64,
    pub period_end: i64,
}

// Exercise statistics
pub struct ExerciseStats {
    pub avg_exercise_hr: f64,
    pub avg_resting_hr: f64,
    pub exercise_sessions: i64,
    pub total_exercise_minutes: f64,
    pub hr_zones: HeartRateZones,
}

// Exercise type comparison
pub struct ExerciseTypeStats {
    pub exercise_type: String,
    pub session_count: i64,
    pub avg_hr: f64,
    pub max_hr: u16,
    pub min_hr: u16,
    pub total_minutes: f64,
}
```

## Requirements Satisfied

- STAT-05: Exercise vs resting heart rate analysis
- STAT-06: HRV estimation with confidence indicator
- STAT-08: Exercise type comparison statistics

## Self-Check: PASSED

- [x] HRVResult struct exists in database.rs
- [x] calculate_hrv method exists in database.rs
- [x] get_hrv_estimate command exists in handlers.rs
- [x] ExerciseStats struct exists in database.rs
- [x] HeartRateZones struct exists in database.rs
- [x] ExerciseTypeStats struct exists in database.rs
- [x] get_exercise_statistics method exists in database.rs
- [x] get_exercise_type_statistics method exists in database.rs
- [x] Commands registered in main.rs
- [x] Commits 2e250b8 and 64c9f2c contain all changes