//! Core code generation configuration structures

/// Code generation configuration
#[derive(Debug, Clone, PartialEq)]
pub struct CodeGenConfig {
    /// Target programming language
    pub language: super::languages::ProgrammingLanguage,
    /// Whether to include comments in generated code
    pub include_comments: bool,
    /// Whether to include type annotations
    pub include_types: bool,
    /// Whether to generate async/await code
    pub async_code: bool,
    /// Indentation style
    pub indentation: IndentationStyle,
    /// Maximum line length
    pub max_line_length: usize,
    /// Whether to include validation code
    pub include_validation: bool,
    /// Whether to generate tests
    pub generate_tests: bool,
    /// Output directory
    pub output_dir: Option<String>,
    /// File naming pattern
    pub file_pattern: String,
}

impl Default for CodeGenConfig {
    fn default() -> Self {
        Self {
            language: super::languages::ProgrammingLanguage::Rust,
            include_comments: true,
            include_types: true,
            async_code: false,
            indentation: IndentationStyle::Spaces(4),
            max_line_length: 100,
            include_validation: true,
            generate_tests: false,
            output_dir: None,
            file_pattern: "{machine_name}.rs".to_string(),
        }
    }
}

impl CodeGenConfig {
    /// Create a new code generation config
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the target programming language
    pub fn language(mut self, language: super::languages::ProgrammingLanguage) -> Self {
        self.language = language;
        self
    }

    /// Enable or disable comments
    pub fn include_comments(mut self, include: bool) -> Self {
        self.include_comments = include;
        self
    }

    /// Enable or disable type annotations
    pub fn include_types(mut self, include: bool) -> Self {
        self.include_types = include;
        self
    }

    /// Enable or disable async code generation
    pub fn async_code(mut self, async_code: bool) -> Self {
        self.async_code = async_code;
        self
    }

    /// Set indentation style
    pub fn indentation(mut self, style: IndentationStyle) -> Self {
        self.indentation = style;
        self
    }

    /// Set maximum line length
    pub fn max_line_length(mut self, length: usize) -> Self {
        self.max_line_length = length;
        self
    }

    /// Enable or disable validation code
    pub fn include_validation(mut self, include: bool) -> Self {
        self.include_validation = include;
        self
    }

    /// Enable or disable test generation
    pub fn generate_tests(mut self, generate: bool) -> Self {
        self.generate_tests = generate;
        self
    }

    /// Set output directory
    pub fn output_dir<S: Into<String>>(mut self, dir: S) -> Self {
        self.output_dir = Some(dir.into());
        self
    }

    /// Set file naming pattern
    pub fn file_pattern<S: Into<String>>(mut self, pattern: S) -> Self {
        self.file_pattern = pattern.into();
        self
    }

    /// Get output path for a machine
    pub fn get_output_path(&self, machine_name: &str) -> String {
        let filename = self.file_pattern.replace("{machine_name}", machine_name);
        if let Some(ref dir) = self.output_dir {
            format!("{}/{}", dir, filename)
        } else {
            filename
        }
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.max_line_length < 50 {
            return Err("max_line_length must be at least 50".to_string());
        }

        if self.file_pattern.trim().is_empty() {
            return Err("file_pattern cannot be empty".to_string());
        }

        if !self.file_pattern.contains("{machine_name}") {
            return Err("file_pattern must contain {machine_name} placeholder".to_string());
        }

        Ok(())
    }

    /// Create a configuration optimized for debugging
    pub fn debug_config() -> Self {
        Self::new()
            .include_comments(true)
            .include_validation(true)
            .generate_tests(true)
            .indentation(IndentationStyle::Spaces(2))
            .max_line_length(120)
    }

    /// Create a configuration optimized for production
    pub fn production_config() -> Self {
        Self::new()
            .include_comments(false)
            .include_validation(false)
            .generate_tests(false)
            .indentation(IndentationStyle::Spaces(4))
            .max_line_length(100)
    }

    /// Get configuration summary
    pub fn summary(&self) -> String {
        format!(
            "CodeGenConfig {{ lang: {}, comments: {}, types: {}, async: {}, tests: {} }}",
            self.language.as_str(),
            self.include_comments,
            self.include_types,
            self.async_code,
            self.generate_tests
        )
    }
}

impl std::fmt::Display for CodeGenConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Indentation style for generated code
#[derive(Debug, Clone, PartialEq)]
pub enum IndentationStyle {
    /// Use spaces for indentation
    Spaces(usize),
    /// Use tabs for indentation
    Tabs,
}

impl IndentationStyle {
    /// Get the indentation string
    pub fn get_string(&self) -> String {
        match self {
            Self::Spaces(count) => " ".repeat(*count),
            Self::Tabs => "\t".to_string(),
        }
    }

    /// Create indentation with 2 spaces
    pub fn two_spaces() -> Self {
        Self::Spaces(2)
    }

    /// Create indentation with 4 spaces
    pub fn four_spaces() -> Self {
        Self::Spaces(4)
    }

    /// Create tab indentation
    pub fn tabs() -> Self {
        Self::Tabs
    }

    /// Get indentation level string
    pub fn indent_level(&self, level: usize) -> String {
        self.get_string().repeat(level)
    }
}

impl Default for IndentationStyle {
    fn default() -> Self {
        Self::Spaces(4)
    }
}

impl std::fmt::Display for IndentationStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Spaces(count) => write!(f, "{} spaces", count),
            Self::Tabs => write!(f, "tabs"),
        }
    }
}
