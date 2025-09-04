//! # Migration Tools
//! 
//! This module provides tools to help migrate from v0.2.x to v1.0.0.

use super::traits::{StateMachineContext, StateMachineEvent, StateMachineState, StateMachine, StoreState, Store};
use super::machine::Machine;
use super::error::{StateMachineError, MigrationError};
use std::collections::HashMap;
use std::fmt::Debug;

/// Migration result
#[derive(Debug, Clone, PartialEq)]
pub enum MigrationResult {
    Success(String),
    Warning(String),
    Error(String),
    Skipped(String),
}

/// Migration report
#[derive(Debug, Clone)]
pub struct MigrationReport {
    /// Total items processed
    pub total_items: usize,
    /// Successful migrations
    pub successful: usize,
    /// Warnings
    pub warnings: usize,
    /// Errors
    pub errors: usize,
    /// Skipped items
    pub skipped: usize,
    /// Detailed results
    pub results: Vec<MigrationResult>,
    /// Migration duration
    pub duration: std::time::Duration,
}

/// Migration analyzer for detecting migration issues
pub struct MigrationAnalyzer {
    /// Issues found during analysis
    issues: Vec<MigrationIssue>,
    /// Migration suggestions
    suggestions: Vec<MigrationSuggestion>,
}

/// Migration issue
#[derive(Debug, Clone)]
pub struct MigrationIssue {
    /// Issue type
    pub issue_type: IssueType,
    /// Issue description
    pub description: String,
    /// File location (if available)
    pub location: Option<String>,
    /// Severity
    pub severity: IssueSeverity,
    /// Suggested fix
    pub suggested_fix: Option<String>,
}

/// Issue type
#[derive(Debug, Clone, PartialEq)]
pub enum IssueType {
    /// Trait bound issues
    TraitBound,
    /// API changes
    ApiChange,
    /// Feature flag issues
    FeatureFlag,
    /// Import issues
    Import,
    /// Type mismatch
    TypeMismatch,
    /// Missing implementation
    MissingImplementation,
}

/// Issue severity
#[derive(Debug, Clone, PartialEq)]
pub enum IssueSeverity {
    /// Critical - must be fixed
    Critical,
    /// High - should be fixed
    High,
    /// Medium - recommended to fix
    Medium,
    /// Low - nice to fix
    Low,
}

/// Migration suggestion
#[derive(Debug, Clone)]
pub struct MigrationSuggestion {
    /// Suggestion title
    pub title: String,
    /// Suggestion description
    pub description: String,
    /// Code example (before)
    pub before_code: Option<String>,
    /// Code example (after)
    pub after_code: Option<String>,
    /// Priority
    pub priority: SuggestionPriority,
}

/// Suggestion priority
#[derive(Debug, Clone, PartialEq)]
pub enum SuggestionPriority {
    /// Critical priority
    Critical,
    /// High priority
    High,
    /// Medium priority
    Medium,
    /// Low priority
    Low,
}

/// Code transformer for automatic migration
pub struct CodeTransformer {
    /// Transformation rules
    rules: Vec<TransformationRule>,
}

/// Transformation rule
#[derive(Debug, Clone)]
pub struct TransformationRule {
    /// Rule name
    pub name: String,
    /// Pattern to match
    pub pattern: String,
    /// Replacement pattern
    pub replacement: String,
    /// Rule description
    pub description: String,
}

impl MigrationAnalyzer {
    /// Create a new migration analyzer
    pub fn new() -> Self {
        Self {
            issues: Vec::new(),
            suggestions: Vec::new(),
        }
    }
    
