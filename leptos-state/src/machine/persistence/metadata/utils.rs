//! Metadata utilities and helper functions

use super::core::MachineMetadata;

/// Metadata utilities
pub struct MetadataUtils;

impl MetadataUtils {
    /// Validate metadata collection
    pub fn validate_collection(metadata_list: &[MachineMetadata]) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Check for duplicate names
        let mut names = std::collections::HashSet::new();
        for metadata in metadata_list {
            if let Some(name) = &metadata.name {
                if !names.insert(name.clone()) {
                    errors.push(format!("Duplicate machine name: {}", name));
                }
            }
        }

        // Validate individual metadata
        for (index, metadata) in metadata_list.iter().enumerate() {
            if let Err(err) = metadata.validate() {
                errors.push(format!("Machine {}: {}", index, err));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Find metadata by name
    pub fn find_by_name<'a>(metadata_list: &'a [MachineMetadata], name: &str) -> Option<&'a MachineMetadata> {
        metadata_list.iter().find(|m| m.name.as_deref() == Some(name))
    }

    /// Find metadata by tag
    pub fn find_by_tag<'a>(metadata_list: &'a [MachineMetadata], tag: &str) -> Vec<&'a MachineMetadata> {
        metadata_list.iter().filter(|m| m.has_tag(tag)).collect()
    }

    /// Find metadata by tags (must have all tags)
    pub fn find_by_tags<'a>(metadata_list: &'a [MachineMetadata], tags: &[String]) -> Vec<&'a MachineMetadata> {
        metadata_list.iter().filter(|m| m.has_tags(tags)).collect()
    }

    /// Filter metadata by age
    pub fn filter_by_age<'a>(metadata_list: &'a [MachineMetadata], max_age_seconds: u64) -> Vec<&'a MachineMetadata> {
        metadata_list.iter().filter(|m| m.age_seconds() <= max_age_seconds).collect()
    }

    /// Filter stale metadata
    pub fn filter_stale<'a>(metadata_list: &'a [MachineMetadata], threshold_seconds: u64) -> Vec<&'a MachineMetadata> {
        metadata_list.iter().filter(|m| !m.is_stale(threshold_seconds)).collect()
    }

    /// Sort metadata by name
    pub fn sort_by_name(metadata_list: &mut [MachineMetadata]) {
        metadata_list.sort_by(|a, b| {
            let a_name = a.display_name();
            let b_name = b.display_name();
            a_name.cmp(b_name)
        });
    }

    /// Sort metadata by creation time (newest first)
    pub fn sort_by_creation_time(metadata_list: &mut [MachineMetadata]) {
        metadata_list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    }

    /// Sort metadata by modification time (newest first)
    pub fn sort_by_modification_time(metadata_list: &mut [MachineMetadata]) {
        metadata_list.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
    }

    /// Group metadata by tag
    pub fn group_by_tag(metadata_list: &[MachineMetadata]) -> std::collections::HashMap<String, Vec<&MachineMetadata>> {
        let mut groups = std::collections::HashMap::new();

        for metadata in metadata_list {
            for tag in &metadata.tags {
                groups.entry(tag.clone()).or_insert_with(Vec::new).push(metadata);
            }
        }

        groups
    }

    /// Group metadata by author
    pub fn group_by_author(metadata_list: &[MachineMetadata]) -> std::collections::HashMap<String, Vec<&MachineMetadata>> {
        let mut groups = std::collections::HashMap::new();

        for metadata in metadata_list {
            if let Some(author) = &metadata.author {
                groups.entry(author.clone()).or_insert_with(Vec::new).push(metadata);
            } else {
                groups.entry("unknown".to_string()).or_insert_with(Vec::new).push(metadata);
            }
        }

        groups
    }

    /// Get statistics about metadata collection
    pub fn collection_stats(metadata_list: &[MachineMetadata]) -> CollectionStats {
        let mut stats = CollectionStats::default();
        let mut authors = std::collections::HashSet::new();
        let mut tags = std::collections::HashSet::new();
        let mut versions = std::collections::HashSet::new();

        for metadata in metadata_list {
            stats.total_machines += 1;

            if metadata.name.is_some() {
                stats.named_machines += 1;
            }

            if metadata.description.is_some() {
                stats.described_machines += 1;
            }

            if let Some(author) = &metadata.author {
                authors.insert(author.clone());
            }

            tags.extend(metadata.tags.iter().cloned());
            versions.insert(metadata.version.clone());

            stats.total_tags += metadata.tags.len();
            stats.total_properties += metadata.properties.len();
        }

        stats.unique_authors = authors.len();
        stats.unique_tags = tags.len();
        stats.unique_versions = versions.len();

        stats
    }

    /// Merge metadata collections (deduplicate by name)
    pub fn merge_collections(collections: Vec<Vec<MachineMetadata>>) -> Vec<MachineMetadata> {
        let mut merged = Vec::new();
        let mut seen_names = std::collections::HashSet::new();

        for collection in collections {
            for metadata in collection {
                if let Some(name) = &metadata.name {
                    if seen_names.insert(name.clone()) {
                        merged.push(metadata);
                    }
                } else {
                    // Include unnamed metadata
                    merged.push(metadata);
                }
            }
        }

        merged
    }

    /// Export metadata to JSON
    pub fn to_json(metadata: &MachineMetadata) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(metadata)
    }

    /// Import metadata from JSON
    pub fn from_json(json: &str) -> Result<MachineMetadata, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Export collection to JSON array
    pub fn collection_to_json(metadata_list: &[MachineMetadata]) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(metadata_list)
    }

    /// Import collection from JSON array
    pub fn collection_from_json(json: &str) -> Result<Vec<MachineMetadata>, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Create a diff between two metadata instances
    pub fn diff_metadata(old: &MachineMetadata, new: &MachineMetadata) -> MetadataDiff {
        let mut changes = Vec::new();

        if old.name != new.name {
            changes.push(MetadataChange::Name(old.name.clone(), new.name.clone()));
        }

        if old.description != new.description {
            changes.push(MetadataChange::Description(old.description.clone(), new.description.clone()));
        }

        if old.version != new.version {
            changes.push(MetadataChange::Version(old.version.clone(), new.version.clone()));
        }

        if old.author != new.author {
            changes.push(MetadataChange::Author(old.author.clone(), new.author.clone()));
        }

        // Check for added/removed tags
        let old_tags: std::collections::HashSet<_> = old.tags.iter().collect();
        let new_tags: std::collections::HashSet<_> = new.tags.iter().collect();

        for &tag in &new_tags {
            if !old_tags.contains(tag) {
                changes.push(MetadataChange::TagAdded(tag.clone()));
            }
        }

        for &tag in &old_tags {
            if !new_tags.contains(tag) {
                changes.push(MetadataChange::TagRemoved(tag.clone()));
            }
        }

        // Check for property changes
        for (key, old_value) in &old.properties {
            if let Some(new_value) = new.properties.get(key) {
                if old_value != new_value {
                    changes.push(MetadataChange::PropertyChanged(key.clone(), old_value.clone(), new_value.clone()));
                }
            } else {
                changes.push(MetadataChange::PropertyRemoved(key.clone(), old_value.clone()));
            }
        }

        for (key, new_value) in &new.properties {
            if !old.properties.contains_key(key) {
                changes.push(MetadataChange::PropertyAdded(key.clone(), new_value.clone()));
            }
        }

        MetadataDiff { changes }
    }
}

