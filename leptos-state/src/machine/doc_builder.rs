//! Documentation builder for fluent configuration

use super::*;
use std::hash::Hash;
use super::doc_data::DocumentationData;
use crate::StateResult;

/// Documentation builder for fluent configuration
pub struct DocumentationBuilder<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> {
    /// Machine being documented
    machine: Machine<C, E, C>,
    /// Configuration
    config: DocumentationConfig,
    /// Documentation data
    data: DocumentationData,
}

impl<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> DocumentationBuilder<C, E> {
    /// Create a new documentation builder
    pub fn new(machine: Machine<C, E, C>) -> Self {
        let data = DocumentationData::new(machine.clone());
        Self {
            machine,
            config: DocumentationConfig::default(),
            data,
        }
    }

    /// Set the machine name
    pub fn with_machine_name(mut self, name: String) -> Self {
        self.data.set_machine_name(name);
        self
    }

    /// Set the machine description
    pub fn with_description(mut self, description: String) -> Self {
        self.data.set_machine_description(description);
        self
    }

    /// Set the output format
    pub fn format(mut self, format: DocumentationFormat) -> Self {
        self.config.format = format;
        self
    }

    /// Set the template
    pub fn template(mut self, template: DocumentationTemplate) -> Self {
        self.config.template = template;
        self
    }

    /// Set the output directory
    pub fn output_dir(mut self, dir: String) -> Self {
        self.config.output_dir = dir;
        self
    }

    /// Set the file prefix
    pub fn file_prefix(mut self, prefix: String) -> Self {
        self.config.file_prefix = prefix;
        self
    }

    /// Include diagrams in the documentation
    pub fn include_diagrams(mut self, include: bool) -> Self {
        self.config.include_diagrams = include;
        self
    }

    /// Include transition tables in the documentation
    pub fn include_tables(mut self, include: bool) -> Self {
        self.config.include_tables = include;
        self
    }

    /// Include implementation details in the documentation
    pub fn include_details(mut self, include: bool) -> Self {
        self.config.include_details = include;
        self
    }

    /// Include performance metrics in the documentation
    pub fn include_performance(mut self, include: bool) -> Self {
        self.config.include_performance = include;
        self
    }

    /// Use dark theme for HTML output
    pub fn dark_theme(mut self) -> Self {
        self.config.styling.dark_theme = true;
        self
    }

    /// Use a specific color scheme
    pub fn color_scheme(mut self, scheme: ColorScheme) -> Self {
        self.config.styling.color_scheme = scheme;
        self
    }

    /// Overwrite existing files
    pub fn overwrite_existing(mut self) -> Self {
        self.config.overwrite_existing = true;
        self
    }

    /// Include timestamp in filenames
    pub fn include_timestamp(mut self) -> Self {
        self.config.include_timestamp = true;
        self
    }

    /// Add a state description
    pub fn describe_state(mut self, state_name: &str, description: String) -> Self {
        if let Some(state) = self.data.get_state_mut(state_name) {
            state.description = Some(description);
        }
        self
    }

    /// Add metadata
    pub fn add_metadata(mut self, key: String, value: String) -> Self {
        self.data.add_metadata(key, value);
        self
    }

    /// Add a custom state
    pub fn add_state(mut self, state: StateInfo) -> Self {
        self.data.add_state(state);
        self
    }

    /// Add a custom transition
    pub fn add_transition(mut self, transition: TransitionInfo) -> Self {
        self.data.add_transition(transition);
        self
    }

    /// Add a custom action
    pub fn add_action(mut self, action: ActionInfo) -> Self {
        self.data.add_action(action);
        self
    }

    /// Add a custom guard
    pub fn add_guard(mut self, guard: GuardInfo) -> Self {
        self.data.add_guard(guard);
        self
    }

    /// Build and generate the documentation
    pub fn build(self) -> StateResult<GeneratedDocument> {
        let generator = DocumentationGenerator::new(self.machine, self.config);
        generator.generate()
    }

    /// Build and save the documentation to a file
    pub fn build_and_save(self, path: Option<&std::path::Path>) -> StateResult<GeneratedDocument> {
        let document = self.build()?;
        let file_path = path.map(|p| p.to_path_buf())
            .unwrap_or_else(|| std::path::Path::new(&self.config.output_dir).join(document.full_filename()));

        // Create output directory if it doesn't exist
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        document.save_to_file(&file_path)?;
        Ok(document)
    }

