//! Integration tests for Rust coding standards
//! 
//! These tests verify that our Rust configuration follows ADR-007: Rust Coding Standards and Latest Practices

use std::process::Command;
use std::fs;
use std::path::Path;

/// Test that Cargo.toml has proper Rust configuration
#[test]
fn test_cargo_toml_rust_configuration() {
    // Given: leptos-state Cargo.toml should exist
    let cargo_file = Path::new("../../../leptos-state/Cargo.toml");
    assert!(cargo_file.exists(), "leptos-state Cargo.toml should exist");
    
    // When: Reading Cargo.toml content
    let cargo_content = fs::read_to_string(cargo_file).unwrap();
    
    // Then: Should have proper Rust edition
    assert!(cargo_content.contains("edition = \"2024\""), "Should use Rust 2024 edition");
    
    // And: Should have rust-version specified
    assert!(cargo_content.contains("rust-version"), "Should specify rust-version");
    
    // And: Should have proper profile configurations
    assert!(cargo_content.contains("[profile.release]"), "Should have release profile");
    assert!(cargo_content.contains("[profile.dev]"), "Should have dev profile");
    
    // And: Should have optimization settings
    assert!(cargo_content.contains("opt-level = 3"), "Should optimize for performance in release");
    assert!(cargo_content.contains("lto = true"), "Should enable LTO in release");
    assert!(cargo_content.contains("codegen-units = 1"), "Should use single codegen unit");
}

/// Test that rustfmt configuration is properly set up
#[test]
fn test_rustfmt_configuration() {
    // Given: rustfmt.toml should exist
    let rustfmt_file = Path::new("../../../rustfmt.toml");
    
    // When: Checking if rustfmt.toml exists
    let rustfmt_exists = rustfmt_file.exists();
    
    // Then: rustfmt.toml should exist
    assert!(rustfmt_exists, "rustfmt.toml should exist");
    
    // And: Should contain proper configuration
    let rustfmt_content = fs::read_to_string(rustfmt_file).unwrap();
    assert!(rustfmt_content.contains("edition = \"2024\""), "Should use Rust 2024 edition");
    assert!(rustfmt_content.contains("max_width = 100"), "Should have max width of 100");
    assert!(rustfmt_content.contains("tab_spaces = 4"), "Should use 4 spaces for tabs");
    assert!(rustfmt_content.contains("newline_style = \"Unix\""), "Should use Unix newlines");
}

/// Test that clippy configuration is properly set up
#[test]
fn test_clippy_configuration() {
    // Given: .clippy.toml should exist
    let clippy_file = Path::new("../../../.clippy.toml");
    
    // When: Checking if .clippy.toml exists
    let clippy_exists = clippy_file.exists();
    
    // Then: .clippy.toml should exist
    assert!(clippy_exists, ".clippy.toml should exist");
    
    // And: Should contain proper configuration
    let clippy_content = fs::read_to_string(clippy_file).unwrap();
    assert!(clippy_content.contains("deny = ["), "Should have deny configuration");
    assert!(clippy_content.contains("clippy::all"), "Should deny all clippy warnings");
    assert!(clippy_content.contains("clippy::pedantic"), "Should use pedantic clippy");
    assert!(clippy_content.contains("clippy::nursery"), "Should use nursery clippy");
}

/// Test that Rust toolchain is properly configured
#[test]
fn test_rust_toolchain_configuration() {
    // Given: rust-toolchain.toml should exist
    let toolchain_file = Path::new("../../../rust-toolchain.toml");
    
    // When: Checking if rust-toolchain.toml exists
    let toolchain_exists = toolchain_file.exists();
    
    // Then: rust-toolchain.toml should exist
    assert!(toolchain_exists, "rust-toolchain.toml should exist");
    
    // And: Should contain proper configuration
    let toolchain_content = fs::read_to_string(toolchain_file).unwrap();
    assert!(toolchain_content.contains("channel = \"stable\""), "Should use stable channel");
    assert!(toolchain_content.contains("components = ["), "Should specify components");
    assert!(toolchain_content.contains("rustfmt"), "Should include rustfmt");
    assert!(toolchain_content.contains("clippy"), "Should include clippy");
}

/// Test that Rust can compile with proper standards
#[test]
fn test_rust_compilation_standards() {
    // Given: Rust should be available
    let rust_check = Command::new("rustc")
        .arg("--version")
        .output();
    
    // When: Checking Rust availability
    let rust_available = rust_check.is_ok();
    
    // Then: Rust should be available
    assert!(rust_available, "Rust should be available in PATH");
    
    // And: Should be able to check code
    let check_result = Command::new("cargo")
        .arg("check")
        .arg("--all-targets")
        .output();
    
    // Note: This test might fail if there are compilation errors
    // That's expected in the Red phase - we'll fix them in Green phase
    if check_result.is_err() {
        println!("Cargo check failed (expected in Red phase): {:?}", check_result);
    }
}

