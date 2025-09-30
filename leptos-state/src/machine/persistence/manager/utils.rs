//! Persistence utilities and helper functions

use super::info::MachineInfo;
use super::backup::BackupEntry;

/// Persistence utilities
pub struct PersistenceUtils;

impl PersistenceUtils {
    /// Validate machine ID format
    pub fn validate_machine_id(id: &str) -> Result<(), String> {
        if id.trim().is_empty() {
            return Err("Machine ID cannot be empty".to_string());
        }

        if id.len() > 255 {
            return Err("Machine ID too long (max 255 characters)".to_string());
        }

        // Check for invalid characters
        let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
        for &ch in &invalid_chars {
            if id.contains(ch) {
                return Err(format!("Machine ID contains invalid character: {}", ch));
            }
        }

        Ok(())
    }

    /// Sanitize machine ID for filesystem use
    pub fn sanitize_machine_id(id: &str) -> String {
        id.chars()
            .map(|c| match c {
                '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
                c if c.is_control() => '_',
                c => c,
            })
            .collect()
    }

    /// Generate a unique machine ID
    pub fn generate_machine_id(prefix: Option<&str>) -> String {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();

        let random_part = (rand::random::<u32>() % 10000).to_string();

        match prefix {
            Some(p) => format!("{}_{}_{}", p, timestamp, random_part),
            None => format!("machine_{}_{}", timestamp, random_part),
        }
    }

    /// Calculate data checksum
    pub fn calculate_checksum(data: &[u8]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Compress data (simplified implementation)
    pub fn compress_data(data: &[u8]) -> Result<Vec<u8>, String> {
        // In a real implementation, this would use a compression library
        // For now, just return the data as-is
        Ok(data.to_vec())
    }

    /// Decompress data (simplified implementation)
    pub fn decompress_data(data: &[u8]) -> Result<Vec<u8>, String> {
        // In a real implementation, this would use a compression library
        // For now, just return the data as-is
        Ok(data.to_vec())
    }

    /// Validate backup data integrity
    pub fn validate_backup_integrity(entry: &BackupEntry, expected_checksum: Option<&str>) -> Result<(), String> {
        if let Some(expected) = expected_checksum {
            let actual = Self::calculate_checksum(&entry.data);
            if actual != expected {
                return Err(format!("Backup checksum mismatch: expected {}, got {}", expected, actual));
            }
        }

        if entry.data.is_empty() {
            return Err("Backup data is empty".to_string());
        }

        Ok(())
    }

    /// Format duration in human readable form
    pub fn format_duration(seconds: u64) -> String {
        if seconds < 60 {
            format!("{}s", seconds)
        } else if seconds < 3600 {
            format!("{}m {}s", seconds / 60, seconds % 60)
        } else if seconds < 86400 {
            format!("{}h {}m", seconds / 3600, (seconds % 3600) / 60)
        } else {
            format!("{}d {}h", seconds / 86400, (seconds % 86400) / 3600)
        }
    }

    /// Parse size string (e.g., "10MB", "500KB")
    pub fn parse_size(size_str: &str) -> Result<u64, String> {
        let size_str = size_str.trim();

        if let Some(bytes) = size_str.strip_suffix("B") {
            bytes.parse::<u64>().map_err(|_| format!("Invalid byte size: {}", bytes))
        } else if let Some(kb) = size_str.strip_suffix("KB") {
            kb.parse::<u64>()
                .map(|n| n * 1024)
                .map_err(|_| format!("Invalid KB size: {}", kb))
        } else if let Some(mb) = size_str.strip_suffix("MB") {
            mb.parse::<u64>()
                .map(|n| n * 1024 * 1024)
                .map_err(|_| format!("Invalid MB size: {}", mb))
        } else if let Some(gb) = size_str.strip_suffix("GB") {
            gb.parse::<u64>()
                .map(|n| n * 1024 * 1024 * 1024)
                .map_err(|_| format!("Invalid GB size: {}", gb))
        } else {
            // Default to bytes
            size_str.parse::<u64>().map_err(|_| format!("Invalid size: {}", size_str))
        }
    }

    /// Get file extension for storage type
    pub fn file_extension_for_storage_type(storage_type: &crate::machine::persistence_core::StorageType) -> &'static str {
        match storage_type {
            crate::machine::persistence_core::StorageType::Memory => "mem",
            crate::machine::persistence_core::StorageType::FileSystem => "json",
            crate::machine::persistence_core::StorageType::LocalStorage => "local",
            crate::machine::persistence_core::StorageType::SessionStorage => "session",
            crate::machine::persistence_core::StorageType::IndexedDB => "idb",
            crate::machine::persistence_core::StorageType::WebSQL => "websql",
            crate::machine::persistence_core::StorageType::Custom(_) => "custom",
        }
    }

    /// Create backup filename
    pub fn create_backup_filename(machine_id: &str, backup_id: &str, storage_type: &crate::machine::persistence_core::StorageType) -> String {
        let ext = Self::file_extension_for_storage_type(storage_type);
        format!("{}_{}.{}", machine_id, backup_id, ext)
    }

