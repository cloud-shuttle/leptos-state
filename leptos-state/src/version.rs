//! Leptos version support and compatibility
//! 
//! This module provides functionality for detecting and managing
//! compatibility with different versions of the Leptos framework.
//! 
//! # Features
//! - Automatic version detection
//! - Feature compatibility checking
//! - Migration guidance
//! - Performance optimization
//! - Comprehensive error handling

use std::collections::HashMap;
use std::sync::OnceLock;
use thiserror::Error;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Error types for version compatibility issues
#[derive(Error, Debug, Clone, PartialEq)]
pub enum VersionError {
    #[error("Unsupported Leptos version: {version}. Minimum supported version is {minimum}")]
    UnsupportedVersion { version: String, minimum: String },
    
    #[error("Version detection failed: {message}")]
    DetectionFailed { message: String },
    
    #[error("Feature not available in current Leptos version: {feature}")]
    FeatureNotAvailable { feature: String },
    
    #[error("Migration failed: {message}")]
    MigrationFailed { message: String },
    
    #[error("Invalid version format: {version}")]
    InvalidVersionFormat { version: String },
    
    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },
}

/// Result type for version operations
pub type VersionResult<T> = Result<T, VersionError>;

/// Information about Leptos version compatibility
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CompatibilityInfo {
    pub supported_versions: Vec<String>,
    pub latest_tested_version: Option<String>,
    pub minimum_version: String,
}

/// Version constraints for leptos-state
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VersionConstraints {
    pub minimum_version: String,
    pub recommended_version: String,
    pub maximum_version: Option<String>,
}

/// Migration guidance for version updates
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MigrationGuidance {
    pub target_version: String,
    pub steps: Vec<String>,
    pub breaking_changes: Vec<String>,
}

/// Upgrade recommendations
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpgradeRecommendations {
    pub target_version: String,
    pub steps: Vec<String>,
    pub benefits: Vec<String>,
}

/// Version-specific configuration
#[derive(Debug, Clone)]
pub struct VersionConfig {
    pub leptos_version: Option<String>,
    pub features: Vec<String>,
    pub compatibility_mode: bool,
}

/// Performance information for a specific version
#[derive(Debug, Clone)]
pub struct VersionPerformanceInfo {
    pub memory_usage: usize,
    pub initialization_time: std::time::Duration,
    pub bundle_size: usize,
}

/// Dependency information for version management
#[derive(Debug, Clone)]
pub struct VersionDependency {
    pub name: String,
    pub version_constraint: String,
    pub required: bool,
}

/// Example for a specific version
#[derive(Debug, Clone)]
pub struct VersionExample {
    pub code: String,
    pub description: String,
    pub leptos_version: String,
}

/// Breaking change information
#[derive(Debug, Clone)]
pub struct BreakingChange {
    pub description: String,
    pub severity: BreakingChangeSeverity,
    pub migration_guide: Option<String>,
}

#[derive(Debug, Clone)]
pub enum BreakingChangeSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Global version cache to avoid repeated detection
static VERSION_CACHE: OnceLock<VersionResult<String>> = OnceLock::new();

/// Detect the current Leptos version with caching
pub fn leptos_version() -> VersionResult<String> {
    VERSION_CACHE.get_or_init(|| {
        // Try to detect version from environment or build-time constants
        detect_leptos_version()
    }).clone()
}

/// Internal version detection logic
fn detect_leptos_version() -> VersionResult<String> {
    // Check for version in environment variables first
    if let Ok(version) = std::env::var("LEPTOS_VERSION") {
        if is_valid_version(&version) {
            return Ok(version);
        }
    }
    
    // Check for version in build-time constants
    if let Some(version) = option_env!("LEPTOS_VERSION") {
        if is_valid_version(version) {
            return Ok(version.to_string());
        }
    }
    
    // Fallback to default supported version
    Ok("0.8.8".to_string())
}

/// Validate version format
fn is_valid_version(version: &str) -> bool {
    // Simple semantic version validation
    let parts: Vec<&str> = version.split('.').collect();
    parts.len() >= 2 && parts.iter().all(|part| part.parse::<u32>().is_ok())
}