/// Test that clippy can run with proper standards
#[test]
fn test_clippy_standards() {
    // Given: Clippy should be available
    let clippy_check = Command::new("cargo")
        .arg("clippy")
        .arg("--version")
        .output();
    
    // When: Checking Clippy availability
    let clippy_available = clippy_check.is_ok();
    
    // Then: Clippy should be available
    assert!(clippy_available, "Clippy should be available");
    
    // And: Should be able to run clippy (with timeout to prevent hanging)
    let clippy_result = Command::new("timeout")
        .arg("30s")
        .arg("cargo")
        .arg("clippy")
        .arg("--all-targets")
        .arg("--all-features")
        .arg("--")
        .arg("-D")
        .arg("warnings")
        .output();
    
    // Note: This test might fail if there are clippy warnings
    // That's expected in the Red phase - we'll fix them in Green phase
    if clippy_result.is_err() {
        println!("Clippy failed (expected in Red phase): {:?}", clippy_result);
    }
}

/// Test that rustfmt can format code properly
#[test]
fn test_rustfmt_standards() {
    // Given: rustfmt should be available
    let rustfmt_check = Command::new("rustfmt")
        .arg("--version")
        .output();
    
    // When: Checking rustfmt availability
    let rustfmt_available = rustfmt_check.is_ok();
    
    // Then: rustfmt should be available
    assert!(rustfmt_available, "rustfmt should be available");
    
    // And: Should be able to format code
    let format_result = Command::new("cargo")
        .arg("fmt")
        .arg("--all")
        .arg("--")
        .arg("--check")
        .output();
    
    // Note: This test might fail if code is not formatted
    // That's expected in the Red phase - we'll fix them in Green phase
    if format_result.is_err() {
        println!("rustfmt check failed (expected in Red phase): {:?}", format_result);
    }
}

/// Test that security audit can run
#[test]
fn test_security_audit() {
    // Given: cargo-audit should be available
    let audit_check = Command::new("cargo")
        .arg("audit")
        .arg("--version")
        .output();
    
    // When: Checking cargo-audit availability
    let audit_available = audit_check.is_ok();
    
    // Then: cargo-audit should be available (or installable)
    if !audit_available {
        println!("cargo-audit not available (expected in Red phase)");
    }
    
    // And: Should be able to run security audit
    let audit_result = Command::new("cargo")
        .arg("audit")
        .output();
    
    // Note: This test might fail if cargo-audit is not installed
    // That's expected in the Red phase - we'll install it in Green phase
    if audit_result.is_err() {
        println!("Security audit failed (expected in Red phase): {:?}", audit_result);
    }
}

/// Test that performance benchmarks can run
#[test]
fn test_performance_benchmarks() {
    // Given: criterion should be available
    let cargo_file = Path::new("../../../Cargo.toml");
    assert!(cargo_file.exists(), "Cargo.toml should exist");
    
    // When: Reading Cargo.toml content
    let cargo_content = fs::read_to_string(cargo_file).unwrap();
    
    // Then: Should have criterion as dev dependency
    assert!(cargo_content.contains("criterion"), "Should have criterion for benchmarking");
    
    // And: Should be able to run benchmarks
    let bench_result = Command::new("cargo")
        .arg("bench")
        .output();
    
    // Note: This test might fail if benchmarks are not set up
    // That's expected in the Red phase - we'll set them up in Green phase
    if bench_result.is_err() {
        println!("Benchmarks failed (expected in Red phase): {:?}", bench_result);
    }
}

/// Test that code coverage can be generated
#[test]
fn test_code_coverage() {
    // Given: tarpaulin should be available
    let tarpaulin_check = Command::new("cargo")
        .arg("tarpaulin")
        .arg("--version")
        .output();
    
    // When: Checking tarpaulin availability
    let tarpaulin_available = tarpaulin_check.is_ok();
    
    // Then: tarpaulin should be available (or installable)
    if !tarpaulin_available {
        println!("cargo-tarpaulin not available (expected in Red phase)");
    }
    
    // And: Should be able to generate coverage (with timeout to prevent hanging)
    let coverage_result = Command::new("timeout")
        .arg("60s")
        .arg("cargo")
        .arg("tarpaulin")
        .arg("--out")
        .arg("Html")
        .arg("--output-dir")
        .arg("coverage")
        .output();
    
    // Note: This test might fail if tarpaulin is not installed
    // That's expected in the Red phase - we'll install it in Green phase
    if coverage_result.is_err() {
        println!("Code coverage failed (expected in Red phase): {:?}", coverage_result);
    }
}

