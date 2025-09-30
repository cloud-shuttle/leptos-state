//! Persistence extensions and utilities

pub mod core;
pub mod machine;
pub mod info;
pub mod builder;
pub mod migration;
pub mod monitoring;

// Re-export the most commonly used items
pub use core::MachinePersistenceExt;
pub use machine::PersistentMachine;
pub use info::PersistenceInfo;
pub use builder::{PersistenceBuilder, factories};
pub use migration::{Migration, MigrationManager, VersionMigration, TransformMigration};
pub use monitoring::{PersistenceMonitor, PersistenceReport, PersistenceStats};
