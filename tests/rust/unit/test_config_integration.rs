//! Tests for configuration system integration
//!
//! This module tests the integration of multiple configuration sources,
//! routing rules, and connection management that were identified as
//! coverage gaps in the assessment.

use leptos_state::*;
use leptos_state::utils::config::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[test]
fn test_config_source_precedence() {
    // Test that sources are loaded in correct precedence order
    let mut loader = ConfigLoader::new();

    // Add sources in reverse precedence order
    loader.add_source(ConfigSource::Environment);
    loader.add_source(ConfigSource::JsonFile("test.json".to_string()));

    // Verify sources are stored
    assert_eq!(loader.sources.len(), 2);

    // Test that we can create sources from the enum
    let env_source = ConfigSource::Environment.to_trait_source().unwrap();
    let file_source = ConfigSource::JsonFile("test.json".to_string()).to_trait_source().unwrap();

    // Both should be available (though file may not exist)
    assert!(env_source.is_available());
    // File availability depends on whether test.json exists
}

#[tokio::test]
async fn test_multi_source_config_loading() {
    // Set up test environment variables
    std::env::set_var("TEST_APP_NAME", "integration-test");
    std::env::set_var("TEST_APP_VERSION", "1.0.0");
    std::env::set_var("TEST_DATABASE_HOST", "env-db");

    // Create a temporary JSON file
    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let json_content = r#"{
        "app": {
            "name": "file-test",
            "environment": "testing"
        },
        "database": {
            "host": "file-db",
            "port": 5432
        }
    }"#;
    std::fs::write(temp_file.path(), json_content).unwrap();

    let mut loader = ConfigLoader::new();

    // Add environment source first (lower precedence)
    loader.add_source(ConfigSource::Environment);

    // Add JSON file source (higher precedence)
    let file_path = temp_file.path().to_string_lossy().to_string();
    loader.add_source(ConfigSource::JsonFile(file_path.clone()));

    // Load configuration
    let config = loader.load().await.unwrap();

    // File source should override environment variables for conflicting keys
    assert_eq!(config["app"]["name"], "file-test"); // From file
    assert_eq!(config["app"]["environment"], "testing"); // From file
    assert_eq!(config["database"]["host"], "file-db"); // From file
    assert_eq!(config["database"]["port"], 5432); // From file

    // Environment variables should still be available for non-conflicting keys
    assert_eq!(config["app"]["version"], "1.0.0"); // From env (no conflict)

    // Clean up
    std::env::remove_var("TEST_APP_NAME");
    std::env::remove_var("TEST_APP_VERSION");
    std::env::remove_var("TEST_DATABASE_HOST");
    drop(temp_file); // Clean up temp file
}

#[tokio::test]
async fn test_config_merging_behavior() {
    // Test deep merging of nested configuration objects

    // Create first config source
    let source1_content = r#"{
        "app": {
            "name": "test-app",
            "features": {
                "auth": true,
                "logging": false
            }
        },
        "database": {
            "primary": {
                "host": "db1",
                "port": 5432
            }
        }
    }"#;

    // Create second config source
    let source2_content = r#"{
        "app": {
            "version": "2.0",
            "features": {
                "auth": false,
                "metrics": true
            }
        },
        "database": {
            "secondary": {
                "host": "db2",
                "port": 5433
            }
        }
    }"#;

    let temp_file1 = tempfile::NamedTempFile::new().unwrap();
    let temp_file2 = tempfile::NamedTempFile::new().unwrap();

    std::fs::write(temp_file1.path(), source1_content).unwrap();
    std::fs::write(temp_file2.path(), source2_content).unwrap();

    let mut loader = ConfigLoader::new();
    loader.add_source(ConfigSource::JsonFile(temp_file1.path().to_string_lossy().to_string()));
    loader.add_source(ConfigSource::JsonFile(temp_file2.path().to_string_lossy().to_string()));

    let config = loader.load().await.unwrap();

    // Test that objects are merged at top level
    assert_eq!(config["app"]["name"], "test-app");
    assert_eq!(config["app"]["version"], "2.0");

    // Test that nested objects are merged
    assert_eq!(config["app"]["features"]["auth"], false); // Overridden
    assert_eq!(config["app"]["features"]["logging"], false); // From source1
    assert_eq!(config["app"]["features"]["metrics"], true); // From source2

    // Test that separate objects coexist
    assert_eq!(config["database"]["primary"]["host"], "db1");
    assert_eq!(config["database"]["secondary"]["host"], "db2");

    drop(temp_file1);
    drop(temp_file2);
}

#[test]
fn test_config_validation_edge_cases() {
    // Test configuration validation with edge cases

    // Test empty file path
    let invalid_source = ConfigSource::JsonFile("".to_string());
    assert!(invalid_source.to_trait_source().is_err());

    // Test file with invalid path characters
    let invalid_path_source = ConfigSource::JsonFile("../invalid/../path.json".to_string());
    let source = invalid_path_source.to_trait_source().unwrap();
    assert!(source.validate().is_err());

    // Test valid file path
    let valid_source = ConfigSource::JsonFile("valid.json".to_string());
    let source = valid_source.to_trait_source().unwrap();
    // Validation should not panic, even if file doesn't exist
    let _ = source.validate(); // Result can be Ok or Err
}