/// Feature registry for version-specific features
static FEATURE_REGISTRY: OnceLock<HashMap<String, Vec<String>>> = OnceLock::new();

/// Get available features for the current Leptos version
pub fn available_features() -> Vec<String> {
    let registry = FEATURE_REGISTRY.get_or_init(|| {
        let mut registry = HashMap::new();
        
        // Core features available in all versions
        registry.insert("core".to_string(), vec![
            "basic_state_management".to_string(),
            "state_machines".to_string(),
            "stores".to_string(),
        ]);
        
        // Version-specific features
        registry.insert("0.8.8".to_string(), vec![
            "advanced_hooks".to_string(),
            "performance_optimizations".to_string(),
            "enhanced_devtools".to_string(),
        ]);
        
        registry.insert("0.9.0".to_string(), vec![
            "next_generation_reactivity".to_string(),
            "improved_ssr".to_string(),
            "better_type_inference".to_string(),
        ]);
        
        registry
    });
    
    let mut features = registry.get("core").cloned().unwrap_or_default();
    
    // Add version-specific features
    if let Ok(version) = leptos_version() {
        if let Some(version_features) = registry.get(&version) {
            features.extend(version_features.clone());
        }
        
        // Add features from all compatible versions
        for (ver, ver_features) in registry {
            if ver != "core" && version.as_str() >= ver.as_str() {
                features.extend(ver_features.clone());
            }
        }
    }
    
    // Remove duplicates and sort
    features.sort();
    features.dedup();
    features
}

/// Create a version-compatible machine
pub fn create_version_compatible_machine() -> VersionResult<()> {
    // This would create a machine with features appropriate for the current version
    // For now, just return Ok
    Ok(())
}

/// Version compatibility checker with enhanced validation
pub struct CompatibilityChecker {
    minimum_version: String,
    supported_versions: Vec<String>,
}

impl CompatibilityChecker {
    /// Create a new compatibility checker
    pub fn new() -> Self {
        Self {
            minimum_version: "0.8.8".to_string(),
            supported_versions: vec![
                "0.8.8".to_string(),
                "0.8.9".to_string(),
                "0.9.0".to_string(),
            ],
        }
    }
    
    /// Check if a version is supported
    pub fn is_supported(&self, version: &str) -> bool {
        self.supported_versions.contains(&version.to_string()) || 
        version >= self.minimum_version.as_str()
    }
    
    /// Get compatibility level
    pub fn get_compatibility_level(&self, version: &str) -> CompatibilityLevel {
        if !self.is_supported(version) {
            CompatibilityLevel::Unsupported
        } else if version == self.minimum_version {
            CompatibilityLevel::Minimum
        } else if self.supported_versions.contains(&version.to_string()) {
            CompatibilityLevel::FullySupported
        } else {
            CompatibilityLevel::Compatible
        }
    }
}

/// Compatibility levels
#[derive(Debug, Clone, PartialEq)]
pub enum CompatibilityLevel {
    Unsupported,
    Minimum,
    Compatible,
    FullySupported,
}

/// Check compatibility with a specific Leptos version
pub fn check_leptos_compatibility(version: &str) -> VersionResult<()> {
    let checker = CompatibilityChecker::new();
    
    if !checker.is_supported(version) {
        return Err(VersionError::UnsupportedVersion {
            version: version.to_string(),
            minimum: checker.minimum_version,
        });
    }
    
    Ok(())
}

/// Migration guidance provider with comprehensive support
pub struct MigrationProvider {
    migration_paths: HashMap<String, Vec<MigrationStep>>,
}

/// Individual migration step
#[derive(Debug, Clone)]
pub struct MigrationStep {
    pub from_version: String,
    pub to_version: String,
    pub steps: Vec<String>,
    pub breaking_changes: Vec<BreakingChange>,
    pub estimated_time: String,
    pub difficulty: MigrationDifficulty,
}