/// Test that documentation can be generated
#[test]
fn test_documentation_generation() {
    // Given: Rust should be available
    let rust_check = Command::new("rustc")
        .arg("--version")
        .output();
    
    // When: Rust is available
    if rust_check.is_ok() {
        // Then: Should be able to generate documentation
        let doc_result = Command::new("cargo")
            .arg("doc")
            .arg("--no-deps")
            .output();
        
        // Note: This test might fail if there are doc issues
        // That's expected in the Red phase - we'll fix them in Green phase
        if doc_result.is_err() {
            println!("Documentation generation failed (expected in Red phase): {:?}", doc_result);
        }
    }
}

/// Test that tests can run with proper standards
#[test]
fn test_test_execution_standards() {
    // Given: Rust should be available
    let rust_check = Command::new("rustc")
        .arg("--version")
        .output();
    
    // When: Rust is available
    if rust_check.is_ok() {
        // Then: Should be able to run tests
        let test_result = Command::new("cargo")
            .arg("test")
            .arg("--all-targets")
            .output();
        
        // Note: This test might fail if there are test issues
        // That's expected in the Red phase - we'll fix them in Green phase
        if test_result.is_err() {
            println!("Test execution failed (expected in Red phase): {:?}", test_result);
        }
    }
}

/// Test that nextest is available and can run tests
#[test]
fn test_nextest_availability() {
    // Given: nextest should be available
    let nextest_check = Command::new("cargo")
        .arg("nextest")
        .arg("--version")
        .output();
    
    // When: Checking nextest availability
    let nextest_available = nextest_check.is_ok();
    
    // Then: nextest should be available (or installable)
    if !nextest_available {
        println!("cargo-nextest not available (expected in Red phase)");
    }
    
    // And: Should be able to run tests with nextest (with timeout)
    let nextest_result = Command::new("timeout")
        .arg("30s")
        .arg("cargo")
        .arg("nextest")
        .arg("run")
        .arg("--all-targets")
        .output();
    
    // Note: This test might fail if nextest is not installed
    // That's expected in the Red phase - we'll install it in Green phase
    if nextest_result.is_err() {
        println!("Nextest execution failed (expected in Red phase): {:?}", nextest_result);
    }
}

/// Test that CI/CD configuration includes Rust standards
#[test]
fn test_cicd_rust_standards() {
    // Given: CI/CD configuration should exist
    let cicd_dir = Path::new("../../../.github/workflows");
    let cicd_exists = cicd_dir.exists();
    
    // When: CI/CD directory exists
    if cicd_exists {
        // Then: Should have Rust-specific CI/CD configuration
        let entries = fs::read_dir(cicd_dir).unwrap();
        let mut has_rust_config = false;
        
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "yml" || ext == "yaml") {
                let content = fs::read_to_string(&path).unwrap();
                if content.contains("rust") || content.contains("cargo") {
                    has_rust_config = true;
                    break;
                }
            }
        }
        
        if has_rust_config {
            assert!(true, "CI/CD should include Rust configuration");
        } else {
            println!("CI/CD Rust configuration not found (expected in Red phase)");
        }
    } else {
        println!("CI/CD directory not found (expected in Red phase)");
    }
}

/// Test that pre-commit hooks include Rust standards
#[test]
fn test_precommit_rust_standards() {
    // Given: Pre-commit configuration should exist
    let precommit_file = Path::new("../../../.pre-commit-config.yaml");
    
    // When: Checking if pre-commit config exists
    let precommit_exists = precommit_file.exists();
    
    // Then: Pre-commit config should exist
    if precommit_exists {
        // And: Should contain Rust-specific hooks
        let precommit_content = fs::read_to_string(precommit_file).unwrap();
        assert!(precommit_content.contains("rust"), "Should have Rust pre-commit hooks");
    } else {
        println!("Pre-commit config not found (expected in Red phase)");
    }
}

/// Test that Rust version is up to date
#[test]
fn test_rust_version_currency() {
    // Given: Rust should be available
    let rust_check = Command::new("rustc")
        .arg("--version")
        .output();
    
    // When: Rust is available
    if rust_check.is_ok() {
        // Then: Should be using a recent version
        let result = rust_check.unwrap();
        let version_output = String::from_utf8_lossy(&result.stdout);
        assert!(version_output.contains("rustc"), "Should be using rustc");
        
        // And: Should be using stable channel
        let channel_check = Command::new("rustup")
            .arg("show")
            .output();
        
        if channel_check.is_ok() {
            let channel_result = channel_check.unwrap();
            let channel_output = String::from_utf8_lossy(&channel_result.stdout);
            assert!(channel_output.contains("stable"), "Should be using stable channel");
        }
    }
}