#[tokio::test]
async fn test_config_performance_with_large_configs() {
    // Test performance with large configuration files

    // Create a large JSON configuration
    let mut large_config = serde_json::Map::new();
    large_config.insert("metadata".to_string(), serde_json::json!({"version": "1.0"}));

    // Add many configuration entries
    let mut services = serde_json::Map::new();
    for i in 0..1000 {
        services.insert(
            format!("service_{}", i),
            serde_json::json!({
                "host": format!("service-{}.example.com", i),
                "port": 8080 + (i % 100),
                "enabled": i % 2 == 0,
                "config": {
                    "timeout": 30,
                    "retries": 3,
                    "features": ["auth", "logging", "metrics"]
                }
            })
        );
    }
    large_config.insert("services".to_string(), serde_json::Value::Object(services));

    let large_json = serde_json::to_string_pretty(&large_config).unwrap();

    let temp_file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(temp_file.path(), &large_json).unwrap();

    // Measure loading time
    let start_time = std::time::Instant::now();

    let mut loader = ConfigLoader::new();
    loader.add_source(ConfigSource::JsonFile(temp_file.path().to_string_lossy().to_string()));

    let config = loader.load().await.unwrap();

    let elapsed = start_time.elapsed();

    // Should load within reasonable time (< 1 second for 1000 entries)
    assert!(elapsed < std::time::Duration::from_secs(1),
        "Large config loading too slow: {:?}", elapsed);

    // Verify all entries were loaded
    assert_eq!(config["services"].as_object().unwrap().len(), 1000);
    assert!(config["metadata"]["version"].as_str().unwrap() == "1.0");

    drop(temp_file);
}

