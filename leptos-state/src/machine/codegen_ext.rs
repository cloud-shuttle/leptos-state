//! Extension traits for code generation

use super::*;

/// Extension trait for adding code generation to machines
pub trait MachineCodeGenExt<
    C: Send + Sync + Clone + 'static,
    E: Send + Clone + 'static,
>
{
    /// Generate code for this machine
    fn generate_code(&self, config: CodeGenConfig) -> Result<GeneratedFile, String>;

    /// Generate code with custom templates
    fn generate_code_with_templates(
        &self,
        config: CodeGenConfig,
        templates: CodeTemplates,
    ) -> Result<GeneratedFile, String>;

    /// Generate code in multiple languages
    fn generate_multi_language(
        &self,
        configs: Vec<CodeGenConfig>,
    ) -> Result<Vec<GeneratedFile>, String>;

    /// Generate code and save to files
    fn generate_and_save(
        &self,
        config: CodeGenConfig,
        output_dir: &std::path::Path,
    ) -> Result<Vec<std::path::PathBuf>, String>;
}

impl<
        C: Send + Sync + Clone + std::fmt::Debug + 'static,
        E: Send + Clone + std::fmt::Debug + PartialEq + 'static,
    > MachineCodeGenExt<C, E> for Machine<C, E, C>
{
    fn generate_code(&self, config: CodeGenConfig) -> Result<GeneratedFile, String> {
        let mut generator: CodeGenerator<C, E> = CodeGenerator::new(config);
        let result: Result<GeneratedFile, String> = generator.generate(self);
        result
    }

    fn generate_code_with_templates(
        &self,
        config: CodeGenConfig,
        templates: CodeTemplates,
    ) -> Result<GeneratedFile, String> {
        let mut generator: CodeGenerator<C, E> = CodeGenerator::new(config);
        generator.templates = templates;
        let result: Result<GeneratedFile, String> = generator.generate(self);
        result
    }

    fn generate_multi_language(
        &self,
        configs: Vec<CodeGenConfig>,
    ) -> Result<Vec<GeneratedFile>, String> {
        let mut results: Vec<GeneratedFile> = Vec::new();

        for config in configs {
            let mut generator: CodeGenerator<C, E> = CodeGenerator::new(config);
            let file: GeneratedFile = generator.generate(self)?;
            results.push(file);
        }

        Ok(results)
    }

    fn generate_and_save(
        &self,
        config: CodeGenConfig,
        output_dir: &std::path::Path,
    ) -> Result<Vec<std::path::PathBuf>, String> {
        let mut generator: CodeGenerator<C, E> = CodeGenerator::new(config);
        let files: Vec<GeneratedFile> = generator.generate_separate_files(self)?;

        let mut saved_paths = Vec::new();
        for file in files {
            let path = file.save_to_file(output_dir)?;
            saved_paths.push(path);
        }

        Ok(saved_paths)
    }
}

/// Extension trait for state machine builders
pub trait MachineBuilderCodeGenExt<
    C: Send + Sync + Clone + 'static,
    E: Send + Clone + 'static,
>
{
    /// Build and generate code
    fn build_and_generate(
        self,
        config: CodeGenConfig,
    ) -> Result<(Machine<C, E, C>, GeneratedFile), String>;
}

impl<
        C: Send + Sync + Clone + std::fmt::Debug + 'static,
        E: Send + Clone + Sync + std::fmt::Debug + PartialEq + 'static,
    > MachineBuilderCodeGenExt<C, E> for crate::machine::MachineBuilder<C, E>
{
    fn build_and_generate(
        self,
        config: CodeGenConfig,
    ) -> Result<(Machine<C, E, C>, GeneratedFile), String> {
        let machine = self.build()?;
        let generated = machine.generate_code(config)?;
        Ok((machine, generated))
    }
}

/// Fluent API for code generation
pub mod codegen {
    use super::*;

    /// Create a code generator for Rust
    pub fn rust<
        C: Send + Sync + Clone + std::fmt::Debug + 'static,
        E: Send + Clone + std::fmt::Debug + PartialEq + 'static,
    >() -> CodeGenerator<C, E> {
        let config = CodeGenConfig {
            language: ProgrammingLanguage::Rust,
            ..Default::default()
        };
        CodeGenerator::new(config)
    }

    /// Create a code generator for TypeScript
    pub fn typescript<
        C: Send + Sync + Clone + std::fmt::Debug + 'static,
        E: Send + Clone + std::fmt::Debug + PartialEq + 'static,
    >() -> CodeGenerator<C, E> {
        let config = CodeGenConfig {
            language: ProgrammingLanguage::TypeScript,
            ..Default::default()
        };
        CodeGenerator::new(config)
    }

    /// Create a code generator for Python
    pub fn python<
        C: Send + Sync + Clone + std::fmt::Debug + 'static,
        E: Send + Clone + std::fmt::Debug + PartialEq + 'static,
    >() -> CodeGenerator<C, E> {
        let config = CodeGenConfig {
            language: ProgrammingLanguage::Python,
            ..Default::default()
        };
        CodeGenerator::new(config)
    }

    /// Create a code generator with custom config
    pub fn with_config<
        C: Send + Sync + Clone + std::fmt::Debug + 'static,
        E: Send + Clone + std::fmt::Debug + PartialEq + 'static,
    >(
        config: CodeGenConfig,
    ) -> CodeGenerator<C, E> {
        CodeGenerator::new(config)
    }

    /// Generate code for a machine
    pub fn generate<
        C: Send + Sync + Clone + std::fmt::Debug + 'static,
        E: Send + Clone + std::fmt::Debug + PartialEq + 'static,
    >(
        machine: &Machine<C, E, C>,
        config: CodeGenConfig,
    ) -> Result<GeneratedFile, String> {
        machine.generate_code(config)
    }

