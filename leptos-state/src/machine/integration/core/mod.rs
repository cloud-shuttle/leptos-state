//! Core integration functionality

pub mod manager;
pub mod adapters;
pub mod health;
pub mod pipeline;

// Re-export the most commonly used items
pub use manager::{IntegrationManager, IntegrationStatistics};
pub use adapters::{IntegrationAdapterTrait, AdapterType, IntegrationAdapter, AdapterFactory};
pub use health::{HealthStatus, HealthCheckResult, HealthMonitor, HealthStatistics};
pub use pipeline::{IntegrationPipeline, IntegrationStep, PipelineConfig, PipelineStats, PipelineBuilder};
