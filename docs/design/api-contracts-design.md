# 📋 API Contracts Design

## Overview
Comprehensive API contract testing and stability framework to ensure backward compatibility and API evolution safety.

## Architecture

### Core Components
```
contracts/
├── core.rs           # Contract framework and types (100 lines)
├── testing.rs        # Contract testing infrastructure (150 lines)
├── validation.rs     # API validation utilities (120 lines)
├── compatibility.rs  # Backward compatibility checks (100 lines)
└── mod.rs            # Contract exports (50 lines)
```

## Contract Framework

```rust
pub struct ApiContract {
    pub name: String,
    pub version: semver::Version,
    pub stability: StabilityLevel,
    pub tests: Vec<ContractTest>,
    pub breaking_changes: Vec<BreakingChange>,
}

#[derive(Debug, Clone)]
pub enum StabilityLevel {
    Experimental,
    Beta,
    Stable,
    Deprecated,
}

pub struct ContractTest {
    pub name: String,
    pub test_fn: Box<dyn Fn() -> Result<(), ContractError> + Send + Sync>,
    pub required: bool,
}

pub struct BreakingChange {
    pub version: semver::Version,
    pub description: String,
    pub migration_guide: String,
}

impl ApiContract {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: semver::Version::parse(version).unwrap(),
            stability: StabilityLevel::Experimental,
            tests: Vec::new(),
            breaking_changes: Vec::new(),
        }
    }

    pub fn add_test<F>(&mut self, name: &str, test: F)
    where
        F: Fn() -> Result<(), ContractError> + Send + Sync + 'static,
    {
        self.tests.push(ContractTest {
            name: name.to_string(),
            test_fn: Box::new(test),
            required: true,
        });
    }

    pub fn add_breaking_change(&mut self, version: &str, description: &str, migration: &str) {
        self.breaking_changes.push(BreakingChange {
            version: semver::Version::parse(version).unwrap(),
            description: description.to_string(),
            migration_guide: migration.to_string(),
        });
    }

    pub fn validate(&self) -> Result<(), ContractError> {
        for test in &self.tests {
            if test.required {
                test.test_fn()?;
            }
        }
        Ok(())
    }
}
```

## Contract Testing Infrastructure

```rust
pub struct ContractTestRunner {
    contracts: HashMap<String, ApiContract>,
    config: TestConfig,
}

impl ContractTestRunner {
    pub fn new() -> Self {
        Self {
            contracts: HashMap::new(),
            config: TestConfig::default(),
        }
    }

    pub fn register_contract(&mut self, contract: ApiContract) {
        self.contracts.insert(contract.name.clone(), contract);
    }

    pub fn run_all_contracts(&self) -> Result<ContractTestResults, ContractError> {
        let mut results = ContractTestResults::new();

        for (name, contract) in &self.contracts {
            let contract_result = self.run_contract(contract)?;
            results.add_contract_result(name.clone(), contract_result);
        }

        Ok(results)
    }

    pub fn run_contract(&self, contract: &ApiContract) -> Result<ContractResult, ContractError> {
        let start = std::time::Instant::now();
        let result = contract.validate();
        let duration = start.elapsed();

        Ok(ContractResult {
            contract_name: contract.name.clone(),
            version: contract.version.clone(),
            stability: contract.stability.clone(),
            passed: result.is_ok(),
            duration,
            errors: if result.is_err() {
                vec![result.unwrap_err()]
            } else {
                vec![]
            },
        })
    }
}

#[derive(Default)]
pub struct TestConfig {
    pub fail_fast: bool,
    pub verbose: bool,
    pub timeout: Duration,
    pub parallel: bool,
}

pub struct ContractTestResults {
    pub total_contracts: usize,
    pub passed_contracts: usize,
    pub failed_contracts: usize,
    pub results: HashMap<String, ContractResult>,
}

impl ContractTestResults {
    pub fn new() -> Self {
        Self {
            total_contracts: 0,
            passed_contracts: 0,
            failed_contracts: 0,
            results: HashMap::new(),
        }
    }

    pub fn add_contract_result(&mut self, name: String, result: ContractResult) {
        self.total_contracts += 1;
        if result.passed {
            self.passed_contracts += 1;
        } else {
            self.failed_contracts += 1;
        }
        self.results.insert(name, result);
    }

    pub fn is_success(&self) -> bool {
        self.failed_contracts == 0
    }
}
```

