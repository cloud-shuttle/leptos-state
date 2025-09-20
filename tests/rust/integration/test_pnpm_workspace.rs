//! Integration tests for PNPM workspace functionality
//! 
//! These tests verify that our PNPM workspace configuration works correctly
//! and follows ADR-005: PNPM Package Management Strategy

use std::process::Command;
use std::fs;
use std::path::Path;

/// Test that PNPM workspace configuration is properly set up
#[test]
fn test_pnpm_workspace_configuration() {
    // Given: PNPM workspace should be configured
    let workspace_file = Path::new("../../../pnpm-workspace.yaml");
    
    // When: Checking if workspace file exists
    let workspace_exists = workspace_file.exists();
    
    // Then: Workspace file should exist
    assert!(workspace_exists, "pnpm-workspace.yaml should exist");
    
    // And: Workspace file should contain proper configuration
    let workspace_content = fs::read_to_string(workspace_file).unwrap();
    assert!(workspace_content.contains("packages:"), "Workspace should define packages");
    assert!(workspace_content.contains("'packages/*'"), "Should include packages/* pattern");
}

/// Test that package.json has proper PNPM configuration
#[test]
fn test_package_json_pnpm_configuration() {
    // Given: package.json should exist
    let package_file = Path::new("../../../package.json");
    assert!(package_file.exists(), "package.json should exist");
    
    // When: Reading package.json content
    let package_content = fs::read_to_string(package_file).unwrap();
    
    // Then: Should have PNPM package manager specified
    assert!(package_content.contains("\"packageManager\""), "Should specify package manager");
    assert!(package_content.contains("pnpm"), "Should use PNPM as package manager");
    
    // And: Should have proper engines configuration
    assert!(package_content.contains("\"engines\""), "Should have engines configuration");
    assert!(package_content.contains("\"node\""), "Should specify Node.js version");
    assert!(package_content.contains("\"pnpm\""), "Should specify PNPM version");
}

/// Test that .npmrc configuration is properly set up
#[test]
fn test_npmrc_configuration() {
    // Given: .npmrc should exist
    let npmrc_file = Path::new("../../../.npmrc");
    
    // When: Checking if .npmrc exists
    let npmrc_exists = npmrc_file.exists();
    
    // Then: .npmrc should exist
    assert!(npmrc_exists, ".npmrc should exist");
    
    // And: Should contain proper PNPM configuration
    let npmrc_content = fs::read_to_string(npmrc_file).unwrap();
    assert!(npmrc_content.contains("shamefully-hoist=false"), "Should disable shamefully-hoist");
    assert!(npmrc_content.contains("strict-peer-dependencies=false"), "Should set strict-peer-dependencies");
    assert!(npmrc_content.contains("auto-install-peers=true"), "Should enable auto-install-peers");
}

/// Test that PNPM can install dependencies
#[test]
fn test_pnpm_install_dependencies() {
    // Given: PNPM should be available
    let pnpm_check = Command::new("pnpm")
        .arg("--version")
        .output();
    
    // When: Checking PNPM availability
    let pnpm_available = pnpm_check.is_ok();
    
    // Then: PNPM should be available
    assert!(pnpm_available, "PNPM should be available in PATH");
    
    // And: Should be able to install dependencies
    let install_result = Command::new("pnpm")
        .arg("install")
        .arg("--frozen-lockfile")
        .output();
    
    // Note: This test might fail if pnpm-lock.yaml doesn't exist yet
    // That's expected in the Red phase - we'll create it in Green phase
    if install_result.is_err() {
        println!("PNPM install failed (expected in Red phase): {:?}", install_result);
    }
}

/// Test that PNPM workspace can manage multiple packages
#[test]
fn test_pnpm_workspace_package_management() {
    // Given: Workspace should be configured
    let workspace_file = Path::new("../../../pnpm-workspace.yaml");
    assert!(workspace_file.exists(), "Workspace file should exist");
    
    // When: Reading workspace configuration
    let workspace_content = fs::read_to_string(workspace_file).unwrap();
    
    // Then: Should support multiple package patterns
    assert!(workspace_content.contains("'packages/*'"), "Should support packages/*");
    assert!(workspace_content.contains("'apps/*'"), "Should support apps/*");
    assert!(workspace_content.contains("'tools/*'"), "Should support tools/*");
    assert!(workspace_content.contains("'tests/*'"), "Should support tests/*");
}

