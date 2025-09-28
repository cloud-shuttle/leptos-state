//! Core code generation functionality

use super::*;
use std::hash::Hash;

/// Code generator for state machines
pub struct CodeGenerator<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> {
    /// Configuration
    pub config: CodeGenConfig,
    /// Templates for code generation
    pub templates: CodeTemplates,
    /// Generated code cache
    pub generated_code: std::collections::HashMap<String, String>,
    /// Generation statistics
    pub stats: GenerationStats,
}

impl<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> CodeGenerator<C, E> {
    /// Create a new code generator
    pub fn new(config: CodeGenConfig) -> Self {
        let templates = CodeTemplates::for_language(config.language.clone());
        Self {
            config,
            templates,
            generated_code: std::collections::HashMap::new(),
            stats: GenerationStats::default(),
        }
    }

    /// Generate code for a state machine
    pub fn generate(&mut self, machine: &Machine<C, E, C>) -> Result<GeneratedFile, String> {
        let start_time = std::time::Instant::now();

        let machine_name = "GeneratedMachine".to_string(); // Could be configurable
        let mut code = String::new();

        // Add header
        if let Some(ref header) = self.config.custom_header {
            code.push_str(header);
            code.push('\n');
        }

        // Generate imports
        if self.config.include_imports {
            code.push_str(&self.generate_imports()?);
            code.push('\n');
        }

        // Generate machine structure
        code.push_str(&self.generate_machine_structure(machine, &machine_name)?);
        code.push('\n');

        // Generate state constants
        code.push_str(&self.generate_state_constants(machine)?);
        code.push('\n');

        // Generate transitions
        code.push_str(&self.generate_transitions(machine, &machine_name)?);
        code.push('\n');

        // Generate guards
        if self.config.include_validation {
            code.push_str(&self.generate_guards(machine, &machine_name)?);
            code.push('\n');
        }

        // Generate actions
        code.push_str(&self.generate_actions(machine, &machine_name)?);
        code.push('\n');

        // Generate events
        code.push_str(&self.generate_events(machine)?);
        code.push('\n');

        // Generate tests
        if self.config.generate_tests {
            code.push_str(&self.generate_tests(machine, &machine_name)?);
            code.push('\n');
        }

        // Add footer
        if let Some(ref footer) = self.config.custom_footer {
            code.push_str(footer);
            code.push('\n');
        }

        // Update statistics
        self.stats.total_files_generated += 1;
        self.stats.total_lines_generated += code.lines().count();
        self.stats.generation_time += start_time.elapsed();

        let file_name = self.config.file_pattern
            .replace("{machine_name}", &machine_name)
            .replace("{language}", &self.config.language.extension());

        Ok(GeneratedFile {
            file_name,
            content: code,
            language: self.config.language.clone(),
            generation_time: start_time.elapsed(),
            line_count: code.lines().count(),
        })
    }

