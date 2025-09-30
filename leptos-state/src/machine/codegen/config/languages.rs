//! Programming language support for code generation

/// Programming languages for code generation
#[derive(Debug, Clone, PartialEq)]
pub enum ProgrammingLanguage {
    /// Rust programming language
    Rust,
    /// TypeScript
    TypeScript,
    /// JavaScript
    JavaScript,
    /// Python
    Python,
    /// C#
    CSharp,
    /// Java
    Java,
    /// Go
    Go,
    /// Swift
    Swift,
    /// Kotlin
    Kotlin,
    /// C++
    Cpp,
    /// PHP
    PHP,
    /// Ruby
    Ruby,
    /// Dart
    Dart,
    /// Scala
    Scala,
    /// Haskell
    Haskell,
    /// Elixir
    Elixir,
}

impl ProgrammingLanguage {
    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::TypeScript => "typescript",
            Self::JavaScript => "javascript",
            Self::Python => "python",
            Self::CSharp => "csharp",
            Self::Java => "java",
            Self::Go => "go",
            Self::Swift => "swift",
            Self::Kotlin => "kotlin",
            Self::Cpp => "cpp",
            Self::PHP => "php",
            Self::Ruby => "ruby",
            Self::Dart => "dart",
            Self::Scala => "scala",
            Self::Haskell => "haskell",
            Self::Elixir => "elixir",
        }
    }

    /// Get file extension
    pub fn file_extension(&self) -> &'static str {
        match self {
            Self::Rust => "rs",
            Self::TypeScript => "ts",
            Self::JavaScript => "js",
            Self::Python => "py",
            Self::CSharp => "cs",
            Self::Java => "java",
            Self::Go => "go",
            Self::Swift => "swift",
            Self::Kotlin => "kt",
            Self::Cpp => "cpp",
            Self::PHP => "php",
            Self::Ruby => "rb",
            Self::Dart => "dart",
            Self::Scala => "scala",
            Self::Haskell => "hs",
            Self::Elixir => "ex",
        }
    }

    /// Check if language supports async/await
    pub fn supports_async(&self) -> bool {
        matches!(
            self,
            Self::Rust
                | Self::TypeScript
                | Self::JavaScript
                | Self::Python
                | Self::CSharp
                | Self::Dart
                | Self::Kotlin
                | Self::Go
                | Self::Swift
        )
    }

    /// Check if language has static typing
    pub fn has_static_typing(&self) -> bool {
        !matches!(self, Self::JavaScript | Self::Python | Self::Ruby | Self::PHP)
    }

    /// Check if language is compiled
    pub fn is_compiled(&self) -> bool {
        matches!(
            self,
            Self::Rust
                | Self::Go
                | Self::CSharp
                | Self::Java
                | Self::Swift
                | Self::Kotlin
                | Self::Cpp
                | Self::Haskell
                | Self::Scala
        )
    }

    /// Check if language is interpreted
    pub fn is_interpreted(&self) -> bool {
        !self.is_compiled()
    }

    /// Check if language runs on JVM
    pub fn runs_on_jvm(&self) -> bool {
        matches!(self, Self::Java | Self::Kotlin | Self::Scala)
    }

    /// Check if language supports object-oriented programming
    pub fn supports_oop(&self) -> bool {
        !matches!(self, Self::Rust | Self::Go | Self::Haskell)
    }

    /// Check if language supports functional programming
    pub fn supports_functional(&self) -> bool {
        matches!(
            self,
            Self::Rust
                | Self::Haskell
                | Self::Scala
                | Self::Elixir
                | Self::FSharp
                | Self::Swift
                | Self::Python
                | Self::JavaScript
                | Self::TypeScript
        )
    }

    /// Get comment style
    pub fn comment_style(&self) -> CommentStyle {
        match self {
            Self::Rust | Self::Go | Self::Cpp | Self::CSharp | Self::Java | Self::Kotlin | Self::Scala | Self::Dart | Self::Swift => {
                CommentStyle::SlashSlash
            }
            Self::JavaScript | Self::TypeScript => CommentStyle::SlashSlash,
            Self::Python | Self::Ruby => CommentStyle::Hash,
            Self::PHP => CommentStyle::SlashSlashOrHash,
            Self::Haskell => CommentStyle::DoubleDash,
            Self::Elixir => CommentStyle::Hash,
        }
    }

    /// Get indentation preference
    pub fn preferred_indentation(&self) -> super::core::IndentationStyle {
        match self {
            Self::Python | Self::YAML => super::core::IndentationStyle::Spaces(4),
            Self::Go => super::core::IndentationStyle::Tabs,
            Self::Rust | Self::JavaScript | Self::TypeScript => super::core::IndentationStyle::Spaces(4),
            _ => super::core::IndentationStyle::Spaces(4),
        }
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "rust" | "rs" => Some(Self::Rust),
            "typescript" | "ts" => Some(Self::TypeScript),
            "javascript" | "js" => Some(Self::JavaScript),
            "python" | "py" => Some(Self::Python),
            "csharp" | "cs" | "c#" => Some(Self::CSharp),
            "java" => Some(Self::Java),
            "go" => Some(Self::Go),
            "swift" => Some(Self::Swift),
            "kotlin" | "kt" => Some(Self::Kotlin),
            "cpp" | "c++" => Some(Self::Cpp),
            "php" => Some(Self::PHP),
            "ruby" | "rb" => Some(Self::Ruby),
            "dart" => Some(Self::Dart),
            "scala" => Some(Self::Scala),
            "haskell" | "hs" => Some(Self::Haskell),
            "elixir" | "ex" => Some(Self::Elixir),
            _ => None,
        }
    }

    /// Get all supported languages
    pub fn all() -> Vec<Self> {
        vec![
            Self::Rust,
            Self::TypeScript,
            Self::JavaScript,
            Self::Python,
            Self::CSharp,
            Self::Java,
            Self::Go,
            Self::Swift,
            Self::Kotlin,
            Self::Cpp,
            Self::PHP,
            Self::Ruby,
            Self::Dart,
            Self::Scala,
            Self::Haskell,
            Self::Elixir,
        ]
    }

    /// Get compiled languages
    pub fn compiled() -> Vec<Self> {
        Self::all().into_iter().filter(|l| l.is_compiled()).collect()
    }

    /// Get interpreted languages
    pub fn interpreted() -> Vec<Self> {
        Self::all().into_iter().filter(|l| l.is_interpreted()).collect()
    }

    /// Get languages supporting async
    pub fn async_supported() -> Vec<Self> {
        Self::all().into_iter().filter(|l| l.supports_async()).collect()
    }

    /// Get languages with static typing
    pub fn statically_typed() -> Vec<Self> {
        Self::all().into_iter().filter(|l| l.has_static_typing()).collect()
    }
}

