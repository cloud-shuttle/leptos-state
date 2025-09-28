//! Builder pattern for code generation

use super::*;
use std::hash::Hash;

/// Code generation builder for fluent configuration
pub struct CodeGenBuilder<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> {
    /// Configuration
    pub config: CodeGenConfig,
    /// Templates
    pub templates: Option<CodeTemplates>,
    /// Custom options
    pub options: CodeGenOptions,
    /// Pre-generation hooks
    pub pre_hooks: Vec<Box<dyn Fn(&mut CodeGenContext) + Send + Sync>>,
    /// Post-generation hooks
    pub post_hooks: Vec<Box<dyn Fn(&mut GeneratedFile) + Send + Sync>>,
    /// Generation context
    pub context: Option<CodeGenContext>,
}

impl<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> CodeGenBuilder<C, E> {
    /// Create a new code generation builder
    pub fn new() -> Self {
        Self {
            config: CodeGenConfig::default(),
            templates: None,
            options: CodeGenOptions::default(),
            pre_hooks: Vec::new(),
            post_hooks: Vec::new(),
            context: None,
        }
    }

    /// Set the target language
    pub fn language(mut self, language: ProgrammingLanguage) -> Self {
        self.config.language = language;
        self
    }

    /// Set custom templates
    pub fn with_templates(mut self, templates: CodeTemplates) -> Self {
        self.templates = Some(templates);
        self
    }

    /// Configure code generation options
    pub fn with_options(mut self, options: CodeGenOptions) -> Self {
        self.options = options;
        self
    }

    /// Include comments in generated code
    pub fn with_comments(mut self, include: bool) -> Self {
        self.config.include_comments = include;
        self
    }

    /// Include type annotations
    pub fn with_types(mut self, include: bool) -> Self {
        self.config.include_types = include;
        self
    }

    /// Generate async code
    pub fn async_code(mut self, async_code: bool) -> Self {
        self.config.async_code = async_code;
        self
    }

    /// Set indentation style
    pub fn indent_with(mut self, style: IndentationStyle) -> Self {
        self.config.indentation = style;
        self
    }

    /// Set maximum line length
    pub fn max_line_length(mut self, length: usize) -> Self {
        self.config.max_line_length = length;
        self
    }

    /// Include validation code
    pub fn with_validation(mut self, include: bool) -> Self {
        self.config.include_validation = include;
        self
    }

    /// Generate tests
    pub fn with_tests(mut self, generate: bool) -> Self {
        self.config.generate_tests = generate;
        self
    }

    /// Set output directory
    pub fn output_dir(mut self, dir: String) -> Self {
        self.config.output_dir = Some(dir);
        self
    }

    /// Set file naming pattern
    pub fn file_pattern(mut self, pattern: String) -> Self {
        self.config.file_pattern = pattern;
        self
    }

    /// Generate separate files
    pub fn separate_files(mut self, separate: bool) -> Self {
        self.options.separate_files = separate;
        self
    }

    /// Include imports
    pub fn with_imports(mut self, include: bool) -> Self {
        self.options.include_imports = include;
        self
    }

    /// Include module declarations
    pub fn with_modules(mut self, include: bool) -> Self {
        self.options.include_modules = include;
        self
    }

    /// Set custom header
    pub fn header(mut self, header: String) -> Self {
        self.options.custom_header = Some(header);
        self
    }

    /// Set custom footer
    pub fn footer(mut self, footer: String) -> Self {
        self.options.custom_footer = Some(footer);
        self
    }

    /// Add a pre-generation hook
    pub fn pre_hook<F>(mut self, hook: F) -> Self
    where
        F: Fn(&mut CodeGenContext) + Send + Sync + 'static,
    {
        self.pre_hooks.push(Box::new(hook));
        self
    }

    /// Add a post-generation hook
    pub fn post_hook<F>(mut self, hook: F) -> Self
    where
        F: Fn(&mut GeneratedFile) + Send + Sync + 'static,
    {
        self.post_hooks.push(Box::new(hook));
        self
    }

    /// Set generation context
    pub fn with_context(mut self, context: CodeGenContext) -> Self {
        self.context = Some(context);
        self
    }

    /// Build the code generator
    pub fn build_generator(mut self) -> CodeGenerator<C, E> {
        let mut generator = CodeGenerator::new(self.config);

        if let Some(templates) = self.templates {
            generator.templates = templates;
        }

        generator
    }

    /// Generate code for a machine
    pub fn generate(mut self, machine: &Machine<C, E, C>) -> Result<GeneratedFile, String> {
        let mut generator = self.build_generator();

        // Run pre-generation hooks
        if let Some(ref mut context) = self.context {
            for hook in &self.pre_hooks {
                hook(context);
            }
        }

        // Generate the code
        let mut file = generator.generate(machine)?;

        // Run post-generation hooks
        for hook in &self.post_hooks {
            hook(&mut file);
        }

        Ok(file)
    }

    /// Generate code and save to file
    pub fn generate_and_save(mut self, machine: &Machine<C, E, C>, output_path: &std::path::Path) -> Result<std::path::PathBuf, String> {
        let file = self.generate(machine)?;
        file.save_to_file(output_path)
    }

    /// Create a pipeline from this builder
    pub fn into_pipeline(self) -> CodeGenPipeline<C, E> {
        let mut pipeline = CodeGenPipeline::new();

        // Add this builder as a step
        let builder_step = BuilderStep {
            builder: self,
        };

        pipeline.add_step(Box::new(builder_step));
        pipeline
    }
}