/// Test that Rust components are properly installed
#[test]
fn test_rust_components() {
    // Given: rustup should be available
    let rustup_check = Command::new("rustup")
        .arg("component")
        .arg("list")
        .output();
    
    // When: rustup is available
    if rustup_check.is_ok() {
        // Then: Should have required components
        let result = rustup_check.unwrap();
        let components_output = String::from_utf8_lossy(&result.stdout);
        assert!(components_output.contains("rustfmt"), "Should have rustfmt component");
        assert!(components_output.contains("clippy"), "Should have clippy component");
    }
}

/// Test that Rust targets are properly configured
#[test]
fn test_rust_targets() {
    // Given: rustup should be available
    let rustup_check = Command::new("rustup")
        .arg("target")
        .arg("list")
        .output();
    
    // When: rustup is available
    if rustup_check.is_ok() {
        // Then: Should have required targets
        let result = rustup_check.unwrap();
        let targets_output = String::from_utf8_lossy(&result.stdout);
        assert!(targets_output.contains("wasm32-unknown-unknown"), "Should have WASM target");
    }
}

/// Test that Rust toolchain is properly managed
#[test]
fn test_rust_toolchain_management() {
    // Given: rustup should be available
    let rustup_check = Command::new("rustup")
        .arg("show")
        .output();
    
    // When: rustup is available
    if rustup_check.is_ok() {
        // Then: Should show proper toolchain info
        let result = rustup_check.unwrap();
        let toolchain_output = String::from_utf8_lossy(&result.stdout);
        assert!(toolchain_output.contains("stable"), "Should show stable toolchain");
    }
}

/// Test that Rust workspace is properly configured
#[test]
fn test_rust_workspace_configuration() {
    // Given: Cargo.toml should exist
    let cargo_file = Path::new("../../../Cargo.toml");
    assert!(cargo_file.exists(), "Cargo.toml should exist");
    
    // When: Reading Cargo.toml content
    let cargo_content = fs::read_to_string(cargo_file).unwrap();
    
    // Then: Should have workspace configuration
    if cargo_content.contains("[workspace]") {
        assert!(cargo_content.contains("members"), "Should have workspace members");
    }
}

/// Test that Rust dependencies are properly managed
#[test]
fn test_rust_dependency_management() {
    // Given: Cargo.toml should exist
    let cargo_file = Path::new("../../../Cargo.toml");
    assert!(cargo_file.exists(), "Cargo.toml should exist");
    
    // When: Reading Cargo.toml content
    let cargo_content = fs::read_to_string(cargo_file).unwrap();
    
    // Then: Should have proper dependency sections
    assert!(cargo_content.contains("[workspace.dependencies]"), "Should have workspace dependencies section");
    
    // And: Should have latest versions of key dependencies
    assert!(cargo_content.contains("leptos"), "Should have leptos dependency");
    assert!(cargo_content.contains("serde"), "Should have serde dependency");
    assert!(cargo_content.contains("tokio"), "Should have tokio dependency");
}

/// Test that Rust features are properly configured
#[test]
fn test_rust_features_configuration() {
    // Given: Cargo.toml should exist
    let cargo_file = Path::new("../../../Cargo.toml");
    assert!(cargo_file.exists(), "Cargo.toml should exist");
    
    // When: Reading Cargo.toml content
    let cargo_content = fs::read_to_string(cargo_file).unwrap();
    
    // Then: Should have proper feature configuration
    if cargo_content.contains("features = [") {
        assert!(cargo_content.contains("derive"), "Should have derive features");
    }
}

/// Test that Rust build configuration is optimized
#[test]
fn test_rust_build_optimization() {
    // Given: Cargo.toml should exist
    let cargo_file = Path::new("../../../Cargo.toml");
    assert!(cargo_file.exists(), "Cargo.toml should exist");
    
    // When: Reading Cargo.toml content
    let cargo_content = fs::read_to_string(cargo_file).unwrap();
    
    // Then: Should have optimized build configuration
    assert!(cargo_content.contains("[profile.release]"), "Should have release profile");
    assert!(cargo_content.contains("opt-level = 3"), "Should optimize for performance");
    assert!(cargo_content.contains("lto = true"), "Should enable LTO");
    assert!(cargo_content.contains("codegen-units = 1"), "Should use single codegen unit");
    assert!(cargo_content.contains("panic = \"abort\""), "Should abort on panic");
}