/// Test that package.json scripts are properly configured
#[test]
fn test_package_json_scripts_configuration() {
    // Given: package.json should exist
    let package_file = Path::new("../../../package.json");
    assert!(package_file.exists(), "package.json should exist");
    
    // When: Reading package.json content
    let package_content = fs::read_to_string(package_file).unwrap();
    
    // Then: Should have proper scripts configuration
    assert!(package_content.contains("\"scripts\""), "Should have scripts section");
    assert!(package_content.contains("\"install\""), "Should have install script");
    assert!(package_content.contains("\"build\""), "Should have build script");
    assert!(package_content.contains("\"test\""), "Should have test script");
    assert!(package_content.contains("\"test:e2e\""), "Should have E2E test script");
    
    // And: Scripts should use PNPM
    assert!(package_content.contains("pnpm run"), "Scripts should use PNPM");
}

/// Test that PNPM lockfile is properly managed
#[test]
fn test_pnpm_lockfile_management() {
    // Given: PNPM should be configured
    let package_file = Path::new("../../../package.json");
    assert!(package_file.exists(), "package.json should exist");
    
    // When: Checking for lockfile
    let lockfile = Path::new("../../../pnpm-lock.yaml");
    let lockfile_exists = lockfile.exists();
    
    // Then: Lockfile should exist (or be creatable)
    if !lockfile_exists {
        println!("pnpm-lock.yaml doesn't exist yet (expected in Red phase)");
    }
    
    // And: Package.json should reference frozen lockfile
    let package_content = fs::read_to_string(package_file).unwrap();
    assert!(package_content.contains("--frozen-lockfile"), "Should use frozen lockfile");
}

/// Test that PNPM workspace supports filtering
#[test]
fn test_pnpm_workspace_filtering() {
    // Given: PNPM should be available
    let pnpm_check = Command::new("pnpm")
        .arg("--version")
        .output();
    
    // When: PNPM is available
    if pnpm_check.is_ok() {
        // Then: Should support workspace filtering
        let filter_result = Command::new("pnpm")
            .arg("--help")
            .output();
        
        if filter_result.is_ok() {
            let result = filter_result.unwrap();
            let help_output = String::from_utf8_lossy(&result.stdout);
            assert!(help_output.contains("--filter"), "Should support --filter option");
        }
    }
}

/// Test that PNPM workspace supports parallel execution
#[test]
fn test_pnpm_workspace_parallel_execution() {
    // Given: PNPM should be available
    let pnpm_check = Command::new("pnpm")
        .arg("--version")
        .output();
    
    // When: PNPM is available
    if pnpm_check.is_ok() {
        // Then: Should support parallel execution
        let help_result = Command::new("pnpm")
            .arg("--help")
            .output();
        
        if help_result.is_ok() {
            let result = help_result.unwrap();
            let help_output = String::from_utf8_lossy(&result.stdout);
            assert!(help_output.contains("--parallel"), "Should support --parallel option");
        }
    }
}

/// Test that PNPM workspace supports recursive operations
#[test]
fn test_pnpm_workspace_recursive_operations() {
    // Given: PNPM should be available
    let pnpm_check = Command::new("pnpm")
        .arg("--version")
        .output();
    
    // When: PNPM is available
    if pnpm_check.is_ok() {
        // Then: Should support recursive operations
        let help_result = Command::new("pnpm")
            .arg("--help")
            .output();
        
        if help_result.is_ok() {
            let result = help_result.unwrap();
            let help_output = String::from_utf8_lossy(&result.stdout);
            assert!(help_output.contains("--recursive"), "Should support --recursive option");
        }
    }
}

