//! Code generation configuration and related structures

pub mod core;
pub mod languages;
pub mod templates;
pub mod options;

// Re-export the most commonly used items
pub use core::{CodeGenConfig, IndentationStyle};
pub use languages::{ProgrammingLanguage, CommentStyle, LanguageFeatures};
pub use templates::{CodeTemplates, TemplateRenderer, TemplateStatistics};
pub use options::{CodeGenOptions, OptimizationStrategy, CodeGenOptionsBuilder, factories};
