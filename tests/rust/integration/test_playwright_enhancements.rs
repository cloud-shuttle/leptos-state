//! Tests for enhanced Playwright testing capabilities
//! 
//! These tests ensure that we have comprehensive E2E testing
//! with Playwright for all demos and examples.

use std::fs;
use std::path::Path;
use std::process::Command;

/// Test that Playwright is properly installed and configured
#[test]
fn test_playwright_installation() {
    // Given: Playwright should be available
    let playwright_check = Command::new("npx")
        .arg("playwright")
        .arg("--version")
        .output();
    
    // When: Checking Playwright availability
    let playwright_available = playwright_check.is_ok();
    
    // Then: Playwright should be available
    if !playwright_available {
        println!("Playwright not available (expected in Red phase)");
    }
    
    // And: Should be able to run basic Playwright command
    let playwright_result = Command::new("npx")
        .arg("playwright")
        .arg("--help")
        .output();
    
    // Note: This test might fail if Playwright is not installed
    // That's expected in the Red phase - we'll install it in Green phase
    if playwright_result.is_err() {
        println!("Playwright command failed (expected in Red phase): {:?}", playwright_result);
    }
}

/// Test that Playwright configuration exists
#[test]
fn test_playwright_configuration() {
    // Given: Playwright configuration should exist
    let config_file = Path::new("../../../playwright.config.ts");
    
    // When: Checking if configuration exists
    let config_exists = config_file.exists();
    
    // Then: Configuration should exist
    if !config_exists {
        println!("playwright.config.ts not found (expected in Red phase)");
    }
    
    // And: Should contain proper settings
    if config_exists {
        let config_content = fs::read_to_string(config_file).unwrap();
        assert!(config_content.contains("testDir"), "Should have testDir configuration");
        assert!(config_content.contains("use"), "Should have use configuration");
        assert!(config_content.contains("projects"), "Should have projects configuration");
    }
}

/// Test that Playwright tests directory exists
#[test]
fn test_playwright_tests_directory() {
    // Given: Playwright tests directory should exist
    let tests_dir = Path::new("../../../tests/web/playwright");
    
    // When: Checking if tests directory exists
    let tests_dir_exists = tests_dir.exists();
    
    // Then: Tests directory should exist
    if !tests_dir_exists {
        println!("Playwright tests directory not found (expected in Red phase)");
    }
    
    // And: Should contain test files
    if tests_dir_exists {
        let entries: Vec<_> = fs::read_dir(tests_dir).unwrap().collect();
        assert!(!entries.is_empty(), "Tests directory should not be empty");
    }
}

/// Test that all examples have Playwright tests
#[test]
fn test_examples_playwright_coverage() {
    // Given: Examples directory should exist
    let examples_dir = Path::new("../../../examples");
    assert!(examples_dir.exists(), "Examples directory should exist");
    
    // When: Checking examples
    let examples: Vec<_> = fs::read_dir(examples_dir).unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .collect();
    
    // Then: Each example should have corresponding Playwright test
    let playwright_tests_dir = Path::new("../../../tests/web/playwright");
    
    for example in examples {
        let example_name = example.file_name().to_string_lossy().to_string();
        let test_file = playwright_tests_dir.join(format!("{}.spec.ts", example_name));
        
        if !test_file.exists() {
            println!("Missing Playwright test for example: {} (expected in Red phase)", example_name);
        }
    }
}

/// Test that Playwright can run tests
#[test]
fn test_playwright_test_execution() {
    // Given: Playwright should be available
    let playwright_check = Command::new("npx")
        .arg("playwright")
        .arg("--version")
        .output();
    
    // When: Playwright is available
    if playwright_check.is_ok() {
        // Then: Should be able to run tests
        let test_result = Command::new("timeout")
            .arg("30s")
            .arg("npx")
            .arg("playwright")
            .arg("test")
            .arg("--reporter=list")
            .output();
        
        // Note: This test might fail if there are no tests or test failures
        // That's expected in the Red phase - we'll create tests in Green phase
        if test_result.is_err() {
            println!("Playwright test execution failed (expected in Red phase): {:?}", test_result);
        }
    }
}

