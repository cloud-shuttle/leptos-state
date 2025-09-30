//! Documentation configuration and formats

use super::*;

/// Documentation configuration for state machines
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentationConfig {
    /// Output format for documentation
    pub format: DocumentationFormat,
    /// Template to use for generation
    pub template: DocumentationTemplate,
    /// Styling configuration
    pub styling: DocumentationStyling,
    /// Include state diagrams
    pub include_diagrams: bool,
    /// Include transition tables
    pub include_tables: bool,
    /// Include action and guard details
    pub include_details: bool,
    /// Include performance metrics
    pub include_performance: bool,
    /// Output directory for generated files
    pub output_dir: String,
    /// File prefix for generated files
    pub file_prefix: String,
    /// Whether to overwrite existing files
    pub overwrite_existing: bool,
    /// Include timestamp in filenames
    pub include_timestamp: bool,
}

impl Default for DocumentationConfig {
    fn default() -> Self {
        Self {
            format: DocumentationFormat::Markdown,
            template: DocumentationTemplate::Default,
            styling: DocumentationStyling::default(),
            include_diagrams: true,
            include_tables: true,
            include_details: true,
            include_performance: false,
            output_dir: "docs".to_string(),
            file_prefix: "state_machine".to_string(),
            overwrite_existing: true,
            include_timestamp: false,
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
    /// JSON format for structured data
    Json,
    /// Plain text format
    Text,
    /// DOT format for graphviz diagrams
    Dot,
    /// PlantUML format for diagrams
    PlantUml,
}

impl DocumentationFormat {
    /// Get the file extension for this format
    pub fn extension(&self) -> &str {
        match self {
            DocumentationFormat::Markdown => "md",
            DocumentationFormat::Html => "html",
            DocumentationFormat::Json => "json",
            DocumentationFormat::Text => "txt",
            DocumentationFormat::Dot => "dot",
            DocumentationFormat::PlantUml => "puml",
        }
    }

    /// Check if this format supports diagrams
    pub fn supports_diagrams(&self) -> bool {
        matches!(
            self,
            DocumentationFormat::Dot | DocumentationFormat::PlantUml | DocumentationFormat::Html
        )
    }
}

/// Documentation templates
#[derive(Debug, Clone, PartialEq)]
pub enum DocumentationTemplate {
    /// Default template with comprehensive documentation
    Default,
    /// Minimal template with basic information
    Minimal,
    /// Technical template focused on implementation details
    Technical,
    /// User-focused template for end users
    User,
    /// API template for developers
    Api,
    /// Custom template (requires custom template data)
    Custom(String),
}

impl DocumentationTemplate {
    /// Get the template name
    pub fn name(&self) -> &str {
        match self {
            DocumentationTemplate::Default => "default",
            DocumentationTemplate::Minimal => "minimal",
            DocumentationTemplate::Technical => "technical",
            DocumentationTemplate::User => "user",
            DocumentationTemplate::Api => "api",
            DocumentationTemplate::Custom(name) => name,
        }
    }

    /// Check if this template includes diagrams
    pub fn includes_diagrams(&self) -> bool {
        !matches!(self, DocumentationTemplate::Minimal)
    }

    /// Check if this template includes technical details
    pub fn includes_technical_details(&self) -> bool {
        matches!(
            self,
            DocumentationTemplate::Default
                | DocumentationTemplate::Technical
                | DocumentationTemplate::Api
        )
    }
}

/// Documentation styling configuration
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentationStyling {
    /// Color scheme for diagrams
    pub color_scheme: ColorScheme,
    /// Font family for text
    pub font_family: String,
    /// Font size for headings
    pub heading_font_size: u32,
    /// Font size for body text
    pub body_font_size: u32,
    /// Line height for text
    pub line_height: f32,
    /// Maximum diagram width
    pub max_diagram_width: u32,
    /// Maximum diagram height
    pub max_diagram_height: u32,
    /// Whether to use dark theme
    pub dark_theme: bool,
}

impl Default for DocumentationStyling {
    fn default() -> Self {
        Self {
            color_scheme: ColorScheme::Default,
            font_family: "Arial, sans-serif".to_string(),
            heading_font_size: 24,
            body_font_size: 14,
            line_height: 1.5,
            max_diagram_width: 800,
            max_diagram_height: 600,
            dark_theme: false,
        }
    }
}

/// Color schemes for documentation
#[derive(Debug, Clone, PartialEq)]
pub enum ColorScheme {
    /// Default color scheme
    Default,
    /// Blue color scheme
    Blue,
    /// Green color scheme
    Green,
    /// Red color scheme
    Red,
    /// Purple color scheme
    Purple,
    /// Custom color scheme
    Custom(Vec<String>),
}

impl ColorScheme {
    /// Get the primary color for this scheme
    pub fn primary_color(&self) -> &str {
        match self {
            ColorScheme::Default => "#4A90E2",
            ColorScheme::Blue => "#007AFF",
            ColorScheme::Green => "#34C759",
            ColorScheme::Red => "#FF3B30",
            ColorScheme::Purple => "#AF52DE",
            ColorScheme::Custom(colors) => colors.get(0).unwrap_or("#4A90E2"),
        }
    }

    /// Get the secondary color for this scheme
    pub fn secondary_color(&self) -> &str {
        match self {
            ColorScheme::Default => "#E5E5EA",
            ColorScheme::Blue => "#E3F2FD",
            ColorScheme::Green => "#E8F5E8",
            ColorScheme::Red => "#FFEBEE",
            ColorScheme::Purple => "#F3E5F5",
            ColorScheme::Custom(colors) => colors.get(1).unwrap_or("#E5E5EA"),
        }
    }
}

/// Documentation generation options
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentationOptions {
    /// Configuration
    pub config: DocumentationConfig,
    /// Additional metadata to include
    pub metadata: std::collections::HashMap<String, String>,
    /// Custom CSS for HTML output
    pub custom_css: Option<String>,
    /// Custom header for documents
    pub custom_header: Option<String>,
    /// Custom footer for documents
    pub custom_footer: Option<String>,
}

impl Default for DocumentationOptions {
    fn default() -> Self {
        Self {
            config: DocumentationConfig::default(),
            metadata: std::collections::HashMap::new(),
            custom_css: None,
            custom_header: None,
            custom_footer: None,
        }
    }
}
