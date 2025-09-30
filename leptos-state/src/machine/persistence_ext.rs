//! Extension traits for persistence

use super::persistence_core::PersistenceError;
use super::*;

/// Extension trait for adding persistence to machines
pub trait MachinePersistenceExt<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static> {
    /// Create a persistent machine
    fn with_persistence(
        self,
        config: PersistenceConfig,
    ) -> Result<PersistentMachine<C, E>, PersistenceError>;

    /// Enable persistence with default configuration
    fn persistent(self) -> Result<PersistentMachine<C, E>, PersistenceError>;
}

impl<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static> MachinePersistenceExt<C, E> for Machine<C, E, C> {
    fn with_persistence(
        self,
        config: PersistenceConfig,
    ) -> Result<PersistentMachine<C, E>, PersistenceError> {
        let storage = super::storage::StorageFactory::new().create_storage(&config.storage_type, super::storage::StorageConfig::new())?;
        let persistence_manager = MachinePersistence::new(storage, config);
        Ok(PersistentMachine::new(self, persistence_manager))
    }

    fn persistent(self) -> Result<PersistentMachine<C, E>, PersistenceError> {
        let config = PersistenceConfig::default();
        self.with_persistence(config)
    }
}

/// A state machine with persistence capabilities
pub struct PersistentMachine<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static> {
    /// The underlying machine
    machine: Machine<C, E, C>,
    /// Persistence manager
    persistence: MachinePersistence<C, E>,
    /// Auto-save enabled
    auto_save_enabled: bool,
}

impl<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static> PersistentMachine<C, E> {
    /// Create a new persistent machine
    pub fn new(machine: Machine<C, E, C>, persistence: MachinePersistence<C, E>) -> Self {
        Self {
            machine,
            persistence,
            auto_save_enabled: false,
        }
    }

    /// Enable auto-save
    pub fn with_auto_save(mut self) -> Self {
        self.auto_save_enabled = true;
        self.persistence
            .start_auto_save(std::sync::Arc::new(self.machine.clone()));
        self
    }

    /// Save the current state
    pub async fn save(&self) -> Result<(), PersistenceError> {
        self.persistence.persist_machine(&self.machine).await
    }

    /// Load a saved state
    pub async fn load(
        machine_id: &str,
        persistence: MachinePersistence<C, E>,
    ) -> Result<Self, PersistenceError> {
        let machine = persistence.restore_machine(machine_id).await?;
        Ok(Self::new(machine, persistence))
    }

    /// Get the machine ID
    pub fn id(&self) -> &str {
        self.machine.id()
    }

    /// Get the current state
    pub fn current_state(&self) -> &MachineStateImpl<C> {
        self.machine.current_state()
    }

    /// Get available states
    pub fn get_states(&self) -> Vec<String> {
        self.machine.get_states()
    }

    /// Transition to a new state
    pub fn transition(&mut self, event: E) -> Result<(), MachineError> {
        let result = self.machine.transition(event);

        // Auto-save if enabled and transition succeeded
        if self.auto_save_enabled && result.is_ok() {
            let persistence = self.persistence.clone();
            let machine = self.machine.clone();

            tokio::spawn(async move {
                if let Err(e) = persistence.persist_machine(&machine).await {
                    eprintln!("Auto-save failed: {:?}", e);
                }
            });
        }

        result
    }

    /// Check if a transition is possible
    pub fn can_transition(&self, target: &str) -> bool {
        self.machine.can_transition(target)
    }

    /// Get the underlying machine
    pub fn machine(&self) -> &Machine<C, E, C> {
        &self.machine
    }

    /// Get the persistence manager
    pub fn persistence(&self) -> &MachinePersistence<C, E> {
        &self.persistence
    }

    /// Create a backup
    pub async fn create_backup(&self) -> Result<String, PersistenceError> {
        MachinePersistence::create_backup(&self.persistence, self.id()).await
    }