    /// Analyze code for migration issues
    pub fn analyze_code(&mut self, code: &str) -> &mut Self {
        // Analyze for common v0.2.x patterns that need migration
        
        // Check for old MachineBuilder usage
        if code.contains("MachineBuilder::new()") {
            self.issues.push(MigrationIssue {
                issue_type: IssueType::ApiChange,
                description: "MachineBuilder::new() is deprecated in v1.0.0".to_string(),
                location: None,
                severity: IssueSeverity::High,
                suggested_fix: Some("Use Machine::new() instead".to_string()),
            });
        }
        
        // Check for old create_store! macro
        if code.contains("create_store!") {
            self.issues.push(MigrationIssue {
                issue_type: IssueType::ApiChange,
                description: "create_store! macro is deprecated in v1.0.0".to_string(),
                location: None,
                severity: IssueSeverity::High,
                suggested_fix: Some("Implement Store trait manually".to_string()),
            });
        }
        
        // Check for old use_machine usage
        if code.contains("use_machine(") && !code.contains("use_machine_with_context") {
            self.issues.push(MigrationIssue {
                issue_type: IssueType::ApiChange,
                description: "use_machine() signature changed in v1.0.0".to_string(),
                location: None,
                severity: IssueSeverity::High,
                suggested_fix: Some("Use use_machine_with_context() or update to new API".to_string()),
            });
        }
        
        // Check for missing trait bounds
        if code.contains("impl StateMachine") && !code.contains("StateMachineContext") {
            self.issues.push(MigrationIssue {
                issue_type: IssueType::TraitBound,
                description: "StateMachine implementations need proper trait bounds in v1.0.0".to_string(),
                location: None,
                severity: IssueSeverity::Critical,
                suggested_fix: Some("Add StateMachineContext, StateMachineEvent, and StateMachineState traits".to_string()),
            });
        }
        
        // Check for old error handling
        if code.contains("StateMachineError") && !code.contains("TransitionError") {
            self.issues.push(MigrationIssue {
                issue_type: IssueType::ApiChange,
                description: "Error types have been reorganized in v1.0.0".to_string(),
                location: None,
                severity: IssueSeverity::Medium,
                suggested_fix: Some("Update error handling to use new error types".to_string()),
            });
        }
        
        self
    }
    
    /// Get all issues found
    pub fn get_issues(&self) -> &[MigrationIssue] {
        &self.issues
    }
    
    /// Get all suggestions
    pub fn get_suggestions(&self) -> &[MigrationSuggestion] {
        &self.suggestions
    }
    
    /// Generate migration suggestions
    pub fn generate_suggestions(&mut self) -> &mut Self {
        // Generate suggestions based on issues found
        
        if self.issues.iter().any(|issue| issue.issue_type == IssueType::ApiChange) {
            self.suggestions.push(MigrationSuggestion {
                title: "Update API Usage".to_string(),
                description: "Several API calls have changed in v1.0.0. Review and update your code.".to_string(),
                before_code: Some("MachineBuilder::new()".to_string()),
                after_code: Some("Machine::new()".to_string()),
                priority: SuggestionPriority::High,
            });
        }
        
        if self.issues.iter().any(|issue| issue.issue_type == IssueType::TraitBound) {
            self.suggestions.push(MigrationSuggestion {
                title: "Add Required Trait Bounds".to_string(),
                description: "v1.0.0 requires explicit trait bounds for type safety.".to_string(),
                before_code: Some("impl StateMachine for MyState".to_string()),
                after_code: Some("impl StateMachine for MyState where MyState: StateMachineState".to_string()),
                priority: SuggestionPriority::Critical,
            });
        }
        
        self
    }
    
    /// Clear all issues and suggestions
    pub fn clear(&mut self) {
        self.issues.clear();
        self.suggestions.clear();
    }
}

impl CodeTransformer {
    /// Create a new code transformer
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
        }
    }
    
    /// Add a transformation rule
    pub fn add_rule(mut self, rule: TransformationRule) -> Self {
        self.rules.push(rule);
        self
    }
    
    /// Add default transformation rules
    pub fn with_default_rules(mut self) -> Self {
        self.rules.extend(vec![
            TransformationRule {
                name: "MachineBuilder to Machine".to_string(),
                pattern: r"MachineBuilder::new\(\)".to_string(),
                replacement: "Machine::new()".to_string(),
                description: "Replace MachineBuilder with Machine".to_string(),
            },
            TransformationRule {
                name: "Update use_machine import".to_string(),
                pattern: r"use leptos_state::use_machine;".to_string(),
                replacement: "use leptos_state::{use_machine, use_machine_with_context};".to_string(),
                description: "Update use_machine import".to_string(),
            },
            TransformationRule {
                name: "Add v1 module import".to_string(),
                pattern: r"use leptos_state::".to_string(),
                replacement: "use leptos_state::v1::*;\nuse leptos_state::".to_string(),
                description: "Add v1 module import".to_string(),
            },
        ]);
        self
    }
    
    /// Transform code using the rules
    pub fn transform(&self, code: &str) -> String {
        let mut transformed = code.to_string();
        
        for rule in &self.rules {
            // Simple string replacement for now
            // In a real implementation, this would use proper AST parsing
            transformed = transformed.replace(&rule.pattern, &rule.replacement);
        }
        
        transformed
    }
    
    /// Get all transformation rules
    pub fn get_rules(&self) -> &[TransformationRule] {
        &self.rules
    }
}

/// Migration helper for common patterns
pub struct MigrationHelper;

