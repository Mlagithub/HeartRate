# Domain Pitfalls

**Domain:** Heart rate analytics and visualization for desktop health monitoring
**Researched:** 2026-03-21
**Mode:** Ecosystem (pitfalls dimension)

## Critical Pitfalls

Mistakes that cause rewrites or major issues.

### Pitfall 1: HRV Estimation Without RR-Interval Data

**What goes wrong:** Attempting to calculate meaningful HRV from aggregated BPM values produces misleading results that users may rely on for health decisions.

**Why it happens:** Developers assume heart rate (BPM) and heart rate variability (HRV) are interchangeable metrics. They are fundamentally different: HRV requires beat-to-beat interval data (milliseconds between consecutive heartbeats), not averaged BPM values.

**Consequences:**
- HRV estimates will have very low correlation with actual HRV measures
- Users get false confidence in health metrics
- App may be called out for misleading health claims
- Cannot validate accuracy without RR-interval comparison

**Prevention:**
- Clearly label as "estimated" or "derived metric" not true HRV
- Use alternative naming: "Heart Rate Stability" or "Rhythm Consistency"
- Document limitations prominently in UI
- Consider RMSSD approximation only if you have per-beat timestamps

**Detection:**
- Your data source only provides BPM values (no RR-intervals)
- You are calculating variance of averaged values rather than intervals

**Phase to address:** Data Analysis phase (when implementing HRV feature)