    /// Get machine info
    pub async fn info(&self) -> Result<MachineInfo, PersistenceError> {
        MachinePersistence::get_machine_info(&self.persistence, self.id()).await
    }
}

impl<C: Clone + Send + Sync + 'static, E: Clone + Send + Sync + 'static> Clone for PersistentMachine<C, E> {
    fn clone(&self) -> Self {
        Self {
            machine: self.machine.clone(),
            persistence: self.persistence.clone(),
            auto_save_enabled: self.auto_save_enabled,
        }
    }
}

/// Persistence information
#[derive(Debug, Clone)]
pub struct PersistenceInfo {
    /// Whether persistence is enabled
    pub enabled: bool,
    /// Storage type
    pub storage_type: String,
    /// Auto-save interval
    pub auto_save_interval: u64,
    /// Number of backups
    pub backup_count: usize,
    /// Total storage size
    pub total_size: u64,
    /// Last save time
    pub last_save: Option<std::time::Instant>,
    /// Last load time
    pub last_load: Option<std::time::Instant>,
}

impl PersistenceInfo {
    /// Create new persistence info
    pub fn new() -> Self {
        Self {
            enabled: false,
            storage_type: "none".to_string(),
            auto_save_interval: 0,
            backup_count: 0,
            total_size: 0,
            last_save: None,
            last_load: None,
        }
    }

    /// Check if auto-save is enabled
    pub fn auto_save_enabled(&self) -> bool {
        self.enabled && self.auto_save_interval > 0
    }

    /// Record a save operation
    pub fn record_save(&mut self) {
        self.last_save = Some(std::time::Instant::now());
    }

    /// Record a load operation
    pub fn record_load(&mut self) {
        self.last_load = Some(std::time::Instant::now());
    }

    /// Get time since last save
    pub fn time_since_save(&self) -> Option<std::time::Duration> {
        self.last_save.map(|time| time.elapsed())
    }

    /// Get time since last load
    pub fn time_since_load(&self) -> Option<std::time::Duration> {
        self.last_load.map(|time| time.elapsed())
    }
}

/// Fluent API for creating persistent machines
pub mod persistence_builder {
    use super::*;

    /// Builder for persistent machines
    pub struct PersistenceBuilder<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static> {
        machine: Option<Machine<C, E, C>>,
        config: PersistenceConfig,
        enable_auto_save: bool,
    }

    impl<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static> PersistenceBuilder<C, E> {
        /// Create a new persistence builder
        pub fn new() -> Self {
            Self {
                machine: None,
                config: PersistenceConfig::default(),
                enable_auto_save: false,
            }
        }

        /// Set the machine to make persistent
        pub fn machine(mut self, machine: Machine<C, E, C>) -> Self {
            self.machine = Some(machine);
            self
        }

        /// Set persistence configuration
        pub fn config(mut self, config: PersistenceConfig) -> Self {
            self.config = config;
            self
        }

        /// Use local storage
        pub fn local_storage(mut self) -> Self {
            self.config.storage_type = StorageType::LocalStorage;
            self
        }

        /// Use memory storage
        pub fn memory_storage(mut self) -> Self {
            self.config.storage_type = StorageType::Memory;
            self
        }

        /// Use file system storage
        pub fn filesystem_storage(mut self, base_dir: std::path::PathBuf) -> Self {
            self.config.storage_type = StorageType::FileSystem;
            self.config.custom_config.insert(
                "base_dir".to_string(),
                base_dir.to_string_lossy().to_string(),
            );
            self
        }

        /// Enable auto-save
        pub fn auto_save(mut self, interval_seconds: u64) -> Self {
            self.config.auto_save_interval = interval_seconds;
            self.enable_auto_save = true;
            self
        }

        /// Enable compression
        pub fn compress(mut self, level: u32) -> Self {
            self.config.compression_level = level;
            self
        }

        /// Enable backups
        pub fn backups(mut self, max_backups: usize) -> Self {
            self.config.max_backups = max_backups;
            self
        }