impl MigrationHelper {
    /// Generate migration guide
    pub fn generate_migration_guide() -> String {
        r#"
# Migration Guide: v0.2.x to v1.0.0

## Overview
This guide helps you migrate from leptos-state v0.2.x to v1.0.0.

## Breaking Changes

### 1. Trait Bounds
v1.0.0 requires explicit trait bounds for type safety:

```rust
// v0.2.x
impl StateMachine for MyState {
    // ...
}

// v1.0.0
impl StateMachine for MyState 
where 
    MyState: StateMachineState<Context = MyContext, Event = MyEvent>
{
    // ...
}
```

### 2. Machine Creation
Machine creation has changed:

```rust
// v0.2.x
let machine = MachineBuilder::new()
    .state("idle")
    .on(Event::Start, "active")
    .build();

// v1.0.0
let machine = Machine::new(MyState::Idle, MyContext::default());
```

### 3. Store Implementation
Store implementation is now trait-based:

```rust
// v0.2.x
create_store! {
    MyStore {
        count: i32,
        name: String,
    }
}

// v1.0.0
#[derive(Clone, PartialEq, Debug, Default)]
struct MyStore {
    count: i32,
    name: String,
}

impl StoreState for MyStore {}
impl Store for MyStore {
    fn create() -> Self {
        Self::default()
    }
    // ... other methods
}
```

### 4. Hooks Usage
Hook usage has been updated:

```rust
// v0.2.x
let (state, send) = use_machine(machine);

// v1.0.0
let machine = use_machine_with_context(MyState::Idle, MyContext::default());
```

## Migration Steps

1. **Update imports**: Add `use leptos_state::v1::*;`
2. **Add trait bounds**: Implement required traits for your types
3. **Update machine creation**: Use new Machine API
4. **Update store implementation**: Use trait-based approach
5. **Update hooks**: Use new hook signatures
6. **Test thoroughly**: Ensure all functionality works

## Tools

Use the migration tools provided in this module:
- `MigrationAnalyzer`: Analyze your code for issues
- `CodeTransformer`: Automatically transform common patterns
- `MigrationHelper`: Get migration guidance

## Support

For questions or issues, please refer to the documentation or open an issue.
"#.to_string()
    }
    
