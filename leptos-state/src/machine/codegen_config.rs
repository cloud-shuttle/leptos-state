//! Code generation configuration and programming languages

use super::*;

/// Code generation configuration
#[derive(Debug, Clone, PartialEq)]
pub struct CodeGenConfig {
    /// Target programming language
    pub language: ProgrammingLanguage,
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

#[derive(Debug, Clone, PartialEq)]
pub enum IndentationStyle {
    /// Use spaces for indentation
    Spaces(usize),
    /// Use tabs for indentation
    Tabs,
}

impl Default for CodeGenConfig {
    fn default() -> Self {
        Self {
            language: ProgrammingLanguage::Rust,
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
    /// Create a new configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the target language
    pub fn language(mut self, language: ProgrammingLanguage) -> Self {
        self.language = language;
        self
    }

    /// Include comments in generated code
    pub fn with_comments(mut self, include: bool) -> Self {
        self.include_comments = include;
        self
    }

    /// Include type annotations
    pub fn with_types(mut self, include: bool) -> Self {
        self.include_types = include;
        self
    }

    /// Generate async/await code
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

    /// Include validation code
    pub fn with_validation(mut self, include: bool) -> Self {
        self.include_validation = include;
        self
    }

    /// Generate tests
    pub fn with_tests(mut self, generate: bool) -> Self {
        self.generate_tests = generate;
        self
    }

    /// Set output directory
    pub fn output_dir(mut self, dir: String) -> Self {
        self.output_dir = Some(dir);
        self
    }

    /// Set file naming pattern
    pub fn file_pattern(mut self, pattern: String) -> Self {
        self.file_pattern = pattern;
        self
    }

    /// Get indentation string
    pub fn indent_string(&self) -> String {
        match &self.indentation {
            IndentationStyle::Spaces(n) => " ".repeat(*n),
            IndentationStyle::Tabs => "\t".to_string(),
        }
    }
}

/// Programming languages for code generation
#[derive(Debug, Clone, PartialEq)]
pub enum ProgrammingLanguage {
    /// Rust programming language
    Rust,
    /// TypeScript/JavaScript
    TypeScript,
    /// Python programming language
    Python,
    /// Go programming language
    Go,
    /// Java programming language
    Java,
    /// C# programming language
    CSharp,
    /// Kotlin programming language
    Kotlin,
    /// Swift programming language
    Swift,
}

impl ProgrammingLanguage {
    /// Get file extension for this language
    pub fn extension(&self) -> &'static str {
        match self {
            ProgrammingLanguage::Rust => "rs",
            ProgrammingLanguage::TypeScript => "ts",
            ProgrammingLanguage::Python => "py",
            ProgrammingLanguage::Go => "go",
            ProgrammingLanguage::Java => "java",
            ProgrammingLanguage::CSharp => "cs",
            ProgrammingLanguage::Kotlin => "kt",
            ProgrammingLanguage::Swift => "swift",
        }
    }

    /// Get comment syntax for this language
    pub fn comment_syntax(&self) -> (&'static str, Option<&'static str>) {
        match self {
            ProgrammingLanguage::Rust | ProgrammingLanguage::Go | ProgrammingLanguage::CSharp |
            ProgrammingLanguage::Java | ProgrammingLanguage::Kotlin | ProgrammingLanguage::Swift => {
                ("//", Some("/* */"))
            }
            ProgrammingLanguage::TypeScript => ("//", Some("/* */")),
            ProgrammingLanguage::Python => ("#", Some("\"\"\" \"\"\"")),
        }
    }

    /// Check if language supports async/await
    pub fn supports_async(&self) -> bool {
        matches!(self, ProgrammingLanguage::Rust | ProgrammingLanguage::TypeScript |
                       ProgrammingLanguage::Python | ProgrammingLanguage::CSharp |
                       ProgrammingLanguage::Kotlin)
    }
}

/// Code generation templates for different languages
pub struct CodeTemplates {
    /// Language this template is for
    pub language: ProgrammingLanguage,
    /// Template for state machine class/struct
    pub machine_template: String,
    /// Template for state definitions
    pub state_template: String,
    /// Template for transition definitions
    pub transition_template: String,
    /// Template for guard definitions
    pub guard_template: String,
    /// Template for action definitions
    pub action_template: String,
    /// Template for event definitions
    pub event_template: String,
}

impl CodeTemplates {
    /// Create default templates for a language
    pub fn for_language(language: ProgrammingLanguage) -> Self {
        match language {
            ProgrammingLanguage::Rust => Self::rust_templates(),
            ProgrammingLanguage::TypeScript => Self::typescript_templates(),
            ProgrammingLanguage::Python => Self::python_templates(),
            _ => Self::rust_templates(), // Fallback
        }
    }

