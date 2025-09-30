//! Core persistence traits and configurations

use super::*;

/// Trait for serializing state machine data
pub trait MachineSerialize {
    /// Serialize the machine to a JSON string
    fn to_json(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;

    /// Serialize the machine to a compact binary format
    fn to_binary(&self) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>;

    /// Get the serialization format version
    fn format_version(&self) -> u32 {
        1
    }
}

/// Trait for deserializing state machine data
pub trait MachineDeserialize<T> {
    /// Deserialize from a JSON string
    fn from_json(json: &str) -> Result<T, Box<dyn std::error::Error + Send + Sync>>;

    /// Deserialize from a binary format
    fn from_binary(data: &[u8]) -> Result<T, Box<dyn std::error::Error + Send + Sync>>;

    /// Check if the format version is supported
    fn supports_version(version: u32) -> bool {
        version == 1
    }
}

/// Persistence configuration for state machines
#[derive(Debug, Clone, PartialEq)]
pub struct PersistenceConfig {
    /// Whether persistence is enabled
    pub enabled: bool,
    /// Storage backend to use
    pub storage_type: StorageType,
    /// Auto-save interval in seconds (0 = disabled)
    pub auto_save_interval: u64,
    /// Maximum number of backups to keep
    pub max_backups: usize,
    /// Compression level (0-9, 0 = no compression)
    pub compression_level: u32,
    /// Whether to validate data on load
    pub validate_on_load: bool,
    /// Custom storage configuration
    pub custom_config: std::collections::HashMap<String, String>,
}

impl Default for PersistenceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            storage_type: StorageType::LocalStorage,
            auto_save_interval: 30, // 30 seconds
            max_backups: 10,
            compression_level: 6,
            validate_on_load: true,
            custom_config: std::collections::HashMap::new(),
        }
    }
}

impl PersistenceConfig {
    /// Create a new persistence config
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable or disable persistence
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set storage type
    pub fn storage_type(mut self, storage_type: StorageType) -> Self {
        self.storage_type = storage_type;
        self
    }

    /// Set auto-save interval
    pub fn auto_save_interval(mut self, interval: u64) -> Self {
        self.auto_save_interval = interval;
        self
    }

    /// Set maximum backups
    pub fn max_backups(mut self, max: usize) -> Self {
        self.max_backups = max;
        self
    }

    /// Set compression level
    pub fn compression_level(mut self, level: u32) -> Self {
        self.compression_level = level;
        self
    }

    /// Enable or disable validation on load
    pub fn validate_on_load(mut self, validate: bool) -> Self {
        self.validate_on_load = validate;
        self
    }

    /// Add custom configuration
    pub fn custom_config(mut self, key: String, value: String) -> Self {
        self.custom_config.insert(key, value);
        self
    }

    /// Check if auto-save is enabled
    pub fn auto_save_enabled(&self) -> bool {
        self.enabled && self.auto_save_interval > 0
    }

    /// Get auto-save duration
    pub fn auto_save_duration(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.auto_save_interval)
    }
}

/// Storage backend types
#[derive(Debug, Clone, PartialEq)]
pub enum StorageType {
    /// Browser localStorage
    LocalStorage,
    /// Browser sessionStorage
    SessionStorage,
    /// IndexedDB
    IndexedDB,
    /// WebSQL (deprecated)
    WebSQL,
    /// File system (server-side)
    FileSystem,
    /// Memory storage (for testing)
    Memory,
    /// Custom storage backend
    Custom(String),
}

impl StorageType {
    /// Check if storage type is browser-based
    pub fn is_browser_storage(&self) -> bool {
        matches!(
            self,
            StorageType::LocalStorage
                | StorageType::SessionStorage
                | StorageType::IndexedDB
                | StorageType::WebSQL
        )
    }

    /// Check if storage type is server-side
    pub fn is_server_storage(&self) -> bool {
        matches!(self, StorageType::FileSystem)
    }

    /// Check if storage type is memory-based
    pub fn is_memory_storage(&self) -> bool {
        matches!(self, StorageType::Memory)
    }

    /// Get storage type as string
    pub fn as_str(&self) -> &str {
        match self {
            StorageType::LocalStorage => "localStorage",
            StorageType::SessionStorage => "sessionStorage",
            StorageType::IndexedDB => "indexedDB",
            StorageType::WebSQL => "webSQL",
            StorageType::FileSystem => "filesystem",
            StorageType::Memory => "memory",
            StorageType::Custom(ref s) => s,
        }
    }
}

/// Backup configuration for state machine persistence
#[derive(Debug, Clone, PartialEq)]
pub struct BackupConfig {
    /// Whether backups are enabled
    pub enabled: bool,
    /// Backup interval in seconds (0 = disabled)
    pub interval: u64,
    /// Maximum number of backups to keep
    pub max_backups: usize,
    /// Backup naming pattern
    pub naming_pattern: String,
    /// Whether to compress backups
    pub compress: bool,
    /// Backup directory/path
    pub backup_path: Option<String>,
    /// Whether to include metadata in backups
    pub include_metadata: bool,
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: 3600, // 1 hour
            max_backups: 10,
            naming_pattern: "backup-{timestamp}-{version}.json".to_string(),
            compress: true,
            backup_path: None,
            include_metadata: true,
        }
    }
}

impl BackupConfig {
    /// Create a new backup config
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable or disable backups
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set backup interval
    pub fn interval(mut self, interval: u64) -> Self {
        self.interval = interval;
        self
    }

