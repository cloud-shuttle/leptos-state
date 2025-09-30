//! Integration metrics and performance monitoring

pub mod core;
pub mod adapter;
pub mod summary;
pub mod analysis;

// Re-export the most commonly used items
pub use core::{IntegrationMetrics, MetricsSnapshot};
pub use adapter::{AdapterMetrics, AdapterMetricsAggregator};
pub use summary::MetricsSummary;
pub use analysis::{Percentiles, ThroughputMetrics, ResourceUsage, PerformanceReport};