## API Validation

```rust
pub struct ApiValidator {
    baseline: ApiSnapshot,
    current: ApiSnapshot,
}

impl ApiValidator {
    pub fn new() -> Self {
        Self {
            baseline: ApiSnapshot::capture_current(),
            current: ApiSnapshot::capture_current(),
        }
    }

    pub fn set_baseline(&mut self) {
        self.baseline = ApiSnapshot::capture_current();
    }

    pub fn validate_compatibility(&self) -> Result<CompatibilityReport, ContractError> {
        let mut report = CompatibilityReport::new();

        // Check for removed public APIs
        for item in &self.baseline.public_items {
            if !self.current.public_items.contains(item) {
                report.add_breaking_change(BreakingChangeInfo {
                    change_type: ChangeType::Removal,
                    item_name: item.name.clone(),
                    item_type: item.item_type.clone(),
                    description: format!("Public API '{}' was removed", item.name),
                });
            }
        }

        // Check for signature changes
        for item in &self.current.public_items {
            if let Some(baseline_item) = self.baseline.find_item(&item.name) {
                if !self.signatures_compatible(&baseline_item.signature, &item.signature) {
                    report.add_breaking_change(BreakingChangeInfo {
                        change_type: ChangeType::SignatureChange,
                        item_name: item.name.clone(),
                        item_type: item.item_type.clone(),
                        description: format!("Signature changed for '{}'", item.name),
                    });
                }
            }
        }

        Ok(report)
    }

    fn signatures_compatible(&self, old: &Signature, new: &Signature) -> bool {
        // Implement signature compatibility checking
        // This is a complex topic - simplified for example
        old.name == new.name && old.visibility == new.visibility
    }
}

#[derive(Debug)]
pub struct ApiSnapshot {
    pub public_items: Vec<PublicItem>,
    pub timestamp: SystemTime,
}

impl ApiSnapshot {
    pub fn capture_current() -> Self {
        // Use rustdoc or similar to capture current public API
        Self {
            public_items: Vec::new(),
            timestamp: SystemTime::now(),
        }
    }

    pub fn find_item(&self, name: &str) -> Option<&PublicItem> {
        self.public_items.iter().find(|item| item.name == name)
    }
}

#[derive(Debug, Clone)]
pub struct PublicItem {
    pub name: String,
    pub item_type: ItemType,
    pub signature: Signature,
    pub visibility: Visibility,
}

#[derive(Debug, Clone)]
pub enum ItemType {
    Function,
    Struct,
    Enum,
    Trait,
    TypeAlias,
    Module,
}

#[derive(Debug, Clone)]
pub struct Signature {
    pub name: String,
    pub visibility: Visibility,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeInfo>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: TypeInfo,
}

#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub name: String,
    pub is_reference: bool,
    pub is_mutable: bool,
    pub generic_params: Vec<String>,
}
```

## Backward Compatibility

```rust
pub struct CompatibilityChecker {
    old_version: semver::Version,
    new_version: semver::Version,
    changes: Vec<Change>,
}

impl CompatibilityChecker {
    pub fn new(old: &str, new: &str) -> Self {
        Self {
            old_version: semver::Version::parse(old).unwrap(),
            new_version: semver::Version::parse(new).unwrap(),
            changes: Vec::new(),
        }
    }

    pub fn add_change(&mut self, change: Change) {
        self.changes.push(change);
    }

    pub fn should_increment_version(&self) -> VersionIncrement {
        let mut breaking = 0;
        let mut features = 0;
        let mut patches = 0;

        for change in &self.changes {
            match change.change_type {
                ChangeType::Breaking => breaking += 1,
                ChangeType::Feature => features += 1,
                ChangeType::Patch => patches += 1,
                _ => {}
            }
        }

        if breaking > 0 {
            VersionIncrement::Major
        } else if features > 0 {
            VersionIncrement::Minor
        } else if patches > 0 {
            VersionIncrement::Patch
        } else {
            VersionIncrement::None
        }
    }

    pub fn generate_migration_guide(&self) -> String {
        let mut guide = String::new();

        for change in &self.changes {
            if matches!(change.change_type, ChangeType::Breaking) {
                guide.push_str(&format!(
                    "## Breaking Change: {}\n\n{}\n\n",
                    change.description, change.migration_guide
                ));
            }
        }

        guide
    }
}

#[derive(Debug, Clone)]
pub struct Change {
    pub change_type: ChangeType,
    pub description: String,
    pub migration_guide: String,
    pub affected_items: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum ChangeType {
    Breaking,
    Feature,
    Patch,
    Documentation,
}

#[derive(Debug, Clone)]
pub enum VersionIncrement {
    Major,
    Minor,
    Patch,
    None,
}
```

