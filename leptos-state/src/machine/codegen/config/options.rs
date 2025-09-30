//! Advanced code generation options

/// Code generation options
#[derive(Debug, Clone, PartialEq)]
pub struct CodeGenOptions {
    /// Whether to generate documentation
    pub generate_docs: bool,
    /// Whether to optimize for performance
    pub optimize_performance: bool,
    /// Whether to optimize for size
    pub optimize_size: bool,
    /// Whether to include debug information
    pub include_debug_info: bool,
    /// Whether to generate benchmarks
    pub generate_benchmarks: bool,
    /// Target platform/architecture
    pub target_platform: Option<String>,
    /// Custom compiler flags
    pub compiler_flags: Vec<String>,
    /// Dependencies to include
    pub dependencies: Vec<String>,
    /// Whether to generate examples
    pub generate_examples: bool,
    /// Code quality level (1-5)
    pub code_quality_level: u8,
    /// Whether to include error handling
    pub include_error_handling: bool,
    /// Whether to generate serialization support
    pub generate_serialization: bool,
    /// Custom metadata
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

impl Default for CodeGenOptions {
    fn default() -> Self {
        Self {
            generate_docs: true,
            optimize_performance: false,
            optimize_size: false,
            include_debug_info: false,
            generate_benchmarks: false,
            target_platform: None,
            compiler_flags: Vec::new(),
            dependencies: Vec::new(),
            generate_examples: false,
            code_quality_level: 3,
            include_error_handling: true,
            generate_serialization: false,
            metadata: std::collections::HashMap::new(),
        }
    }
}

impl CodeGenOptions {
    /// Create new options
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable documentation generation
    pub fn with_docs(mut self, generate: bool) -> Self {
        self.generate_docs = generate;
        self
    }

    /// Enable performance optimization
    pub fn optimize_performance(mut self, optimize: bool) -> Self {
        self.optimize_performance = optimize;
        self
    }

    /// Enable size optimization
    pub fn optimize_size(mut self, optimize: bool) -> Self {
        self.optimize_size = optimize;
        self
    }

    /// Include debug information
    pub fn with_debug_info(mut self, include: bool) -> Self {
        self.include_debug_info = include;
        self
    }

    /// Enable benchmark generation
    pub fn with_benchmarks(mut self, generate: bool) -> Self {
        self.generate_benchmarks = generate;
        self
    }

    /// Set target platform
    pub fn target_platform<S: Into<String>>(mut self, platform: S) -> Self {
        self.target_platform = Some(platform.into());
        self
    }

    /// Add compiler flag
    pub fn add_compiler_flag<S: Into<String>>(mut self, flag: S) -> Self {
        self.compiler_flags.push(flag.into());
        self
    }

    /// Add dependency
    pub fn add_dependency<S: Into<String>>(mut self, dependency: S) -> Self {
        self.dependencies.push(dependency.into());
        self
    }

    /// Enable example generation
    pub fn with_examples(mut self, generate: bool) -> Self {
        self.generate_examples = generate;
        self
    }

    /// Set code quality level
    pub fn code_quality_level(mut self, level: u8) -> Self {
        self.code_quality_level = level.clamp(1, 5);
        self
    }

    /// Include error handling
    pub fn with_error_handling(mut self, include: bool) -> Self {
        self.include_error_handling = include;
        self
    }

    /// Enable serialization generation
    pub fn with_serialization(mut self, generate: bool) -> Self {
        self.generate_serialization = generate;
        self
    }

    /// Add custom metadata
    pub fn add_metadata<K: Into<String>, V: Into<serde_json::Value>>(mut self, key: K, value: V) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Validate options
    pub fn validate(&self) -> Result<(), String> {
        if self.code_quality_level == 0 || self.code_quality_level > 5 {
            return Err("code_quality_level must be between 1 and 5".to_string());
        }

        // Check for conflicting optimizations
        if self.optimize_performance && self.optimize_size {
            return Err("Cannot optimize for both performance and size simultaneously".to_string());
        }

        Ok(())
    }

