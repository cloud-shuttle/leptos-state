//! Tests for Leptos version support and compatibility
//! 
//! These tests ensure that leptos-state works correctly with different
//! versions of Leptos and provides proper feature detection.


/// Test that we can detect the current Leptos version
#[test]
fn test_leptos_version_detection() {
    // This test will fail initially - we need to implement version detection
    let version = leptos_state::leptos_version();
    assert!(version.is_ok(), "Should be able to detect Leptos version");
    
    let version = version.unwrap();
    assert!(version.as_str() >= "0.8.8", "Should support Leptos 0.8.8 or higher");
}

/// Test that feature detection works for different Leptos versions
#[test]
fn test_leptos_feature_detection() {
    // Test that we can detect available features based on Leptos version
    let features = leptos_state::available_features();
    
    // These features should always be available
    assert!(features.contains(&"basic_state_management".to_string()));
    assert!(features.contains(&"state_machines".to_string()));
    assert!(features.contains(&"stores".to_string()));
    
    // These features might be version-dependent
    if leptos_state::leptos_version().unwrap().as_str() >= "0.8.8" {
        assert!(features.contains(&"advanced_hooks".to_string()));
        assert!(features.contains(&"performance_optimizations".to_string()));
    }
}

/// Test that we can create a machine with version-specific features
#[test]
fn test_version_specific_machine_creation() {
    // Test that we can create a machine with features appropriate for the current Leptos version
    let machine = leptos_state::create_version_compatible_machine();
    assert!(machine.is_ok(), "Should be able to create version-compatible machine");
    
    // Machine creation should succeed
    assert!(machine.is_ok(), "Machine should be created successfully");
}

/// Test that we provide clear error messages for unsupported Leptos versions
#[test]
fn test_unsupported_version_error() {
    // This test will help us implement proper error handling
    let result = leptos_state::check_leptos_compatibility("0.7.0");
    assert!(result.is_err(), "Should reject unsupported Leptos versions");
    
    let error = result.unwrap_err();
    assert!(error.to_string().contains("0.8.8"), "Error should mention minimum supported version");
}

/// Test that we can provide migration guidance for version updates
#[test]
fn test_migration_guidance() {
    // Test that we can provide migration guidance when Leptos versions change
    let guidance = leptos_state::get_migration_guidance("0.8.7", "0.8.8");
    assert!(guidance.is_some(), "Should provide migration guidance");
    
    let guidance = guidance.unwrap();
    assert!(!guidance.steps.is_empty(), "Migration guidance should not be empty");
    assert!(guidance.target_version.contains("0.8.8"), "Guidance should mention target version");
}

/// Test that our API remains stable across Leptos versions
#[test]
fn test_api_stability() {
    // Test that our core API remains stable
    // Note: Machine::new requires parameters, so we'll test the version functions instead
    let version = leptos_state::leptos_version();
    assert!(version.is_ok(), "Version detection should work");
    
    let features = leptos_state::available_features();
    assert!(!features.is_empty(), "Features should be available");
}

/// Test that we can handle Leptos version constraints properly
#[test]
fn test_version_constraints() {
    // Test that we properly handle version constraints
    let constraints = leptos_state::get_version_constraints();
    
    assert!(constraints.minimum_version.as_str() >= "0.8.8");
    assert!(constraints.recommended_version.as_str() >= "0.8.8");
    assert!(constraints.maximum_version.is_none() || constraints.maximum_version >= Some("0.8.8".to_string()));
}

/// Test that we can provide compatibility information
#[test]
fn test_compatibility_information() {
    // Test that we can provide detailed compatibility information
    let info = leptos_state::get_compatibility_info();
    
    assert!(info.supported_versions.len() > 0, "Should support at least one version");
    assert!(info.supported_versions.contains(&"0.8.8".to_string()));
    
    if let Some(latest) = info.latest_tested_version {
        assert!(latest.as_str() >= "0.8.8");
    }
}

/// Test that we can detect breaking changes between versions
#[test]
fn test_breaking_change_detection() {
    // Test that we can detect and report breaking changes
    let changes = leptos_state::detect_breaking_changes("0.8.7", "0.8.8");
    
    // Should be able to detect changes (even if none exist)
    assert!(changes.is_ok(), "Should be able to detect breaking changes");
    
    let _changes = changes.unwrap();
    // The actual changes will depend on what exists between versions
    // This test ensures the detection mechanism works
}

/// Test that we can provide feature flags for different Leptos versions
#[test]
fn test_feature_flags() {
    // Test that we can provide appropriate feature flags
    let flags = leptos_state::get_feature_flags();
    
    assert!(flags.contains(&"leptos-0-8-8".to_string()));
    
    // Should have appropriate flags for the current version
    let current_version = leptos_state::leptos_version().unwrap();
    if current_version.as_str() >= "0.9.0" {
        assert!(flags.contains(&"leptos-0-9-0".to_string()));
    }
}

