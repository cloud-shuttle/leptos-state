//! Integration configuration structures

pub mod core;
pub mod routing;
pub mod retry;
pub mod connection;

// Re-export the most commonly used items
pub use core::{IntegrationConfig, IntegrationConfigBuilder, factories as integration_factories};
pub use routing::config::{EventRoutingConfig, EventRoutingConfigBuilder};
pub use routing::rules::RoutingRule;
pub use routing::patterns::EventPattern;
pub use routing::transformations::EventTransformation;
pub use retry::{RetryConfig, RetryConfigBuilder, factories as retry_factories};
pub use connection::{ConnectionConfig, Credentials, PoolConfig, ConnectionConfigBuilder, factories as connection_factories};