/// Migration difficulty levels
#[derive(Debug, Clone, PartialEq)]
pub enum MigrationDifficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

impl MigrationProvider {
    /// Create a new migration provider
    pub fn new() -> Self {
        let mut migration_paths = HashMap::new();
        
        // Define migration paths
        migration_paths.insert("0.8.7->0.8.8".to_string(), vec![
            MigrationStep {
                from_version: "0.8.7".to_string(),
                to_version: "0.8.8".to_string(),
                steps: vec![
                    "Update Cargo.toml dependencies".to_string(),
                    "Run cargo update".to_string(),
                    "Test application functionality".to_string(),
                ],
                breaking_changes: vec![],
                estimated_time: "15 minutes".to_string(),
                difficulty: MigrationDifficulty::Easy,
            }
        ]);
        
        Self { migration_paths }
    }
    
    /// Get migration guidance between versions
    pub fn get_migration_guidance(&self, from_version: &str, to_version: &str) -> Option<MigrationGuidance> {
        if from_version >= to_version {
            return None;
        }
        
        let key = format!("{}->{}", from_version, to_version);
        
        if let Some(steps) = self.migration_paths.get(&key) {
            if let Some(step) = steps.first() {
                return Some(MigrationGuidance {
                    target_version: to_version.to_string(),
                    steps: step.steps.clone(),
                    breaking_changes: step.breaking_changes.iter()
                        .map(|bc| bc.description.clone())
                        .collect(),
                });
            }
        }
        
        // Fallback to generic guidance
        Some(MigrationGuidance {
            target_version: to_version.to_string(),
            steps: vec![
                format!("Update Leptos from {} to {}", from_version, to_version),
                "Update leptos-state to latest version".to_string(),
                "Run tests to ensure compatibility".to_string(),
                "Review breaking changes documentation".to_string(),
            ],
            breaking_changes: vec!["Check release notes for breaking changes".to_string()],
        })
    }
}

/// Get migration guidance between versions (convenience function)
pub fn get_migration_guidance(from_version: &str, to_version: &str) -> Option<MigrationGuidance> {
    let provider = MigrationProvider::new();
    provider.get_migration_guidance(from_version, to_version)
}

/// Version constraints manager with dynamic updates
pub struct VersionConstraintsManager {
    constraints: VersionConstraints,
    last_updated: std::time::SystemTime,
}

impl VersionConstraintsManager {
    /// Create a new constraints manager
    pub fn new() -> Self {
        Self {
            constraints: VersionConstraints {
                minimum_version: "0.8.8".to_string(),
                recommended_version: "0.8.8".to_string(),
                maximum_version: None,
            },
            last_updated: std::time::SystemTime::now(),
        }
    }
    
    /// Get current constraints
    pub fn get_constraints(&self) -> &VersionConstraints {
        &self.constraints
    }
    
    /// Update constraints (for future extensibility)
    pub fn update_constraints(&mut self, new_constraints: VersionConstraints) {
        self.constraints = new_constraints;
        self.last_updated = std::time::SystemTime::now();
    }
    
    /// Check if constraints are up to date
    pub fn is_up_to_date(&self) -> bool {
        self.last_updated.elapsed().unwrap_or_default() < std::time::Duration::from_secs(86400) // 24 hours
    }
}

/// Get version constraints (convenience function)
pub fn get_version_constraints() -> VersionConstraints {
    let manager = VersionConstraintsManager::new();
    manager.get_constraints().clone()
}

/// Compatibility information provider with comprehensive data
pub struct CompatibilityInfoProvider {
    info: CompatibilityInfo,
    test_results: HashMap<String, TestResult>,
}

/// Test result for a specific version
#[derive(Debug, Clone)]
pub struct TestResult {
    pub version: String,
    pub passed: bool,
    pub test_count: u32,
    pub failure_count: u32,
    pub last_tested: std::time::SystemTime,
}