**Sources:**
- [PMC: Guidelines for HRV Measurement](https://pmc.ncbi.nlm.nih.gov/articles/PMC5624990/) - "HRV is fundamentally based on changes in the time intervals between consecutive heartbeats called interbeat intervals"
- [Whoop HRV Guide](https://www.whoop.com/us/en/thelocker/heart-rate-variability-hrv/) - HRV requires RMSSD calculation from beat-to-beat data

---

### Pitfall 2: Aggregating Heart Rate Data Without Time-Weighting

**What goes wrong:** Daily/weekly average heart rate calculations become misleading when data is sparse or unevenly distributed.

**Why it happens:** Developers calculate simple averages of all BPM values without considering that heart rate varies dramatically by activity level. A user who exercises for 30 minutes (high HR) and sits for 8 hours (low HR) will get a skewed "average" if the exercise data dominates due to higher sampling frequency during activity.

**Consequences:**
- Users see confusing statistics that don't match their experience
- Exercise vs. resting analysis becomes unreliable
- Trend detection fails when sampling patterns change

**Prevention:**
- Use time-weighted averages (weight by duration between measurements)
- Separate aggregation by activity context (resting vs. active)
- Require minimum data density before showing statistics
- Show data coverage alongside statistics (e.g., "based on 4 hours of data")

**Detection:**
- Statistics page shows only record count, not time coverage
- Aggregation uses simple `AVG(bpm)` queries
- No handling for gaps in data collection

**Phase to address:** Data Analysis phase (statistics implementation)

---

### Pitfall 3: Exercise Detection Without Personalized Baselines

**What goes wrong:** Static heart rate thresholds for exercise detection produce high false positive/negative rates across different users.

**Why it happens:** Developers use fixed thresholds (e.g., "above 100 BPM = exercise") without accounting for:
- Resting heart rate variation (40-100 BPM is normal range)
- Fitness level differences (athletes have lower resting HR)
- Age-based maximum heart rate differences
- Individual cardiac conditions

**Consequences:**
- Sedentary users with naturally high HR see false "exercise" flags
- Fit users with low resting HR miss light exercise detection
- Users lose trust in auto-detection and disable the feature
- Manual tagging becomes the only reliable option

**Prevention:**
- Establish user-specific resting baseline first (requires 3-7 days of data)
- Use percentage above baseline rather than absolute thresholds
- Implement grace period for baseline establishment
- Provide manual calibration option
- Consider heart rate change rate (sudden spike vs. gradual increase)

**Detection:**
- Exercise detection uses hardcoded threshold values
- No personalization or learning phase
- User's resting HR is not calculated/stored

**Phase to address:** Data Analysis phase (exercise auto-detection feature)

---

### Pitfall 4: Chart Performance Degradation With Long Sessions

**What goes wrong:** Real-time chart becomes unresponsive or choppy as session duration increases.

**Why it happens:** Chart.js renders all data points even when most are off-screen. Each measurement triggers a full re-render of the chart dataset.

**Current code issue:** `HeartRateChart.svelte` subscribes to the entire store and updates on every measurement with only 250ms throttle. The chart creates new arrays on each update (`history.slice(-MAX_POINTS)`).

**Consequences:**
- UI freezes during long recording sessions
- Battery drain on laptops
- User frustration with laggy interface
- Full-screen mode becomes unusable

**Prevention:**
- Implement proper decimation (downsampling) before passing data to chart
- Use Chart.js decimation plugin or manual data reduction
- Set `parsing: false` and provide pre-parsed data
- Disable animations for streaming data (`animation: false`)
- Consider `spanGaps: true` for sparse data
- For history charts, render only visible time range

**Detection:**
- Chart re-renders full dataset on each update
- No decimation or downsampling applied
- Animation duration > 0 for streaming data
- User reports UI lag during long sessions

**Phase to address:** UI Enhancement phase (chart optimization)

**Sources:**
- [Chart.js Performance Documentation](https://www.chartjs.org/docs/latest/general/performance.html) - "For large datasets, decimate data before rendering"

---

## Moderate Pitfalls

### Pitfall 5: Missing Artifact/Outlier Handling

**What goes wrong:** Transient spikes (sensor artifacts, motion, connection issues) corrupt statistics and chart display.

**Why it happens:** BLE heart rate monitors occasionally transmit erroneous values (e.g., 255 BPM spike from sensor disconnect/reconnect). These are not filtered out before processing.

**Consequences:**
- Session max shows unrealistic values (e.g., "Max: 255 BPM")
- Chart shows dramatic spikes that compress normal data
- Statistics become meaningless
- Auto-detection triggers falsely on artifacts

**Prevention:**
- Implement rate-of-change validation (HR cannot jump 50+ BPM in 1 second physiologically)
- Filter values outside physiological range (30-220 BPM for adults)
- Apply median filter or moving average for display
- Flag artifacts in database for exclusion from statistics
- Show filtered vs. raw data option for debugging

**Detection:**
- No input validation on incoming BPM values
- Statistics show unrealistic max/min values
- Chart has visible spike artifacts
- No outlier detection code in data pipeline

**Phase to address:** Data Analysis phase (statistics implementation)

---

### Pitfall 6: Fullscreen Mode Platform Inconsistencies

**What goes wrong:** Fullscreen mode behaves differently across Windows, macOS, and Linux, creating user confusion and potential support burden.

**Why it happens:** Each platform handles fullscreen differently:
- macOS: Uses native fullscreen animation, creates new "Space"
- Windows: Standard fullscreen, taskbar may overlap
- Linux: Behavior varies by window manager (GNOME, KDE, etc.)

**Current code issue:** `FullscreenMode.svelte` uses only `setFullscreen(true/false)` without handling platform differences or the `setSimpleFullscreen` option for macOS.

**Consequences:**
- Escape key behavior differs by platform
- Exit fullscreen requires different actions on different platforms
- macOS users may get stuck in fullscreen Spaces
- Touch/gesture interactions differ

**Prevention:**
- Use `setSimpleFullscreen` for macOS (less intrusive than native fullscreen)
- Test all three platforms during development
- Provide consistent exit mechanism (ESC key + click to exit)
- Document platform-specific behavior
- Consider kiosk mode for true locked fullscreen

**Detection:**
- Fullscreen only tested on one platform
- No macOS-specific handling
- Escape key handler conflicts with platform behavior

**Phase to address:** UI Enhancement phase (fullscreen refinement)

**Sources:**
- [Tauri Window API](https://tauri.app/reference/javascript/api/namespacewindow/) - setFullscreen and setSimpleFullscreen methods

---

### Pitfall 7: Statistics Lost on Page Refresh

**What goes wrong:** In-memory session statistics are lost when user refreshes the page or navigates away.

**Current code issue:** `heartRate.ts` stores `sessionStats` and `totalBpm` as module-level variables. These reset to initial values on any page reload.

**Consequences:**
- User loses current session min/max/avg on refresh
- Cannot rely on app for continuous monitoring
- Data exists in database but UI shows "new session"

**Prevention:**
- Persist session ID and current stats to localStorage
- On app load, check for existing session in localStorage
- Allow session resumption within time window (e.g., 5 minutes)
- Store cumulative stats in database, not just individual measurements

**Detection:**
- Session stats stored only in JavaScript memory
- No localStorage persistence for session state
- Page refresh resets statistics display

**Phase to address:** UI Enhancement phase (session persistence)

---

### Pitfall 8: Y-Axis Auto-Scaling Hides Trend Information

**What goes wrong:** Dynamic Y-axis scaling makes it impossible to visually compare heart rate across time periods.

**Why it happens:** Current `calculateYAxisRange()` in `HeartRateChart.svelte` adjusts Y-axis to fit current data range with 15% padding. This means a chart showing 70-80 BPM looks identical to one showing 120-130 BPM (both fill the vertical space).

**Consequences:**
- Users cannot visually distinguish resting from elevated heart rate
- Small fluctuations are exaggerated misleadingly
- Long-term trend comparison is impossible
- Full-screen mode with auto-scaling defeats the purpose

**Prevention:**
- Offer both fixed and dynamic Y-axis modes
- For health monitoring, prefer fixed physiological range (40-180 BPM)
- Show reference lines for resting HR and zones
- In comparison views, use consistent Y-axis scale
- Document the trade-off clearly in UI

**Detection:**
- Chart always uses data-driven Y-axis range
- No fixed scale option
- Small HR changes appear as large chart movements

**Phase to address:** UI Enhancement phase (chart configuration)

---

## Minor Pitfalls

### Pitfall 9: Timezone Handling in Data Export

**What goes wrong:** CSV/JSON exports use local timestamps without timezone information, causing confusion when data is analyzed across timezones.

**Prevention:**
- Store timestamps as UTC in database
- Export with ISO 8601 format including timezone
- Allow user to choose export timezone

**Phase to address:** Data Analysis phase (export feature)

---

### Pitfall 10: Heart Rate Zone Thresholds Not Personalized

**What goes wrong:** Hardcoded zone boundaries (60, 100, 140, 170) don't account for individual physiology.

**Current code issue:** `HeartRateChart.svelte` and `FullscreenMode.svelte` use fixed zone thresholds.

**Prevention:**
- Calculate zones based on user's maximum HR (220-age formula or measured)
- Allow manual zone customization
- Show zone boundaries in settings

**Phase to address:** Data Analysis phase (zone configuration)

---

## Phase-Specific Warnings

| Phase Topic | Likely Pitfall | Mitigation |
|-------------|----------------|------------|
| HRV Feature Implementation | Pitfall #1: HRV without RR-intervals | Clear labeling, use alternative metric naming |
| Exercise Auto-Detection | Pitfall #3: Static thresholds | Require baseline period, use percentage-based detection |
| Statistics Implementation | Pitfall #2: Unweighted averages | Time-weight aggregation, show data coverage |
| Chart Optimization | Pitfall #4: Performance degradation | Implement decimation, disable animations |
| Fullscreen Enhancement | Pitfall #6: Platform inconsistencies | Test all platforms, use setSimpleFullscreen for macOS |
| Data Export | Pitfall #9: Timezone issues | Use ISO 8601, store UTC |

---

## Key Takeaways for Roadmap

1. **HRV estimation is fundamentally limited** by the data source. If RR-intervals are not available from BLE, label the feature carefully and consider alternative metrics.

2. **Statistics require defensive design**: time-weighting, outlier filtering, minimum data requirements, and clear labeling of limitations.

3. **Chart performance must be designed in from the start** - retroactive optimization is painful. Decimation and animation disabling are essential for real-time streaming.

4. **Personalization is not optional** for health metrics. Static thresholds will cause user complaints and feature abandonment.

5. **Cross-platform testing** for fullscreen and window management is essential - do not assume macOS behavior on Windows/Linux.

---

## Sources

| Source | Confidence | Relevance |
|--------|------------|-----------|
| [PMC: Guidelines for HRV Measurement](https://pmc.ncbi.nlm.nih.gov/articles/PMC5624990/) | HIGH | HRV accuracy, sampling requirements, artifact impact |
| [Chart.js Performance Documentation](https://www.chartjs.org/docs/latest/general/performance.html) | HIGH | Chart optimization strategies |
| [Whoop HRV Guide](https://www.whoop.com/us/en/thelocker/heart-rate-variability-hrv/) | MEDIUM | Practical HRV measurement approach |
| [Tauri Window API](https://tauri.app/reference/javascript/api/namespacewindow/) | HIGH | Fullscreen implementation details |
| Codebase analysis (CONCERNS.md, store files) | HIGH | Current implementation vulnerabilities |

---

*Research completed: 2026-03-21*