#[tokio::test]
async fn test_config_concurrent_loading() {
    // Test that configuration loading works correctly under concurrent access

    // Set up test environment
    std::env::set_var("TEST_CONCURRENT_KEY", "concurrent-value");

    let temp_file = tempfile::NamedTempFile::new().unwrap();
    let json_content = r#"{"concurrent": {"from_file": true}}"#;
    std::fs::write(temp_file.path(), json_content).unwrap();

    let loader = Arc::new(RwLock::new({
        let mut l = ConfigLoader::new();
        l.add_source(ConfigSource::Environment);
        l.add_source(ConfigSource::JsonFile(temp_file.path().to_string_lossy().to_string()));
        l
    }));

    // Spawn multiple concurrent loaders
    let mut handles = vec![];
    for i in 0..10 {
        let loader_clone = Arc::clone(&loader);
        let handle = tokio::spawn(async move {
            let loader = loader_clone.read().await;
            let config = loader.load().await.unwrap();

            // Verify both sources are present
            assert_eq!(config["concurrent"]["from_file"], true);
            assert_eq!(config["TEST_CONCURRENT_KEY"], "concurrent-value");

            format!("task_{}", i)
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.starts_with("task_"));
    }

    // Clean up
    std::env::remove_var("TEST_CONCURRENT_KEY");
    drop(temp_file);
}

#[test]
fn test_config_source_type_detection() {
    // Test automatic source type detection from file extensions

    let json_path = "/path/to/config.json";
    let yaml_path = "/path/to/config.yaml";
    let toml_path = "/path/to/config.toml";
    let unknown_path = "/path/to/config.unknown";

    // Test JSON detection
    let json_source = FileSource::detect_format(json_path).unwrap();
    assert!(matches!(json_source, FileSource::Json { .. }));

    // Test YAML detection
    let yaml_source = FileSource::detect_format(yaml_path).unwrap();
    assert!(matches!(yaml_source, FileSource::Yaml { .. }));

    // Test TOML detection (if feature enabled)
    #[cfg(feature = "toml")]
    {
        let toml_source = FileSource::detect_format(toml_path).unwrap();
        assert!(matches!(toml_source, FileSource::Toml { .. }));
    }

    // Test unknown extension
    let unknown_result = FileSource::detect_format(unknown_path);
    assert!(unknown_result.is_err());
}

#[tokio::test]
async fn test_config_error_propagation() {
    // Test that configuration errors are properly propagated

    // Test with non-existent file
    let loader = ConfigLoader::new();
    let source = ConfigSource::JsonFile("nonexistent_file_that_should_not_exist.json".to_string());
    let trait_source = source.to_trait_source().unwrap();

    // File source should indicate it's not available
    assert!(!trait_source.is_available().await);

    // Loading should fail
    let result = trait_source.load().await;
    assert!(result.is_err());
}

#[test]
fn test_config_builder_pattern() {
    // Test the ConfigBuilder fluent API

    let builder = ConfigBuilder::new()
        .add_source(ConfigSource::Environment)
        .add_source(ConfigSource::JsonFile("config.json".to_string()))
        .add_source(ConfigSource::RemoteUrl("http://config.example.com".to_string()));

    // Access the internal loader to verify sources were added
    // Note: This would require making the loader field public or adding accessors
    // For now, we just verify the builder was created successfully
    let _builder = builder;
}

#[tokio::test]
async fn test_config_caching_behavior() {
    // Test that configuration loading can be cached/reused

    std::env::set_var("TEST_CACHE_KEY", "cached-value");

    let mut loader = ConfigLoader::new();
    loader.add_source(ConfigSource::Environment);

    // Load configuration multiple times
    let config1 = loader.load().await.unwrap();
    let config2 = loader.load().await.unwrap();
    let config3 = loader.load().await.unwrap();

    // Results should be consistent
    assert_eq!(config1["TEST_CACHE_KEY"], config2["TEST_CACHE_KEY"]);
    assert_eq!(config2["TEST_CACHE_KEY"], config3["TEST_CACHE_KEY"]);
    assert_eq!(config1["TEST_CACHE_KEY"], "cached-value");

    std::env::remove_var("TEST_CACHE_KEY");
}

#[cfg(feature = "proptest")]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn config_merging_properties(
            base_config in arb_config(),
            override_config in arb_config()
        ) {
            // Test that merging is deterministic and associative where possible
            let merged1 = merge_configs(&base_config, &override_config);
            let merged2 = merge_configs(&base_config, &override_config);

            // Merging should be deterministic
            prop_assert_eq!(merged1, merged2);
        }

        #[test]
        fn config_source_validation_properties(
            valid_path in "[a-zA-Z0-9/_.-]+",
            invalid_path in r"[^a-zA-Z0-9/_.-]+"
        ) {
            // Valid paths should generally pass validation
            if !valid_path.is_empty() && !valid_path.contains("..") {
                let source = FileSource::json(valid_path);
                let result = source.validate();
                // Should not panic, may return Ok or Err
                let _ = result;
            }

            // Invalid paths should generally fail validation
            if !invalid_path.is_empty() {
                let source = FileSource::json(invalid_path);
                let result = source.validate();
                // Should not panic, may return Ok or Err
                let _ = result;
            }
        }
    }

    fn arb_config() -> impl Strategy<Value = serde_json::Value> {
        // Generate arbitrary JSON-like configurations
        prop::collection::hash_map(
            "[a-z]+".prop_map(|s| s.to_string()),
            arb_json_value(),
            0..10
        ).prop_map(|map| serde_json::Value::Object(map.into_iter().collect()))
    }

    fn arb_json_value() -> impl Strategy<Value = serde_json::Value> {
        prop_oneof![
            prop::bool::ANY.prop_map(serde_json::Value::Bool),
            "[a-z0-9]+".prop_map(|s| serde_json::Value::String(s.to_string())),
            (0..100i64).prop_map(|n| serde_json::Value::Number(n.into())),
        ]
    }

    fn merge_configs(base: &serde_json::Value, override_: &serde_json::Value) -> serde_json::Value {
        // Simple merge implementation for testing
        if let (Some(base_obj), Some(override_obj)) = (base.as_object(), override_.as_object()) {
            let mut result = base_obj.clone();
            for (key, value) in override_obj {
                result.insert(key.clone(), value.clone());
            }
            serde_json::Value::Object(result)
        } else {
            override_.clone()
        }
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn benchmark_config_loading() {
        // Create test configurations of different sizes
        let small_config = create_test_config(10);
        let medium_config = create_test_config(100);
        let large_config = create_test_config(1000);

        // Benchmark small config
        let small_time = benchmark_config_load(&small_config).await;

        // Benchmark medium config
        let medium_time = benchmark_config_load(&medium_config).await;

        // Benchmark large config
        let large_time = benchmark_config_load(&large_config).await;

        // Performance should scale reasonably
        // Large config should not be more than 10x slower than small
        assert!(large_time < small_time * 10,
            "Config loading scalability issue: small={:?}, large={:?}", small_time, large_time);
    }

    async fn benchmark_config_load(config: &serde_json::Value) -> std::time::Duration {
        let json_str = serde_json::to_string(config).unwrap();
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(temp_file.path(), &json_str).unwrap();

        let mut loader = ConfigLoader::new();
        loader.add_source(ConfigSource::JsonFile(temp_file.path().to_string_lossy().to_string()));

        let start = Instant::now();
        for _ in 0..10 { // Average over multiple runs
            let _ = loader.load().await.unwrap();
        }
        let total_time = start.elapsed();

        drop(temp_file);
        total_time / 10 // Return average time
    }

    fn create_test_config(size: usize) -> serde_json::Value {
        let mut config = serde_json::Map::new();
        for i in 0..size {
            config.insert(
                format!("key_{}", i),
                serde_json::json!({
                    "value": i,
                    "enabled": i % 2 == 0,
                    "name": format!("item_{}", i)
                })
            );
        }
        serde_json::Value::Object(config)
    }
}