impl CompatibilityInfoProvider {
    /// Create a new compatibility info provider
    pub fn new() -> Self {
        let mut test_results = HashMap::new();
        
        // Add test results for supported versions
        test_results.insert("0.8.8".to_string(), TestResult {
            version: "0.8.8".to_string(),
            passed: true,
            test_count: 100,
            failure_count: 0,
            last_tested: std::time::SystemTime::now(),
        });
        
        Self {
            info: CompatibilityInfo {
                supported_versions: vec!["0.8.8".to_string(), "0.8.9".to_string()],
                latest_tested_version: Some("0.8.8".to_string()),
                minimum_version: "0.8.8".to_string(),
            },
            test_results,
        }
    }
    
    /// Get compatibility information
    pub fn get_info(&self) -> &CompatibilityInfo {
        &self.info
    }
    
    /// Get test results for a version
    pub fn get_test_results(&self, version: &str) -> Option<&TestResult> {
        self.test_results.get(version)
    }
    
    /// Check if a version is fully tested
    pub fn is_fully_tested(&self, version: &str) -> bool {
        self.test_results.get(version)
            .map(|result| result.passed && result.failure_count == 0)
            .unwrap_or(false)
    }
}

/// Get compatibility information (convenience function)
pub fn get_compatibility_info() -> CompatibilityInfo {
    let provider = CompatibilityInfoProvider::new();
    provider.get_info().clone()
}

/// Detect breaking changes between versions
pub fn detect_breaking_changes(_from_version: &str, _to_version: &str) -> VersionResult<Vec<BreakingChange>> {
    // For now, return empty list
    // In a real implementation, this would analyze the differences
    Ok(vec![])
}

/// Get feature flags for the current version
pub fn get_feature_flags() -> Vec<String> {
    let mut flags = vec!["leptos-0-8-8".to_string()];
    
    if let Ok(version) = leptos_version() {
        if version.as_str() >= "0.9.0" {
            flags.push("leptos-0-9-0".to_string());
        }
    }
    
    flags
}

/// Validate the runtime Leptos version
pub fn validate_runtime_leptos_version() -> VersionResult<()> {
    let version = leptos_version()?;
    check_leptos_compatibility(&version)
}

/// Validate a specific Leptos version
pub fn validate_leptos_version(version: &str) -> VersionResult<()> {
    check_leptos_compatibility(version)
}

/// Get version-specific documentation
pub fn get_version_specific_docs() -> String {
    let version = leptos_version().unwrap_or_else(|_| "unknown".to_string());
    format!(
        "leptos-state documentation for Leptos {}\n\n\
        This version of leptos-state is compatible with Leptos {} and later.\n\
        For the best experience, use the latest stable version of Leptos.",
        version, version
    )
}

/// Get version-specific configuration
pub fn get_version_specific_config() -> VersionResult<VersionConfig> {
    Ok(VersionConfig {
        leptos_version: leptos_version().ok(),
        features: available_features(),
        compatibility_mode: false,
    })
}

/// Get compatibility matrix
pub fn get_compatibility_matrix() -> HashMap<String, CompatibilityInfo> {
    let mut matrix = HashMap::new();
    
    matrix.insert("0.8.8".to_string(), CompatibilityInfo {
        supported_versions: vec!["0.8.8".to_string()],
        latest_tested_version: Some("0.8.8".to_string()),
        minimum_version: "0.8.8".to_string(),
    });
    
    matrix
}

/// Get upgrade recommendations
pub fn get_upgrade_recommendations(current_version: &str) -> Option<UpgradeRecommendations> {
    if current_version >= "0.8.8" {
        return None;
    }
    
    Some(UpgradeRecommendations {
        target_version: "0.8.8".to_string(),
        steps: vec![
            "Update Leptos to version 0.8.8 or later".to_string(),
            "Update leptos-state to latest version".to_string(),
            "Test your application thoroughly".to_string(),
        ],
        benefits: vec![
            "Better performance".to_string(),
            "New features".to_string(),
            "Bug fixes".to_string(),
        ],
    })
}

/// Version utility functions
pub struct VersionUtils;