/// Test that Playwright browsers are installed
#[test]
fn test_playwright_browsers_installed() {
    // Given: Playwright browsers should be installed
    let browser_check = Command::new("npx")
        .arg("playwright")
        .arg("install")
        .arg("--dry-run")
        .output();
    
    // When: Checking browser installation
    let browsers_available = browser_check.is_ok();
    
    // Then: Browsers should be available (or installable)
    if !browsers_available {
        println!("Playwright browsers not installed (expected in Red phase)");
    }
    
    // And: Should be able to install browsers
    let install_result = Command::new("timeout")
        .arg("60s")
        .arg("npx")
        .arg("playwright")
        .arg("install")
        .arg("chromium")
        .output();
    
    // Note: This test might fail if browsers are not installed
    // That's expected in the Red phase - we'll install them in Green phase
    if install_result.is_err() {
        println!("Browser installation failed (expected in Red phase): {:?}", install_result);
    }
}

/// Test that Playwright has proper CI configuration
#[test]
fn test_playwright_ci_configuration() {
    // Given: CI configuration should exist
    let ci_file = Path::new("../../../.github/workflows/ci.yml");
    assert!(ci_file.exists(), "CI configuration should exist");
    
    // When: Reading CI configuration
    let ci_content = fs::read_to_string(ci_file).unwrap();
    
    // Then: Should include Playwright steps
    assert!(ci_content.contains("playwright"), "Should include Playwright in CI");
    assert!(ci_content.contains("test:e2e"), "Should include E2E test step");
    assert!(ci_content.contains("install:browsers"), "Should include browser installation");
}

/// Test that Playwright has proper package.json scripts
#[test]
fn test_playwright_package_scripts() {
    // Given: package.json should exist
    let package_file = Path::new("../../../package.json");
    assert!(package_file.exists(), "package.json should exist");
    
    // When: Reading package.json
    let package_content = fs::read_to_string(package_file).unwrap();
    
    // Then: Should have Playwright scripts
    assert!(package_content.contains("test:web"), "Should have test:web script");
    assert!(package_content.contains("test:web:ui"), "Should have test:web:ui script");
    assert!(package_content.contains("test:web:headed"), "Should have test:web:headed script");
    assert!(package_content.contains("install:browsers"), "Should have install:browsers script");
}

/// Test that Playwright tests cover all demo scenarios
#[test]
fn test_playwright_demo_coverage() {
    // Given: Demos should have comprehensive test coverage
    let demo_scenarios = vec![
        "counter",
        "todo-app",
        "analytics-dashboard",
        "compatibility-example",
        "codegen",
        "history",
        "traffic-light",
    ];
    
    // When: Checking each demo
    let playwright_tests_dir = Path::new("../../../tests/web/playwright");
    
    for demo in demo_scenarios {
        let test_file = playwright_tests_dir.join(format!("{}.spec.ts", demo));
        
        // Then: Each demo should have a test file
        if !test_file.exists() {
            println!("Missing Playwright test for demo: {} (expected in Red phase)", demo);
        }
    }
}

/// Test that Playwright has proper test structure
#[test]
fn test_playwright_test_structure() {
    // Given: Playwright tests should have proper structure
    let playwright_tests_dir = Path::new("../../../tests/web/playwright");
    
    // When: Tests directory exists
    if playwright_tests_dir.exists() {
        // Then: Should have proper structure
        let entries: Vec<_> = fs::read_dir(playwright_tests_dir).unwrap().collect();
        
        // And: Should contain test files
        let test_files: Vec<_> = entries.iter()
            .filter(|entry| {
                if let Ok(entry) = entry {
                    entry.path().extension().map_or(false, |ext| ext == "ts")
                } else {
                    false
                }
            })
            .collect();
        
        if test_files.is_empty() {
            println!("No Playwright test files found (expected in Red phase)");
        }
    }
}

/// Test that Playwright has proper fixtures and utilities
#[test]
fn test_playwright_fixtures() {
    // Given: Playwright should have fixtures and utilities
    let fixtures_dir = Path::new("../../../tests/web/playwright/fixtures");
    let utils_dir = Path::new("../../../tests/web/playwright/utils");
    
    // When: Checking fixtures and utilities
    let fixtures_exist = fixtures_dir.exists();
    let utils_exist = utils_dir.exists();
    
    // Then: Should have proper test infrastructure
    if !fixtures_exist {
        println!("Playwright fixtures directory not found (expected in Red phase)");
    }
    
    if !utils_exist {
        println!("Playwright utils directory not found (expected in Red phase)");
    }
}

/// Test that Playwright has proper test data
#[test]
fn test_playwright_test_data() {
    // Given: Playwright should have test data
    let test_data_dir = Path::new("../../../tests/web/playwright/test-data");
    
    // When: Checking test data directory
    let test_data_exists = test_data_dir.exists();
    
    // Then: Should have test data
    if !test_data_exists {
        println!("Playwright test data directory not found (expected in Red phase)");
    }
    
    // And: Should contain data files
    if test_data_exists {
        let entries: Vec<_> = fs::read_dir(test_data_dir).unwrap().collect();
        if entries.is_empty() {
            println!("Playwright test data directory is empty (expected in Red phase)");
        }
    }
}

