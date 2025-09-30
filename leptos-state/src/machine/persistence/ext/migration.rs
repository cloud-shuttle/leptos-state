//! Migration support for schema changes

use crate::machine::persistence_core::PersistenceError;

/// Migration support for schema changes
pub trait Migration {
    /// Migration version
    fn version(&self) -> u32;

    /// Migration description
    fn description(&self) -> &str;

    /// Check if migration can be applied
    fn can_apply(&self, current_version: u32) -> bool;

    /// Apply the migration
    fn apply(&self, data: &[u8]) -> Result<Vec<u8>, PersistenceError>;
}

/// Migration manager for handling schema changes
pub struct MigrationManager {
    migrations: Vec<Box<dyn Migration + Send + Sync>>,
}

impl MigrationManager {
    /// Create a new migration manager
    pub fn new() -> Self {
        Self {
            migrations: Vec::new(),
        }
    }

    /// Add a migration
    pub fn add_migration<M: Migration + Send + Sync + 'static>(mut self, migration: M) -> Self {
        self.migrations.push(Box::new(migration));
        self.migrations.sort_by_key(|m| m.version());
        self
    }

    /// Get migrations that can be applied to reach target version
    pub fn get_migrations_for_upgrade(&self, current_version: u32, target_version: u32) -> Vec<&Box<dyn Migration + Send + Sync>> {
        self.migrations
            .iter()
            .filter(|m| m.version() > current_version && m.version() <= target_version)
            .collect()
    }

    /// Apply migrations to data
    pub async fn apply_migrations(&self, mut data: Vec<u8>, current_version: u32, target_version: u32) -> Result<Vec<u8>, PersistenceError> {
        let migrations = self.get_migrations_for_upgrade(current_version, target_version);

        for migration in migrations {
            if migration.can_apply(current_version) {
                data = migration.apply(&data)?;
            }
        }

        Ok(data)
    }

    /// Get the latest migration version
    pub fn latest_version(&self) -> u32 {
        self.migrations
            .iter()
            .map(|m| m.version())
            .max()
            .unwrap_or(0)
    }

    /// Check if migrations are available for upgrade
    pub fn can_upgrade(&self, current_version: u32) -> bool {
        current_version < self.latest_version()
    }
}

impl Default for MigrationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple version bump migration
pub struct VersionMigration {
    from_version: u32,
    to_version: u32,
    description: String,
}

impl VersionMigration {
    /// Create a new version migration
    pub fn new(from_version: u32, to_version: u32, description: String) -> Self {
        Self {
            from_version,
            to_version,
            description,
        }
    }
}

impl Migration for VersionMigration {
    fn version(&self) -> u32 {
        self.to_version
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn can_apply(&self, current_version: u32) -> bool {
        current_version == self.from_version
    }

    fn apply(&self, data: &[u8]) -> Result<Vec<u8>, PersistenceError> {
        // For version migrations, we typically just pass through the data
        // Real migrations would transform the data format
        Ok(data.to_vec())
    }
}

/// Data transformation migration
pub struct TransformMigration<F> {
    version: u32,
    description: String,
    transformer: F,
}

impl<F> TransformMigration<F>
where
    F: Fn(&[u8]) -> Result<Vec<u8>, PersistenceError> + Send + Sync,
{
    /// Create a new transform migration
    pub fn new(version: u32, description: String, transformer: F) -> Self {
        Self {
            version,
            description,
            transformer,
        }
    }
}

impl<F> Migration for TransformMigration<F>
where
    F: Fn(&[u8]) -> Result<Vec<u8>, PersistenceError> + Send + Sync,
{
    fn version(&self) -> u32 {
        self.version
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn can_apply(&self, _current_version: u32) -> bool {
        true // Transform migrations can generally be applied
    }

    fn apply(&self, data: &[u8]) -> Result<Vec<u8>, PersistenceError> {
        (self.transformer)(data)
    }
}
