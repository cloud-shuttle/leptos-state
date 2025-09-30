//! Performance configuration and optimization strategies

use super::*;
use std::time::Duration;

/// Performance configuration for state machines
#[derive(Debug, Clone, PartialEq)]
pub struct PerformanceConfig {
    /// Enable caching of transition results
    pub enable_caching: bool,
    /// Maximum cache size in bytes
    pub max_cache_size: usize,
    /// Cache TTL for entries
    pub cache_ttl: Duration,
    /// Enable lazy evaluation of expensive operations
    pub enable_lazy_evaluation: bool,
    /// Enable performance profiling
    pub enable_profiling: bool,
    /// Profile sample rate (1.0 = sample all, 0.1 = sample 10%)
    pub profile_sample_rate: f64,
    /// Enable memory tracking
    pub enable_memory_tracking: bool,
    /// Maximum memory usage before triggering optimization
    pub max_memory_usage: usize,
    /// Optimization strategy to use
    pub optimization_strategy: OptimizationStrategy,
    /// Whether to enable parallel processing
    pub enable_parallel_processing: bool,
    /// Number of worker threads for parallel processing
    pub worker_threads: usize,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_caching: true,
            max_cache_size: 10 * 1024 * 1024,    // 10MB
            cache_ttl: Duration::from_secs(300), // 5 minutes
            enable_lazy_evaluation: true,
            enable_profiling: false,
            profile_sample_rate: 0.1, // 10% sampling
            enable_memory_tracking: true,
            max_memory_usage: 50 * 1024 * 1024, // 50MB
            optimization_strategy: OptimizationStrategy::Balanced,
            enable_parallel_processing: false,
            worker_threads: num_cpus::get(),
        }
    }
}

/// Performance optimization strategies
#[derive(Debug, Clone, PartialEq)]
pub enum OptimizationStrategy {
    /// Prioritize speed over memory usage
    Speed,
    /// Balance speed and memory usage
    Balanced,
    /// Prioritize memory efficiency over speed
    Memory,
    /// Aggressive optimization for high-throughput scenarios
    Aggressive,
    /// Conservative optimization for low-resource environments
    Conservative,
    /// Custom optimization strategy
    Custom(Vec<String>),
}

impl OptimizationStrategy {
    /// Get the strategy name
    pub fn name(&self) -> &str {
        match self {
            OptimizationStrategy::Speed => "speed",
            OptimizationStrategy::Balanced => "balanced",
            OptimizationStrategy::Memory => "memory",
            OptimizationStrategy::Aggressive => "aggressive",
            OptimizationStrategy::Conservative => "conservative",
            OptimizationStrategy::Custom(_) => "custom",
        }
    }

    /// Get optimization parameters for this strategy
    pub fn parameters(&self) -> OptimizationParameters {
        match self {
            OptimizationStrategy::Speed => OptimizationParameters {
                cache_priority: 0.9,
                memory_priority: 0.1,
                cpu_priority: 0.8,
                parallelization_level: 0.8,
            },
            OptimizationStrategy::Balanced => OptimizationParameters {
                cache_priority: 0.6,
                memory_priority: 0.4,
                cpu_priority: 0.6,
                parallelization_level: 0.5,
            },
            OptimizationStrategy::Memory => OptimizationParameters {
                cache_priority: 0.2,
                memory_priority: 0.8,
                cpu_priority: 0.3,
                parallelization_level: 0.2,
            },
            OptimizationStrategy::Aggressive => OptimizationParameters {
                cache_priority: 0.95,
                memory_priority: 0.05,
                cpu_priority: 0.9,
                parallelization_level: 0.9,
            },
            OptimizationStrategy::Conservative => OptimizationParameters {
                cache_priority: 0.3,
                memory_priority: 0.7,
                cpu_priority: 0.4,
                parallelization_level: 0.1,
            },
            OptimizationStrategy::Custom(params) => OptimizationParameters {
                cache_priority: 0.5,
                memory_priority: 0.5,
                cpu_priority: 0.5,
                parallelization_level: 0.5,
            },
        }
    }
}

/// Optimization parameters for performance tuning
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationParameters {
    /// Priority for caching (0.0 to 1.0)
    pub cache_priority: f64,
    /// Priority for memory efficiency (0.0 to 1.0)
    pub memory_priority: f64,
    /// Priority for CPU utilization (0.0 to 1.0)
    pub cpu_priority: f64,
    /// Level of parallelization (0.0 to 1.0)
    pub parallelization_level: f64,
}