    /// Generate imports for the target language
    fn generate_imports(&self) -> Result<String, String> {
        match self.config.language {
            ProgrammingLanguage::Rust => Ok(r#"use std::collections::HashMap;
use std::sync::{Arc, Mutex};
"#.to_string()),
            ProgrammingLanguage::TypeScript => Ok(r#"import { EventEmitter } from 'events';
"#.to_string()),
            ProgrammingLanguage::Python => Ok(r#"from typing import Dict, Any, Optional
from enum import Enum
"#.to_string()),
            _ => Ok(String::new()),
        }
    }

    /// Generate machine structure
    fn generate_machine_structure(&self, machine: &Machine<C, E, C>, machine_name: &str) -> Result<String, String> {
        let mut result = self.templates.machine_template.clone();

        result = result.replace("{machine_name}", machine_name);
        result = result.replace("{initial_state}", &machine.initial_state());

        Ok(result)
    }

    /// Generate state constants
    fn generate_state_constants(&self, machine: &Machine<C, E, C>) -> Result<String, String> {
        let mut result = String::new();

        for state_name in machine.get_states() {
            let mut state_code = self.templates.state_template.clone();
            state_code = state_code.replace("{state_name}", state_name);
            result.push_str(&state_code);
            result.push('\n');
        }

        Ok(result)
    }

    /// Generate transitions
    fn generate_transitions(&self, machine: &Machine<C, E, C>, machine_name: &str) -> Result<String, String> {
        let mut result = String::new();

        for state_name in machine.get_states() {
            if let Some(state_node) = machine.states_map().get(state_name) {
                for transition in &state_node.transitions {
                    let transition_name = format!("transition_{}_to_{}",
                        state_name.to_lowercase(),
                        transition.target.to_lowercase());

                    let mut transition_code = self.templates.transition_template.clone();
                    transition_code = transition_code.replace("{transition_name}", &transition_name);
                    transition_code = transition_code.replace("{from_state}", state_name);
                    transition_code = transition_code.replace("{to_state}", &transition.target);
                    transition_code = transition_code.replace("{event_param}", ""); // Simplified

                    result.push_str(&transition_code);
                    result.push('\n');
                }
            }
        }

        Ok(result)
    }

    /// Generate guards
    fn generate_guards(&self, machine: &Machine<C, E, C>, machine_name: &str) -> Result<String, String> {
        let mut result = String::new();

        // For now, generate simple placeholder guards
        result.push_str("// Guard implementations\n");

        for state_name in machine.get_states() {
            if let Some(state_node) = machine.states_map().get(state_name) {
                for (i, transition) in state_node.transitions.iter().enumerate() {
                    let guard_name = format!("guard_{}_transition_{}", state_name.to_lowercase(), i);

                    let mut guard_code = self.templates.guard_template.clone();
                    guard_code = guard_code.replace("{guard_name}", &guard_name);
                    guard_code = guard_code.replace("{context_param}", "&self.context");

                    result.push_str(&guard_code);
                    result.push('\n');
                }
            }
        }

        Ok(result)
    }

    /// Generate actions
    fn generate_actions(&self, machine: &Machine<C, E, C>, machine_name: &str) -> Result<String, String> {
        let mut result = String::new();

        // For now, generate simple placeholder actions
        result.push_str("// Action implementations\n");

        for state_name in machine.get_states() {
            if let Some(state_node) = machine.states_map().get(state_name) {
                // Entry actions
                if !state_node.entry_actions.is_empty() {
                    let action_name = format!("entry_actions_{}", state_name.to_lowercase());
                    let mut action_code = self.templates.action_template.clone();
                    action_code = action_code.replace("{action_name}", &action_name);
                    action_code = action_code.replace("{context_param}", "&mut self.context");
                    result.push_str(&action_code);
                    result.push('\n');
                }

                // Exit actions
                if !state_node.exit_actions.is_empty() {
                    let action_name = format!("exit_actions_{}", state_name.to_lowercase());
                    let mut action_code = self.templates.action_template.clone();
                    action_code = action_code.replace("{action_name}", &action_name);
                    action_code = action_code.replace("{context_param}", "&mut self.context");
                    result.push_str(&action_code);
                    result.push('\n');
                }
            }
        }

        Ok(result)
    }

    /// Generate events
    fn generate_events(&self, machine: &Machine<C, E, C>) -> Result<String, String> {
        let mut result = String::new();

        // For now, generate simple placeholder events
        result.push_str("// Event definitions\n");

        // We could analyze the machine to extract event types, but for now use placeholders
        let event_names = ["Event1", "Event2", "Event3"]; // Placeholder

        for event_name in &event_names {
            let mut event_code = self.templates.event_template.clone();
            event_code = event_code.replace("{event_name}", event_name);
            result.push_str(&event_code);
            result.push('\n');
        }

        Ok(result)
    }

    /// Generate tests
    fn generate_tests(&self, machine: &Machine<C, E, C>, machine_name: &str) -> Result<String, String> {
        match self.config.language {
            ProgrammingLanguage::Rust => self.generate_rust_tests(machine, machine_name),
            ProgrammingLanguage::TypeScript => self.generate_typescript_tests(machine, machine_name),
            ProgrammingLanguage::Python => self.generate_python_tests(machine, machine_name),
            _ => Ok("// Tests not supported for this language\n".to_string()),
        }
    }

    /// Generate Rust tests
    fn generate_rust_tests(&self, machine: &Machine<C, E, C>, machine_name: &str) -> Result<String, String> {
        let mut result = String::new();

        result.push_str("#[cfg(test)]\n");
        result.push_str("mod tests {\n");
        result.push_str("    use super::*;\n");
        result.push('\n');
        result.push_str("    #[test]\n");
        result.push_str("    fn test_machine_creation() {\n");
        result.push_str(&format!("        let machine = {}::new(Default::default());\n", machine_name));
        result.push_str(&format!("        assert_eq!(machine.current_state(), \"{}\");\n", machine.initial_state()));
        result.push_str("    }\n");
        result.push('\n');
        result.push_str("    #[test]\n");
        result.push_str("    fn test_basic_transitions() {\n");
        result.push_str(&format!("        let mut machine = {}::new(Default::default());\n", machine_name));
        result.push_str("        // Add transition tests here\n");
        result.push_str("    }\n");
        result.push_str("}\n");

        Ok(result)
    }

    /// Generate TypeScript tests
    fn generate_typescript_tests(&self, machine: &Machine<C, E, C>, machine_name: &str) -> Result<String, String> {
        let mut result = String::new();

        result.push_str("describe('");
        result.push_str(machine_name);
        result.push_str("', () => {\n");
        result.push_str("    it('should create machine correctly', () => {\n");
        result.push_str(&format!("        const machine = new {}(defaultContext);\n", machine_name));
        result.push_str(&format!("        expect(machine.getCurrentState()).toBe('{}');\n", machine.initial_state()));
        result.push_str("    });\n");
        result.push('\n');
        result.push_str("    it('should handle basic transitions', () => {\n");
        result.push_str(&format!("        const machine = new {}(defaultContext);\n", machine_name));
        result.push_str("        // Add transition tests here\n");
        result.push_str("    });\n");
        result.push_str("});\n");

        Ok(result)
    }

    /// Generate Python tests
    fn generate_python_tests(&self, machine: &Machine<C, E, C>, machine_name: &str) -> Result<String, String> {
        let mut result = String::new();

        result.push_str("import unittest\n");
        result.push('\n');
        result.push_str(&format!("class Test{}(unittest.TestCase):\n", machine_name));
        result.push_str("    def test_machine_creation(self):\n");
        result.push_str(&format!("        machine = {}(default_context)\n", machine_name));
        result.push_str(&format!("        self.assertEqual(machine.get_current_state(), '{}')\n", machine.initial_state()));
        result.push('\n');
        result.push_str("    def test_basic_transitions(self):\n");
        result.push_str(&format!("        machine = {}(default_context)\n", machine_name));
        result.push_str("        # Add transition tests here\n");
        result.push_str("        pass\n");
        result.push('\n');
        result.push_str("if __name__ == '__main__':\n");
        result.push_str("    unittest.main()\n");

        Ok(result)
    }

    /// Generate multiple files if configured
    pub fn generate_separate_files(&mut self, machine: &Machine<C, E, C>) -> Result<Vec<GeneratedFile>, String> {
        let mut files = Vec::new();

        if self.config.separate_files {
            // Generate main machine file
            let main_file = self.generate(machine)?;
            files.push(main_file);

            // Generate separate files for different components
            if self.config.generate_tests {
                let test_file = self.generate_test_file(machine)?;
                files.push(test_file);
            }
        } else {
            // Generate single file
            let single_file = self.generate(machine)?;
            files.push(single_file);
        }

        Ok(files)
    }

    /// Generate separate test file
    fn generate_test_file(&self, machine: &Machine<C, E, C>) -> Result<GeneratedFile, String> {
        let machine_name = "GeneratedMachine".to_string();
        let content = self.generate_tests(machine, &machine_name)?;

        let file_name = format!("{}_test.{}", machine_name.to_lowercase(),
                               self.config.language.extension());

        Ok(GeneratedFile {
            file_name,
            content,
            language: self.config.language.clone(),
            generation_time: std::time::Duration::from_nanos(0), // Not tracked
            line_count: content.lines().count(),
        })
    }

    /// Get generation statistics
    pub fn stats(&self) -> &GenerationStats {
        &self.stats
    }

    /// Clear generated code cache
    pub fn clear_cache(&mut self) {
        self.generated_code.clear();
    }
}

/// Generation statistics
#[derive(Debug, Clone, Default)]
pub struct GenerationStats {
    /// Total files generated
    pub total_files_generated: usize,
    /// Total lines of code generated
    pub total_lines_generated: usize,
    /// Total generation time
    pub generation_time: std::time::Duration,
    /// Languages used
    pub languages_used: std::collections::HashSet<ProgrammingLanguage>,
}

impl GenerationStats {
    /// Get average lines per file
    pub fn avg_lines_per_file(&self) -> f64 {
        if self.total_files_generated == 0 {
            0.0
        } else {
            self.total_lines_generated as f64 / self.total_files_generated as f64
        }
    }

    /// Get average generation time per file
    pub fn avg_generation_time(&self) -> std::time::Duration {
        if self.total_files_generated == 0 {
            std::time::Duration::from_nanos(0)
        } else {
            self.generation_time / self.total_files_generated as u32
        }
    }

    /// Add language usage
    pub fn add_language(&mut self, language: ProgrammingLanguage) {
        self.languages_used.insert(language);
    }
}