        /// Build the persistent machine
        pub fn build(self) -> Result<PersistentMachine<C, E>, PersistenceError> {
            let machine = self
                .machine
                .ok_or_else(|| PersistenceError::ConfigError("No machine provided".to_string()))?;

            let storage = persistence_storage::StorageFactory::create_storage_with_config(
                &self.config.storage_type,
                &self.config.custom_config,
            )?;

            let persistence = MachinePersistence::new(storage, self.config);
            let mut persistent_machine = PersistentMachine::new(machine, persistence);

            if self.enable_auto_save {
                persistent_machine = persistent_machine.with_auto_save();
            }

            Ok(persistent_machine)
        }
    }

    /// Create a persistence builder
    pub fn persistent_machine<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static>() -> PersistenceBuilder<C, E> {
        PersistenceBuilder::new()
    }

    /// Create a machine with local storage persistence
    pub fn with_local_storage<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static>(
        machine: Machine<C, E, C>,
    ) -> Result<PersistentMachine<C, E>, PersistenceError> {
        persistent_machine()
            .machine(machine)
            .local_storage()
            .build()
    }

    /// Create a machine with memory persistence (for testing)
    pub fn with_memory_storage<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static>(
        machine: Machine<C, E, C>,
    ) -> Result<PersistentMachine<C, E>, PersistenceError> {
        persistent_machine()
            .machine(machine)
            .memory_storage()
            .build()
    }

    /// Create a machine with file system persistence
    pub fn with_filesystem_storage<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static>(
        machine: Machine<C, E, C>,
        base_dir: std::path::PathBuf,
    ) -> Result<PersistentMachine<C, E>, PersistenceError> {
        persistent_machine()
            .machine(machine)
            .filesystem_storage(base_dir)
            .build()
    }
}

/// Migration support for schema changes
pub mod migrations {
    use super::*;

    /// Migration function type
    pub type MigrationFn<C> = Box<
        dyn Fn(SerializedMachine<C, (), C>) -> Result<SerializedMachine<C, (), C>, PersistenceError>
            + Send
            + Sync,
    >;

    /// Migration manager
    pub struct MigrationManager<C> {
        /// Current schema version
        pub current_version: u32,
        /// Migration functions
        pub migrations: std::collections::HashMap<u32, MigrationFn<C>>,
    }

    impl<C> MigrationManager<C> {
        /// Create a new migration manager
        pub fn new() -> Self {
            Self {
                current_version: 1,
                migrations: std::collections::HashMap::new(),
            }
        }

        /// Add a migration
        pub fn add_migration<F>(&mut self, from_version: u32, migration: F)
        where
            F: Fn(
                    SerializedMachine<C, (), C>,
                ) -> Result<SerializedMachine<C, (), C>, PersistenceError>
                + Send
                + Sync
                + 'static,
        {
            self.migrations.insert(from_version, Box::new(migration));
        }

        /// Apply migrations to serialized data
        pub fn apply_migrations(
            &self,
            mut data: SerializedMachine<C, (), C>,
        ) -> Result<SerializedMachine<C, (), C>, PersistenceError> {
            while data.version < self.current_version {
                if let Some(migration) = self.migrations.get(&data.version) {
                    data = migration(data)?;
                    data.version += 1;
                } else {
                    return Err(PersistenceError::VersionError {
                        current: data.version,
                        required: self.current_version,
                    });
                }
            }
            Ok(data)
        }

        /// Check if a version is supported
        pub fn supports_version(&self, version: u32) -> bool {
            version <= self.current_version
        }
    }

    /// Create a simple migration that updates the version
    pub fn version_migration<C>(new_version: u32) -> MigrationFn<C> {
        Box::new(move |mut data| {
            data.version = new_version;
            Ok(data)
        })
    }
}

/// Performance monitoring for persistence
pub mod monitoring {
    use super::*;

