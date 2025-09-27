//! Visualization configuration and export formats

use super::*;

/// State machine visualization configuration
#[derive(Debug, Clone, PartialEq)]
pub struct VisualizationConfig {
    /// Whether to show state descriptions
    pub show_descriptions: bool,
    /// Whether to show guard conditions
    pub show_guards: bool,
    /// Whether to show actions
    pub show_actions: bool,
    /// Whether to show hierarchical relationships
    pub show_hierarchy: bool,
    /// Whether to use colors in output
    pub use_colors: bool,
    /// Layout direction for diagrams
    pub layout_direction: LayoutDirection,
    /// Node spacing in diagrams
    pub node_spacing: f64,
    /// Font size for labels
    pub font_size: f64,
    /// Whether to include timing information
    pub show_timing: bool,
    /// Maximum depth for hierarchical display
    pub max_depth: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutDirection {
    /// Left to right layout
    LeftToRight,
    /// Top to bottom layout
    TopToBottom,
    /// Right to left layout
    RightToLeft,
    /// Bottom to top layout
    BottomToTop,
}

impl Default for VisualizationConfig {
    fn default() -> Self {
        Self {
            show_descriptions: true,
            show_guards: true,
            show_actions: true,
            show_hierarchy: true,
            use_colors: true,
            layout_direction: LayoutDirection::LeftToRight,
            node_spacing: 100.0,
            font_size: 12.0,
            show_timing: false,
            max_depth: None,
        }
    }
}

impl VisualizationConfig {
    /// Create a new visualization config with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Show state descriptions
    pub fn with_descriptions(mut self, show: bool) -> Self {
        self.show_descriptions = show;
        self
    }

    /// Show guard conditions
    pub fn with_guards(mut self, show: bool) -> Self {
        self.show_guards = show;
        self
    }

    /// Show actions
    pub fn with_actions(mut self, show: bool) -> Self {
        self.show_actions = show;
        self
    }

    /// Show hierarchical relationships
    pub fn with_hierarchy(mut self, show: bool) -> Self {
        self.show_hierarchy = show;
        self
    }

    /// Use colors in output
    pub fn with_colors(mut self, use_colors: bool) -> Self {
        self.use_colors = use_colors;
        self
    }

    /// Set layout direction
    pub fn with_layout(mut self, direction: LayoutDirection) -> Self {
        self.layout_direction = direction;
        self
    }

    /// Set node spacing
    pub fn with_spacing(mut self, spacing: f64) -> Self {
        self.node_spacing = spacing;
        self
    }

    /// Set font size
    pub fn with_font_size(mut self, size: f64) -> Self {
        self.font_size = size;
        self
    }

    /// Show timing information
    pub fn with_timing(mut self, show: bool) -> Self {
        self.show_timing = show;
        self
    }

    /// Set maximum depth for hierarchical display
    pub fn with_max_depth(mut self, depth: Option<usize>) -> Self {
        self.max_depth = depth;
        self
    }
}

/// Export formats for state diagrams
#[derive(Debug, Clone, PartialEq)]
pub enum ExportFormat {
    /// GraphViz DOT format
    Dot,
    /// Mermaid format
    Mermaid,
    /// PlantUML format
    PlantUml,
    /// JSON format
    Json,
    /// SVG format (rendered)
    Svg,
    /// PNG format (rendered)
    Png,
}

impl ExportFormat {
    /// Get file extension for this format
    pub fn extension(&self) -> &'static str {
        match self {
            ExportFormat::Dot => "dot",
            ExportFormat::Mermaid => "mmd",
            ExportFormat::PlantUml => "puml",
            ExportFormat::Json => "json",
            ExportFormat::Svg => "svg",
            ExportFormat::Png => "png",
        }
    }

    /// Check if this format requires rendering
    pub fn requires_rendering(&self) -> bool {
        matches!(self, ExportFormat::Svg | ExportFormat::Png)
    }