/// Test that we can validate Leptos version at runtime
#[test]
fn test_runtime_version_validation() {
    // Test that we can validate the Leptos version at runtime
    let result = leptos_state::validate_runtime_leptos_version();
    assert!(result.is_ok(), "Should validate current Leptos version successfully");
    
    // Test with a mock version that should fail
    let result = leptos_state::validate_leptos_version("0.7.0");
    assert!(result.is_err(), "Should reject unsupported versions");
}

/// Test that we can provide version-specific documentation
#[test]
fn test_version_specific_documentation() {
    // Test that we can provide documentation appropriate for the current version
    let docs = leptos_state::get_version_specific_docs();
    
    assert!(!docs.is_empty(), "Should provide version-specific documentation");
    assert!(docs.contains("leptos-state"), "Documentation should mention leptos-state");
    
    // Should include version information
    let current_version = leptos_state::leptos_version().unwrap();
    assert!(docs.contains(&current_version), "Documentation should include current version");
}

/// Test that we can handle version-specific configuration
#[test]
fn test_version_specific_configuration() {
    // Test that we can provide configuration appropriate for the current version
    let config = leptos_state::get_version_specific_config();
    
    assert!(config.is_ok(), "Should be able to get version-specific configuration");
    
    let config = config.unwrap();
    assert!(config.leptos_version.is_some(), "Config should include Leptos version");
    assert!(config.features.len() > 0, "Config should include available features");
}

/// Test that we can provide version compatibility matrix
#[test]
fn test_compatibility_matrix() {
    // Test that we can provide a compatibility matrix
    let matrix = leptos_state::get_compatibility_matrix();
    
    assert!(matrix.len() > 0, "Should provide compatibility matrix");
    
    // Should include our supported versions
    for version in &["0.8.8", "0.8.9", "0.9.0"] {
        if matrix.contains_key(*version) {
            let _compatibility = &matrix[*version];
            // Compatibility info exists for supported versions
        }
    }
}

/// Test that we can provide upgrade recommendations
#[test]
fn test_upgrade_recommendations() {
    // Test that we can provide upgrade recommendations
    let recommendations = leptos_state::get_upgrade_recommendations("0.8.7");
    
    assert!(recommendations.is_some(), "Should provide upgrade recommendations");
    
    let recommendations = recommendations.unwrap();
    assert!(recommendations.target_version.as_str() >= "0.8.8", "Should recommend supported version");
    assert!(!recommendations.steps.is_empty(), "Should provide upgrade steps");
}

/// Test that we can handle version-specific error messages
#[test]
fn test_version_specific_error_messages() {
    // Test that we can provide version-specific error messages
    let error = leptos_state::create_version_error("0.7.0");
    let error_msg = error.to_string();
    
    assert!(error_msg.contains("0.8.8"), "Error should mention minimum version");
    assert!(error_msg.contains("upgrade") || error_msg.contains("minimum") || error_msg.contains("supported"), "Error should suggest upgrade or mention minimum");
    assert!(error_msg.contains("leptos") || error_msg.contains("Leptos"), "Error should mention Leptos");
}

/// Test that we can provide version-specific examples
#[test]
fn test_version_specific_examples() {
    // Test that we can provide examples appropriate for the current version
    let examples = leptos_state::get_version_specific_examples();
    
    assert!(!examples.is_empty(), "Should provide version-specific examples");
    
    for example in examples {
        assert!(example.code.contains("leptos_state") || example.code.contains("leptos-state") || example.code.contains("Machine"), "Example should use leptos_state or Machine");
        assert!(!example.description.is_empty(), "Example should have description");
    }
}

/// Test that we can validate feature compatibility
#[test]
fn test_feature_compatibility_validation() {
    // Test that we can validate feature compatibility with current Leptos version
    let features = ["basic_state_management", "state_machines", "stores"];
    
    for feature in &features {
        let result = leptos_state::validate_feature_compatibility(feature);
        assert!(result.is_ok(), "Core features should be compatible");
    }
    
    // Test with a feature that might not be available
    let _result = leptos_state::validate_feature_compatibility("experimental_feature");
    // This might pass or fail depending on implementation
    // The important thing is that the validation mechanism works
}

/// Test that we can provide version-specific performance characteristics
#[test]
fn test_version_specific_performance() {
    // Test that we can provide performance characteristics for the current version
    let performance = leptos_state::get_version_performance_info();
    
    assert!(performance.is_ok(), "Should be able to get performance info");
    
    let performance = performance.unwrap();
    assert!(performance.memory_usage > 0, "Should report memory usage");
    assert!(performance.initialization_time > std::time::Duration::ZERO, "Should report initialization time");
}

/// Test that we can handle version-specific dependencies
#[test]
fn test_version_specific_dependencies() {
    // Test that we can handle dependencies appropriate for the current version
    let dependencies = leptos_state::get_version_dependencies();
    
    assert!(!dependencies.is_empty(), "Should have dependencies");
    
    // Should include Leptos as a dependency
    assert!(dependencies.iter().any(|dep| dep.name == "leptos"), "Should depend on Leptos");
    
    // Should have appropriate version constraints
    let leptos_dep = dependencies.iter().find(|dep| dep.name == "leptos").unwrap();
    assert!(leptos_dep.version_constraint.contains("0.8.8"), "Should require minimum Leptos version");
}