    /// Generate example migration
    pub fn generate_example_migration() -> (String, String) {
        let before = r#"
// v0.2.x code
use leptos_state::{MachineBuilder, use_machine, create_store};

create_store! {
    CounterStore {
        count: i32,
        name: String,
    }
}

#[derive(Clone, Debug)]
enum CounterEvent {
    Increment,
    Decrement,
}

fn Counter() -> impl IntoView {
    let machine = MachineBuilder::new()
        .state("idle")
        .on(CounterEvent::Increment, "active")
        .build();
    
    let (state, send) = use_machine(machine);
    
    view! {
        <div>
            <h2>"Counter: " {state.value()}</h2>
            <button on:click=move |_| send(CounterEvent::Increment)>
                "Increment"
            </button>
        </div>
    }
}
"#.to_string();
        
        let after = r#"
// v1.0.0 code
use leptos_state::v1::*;
use leptos_state::{use_machine_with_context};

#[derive(Clone, PartialEq, Debug, Default)]
struct CounterStore {
    count: i32,
    name: String,
}

impl StoreState for CounterStore {}
impl Store for CounterStore {
    fn create() -> Self {
        Self::default()
    }
    // ... other methods
}

#[derive(Clone, Debug, PartialEq, Default)]
enum CounterEvent {
    #[default]
    Increment,
    Decrement,
}

impl StateMachineEvent for CounterEvent {}

#[derive(Clone, Debug, PartialEq, Default)]
enum CounterState {
    #[default]
    Idle,
    Active,
}

impl StateMachineState for CounterState {
    type Context = CounterStore;
    type Event = CounterEvent;
}

impl StateMachine for CounterState {
    fn initial_state(&self) -> Self {
        CounterState::Idle
    }
    // ... other methods
}

fn Counter() -> impl IntoView {
    let initial_context = CounterStore::default();
    let machine = use_machine_with_context(CounterState::Idle, initial_context);
    
    view! {
        <div>
            <h2>"Counter: " {machine.state()}</h2>
            <button on:click=move |_| machine.send(CounterEvent::Increment)>
                "Increment"
            </button>
        </div>
    }
}
"#.to_string();
        
        (before, after)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_migration_analyzer_creation() {
        let analyzer = MigrationAnalyzer::new();
        
        assert_eq!(analyzer.get_issues().len(), 0);
        assert_eq!(analyzer.get_suggestions().len(), 0);
    }
    
    #[test]
    fn test_migration_analyzer_analyze_code() {
        let mut analyzer = MigrationAnalyzer::new();
        
        let code = r#"
        let machine = MachineBuilder::new()
            .state("idle")
            .build();
        "#;
        
        analyzer.analyze_code(code);
        
        assert!(analyzer.get_issues().len() > 0);
        assert!(analyzer.get_issues().iter().any(|issue| 
            issue.issue_type == IssueType::ApiChange
        ));
    }
    
    #[test]
    fn test_migration_analyzer_generate_suggestions() {
        let mut analyzer = MigrationAnalyzer::new();
        
        let code = r#"
        let machine = MachineBuilder::new()
            .state("idle")
            .build();
        "#;
        
        analyzer.analyze_code(code).generate_suggestions();
        
        assert!(analyzer.get_suggestions().len() > 0);
    }
    
    #[test]
    fn test_code_transformer_creation() {
        let transformer = CodeTransformer::new();
        
        assert_eq!(transformer.get_rules().len(), 0);
    }
    
    #[test]
    fn test_code_transformer_with_default_rules() {
        let transformer = CodeTransformer::new().with_default_rules();
        
        assert!(transformer.get_rules().len() > 0);
    }
    
    #[test]
    fn test_code_transformer_transform() {
        let transformer = CodeTransformer::new()
            .add_rule(TransformationRule {
                name: "Test rule".to_string(),
                pattern: "old_pattern".to_string(),
                replacement: "new_pattern".to_string(),
                description: "Test transformation".to_string(),
            });
        
        let code = "This is old_pattern code";
        let transformed = transformer.transform(code);
        
        assert!(transformed.contains("new_pattern"));
        assert!(!transformed.contains("old_pattern"));
    }
    
    #[test]
    fn test_migration_helper_generate_guide() {
        let guide = MigrationHelper::generate_migration_guide();
        
        assert!(guide.contains("Migration Guide"));
        assert!(guide.contains("v0.2.x"));
        assert!(guide.contains("v1.0.0"));
    }
    
    #[test]
    fn test_migration_helper_generate_example() {
        let (before, after) = MigrationHelper::generate_example_migration();
        
        assert!(before.contains("v0.2.x"));
        assert!(after.contains("v1.0.0"));
        assert!(before.contains("MachineBuilder"));
        assert!(after.contains("use_machine_with_context"));
    }
    
    #[test]
    fn test_migration_result_creation() {
        let success = MigrationResult::Success("Migration completed".to_string());
        let warning = MigrationResult::Warning("Minor issue found".to_string());
        let error = MigrationResult::Error("Migration failed".to_string());
        let skipped = MigrationResult::Skipped("Item skipped".to_string());
        
        assert_eq!(success, MigrationResult::Success("Migration completed".to_string()));
        assert_eq!(warning, MigrationResult::Warning("Minor issue found".to_string()));
        assert_eq!(error, MigrationResult::Error("Migration failed".to_string()));
        assert_eq!(skipped, MigrationResult::Skipped("Item skipped".to_string()));
    }
    
    #[test]
    fn test_migration_report_creation() {
        let report = MigrationReport {
            total_items: 10,
            successful: 8,
            warnings: 1,
            errors: 1,
            skipped: 0,
            results: vec![
                MigrationResult::Success("Item 1".to_string()),
                MigrationResult::Warning("Item 2".to_string()),
            ],
            duration: std::time::Duration::from_secs(5),
        };
        
        assert_eq!(report.total_items, 10);
        assert_eq!(report.successful, 8);
        assert_eq!(report.warnings, 1);
        assert_eq!(report.errors, 1);
        assert_eq!(report.skipped, 0);
    }
    
    #[test]
    fn test_issue_type_enum() {
        let trait_bound = IssueType::TraitBound;
        let api_change = IssueType::ApiChange;
        let feature_flag = IssueType::FeatureFlag;
        
        assert_eq!(trait_bound, IssueType::TraitBound);
        assert_eq!(api_change, IssueType::ApiChange);
        assert_eq!(feature_flag, IssueType::FeatureFlag);
    }
    
    #[test]
    fn test_issue_severity_enum() {
        let critical = IssueSeverity::Critical;
        let high = IssueSeverity::High;
        let medium = IssueSeverity::Medium;
        let low = IssueSeverity::Low;
        
        assert_eq!(critical, IssueSeverity::Critical);
        assert_eq!(high, IssueSeverity::High);
        assert_eq!(medium, IssueSeverity::Medium);
        assert_eq!(low, IssueSeverity::Low);
    }
    
    #[test]
    fn test_suggestion_priority_enum() {
        let high = SuggestionPriority::High;
        let medium = SuggestionPriority::Medium;
        let low = SuggestionPriority::Low;
        
        assert_eq!(high, SuggestionPriority::High);
        assert_eq!(medium, SuggestionPriority::Medium);
        assert_eq!(low, SuggestionPriority::Low);
    }
}