/// Statistics about a metadata collection
#[derive(Debug, Clone, Default)]
pub struct CollectionStats {
    /// Total number of machines
    pub total_machines: usize,
    /// Number of machines with names
    pub named_machines: usize,
    /// Number of machines with descriptions
    pub described_machines: usize,
    /// Number of unique authors
    pub unique_authors: usize,
    /// Number of unique tags
    pub unique_tags: usize,
    /// Number of unique versions
    pub unique_versions: usize,
    /// Total number of tags across all machines
    pub total_tags: usize,
    /// Total number of properties across all machines
    pub total_properties: usize,
}

impl CollectionStats {
    /// Get naming coverage (percentage of machines with names)
    pub fn naming_coverage(&self) -> f64 {
        if self.total_machines == 0 {
            0.0
        } else {
            self.named_machines as f64 / self.total_machines as f64
        }
    }

    /// Get description coverage (percentage of machines with descriptions)
    pub fn description_coverage(&self) -> f64 {
        if self.total_machines == 0 {
            0.0
        } else {
            self.described_machines as f64 / self.total_machines as f64
        }
    }

    /// Get average tags per machine
    pub fn avg_tags_per_machine(&self) -> f64 {
        if self.total_machines == 0 {
            0.0
        } else {
            self.total_tags as f64 / self.total_machines as f64
        }
    }