    /// Set maximum backups
    pub fn max_backups(mut self, max: usize) -> Self {
        self.max_backups = max;
        self
    }

    /// Set naming pattern
    pub fn naming_pattern(mut self, pattern: String) -> Self {
        self.naming_pattern = pattern;
        self
    }

    /// Enable or disable compression
    pub fn compress(mut self, compress: bool) -> Self {
        self.compress = compress;
        self
    }

    /// Set backup path
    pub fn backup_path(mut self, path: String) -> Self {
        self.backup_path = Some(path);
        self
    }

    /// Enable or disable metadata inclusion
    pub fn include_metadata(mut self, include: bool) -> Self {
        self.include_metadata = include;
        self
    }

    /// Check if backups are scheduled
    pub fn scheduled_backups_enabled(&self) -> bool {
        self.enabled && self.interval > 0
    }

    /// Get backup interval as duration
    pub fn backup_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.interval)
    }

    /// Generate backup filename
    pub fn generate_backup_name(&self, timestamp: u64, version: u32) -> String {
        self.naming_pattern
            .replace("{timestamp}", &timestamp.to_string())
            .replace("{version}", &version.to_string())
    }
}

/// Persistence strategy types
#[derive(Debug, Clone, PartialEq)]
pub enum PersistenceStrategy {
    /// Save on every state change
    Immediate,
    /// Save periodically
    Periodic(std::time::Duration),
    /// Save on demand only
    Manual,
    /// Save on application events (blur, beforeunload, etc.)
    EventBased(Vec<String>),
    /// Hybrid approach combining multiple strategies
    Hybrid(Vec<PersistenceStrategy>),
}

impl PersistenceStrategy {
    /// Create immediate persistence
    pub fn immediate() -> Self {
        PersistenceStrategy::Immediate
    }

    /// Create periodic persistence
    pub fn periodic(interval: std::time::Duration) -> Self {
        PersistenceStrategy::Periodic(interval)
    }

    /// Create manual persistence
    pub fn manual() -> Self {
        PersistenceStrategy::Manual
    }

    /// Create event-based persistence
    pub fn event_based(events: Vec<String>) -> Self {
        PersistenceStrategy::EventBased(events)
    }

    /// Check if strategy requires immediate saving
    pub fn is_immediate(&self) -> bool {
        matches!(self, PersistenceStrategy::Immediate)
    }

    /// Check if strategy is periodic
    pub fn is_periodic(&self) -> bool {
        matches!(self, PersistenceStrategy::Periodic(_))
    }

    /// Get periodic interval if applicable
    pub fn periodic_interval(&self) -> Option<std::time::Duration> {
        match self {
            PersistenceStrategy::Periodic(duration) => Some(*duration),
            PersistenceStrategy::Hybrid(strategies) => {
                strategies.iter().find_map(|s| s.periodic_interval())
            }
            _ => None,
        }
    }

    /// Check if strategy should save based on event
    pub fn should_save_on_event(&self, event: &str) -> bool {
        match self {
            PersistenceStrategy::EventBased(events) => events.contains(&event.to_string()),
            PersistenceStrategy::Hybrid(strategies) => {
                strategies.iter().any(|s| s.should_save_on_event(event))
            }
            _ => false,
        }
    }
}

/// Persistence error types
#[derive(Debug, Clone)]
pub enum PersistenceError {
    /// Storage backend error
    StorageError(String),
    /// Serialization error
    SerializationError(String),
    /// Deserialization error
    DeserializationError(String),
    /// Validation error
    ValidationError(String),
    /// Configuration error
    ConfigError(String),
    /// Version compatibility error
    VersionError { current: u32, required: u32 },
    /// Backup error
    BackupError(String),
    /// Restore error
    RestoreError(String),
}

impl std::fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PersistenceError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            PersistenceError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            PersistenceError::DeserializationError(msg) => {
                write!(f, "Deserialization error: {}", msg)
            }
            PersistenceError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            PersistenceError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            PersistenceError::VersionError { current, required } => {
                write!(
                    f,
                    "Version error: current={}, required={}",
                    current, required
                )
            }
            PersistenceError::BackupError(msg) => write!(f, "Backup error: {}", msg),
            PersistenceError::RestoreError(msg) => write!(f, "Restore error: {}", msg),
        }
    }
}

impl std::error::Error for PersistenceError {}

impl PersistenceError {
    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            PersistenceError::StorageError(_) => true,
            PersistenceError::SerializationError(_) => false,
            PersistenceError::DeserializationError(_) => false,
            PersistenceError::ValidationError(_) => false,
            PersistenceError::ConfigError(_) => false,
            PersistenceError::VersionError { .. } => false,
            PersistenceError::BackupError(_) => true,
            PersistenceError::RestoreError(_) => true,
        }
    }

    /// Get error category
    pub fn category(&self) -> &'static str {
        match self {
            PersistenceError::StorageError(_) => "storage",
            PersistenceError::SerializationError(_) => "serialization",
            PersistenceError::DeserializationError(_) => "deserialization",
            PersistenceError::ValidationError(_) => "validation",
            PersistenceError::ConfigError(_) => "configuration",
            PersistenceError::VersionError { .. } => "version",
            PersistenceError::BackupError(_) => "backup",
            PersistenceError::RestoreError(_) => "restore",
        }
    }
}
