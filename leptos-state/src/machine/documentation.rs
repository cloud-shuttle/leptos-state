//! State Machine Documentation Generator
//!
//! This module provides comprehensive automatic documentation generation
//! for state machines, including multiple formats, templates, and diagrams.

use super::*;
use crate::machine::visualization::ExportFormat;
use crate::utils::types::{StateError, StateResult};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;

use std::sync::{Arc, RwLock};
use std::time::Instant;

#[cfg(feature = "serde_json")]
use serde_json;
#[cfg(feature = "serde_yaml")]
use serde_yaml;

/// Documentation configuration for state machines
#[derive(Debug, Clone)]
pub struct DocumentationConfig {
    /// Whether documentation generation is enabled
    pub enabled: bool,
    /// Output formats to generate
    pub output_formats: Vec<DocumentationFormat>,
    /// Output directory for generated documentation
    pub output_directory: String,
    /// Documentation template to use
    pub template: DocumentationTemplate,
    /// Whether to include diagrams
    pub include_diagrams: bool,
    /// Whether to include code examples
    pub include_code_examples: bool,
    /// Whether to include API documentation
    pub include_api_docs: bool,
    /// Whether to include usage examples
    pub include_usage_examples: bool,
    /// Custom metadata for documentation
    pub metadata: HashMap<String, String>,
    /// Documentation styling
    pub styling: DocumentationStyling,
}

impl Default for DocumentationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            output_formats: vec![DocumentationFormat::Markdown, DocumentationFormat::Html],
            output_directory: "docs".to_string(),
            template: DocumentationTemplate::Default,
            include_diagrams: true,
            include_code_examples: true,
            include_api_docs: true,
            include_usage_examples: true,
            metadata: HashMap::new(),
            styling: DocumentationStyling::default(),
        }
    }
}

/// Documentation output formats
#[derive(Debug, Clone, PartialEq)]
pub enum DocumentationFormat {
    /// Markdown format
    Markdown,
    /// HTML format
    Html,
    /// PDF format
    Pdf,
    /// AsciiDoc format
    AsciiDoc,
    /// ReStructuredText format
    Rst,
    /// JSON format
    Json,
    /// YAML format
    Yaml,
    /// Custom format
    Custom(String),
}

/// Documentation templates
#[derive(Debug, Clone, PartialEq)]
pub enum DocumentationTemplate {
    /// Default template
    Default,
    /// Minimal template
    Minimal,
    /// Comprehensive template
    Comprehensive,
    /// API documentation template
    ApiDocs,
    /// User guide template
    UserGuide,
    /// Custom template
    Custom(String),
}

/// Documentation styling configuration
#[derive(Debug, Clone)]
pub struct DocumentationStyling {
    /// CSS theme to use
    pub theme: String,
    /// Custom CSS
    pub custom_css: Option<String>,
    /// Logo URL
    pub logo_url: Option<String>,
    /// Primary color
    pub primary_color: String,
    /// Secondary color
    pub secondary_color: String,
    /// Font family
    pub font_family: String,
    /// Font size
    pub font_size: String,
}

impl Default for DocumentationStyling {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            custom_css: None,
            logo_url: None,
            primary_color: "#007acc".to_string(),
            secondary_color: "#6c757d".to_string(),
            font_family: "system-ui, -apple-system, sans-serif".to_string(),
            font_size: "14px".to_string(),
        }
    }
}

/// Documentation generator for state machines
pub struct DocumentationGenerator<C: Send + Sync + Clone + Default + Debug, E: Clone + PartialEq + Debug + Send + Sync + Default> {
    config: DocumentationConfig,
    machine: Arc<Machine<C, E>>,
    templates: Arc<RwLock<HashMap<String, String>>>,
    generated_docs: Arc<RwLock<Vec<GeneratedDocument>>>,
}

