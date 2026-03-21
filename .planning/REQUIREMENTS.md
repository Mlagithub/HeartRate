# Requirements: Heart Rate Receiver v2.0

**Defined:** 2026-03-21
**Core Value:** Clear, actionable heart rate insights at a glance

## v1 Requirements

Requirements for Heart Rate Receiver v2.0 enhancement. Each maps to roadmap phases.

### UI Enhancement

- [x] **UI-01**: Minimalist health-focused visual design with clean typography, ample whitespace, and calming color palette
- [x] **UI-02**: Full-screen mode with auto-scaling Y-axis that amplifies small heart rate fluctuations (e.g., 72→75 BPM visible)
- [x] **UI-03**: Configurable full-screen UI elements (heart rate value, curve, session stats, status info) configurable in main settings
- [x] **UI-04**: Heart rate curve X-axis toggle: recent N minutes (customizable by user) vs all data — available in main view, fullscreen, and statistics page

### Statistics & Analytics

- [x] **STAT-01**: Daily statistics view showing min, max, avg heart rate with record count
- [x] **STAT-02**: Weekly statistics view with aggregated metrics and trends
- [x] **STAT-03**: Monthly statistics view with aggregated metrics and trends
- [x] **STAT-04**: Yearly statistics view with aggregated metrics and trends
- [x] **STAT-05**: Exercise vs resting heart rate analysis with comparison charts
- [x] **STAT-06**: HRV (Heart Rate Variability) estimation displayed with confidence indicator noting it's estimated from BPM
- [ ] **STAT-07**: Time dimension charts showing trends over selected period
- [x] **STAT-08**: Exercise type comparison showing heart rate patterns by activity

### Exercise Tracking

- [x] **EX-01**: Manual exercise tagging — user can mark session as exercise with optional type
- [x] **EX-02**: Automatic exercise detection based on heart rate patterns (sustained elevated HR, recovery patterns)
- [x] **EX-03**: Exercise detection confidence indicator with user confirmation prompt

### Data Export

- [x] **EXP-01**: Export heart rate data to CSV format with timestamp, BPM, session ID columns
- [x] **EXP-02**: Export heart rate data to JSON format
- [x] **EXP-03**: Date range selection for exports (all time, specific period)
- [x] **EXP-04**: Native file save dialog via Tauri

### Statistics Page

- [ ] **PAGE-01**: Transform history page into tabbed interface with "History" and "Statistics" tabs
- [x] **PAGE-02**: Time dimension selector (daily/weekly/monthly) in statistics tab
- [ ] **PAGE-03**: Exercise type comparison view in statistics tab
- [ ] **PAGE-04**: Trend analysis charts with moving averages

## v2 Requirements

Deferred to future release. Tracked but not in current roadmap.

### Notifications

- **NOTF-01**: Heart rate zone alerts with customizable thresholds
- **NOTF-02**: Resting heart rate change notifications
- **NOTF-03**: HRV trend alerts

### Advanced Features

- **ADV-01**: Personalized heart rate zones based on resting HR
- **ADV-02**: Fitness trend analysis over time
- **ADV-03**: Sleep heart rate analysis
- **ADV-04**: Recovery heart rate metrics

## Out of Scope

Explicitly excluded. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Mobile platform | Desktop-only for this version, requires major architecture change |
| Cloud sync/backup | Local-only storage, no server infrastructure |
| Multi-user/accounts | Single-user desktop app, no authentication needed |
| Real RR-interval HRV | Would require database schema change + BLE protocol verification; using BPM estimation instead |
| Real-time streaming to external services | Out of scope for privacy and complexity reasons |
| Social/sharing features | Not aligned with core value of personal insights |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| UI-01 | Phase 1 | Complete |
| UI-02 | Phase 1 | Complete |
| UI-03 | Phase 1 | Complete |
| UI-04 | Phase 1 | Complete |
| STAT-01 | Phase 2 | Complete |
| STAT-02 | Phase 2 | Complete |
| STAT-03 | Phase 2 | Complete |
| STAT-04 | Phase 2 | Complete |
| STAT-05 | Phase 3 | Complete |
| STAT-06 | Phase 3 | Complete |
| STAT-07 | Phase 2 | Pending |
| STAT-08 | Phase 3 | Complete |
| EX-01 | Phase 3 | Complete |
| EX-02 | Phase 3 | Complete |
| EX-03 | Phase 3 | Complete |
| EXP-01 | Phase 1 | Complete |
| EXP-02 | Phase 1 | Complete |
| EXP-03 | Phase 1 | Complete |
| EXP-04 | Phase 1 | Complete |
| PAGE-01 | Phase 2 | Pending |
| PAGE-02 | Phase 2 | Complete |
| PAGE-03 | Phase 3 | Pending |
| PAGE-04 | Phase 2 | Pending |

**Coverage:**
- v1 requirements: 23 total
- Mapped to phases: 23
- Unmapped: 0 ✓

---
*Requirements defined: 2026-03-21*
*Last updated: 2026-03-21 after initial definition*