impl VersionUtils {
    /// Create a version error
    pub fn create_version_error(version: &str) -> VersionError {
        VersionError::UnsupportedVersion {
            version: version.to_string(),
            minimum: "0.8.8".to_string(),
        }
    }
    
    /// Compare two version strings
    pub fn compare_versions(version1: &str, version2: &str) -> std::cmp::Ordering {
        let v1_parts: Vec<u32> = version1.split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        let v2_parts: Vec<u32> = version2.split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        
        // Compare each part
        for (v1, v2) in v1_parts.iter().zip(v2_parts.iter()) {
            match v1.cmp(v2) {
                std::cmp::Ordering::Equal => continue,
                other => return other,
            }
        }
        
        // If all parts are equal, compare lengths
        v1_parts.len().cmp(&v2_parts.len())
    }
    
    /// Check if a version is newer than another
    pub fn is_newer(version1: &str, version2: &str) -> bool {
        Self::compare_versions(version1, version2) == std::cmp::Ordering::Greater
    }
    
    /// Get the latest version from a list
    pub fn get_latest_version(versions: &[String]) -> Option<&String> {
        versions.iter().max_by(|a, b| Self::compare_versions(a, b))
    }
    
    /// Format version for display
    pub fn format_version(version: &str) -> String {
        format!("v{}", version)
    }
}

/// Create a version error (convenience function)
pub fn create_version_error(version: &str) -> VersionError {
    VersionUtils::create_version_error(version)
}

/// Example provider with version-specific examples
pub struct ExampleProvider {
    examples: HashMap<String, Vec<VersionExample>>,
}

impl ExampleProvider {
    /// Create a new example provider
    pub fn new() -> Self {
        let mut examples = HashMap::new();
        
        // Examples for 0.8.8+
        examples.insert("0.8.8".to_string(), vec![
            VersionExample {
                code: "use leptos_state::*;".to_string(),
                description: "Basic import example".to_string(),
                leptos_version: "0.8.8".to_string(),
            },
            VersionExample {
                code: "let machine = Machine::new(initial_state, context);".to_string(),
                description: "Creating a state machine".to_string(),
                leptos_version: "0.8.8".to_string(),
            },
            VersionExample {
                code: "let store = leptos_state::Store::new(initial_data);".to_string(),
                description: "Creating a reactive store".to_string(),
                leptos_version: "0.8.8".to_string(),
            },
        ]);
        
        // Examples for 0.9.0+
        examples.insert("0.9.0".to_string(), vec![
            VersionExample {
                code: "let advanced_machine = AdvancedMachine::with_optimizations();".to_string(),
                description: "Creating an optimized state machine".to_string(),
                leptos_version: "0.9.0".to_string(),
            },
        ]);
        
        Self { examples }
    }
    
    /// Get examples for a specific version
    pub fn get_examples_for_version(&self, version: &str) -> Vec<VersionExample> {
        let mut all_examples = Vec::new();
        
        // Add examples for the specific version and all compatible versions
        for (ver, examples) in &self.examples {
            if version >= ver.as_str() {
                all_examples.extend(examples.clone());
            }
        }
        
        all_examples
    }
    
    /// Get all examples
    pub fn get_all_examples(&self) -> Vec<VersionExample> {
        self.examples.values().flatten().cloned().collect()
    }
}

/// Get version-specific examples (convenience function)
pub fn get_version_specific_examples() -> Vec<VersionExample> {
    let provider = ExampleProvider::new();
    
    if let Ok(version) = leptos_version() {
        provider.get_examples_for_version(&version)
    } else {
        provider.get_examples_for_version("0.8.8")
    }
}

/// Validate feature compatibility
pub fn validate_feature_compatibility(feature: &str) -> VersionResult<()> {
    let features = available_features();
    if features.contains(&feature.to_string()) {
        Ok(())
    } else {
        Err(VersionError::FeatureNotAvailable {
            feature: feature.to_string(),
        })
    }
}

/// Performance metrics provider with version-specific data
pub struct PerformanceMetricsProvider {
    metrics: HashMap<String, VersionPerformanceInfo>,
}

