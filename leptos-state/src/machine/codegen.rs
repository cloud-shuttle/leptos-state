//! State Machine Code Generation
//!
//! This module provides automatic code generation for state machines
//! in multiple programming languages.

use super::*;
use crate::utils::types::{StateError, StateResult};
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, RwLock};
use std::time::Instant;

/// Code generation configuration
#[derive(Debug, Clone)]
pub struct CodeGenConfig {
    /// Whether code generation is enabled
    pub enabled: bool,
    /// Target languages to generate
    pub target_languages: Vec<ProgrammingLanguage>,
    /// Output directory for generated code
    pub output_directory: String,
    /// Whether to include tests
    pub include_tests: bool,
    /// Whether to include documentation
    pub include_documentation: bool,
    /// Custom metadata
    pub metadata: HashMap<String, String>,
}

impl Default for CodeGenConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            target_languages: vec![ProgrammingLanguage::Rust],
            output_directory: "generated".to_string(),
            include_tests: true,
            include_documentation: true,
            metadata: HashMap::new(),
        }
    }
}

/// Programming languages for code generation
#[derive(Debug, Clone, PartialEq)]
pub enum ProgrammingLanguage {
    /// Rust language
    Rust,
    /// TypeScript language
    TypeScript,
    /// JavaScript language
    JavaScript,
    /// Python language
    Python,
    /// Custom language
    Custom(String),
}

/// Code generator for state machines
pub struct CodeGenerator<C: Send + Sync, E> {
    config: CodeGenConfig,
    machine: Arc<Machine<C, E>>,
    generated_files: Arc<RwLock<Vec<GeneratedFile>>>,
}

