//! Persistence statistics and metrics

use crate::machine::persistence::storage::StorageInfo;

/// Persistence statistics
#[derive(Debug, Clone)]
pub struct PersistenceStats {
    /// Total number of machines
    pub total_machines: usize,
    /// Number of active machines
    pub active_machines: usize,
    /// Auto-save enabled
    pub auto_save_enabled: bool,
    /// Total size in bytes
    pub total_size_bytes: u64,
    /// Last backup time
    pub last_backup_time: Option<u64>,
    /// Number of backups
    pub backup_count: usize,
    /// Storage information
    pub storage_info: StorageInfo,
}

impl PersistenceStats {
    /// Create empty statistics
    pub fn new() -> Self {
        Self {
            total_machines: 0,
            active_machines: 0,
            auto_save_enabled: false,
            total_size_bytes: 0,
            last_backup_time: None,
            backup_count: 0,
            storage_info: StorageInfo::default(),
        }
    }

    /// Calculate active machine percentage
    pub fn active_machine_percentage(&self) -> f64 {
        if self.total_machines == 0 {
            0.0
        } else {
            (self.active_machines as f64 / self.total_machines as f64) * 100.0
        }
    }

    /// Calculate average machine size
    pub fn average_machine_size(&self) -> f64 {
        if self.total_machines == 0 {
            0.0
        } else {
            self.total_size_bytes as f64 / self.total_machines as f64
        }
    }

    /// Check if backups are recent (within last 24 hours)
    pub fn has_recent_backup(&self) -> bool {
        self.last_backup_time.map_or(false, |time| {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            now.saturating_sub(time) < 86400 // 24 hours
        })
    }

    /// Get formatted total size
    pub fn formatted_total_size(&self) -> String {
        format_bytes(self.total_size_bytes)
    }

    /// Get formatted average size
    pub fn formatted_average_size(&self) -> String {
        format_bytes(self.average_machine_size() as u64)
    }

    /// Get storage efficiency (if available)
    pub fn storage_efficiency(&self) -> Option<f64> {
        // This would depend on the storage backend providing efficiency metrics
        None
    }

    /// Generate summary report
    pub fn summary(&self) -> String {
        format!(
            "Persistence: {} machines ({} active), {}, {} backups, storage: {}",
            self.total_machines,
            self.active_machines,
            self.formatted_total_size(),
            self.backup_count,
            self.storage_info.storage_type.as_str()
        )
    }

    /// Generate detailed report
    pub fn detailed_report(&self) -> String {
        let mut report = format!("Persistence Statistics Report\n");
        report.push_str(&format!("={}\n", "=".repeat(40)));
        report.push_str(&format!("Total Machines: {}\n", self.total_machines));
        report.push_str(&format!("Active Machines: {} ({:.1}%)\n", self.active_machines, self.active_machine_percentage()));
        report.push_str(&format!("Auto-save Enabled: {}\n", self.auto_save_enabled));
        report.push_str(&format!("Total Size: {}\n", self.formatted_total_size()));
        report.push_str(&format!("Average Size: {}\n", self.formatted_average_size()));
        report.push_str(&format!("Backup Count: {}\n", self.backup_count));
        report.push_str(&format!("Recent Backup: {}\n", self.has_recent_backup()));
        report.push_str(&format!("Storage Type: {}\n", self.storage_info.storage_type.as_str()));
        report.push_str(&format!("Storage Keys: {}\n", self.storage_info.key_count));
        report
    }
}

impl std::fmt::Display for PersistenceStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

impl Default for PersistenceStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to format bytes
fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, UNITS[unit_index])
}