    /// Persistence metrics
    #[derive(Debug, Clone)]
    pub struct PersistenceMetrics {
        /// Total save operations
        pub saves_total: u64,
        /// Total load operations
        pub loads_total: u64,
        /// Total save time
        pub save_time_total: std::time::Duration,
        /// Total load time
        pub load_time_total: std::time::Duration,
        /// Save errors
        pub save_errors: u64,
        /// Load errors
        pub load_errors: u64,
        /// Cache hit rate
        pub cache_hit_rate: f64,
    }

    impl PersistenceMetrics {
        /// Create new metrics
        pub fn new() -> Self {
            Self {
                saves_total: 0,
                loads_total: 0,
                save_time_total: std::time::Duration::from_nanos(0),
                load_time_total: std::time::Duration::from_nanos(0),
                save_errors: 0,
                load_errors: 0,
                cache_hit_rate: 0.0,
            }
        }

        /// Record a save operation
        pub fn record_save(&mut self, duration: std::time::Duration, success: bool) {
            self.saves_total += 1;
            self.save_time_total += duration;
            if !success {
                self.save_errors += 1;
            }
        }

        /// Record a load operation
        pub fn record_load(&mut self, duration: std::time::Duration, success: bool) {
            self.loads_total += 1;
            self.load_time_total += duration;
            if !success {
                self.load_errors += 1;
            }
        }

        /// Get average save time
        pub fn avg_save_time(&self) -> std::time::Duration {
            if self.saves_total == 0 {
                std::time::Duration::from_nanos(0)
            } else {
                self.save_time_total / self.saves_total as u32
            }
        }

        /// Get average load time
        pub fn avg_load_time(&self) -> std::time::Duration {
            if self.loads_total == 0 {
                std::time::Duration::from_nanos(0)
            } else {
                self.load_time_total / self.loads_total as u32
            }
        }

        /// Get save success rate
        pub fn save_success_rate(&self) -> f64 {
            if self.saves_total == 0 {
                0.0
            } else {
                (self.saves_total - self.save_errors) as f64 / self.saves_total as f64
            }
        }

        /// Get load success rate
        pub fn load_success_rate(&self) -> f64 {
            if self.loads_total == 0 {
                0.0
            } else {
                (self.loads_total - self.load_errors) as f64 / self.loads_total as f64
            }
        }
    }

    /// Persistence monitor
    pub struct PersistenceMonitor {
        /// Metrics
        pub metrics: std::sync::RwLock<PersistenceMetrics>,
        /// Alert thresholds
        pub error_threshold: f64,
        /// Alert callback
        pub alert_callback: Option<Box<dyn Fn(&str) + Send + Sync>>,
    }

    impl PersistenceMonitor {
        /// Create a new monitor
        pub fn new() -> Self {
            Self {
                metrics: std::sync::RwLock::new(PersistenceMetrics::new()),
                error_threshold: 0.1, // 10% error rate
                alert_callback: None,
            }
        }

        /// Set alert callback
        pub fn with_alerts<F>(mut self, callback: F) -> Self
        where
            F: Fn(&str) + Send + Sync + 'static,
        {
            self.alert_callback = Some(Box::new(callback));
            self
        }

        /// Record metrics and check for alerts
        pub fn record_and_check(
            &self,
            operation: &str,
            success: bool,
            duration: std::time::Duration,
        ) {
            let mut metrics = self.metrics.write().unwrap();

            match operation {
                "save" => metrics.record_save(duration, success),
                "load" => metrics.record_load(duration, success),
                _ => {}
            }

            // Check for alerts
            if let Some(ref callback) = self.alert_callback {
                let error_rate = if operation == "save" {
                    1.0 - metrics.save_success_rate()
                } else {
                    1.0 - metrics.load_success_rate()
                };

                if error_rate > self.error_threshold {
                    callback(&format!(
                        "High error rate for {}: {:.2}%",
                        operation,
                        error_rate * 100.0
                    ));
                }
            }
        }

        /// Get current metrics
        pub fn get_metrics(&self) -> PersistenceMetrics {
            self.metrics.read().unwrap().clone()
        }
    }
}