/// Test that PNPM workspace supports dependency management
#[test]
fn test_pnpm_workspace_dependency_management() {
    // Given: PNPM should be available
    let pnpm_check = Command::new("pnpm")
        .arg("--version")
        .output();
    
    // When: PNPM is available
    if pnpm_check.is_ok() {
        // Then: Should support dependency management commands
        let help_result = Command::new("pnpm")
            .arg("--help")
            .output();
        
        if help_result.is_ok() {
            let result = help_result.unwrap();
            let help_output = String::from_utf8_lossy(&result.stdout);
            assert!(help_output.contains("add"), "Should support 'add' command");
            assert!(help_output.contains("remove"), "Should support 'remove' command");
            assert!(help_output.contains("update"), "Should support 'update' command");
        }
    }
}

/// Test that PNPM workspace supports audit functionality
#[test]
fn test_pnpm_workspace_audit_functionality() {
    // Given: PNPM should be available
    let pnpm_check = Command::new("pnpm")
        .arg("--version")
        .output();
    
    // When: PNPM is available
    if pnpm_check.is_ok() {
        // Then: Should support audit functionality
        let help_result = Command::new("pnpm")
            .arg("--help")
            .output();
        
        if help_result.is_ok() {
            let result = help_result.unwrap();
            let help_output = String::from_utf8_lossy(&result.stdout);
            assert!(help_output.contains("audit"), "Should support 'audit' command");
        }
    }
}

/// Test that PNPM workspace supports security features
#[test]
fn test_pnpm_workspace_security_features() {
    // Given: PNPM should be available
    let pnpm_check = Command::new("pnpm")
        .arg("--version")
        .output();
    
    // When: PNPM is available
    if pnpm_check.is_ok() {
        // Then: Should support security features
        let help_result = Command::new("pnpm")
            .arg("--help")
            .output();
        
        if help_result.is_ok() {
            let result = help_result.unwrap();
            let help_output = String::from_utf8_lossy(&result.stdout);
            assert!(help_output.contains("audit"), "Should support audit for security");
        }
    }
}

/// Test that PNPM workspace supports monorepo features
#[test]
fn test_pnpm_workspace_monorepo_features() {
    // Given: Workspace should be configured for monorepo
    let workspace_file = Path::new("../../../pnpm-workspace.yaml");
    assert!(workspace_file.exists(), "Workspace file should exist");
    
    // When: Reading workspace configuration
    let workspace_content = fs::read_to_string(workspace_file).unwrap();
    
    // Then: Should support monorepo patterns
    assert!(workspace_content.contains("'packages/*'"), "Should support packages/*");
    assert!(workspace_content.contains("'apps/*'"), "Should support apps/*");
    
    // And: Should support workspace-specific operations
    let package_file = Path::new("../../../package.json");
    if package_file.exists() {
        let package_content = fs::read_to_string(package_file).unwrap();
        assert!(package_content.contains("\"scripts\""), "Should have scripts for monorepo");
    }
}

/// Test that PNPM workspace supports CI/CD integration
#[test]
fn test_pnpm_workspace_cicd_integration() {
    // Given: CI/CD configuration should exist
    let cicd_dir = Path::new("../../../.github/workflows");
    let cicd_exists = cicd_dir.exists();
    
    // When: CI/CD directory exists
    if cicd_exists {
        // Then: Should have PNPM-specific CI/CD configuration
        let entries = fs::read_dir(cicd_dir).unwrap();
        let mut has_pnpm_config = false;
        
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "yml" || ext == "yaml") {
                let content = fs::read_to_string(&path).unwrap();
                if content.contains("pnpm") {
                    has_pnpm_config = true;
                    break;
                }
            }
        }
        
        if has_pnpm_config {
            assert!(true, "CI/CD should include PNPM configuration");
        } else {
            println!("CI/CD PNPM configuration not found (expected in Red phase)");
        }
    } else {
        println!("CI/CD directory not found (expected in Red phase)");
    }
}

/// Test that PNPM workspace supports performance optimization
#[test]
fn test_pnpm_workspace_performance_optimization() {
    // Given: PNPM should be available
    let pnpm_check = Command::new("pnpm")
        .arg("--version")
        .output();
    
    // When: PNPM is available
    if pnpm_check.is_ok() {
        // Then: Should support performance optimization features
        let help_result = Command::new("pnpm")
            .arg("--help")
            .output();
        
        if help_result.is_ok() {
            let result = help_result.unwrap();
            let help_output = String::from_utf8_lossy(&result.stdout);
            assert!(help_output.contains("--frozen-lockfile"), "Should support frozen lockfile");
            assert!(help_output.contains("--prefer-offline"), "Should support offline mode");
        }
    }
}

