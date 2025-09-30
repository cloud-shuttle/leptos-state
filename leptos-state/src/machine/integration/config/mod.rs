//! Integration configuration structures

pub mod core;
pub mod routing;
pub mod retry;
pub mod connection;

// Re-export the most commonly used items
pub use core::{IntegrationConfig, IntegrationConfigBuilder, factories as integration_factories};
pub use routing::{EventRoutingConfig, RoutingRule, EventPattern, EventTransformation, EventRoutingConfigBuilder};
pub use retry::{RetryConfig, RetryConfigBuilder, factories as retry_factories};
pub use connection::{ConnectionConfig, Credentials, PoolConfig, ConnectionConfigBuilder, factories as connection_factories};