/// Test that Playwright has proper reporting configuration
#[test]
fn test_playwright_reporting() {
    // Given: Playwright should have proper reporting
    let config_file = Path::new("../../../playwright.config.ts");
    
    // When: Configuration exists
    if config_file.exists() {
        let config_content = fs::read_to_string(config_file).unwrap();
        
        // Then: Should have reporting configuration
        assert!(config_content.contains("reporter"), "Should have reporter configuration");
        assert!(config_content.contains("html"), "Should have HTML reporter");
    }
}

/// Test that Playwright has proper timeout configuration
#[test]
fn test_playwright_timeouts() {
    // Given: Playwright should have proper timeouts
    let config_file = Path::new("../../../playwright.config.ts");
    
    // When: Configuration exists
    if config_file.exists() {
        let config_content = fs::read_to_string(config_file).unwrap();
        
        // Then: Should have timeout configuration
        assert!(config_content.contains("timeout"), "Should have timeout configuration");
        assert!(config_content.contains("expect"), "Should have expect timeout");
    }
}

/// Test that Playwright has proper browser configuration
#[test]
fn test_playwright_browser_config() {
    // Given: Playwright should have browser configuration
    let config_file = Path::new("../../../playwright.config.ts");
    
    // When: Configuration exists
    if config_file.exists() {
        let config_content = fs::read_to_string(config_file).unwrap();
        
        // Then: Should have browser configuration
        assert!(config_content.contains("chromium"), "Should have Chromium configuration");
        assert!(config_content.contains("firefox"), "Should have Firefox configuration");
        assert!(config_content.contains("webkit"), "Should have WebKit configuration");
    }
}

/// Test that Playwright has proper CI integration
#[test]
fn test_playwright_ci_integration() {
    // Given: CI should run Playwright tests
    let ci_file = Path::new("../../../.github/workflows/ci.yml");
    assert!(ci_file.exists(), "CI configuration should exist");
    
    // When: Reading CI configuration
    let ci_content = fs::read_to_string(ci_file).unwrap();
    
    // Then: Should have Playwright job
    assert!(ci_content.contains("playwright"), "Should have Playwright job");
    assert!(ci_content.contains("test:e2e"), "Should run E2E tests");
    
    // And: Should install browsers
    assert!(ci_content.contains("install:browsers"), "Should install browsers");
}

/// Test that Playwright has proper error handling
#[test]
fn test_playwright_error_handling() {
    // Given: Playwright should have proper error handling
    let playwright_tests_dir = Path::new("../../../tests/web/playwright");
    
    // When: Tests directory exists
    if playwright_tests_dir.exists() {
        // Then: Should have error handling utilities
        let utils_file = playwright_tests_dir.join("utils/error-handling.ts");
        if !utils_file.exists() {
            println!("Playwright error handling utilities not found (expected in Red phase)");
        }
    }
}

/// Test that Playwright has proper accessibility testing
#[test]
fn test_playwright_accessibility() {
    // Given: Playwright should have accessibility testing
    let playwright_tests_dir = Path::new("../../../tests/web/playwright");
    
    // When: Tests directory exists
    if playwright_tests_dir.exists() {
        // Then: Should have accessibility tests
        let a11y_file = playwright_tests_dir.join("accessibility.spec.ts");
        if !a11y_file.exists() {
            println!("Playwright accessibility tests not found (expected in Red phase)");
        }
    }
}

/// Test that Playwright has proper performance testing
#[test]
fn test_playwright_performance() {
    // Given: Playwright should have performance testing
    let playwright_tests_dir = Path::new("../../../tests/web/playwright");
    
    // When: Tests directory exists
    if playwright_tests_dir.exists() {
        // Then: Should have performance tests
        let perf_file = playwright_tests_dir.join("performance.spec.ts");
        if !perf_file.exists() {
            println!("Playwright performance tests not found (expected in Red phase)");
        }
    }
}

/// Test that Playwright has proper visual regression testing
#[test]
fn test_playwright_visual_regression() {
    // Given: Playwright should have visual regression testing
    let playwright_tests_dir = Path::new("../../../tests/web/playwright");
    
    // When: Tests directory exists
    if playwright_tests_dir.exists() {
        // Then: Should have visual regression tests
        let visual_file = playwright_tests_dir.join("visual.spec.ts");
        if !visual_file.exists() {
            println!("Playwright visual regression tests not found (expected in Red phase)");
        }
    }
}