    /// Extract machine ID from filename
    pub fn extract_machine_id_from_filename(filename: &str) -> Option<&str> {
        filename.split('_').next()
    }

    /// Extract backup ID from filename
    pub fn extract_backup_id_from_filename(filename: &str) -> Option<&str> {
        let parts: Vec<&str> = filename.split('_').collect();
        if parts.len() >= 2 {
            parts.get(1).and_then(|s| s.split('.').next())
        } else {
            None
        }
    }

    /// Group machine infos by age
    pub fn group_machine_infos_by_age(machine_infos: &[MachineInfo]) -> std::collections::HashMap<&'static str, Vec<&MachineInfo>> {
        let mut groups = std::collections::HashMap::new();

        for info in machine_infos {
            let age_seconds = info.age_seconds();
            let category = if age_seconds < 3600 {
                "recent" // < 1 hour
            } else if age_seconds < 86400 {
                "today" // < 1 day
            } else if age_seconds < 604800 {
                "week" // < 1 week
            } else if age_seconds < 2592000 {
                "month" // < 1 month
            } else {
                "old" // > 1 month
            };

            groups.entry(category).or_insert_with(Vec::new).push(info);
        }

        groups
    }

    /// Sort machine infos by size
    pub fn sort_machine_infos_by_size(machine_infos: &mut [MachineInfo]) {
        machine_infos.sort_by_key(|info| info.size_bytes);
    }

    /// Sort machine infos by age
    pub fn sort_machine_infos_by_age(machine_infos: &mut [MachineInfo]) {
        machine_infos.sort_by_key(|info| info.created_at);
    }

    /// Calculate total size of machine infos
    pub fn calculate_total_size(machine_infos: &[MachineInfo]) -> u64 {
        machine_infos.iter().map(|info| info.size_bytes).sum()
    }

    /// Find largest machines
    pub fn find_largest_machines(machine_infos: &[MachineInfo], count: usize) -> Vec<&MachineInfo> {
        let mut sorted: Vec<&MachineInfo> = machine_infos.iter().collect();
        sorted.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));
        sorted.into_iter().take(count).collect()
    }

    /// Find oldest machines
    pub fn find_oldest_machines(machine_infos: &[MachineInfo], count: usize) -> Vec<&MachineInfo> {
        let mut sorted: Vec<&MachineInfo> = machine_infos.iter().collect();
        sorted.sort_by_key(|info| info.created_at);
        sorted.into_iter().take(count).collect()
    }

    /// Generate storage report
    pub fn generate_storage_report(machine_infos: &[MachineInfo]) -> String {
        let total_machines = machine_infos.len();
        let total_size = Self::calculate_total_size(machine_infos);
        let avg_size = if total_machines > 0 {
            total_size / total_machines as u64
        } else {
            0
        };

        let age_groups = Self::group_machine_infos_by_age(machine_infos);

        let mut report = format!("Storage Report\n");
        report.push_str(&format!("={}\n", "=".repeat(20)));
        report.push_str(&format!("Total Machines: {}\n", total_machines));
        report.push_str(&format!("Total Size: {:.1} MB\n", total_size as f64 / (1024.0 * 1024.0)));
        report.push_str(&format!("Average Size: {:.1} KB\n", avg_size as f64 / 1024.0));

        report.push_str("\nAge Distribution:\n");
        for (category, machines) in &age_groups {
            report.push_str(&format!("  {}: {}\n", category, machines.len()));
        }

        if total_machines > 0 {
            let largest = Self::find_largest_machines(machine_infos, std::cmp::min(5, total_machines));
            report.push_str("\nLargest Machines:\n");
            for machine in largest {
                report.push_str(&format!("  {}: {:.1} KB\n", machine.display_name(), machine.size_bytes as f64 / 1024.0));
            }
        }

        report
    }

    /// Validate storage path
    pub fn validate_storage_path(path: &std::path::Path) -> Result<(), String> {
        if !path.exists() {
            return Err(format!("Storage path does not exist: {}", path.display()));
        }

        if !path.is_dir() {
            return Err(format!("Storage path is not a directory: {}", path.display()));
        }

        // Check if writable
        match std::fs::File::create(path.join(".test")) {
            Ok(file) => {
                drop(file);
                let _ = std::fs::remove_file(path.join(".test"));
                Ok(())
            }
            Err(e) => Err(format!("Storage path is not writable: {}", e)),
        }
    }

    /// Create directory if it doesn't exist
    pub fn ensure_directory_exists(path: &std::path::Path) -> Result<(), String> {
        if !path.exists() {
            std::fs::create_dir_all(path)
                .map_err(|e| format!("Failed to create directory {}: {}", path.display(), e))?;
        }
        Ok(())
    }

    /// Clean filename for filesystem use
    pub fn clean_filename(filename: &str) -> String {
        filename
            .chars()
            .map(|c| match c {
                '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0'..='\x1f' => '_',
                c => c,
            })
            .collect()
    }
}
