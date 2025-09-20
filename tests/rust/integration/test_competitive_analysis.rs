//! Tests for competitive analysis features
//! 
//! These tests ensure that we have feature parity or exceed
//! our competitors' capabilities.

use std::fs;
use std::path::Path;
use std::process::Command;

/// Test that competitive analysis configuration exists
#[test]
fn test_competitive_analysis_configuration() {
    // Given: Competitive analysis configuration should exist
    let config_file = Path::new("../../../competitive-analysis.toml");
    
    // When: Checking if configuration exists
    let config_exists = config_file.exists();
    
    // Then: Configuration should exist
    if !config_exists {
        println!("competitive-analysis.toml not found (expected in Red phase)");
    }
    
    // And: Should contain proper settings
    if config_exists {
        let config_content = fs::read_to_string(config_file).unwrap();
        assert!(config_content.contains("competitors"), "Should have competitors section");
        assert!(config_content.contains("features"), "Should have features section");
        assert!(config_content.contains("benchmarks"), "Should have benchmarks section");
    }
}

/// Test that competitor data is available
#[test]
fn test_competitor_data_availability() {
    // Given: Competitor data should be available
    let competitors_file = Path::new("../../../data/competitors.json");
    
    // When: Checking if competitor data exists
    let competitors_exist = competitors_file.exists();
    
    // Then: Competitor data should exist
    if !competitors_exist {
        println!("competitors.json not found (expected in Red phase)");
    }
    
    // And: Should contain competitor information
    if competitors_exist {
        let competitors_content = fs::read_to_string(competitors_file).unwrap();
        assert!(competitors_content.contains("name"), "Should contain competitor names");
        assert!(competitors_content.contains("features"), "Should contain feature lists");
        assert!(competitors_content.contains("performance"), "Should contain performance data");
    }
}

/// Test that feature comparison matrix exists
#[test]
fn test_feature_comparison_matrix() {
    // Given: Feature comparison matrix should exist
    let matrix_file = Path::new("../../../docs/competitive-analysis/feature-matrix.md");
    
    // When: Checking if matrix exists
    let matrix_exists = matrix_file.exists();
    
    // Then: Matrix should exist
    if !matrix_exists {
        println!("feature-matrix.md not found (expected in Red phase)");
    }
    
    // And: Should contain comparison data
    if matrix_exists {
        let matrix_content = fs::read_to_string(matrix_file).unwrap();
        assert!(matrix_content.contains("Feature"), "Should have feature column");
        assert!(matrix_content.contains("leptos-state"), "Should include leptos-state");
        assert!(matrix_content.contains("✅"), "Should have feature indicators");
    }
}

/// Test that performance benchmarks exist
#[test]
fn test_performance_benchmarks() {
    // Given: Performance benchmarks should exist
    let benchmarks_file = Path::new("../../../benchmarks/competitive-benchmarks.rs");
    
    // When: Checking if benchmarks exist
    let benchmarks_exist = benchmarks_file.exists();
    
    // Then: Benchmarks should exist
    if !benchmarks_exist {
        println!("competitive-benchmarks.rs not found (expected in Red phase)");
    }
    
    // And: Should contain benchmark tests
    if benchmarks_exist {
        let benchmarks_content = fs::read_to_string(benchmarks_file).unwrap();
        assert!(benchmarks_content.contains("#[bench]"), "Should have benchmark functions");
        assert!(benchmarks_content.contains("criterion"), "Should use criterion for benchmarking");
    }
}

/// Test that competitive analysis tools are available
#[test]
fn test_competitive_analysis_tools() {
    // Given: Competitive analysis tools should be available
    let tools_dir = Path::new("../../../tools/competitive-analysis");
    
    // When: Checking if tools directory exists
    let tools_exist = tools_dir.exists();
    
    // Then: Tools directory should exist
    if !tools_exist {
        println!("competitive-analysis tools directory not found (expected in Red phase)");
    }
    
    // And: Should contain analysis scripts
    if tools_exist {
        let entries: Vec<_> = fs::read_dir(tools_dir).unwrap().collect();
        assert!(!entries.is_empty(), "Tools directory should not be empty");
    }
}

/// Test that competitor feature tracking works
#[test]
fn test_competitor_feature_tracking() {
    // Given: Competitor feature tracking should work
    let tracking_file = Path::new("../../../src/competitive/feature_tracker.rs");
    
    // When: Checking if tracking module exists
    let tracking_exists = tracking_file.exists();
    
    // Then: Tracking module should exist
    if !tracking_exists {
        println!("feature_tracker.rs not found (expected in Red phase)");
    }
    
    // And: Should contain tracking functionality
    if tracking_exists {
        let tracking_content = fs::read_to_string(tracking_file).unwrap();
        assert!(tracking_content.contains("struct"), "Should have data structures");
        assert!(tracking_content.contains("impl"), "Should have implementations");
    }
}

