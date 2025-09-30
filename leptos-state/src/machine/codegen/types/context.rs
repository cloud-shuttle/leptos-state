//! Code generation context

use crate::machine::codegen_config::ProgrammingLanguage;

/// Code generation context
#[derive(Debug, Clone)]
pub struct CodeGenContext {
    /// Target programming language
    pub language: ProgrammingLanguage,
    /// Output directory
    pub output_dir: std::path::PathBuf,
    /// Package/module name
    pub package_name: String,
    /// Author information
    pub author: Option<String>,
    /// Version information
    pub version: Option<String>,
    /// Generation options
    pub options: CodeGenOptions,
    /// Metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl CodeGenContext {
    /// Create a new code generation context
    pub fn new(language: ProgrammingLanguage, output_dir: std::path::PathBuf) -> Self {
        Self {
            language,
            output_dir,
            package_name: "generated".to_string(),
            author: None,
            version: None,
            options: CodeGenOptions::default(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Set package name
    pub fn with_package_name(mut self, name: String) -> Self {
        self.package_name = name;
        self
    }

    /// Set author
    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    /// Set version
    pub fn with_version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }

    /// Set options
    pub fn with_options(mut self, options: CodeGenOptions) -> Self {
        self.options = options;
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Get output path for a file
    pub fn output_path(&self, file_name: &str) -> std::path::PathBuf {
        self.output_dir.join(file_name)
    }

    /// Get file extension for the target language
    pub fn file_extension(&self) -> &str {
        self.language.extension()
    }

    /// Get language name
    pub fn language_name(&self) -> &str {
        self.language.as_str()
    }

    /// Check if generation should include comments
    pub fn should_include_comments(&self) -> bool {
        self.options.include_comments
    }

    /// Check if generation should include documentation
    pub fn should_include_docs(&self) -> bool {
        self.options.include_docs
    }

    /// Check if generation should optimize output
    pub fn should_optimize(&self) -> bool {
        self.options.optimize
    }

    /// Get author or default
    pub fn author(&self) -> &str {
        self.author.as_deref().unwrap_or("Generated")
    }

    /// Get version or default
    pub fn version(&self) -> &str {
        self.version.as_deref().unwrap_or("1.0.0")
    }

    /// Get metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// Check if context is valid
    pub fn is_valid(&self) -> bool {
        !self.package_name.is_empty() && self.output_dir.exists()
    }
}

/// Code generation options
#[derive(Debug, Clone)]
pub struct CodeGenOptions {
    /// Include comments in generated code
    pub include_comments: bool,
    /// Include documentation in generated code
    pub include_docs: bool,
    /// Optimize generated code
    pub optimize: bool,
    /// Indentation style
    pub indentation: IndentationStyle,
    /// Maximum line length
    pub max_line_length: usize,
}

impl CodeGenOptions {
    /// Create new options with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable comments
    pub fn with_comments(mut self, include: bool) -> Self {
        self.include_comments = include;
        self
    }

    /// Enable documentation
    pub fn with_docs(mut self, include: bool) -> Self {
        self.include_docs = include;
        self
    }

    /// Enable optimization
    pub fn with_optimization(mut self, optimize: bool) -> Self {
        self.optimize = optimize;
        self
    }

    /// Set indentation style
    pub fn with_indentation(mut self, style: IndentationStyle) -> Self {
        self.indentation = style;
        self
    }

    /// Set max line length
    pub fn with_max_line_length(mut self, length: usize) -> Self {
        self.max_line_length = length;
        self
    }
}

impl Default for CodeGenOptions {
    fn default() -> Self {
        Self {
            include_comments: true,
            include_docs: true,
            optimize: false,
            indentation: IndentationStyle::Spaces(4),
            max_line_length: 100,
        }
    }
}

/// Indentation style
#[derive(Debug, Clone)]
pub enum IndentationStyle {
    /// Use spaces
    Spaces(usize),
    /// Use tabs
    Tabs,
}

impl IndentationStyle {
    /// Get indentation string
    pub fn to_string(&self, level: usize) -> String {
        match self {
            IndentationStyle::Spaces(count) => " ".repeat(count * level),
            IndentationStyle::Tabs => "\t".repeat(level),
        }
    }

    /// Get single indentation unit
    pub fn unit(&self) -> &str {
        match self {
            IndentationStyle::Spaces(_) => " ",
            IndentationStyle::Tabs => "\t",
        }
    }
}

impl Default for CodeGenContext {
    fn default() -> Self {
        Self::new(ProgrammingLanguage::Rust, std::path::PathBuf::from("."))
    }
}
