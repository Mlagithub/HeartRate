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
}