    /// Get optimization strategy
    pub fn optimization_strategy(&self) -> OptimizationStrategy {
        if self.optimize_performance {
            OptimizationStrategy::Performance
        } else if self.optimize_size {
            OptimizationStrategy::Size
        } else {
            OptimizationStrategy::Balanced
        }
    }

    /// Check if options are suitable for production
    pub fn is_production_ready(&self) -> bool {
        self.include_error_handling && self.code_quality_level >= 3
    }

    /// Check if options are suitable for development
    pub fn is_development_ready(&self) -> bool {
        self.include_debug_info && self.generate_tests
    }

    /// Get required dependencies for the current configuration
    pub fn required_dependencies(&self) -> Vec<String> {
        let mut deps = self.dependencies.clone();

        if self.generate_serialization {
            deps.push("serde".to_string());
        }

        if self.generate_benchmarks {
            deps.push("criterion".to_string());
        }

        deps
    }

    /// Get recommended compiler flags
    pub fn recommended_compiler_flags(&self) -> Vec<String> {
        let mut flags = self.compiler_flags.clone();

        match self.optimization_strategy() {
            OptimizationStrategy::Performance => {
                flags.push("-O3".to_string());
                flags.push("--inline-functions".to_string());
            }
            OptimizationStrategy::Size => {
                flags.push("-Os".to_string());
                flags.push("--gc-sections".to_string());
            }
            OptimizationStrategy::Balanced => {
                flags.push("-O2".to_string());
            }
        }

        if self.include_debug_info {
            flags.push("-g".to_string());
        }

        flags
    }

    /// Merge with another options instance
    pub fn merge(&mut self, other: &CodeGenOptions) {
        if other.generate_docs {
            self.generate_docs = true;
        }

        if other.optimize_performance {
            self.optimize_performance = true;
        }

        if other.optimize_size {
            self.optimize_size = true;
        }

        if other.include_debug_info {
            self.include_debug_info = true;
        }

        if other.generate_benchmarks {
            self.generate_benchmarks = true;
        }

        if self.target_platform.is_none() {
            self.target_platform = other.target_platform.clone();
        }

        // Merge compiler flags (avoid duplicates)
        for flag in &other.compiler_flags {
            if !self.compiler_flags.contains(flag) {
                self.compiler_flags.push(flag.clone());
            }
        }

        // Merge dependencies (avoid duplicates)
        for dep in &other.dependencies {
            if !self.dependencies.contains(dep) {
                self.dependencies.push(dep.clone());
            }
        }

        if other.generate_examples {
            self.generate_examples = true;
        }

        self.code_quality_level = self.code_quality_level.max(other.code_quality_level);

        if other.include_error_handling {
            self.include_error_handling = true;
        }

        if other.generate_serialization {
            self.generate_serialization = true;
        }

        // Merge metadata (self takes precedence)
        for (key, value) in &other.metadata {
            if !self.metadata.contains_key(key) {
                self.metadata.insert(key.clone(), value.clone());
            }
        }
    }

    /// Create options for maximum performance
    pub fn maximum_performance() -> Self {
        Self::new()
            .optimize_performance(true)
            .with_debug_info(false)
            .code_quality_level(5)
            .with_error_handling(false)
    }

    /// Create options for minimum size
    pub fn minimum_size() -> Self {
        Self::new()
            .optimize_size(true)
            .with_debug_info(false)
            .code_quality_level(3)
    }

    /// Create options for development
    pub fn development() -> Self {
        Self::new()
            .with_debug_info(true)
            .generate_tests(true)
            .generate_examples(true)
            .code_quality_level(4)
    }