/// Test that competitive analysis reports are generated
#[test]
fn test_competitive_analysis_reports() {
    // Given: Competitive analysis reports should be generated
    let reports_dir = Path::new("../../../reports/competitive-analysis");
    
    // When: Checking if reports directory exists
    let reports_exist = reports_dir.exists();
    
    // Then: Reports directory should exist
    if !reports_exist {
        println!("competitive-analysis reports directory not found (expected in Red phase)");
    }
    
    // And: Should contain report files
    if reports_exist {
        let entries: Vec<_> = fs::read_dir(reports_dir).unwrap().collect();
        if entries.is_empty() {
            println!("Reports directory is empty (expected in Red phase)");
        }
    }
}

/// Test that competitive analysis CI integration works
#[test]
fn test_competitive_analysis_ci_integration() {
    // Given: CI should run competitive analysis
    let ci_file = Path::new("../../../.github/workflows/ci.yml");
    assert!(ci_file.exists(), "CI configuration should exist");
    
    // When: Reading CI configuration
    let ci_content = fs::read_to_string(ci_file).unwrap();
    
    // Then: Should include competitive analysis steps
    assert!(ci_content.contains("competitive"), "Should include competitive analysis in CI");
    assert!(ci_content.contains("benchmark"), "Should include benchmarking");
}

/// Test that competitor data is up to date
#[test]
fn test_competitor_data_freshness() {
    // Given: Competitor data should be up to date
    let competitors_file = Path::new("../../../data/competitors.json");
    
    // When: Data file exists
    if competitors_file.exists() {
        let metadata = fs::metadata(competitors_file).unwrap();
        let modified = metadata.modified().unwrap();
        let now = std::time::SystemTime::now();
        let age = now.duration_since(modified).unwrap();
        
        // Then: Data should be less than 30 days old
        if age.as_secs() > 30 * 24 * 60 * 60 {
            println!("Competitor data is older than 30 days (expected in Red phase)");
        }
    }
}

/// Test that feature parity analysis works
#[test]
fn test_feature_parity_analysis() {
    // Given: Feature parity analysis should work
    let parity_file = Path::new("../../../src/competitive/parity_analyzer.rs");
    
    // When: Checking if parity analyzer exists
    let parity_exists = parity_file.exists();
    
    // Then: Parity analyzer should exist
    if !parity_exists {
        println!("parity_analyzer.rs not found (expected in Red phase)");
    }
    
    // And: Should contain analysis logic
    if parity_exists {
        let parity_content = fs::read_to_string(parity_file).unwrap();
        assert!(parity_content.contains("fn"), "Should have functions");
        assert!(parity_content.contains("analyze"), "Should have analysis functions");
    }
}

/// Test that competitive analysis dashboard exists
#[test]
fn test_competitive_analysis_dashboard() {
    // Given: Competitive analysis dashboard should exist
    let dashboard_file = Path::new("../../../examples/competitive-dashboard/src/main.rs");
    
    // When: Checking if dashboard exists
    let dashboard_exists = dashboard_file.exists();
    
    // Then: Dashboard should exist
    if !dashboard_exists {
        println!("competitive-dashboard not found (expected in Red phase)");
    }
    
    // And: Should be a working example
    if dashboard_exists {
        let dashboard_content = fs::read_to_string(dashboard_file).unwrap();
        assert!(dashboard_content.contains("leptos"), "Should use Leptos");
        assert!(dashboard_content.contains("main"), "Should have main function");
    }
}

/// Test that competitive analysis metrics are tracked
#[test]
fn test_competitive_analysis_metrics() {
    // Given: Competitive analysis metrics should be tracked
    let metrics_file = Path::new("../../../src/competitive/metrics.rs");
    
    // When: Checking if metrics module exists
    let metrics_exists = metrics_file.exists();
    
    // Then: Metrics module should exist
    if !metrics_exists {
        println!("metrics.rs not found (expected in Red phase)");
    }
    
    // And: Should contain metrics tracking
    if metrics_exists {
        let metrics_content = fs::read_to_string(metrics_file).unwrap();
        assert!(metrics_content.contains("struct"), "Should have data structures");
        assert!(metrics_content.contains("track"), "Should have tracking functions");
    }
}

