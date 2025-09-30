//! Persistence metadata structures and utilities

pub mod core;
pub mod schema;
pub mod stats;
pub mod builder;
pub mod utils;

// Re-export the most commonly used items
pub use core::MachineMetadata;
pub use schema::{SchemaInfo, ValidationRule, ValidationType};
pub use stats::{MachineStats, HealthStatus};
pub use builder::{MetadataBuilder, factories};
pub use utils::{MetadataUtils, CollectionStats, MetadataDiff, MetadataChange};