## Integration with CI/CD

```rust
pub struct ContractCiIntegration {
    runner: ContractTestRunner,
    reporter: ContractReporter,
}

impl ContractCiIntegration {
    pub fn new() -> Self {
        Self {
            runner: ContractTestRunner::new(),
            reporter: ContractReporter::new(),
        }
    }

    pub fn run_ci_checks(&self) -> Result<CiResult, ContractError> {
        let results = self.runner.run_all_contracts()?;

        if results.is_success() {
            self.reporter.report_success(&results);
            Ok(CiResult::Pass)
        } else {
            self.reporter.report_failure(&results);
            Ok(CiResult::Fail)
        }
    }
}

pub enum CiResult {
    Pass,
    Fail,
}

pub struct ContractReporter {
    output_dir: PathBuf,
}

impl ContractReporter {
    pub fn new() -> Self {
        Self {
            output_dir: PathBuf::from("contract-reports"),
        }
    }

    pub fn report_success(&self, results: &ContractTestResults) {
        let report = self.generate_html_report(results, true);
        std::fs::write(self.output_dir.join("success.html"), report).ok();
    }

    pub fn report_failure(&self, results: &ContractTestResults) {
        let report = self.generate_html_report(results, false);
        std::fs::write(self.output_dir.join("failure.html"), report).ok();
    }

    fn generate_html_report(&self, results: &ContractTestResults, success: bool) -> String {
        format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head><title>Contract Test Report</title></head>
            <body>
                <h1>API Contract Test Report</h1>
                <p>Status: {}</p>
                <p>Total Contracts: {}</p>
                <p>Passed: {}</p>
                <p>Failed: {}</p>
            </body>
            </html>
            "#,
            if success { "SUCCESS" } else { "FAILURE" },
            results.total_contracts,
            results.passed_contracts,
            results.failed_contracts
        )
    }
}
```

## Testing Strategy

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_creation_works() {
        let contract = ApiContract::new("test_contract", "1.0.0");
        assert_eq!(contract.name, "test_contract");
        assert_eq!(contract.version.to_string(), "1.0.0");
    }

    #[test]
    fn contract_validation_passes() {
        let mut contract = ApiContract::new("test", "1.0.0");
        contract.add_test("basic_test", || Ok(()));

        assert!(contract.validate().is_ok());
    }

    #[test]
    fn contract_validation_fails() {
        let mut contract = ApiContract::new("test", "1.0.0");
        contract.add_test("failing_test", || Err(ContractError::TestFailed("Test failed".to_string())));

        assert!(contract.validate().is_err());
    }

    #[test]
    fn version_increment_detection() {
        let mut checker = CompatibilityChecker::new("1.0.0", "2.0.0");
        checker.add_change(Change {
            change_type: ChangeType::Breaking,
            description: "Breaking change".to_string(),
            migration_guide: "Migration guide".to_string(),
            affected_items: vec!["item1".to_string()],
        });

        assert!(matches!(checker.should_increment_version(), VersionIncrement::Major));
    }
}
```

## Performance Considerations

- **Snapshot Generation:** Cache API snapshots to avoid regeneration
- **Parallel Testing:** Run contract tests in parallel when possible
- **Incremental Validation:** Only validate changed APIs
- **Memory Management:** Clean up old snapshots and reports

## Future Extensions

- [ ] Visual diff tools for API changes
- [ ] Automatic migration guide generation
- [ ] Integration with semantic release
- [ ] Cross-language contract testing