impl<C, E> DocumentationGenerator<C, E>
where
    C: Clone + std::fmt::Debug + Send + Sync + Default,
    E: Clone + std::fmt::Debug + Event + Send + Sync + PartialEq + Default,
{
    pub fn new(machine: Machine<C, E>, config: DocumentationConfig) -> Self {
        Self {
            config,
            machine: Arc::new(machine),
            templates: Arc::new(RwLock::new(HashMap::new())),
            generated_docs: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Generate documentation for the state machine
    pub fn generate_documentation(&self) -> StateResult<Vec<GeneratedDocument>> {
        let start_time = Instant::now();
        let mut generated_docs = Vec::new();

        // Generate documentation for each format
        for format in &self.config.output_formats {
            let doc = self.generate_format_documentation(format)?;
            generated_docs.push(doc);
        }

        // Save documentation to files
        if !self.config.output_directory.is_empty() {
            self.save_documentation(&generated_docs)?;
        }

        // Update generated docs
        if let Ok(mut docs) = self.generated_docs.write() {
            *docs = generated_docs.clone();
        }

        println!("Documentation generated in {:?}", start_time.elapsed());
        Ok(generated_docs)
    }

    /// Generate documentation for a specific format
    fn generate_format_documentation(
        &self,
        format: &DocumentationFormat,
    ) -> StateResult<GeneratedDocument> {
        let content = match format {
            DocumentationFormat::Markdown => self.generate_markdown_documentation()?,
            DocumentationFormat::Html => self.generate_html_documentation()?,
            DocumentationFormat::Pdf => self.generate_pdf_documentation()?,
            DocumentationFormat::AsciiDoc => self.generate_asciidoc_documentation()?,
            DocumentationFormat::Rst => self.generate_rst_documentation()?,
            DocumentationFormat::Json => self.generate_json_documentation()?,
            DocumentationFormat::Yaml => self.generate_yaml_documentation()?,
            DocumentationFormat::Custom(custom_format) => {
                self.generate_custom_documentation(custom_format)?
            }
        };

        Ok(GeneratedDocument {
            format: format.clone(),
            content,
            generated_at: Instant::now(),
            file_path: self.get_output_path(format),
        })
    }

    /// Generate Markdown documentation
    fn generate_markdown_documentation(&self) -> StateResult<String> {
        let mut markdown = String::new();

        // Title
        markdown.push_str("# State Machine Documentation\n\n");

        // Overview
        markdown.push_str("## Overview\n\n");
        markdown.push_str(
            "This document provides comprehensive documentation for the state machine.\n\n",
        );

        // States
        markdown.push_str("## States\n\n");
        let states = self.machine.get_states();
        for state in states {
            markdown.push_str(&format!("### {}\n\n", state));
            markdown.push_str("State description and behavior.\n\n");
        }

        // Events
        markdown.push_str("## Events\n\n");
        let events = self.get_machine_events();
        for event in events {
            markdown.push_str(&format!("### {}\n\n", event));
            markdown.push_str("Event description and effects.\n\n");
        }

        // Transitions
        markdown.push_str("## Transitions\n\n");
        let transitions = self.get_machine_transitions();
        for transition in transitions {
            markdown.push_str(&format!(
                "- **{}** → **{}** (Event: {})\n",
                transition.from, transition.to, transition.event
            ));
        }
        markdown.push_str("\n");

        // Guards
        if self.config.include_api_docs {
            markdown.push_str("## Guards\n\n");
            markdown.push_str("State transition guards and conditions.\n\n");
        }

        // Actions
        if self.config.include_api_docs {
            markdown.push_str("## Actions\n\n");
            markdown.push_str("State entry/exit actions and transition actions.\n\n");
        }

        // Usage Examples
        if self.config.include_usage_examples {
            markdown.push_str("## Usage Examples\n\n");
            markdown.push_str("```rust\n");
            markdown.push_str("// Example state machine usage\n");
            markdown.push_str("let machine = MachineBuilder::new()\n");
            markdown.push_str("    .state(\"idle\")\n");
            markdown.push_str("    .on(Event::Start, \"running\")\n");
            markdown.push_str("    .build();\n");
            markdown.push_str("```\n\n");
        }

        // API Reference
        if self.config.include_api_docs {
            markdown.push_str("## API Reference\n\n");
            markdown.push_str("### MachineBuilder\n\n");
            markdown.push_str("The main builder for creating state machines.\n\n");
            markdown.push_str("### Methods\n\n");
            markdown.push_str("- `state(name)` - Define a new state\n");
            markdown.push_str("- `on(event, target)` - Define a transition\n");
            markdown.push_str("- `build()` - Build the state machine\n\n");
        }

        // Diagrams
        if self.config.include_diagrams {
            markdown.push_str("## State Diagram\n\n");
            if let Ok(diagram) = self.machine.export_diagram(ExportFormat::Mermaid) {
                markdown.push_str("```mermaid\n");
                markdown.push_str(&diagram);
                markdown.push_str("\n```\n\n");
            }
        }

        Ok(markdown)
    }

    /// Generate HTML documentation
    fn generate_html_documentation(&self) -> StateResult<String> {
        let mut html = String::new();

        // HTML header
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"en\">\n");
        html.push_str("<head>\n");
        html.push_str("    <meta charset=\"UTF-8\">\n");
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str(&format!("    <title>State Machine Documentation</title>\n"));
        html.push_str(&format!("    <style>\n"));
        html.push_str(&format!(
            "        body {{ font-family: {}; font-size: {}; }}\n",
            self.config.styling.font_family, self.config.styling.font_size
        ));
        html.push_str(&format!(
            "        .primary {{ color: {}; }}\n",
            self.config.styling.primary_color
        ));
        html.push_str(&format!(
            "        .secondary {{ color: {}; }}\n",
            self.config.styling.secondary_color
        ));
        html.push_str("        .state { background-color: #f8f9fa; padding: 10px; margin: 10px 0; border-radius: 5px; }\n");
        html.push_str("        .event { background-color: #e9ecef; padding: 10px; margin: 10px 0; border-radius: 5px; }\n");
        html.push_str("        .transition { background-color: #dee2e6; padding: 5px; margin: 5px 0; border-radius: 3px; }\n");
        html.push_str("        pre { background-color: #f8f9fa; padding: 15px; border-radius: 5px; overflow-x: auto; }\n");
        html.push_str(
            "        code { background-color: #f1f3f4; padding: 2px 4px; border-radius: 3px; }\n",
        );
        html.push_str("    </style>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");

        // Title
        html.push_str(&format!(
            "    <h1 class=\"primary\">State Machine Documentation</h1>\n"
        ));

        // Overview
        html.push_str("    <h2>Overview</h2>\n");
        html.push_str("    <p>This document provides comprehensive documentation for the state machine.</p>\n");

        // States
        html.push_str("    <h2>States</h2>\n");
        let states = self.machine.get_states();
        for state in states {
            html.push_str(&format!("    <div class=\"state\">\n"));
            html.push_str(&format!("        <h3>{}</h3>\n", state));
            html.push_str("        <p>State description and behavior.</p>\n");
            html.push_str("    </div>\n");
        }

        // Events
        html.push_str("    <h2>Events</h2>\n");
        let events = self.get_machine_events();
        for event in events {
            html.push_str(&format!("    <div class=\"event\">\n"));
            html.push_str(&format!("        <h3>{}</h3>\n", event));
            html.push_str("        <p>Event description and effects.</p>\n");
            html.push_str("    </div>\n");
        }

        // Transitions
        html.push_str("    <h2>Transitions</h2>\n");
        let transitions = self.get_machine_transitions();
        for transition in transitions {
            html.push_str(&format!("    <div class=\"transition\">\n"));
            html.push_str(&format!(
                "        <strong>{} → {}</strong> (Event: {})\n",
                transition.from, transition.to, transition.event
            ));
            html.push_str("    </div>\n");
        }

        // Usage Examples
        if self.config.include_usage_examples {
            html.push_str("    <h2>Usage Examples</h2>\n");
            html.push_str("    <pre><code>// Example state machine usage\n");
            html.push_str("let machine = MachineBuilder::new()\n");
            html.push_str("    .state(\"idle\")\n");
            html.push_str("    .on(Event::Start, \"running\")\n");
            html.push_str("    .build();</code></pre>\n");
        }

        // Diagrams
        if self.config.include_diagrams {
            html.push_str("    <h2>State Diagram</h2>\n");
            if let Ok(diagram) = self.machine.export_diagram(ExportFormat::Mermaid) {
                html.push_str("    <pre><code class=\"mermaid\">\n");
                html.push_str(&diagram);
                html.push_str("\n    </code></pre>\n");
                html.push_str("    <script src=\"https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js\"></script>\n");
                html.push_str("    <script>mermaid.initialize({startOnLoad:true});</script>\n");
            }
        }

        // Footer
        html.push_str(&format!("    <footer style=\"margin-top: 50px; padding: 20px; border-top: 1px solid #dee2e6; text-align: center; color: {};\">\n", self.config.styling.secondary_color));
        html.push_str("        <p>Generated by Leptos State Machine Documentation Generator</p>\n");
        html.push_str("    </footer>\n");

        html.push_str("</body>\n");
        html.push_str("</html>\n");

        Ok(html)
    }

    /// Generate PDF documentation
    fn generate_pdf_documentation(&self) -> StateResult<String> {
        // For now, return HTML content that can be converted to PDF
        // In a real implementation, this would use a PDF library
        self.generate_html_documentation()
    }

    /// Generate AsciiDoc documentation
    fn generate_asciidoc_documentation(&self) -> StateResult<String> {
        let mut asciidoc = String::new();

        // Title
        asciidoc.push_str("= State Machine Documentation\n\n");

        // Overview
        asciidoc.push_str("== Overview\n\n");
        asciidoc.push_str(
            "This document provides comprehensive documentation for the state machine.\n\n",
        );

        // States
        asciidoc.push_str("== States\n\n");
        let states = self.machine.get_states();
        for state in states {
            asciidoc.push_str(&format!("=== {}\n\n", state));
            asciidoc.push_str("State description and behavior.\n\n");
        }

        // Events
        asciidoc.push_str("== Events\n\n");
        let events = self.get_machine_events();
        for event in events {
            asciidoc.push_str(&format!("=== {}\n\n", event));
            asciidoc.push_str("Event description and effects.\n\n");
        }

        // Transitions
        asciidoc.push_str("== Transitions\n\n");
        let transitions = self.get_machine_transitions();
        for transition in transitions {
            asciidoc.push_str(&format!(
                "* **{}** → **{}** (Event: {})\n",
                transition.from, transition.to, transition.event
            ));
        }
        asciidoc.push_str("\n");

        Ok(asciidoc)
    }

    /// Generate ReStructuredText documentation
    fn generate_rst_documentation(&self) -> StateResult<String> {
        let mut rst = String::new();

        // Title
        rst.push_str("State Machine Documentation\n");
        rst.push_str("===========================\n\n");

        // Overview
        rst.push_str("Overview\n");
        rst.push_str("--------\n\n");
        rst.push_str(
            "This document provides comprehensive documentation for the state machine.\n\n",
        );

        // States
        rst.push_str("States\n");
        rst.push_str("------\n\n");
        let states = self.machine.get_states();
        for state in states {
            rst.push_str(&format!("{}\n", state));
            rst.push_str(&format!("{}\n", "~".repeat(state.len())));
            rst.push_str("State description and behavior.\n\n");
        }

        // Events
        rst.push_str("Events\n");
        rst.push_str("------\n\n");
        let events = self.get_machine_events();
        for event in events {
            rst.push_str(&format!("{}\n", event));
            rst.push_str(&format!("{}\n", "~".repeat(event.len())));
            rst.push_str("Event description and effects.\n\n");
        }

        Ok(rst)
    }

    /// Generate JSON documentation
    fn generate_json_documentation(&self) -> StateResult<String> {
        let _doc = DocumentationData {
            title: "State Machine Documentation".to_string(),
            states: self.machine.get_states(),
            events: self.get_machine_events(),
            transitions: self.get_machine_transitions(),
            generated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        #[cfg(feature = "serde_json")]
        {
            serde_json::to_string_pretty(&_doc).map_err(|e| {
                StateError::custom(format!("Failed to serialize documentation: {}", e))
            })
        }

        #[cfg(not(feature = "serde_json"))]
        Err(StateError::new("JSON export requires serde_json feature"))
    }

    /// Generate YAML documentation
    fn generate_yaml_documentation(&self) -> StateResult<String> {
        let _doc = DocumentationData {
            title: "State Machine Documentation".to_string(),
            states: self.machine.get_states(),
            events: self.get_machine_events(),
            transitions: self.get_machine_transitions(),
            generated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        #[cfg(feature = "serialization")]
        {
            serde_yaml::to_string(&_doc).map_err(|e| {
                StateError::custom(format!("Failed to serialize documentation: {}", e))
            })
        }

        #[cfg(not(feature = "serialization"))]
        Err(StateError::new(
            "YAML export requires serialization feature",
        ))
    }

    /// Generate custom format documentation
    fn generate_custom_documentation(&self, format: &str) -> StateResult<String> {
        // Load custom template
        if let Ok(templates) = self.templates.read() {
            if let Some(template) = templates.get(format) {
                return self.render_template(template);
            }
        }

        Err(StateError::custom(format!(
            "Unknown custom format: {}",
            format
        )))
    }

    /// Render a template with machine data
    fn render_template(&self, template: &str) -> StateResult<String> {
        let mut rendered = template.to_string();

        // Replace placeholders with actual data
        rendered = rendered.replace("{{title}}", "State Machine Documentation");
        rendered = rendered.replace("{{states}}", &self.machine.get_states().join(", "));
        rendered = rendered.replace("{{events}}", &self.get_machine_events().join(", "));

        Ok(rendered)
    }

    /// Get machine events
    fn get_machine_events(&self) -> Vec<String> {
        // This would extract events from the machine
        // For now, return a placeholder
        vec!["start".to_string(), "stop".to_string(), "pause".to_string()]
    }

    /// Get machine transitions
    fn get_machine_transitions(&self) -> Vec<TransitionInfo> {
        // This would extract transitions from the machine
        // For now, return placeholders
        vec![
            TransitionInfo {
                from: "idle".to_string(),
                to: "running".to_string(),
                event: "start".to_string(),
            },
            TransitionInfo {
                from: "running".to_string(),
                to: "paused".to_string(),
                event: "pause".to_string(),
            },
            TransitionInfo {
                from: "running".to_string(),
                to: "idle".to_string(),
                event: "stop".to_string(),
            },
        ]
    }

    /// Get output path for a format
    fn get_output_path(&self, format: &DocumentationFormat) -> String {
        let filename = match format {
            DocumentationFormat::Markdown => "documentation.md".to_string(),
            DocumentationFormat::Html => "documentation.html".to_string(),
            DocumentationFormat::Pdf => "documentation.pdf".to_string(),
            DocumentationFormat::AsciiDoc => "documentation.adoc".to_string(),
            DocumentationFormat::Rst => "documentation.rst".to_string(),
            DocumentationFormat::Json => "documentation.json".to_string(),
            DocumentationFormat::Yaml => "documentation.yaml".to_string(),
            DocumentationFormat::Custom(name) => format!("documentation_{}.txt", name),
        };

        format!("{}/{}", self.config.output_directory, filename)
    }

    /// Save documentation to files
    fn save_documentation(&self, docs: &[GeneratedDocument]) -> StateResult<()> {
        // Create output directory if it doesn't exist
        if let Err(_) = fs::create_dir_all(&self.config.output_directory) {
            return Err(StateError::custom(format!(
                "Failed to create output directory: {}",
                self.config.output_directory
            )));
        }

        // Save each document
        for doc in docs {
            if let Err(e) = fs::write(&doc.file_path, &doc.content) {
                return Err(StateError::custom(format!(
                    "Failed to write documentation to {}: {}",
                    doc.file_path, e
                )));
            }
            println!("Documentation saved to: {}", doc.file_path);
        }

        Ok(())
    }

    /// Add a custom template
    pub fn add_template(&self, name: String, template: String) {
        if let Ok(mut templates) = self.templates.write() {
            templates.insert(name, template);
        }
    }

    /// Get generated documentation
    pub fn get_generated_documentation(&self) -> Vec<GeneratedDocument> {
        if let Ok(docs) = self.generated_docs.read() {
            docs.clone()
        } else {
            Vec::new()
        }
    }

    /// Generate documentation index
    pub fn generate_index(&self) -> StateResult<String> {
        let mut index = String::new();

        index.push_str("# Documentation Index\n\n");
        index.push_str("Generated documentation files:\n\n");

        let docs = self.get_generated_documentation();
        for doc in docs {
            index.push_str(&format!(
                "- [{}]({})\n",
                format!("{:?}", doc.format),
                doc.file_path
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
            "- Formats: {}\n",
            self.config
                .output_formats
                .iter()
                .map(|f| format!("{:?}", f))
                .collect::<Vec<_>>()
                .join(", ")
        ));

        Ok(index)
    }
}

/// Generated document information
#[derive(Debug, Clone)]
pub struct GeneratedDocument {
    /// Document format
    pub format: DocumentationFormat,
    /// Document content
    pub content: String,
    /// Generation timestamp
    pub generated_at: Instant,
    /// Output file path
    pub file_path: String,
}

/// Transition information for documentation
#[derive(Debug, Clone)]
#[cfg_attr(
    any(feature = "serialization", feature = "serde_json"),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct TransitionInfo {
    /// Source state
    pub from: String,
    /// Target state
    pub to: String,
    /// Triggering event
    pub event: String,
}

/// Documentation data structure
#[derive(Debug, Clone)]
#[cfg_attr(
    any(feature = "serialization", feature = "serde_json"),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct DocumentationData {
    /// Documentation title
    pub title: String,
    /// List of states
    pub states: Vec<String>,
    /// List of events
    pub events: Vec<String>,
    /// List of transitions
    pub transitions: Vec<TransitionInfo>,
    /// Generation timestamp
    pub generated_at: u64,
}

/// Extension trait for adding documentation to machines
pub trait MachineDocumentationExt<C: Send + Sync + Clone + Default + Debug, E: Clone + PartialEq + Debug + Send + Sync + Default> {
    /// Add documentation generation capabilities to the machine
    fn with_documentation(self, config: DocumentationConfig) -> DocumentationGenerator<C, E>;
}

impl<C, E> MachineDocumentationExt<C, E> for Machine<C, E>
where
    C: Clone + std::fmt::Debug + Send + Sync + Default,
    E: Clone + std::fmt::Debug + Event + Send + Sync + PartialEq + Default,
{
    fn with_documentation(self, config: DocumentationConfig) -> DocumentationGenerator<C, E> {
        DocumentationGenerator::new(self, config)
    }
}

/// Documentation builder for fluent configuration
pub struct DocumentationBuilder<C: Send + Sync + Clone + Default + Debug, E: Clone + PartialEq + Debug + Send + Sync + Default> {
    machine: Machine<C, E>,
    config: DocumentationConfig,
}

impl<C, E> DocumentationBuilder<C, E>
where
    C: Clone + std::fmt::Debug + Send + Sync + Default,
    E: Clone + std::fmt::Debug + Event + Send + Sync + PartialEq + Default,
{
    pub fn new(machine: Machine<C, E>) -> Self {
        Self {
            machine,
            config: DocumentationConfig::default(),
        }
    }

    pub fn with_config(mut self, config: DocumentationConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_format(mut self, format: DocumentationFormat) -> Self {
        self.config.output_formats.push(format);
        self
    }

    pub fn with_formats(mut self, formats: Vec<DocumentationFormat>) -> Self {
        self.config.output_formats = formats;
        self
    }

    pub fn with_output_directory(mut self, directory: String) -> Self {
        self.config.output_directory = directory;
        self
    }

    pub fn with_template(mut self, template: DocumentationTemplate) -> Self {
        self.config.template = template;
        self
    }

    pub fn with_diagrams(mut self, include: bool) -> Self {
        self.config.include_diagrams = include;
        self
    }

    pub fn with_code_examples(mut self, include: bool) -> Self {
        self.config.include_code_examples = include;
        self
    }

    pub fn with_api_docs(mut self, include: bool) -> Self {
        self.config.include_api_docs = include;
        self
    }

    pub fn with_usage_examples(mut self, include: bool) -> Self {
        self.config.include_usage_examples = include;
        self
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.config.metadata.insert(key, value);
        self
    }

    pub fn with_styling(mut self, styling: DocumentationStyling) -> Self {
        self.config.styling = styling;
        self
    }

    pub fn build(self) -> DocumentationGenerator<C, E> {
        DocumentationGenerator::new(self.machine, self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::machine::*;

    #[derive(Debug, Clone, PartialEq)]
    #[derive(Default)]
    struct TestContext {
        count: i32,
        name: String,
    }

    #[derive(Debug, Clone, PartialEq, Default)]
    enum TestEvent {
        #[default]
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
    fn test_documentation_config_default() {
        let config = DocumentationConfig::default();
        assert!(config.enabled);
        assert_eq!(config.output_formats.len(), 2);
        assert_eq!(config.output_directory, "docs");
    }

    #[test]
    fn test_markdown_documentation_generation() {
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .state("idle")
            .on(TestEvent::Increment, "counting")
            .state("counting")
            .on(TestEvent::Decrement, "idle")
            .build();

        let config = DocumentationConfig {
            output_formats: vec![DocumentationFormat::Markdown],
            output_directory: "test_docs".to_string(),
            ..Default::default()
        };

        let generator = DocumentationGenerator::new(machine, config);
        let docs = generator.generate_documentation().unwrap();

        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0].format, DocumentationFormat::Markdown);
        assert!(docs[0].content.contains("# State Machine Documentation"));
        assert!(docs[0].content.contains("## States"));
        assert!(docs[0].content.contains("## Events"));
    }

    #[test]
    fn test_html_documentation_generation() {
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .state("idle")
            .on(TestEvent::Increment, "counting")
            .state("counting")
            .on(TestEvent::Decrement, "idle")
            .build();

        let config = DocumentationConfig {
            output_formats: vec![DocumentationFormat::Html],
            output_directory: "test_docs".to_string(),
            ..Default::default()
        };

        let generator = DocumentationGenerator::new(machine, config);
        let docs = generator.generate_documentation().unwrap();

        assert_eq!(docs.len(), 1);
        assert_eq!(docs[0].format, DocumentationFormat::Html);
        assert!(docs[0].content.contains("<!DOCTYPE html>"));
        assert!(docs[0].content.contains("<h1"));
        assert!(docs[0].content.contains("</html>"));
    }

    #[test]
    fn test_json_documentation_generation() {
        #[cfg(feature = "serde_json")]
        {
            let machine = MachineBuilder::<TestContext, TestEvent>::new()
                .state("idle")
                .on(TestEvent::Increment, "counting")
                .state("counting")
                .on(TestEvent::Decrement, "idle")
                .build();

            let config = DocumentationConfig {
                output_formats: vec![DocumentationFormat::Json],
                output_directory: "test_docs".to_string(),
                ..Default::default()
            };

            let generator = DocumentationGenerator::new(machine, config);
            let docs = generator.generate_documentation().unwrap();

            assert_eq!(docs.len(), 1);
            assert_eq!(docs[0].format, DocumentationFormat::Json);
            assert!(docs[0].content.contains("\"title\""));
            assert!(docs[0].content.contains("\"states\""));
            assert!(docs[0].content.contains("\"events\""));
        }

        #[cfg(not(feature = "serde_json"))]
        {
            // Skip test when serde_json feature is not enabled
            println!("Skipping JSON documentation test - serde_json feature not enabled");
        }
    }

    #[test]
    fn test_documentation_builder() {
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .state("idle")
            .on(TestEvent::Increment, "counting")
            .state("counting")
            .on(TestEvent::Decrement, "idle")
            .build();

        let generator = DocumentationBuilder::new(machine)
            .with_formats(vec![
                DocumentationFormat::Markdown,
                DocumentationFormat::Html,
            ])
            .with_output_directory("custom_docs".to_string())
            .with_template(DocumentationTemplate::Comprehensive)
            .with_diagrams(true)
            .with_code_examples(true)
            .with_api_docs(true)
            .with_usage_examples(true)
            .with_metadata("version".to_string(), "1.0.0".to_string())
            .with_styling(DocumentationStyling {
                theme: "dark".to_string(),
                primary_color: "#ff0000".to_string(),
                secondary_color: "#00ff00".to_string(),
                ..Default::default()
            })
            .build();

        let config = generator.config;
        assert_eq!(config.output_formats.len(), 2);
        assert_eq!(config.output_directory, "custom_docs");
        assert_eq!(config.template, DocumentationTemplate::Comprehensive);
        assert!(config.include_diagrams);
        assert!(config.include_code_examples);
        assert!(config.include_api_docs);
        assert!(config.include_usage_examples);
        assert_eq!(config.metadata.get("version"), Some(&"1.0.0".to_string()));
        assert_eq!(config.styling.theme, "dark");
        assert_eq!(config.styling.primary_color, "#ff0000");
    }

    #[test]
    fn test_custom_template() {
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .state("idle")
            .on(TestEvent::Increment, "counting")
            .state("counting")
            .on(TestEvent::Decrement, "idle")
            .build();

        let config = DocumentationConfig {
            output_formats: vec![DocumentationFormat::Custom("custom".to_string())],
            output_directory: "test_docs".to_string(),
            ..Default::default()
        };

        let generator = DocumentationGenerator::new(machine, config);

        // Add custom template
        generator.add_template(
            "custom".to_string(),
            "Title: {{title}}\nStates: {{states}}\nEvents: {{events}}".to_string(),
        );

        let docs = generator.generate_documentation().unwrap();
        assert_eq!(docs.len(), 1);
        assert!(docs[0]
            .content
            .contains("Title: State Machine Documentation"));
        assert!(docs[0].content.contains("States:"));
        assert!(docs[0].content.contains("Events:"));
    }
}
