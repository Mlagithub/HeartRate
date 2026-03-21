use rusqlite::{Connection, Result as SqliteResult, params};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Mutex;

/// Heart rate record for database storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartRateRecord {
    pub id: Option<i64>,
    pub bpm: u16,
    pub sensor_contact: Option<bool>,
    pub timestamp: i64,
    pub session_id: String,
}

/// Alert settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertSettings {
    pub enabled: bool,
    pub low_threshold: u16,
    pub high_threshold: u16,
    pub notify_on_low: bool,
    pub notify_on_high: bool,
}

impl Default for AlertSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            low_threshold: 50,
            high_threshold: 180,
            notify_on_low: true,
            notify_on_high: true,
        }
    }
}

/// Aggregated statistics for a time period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodStats {
    pub period: String,        // ISO date string (YYYY-MM-DD, YYYY-WW, YYYY-MM, YYYY)
    pub min_bpm: u16,
    pub max_bpm: u16,
    pub avg_bpm: f64,
    pub record_count: i64,
}

/// Exercise tag for a session (per D-01, D-03, D-04)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseTag {
    pub session_id: String,
    pub exercise_type: String,  // "Running", "Cycling", "Swimming", "Gym", "Other" per D-02
    pub is_confirmed: bool,     // true for manual tags, false for auto-detected pending confirmation
    pub confidence: Option<f64>, // 0.0-1.0 for auto-detected, None for manual
    pub tagged_at: i64,         // timestamp when tagged
}

/// Session information for display in session list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub session_id: String,
    pub start_time: i64,
    pub end_time: i64,
    pub record_count: i64,
    pub avg_bpm: f64,
    pub exercise_tag: Option<ExerciseTag>,
}

/// HRV estimation result (per D-09, D-10, D-11)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HRVResult {
    pub hrv_value: f64,        // Estimated RMSSD-style value (ms)
    pub is_estimated: bool,    // Always true per D-11
    pub confidence: String,    // "low", "medium", "high" based on data quantity
    pub data_points: i64,      // Number of BPM readings used
    pub period_start: i64,
    pub period_end: i64,
}

/// Heart rate zones for exercise analysis (per D-14)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartRateZones {
    pub zone1_pct: f64,  // 50-60% max HR
    pub zone2_pct: f64,  // 60-70% max HR
    pub zone3_pct: f64,  // 70-80% max HR
    pub zone4_pct: f64,  // 80-90% max HR
    pub zone5_pct: f64,  // 90-100% max HR
}

impl Default for HeartRateZones {
    fn default() -> Self {
        Self {
            zone1_pct: 0.0,
            zone2_pct: 0.0,
            zone3_pct: 0.0,
            zone4_pct: 0.0,
            zone5_pct: 0.0,
        }
    }
}

/// Exercise vs resting comparison statistics (per D-14, D-15)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseStats {
    pub avg_exercise_hr: f64,
    pub avg_resting_hr: f64,
    pub exercise_sessions: i64,
    pub total_exercise_minutes: f64,
    pub hr_zones: HeartRateZones,
}

/// Statistics by exercise type (per STAT-08)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseTypeStats {
    pub exercise_type: String,
    pub session_count: i64,
    pub avg_hr: f64,
    pub max_hr: u16,
    pub min_hr: u16,
    pub total_minutes: f64,
}

/// Exercise detection result (per D-05, D-08)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionResult {
    pub session_id: String,
    pub is_exercise: bool,
    pub confidence: f64,  // 0.0-1.0
    pub reason: String,   // "Elevated HR for N minutes", "Insufficient baseline", etc.
}

/// Personal resting baseline statistics (per D-06, D-15)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineStats {
    pub resting_avg: f64,
    pub resting_stddev: f64,
    pub data_days: i64,
}