    /// Get the current configuration
    pub fn config(&self) -> &DocumentationConfig {
        &self.config
    }

    /// Get the current data
    pub fn data(&self) -> &DocumentationData {
        &self.data
    }

    /// Get mutable access to configuration
    pub fn config_mut(&mut self) -> &mut DocumentationConfig {
        &mut self.config
    }

    /// Get mutable access to data
    pub fn data_mut(&mut self) -> &mut DocumentationData {
        &mut self.data
    }
}

/// Extension trait for adding documentation to machines
pub trait MachineDocumentationExt<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> {
    /// Create a documentation builder for this machine
    fn document(&self) -> DocumentationBuilder<C, E>;
}

impl<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> MachineDocumentationExt<C, E> for Machine<C, E, C> {
    fn document(&self) -> DocumentationBuilder<C, E> {
        DocumentationBuilder::new(self.clone())
    }
}

/// Batch documentation generation
pub struct DocumentationBatch<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> {
    /// Machines to document
    machines: Vec<(String, Machine<C, E, C>)>,
    /// Base configuration
    base_config: DocumentationConfig,
    /// Generated documents
    documents: Vec<GeneratedDocument>,
}

impl<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> DocumentationBatch<C, E> {
    /// Create a new documentation batch
    pub fn new(base_config: DocumentationConfig) -> Self {
        Self {
            machines: Vec::new(),
            base_config,
            documents: Vec::new(),
        }
    }

    /// Add a machine to the batch
    pub fn add_machine(&mut self, name: String, machine: Machine<C, E, C>) {
        self.machines.push((name, machine));
    }

    /// Generate documentation for all machines in the batch
    pub fn generate_all(&mut self) -> StateResult<&[GeneratedDocument]> {
        self.documents.clear();

        for (name, machine) in &self.machines {
            let mut config = self.base_config.clone();
            config.file_prefix = name.clone();

            let generator = DocumentationGenerator::new(machine.clone(), config);
            let document = generator.generate()?;
            self.documents.push(document);
        }

        Ok(&self.documents)
    }

    /// Save all generated documents
    pub fn save_all(&self, base_dir: &std::path::Path) -> StateResult<Vec<std::path::PathBuf>> {
        let mut saved_paths = Vec::new();

        for document in &self.documents {
            let file_path = base_dir.join(&document.filename);
            document.save_to_file(&file_path)?;
            saved_paths.push(file_path);
        }

        Ok(saved_paths)
    }

    /// Get the generated documents
    pub fn documents(&self) -> &[GeneratedDocument] {
        &self.documents
    }
}

/// Documentation presets
pub struct DocumentationPresets;

impl DocumentationPresets {
    /// Create a comprehensive documentation configuration
    pub fn comprehensive() -> DocumentationConfig {
        DocumentationConfig {
            format: DocumentationFormat::Markdown,
            template: DocumentationTemplate::Default,
            styling: DocumentationStyling::default(),
            include_diagrams: true,
            include_tables: true,
            include_details: true,
            include_performance: true,
            output_dir: "docs".to_string(),
            file_prefix: "state_machine".to_string(),
            overwrite_existing: true,
            include_timestamp: true,
        }
    }

    /// Create a minimal documentation configuration
    pub fn minimal() -> DocumentationConfig {
        DocumentationConfig {
            format: DocumentationFormat::Markdown,
            template: DocumentationTemplate::Minimal,
            styling: DocumentationStyling::default(),
            include_diagrams: false,
            include_tables: false,
            include_details: false,
            include_performance: false,
            output_dir: "docs".to_string(),
            file_prefix: "state_machine".to_string(),
            overwrite_existing: true,
            include_timestamp: false,
        }
    }

    /// Create an API documentation configuration
    pub fn api() -> DocumentationConfig {
        DocumentationConfig {
            format: DocumentationFormat::Html,
            template: DocumentationTemplate::Api,
            styling: DocumentationStyling::default(),
            include_diagrams: true,
            include_tables: true,
            include_details: true,
            include_performance: false,
            output_dir: "docs/api".to_string(),
            file_prefix: "api".to_string(),
            overwrite_existing: true,
            include_timestamp: true,
        }
    }
}
