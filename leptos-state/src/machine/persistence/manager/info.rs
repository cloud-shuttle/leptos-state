//! Machine information and metadata for persistence

/// Machine information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MachineInfo {
    /// Machine ID
    pub id: String,
    /// Machine name
    pub name: Option<String>,
    /// Creation time
    pub created_at: u64,
    /// Last modified time
    pub last_modified: u64,
    /// Size in bytes
    pub size_bytes: u64,
    /// State count
    pub state_count: u32,
    /// Transition count
    pub transition_count: u32,
    /// Has auto-save enabled
    pub auto_save_enabled: bool,
    /// Backup count
    pub backup_count: u32,
    /// Last backup time
    pub last_backup_time: Option<u64>,
}

impl MachineInfo {
    /// Create new machine info
    pub fn new(id: String) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id,
            name: None,
            created_at: now,
            last_modified: now,
            size_bytes: 0,
            state_count: 0,
            transition_count: 0,
            auto_save_enabled: false,
            backup_count: 0,
            last_backup_time: None,
        }
    }

    /// Set machine name
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Update size
    pub fn with_size(mut self, size_bytes: u64) -> Self {
        self.size_bytes = size_bytes;
        self.last_modified = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self
    }

    /// Set state count
    pub fn with_state_count(mut self, count: u32) -> Self {
        self.state_count = count;
        self
    }

    /// Set transition count
    pub fn with_transition_count(mut self, count: u32) -> Self {
        self.transition_count = count;
        self
    }

    /// Enable auto-save
    pub fn with_auto_save(mut self, enabled: bool) -> Self {
        self.auto_save_enabled = enabled;
        self
    }

    /// Update backup info
    pub fn with_backup_info(mut self, count: u32, last_time: Option<u64>) -> Self {
        self.backup_count = count;
        self.last_backup_time = last_time;
        self
    }

    /// Touch (update last modified)
    pub fn touch(&mut self) {
        self.last_modified = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }

    /// Get display name
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or(&self.id)
    }

    /// Check if recently modified
    pub fn is_recently_modified(&self, threshold_seconds: u64) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now.saturating_sub(self.last_modified) <= threshold_seconds
    }

    /// Check if has backups
    pub fn has_backups(&self) -> bool {
        self.backup_count > 0
    }

    /// Get age in seconds
    pub fn age_seconds(&self) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now.saturating_sub(self.created_at)
    }

    /// Get formatted size
    pub fn formatted_size(&self) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = self.size_bytes as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        format!("{:.1} {}", size, UNITS[unit_index])
    }

    /// Get summary
    pub fn summary(&self) -> String {
        let name = self.display_name();
        let size = self.formatted_size();
        let age_days = self.age_seconds() / 86400;

        format!(
            "Machine '{}' ({}), {} states, {} transitions, {} backups, {} old",
            name, size, self.state_count, self.transition_count, self.backup_count, age_days
        )
    }
}

impl std::fmt::Display for MachineInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

impl std::hash::Hash for MachineInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for MachineInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for MachineInfo {}

impl PartialOrd for MachineInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MachineInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}