    /// Get options summary
    pub fn summary(&self) -> String {
        format!(
            "CodeGenOptions(quality: {}, optimization: {}, debug: {}, tests: {})",
            self.code_quality_level,
            self.optimization_strategy().as_str(),
            self.include_debug_info,
            self.generate_tests
        )
    }
}

impl std::fmt::Display for CodeGenOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Optimization strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationStrategy {
    /// Optimize for performance
    Performance,
    /// Optimize for size
    Size,
    /// Balanced optimization
    Balanced,
}

impl OptimizationStrategy {
    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Performance => "performance",
            Self::Size => "size",
            Self::Balanced => "balanced",
        }
    }
}

impl std::fmt::Display for OptimizationStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Options builder for fluent construction
pub struct CodeGenOptionsBuilder {
    options: CodeGenOptions,
}

impl CodeGenOptionsBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            options: CodeGenOptions::new(),
        }
    }

    /// Set all options for maximum performance
    pub fn maximum_performance() -> Self {
        Self {
            options: CodeGenOptions::maximum_performance(),
        }
    }

    /// Set all options for minimum size
    pub fn minimum_size() -> Self {
        Self {
            options: CodeGenOptions::minimum_size(),
        }
    }

    /// Set all options for development
    pub fn development() -> Self {
        Self {
            options: CodeGenOptions::development(),
        }
    }

    /// Configure documentation generation
    pub fn docs(mut self, generate: bool) -> Self {
        self.options.generate_docs = generate;
        self
    }

    /// Configure performance optimization
    pub fn performance_optimization(mut self, optimize: bool) -> Self {
        self.options.optimize_performance = optimize;
        self
    }

    /// Configure size optimization
    pub fn size_optimization(mut self, optimize: bool) -> Self {
        self.options.optimize_size = optimize;
        self
    }

    /// Configure debug information
    pub fn debug_info(mut self, include: bool) -> Self {
        self.options.include_debug_info = include;
        self
    }

    /// Configure benchmark generation
    pub fn benchmarks(mut self, generate: bool) -> Self {
        self.options.generate_benchmarks = generate;
        self
    }

    /// Configure example generation
    pub fn examples(mut self, generate: bool) -> Self {
        self.options.generate_examples = generate;
        self
    }

    /// Configure serialization generation
    pub fn serialization(mut self, generate: bool) -> Self {
        self.options.generate_serialization = generate;
        self
    }

    /// Set code quality level
    pub fn quality_level(mut self, level: u8) -> Self {
        self.options.code_quality_level = level;
        self
    }

    /// Add a dependency
    pub fn dependency<S: Into<String>>(mut self, dep: S) -> Self {
        self.options.dependencies.push(dep.into());
        self
    }

    /// Add a compiler flag
    pub fn compiler_flag<S: Into<String>>(mut self, flag: S) -> Self {
        self.options.compiler_flags.push(flag.into());
        self
    }

    /// Add custom metadata
    pub fn metadata<K: Into<String>, V: Into<serde_json::Value>>(mut self, key: K, value: V) -> Self {
        self.options.metadata.insert(key.into(), value.into());
        self
    }

    /// Build the options
    pub fn build(self) -> CodeGenOptions {
        self.options
    }

    /// Build and validate the options
    pub fn build_validated(self) -> Result<CodeGenOptions, String> {
        let options = self.build();
        options.validate()?;
        Ok(options)
    }
}

impl Default for CodeGenOptionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience functions for creating code generation options
pub mod factories {
    use super::*;

    /// Create default options
    pub fn default() -> CodeGenOptions {
        CodeGenOptions::default()
    }

    /// Create options for maximum performance
    pub fn maximum_performance() -> CodeGenOptions {
        CodeGenOptions::maximum_performance()
    }

    /// Create options for minimum size
    pub fn minimum_size() -> CodeGenOptions {
        CodeGenOptions::minimum_size()
    }

    /// Create options for development
    pub fn development() -> CodeGenOptions {
        CodeGenOptions::development()
    }

    /// Create custom options using builder
    pub fn custom<F>(f: F) -> CodeGenOptions
    where
        F: FnOnce(CodeGenOptionsBuilder) -> CodeGenOptionsBuilder,
    {
        let builder = CodeGenOptionsBuilder::new();
        f(builder).build()
    }
}