    /// Generate code in multiple languages
    pub fn generate_multi<
        C: Send + Sync + Clone + std::fmt::Debug + 'static,
        E: Send + Clone + std::fmt::Debug + PartialEq + 'static,
    >(
        machine: &Machine<C, E, C>,
        languages: Vec<ProgrammingLanguage>,
    ) -> Result<Vec<GeneratedFile>, String> {
        let configs = languages
            .into_iter()
            .map(|lang| CodeGenConfig {
                language: lang,
                ..Default::default()
            })
            .collect();

        machine.generate_multi_language(configs)
    }

    /// Generate and save code
    pub fn generate_and_save<
        C: Send + Sync + Clone + std::fmt::Debug + 'static,
        E: Send + Clone + std::fmt::Debug + PartialEq + 'static,
    >(
        machine: &Machine<C, E, C>,
        config: CodeGenConfig,
        output_dir: &std::path::Path,
    ) -> Result<Vec<std::path::PathBuf>, String> {
        machine.generate_and_save(config, output_dir)
    }

    /// Create templates for a language
    pub fn templates_for(language: ProgrammingLanguage) -> CodeTemplates {
        CodeTemplates::for_language(language)
    }

    /// Create custom templates
    pub fn custom_templates(
        language: ProgrammingLanguage,
        machine_template: String,
    ) -> CodeTemplates {
        CodeTemplates {
            language,
            machine_template,
            state_template: String::new(),
            transition_template: String::new(),
            guard_template: String::new(),
            action_template: String::new(),
            event_template: String::new(),
        }
    }
}

/// Code generation pipeline for complex workflows
pub struct CodeGenPipeline<
    C: Send + Sync + Clone + PartialEq + 'static,
    E: Clone + Send + Sync + 'static,
> {
    /// Pipeline steps
    pub steps: Vec<Box<dyn CodeGenStep<C, E>>>,
    /// Pipeline configuration
    pub config: PipelineConfig,
}

#[derive(Debug, Clone)]
pub struct PipelineConfig {
    /// Whether to stop on first error
    pub stop_on_error: bool,
    /// Whether to continue processing even if steps fail
    pub continue_on_failure: bool,
    /// Output directory for generated files
    pub output_dir: Option<std::path::PathBuf>,
    /// Whether to overwrite existing files
    pub overwrite_existing: bool,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            stop_on_error: true,
            continue_on_failure: false,
            output_dir: None,
            overwrite_existing: true,
        }
    }
}

impl<
        C: Send + Sync + Clone + std::fmt::Debug + 'static,
        E: Send + Clone + std::fmt::Debug + PartialEq + 'static,
    > CodeGenPipeline<C, E>
{
    /// Create a new pipeline
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            config: PipelineConfig::default(),
        }
    }

    /// Add a step to the pipeline
    pub fn add_step(&mut self, step: Box<dyn CodeGenStep<C, E>>) {
        self.steps.push(step);
    }

    /// Set pipeline configuration
    pub fn with_config(mut self, config: PipelineConfig) -> Self {
        self.config = config;
        self
    }

    /// Execute the pipeline
    pub fn execute(&self, machine: &Machine<C, E, C>) -> Result<PipelineResult, String> {
        let mut results = Vec::new();
        let mut errors = Vec::new();

        for step in &self.steps {
            match step.execute(machine, &self.config) {
                Ok(result) => {
                    results.push(result);
                }
                Err(error) => {
                    errors.push(error.clone());

                    if self.config.stop_on_error {
                        return Err(error);
                    }
                }
            }
        }

        Ok(PipelineResult {
            successful_steps: results.len(),
            failed_steps: errors.len(),
            results,
            errors,
            total_execution_time: results.iter().map(|r| r.generation_time).sum(),
        })
    }
}

/// Code generation step trait
pub trait CodeGenStep<
    C: Send + Sync + Clone + std::fmt::Debug + 'static,
    E: Send + Clone + std::fmt::Debug + PartialEq + 'static,
>
{
    /// Execute this step
    fn execute(
        &self,
        machine: &Machine<C, E, C>,
        config: &PipelineConfig,
    ) -> Result<GeneratedFile, String>;

    /// Get step name
    fn name(&self) -> &str;

    /// Get step description
    fn description(&self) -> &str {
        ""
    }
}

/// Pipeline execution result
#[derive(Debug, Clone)]
pub struct PipelineResult {
    /// Number of successful steps
    pub successful_steps: usize,
    /// Number of failed steps
    pub failed_steps: usize,
    /// Results from successful steps
    pub results: Vec<GeneratedFile>,
    /// Errors from failed steps
    pub errors: Vec<String>,
    /// Total execution time
    pub total_execution_time: std::time::Duration,
}

impl PipelineResult {
    /// Check if pipeline execution was successful
    pub fn is_successful(&self) -> bool {
        self.failed_steps == 0
    }

    /// Get success rate as percentage
    pub fn success_rate(&self) -> f64 {
        let total = self.successful_steps + self.failed_steps;
        if total == 0 {
            0.0
        } else {
            self.successful_steps as f64 / total as f64 * 100.0
        }
    }

    /// Get all generated files
    pub fn all_files(&self) -> &[GeneratedFile] {
        &self.results
    }

    /// Get total lines generated
    pub fn total_lines(&self) -> usize {
        self.results.iter().map(|f| f.line_count).sum()
    }

    /// Save all generated files
    pub fn save_all(&self, base_dir: &std::path::Path) -> Result<Vec<std::path::PathBuf>, String> {
        let mut saved_paths = Vec::new();

        for file in &self.results {
            let path = file.save_to_file(base_dir)?;
            saved_paths.push(path);
        }

        Ok(saved_paths)
    }
}