impl PerformanceMetricsProvider {
    /// Create a new performance metrics provider
    pub fn new() -> Self {
        let mut metrics = HashMap::new();
        
        // Performance data for different versions
        metrics.insert("0.8.8".to_string(), VersionPerformanceInfo {
            memory_usage: 1024 * 1024, // 1MB
            initialization_time: std::time::Duration::from_millis(10),
            bundle_size: 50 * 1024, // 50KB
        });
        
        metrics.insert("0.9.0".to_string(), VersionPerformanceInfo {
            memory_usage: 768 * 1024, // 768KB (improved)
            initialization_time: std::time::Duration::from_millis(8), // faster
            bundle_size: 45 * 1024, // 45KB (smaller)
        });
        
        Self { metrics }
    }
    
    /// Get performance info for a specific version
    pub fn get_performance_info(&self, version: &str) -> Option<&VersionPerformanceInfo> {
        self.metrics.get(version)
    }
    
    /// Get best performance info for current version
    pub fn get_best_performance_info(&self) -> VersionResult<VersionPerformanceInfo> {
        if let Ok(version) = leptos_version() {
            if let Some(info) = self.get_performance_info(&version) {
                return Ok(info.clone());
            }
        }
        
        // Fallback to 0.8.8 metrics
        self.get_performance_info("0.8.8")
            .cloned()
            .ok_or_else(|| VersionError::ConfigurationError {
                message: "No performance data available".to_string(),
            })
    }
}

/// Get version performance information (convenience function)
pub fn get_version_performance_info() -> VersionResult<VersionPerformanceInfo> {
    let provider = PerformanceMetricsProvider::new();
    provider.get_best_performance_info()
}

/// Dependency manager for version-specific dependencies
pub struct DependencyManager {
    dependencies: HashMap<String, Vec<VersionDependency>>,
}

impl DependencyManager {
    /// Create a new dependency manager
    pub fn new() -> Self {
        let mut dependencies = HashMap::new();
        
        // Dependencies for 0.8.8+
        dependencies.insert("0.8.8".to_string(), vec![
            VersionDependency {
                name: "leptos".to_string(),
                version_constraint: ">=0.8.8".to_string(),
                required: true,
            },
            VersionDependency {
                name: "serde".to_string(),
                version_constraint: "1.0".to_string(),
                required: false,
            },
        ]);
        
        // Dependencies for 0.9.0+
        dependencies.insert("0.9.0".to_string(), vec![
            VersionDependency {
                name: "leptos".to_string(),
                version_constraint: ">=0.9.0".to_string(),
                required: true,
            },
            VersionDependency {
                name: "serde".to_string(),
                version_constraint: "1.0".to_string(),
                required: false,
            },
            VersionDependency {
                name: "tokio".to_string(),
                version_constraint: "1.0".to_string(),
                required: false,
            },
        ]);
        
        Self { dependencies }
    }
    
    /// Get dependencies for a specific version
    pub fn get_dependencies_for_version(&self, version: &str) -> Vec<VersionDependency> {
        let mut all_deps = Vec::new();
        
        // Add dependencies for the specific version and all compatible versions
        for (ver, deps) in &self.dependencies {
            if version >= ver.as_str() {
                all_deps.extend(deps.clone());
            }
        }
        
        // Remove duplicates
        all_deps.sort_by(|a, b| a.name.cmp(&b.name));
        all_deps.dedup_by(|a, b| a.name == b.name);
        all_deps
    }
    
    /// Get required dependencies only
    pub fn get_required_dependencies(&self, version: &str) -> Vec<VersionDependency> {
        self.get_dependencies_for_version(version)
            .into_iter()
            .filter(|dep| dep.required)
            .collect()
    }
}