    /// Get average properties per machine
    pub fn avg_properties_per_machine(&self) -> f64 {
        if self.total_machines == 0 {
            0.0
        } else {
            self.total_properties as f64 / self.total_machines as f64
        }
    }

    /// Get summary
    pub fn summary(&self) -> String {
        format!(
            "CollectionStats {{ machines: {}, named: {:.1}%, described: {:.1}%, tags: {} ({:.1} avg), authors: {} }}",
            self.total_machines,
            self.naming_coverage() * 100.0,
            self.description_coverage() * 100.0,
            self.total_tags,
            self.avg_tags_per_machine(),
            self.unique_authors
        )
    }
}

impl std::fmt::Display for CollectionStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Difference between two metadata instances
#[derive(Debug, Clone)]
pub struct MetadataDiff {
    /// List of changes
    pub changes: Vec<MetadataChange>,
}

impl MetadataDiff {
    /// Check if there are any changes
    pub fn has_changes(&self) -> bool {
        !self.changes.is_empty()
    }

    /// Get number of changes
    pub fn change_count(&self) -> usize {
        self.changes.len()
    }

    /// Check if changes are significant (not just timestamps)
    pub fn has_significant_changes(&self) -> bool {
        self.changes.iter().any(|change| !matches!(change, MetadataChange::ModifiedAt(_, _)))
    }
}

/// Individual metadata change
#[derive(Debug, Clone)]
pub enum MetadataChange {
    /// Name changed
    Name(Option<String>, Option<String>),
    /// Description changed
    Description(Option<String>, Option<String>),
    /// Version changed
    Version(String, String),
    /// Author changed
    Author(Option<String>, Option<String>),
    /// Tag added
    TagAdded(String),
    /// Tag removed
    TagRemoved(String),
    /// Property added
    PropertyAdded(String, serde_json::Value),
    /// Property removed
    PropertyRemoved(String, serde_json::Value),
    /// Property changed
    PropertyChanged(String, serde_json::Value, serde_json::Value),
    /// Modified timestamp changed
    ModifiedAt(u64, u64),
}

impl std::fmt::Display for MetadataChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Name(old, new) => write!(f, "Name: {:?} → {:?}", old, new),
            Self::Description(old, new) => write!(f, "Description: {:?} → {:?}", old, new),
            Self::Version(old, new) => write!(f, "Version: {} → {}", old, new),
            Self::Author(old, new) => write!(f, "Author: {:?} → {:?}", old, new),
            Self::TagAdded(tag) => write!(f, "Tag added: {}", tag),
            Self::TagRemoved(tag) => write!(f, "Tag removed: {}", tag),
            Self::PropertyAdded(key, _) => write!(f, "Property added: {}", key),
            Self::PropertyRemoved(key, _) => write!(f, "Property removed: {}", key),
            Self::PropertyChanged(key, _, _) => write!(f, "Property changed: {}", key),
            Self::ModifiedAt(old, new) => write!(f, "Modified: {} → {}", old, new),
        }
    }
}