    /// Rust code templates
    fn rust_templates() -> Self {
        Self {
            language: ProgrammingLanguage::Rust,
            machine_template: r#"
/// Auto-generated state machine: {machine_name}
#[derive(Debug, Clone)]
pub struct {machine_name}<C, E> {{
    current_state: String,
    context: C,
    _phantom: std::marker::PhantomData<E>,
}}

impl<C, E> {machine_name}<C, E> {{
    /// Create a new state machine instance
    pub fn new(initial_context: C) -> Self {{
        Self {{
            current_state: "{initial_state}".to_string(),
            context: initial_context,
            _phantom: std::marker::PhantomData,
        }}
    }}

    /// Get current state
    pub fn current_state(&self) -> &str {{
        &self.current_state
    }}

    /// Get current context
    pub fn context(&self) -> &C {{
        &self.context
    }}

    /// Get mutable context
    pub fn context_mut(&mut self) -> &mut C {{
        &mut self.context
    }}
}}
"#.to_string(),
            state_template: r#"
    /// State: {state_name}
    const {state_name}: &str = "{state_name}";
"#.to_string(),
            transition_template: r#"
    /// Transition from {from_state} to {to_state}
    pub fn {transition_name}(&mut self{event_param}) -> Result<(), String> {{
        if self.current_state != "{from_state}" {{
            return Err("Invalid state for transition".to_string());
        }}
        // Guard checks would go here
        // Actions would be executed here
        self.current_state = "{to_state}".to_string();
        Ok(())
    }}
"#.to_string(),
            guard_template: r#"
    /// Guard: {guard_name}
    fn {guard_name}(&self{context_param}) -> bool {{
        // Guard implementation
        true
    }}
"#.to_string(),
            action_template: r#"
    /// Action: {action_name}
    fn {action_name}(&mut self{context_param}) {{
        // Action implementation
    }}
"#.to_string(),
            event_template: r#"
    /// Event: {event_name}
    #[derive(Debug, Clone)]
    pub struct {event_name};
"#.to_string(),
        }
    }

    /// TypeScript code templates
    fn typescript_templates() -> Self {
        Self {
            language: ProgrammingLanguage::TypeScript,
            machine_template: r#"
/**
 * Auto-generated state machine: {machine_name}
 */
export class {machine_name}<C, E> {{
    private currentState: string;
    private context: C;

    constructor(initialContext: C) {{
        this.currentState = "{initial_state}";
        this.context = initialContext;
    }}

    /**
     * Get current state
     */
    public getCurrentState(): string {{
        return this.currentState;
    }}

    /**
     * Get current context
     */
    public getContext(): C {{
        return this.context;
    }}

    /**
     * Get mutable context
     */
    public getContextMut(): C {{
        return this.context;
    }}
}}
"#.to_string(),
            state_template: r#"
    // State: {state_name}
    public static readonly {state_name}: string = "{state_name}";
"#.to_string(),
            transition_template: r#"
    /**
     * Transition from {from_state} to {to_state}
     */
    public {transition_name}({event_param}): void {{
        if (this.currentState !== "{from_state}") {{
            throw new Error("Invalid state for transition");
        }}
        // Guard checks would go here
        // Actions would be executed here
        this.currentState = "{to_state}";
    }}
"#.to_string(),
            guard_template: r#"
    /**
     * Guard: {guard_name}
     */
    private {guard_name}({context_param}): boolean {{
        // Guard implementation
        return true;
    }}
"#.to_string(),
            action_template: r#"
    /**
     * Action: {action_name}
     */
    private {action_name}({context_param}): void {{
        // Action implementation
    }}
"#.to_string(),
            event_template: r#"
    /**
     * Event: {event_name}
     */
    export class {event_name} {{
        // Event implementation
    }}
"#.to_string(),
        }
    }

    /// Python code templates
    fn python_templates() -> Self {
        Self {
            language: ProgrammingLanguage::Python,
            machine_template: r#"# Auto-generated state machine: {machine_name}
class {machine_name}:
    def __init__(self, initial_context):
        self.current_state = "{initial_state}"
        self.context = initial_context

    def get_current_state(self):
        return self.current_state

    def get_context(self):
        return self.context

    def get_context_mut(self):
        return self.context
"#.to_string(),
            state_template: r#"
    # State: {state_name}
    {state_name} = "{state_name}"
"#.to_string(),
            transition_template: r#"
    def {transition_name}(self{event_param}):
        if self.current_state != "{from_state}":
            raise ValueError("Invalid state for transition")
        # Guard checks would go here
        # Actions would be executed here
        self.current_state = "{to_state}"
"#.to_string(),
            guard_template: r#"
    def {guard_name}(self{context_param}):
        # Guard implementation
        return True
"#.to_string(),
            action_template: r#"
    def {action_name}(self{context_param}):
        # Action implementation
        pass
"#.to_string(),
            event_template: r#"# Event: {event_name}
class {event_name}:
    pass
"#.to_string(),
        }
    }
}

/// Code generation options
#[derive(Debug, Clone, PartialEq)]
pub struct CodeGenOptions {
    /// Whether to generate separate files
    pub separate_files: bool,
    /// Whether to include imports
    pub include_imports: bool,
    /// Whether to include module declarations
    pub include_modules: bool,
    /// Custom header to include
    pub custom_header: Option<String>,
    /// Custom footer to include
    pub custom_footer: Option<String>,
}

impl Default for CodeGenOptions {
    fn default() -> Self {
        Self {
            separate_files: false,
            include_imports: true,
            include_modules: true,
            custom_header: None,
            custom_footer: None,
        }
    }
}

impl CodeGenOptions {
    /// Generate separate files
    pub fn separate_files(mut self, separate: bool) -> Self {
        self.separate_files = separate;
        self
    }

    /// Include imports
    pub fn with_imports(mut self, include: bool) -> Self {
        self.include_imports = include;
        self
    }

    /// Include module declarations
    pub fn with_modules(mut self, include: bool) -> Self {
        self.include_modules = include;
        self
    }

    /// Set custom header
    pub fn header(mut self, header: String) -> Self {
        self.custom_header = Some(header);
        self
    }

    /// Set custom footer
    pub fn footer(mut self, footer: String) -> Self {
        self.custom_footer = Some(header);
        self
    }
}
