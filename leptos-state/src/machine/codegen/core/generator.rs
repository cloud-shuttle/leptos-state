//! Core code generator for state machines

use crate::machine::codegen::config::{CodeGenConfig, CodeTemplates};
use crate::machine::codegen::types::GeneratedFile;
use crate::machine::{Machine, MachineStateImpl};

/// Code generator for state machines
pub struct CodeGenerator<
    C: Send + Sync + Clone + PartialEq + 'static,
    E: Clone + Send + Sync + std::hash::Hash + Eq + 'static,
> {
    /// Configuration
    pub config: CodeGenConfig,
    /// Templates for code generation
    pub templates: CodeTemplates,
    /// Generated code cache
    pub generated_code: std::collections::HashMap<String, String>,
    /// Generation statistics
    pub stats: super::stats::GenerationStats,
    /// Phantom data for unused type parameter
    pub _phantom: std::marker::PhantomData<C>,
}

impl<
        C: Send + Sync + Clone + PartialEq + 'static,
        E: Clone + Send + Sync + std::hash::Hash + Eq + 'static,
    > CodeGenerator<C, E>
{
    /// Create a new code generator
    pub fn new(config: CodeGenConfig) -> Self {
        let templates = CodeTemplates::for_language(config.language.clone());
        Self {
            config,
            templates,
            generated_code: std::collections::HashMap::new(),
            stats: super::stats::GenerationStats::default(),
            _phantom: std::marker::PhantomData,
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
        code.push_str(&self.generate_imports(&machine_name)?);
        code.push('\n');

        // Generate machine structure
        code.push_str(&self.generate_machine_structure(machine, &machine_name)?);
        code.push('\n');

        // Generate state constants
        code.push_str(&self.generate_state_constants(machine, &machine_name)?);
        code.push('\n');

        // Generate transitions
        code.push_str(&self.generate_transitions(machine, &machine_name)?);
        code.push('\n');

        // Generate guards if enabled
        if self.config.include_guards {
            code.push_str(&self.generate_guards(machine, &machine_name)?);
            code.push('\n');
        }

        // Generate actions if enabled
        if self.config.include_actions {
            code.push_str(&self.generate_actions(machine, &machine_name)?);
            code.push('\n');
        }

        // Generate events
        code.push_str(&self.generate_events(machine, &machine_name)?);
        code.push('\n');

        // Generate tests if enabled
        if self.config.generate_tests {
            code.push_str(&self.generate_tests(machine, &machine_name)?);
            code.push('\n');
        }

        let generation_time = start_time.elapsed();
        self.stats.record_generation(code.lines().count(), generation_time);

        // Cache the generated code
        self.generated_code.insert(machine_name.clone(), code.clone());

        Ok(GeneratedFile {
            filename: format!("{}.rs", machine_name),
            content: code,
            language: self.config.language.clone(),
            machine_name,
            generation_time,
            line_count: code.lines().count(),
        })
    }

    /// Get generation statistics
    pub fn stats(&self) -> &super::stats::GenerationStats {
        &self.stats
    }

    /// Clear generated code cache
    pub fn clear_cache(&mut self) {
        self.generated_code.clear();
        self.stats = super::stats::GenerationStats::default();
    }

    /// Get cached code for a machine
    pub fn get_cached_code(&self, machine_name: &str) -> Option<&String> {
        self.generated_code.get(machine_name)
    }

    /// Check if code is cached for a machine
    pub fn is_cached(&self, machine_name: &str) -> bool {
        self.generated_code.contains_key(machine_name)
    }

    /// Get cache size
    pub fn cache_size(&self) -> usize {
        self.generated_code.len()
    }

    /// Get configuration
    pub fn config(&self) -> &CodeGenConfig {
        &self.config
    }

    /// Update configuration
    pub fn update_config(&mut self, config: CodeGenConfig) {
        self.config = config;
        self.templates = CodeTemplates::for_language(self.config.language.clone());
    }
}

impl<
        C: Send + Sync + Clone + PartialEq + 'static,
        E: Clone + Send + Sync + std::hash::Hash + Eq + 'static,
    > std::fmt::Debug for CodeGenerator<C, E>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CodeGenerator")
            .field("language", &self.config.language)
            .field("cache_size", &self.cache_size())
            .field("stats", &self.stats)
            .finish()
    }
}

impl<
        C: Send + Sync + Clone + PartialEq + 'static,
        E: Clone + Send + Sync + std::hash::Hash + Eq + 'static,
    > std::fmt::Display for CodeGenerator<C, E>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CodeGenerator({} language, {} cached, {} total lines)",
            self.config.language.as_str(),
            self.cache_size(),
            self.stats.total_lines_generated
        )
    }
}
