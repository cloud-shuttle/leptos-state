//! Backup manager and backup entry structures

use crate::machine::persistence_core::{BackupConfig, PersistenceError, StorageType};
use std::path::PathBuf;

/// Backup manager
#[derive(Debug, Clone)]
pub struct BackupManager {
    /// Storage type
    storage_type: StorageType,
    /// Backup configuration
    config: BackupConfig,
    /// Backup entries
    backups: std::sync::RwLock<Vec<BackupEntry>>,
    /// Last backup time
    last_backup_time: std::sync::RwLock<Option<u64>>,
}

impl BackupManager {
    /// Create a new backup manager
    pub fn new(storage_type: StorageType, config: BackupConfig) -> Self {
        Self {
            storage_type,
            config,
            backups: std::sync::RwLock::new(Vec::new()),
            last_backup_time: std::sync::RwLock::new(None),
        }
    }

    /// Create a backup
    pub async fn create_backup(&self, machine_id: &str, data: &[u8]) -> Result<String, PersistenceError> {
        let backup_id = format!("{}_{}", machine_id, generate_backup_id());
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let entry = BackupEntry {
            id: backup_id.clone(),
            machine_id: machine_id.to_string(),
            timestamp,
            size_bytes: data.len() as u64,
            storage_type: self.storage_type.clone(),
            data: data.to_vec(),
        };

        // Store backup based on configuration
        if let Some(ref path) = self.config.backup_path {
            let path_buf = std::path::PathBuf::from(path);
            self.store_local_backup(&entry, &path_buf).await?;
        } else {
            return Err(PersistenceError::ConfigError("Backup path not configured".to_string()));
        }

        // Add to in-memory list
        self.backups.write().unwrap().push(entry);
        *self.last_backup_time.write().unwrap() = Some(timestamp);

        // Cleanup old backups if needed
        if self.config.max_backups > 0 {
            self.cleanup_old_backups_for_machine(machine_id).await?;
        }

        Ok(backup_id)
    }

    /// Restore a backup
    pub async fn restore_backup(&self, backup_id: &str) -> Result<Vec<u8>, PersistenceError> {
        let backups = self.backups.read().unwrap();
        let entry = backups.iter()
            .find(|b| b.id == backup_id)
            .ok_or_else(|| PersistenceError::NotFound(format!("Backup {} not found", backup_id)))?;

        if let Some(ref path) = self.config.backup_path {
            let path_buf = std::path::PathBuf::from(path);
            self.restore_local_backup(entry, &path_buf).await
        } else {
            Err(PersistenceError::ConfigError("Backup path not configured".to_string()))
        }
    }

    /// List all backups
    pub fn list_backups(&self) -> Vec<BackupEntry> {
        self.backups.read().unwrap().clone()
    }

    /// List backups for a specific machine
    pub fn list_backups_for_machine(&self, machine_id: &str) -> Vec<BackupEntry> {
        self.backups.read().unwrap()
            .iter()
            .filter(|b| b.machine_id == machine_id)
            .cloned()
            .collect()
    }

    /// Delete a backup
    pub async fn delete_backup(&self, backup_id: &str) -> Result<(), PersistenceError> {
        let mut backups = self.backups.write().unwrap();
        let index = backups.iter().position(|b| b.id == backup_id)
            .ok_or_else(|| PersistenceError::NotFound(format!("Backup {} not found", backup_id)))?;

        let entry = backups.remove(index);

        // Delete from storage
        if let Some(path) = Self::get_backup_path(&self.config) {
            self.delete_local_backup(&entry, &path).await?;
        } else {
            return Err(PersistenceError::ConfigError("Backup path not configured".to_string()));
        }

        Ok(())
    }

    /// Clean up old backups
    pub async fn cleanup_old_backups(&self) -> Result<(), PersistenceError> {
        if self.config.max_backups == 0 {
            return Ok(());
        }

        let mut backups = self.backups.write().unwrap();
        let mut machines: std::collections::HashMap<String, Vec<usize>> = std::collections::HashMap::new();

        // Group backups by machine
        for (index, backup) in backups.iter().enumerate() {
            machines.entry(backup.machine_id.clone()).or_default().push(index);
        }

        // Clean up each machine's backups
        for indices in machines.values() {
            if indices.len() > self.config.max_backups as usize {
                let mut indices_to_remove: Vec<usize> = indices.clone();
                // Sort by timestamp (oldest first)
                indices_to_remove.sort_by_key(|&i| backups[i].timestamp);

                // Remove oldest backups beyond the limit
                let to_remove = indices_to_remove.len() - self.config.max_backups as usize;
                for &index in &indices_to_remove[..to_remove] {
                    let entry = &backups[index];

                    // Delete from storage
                    if let Some(path) = Self::get_backup_path(&self.config) {
                        let _ = self.delete_local_backup(entry, &path).await;
                    }
                }

                // Remove from in-memory list (in reverse order to maintain indices)
                for &index in indices_to_remove[..to_remove].iter().rev() {
                    backups.remove(index);
                }
            }
        }

        Ok(())
    }

