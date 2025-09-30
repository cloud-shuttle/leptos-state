//! Configuration structures and utilities

pub mod types;
pub mod core;
pub mod logging;
pub mod environment;
pub mod sources;
pub mod manager;

// Re-export the most commonly used items
pub use types::{StoreId, MachineId, StateId, EventId, IdentifierUtils, formatting};
pub use core::Config;
pub use logging::{LogLevel, LoggerConfig};
pub use environment::{Environment, EnvironmentLoader};
pub use sources::{ConfigSource, ConfigLoader, ConfigBuilder};
pub use manager::{ConfigManager, ConfigStats, global_manager, current_config};
