//! Persistence information and metadata

use std::time::SystemTime;

/// Persistence information
#[derive(Debug, Clone)]
pub struct PersistenceInfo {
    /// Storage type being used
    pub storage_type: String,
    /// Whether auto-save is enabled
    pub auto_save_enabled: bool,
    /// Last save time
    pub last_save_time: Option<SystemTime>,
    /// Number of saves performed
    pub save_count: u64,
}

impl PersistenceInfo {
    /// Create new persistence info
    pub fn new(storage_type: String) -> Self {
        Self {
            storage_type,
            auto_save_enabled: false,
            last_save_time: None,
            save_count: 0,
        }
    }

    /// Set auto-save enabled
    pub fn with_auto_save(mut self, enabled: bool) -> Self {
        self.auto_save_enabled = enabled;
        self
    }

    /// Record a save operation
    pub fn record_save(&mut self) {
        self.last_save_time = Some(SystemTime::now());
        self.save_count += 1;
    }

    /// Get time since last save
    pub fn time_since_last_save(&self) -> Option<std::time::Duration> {
        self.last_save_time.and_then(|time| time.elapsed().ok())
    }

    /// Check if persistence is active (has been used)
    pub fn is_active(&self) -> bool {
        self.save_count > 0
    }

    /// Get save frequency (saves per second, based on recent activity)
    pub fn save_frequency(&self) -> Option<f64> {
        if let Some(time_since_last) = self.time_since_last_save() {
            if time_since_last.as_secs() > 0 {
                self.save_count as f64 / time_since_last.as_secs_f64()
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get summary of persistence activity
    pub fn summary(&self) -> String {
        let mut summary = format!("Persistence: {} storage", self.storage_type);

        if self.auto_save_enabled {
            summary.push_str(" (auto-save enabled)");
        }

        summary.push_str(&format!(", {} saves", self.save_count));

        if let Some(time) = self.last_save_time {
            if let Ok(duration) = time.elapsed() {
                summary.push_str(&format!(", last save {:.1}s ago", duration.as_secs_f64()));
            }
        }

        summary
    }

    /// Check if a save should be performed (based on time threshold)
    pub fn should_save(&self, threshold: std::time::Duration) -> bool {
        self.time_since_last_save()
            .map(|duration| duration >= threshold)
            .unwrap_or(true)
    }
}

impl Default for PersistenceInfo {
    fn default() -> Self {
        Self::new("unknown".to_string())
    }
}

impl std::fmt::Display for PersistenceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}