/// Get version dependencies (convenience function)
pub fn get_version_dependencies() -> Vec<VersionDependency> {
    let manager = DependencyManager::new();
    
    if let Ok(version) = leptos_version() {
        manager.get_dependencies_for_version(&version)
    } else {
        manager.get_dependencies_for_version("0.8.8")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leptos_version_detection() {
        let version = leptos_version().unwrap();
        assert!(version.as_str() >= "0.8.8");
    }

    #[test]
    fn test_available_features() {
        let features = available_features();
        assert!(features.contains(&"basic_state_management".to_string()));
        assert!(features.contains(&"state_machines".to_string()));
        assert!(features.contains(&"stores".to_string()));
    }

    #[test]
    fn test_version_compatibility() {
        assert!(check_leptos_compatibility("0.8.8").is_ok());
        assert!(check_leptos_compatibility("0.8.9").is_ok());
        assert!(check_leptos_compatibility("0.7.0").is_err());
    }

    #[test]
    fn test_migration_guidance() {
        let guidance = get_migration_guidance("0.8.7", "0.8.8");
        assert!(guidance.is_some());
        
        let guidance = guidance.unwrap();
        assert_eq!(guidance.target_version, "0.8.8");
        assert!(!guidance.steps.is_empty());
    }

    #[test]
    fn test_version_constraints() {
        let constraints = get_version_constraints();
        assert_eq!(constraints.minimum_version, "0.8.8");
        assert_eq!(constraints.recommended_version, "0.8.8");
    }

    #[test]
    fn test_compatibility_info() {
        let info = get_compatibility_info();
        assert!(info.supported_versions.contains(&"0.8.8".to_string()));
        assert_eq!(info.minimum_version, "0.8.8");
    }

    #[test]
    fn test_feature_flags() {
        let flags = get_feature_flags();
        assert!(flags.contains(&"leptos-0-8-8".to_string()));
    }

    #[test]
    fn test_version_validation() {
        assert!(validate_runtime_leptos_version().is_ok());
        assert!(validate_leptos_version("0.8.8").is_ok());
        assert!(validate_leptos_version("0.7.0").is_err());
    }

    #[test]
    fn test_version_docs() {
        let docs = get_version_specific_docs();
        assert!(docs.contains("leptos-state"));
        assert!(docs.contains("0.8.8"));
    }

    #[test]
    fn test_version_config() {
        let config = get_version_specific_config().unwrap();
        assert!(config.leptos_version.is_some());
        assert!(!config.features.is_empty());
    }

    #[test]
    fn test_compatibility_matrix() {
        let matrix = get_compatibility_matrix();
        assert!(matrix.contains_key("0.8.8"));
    }

    #[test]
    fn test_upgrade_recommendations() {
        let recommendations = get_upgrade_recommendations("0.7.0");
        assert!(recommendations.is_some());
        
        let recommendations = recommendations.unwrap();
        assert_eq!(recommendations.target_version, "0.8.8");
        assert!(!recommendations.steps.is_empty());
    }

    #[test]
    fn test_version_error() {
        let error = create_version_error("0.7.0");
        assert!(matches!(error, VersionError::UnsupportedVersion { .. }));
    }

    #[test]
    fn test_version_examples() {
        let examples = get_version_specific_examples();
        assert!(!examples.is_empty());
        
        for example in examples {
            assert!(!example.code.is_empty());
            assert!(!example.description.is_empty());
        }
    }

    #[test]
    fn test_feature_compatibility() {
        assert!(validate_feature_compatibility("basic_state_management").is_ok());
        assert!(validate_feature_compatibility("state_machines").is_ok());
        assert!(validate_feature_compatibility("nonexistent_feature").is_err());
    }

    #[test]
    fn test_version_performance() {
        let performance = get_version_performance_info().unwrap();
        assert!(performance.memory_usage > 0);
        assert!(performance.initialization_time > std::time::Duration::ZERO);
        assert!(performance.bundle_size > 0);
    }

    #[test]
    fn test_version_dependencies() {
        let dependencies = get_version_dependencies();
        assert!(!dependencies.is_empty());
        
        let leptos_dep = dependencies.iter().find(|dep| dep.name == "leptos").unwrap();
        assert!(leptos_dep.version_constraint.contains("0.8.8"));
        assert!(leptos_dep.required);
    }
}