/// Database manager for heart rate data
pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    /// Create a new database connection
    pub fn new(path: &Path) -> SqliteResult<Self> {
        let conn = Connection::open(path)?;

        // Create tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS heart_rate_records (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                bpm INTEGER NOT NULL,
                sensor_contact INTEGER,
                timestamp INTEGER NOT NULL,
                session_id TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS alert_settings (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                enabled INTEGER NOT NULL,
                low_threshold INTEGER NOT NULL,
                high_threshold INTEGER NOT NULL,
                notify_on_low INTEGER NOT NULL,
                notify_on_high INTEGER NOT NULL
            )",
            [],
        )?;

        // Insert default alert settings if not exists
        conn.execute(
            "INSERT OR IGNORE INTO alert_settings (id, enabled, low_threshold, high_threshold, notify_on_low, notify_on_high)
             VALUES (1, 1, 50, 180, 1, 1)",
            [],
        )?;

        // Create indexes for faster queries
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_timestamp ON heart_rate_records(timestamp)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_session_id ON heart_rate_records(session_id)",
            [],
        )?;

        // Create exercise_tags table for session-level exercise marks (per D-04)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS exercise_tags (
                session_id TEXT PRIMARY KEY,
                exercise_type TEXT NOT NULL,
                is_confirmed INTEGER NOT NULL,
                confidence REAL,
                tagged_at INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Helper to get connection with proper error handling
    fn get_conn(&self) -> SqliteResult<std::sync::MutexGuard<'_, Connection>> {
        self.conn.lock().map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Database lock poisoned: {}", e)
        ))))
    }

    /// Save a heart rate record
    pub fn save_heart_rate(&self, record: &HeartRateRecord) -> SqliteResult<i64> {
        let conn = self.get_conn()?;
        conn.execute(
            "INSERT INTO heart_rate_records (bpm, sensor_contact, timestamp, session_id)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                record.bpm as i64,
                record.sensor_contact.map(|b| if b { 1i64 } else { 0i64 }),
                record.timestamp,
                &record.session_id,
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Get heart rate history
    pub fn get_history(&self, limit: i64, offset: i64) -> SqliteResult<Vec<HeartRateRecord>> {
        let conn = self.get_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, bpm, sensor_contact, timestamp, session_id
             FROM heart_rate_records
             ORDER BY timestamp DESC
             LIMIT ?1 OFFSET ?2",
        )?;

        let records = stmt
            .query_map(params![limit, offset], |row| {
                let sensor_contact_val: Option<i64> = row.get(2)?;
                let sensor_contact = sensor_contact_val.map(|v| v != 0);

                Ok(HeartRateRecord {
                    id: Some(row.get(0)?),
                    bpm: row.get::<_, i64>(1)? as u16,
                    sensor_contact,
                    timestamp: row.get(3)?,
                    session_id: row.get(4)?,
                })
            })?
            .collect::<SqliteResult<Vec<_>>>()?;

        Ok(records)
    }

    /// Get heart rate history for a time range
    pub fn get_history_range(
        &self,
        start_time: i64,
        end_time: i64,
    ) -> SqliteResult<Vec<HeartRateRecord>> {
        let conn = self.get_conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, bpm, sensor_contact, timestamp, session_id
             FROM heart_rate_records
             WHERE timestamp >= ?1 AND timestamp <= ?2
             ORDER BY timestamp ASC",
        )?;

        let records = stmt
            .query_map(params![start_time, end_time], |row| {
                let sensor_contact_val: Option<i64> = row.get(2)?;
                let sensor_contact = sensor_contact_val.map(|v| v != 0);

                Ok(HeartRateRecord {
                    id: Some(row.get(0)?),
                    bpm: row.get::<_, i64>(1)? as u16,
                    sensor_contact,
                    timestamp: row.get(3)?,
                    session_id: row.get(4)?,
                })
            })?
            .collect::<SqliteResult<Vec<_>>>()?;

        Ok(records)
    }

    /// Clear all records
    pub fn clear_all(&self) -> SqliteResult<()> {
        let conn = self.get_conn()?;
        conn.execute("DELETE FROM heart_rate_records", [])?;
        Ok(())
    }

    /// Get alert settings
    pub fn get_alert_settings(&self) -> SqliteResult<AlertSettings> {
        let conn = self.get_conn()?;
        let mut stmt = conn.prepare(
            "SELECT enabled, low_threshold, high_threshold, notify_on_low, notify_on_high
             FROM alert_settings WHERE id = 1",
        )?;

        let settings = stmt.query_row([], |row| {
            Ok(AlertSettings {
                enabled: row.get::<_, i64>(0)? != 0,
                low_threshold: row.get::<_, i64>(1)? as u16,
                high_threshold: row.get::<_, i64>(2)? as u16,
                notify_on_low: row.get::<_, i64>(3)? != 0,
                notify_on_high: row.get::<_, i64>(4)? != 0,
            })
        })?;

        Ok(settings)
    }

    /// Update alert settings
    pub fn set_alert_settings(&self, settings: &AlertSettings) -> SqliteResult<()> {
        let conn = self.get_conn()?;
        conn.execute(
            "UPDATE alert_settings
             SET enabled = ?1, low_threshold = ?2, high_threshold = ?3,
                 notify_on_low = ?4, notify_on_high = ?5
             WHERE id = 1",
            params![
                if settings.enabled { 1i64 } else { 0i64 },
                settings.low_threshold as i64,
                settings.high_threshold as i64,
                if settings.notify_on_low { 1i64 } else { 0i64 },
                if settings.notify_on_high { 1i64 } else { 0i64 },
            ],
        )?;
        Ok(())
    }

    /// Get aggregated statistics by time dimension
    pub fn get_statistics(
        &self,
        dimension: &str,  // "daily", "weekly", "monthly", "yearly"
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> SqliteResult<Vec<PeriodStats>> {
        let conn = self.get_conn()?;

        let period_expr = match dimension {
            "daily" => "date(timestamp / 1000, 'unixepoch', 'localtime')",
            "weekly" => "strftime('%Y-%W', timestamp / 1000, 'unixepoch', 'localtime')",
            "monthly" => "strftime('%Y-%m', timestamp / 1000, 'unixepoch', 'localtime')",
            "yearly" => "strftime('%Y', timestamp / 1000, 'unixepoch', 'localtime')",
            _ => return Err(rusqlite::Error::InvalidParameterName("Invalid dimension".into())),
        };

        let sql = format!(
            "SELECT
                {} as period,
                MIN(bpm) as min_bpm,
                MAX(bpm) as max_bpm,
                AVG(bpm) as avg_bpm,
                COUNT(*) as record_count
             FROM heart_rate_records
             WHERE (?1 IS NULL OR timestamp >= ?1)
               AND (?2 IS NULL OR timestamp <= ?2)
             GROUP BY period
             ORDER BY period ASC",
            period_expr
        );

        let mut stmt = conn.prepare(&sql)?;
        let stats = stmt
            .query_map(params![start_time, end_time], |row| {
                Ok(PeriodStats {
                    period: row.get(0)?,
                    min_bpm: row.get::<_, i64>(1)? as u16,
                    max_bpm: row.get::<_, i64>(2)? as u16,
                    avg_bpm: row.get(3)?,
                    record_count: row.get(4)?,
                })
            })?
            .collect::<SqliteResult<Vec<_>>>()?;

        Ok(stats)
    }

    /// Calculate HRV estimation from BPM variance (per D-09, D-10)
    /// Uses variance-based approximation since we don't have RR-intervals
    /// Formula: sqrt(variance) * scaling_factor approximates RMSSD
    pub fn calculate_hrv(
        &self,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> SqliteResult<Option<HRVResult>> {
        let conn = self.get_conn()?;

        // Query for count, avg, and avg of squares to compute variance
        let sql = "SELECT
                COUNT(*) as count,
                AVG(bpm) as avg_bpm,
                AVG(bpm * bpm) as avg_bpm_sq,
                MIN(timestamp) as min_ts,
                MAX(timestamp) as max_ts
            FROM heart_rate_records
            WHERE (?1 IS NULL OR timestamp >= ?1)
              AND (?2 IS NULL OR timestamp <= ?2)";

        let result: Option<(i64, f64, f64, Option<i64>, Option<i64>)> = conn
            .query_row(sql, params![start_time, end_time], |row| {
                let count: i64 = row.get(0)?;
                if count == 0 {
                    Ok(None)
                } else {
                    Ok(Some((
                        count,
                        row.get(1)?,
                        row.get(2)?,
                        row.get(3)?,
                        row.get(4)?,
                    )))
                }
            })?;

        let (count, avg_bpm, avg_bpm_sq, min_ts, max_ts) = match result {
            Some(data) => data,
            None => return Ok(None),
        };

        // Calculate variance: E[X^2] - E[X]^2
        let variance = avg_bpm_sq - (avg_bpm * avg_bpm);

        // RMSSD approximation: sqrt(variance) * scaling factor to ms range
        // Using scaling factor of 10 to convert BPM variance to approximate ms
        let hrv_value = variance.sqrt() * 10.0;

        // Determine confidence based on data points
        let confidence = if count >= 100 {
            "high"
        } else if count >= 50 {
            "medium"
        } else {
            "low"
        };

        Ok(Some(HRVResult {
            hrv_value,
            is_estimated: true, // Always true per D-11
            confidence: confidence.to_string(),
            data_points: count,
            period_start: min_ts.unwrap_or(0),
            period_end: max_ts.unwrap_or(0),
        }))
    }

    /// Tag a session as exercise (per D-01, D-03, D-04)
    pub fn tag_exercise(&self, tag: &ExerciseTag) -> SqliteResult<()> {
        let conn = self.get_conn()?;
        conn.execute(
            "INSERT OR REPLACE INTO exercise_tags
             (session_id, exercise_type, is_confirmed, confidence, tagged_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                &tag.session_id,
                &tag.exercise_type,
                if tag.is_confirmed { 1i64 } else { 0i64 },
                tag.confidence,
                tag.tagged_at,
            ],
        )?;
        Ok(())
    }

    /// Get exercise tag for a session
    pub fn get_exercise_tag(&self, session_id: &str) -> SqliteResult<Option<ExerciseTag>> {
        let conn = self.get_conn()?;
        let mut stmt = conn.prepare(
            "SELECT session_id, exercise_type, is_confirmed, confidence, tagged_at
             FROM exercise_tags WHERE session_id = ?1",
        )?;

        let result = stmt
            .query_row(params![session_id], |row| {
                Ok(ExerciseTag {
                    session_id: row.get(0)?,
                    exercise_type: row.get(1)?,
                    is_confirmed: row.get::<_, i64>(2)? != 0,
                    confidence: row.get(3)?,
                    tagged_at: row.get(4)?,
                })
            })
            .optional()?;

        Ok(result)
    }

    /// Get all sessions with exercise tags (confirmed only by default)
    pub fn get_sessions_with_exercise(&self, confirmed_only: bool) -> SqliteResult<Vec<ExerciseTag>> {
        let conn = self.get_conn()?;
        let sql = if confirmed_only {
            "SELECT session_id, exercise_type, is_confirmed, confidence, tagged_at
             FROM exercise_tags WHERE is_confirmed = 1
             ORDER BY tagged_at DESC"
        } else {
            "SELECT session_id, exercise_type, is_confirmed, confidence, tagged_at
             FROM exercise_tags
             ORDER BY tagged_at DESC"
        };

        let mut stmt = conn.prepare(sql)?;
        let tags = stmt
            .query_map([], |row| {
                Ok(ExerciseTag {
                    session_id: row.get(0)?,
                    exercise_type: row.get(1)?,
                    is_confirmed: row.get::<_, i64>(2)? != 0,
                    confidence: row.get(3)?,
                    tagged_at: row.get(4)?,
                })
            })?
            .collect::<SqliteResult<Vec<_>>>()?;

        Ok(tags)
    }

    /// Get unique sessions from heart_rate_records with aggregated info
    pub fn get_sessions(&self, limit: i64, offset: i64) -> SqliteResult<Vec<SessionInfo>> {
        let conn = self.get_conn()?;
        let mut stmt = conn.prepare(
            "SELECT
                session_id,
                MIN(timestamp) as start_time,
                MAX(timestamp) as end_time,
                COUNT(*) as record_count,
                AVG(bpm) as avg_bpm
             FROM heart_rate_records
             GROUP BY session_id
             ORDER BY start_time DESC
             LIMIT ?1 OFFSET ?2",
        )?;

        let sessions = stmt
            .query_map(params![limit, offset], |row| {
                Ok(SessionInfo {
                    session_id: row.get(0)?,
                    start_time: row.get(1)?,
                    end_time: row.get(2)?,
                    record_count: row.get(3)?,
                    avg_bpm: row.get(4)?,
                    exercise_tag: None, // Will be populated separately
                })
            })?
            .collect::<SqliteResult<Vec<_>>>()?;

        // Fetch exercise tags for these sessions
        let mut result = Vec::with_capacity(sessions.len());
        for mut session in sessions {
            session.exercise_tag = self.get_exercise_tag(&session.session_id)?;
            result.push(session);
        }

        Ok(result)
    }

    /// Remove exercise tag for a session
    pub fn remove_exercise_tag(&self, session_id: &str) -> SqliteResult<()> {
        let conn = self.get_conn()?;
        conn.execute(
            "DELETE FROM exercise_tags WHERE session_id = ?1",
            params![session_id],
        )?;
        Ok(())
    }

    /// Calculate personal resting baseline (per D-06, D-15)
    /// Returns None if less than 7 days of data
    pub fn calculate_resting_baseline(&self) -> SqliteResult<Option<BaselineStats>> {
        let conn = self.get_conn()?;

        // Get data from the past 30 days, looking at the lowest 10th percentile HR
        // to estimate resting heart rate
        let sql = "
            WITH daily_mins AS (
                SELECT
                    date(timestamp / 1000, 'unixepoch', 'localtime') as day,
                    MIN(bpm) as daily_min
                FROM heart_rate_records
                WHERE timestamp >= (strftime('%s', 'now', '-30 days') * 1000)
                GROUP BY day
            )
            SELECT
                COUNT(DISTINCT day) as data_days,
                AVG(daily_min) as resting_avg
            FROM daily_mins
        ";

        let result: Option<(i64, f64)> = conn
            .query_row(sql, [], |row| {
                let days: i64 = row.get(0)?;
                if days < 7 {
                    Ok(None)
                } else {
                    Ok(Some((days, row.get(1)?)))
                }
            })
            .optional()?;

        let (data_days, resting_avg) = match result {
            Some(Some(data)) => data,
            _ => return Ok(None),
        };

        // Calculate standard deviation of daily minimums
        let stddev_sql = "
            WITH daily_mins AS (
                SELECT
                    date(timestamp / 1000, 'unixepoch', 'localtime') as day,
                    MIN(bpm) as daily_min
                FROM heart_rate_records
                WHERE timestamp >= (strftime('%s', 'now', '-30 days') * 1000)
                GROUP BY day
            )
            SELECT
                AVG(daily_min * daily_min) - AVG(daily_min) * AVG(daily_min) as variance
            FROM daily_mins
        ";

        let variance: f64 = conn.query_row(stddev_sql, [], |row| row.get(0))?;
        let resting_stddev = variance.sqrt();

        Ok(Some(BaselineStats {
            resting_avg,
            resting_stddev,
            data_days,
        }))
    }

    /// Detect if a session is exercise (per D-05)
    /// Threshold: sustained HR > resting_avg + 30 BPM for 5+ minutes
    pub fn detect_exercise(&self, session_id: &str) -> SqliteResult<DetectionResult> {
        // First check if we have a baseline
        let baseline = self.calculate_resting_baseline()?;

        // Get session statistics
        let conn = self.get_conn()?;
        let session_sql = "
            SELECT
                COUNT(*) as record_count,
                AVG(bpm) as avg_bpm,
                MAX(bpm) as max_bpm,
                MIN(timestamp) as start_time,
                MAX(timestamp) as end_time
            FROM heart_rate_records
            WHERE session_id = ?1
        ";

        let session_info: Option<(i64, f64, i64, i64, i64)> = conn
            .query_row(session_sql, params![session_id], |row| {
                let count: i64 = row.get(0)?;
                if count == 0 {
                    Ok(None)
                } else {
                    Ok(Some((
                        count,
                        row.get(1)?,
                        row.get(2)?,
                        row.get(3)?,
                        row.get(4)?,
                    )))
                }
            })
            .optional()?;

        let (record_count, avg_bpm, max_bpm, start_time, end_time) = match session_info {
            Some(Some(data)) => data,
            _ => {
                return Ok(DetectionResult {
                    session_id: session_id.to_string(),
                    is_exercise: false,
                    confidence: 0.0,
                    reason: "Session not found or empty".to_string(),
                });
            }
        };

        // Check session duration (need at least 5 minutes = 300000ms)
        let duration_ms = end_time - start_time;
        if duration_ms < 300_000 {
            return Ok(DetectionResult {
                session_id: session_id.to_string(),
                is_exercise: false,
                confidence: 0.0,
                reason: format!("Session too short: {} minutes", duration_ms / 60000),
            });
        }

        // If no baseline, can't detect
        let baseline = match baseline {
            Some(b) => b,
            None => {
                return Ok(DetectionResult {
                    session_id: session_id.to_string(),
                    is_exercise: false,
                    confidence: 0.0,
                    reason: "Insufficient baseline data (need 7+ days)".to_string(),
                });
            }
        };

        // Detection threshold: avg HR > resting_avg + 30 BPM
        let threshold = baseline.resting_avg + 30.0;
        let is_exercise = avg_bpm > threshold;

        // Confidence calculation (per D-08)
        // Base confidence: how much above threshold, scaled
        // Higher HR above threshold = higher confidence
        let hr_above_threshold = avg_bpm - threshold;
        let confidence = if is_exercise {
            // Scale confidence from 0.5 to 1.0 based on how much above threshold
            // 0 BPM above = 0.5, 40+ BPM above = 1.0
            (0.5 + hr_above_threshold / 80.0).min(1.0).max(0.5)
        } else {
            0.0
        };

        let reason = if is_exercise {
            format!(
                "Elevated HR: avg {:.0} BPM vs resting {:.0} BPM (threshold {:.0})",
                avg_bpm, baseline.resting_avg, threshold
            )
        } else {
            format!(
                "Normal HR: avg {:.0} BPM below threshold {:.0}",
                avg_bpm, threshold
            )
        };

        Ok(DetectionResult {
            session_id: session_id.to_string(),
            is_exercise,
            confidence,
            reason,
        })
    }

    /// Detect exercise for all untagged sessions
    pub fn detect_exercise_for_sessions(&self) -> SqliteResult<Vec<DetectionResult>> {
        let conn = self.get_conn()?;

        // Get all session IDs that don't have exercise tags
        let sql = "
            SELECT DISTINCT session_id
            FROM heart_rate_records
            WHERE session_id NOT IN (SELECT session_id FROM exercise_tags)
        ";

        let mut stmt = conn.prepare(sql)?;
        let session_ids: Vec<String> = stmt
            .query_map([], |row| row.get(0))?
            .collect::<SqliteResult<Vec<_>>>()?;

        let mut results = Vec::with_capacity(session_ids.len());
        for session_id in session_ids {
            let result = self.detect_exercise(&session_id)?;
            // Only include results with confidence >= 0.5 per D-08
            if result.is_exercise && result.confidence >= 0.5 {
                results.push(result);
            }
        }

        Ok(results)
    }

    /// Get exercise vs resting comparison statistics (per D-14, D-15)
    pub fn get_exercise_statistics(&self) -> SqliteResult<ExerciseStats> {
        let conn = self.get_conn()?;

        // Get average HR from confirmed exercise sessions
        let avg_exercise_hr: Option<f64> = conn.query_row(
            "SELECT AVG(h.bpm)
             FROM heart_rate_records h
             JOIN exercise_tags e ON h.session_id = e.session_id
             WHERE e.is_confirmed = 1",
            [],
            |row| row.get(0),
        )?;

        // Get exercise session count
        let exercise_sessions: i64 = conn.query_row(
            "SELECT COUNT(DISTINCT session_id) FROM exercise_tags WHERE is_confirmed = 1",
            [],
            |row| row.get(0),
        )?;

        // Calculate total exercise minutes from session durations
        let total_exercise_minutes: f64 = conn.query_row(
            "SELECT COALESCE(SUM(duration_minutes), 0.0)
             FROM (
                 SELECT (MAX(h.timestamp) - MIN(h.timestamp)) / 60000.0 as duration_minutes
                 FROM heart_rate_records h
                 JOIN exercise_tags e ON h.session_id = e.session_id
                 WHERE e.is_confirmed = 1
                 GROUP BY h.session_id
             )",
            [],
            |row| row.get(0),
        )?;

        // Get all exercise HR data for zone calculation
        let hr_data: Vec<u16> = conn
            .prepare(
                "SELECT h.bpm
                 FROM heart_rate_records h
                 JOIN exercise_tags e ON h.session_id = e.session_id
                 WHERE e.is_confirmed = 1",
            )?
            .query_map([], |row| Ok(row.get::<_, i64>(0)? as u16))?
            .collect::<SqliteResult<Vec<_>>>()?;

        // Calculate HR zones (assuming age 40 for max HR = 180)
        let max_hr = 180.0; // 220 - 40 (default age)
        let hr_zones = if !hr_data.is_empty() {
            let total = hr_data.len() as f64;
            let zone1_count = hr_data.iter().filter(|&&hr| hr as f64 >= max_hr * 0.5 && hr as f64 < max_hr * 0.6).count();
            let zone2_count = hr_data.iter().filter(|&&hr| hr as f64 >= max_hr * 0.6 && hr as f64 < max_hr * 0.7).count();
            let zone3_count = hr_data.iter().filter(|&&hr| hr as f64 >= max_hr * 0.7 && hr as f64 < max_hr * 0.8).count();
            let zone4_count = hr_data.iter().filter(|&&hr| hr as f64 >= max_hr * 0.8 && hr as f64 < max_hr * 0.9).count();
            let zone5_count = hr_data.iter().filter(|&&hr| hr as f64 >= max_hr * 0.9).count();

            HeartRateZones {
                zone1_pct: (zone1_count as f64 / total) * 100.0,
                zone2_pct: (zone2_count as f64 / total) * 100.0,
                zone3_pct: (zone3_count as f64 / total) * 100.0,
                zone4_pct: (zone4_count as f64 / total) * 100.0,
                zone5_pct: (zone5_count as f64 / total) * 100.0,
            }
        } else {
            HeartRateZones::default()
        };

        // Get resting baseline
        let avg_resting_hr = match self.calculate_resting_baseline()? {
            Some(baseline) => baseline.resting_avg,
            None => 70.0, // Default if no baseline data
        };

        Ok(ExerciseStats {
            avg_exercise_hr: avg_exercise_hr.unwrap_or(0.0),
            avg_resting_hr,
            exercise_sessions,
            total_exercise_minutes,
            hr_zones,
        })
    }

    /// Get statistics grouped by exercise type (per STAT-08)
    pub fn get_exercise_type_statistics(&self) -> SqliteResult<Vec<ExerciseTypeStats>> {
        let conn = self.get_conn()?;

        let mut stmt = conn.prepare(
            "SELECT
                e.exercise_type,
                COUNT(DISTINCT e.session_id) as session_count,
                AVG(h.bpm) as avg_hr,
                MAX(h.bpm) as max_hr,
                MIN(h.bpm) as min_hr,
                SUM((SELECT MAX(timestamp) - MIN(timestamp)
                     FROM heart_rate_records h2
                     WHERE h2.session_id = e.session_id) / 60000.0) as total_minutes
             FROM exercise_tags e
             JOIN heart_rate_records h ON e.session_id = h.session_id
             WHERE e.is_confirmed = 1
             GROUP BY e.exercise_type
             ORDER BY session_count DESC",
        )?;

        let stats = stmt
            .query_map([], |row| {
                Ok(ExerciseTypeStats {
                    exercise_type: row.get(0)?,
                    session_count: row.get(1)?,
                    avg_hr: row.get(2)?,
                    max_hr: row.get::<_, i64>(3)? as u16,
                    min_hr: row.get::<_, i64>(4)? as u16,
                    total_minutes: row.get::<_, Option<f64>>(5)?.unwrap_or(0.0),
                })
            })?
            .collect::<SqliteResult<Vec<_>>>()?;

        Ok(stats)
    }
}