/// Builder step for pipeline integration
struct BuilderStep<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> {
    builder: CodeGenBuilder<C, E>,
}

impl<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> CodeGenStep<C, E> for BuilderStep<C, E> {
    fn execute(&self, machine: &Machine<C, E, C>, _config: &PipelineConfig) -> Result<GeneratedFile, String> {
        // Create a copy of the builder for this execution
        let mut builder = CodeGenBuilder {
            config: self.builder.config.clone(),
            templates: self.builder.templates.clone(),
            options: self.builder.options.clone(),
            pre_hooks: self.builder.pre_hooks.clone(),
            post_hooks: self.builder.post_hooks.clone(),
            context: self.builder.context.clone(),
        };

        builder.generate(machine)
    }

    fn name(&self) -> &str {
        "BuilderStep"
    }

    fn description(&self) -> &str {
        "Code generation using builder pattern"
    }
}

/// Fluent API for building code generators
pub mod builder {
    use super::*;

    /// Start building a code generator for Rust
    pub fn rust<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>() -> CodeGenBuilder<C, E> {
        CodeGenBuilder::new().language(ProgrammingLanguage::Rust)
    }

    /// Start building a code generator for TypeScript
    pub fn typescript<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>() -> CodeGenBuilder<C, E> {
        CodeGenBuilder::new().language(ProgrammingLanguage::TypeScript)
    }

    /// Start building a code generator for Python
    pub fn python<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>() -> CodeGenBuilder<C, E> {
        CodeGenBuilder::new().language(ProgrammingLanguage::Python)
    }

    /// Start building with custom language
    pub fn for_language<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>(language: ProgrammingLanguage) -> CodeGenBuilder<C, E> {
        CodeGenBuilder::new().language(language)
    }

    /// Create a builder from existing config
    pub fn from_config<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>(config: CodeGenConfig) -> CodeGenBuilder<C, E> {
        CodeGenBuilder {
            config,
            templates: None,
            options: CodeGenOptions::default(),
            pre_hooks: Vec::new(),
            post_hooks: Vec::new(),
            context: None,
        }
    }

    /// Create a builder with custom templates
    pub fn with_templates<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>(
        templates: CodeTemplates
    ) -> CodeGenBuilder<C, E> {
        CodeGenBuilder::new().with_templates(templates)
    }

    /// Configure indentation
    pub fn indent_spaces<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>(
        spaces: usize
    ) -> Box<dyn FnOnce(CodeGenBuilder<C, E>) -> CodeGenBuilder<C, E>> {
        Box::new(move |builder| builder.indent_with(IndentationStyle::Spaces(spaces)))
    }

    /// Configure indentation with tabs
    pub fn indent_tabs<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>() -> Box<dyn FnOnce(CodeGenBuilder<C, E>) -> CodeGenBuilder<C, E>> {
        Box::new(|builder| builder.indent_with(IndentationStyle::Tabs))
    }

    /// Add a comment header
    pub fn with_header<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>(
        header: String
    ) -> Box<dyn FnOnce(CodeGenBuilder<C, E>) -> CodeGenBuilder<C, E>> {
        Box::new(move |builder| builder.header(header))
    }

    /// Add a comment footer
    pub fn with_footer<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>(
        footer: String
    ) -> Box<dyn FnOnce(CodeGenBuilder<C, E>) -> CodeGenBuilder<C, E>> {
        Box::new(move |builder| builder.footer(footer))
    }

    /// Enable test generation
    pub fn with_tests<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>() -> Box<dyn FnOnce(CodeGenBuilder<C, E>) -> CodeGenBuilder<C, E>> {
        Box::new(|builder| builder.with_tests(true))
    }

    /// Enable validation code generation
    pub fn with_validation<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>() -> Box<dyn FnOnce(CodeGenBuilder<C, E>) -> CodeGenBuilder<C, E>> {
        Box::new(|builder| builder.with_validation(true))
    }
}

/// Pre-configured builders for common use cases
pub mod presets {
    use super::*;

    /// Create a minimal code generator (no comments, no tests)
    pub fn minimal<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>(
        language: ProgrammingLanguage
    ) -> CodeGenBuilder<C, E> {
        CodeGenBuilder::new()
            .language(language)
            .with_comments(false)
            .with_types(false)
            .with_tests(false)
            .with_validation(false)
    }

    /// Create a comprehensive code generator (everything enabled)
    pub fn comprehensive<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>(
        language: ProgrammingLanguage
    ) -> CodeGenBuilder<C, E> {
        CodeGenBuilder::new()
            .language(language)
            .with_comments(true)
            .with_types(true)
            .async_code(true)
            .with_tests(true)
            .with_validation(true)
            .separate_files(true)
    }

    /// Create a web-ready code generator for JavaScript/TypeScript
    pub fn web_ready<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>() -> CodeGenBuilder<C, E> {
        CodeGenBuilder::new()
            .language(ProgrammingLanguage::TypeScript)
            .async_code(true)
            .with_comments(true)
            .with_types(true)
            .with_tests(true)
            .file_pattern("{machine_name}.ts".to_string())
    }

    /// Create a library-ready code generator for Rust
    pub fn library_ready<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>() -> CodeGenBuilder<C, E> {
        CodeGenBuilder::new()
            .language(ProgrammingLanguage::Rust)
            .with_comments(true)
            .with_types(true)
            .async_code(true)
            .with_tests(true)
            .with_validation(true)
            .separate_files(true)
            .file_pattern("src/{machine_name}.rs".to_string())
    }
}