    /// Get MIME type for this format
    pub fn mime_type(&self) -> &'static str {
        match self {
            ExportFormat::Dot => "text/vnd.graphviz",
            ExportFormat::Mermaid => "text/plain",
            ExportFormat::PlantUml => "text/plain",
            ExportFormat::Json => "application/json",
            ExportFormat::Svg => "image/svg+xml",
            ExportFormat::Png => "image/png",
        }
    }
}

/// Theme configuration for visualizations
#[derive(Debug, Clone, PartialEq)]
pub struct VisualizationTheme {
    /// Background color
    pub background_color: String,
    /// State node color
    pub state_color: String,
    /// Initial state color
    pub initial_state_color: String,
    /// Final state color
    pub final_state_color: String,
    /// Transition color
    pub transition_color: String,
    /// Guard condition color
    pub guard_color: String,
    /// Action color
    pub action_color: String,
    /// Error color
    pub error_color: String,
    /// Success color
    pub success_color: String,
}

impl Default for VisualizationTheme {
    fn default() -> Self {
        Self {
            background_color: "#ffffff".to_string(),
            state_color: "#e1f5fe".to_string(),
            initial_state_color: "#c8e6c9".to_string(),
            final_state_color: "#ffcdd2".to_string(),
            transition_color: "#666666".to_string(),
            guard_color: "#ff9800".to_string(),
            action_color: "#2196f3".to_string(),
            error_color: "#f44336".to_string(),
            success_color: "#4caf50".to_string(),
        }
    }
}

impl VisualizationTheme {
    /// Create a new theme with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set background color
    pub fn with_background(mut self, color: String) -> Self {
        self.background_color = color;
        self
    }

    /// Set state node color
    pub fn with_state_color(mut self, color: String) -> Self {
        self.state_color = color;
        self
    }

    /// Set initial state color
    pub fn with_initial_color(mut self, color: String) -> Self {
        self.initial_state_color = color;
        self
    }

    /// Set final state color
    pub fn with_final_color(mut self, color: String) -> Self {
        self.final_state_color = color;
        self
    }

    /// Create a dark theme
    pub fn dark() -> Self {
        Self {
            background_color: "#1e1e1e".to_string(),
            state_color: "#2d3748".to_string(),
            initial_state_color: "#22543d".to_string(),
            final_state_color: "#742a2a".to_string(),
            transition_color: "#a0aec0".to_string(),
            guard_color: "#ed8936".to_string(),
            action_color: "#3182ce".to_string(),
            error_color: "#e53e3e".to_string(),
            success_color: "#38a169".to_string(),
        }
    }

    /// Create a high contrast theme
    pub fn high_contrast() -> Self {
        Self {
            background_color: "#ffffff".to_string(),
            state_color: "#000000".to_string(),
            initial_state_color: "#000000".to_string(),
            final_state_color: "#000000".to_string(),
            transition_color: "#000000".to_string(),
            guard_color: "#000000".to_string(),
            action_color: "#000000".to_string(),
            error_color: "#ff0000".to_string(),
            success_color: "#000000".to_string(),
        }
    }
}

/// Rendering options for visualizations
#[derive(Debug, Clone, PartialEq)]
pub struct RenderingOptions {
    /// Image width in pixels
    pub width: u32,
    /// Image height in pixels
    pub height: u32,
    /// DPI for rendered images
    pub dpi: u32,
    /// Padding around the diagram
    pub padding: u32,
    /// Whether to include a legend
    pub include_legend: bool,
    /// Whether to include metadata
    pub include_metadata: bool,
}

impl Default for RenderingOptions {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            dpi: 96,
            padding: 20,
            include_legend: true,
            include_metadata: false,
        }
    }
}

impl RenderingOptions {
    /// Create new rendering options with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set dimensions
    pub fn with_dimensions(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set DPI
    pub fn with_dpi(mut self, dpi: u32) -> Self {
        self.dpi = dpi;
        self
    }

    /// Set padding
    pub fn with_padding(mut self, padding: u32) -> Self {
        self.padding = padding;
        self
    }

    /// Include legend
    pub fn with_legend(mut self, include: bool) -> Self {
        self.include_legend = include;
        self
    }

    /// Include metadata
    pub fn with_metadata(mut self, include: bool) -> Self {
        self.include_metadata = include;
        self
    }
}