impl Default for ProgrammingLanguage {
    fn default() -> Self {
        Self::Rust
    }
}

impl std::fmt::Display for ProgrammingLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for ProgrammingLanguage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s).ok_or_else(|| format!("Unsupported programming language: {}", s))
    }
}

/// Comment style for different languages
#[derive(Debug, Clone, PartialEq)]
pub enum CommentStyle {
    /// // comments
    SlashSlash,
    /// # comments
    Hash,
    /// -- comments
    DoubleDash,
    /// /* */ block comments
    SlashStar,
    /// // or # comments
    SlashSlashOrHash,
}

impl CommentStyle {
    /// Get single line comment prefix
    pub fn single_line_prefix(&self) -> &'static str {
        match self {
            Self::SlashSlash => "//",
            Self::Hash => "#",
            Self::DoubleDash => "--",
            Self::SlashStar => "/*",
            Self::SlashSlashOrHash => "//",
        }
    }

    /// Get block comment start
    pub fn block_start(&self) -> Option<&'static str> {
        match self {
            Self::SlashStar => Some("/*"),
            _ => None,
        }
    }

    /// Get block comment end
    pub fn block_end(&self) -> Option<&'static str> {
        match self {
            Self::SlashStar => Some("*/"),
            _ => None,
        }
    }

    /// Check if supports block comments
    pub fn supports_block_comments(&self) -> bool {
        self.block_start().is_some()
    }
}

/// Language feature flags
#[derive(Debug, Clone)]
pub struct LanguageFeatures {
    /// Supports async/await
    pub async_await: bool,
    /// Has static typing
    pub static_typing: bool,
    /// Is compiled language
    pub compiled: bool,
    /// Supports OOP
    pub object_oriented: bool,
    /// Supports functional programming
    pub functional: bool,
    /// Runs on JVM
    pub jvm_based: bool,
}

impl LanguageFeatures {
    /// Get features for a programming language
    pub fn for_language(lang: &ProgrammingLanguage) -> Self {
        Self {
            async_await: lang.supports_async(),
            static_typing: lang.has_static_typing(),
            compiled: lang.is_compiled(),
            object_oriented: lang.supports_oop(),
            functional: lang.supports_functional(),
            jvm_based: lang.runs_on_jvm(),
        }
    }

    /// Check if language is suitable for systems programming
    pub fn is_systems_programming(&self) -> bool {
        self.compiled && self.static_typing
    }

    /// Check if language is suitable for web development
    pub fn is_web_development(&self) -> bool {
        self.async_await || !self.compiled
    }

    /// Check if language supports modern programming paradigms
    pub fn is_modern(&self) -> bool {
        self.async_await && self.functional
    }
}