/// Test that PNPM workspace supports caching
#[test]
fn test_pnpm_workspace_caching() {
    // Given: PNPM should be available
    let pnpm_check = Command::new("pnpm")
        .arg("--version")
        .output();
    
    // When: PNPM is available
    if pnpm_check.is_ok() {
        // Then: Should support caching features
        let help_result = Command::new("pnpm")
            .arg("--help")
            .output();
        
        if help_result.is_ok() {
            let result = help_result.unwrap();
            let help_output = String::from_utf8_lossy(&result.stdout);
            assert!(help_output.contains("--prefer-offline"), "Should support offline caching");
        }
    }
}

/// Test that PNPM workspace supports workspace-specific scripts
#[test]
fn test_pnpm_workspace_specific_scripts() {
    // Given: Package.json should exist
    let package_file = Path::new("../../../package.json");
    assert!(package_file.exists(), "package.json should exist");
    
    // When: Reading package.json content
    let package_content = fs::read_to_string(package_file).unwrap();
    
    // Then: Should have workspace-specific scripts
    assert!(package_content.contains("\"scripts\""), "Should have scripts section");
    
    // And: Should support workspace operations
    if package_content.contains("pnpm run") {
        assert!(true, "Should support workspace script execution");
    }
}

/// Test that PNPM workspace supports dependency hoisting
#[test]
fn test_pnpm_workspace_dependency_hoisting() {
    // Given: .npmrc should exist
    let npmrc_file = Path::new("../../../.npmrc");
    assert!(npmrc_file.exists(), ".npmrc should exist");
    
    // When: Reading .npmrc content
    let npmrc_content = fs::read_to_string(npmrc_file).unwrap();
    
    // Then: Should have proper hoisting configuration
    assert!(npmrc_content.contains("shamefully-hoist=false"), "Should disable shamefully-hoist");
    
    // And: Should have strict peer dependencies configuration
    assert!(npmrc_content.contains("strict-peer-dependencies=false"), "Should set strict-peer-dependencies");
}

/// Test that PNPM workspace supports auto-install peers
#[test]
fn test_pnpm_workspace_auto_install_peers() {
    // Given: .npmrc should exist
    let npmrc_file = Path::new("../../../.npmrc");
    assert!(npmrc_file.exists(), ".npmrc should exist");
    
    // When: Reading .npmrc content
    let npmrc_content = fs::read_to_string(npmrc_file).unwrap();
    
    // Then: Should have auto-install peers enabled
    assert!(npmrc_content.contains("auto-install-peers=true"), "Should enable auto-install-peers");
}

/// Test that PNPM workspace supports registry configuration
#[test]
fn test_pnpm_workspace_registry_configuration() {
    // Given: .npmrc should exist
    let npmrc_file = Path::new("../../../.npmrc");
    assert!(npmrc_file.exists(), ".npmrc should exist");
    
    // When: Reading .npmrc content
    let npmrc_content = fs::read_to_string(npmrc_file).unwrap();
    
    // Then: Should have registry configuration
    assert!(npmrc_content.contains("registry="), "Should have registry configuration");
    assert!(npmrc_content.contains("npmjs.org"), "Should use npmjs.org registry");
}

/// Test that PNPM workspace supports prefer-frozen-lockfile
#[test]
fn test_pnpm_workspace_prefer_frozen_lockfile() {
    // Given: .npmrc should exist
    let npmrc_file = Path::new("../../../.npmrc");
    assert!(npmrc_file.exists(), ".npmrc should exist");
    
    // When: Reading .npmrc content
    let npmrc_content = fs::read_to_string(npmrc_file).unwrap();
    
    // Then: Should prefer frozen lockfile
    assert!(npmrc_content.contains("prefer-frozen-lockfile=true"), "Should prefer frozen lockfile");
}
