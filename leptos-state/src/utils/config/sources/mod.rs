//! Modular configuration sources
//!
//! This module provides various configuration source implementations
//! that can load configuration from different backends.

pub mod traits;
pub mod file;
pub mod http;
pub mod database;
pub mod environment;
pub mod custom;
pub mod compatibility;

// Re-export commonly used types for convenience
pub use traits::{ConfigSourceTrait, ConfigError, SourceMetadata};
pub use file::{FileSource};
pub use http::{HttpSource, RemoteUrl};
pub use database::{DatabaseSource, Database};
pub use environment::{EnvironmentSource, Environment};
pub use custom::{CustomSource, CustomSourceWithValidation, BoxedCustomSource, boxed};

// Legacy compatibility types (deprecated - use new trait-based sources)
pub use compatibility::{ConfigSource, ConfigLoader, ConfigBuilder};

// Legacy aliases for backward compatibility
pub type JsonFile = FileSource;
#[cfg(feature = "toml")]
pub type TomlFile = FileSource;
pub type YamlFile = FileSource;