    /// Clean up old backups for a specific machine
    async fn cleanup_old_backups_for_machine(&self, machine_id: &str) -> Result<(), PersistenceError> {
        if self.config.max_backups == 0 {
            return Ok(());
        }

        let mut backups = self.backups.write().unwrap();
        let machine_backups: Vec<usize> = backups.iter()
            .enumerate()
            .filter(|(_, b)| b.machine_id == machine_id)
            .map(|(i, _)| i)
            .collect();

        if machine_backups.len() > self.config.max_backups as usize {
            let to_remove = machine_backups.len() - self.config.max_backups as usize;

            // Sort by timestamp (oldest first)
            let mut to_remove_indices: Vec<usize> = machine_backups.clone();
            to_remove_indices.sort_by_key(|&i| backups[i].timestamp);

            // Remove oldest backups
            for &index in &to_remove_indices[..to_remove] {
                let entry = &backups[index];

                // Delete from storage
                if let Some(path) = Self::get_backup_path(&self.config) {
                    self.delete_local_backup(entry, &path).await?;
                }

                // Remove from in-memory list
                if let Some(pos) = backups.iter().position(|b| b.id == entry.id) {
                    backups.remove(pos);
                }
            }
        }

        Ok(())
    }

    /// Store backup locally
    async fn store_local_backup(&self, entry: &BackupEntry, base_path: &PathBuf) -> Result<(), PersistenceError> {
        let backup_path = base_path.join(format!("{}.backup", entry.id));
        tokio::fs::create_dir_all(base_path).await
            .map_err(|e| PersistenceError::IoError(e))?;
        tokio::fs::write(&backup_path, &entry.data).await
            .map_err(|e| PersistenceError::IoError(e))?;
        Ok(())
    }

    /// Restore backup from local storage
    async fn restore_local_backup(&self, entry: &BackupEntry, base_path: &PathBuf) -> Result<Vec<u8>, PersistenceError> {
        let backup_path = base_path.join(format!("{}.backup", entry.id));
        tokio::fs::read(&backup_path).await
            .map_err(|e| PersistenceError::IoError(e))
    }

    /// Delete backup from local storage
    async fn delete_local_backup(&self, entry: &BackupEntry, base_path: &PathBuf) -> Result<(), PersistenceError> {
        let backup_path = base_path.join(format!("{}.backup", entry.id));
        tokio::fs::remove_file(&backup_path).await
            .map_err(|e| PersistenceError::IoError(e))
    }

    /// Get backup count
    pub fn backup_count(&self) -> usize {
        self.backups.read().unwrap().len()
    }

    /// Get last backup time
    pub fn last_backup_time(&self) -> Option<u64> {
        *self.last_backup_time.read().unwrap()
    }

    /// Get configuration
    pub fn config(&self) -> &BackupConfig {
        &self.config
    }

    /// Get backup path as PathBuf
    fn get_backup_path(config: &BackupConfig) -> Option<std::path::PathBuf> {
        config.backup_path.as_ref().map(std::path::PathBuf::from)
    }
}

/// Backup entry information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BackupEntry {
    /// Backup ID
    pub id: String,
    /// Machine ID
    pub machine_id: String,
    /// Creation timestamp
    pub timestamp: u64,
    /// Size in bytes
    pub size_bytes: u64,
    /// Storage type used
    pub storage_type: StorageType,
    /// Backup data (in memory for simplicity)
    #[serde(skip)]
    pub data: Vec<u8>,
}

impl BackupEntry {
    /// Create a new backup entry
    pub fn new(id: String, machine_id: String, data: Vec<u8>, storage_type: StorageType) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id,
            machine_id,
            timestamp,
            size_bytes: data.len() as u64,
            storage_type,
            data,
        }
    }

    /// Get age in seconds
    pub fn age_seconds(&self) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now.saturating_sub(self.timestamp)
    }

    /// Check if backup is older than threshold
    pub fn is_older_than(&self, seconds: u64) -> bool {
        self.age_seconds() > seconds
    }

    /// Get formatted size
    pub fn formatted_size(&self) -> String {
        format_bytes(self.size_bytes)
    }

    /// Get formatted age
    pub fn formatted_age(&self) -> String {
        let age = self.age_seconds();
        if age < 60 {
            format!("{}s ago", age)
        } else if age < 3600 {
            format!("{}m ago", age / 60)
        } else if age < 86400 {
            format!("{}h ago", age / 3600)
        } else {
            format!("{}d ago", age / 86400)
        }
    }

    /// Get summary
    pub fn summary(&self) -> String {
        format!(
            "Backup '{}' for machine '{}' ({}), created {}",
            self.id, self.machine_id, self.formatted_size(), self.formatted_age()
        )
    }
}

impl std::fmt::Display for BackupEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

impl std::hash::Hash for BackupEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for BackupEntry {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for BackupEntry {}

impl PartialOrd for BackupEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BackupEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

/// Generate a unique backup ID
fn generate_backup_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    format!("backup_{}", timestamp)
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