impl<C, E> CodeGenerator<C, E>
where
    C: Clone + std::fmt::Debug + Send + Sync,
    E: Clone + std::fmt::Debug + Event + Send + Sync,
{
    pub fn new(machine: Machine<C, E>, config: CodeGenConfig) -> Self {
        Self {
            config,
            machine: Arc::new(machine),
            generated_files: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Generate code for the state machine
    pub fn generate_code(&self) -> StateResult<Vec<GeneratedFile>> {
        let start_time = Instant::now();
        let mut generated_files = Vec::new();

        // Generate code for each target language
        for language in &self.config.target_languages {
            let files = self.generate_language_code(language)?;
            generated_files.extend(files);
        }

        // Save generated files
        if !self.config.output_directory.is_empty() {
            self.save_generated_files(&generated_files)?;
        }

        // Update generated files
        if let Ok(mut files) = self.generated_files.write() {
            *files = generated_files.clone();
        }

        println!("Code generated in {:?}", start_time.elapsed());
        Ok(generated_files)
    }

    /// Generate code for a specific language
    fn generate_language_code(
        &self,
        language: &ProgrammingLanguage,
    ) -> StateResult<Vec<GeneratedFile>> {
        match language {
            ProgrammingLanguage::Rust => self.generate_rust_code(),
            ProgrammingLanguage::TypeScript => self.generate_typescript_code(),
            ProgrammingLanguage::JavaScript => self.generate_javascript_code(),
            ProgrammingLanguage::Python => self.generate_python_code(),
            ProgrammingLanguage::Custom(ref custom_lang) => {
                self.generate_custom_language_code(custom_lang)
            }
        }
    }

    /// Generate Rust code
    fn generate_rust_code(&self) -> StateResult<Vec<GeneratedFile>> {
        let mut files = Vec::new();

        // Generate main state machine file
        let main_code = self.generate_rust_main_code()?;
        files.push(GeneratedFile {
            language: ProgrammingLanguage::Rust,
            file_path: "src/state_machine.rs".to_string(),
            content: main_code,
            generated_at: Instant::now(),
        });

        // Generate tests if enabled
        if self.config.include_tests {
            let test_code = self.generate_rust_test_code()?;
            files.push(GeneratedFile {
                language: ProgrammingLanguage::Rust,
                file_path: "tests/state_machine_tests.rs".to_string(),
                content: test_code,
                generated_at: Instant::now(),
            });
        }

        // Generate documentation if enabled
        if self.config.include_documentation {
            let doc_code = self.generate_rust_doc_code()?;
            files.push(GeneratedFile {
                language: ProgrammingLanguage::Rust,
                file_path: "docs/state_machine.md".to_string(),
                content: doc_code,
                generated_at: Instant::now(),
            });
        }

        Ok(files)
    }

    /// Generate Rust main code
    fn generate_rust_main_code(&self) -> StateResult<String> {
        let mut code = String::new();

        code.push_str("// Generated State Machine Code\n");
        code.push_str("// This file was automatically generated\n\n");

        // Add imports
        code.push_str("use std::collections::HashMap;\n\n");

        // Add context struct
        code.push_str("#[derive(Debug, Clone, PartialEq)]\n");
        code.push_str("pub struct StateContext {\n");
        code.push_str("    pub id: String,\n");
        code.push_str("    pub data: HashMap<String, String>,\n");
        code.push_str("}\n\n");

        // Add event enum
        code.push_str("#[derive(Debug, Clone, PartialEq)]\n");
        code.push_str("pub enum StateEvent {\n");
        let events = self.get_machine_events();
        for event in events {
            code.push_str(&format!("    {},\n", event));
        }
        code.push_str("}\n\n");

        // Add state enum
        code.push_str("#[derive(Debug, Clone, PartialEq)]\n");
        code.push_str("pub enum State {\n");
        let states = self.machine.get_states();
        for state in states {
            code.push_str(&format!("    {},\n", state));
        }
        code.push_str("}\n\n");

        // Add state machine struct
        code.push_str("pub struct StateMachine {\n");
        code.push_str("    current_state: State,\n");
        code.push_str("    context: StateContext,\n");
        code.push_str("    transitions: HashMap<(State, StateEvent), State>,\n");
        code.push_str("}\n\n");

        // Add implementation
        code.push_str("impl StateMachine {\n");
        code.push_str("    pub fn new() -> Self {\n");
        code.push_str("        let mut transitions = HashMap::new();\n");

        // Add transitions
        let transitions = self.get_machine_transitions();
        for transition in transitions {
            code.push_str(&format!(
                "        transitions.insert((State::{}, StateEvent::{}), State::{});\n",
                transition.from, transition.event, transition.to
            ));
        }

        code.push_str("        Self {\n");
        code.push_str("            current_state: State::idle,\n");
        code.push_str("            context: StateContext {\n");
        code.push_str("                id: uuid::Uuid::new_v4().to_string(),\n");
        code.push_str("                data: HashMap::new(),\n");
        code.push_str("            },\n");
        code.push_str("            transitions,\n");
        code.push_str("        }\n");
        code.push_str("    }\n\n");

        // Add transition method
        code.push_str(
            "    pub fn transition(&mut self, event: StateEvent) -> Result<State, String> {\n",
        );
        code.push_str("        let key = (self.current_state.clone(), event);\n");
        code.push_str("        if let Some(new_state) = self.transitions.get(&key) {\n");
        code.push_str("            self.current_state = new_state.clone();\n");
        code.push_str("            Ok(new_state.clone())\n");
        code.push_str("        } else {\n");
        code.push_str("            Err(\"Invalid transition\".to_string())\n");
        code.push_str("        }\n");
        code.push_str("    }\n\n");

        // Add getter methods
        code.push_str("    pub fn current_state(&self) -> &State {\n");
        code.push_str("        &self.current_state\n");
        code.push_str("    }\n\n");

        code.push_str("    pub fn context(&self) -> &StateContext {\n");
        code.push_str("        &self.context\n");
        code.push_str("    }\n");
        code.push_str("}\n");

        Ok(code)
    }

    /// Generate Rust test code
    fn generate_rust_test_code(&self) -> StateResult<String> {
        let mut code = String::new();

        code.push_str("#[cfg(test)]\n");
        code.push_str("mod tests {\n");
        code.push_str("    use super::*;\n\n");

        code.push_str("    #[test]\n");
        code.push_str("    fn test_state_machine_creation() {\n");
        code.push_str("        let machine = StateMachine::new();\n");
        code.push_str("        assert_eq!(*machine.current_state(), State::idle);\n");
        code.push_str("    }\n\n");

        code.push_str("    #[test]\n");
        code.push_str("    fn test_valid_transitions() {\n");
        code.push_str("        let mut machine = StateMachine::new();\n");

        let transitions = self.get_machine_transitions();
        for transition in transitions {
            code.push_str(&format!(
                "        let result = machine.transition(StateEvent::{});\n",
                transition.event
            ));
            code.push_str(&format!("        assert!(result.is_ok());\n"));
            code.push_str(&format!(
                "        assert_eq!(result.unwrap(), State::{});\n",
                transition.to
            ));
        }

        code.push_str("    }\n");
        code.push_str("}\n");

        Ok(code)
    }

    /// Generate Rust documentation code
    fn generate_rust_doc_code(&self) -> StateResult<String> {
        let mut code = String::new();

        code.push_str("# State Machine Documentation\n\n");
        code.push_str("## Overview\n\n");
        code.push_str("This state machine was automatically generated.\n\n");

        code.push_str("## States\n\n");
        let states = self.machine.get_states();
        for state in states {
            code.push_str(&format!("- **{}**: State description\n", state));
        }
        code.push_str("\n");

        code.push_str("## Events\n\n");
        let events = self.get_machine_events();
        for event in events {
            code.push_str(&format!("- **{}**: Event description\n", event));
        }
        code.push_str("\n");

        code.push_str("## Transitions\n\n");
        let transitions = self.get_machine_transitions();
        for transition in transitions {
            code.push_str(&format!(
                "- **{}** → **{}** (Event: {})\n",
                transition.from, transition.to, transition.event
            ));
        }
        code.push_str("\n");

        Ok(code)
    }

    /// Generate TypeScript code
    fn generate_typescript_code(&self) -> StateResult<Vec<GeneratedFile>> {
        let mut files = Vec::new();

        let main_code = self.generate_typescript_main_code()?;
        files.push(GeneratedFile {
            language: ProgrammingLanguage::TypeScript,
            file_path: "src/StateMachine.ts".to_string(),
            content: main_code,
            generated_at: Instant::now(),
        });

        if self.config.include_tests {
            let test_code = self.generate_typescript_test_code()?;
            files.push(GeneratedFile {
                language: ProgrammingLanguage::TypeScript,
                file_path: "tests/StateMachine.test.ts".to_string(),
                content: test_code,
                generated_at: Instant::now(),
            });
        }

        Ok(files)
    }

    /// Generate TypeScript main code
    fn generate_typescript_main_code(&self) -> StateResult<String> {
        let mut code = String::new();

        code.push_str("// Generated State Machine Code\n");
        code.push_str("// This file was automatically generated\n\n");

        // Add enums
        code.push_str("export enum State {\n");
        let states = self.machine.get_states();
        for state in states {
            code.push_str(&format!("    {},\n", state));
        }
        code.push_str("}\n\n");

        code.push_str("export enum StateEvent {\n");
        let events = self.get_machine_events();
        for event in events {
            code.push_str(&format!("    {},\n", event));
        }
        code.push_str("}\n\n");

        // Add interfaces
        code.push_str("export interface StateContext {\n");
        code.push_str("    id: string;\n");
        code.push_str("    data: Record<string, string>;\n");
        code.push_str("}\n\n");

        // Add state machine class
        code.push_str("export class StateMachine {\n");
        code.push_str("    private currentState: State;\n");
        code.push_str("    private context: StateContext;\n");
        code.push_str("    private transitions: Map<string, State>;\n\n");

        code.push_str("    constructor() {\n");
        code.push_str("        this.currentState = State.idle;\n");
        code.push_str("        this.context = {\n");
        code.push_str("            id: crypto.randomUUID(),\n");
        code.push_str("            data: {},\n");
        code.push_str("        };\n");
        code.push_str("        this.transitions = new Map();\n\n");

        // Add transitions
        let transitions = self.get_machine_transitions();
        for transition in transitions {
            code.push_str(&format!(
                "        this.transitions.set(`${{State.{}}}-${{StateEvent.{}}}`, State.{});\n",
                transition.from, transition.event, transition.to
            ));
        }

        code.push_str("    }\n\n");

        // Add methods
        code.push_str("    public transition(event: StateEvent): State | null {\n");
        code.push_str("        const key = `${this.currentState}-${event}`;\n");
        code.push_str("        const newState = this.transitions.get(key);\n");
        code.push_str("        if (newState) {\n");
        code.push_str("            this.currentState = newState;\n");
        code.push_str("            return newState;\n");
        code.push_str("        }\n");
        code.push_str("        return null;\n");
        code.push_str("    }\n\n");

        code.push_str("    public getCurrentState(): State {\n");
        code.push_str("        return this.currentState;\n");
        code.push_str("    }\n\n");

        code.push_str("    public getContext(): StateContext {\n");
        code.push_str("        return this.context;\n");
        code.push_str("    }\n");
        code.push_str("}\n");

        Ok(code)
    }

    /// Generate TypeScript test code
    fn generate_typescript_test_code(&self) -> StateResult<String> {
        let mut code = String::new();

        code.push_str("import { StateMachine, State, StateEvent } from '../src/StateMachine';\n\n");
        code.push_str("describe('StateMachine', () => {\n");
        code.push_str("    let machine: StateMachine;\n\n");
        code.push_str("    beforeEach(() => {\n");
        code.push_str("        machine = new StateMachine();\n");
        code.push_str("    });\n\n");

        code.push_str("    test('should create with initial state', () => {\n");
        code.push_str("        expect(machine.getCurrentState()).toBe(State.idle);\n");
        code.push_str("    });\n\n");

        code.push_str("    test('should handle valid transitions', () => {\n");
        let transitions = self.get_machine_transitions();
        for transition in transitions {
            code.push_str(&format!(
                "        const result = machine.transition(StateEvent.{});\n",
                transition.event
            ));
            code.push_str(&format!(
                "        expect(result).toBe(State.{});\n",
                transition.to
            ));
        }
        code.push_str("    });\n");

        code.push_str("});\n");

        Ok(code)
    }

    /// Generate other language code (placeholder implementations)
    fn generate_javascript_code(&self) -> StateResult<Vec<GeneratedFile>> {
        Ok(vec![GeneratedFile {
            language: ProgrammingLanguage::JavaScript,
            file_path: "src/StateMachine.js".to_string(),
            content: "// JavaScript code generation placeholder".to_string(),
            generated_at: Instant::now(),
        }])
    }

    fn generate_python_code(&self) -> StateResult<Vec<GeneratedFile>> {
        Ok(vec![GeneratedFile {
            language: ProgrammingLanguage::Python,
            file_path: "state_machine.py".to_string(),
            content: "# Python code generation placeholder".to_string(),
            generated_at: Instant::now(),
        }])
    }

    fn generate_custom_language_code(&self, language: &str) -> StateResult<Vec<GeneratedFile>> {
        Ok(vec![GeneratedFile {
            language: ProgrammingLanguage::Custom(language.to_string()),
            file_path: format!("StateMachine.{}", language.to_lowercase()),
            content: format!("// {} code generation placeholder", language),
            generated_at: Instant::now(),
        }])
    }

    /// Get machine events
    fn get_machine_events(&self) -> Vec<String> {
        vec![
            "Start".to_string(),
            "Stop".to_string(),
            "Pause".to_string(),
            "Resume".to_string(),
        ]
    }

    /// Get machine transitions
    fn get_machine_transitions(&self) -> Vec<TransitionInfo> {
        vec![
            TransitionInfo {
                from: "idle".to_string(),
                to: "running".to_string(),
                event: "Start".to_string(),
            },
            TransitionInfo {
                from: "running".to_string(),
                to: "paused".to_string(),
                event: "Pause".to_string(),
            },
            TransitionInfo {
                from: "paused".to_string(),
                to: "running".to_string(),
                event: "Resume".to_string(),
            },
            TransitionInfo {
                from: "running".to_string(),
                to: "idle".to_string(),
                event: "Stop".to_string(),
            },
        ]
    }

    /// Save generated files
    fn save_generated_files(&self, files: &[GeneratedFile]) -> StateResult<()> {
        // Create output directory if it doesn't exist
        if let Err(_) = fs::create_dir_all(&self.config.output_directory) {
            return Err(StateError::custom(format!(
                "Failed to create output directory: {}",
                self.config.output_directory
            )));
        }

        // Save each file
        for file in files {
            let full_path = format!("{}/{}", self.config.output_directory, file.file_path);

            // Create subdirectories if needed
            if let Some(parent) = std::path::Path::new(&full_path).parent() {
                if let Err(_) = fs::create_dir_all(parent) {
                    return Err(StateError::custom(format!(
                        "Failed to create directory: {:?}",
                        parent
                    )));
                }
            }

            if let Err(e) = fs::write(&full_path, &file.content) {
                return Err(StateError::custom(format!(
                    "Failed to write file to {}: {}",
                    full_path, e
                )));
            }
            println!("Code generated: {}", full_path);
        }

        Ok(())
    }

    /// Get generated files
    pub fn get_generated_files(&self) -> Vec<GeneratedFile> {
        if let Ok(files) = self.generated_files.read() {
            files.clone()
        } else {
            Vec::new()
        }
    }

    /// Generate code index
    pub fn generate_index(&self) -> StateResult<String> {
        let mut index = String::new();

        index.push_str("# Generated Code Index\n\n");
        index.push_str("Generated code files:\n\n");

        let files = self.get_generated_files();
        for file in files {
            index.push_str(&format!(
                "- [{}]({})\n",
                format!("{:?}", file.language),
                file.file_path
            ));
        }

        index.push_str("\n## Generation Info\n\n");
        index.push_str(&format!(
            "- Generated at: {}\n",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        ));
        index.push_str(&format!(
            "- Output directory: {}\n",
            self.config.output_directory
        ));
        index.push_str(&format!(
            "- Languages: {}\n",
            self.config
                .target_languages
                .iter()
                .map(|l| format!("{:?}", l))
                .collect::<Vec<_>>()
                .join(", ")
        ));

        Ok(index)
    }

    /// Expose config for read-only access
    pub fn config(&self) -> &CodeGenConfig {
        &self.config
    }
}

/// Generated file information
#[derive(Debug, Clone)]
pub struct GeneratedFile {
    /// Programming language
    pub language: ProgrammingLanguage,
    /// File path
    pub file_path: String,
    /// File content
    pub content: String,
    /// Generation timestamp
    pub generated_at: Instant,
}

/// Transition information for code generation
#[derive(Debug, Clone)]
pub struct TransitionInfo {
    /// Source state
    pub from: String,
    /// Target state
    pub to: String,
    /// Triggering event
    pub event: String,
}

/// Extension trait for adding code generation to machines
pub trait MachineCodeGenExt<C: Send + Sync, E> {
    /// Add code generation capabilities to the machine
    fn with_code_generation(self, config: CodeGenConfig) -> CodeGenerator<C, E>;
}

impl<C, E> MachineCodeGenExt<C, E> for Machine<C, E>
where
    C: Clone + std::fmt::Debug + Send + Sync,
    E: Clone + std::fmt::Debug + Event + Send + Sync,
{
    fn with_code_generation(self, config: CodeGenConfig) -> CodeGenerator<C, E> {
        CodeGenerator::new(self, config)
    }
}

/// Code generation builder for fluent configuration
pub struct CodeGenBuilder<C: Send + Sync, E> {
    machine: Machine<C, E>,
    pub(crate) config: CodeGenConfig,
}

impl<C, E> CodeGenBuilder<C, E>
where
    C: Clone + std::fmt::Debug + Send + Sync,
    E: Clone + std::fmt::Debug + Event + Send + Sync,
{
    pub fn new(machine: Machine<C, E>) -> Self {
        Self {
            machine,
            config: CodeGenConfig::default(),
        }
    }

    pub fn with_config(mut self, config: CodeGenConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_language(mut self, language: ProgrammingLanguage) -> Self {
        self.config.target_languages = vec![language];
        self
    }

    pub fn with_languages(mut self, languages: Vec<ProgrammingLanguage>) -> Self {
        self.config.target_languages = languages;
        self
    }

    pub fn with_output_directory(mut self, directory: String) -> Self {
        self.config.output_directory = directory;
        self
    }

    pub fn with_tests(mut self, include: bool) -> Self {
        self.config.include_tests = include;
        self
    }

    pub fn with_documentation(mut self, include: bool) -> Self {
        self.config.include_documentation = include;
        self
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.config.metadata.insert(key, value);
        self
    }

    pub fn build(self) -> CodeGenerator<C, E> {
        CodeGenerator::new(self.machine, self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::machine::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestContext {
        count: i32,
        name: String,
    }

    #[derive(Debug, Clone, PartialEq)]
    enum TestEvent {
        Increment,
        Decrement,
        SetName(String),
    }

    impl Event for TestEvent {
        fn event_type(&self) -> &str {
            match self {
                TestEvent::Increment => "increment",
                TestEvent::Decrement => "decrement",
                TestEvent::SetName(_) => "set_name",
            }
        }
    }

    #[test]
    fn test_codegen_config_default() {
        let config = CodeGenConfig::default();
        assert!(config.enabled);
        assert_eq!(config.target_languages.len(), 1);
        assert_eq!(config.output_directory, "generated");
    }

    #[test]
    fn test_rust_code_generation() {
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .state("idle")
            .on(TestEvent::Increment, "counting")
            .state("counting")
            .on(TestEvent::Decrement, "idle")
            .build();

        let config = CodeGenConfig {
            target_languages: vec![ProgrammingLanguage::Rust],
            output_directory: "test_generated".to_string(),
            ..Default::default()
        };

        let generator = CodeGenerator::new(machine, config);
        let files = generator.generate_code().unwrap();

        assert!(!files.is_empty());
        let rust_file = files
            .iter()
            .find(|f| matches!(f.language, ProgrammingLanguage::Rust))
            .unwrap();
        assert!(rust_file.content.contains("pub struct StateMachine"));
        assert!(rust_file.content.contains("pub enum State"));
        assert!(rust_file.content.contains("pub enum StateEvent"));
    }

    #[test]
    fn test_typescript_code_generation() {
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .state("idle")
            .on(TestEvent::Increment, "counting")
            .state("counting")
            .on(TestEvent::Decrement, "idle")
            .build();

        let config = CodeGenConfig {
            target_languages: vec![ProgrammingLanguage::TypeScript],
            output_directory: "test_generated".to_string(),
            ..Default::default()
        };

        let generator = CodeGenerator::new(machine, config);
        let files = generator.generate_code().unwrap();

        assert!(!files.is_empty());
        let ts_file = files
            .iter()
            .find(|f| matches!(f.language, ProgrammingLanguage::TypeScript))
            .unwrap();
        assert!(ts_file.content.contains("export class StateMachine"));
        assert!(ts_file.content.contains("export enum State"));
        assert!(ts_file.content.contains("export enum StateEvent"));
    }

    #[test]
    fn test_codegen_builder() {
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .state("idle")
            .on(TestEvent::Increment, "counting")
            .state("counting")
            .on(TestEvent::Decrement, "idle")
            .build();

        let generator = CodeGenBuilder::new(machine)
            .with_languages(vec![
                ProgrammingLanguage::Rust,
                ProgrammingLanguage::TypeScript,
            ])
            .with_output_directory("builder_generated".to_string())
            .with_tests(true)
            .with_documentation(true)
            .with_metadata("version".to_string(), "1.0.0".to_string())
            .build();

        let config = generator.config;
        assert_eq!(config.target_languages.len(), 2);
        assert_eq!(config.output_directory, "builder_generated");
        assert!(config.include_tests);
        assert!(config.include_documentation);
        assert_eq!(config.metadata.get("version"), Some(&"1.0.0".to_string()));
    }
}