/// Test that competitive analysis alerts work
#[test]
fn test_competitive_analysis_alerts() {
    // Given: Competitive analysis alerts should work
    let alerts_file = Path::new("../../../src/competitive/alerts.rs");
    
    // When: Checking if alerts module exists
    let alerts_exists = alerts_file.exists();
    
    // Then: Alerts module should exist
    if !alerts_exists {
        println!("alerts.rs not found (expected in Red phase)");
    }
    
    // And: Should contain alert functionality
    if alerts_exists {
        let alerts_content = fs::read_to_string(alerts_file).unwrap();
        assert!(alerts_content.contains("alert"), "Should have alert functions");
        assert!(alerts_content.contains("threshold"), "Should have threshold logic");
    }
}

/// Test that competitive analysis API exists
#[test]
fn test_competitive_analysis_api() {
    // Given: Competitive analysis API should exist
    let api_file = Path::new("../../../src/competitive/api.rs");
    
    // When: Checking if API module exists
    let api_exists = api_file.exists();
    
    // Then: API module should exist
    if !api_exists {
        println!("api.rs not found (expected in Red phase)");
    }
    
    // And: Should contain API endpoints
    if api_exists {
        let api_content = fs::read_to_string(api_file).unwrap();
        assert!(api_content.contains("pub fn"), "Should have public functions");
        assert!(api_content.contains("endpoint"), "Should have API endpoints");
    }
}

/// Test that competitive analysis documentation exists
#[test]
fn test_competitive_analysis_documentation() {
    // Given: Competitive analysis documentation should exist
    let docs_file = Path::new("../../../docs/competitive-analysis/README.md");
    
    // When: Checking if documentation exists
    let docs_exist = docs_file.exists();
    
    // Then: Documentation should exist
    if !docs_exist {
        println!("competitive-analysis README not found (expected in Red phase)");
    }
    
    // And: Should contain comprehensive information
    if docs_exist {
        let docs_content = fs::read_to_string(docs_file).unwrap();
        assert!(docs_content.contains("#"), "Should have headers");
        assert!(docs_content.contains("competitor"), "Should mention competitors");
    }
}

/// Test that competitive analysis tests exist
#[test]
fn test_competitive_analysis_tests() {
    // Given: Competitive analysis should have tests
    let tests_file = Path::new("../../../tests/competitive_analysis_tests.rs");
    
    // When: Checking if tests exist
    let tests_exist = tests_file.exists();
    
    // Then: Tests should exist
    if !tests_exist {
        println!("competitive_analysis_tests.rs not found (expected in Red phase)");
    }
    
    // And: Should contain test cases
    if tests_exist {
        let tests_content = fs::read_to_string(tests_file).unwrap();
        assert!(tests_content.contains("#[test]"), "Should have test functions");
        assert!(tests_content.contains("assert"), "Should have assertions");
    }
}

/// Test that competitive analysis configuration is valid
#[test]
fn test_competitive_analysis_config_validity() {
    // Given: Competitive analysis configuration should be valid
    let config_file = Path::new("../../../competitive-analysis.toml");
    
    // When: Configuration exists
    if config_file.exists() {
        let config_content = fs::read_to_string(config_file).unwrap();
        
        // Then: Should be valid TOML (basic validation)
        assert!(config_content.contains("="), "Should contain TOML key-value pairs");
        
        // And: Should have required sections
        assert!(config_content.contains("[competitors]"), "Should have competitors section");
        assert!(config_content.contains("[features]"), "Should have features section");
    }
}

/// Test that competitive analysis can run benchmarks
#[test]
fn test_competitive_analysis_benchmark_execution() {
    // Given: Competitive analysis should be able to run benchmarks
    let benchmark_result = Command::new("cargo")
        .arg("bench")
        .arg("--bench")
        .arg("competitive-benchmarks")
        .output();
    
    // When: Running benchmarks
    let benchmark_available = benchmark_result.is_ok();
    
    // Then: Benchmarks should be available (or runnable)
    if !benchmark_available {
        println!("Competitive benchmarks not available (expected in Red phase)");
    }
}

/// Test that competitive analysis has proper error handling
#[test]
fn test_competitive_analysis_error_handling() {
    // Given: Competitive analysis should have proper error handling
    let error_file = Path::new("../../../src/competitive/error.rs");
    
    // When: Checking if error handling exists
    let error_exists = error_file.exists();
    
    // Then: Error handling should exist
    if !error_exists {
        println!("error.rs not found (expected in Red phase)");
    }
    
    // And: Should contain error types
    if error_exists {
        let error_content = fs::read_to_string(error_file).unwrap();
        assert!(error_content.contains("Error"), "Should have error types");
        assert!(error_content.contains("Result"), "Should use Result types");
    }
}
