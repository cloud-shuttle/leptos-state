//! Performance builder for fluent configuration

use super::*;
use std::hash::Hash;

/// Performance builder for fluent configuration
pub struct PerformanceBuilder<
    C: Send + Sync + Clone + 'static,
    E: Clone + Send + Sync + Hash + Eq + 'static,
> {
    /// Machine to optimize
    machine: Machine<C, E, C>,
    /// Performance configuration
    config: PerformanceConfig,
}

impl<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>
    PerformanceBuilder<C, E>
{
    /// Create a new performance builder
    pub fn new(machine: Machine<C, E, C>) -> Self {
        Self {
            machine,
            config: PerformanceConfig::default(),
        }
    }

    /// Set cache configuration
    pub fn with_cache(mut self, enable: bool, max_size: usize, ttl: std::time::Duration) -> Self {
        self.config.enable_caching = enable;
        self.config.max_cache_size = max_size;
        self.config.cache_ttl = ttl;
        self
    }

    /// Enable caching with default settings
    pub fn enable_caching(mut self) -> Self {
        self.config.enable_caching = true;
        self
    }

    /// Disable caching
    pub fn disable_caching(mut self) -> Self {
        self.config.enable_caching = false;
        self
    }

    /// Set lazy evaluation
    pub fn with_lazy_evaluation(mut self, enable: bool) -> Self {
        self.config.enable_lazy_evaluation = enable;
        self
    }

    /// Enable lazy evaluation
    pub fn enable_lazy_evaluation(mut self) -> Self {
        self.config.enable_lazy_evaluation = true;
        self
    }

    /// Disable lazy evaluation
    pub fn disable_lazy_evaluation(mut self) -> Self {
        self.config.enable_lazy_evaluation = false;
        self
    }

    /// Set profiling
    pub fn with_profiling(mut self, enable: bool, sample_rate: f64) -> Self {
        self.config.enable_profiling = enable;
        self.config.profile_sample_rate = sample_rate;
        self
    }

    /// Enable profiling
    pub fn enable_profiling(mut self) -> Self {
        self.config.enable_profiling = true;
        self
    }

    /// Disable profiling
    pub fn disable_profiling(mut self) -> Self {
        self.config.enable_profiling = false;
        self
    }

    /// Set memory tracking
    pub fn with_memory_tracking(mut self, enable: bool, max_usage: usize) -> Self {
        self.config.enable_memory_tracking = enable;
        self.config.max_memory_usage = max_usage;
        self
    }

    /// Enable memory tracking
    pub fn enable_memory_tracking(mut self) -> Self {
        self.config.enable_memory_tracking = true;
        self
    }

    /// Disable memory tracking
    pub fn disable_memory_tracking(mut self) -> Self {
        self.config.enable_memory_tracking = false;
        self
    }

    /// Set optimization strategy
    pub fn with_strategy(mut self, strategy: OptimizationStrategy) -> Self {
        self.config.optimization_strategy = strategy;
        self
    }

    /// Use speed optimization strategy
    pub fn optimize_for_speed(mut self) -> Self {
        self.config.optimization_strategy = OptimizationStrategy::Speed;
        self
    }

    /// Use balanced optimization strategy
    pub fn optimize_for_balance(mut self) -> Self {
        self.config.optimization_strategy = OptimizationStrategy::Balanced;
        self
    }

    /// Use memory optimization strategy
    pub fn optimize_for_memory(mut self) -> Self {
        self.config.optimization_strategy = OptimizationStrategy::Memory;
        self
    }

    /// Use aggressive optimization strategy
    pub fn optimize_aggressively(mut self) -> Self {
        self.config.optimization_strategy = OptimizationStrategy::Aggressive;
        self
    }

    /// Set parallel processing
    pub fn with_parallel_processing(mut self, enable: bool, threads: usize) -> Self {
        self.config.enable_parallel_processing = enable;
        self.config.worker_threads = threads;
        self
    }

    /// Enable parallel processing
    pub fn enable_parallel_processing(mut self) -> Self {
        self.config.enable_parallel_processing = true;
        self
    }

    /// Disable parallel processing
    pub fn disable_parallel_processing(mut self) -> Self {
        self.config.enable_parallel_processing = false;
        self
    }

    /// Set coverage threshold
    pub fn with_coverage_threshold(mut self, threshold: f64) -> Self {
        self.config.coverage_threshold = threshold;
        self
    }

    /// Set all configuration at once
    pub fn with_config(mut self, config: PerformanceConfig) -> Self {
        self.config = config;
        self
    }

    /// Build the optimized machine
    pub fn build(self) -> OptimizedMachine<C, E> {
        OptimizedMachine::new(self.machine, self.config)
    }

    /// Build and immediately start profiling
    pub fn build_and_profile(self) -> OptimizedMachine<C, E> {
        let mut machine = OptimizedMachine::new(self.machine, self.config);
        machine.enable_profiling();
        machine
    }

    /// Get the current configuration
    pub fn config(&self) -> &PerformanceConfig {
        &self.config
    }

    /// Get a mutable reference to the configuration for advanced customization
    pub fn config_mut(&mut self) -> &mut PerformanceConfig {
        &mut self.config
    }
}

/// Extension trait for fluent performance optimization
pub trait PerformanceOptimizationExt<
    C: Send + Sync + Clone + 'static,
    E: Clone + Send + Sync + Hash + Eq + 'static,
>
{
    /// Start building performance optimizations
    fn optimize_performance(self) -> PerformanceBuilder<C, E>;
}

impl<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static>
    PerformanceOptimizationExt<C, E> for Machine<C, E, C>
{
    fn optimize_performance(self) -> PerformanceBuilder<C, E> {
        PerformanceBuilder::new(self)
    }
}

/// Performance configuration presets
pub struct PerformancePresets;

impl PerformancePresets {
    /// Configuration for high-performance scenarios
    pub fn high_performance() -> PerformanceConfig {
        PerformanceConfig {
            enable_caching: true,
            max_cache_size: 100 * 1024 * 1024,              // 100MB
            cache_ttl: std::time::Duration::from_secs(600), // 10 minutes
            enable_lazy_evaluation: false,                  // Disable for speed
            enable_profiling: false,
            profile_sample_rate: 0.01, // Minimal profiling
            enable_memory_tracking: true,
            max_memory_usage: 500 * 1024 * 1024, // 500MB
            optimization_strategy: OptimizationStrategy::Speed,
            enable_parallel_processing: true,
            worker_threads: num_cpus::get() * 2,
        }
    }

    /// Configuration for memory-constrained environments
    pub fn memory_efficient() -> PerformanceConfig {
        PerformanceConfig {
            enable_caching: true,
            max_cache_size: 10 * 1024 * 1024,              // 10MB
            cache_ttl: std::time::Duration::from_secs(60), // 1 minute
            enable_lazy_evaluation: true,
            enable_profiling: false,
            profile_sample_rate: 0.0,
            enable_memory_tracking: true,
            max_memory_usage: 50 * 1024 * 1024, // 50MB
            optimization_strategy: OptimizationStrategy::Memory,
            enable_parallel_processing: false,
            worker_threads: 1,
        }
    }

    /// Configuration for development and debugging
    pub fn development() -> PerformanceConfig {
        PerformanceConfig {
            enable_caching: true,
            max_cache_size: 50 * 1024 * 1024,               // 50MB
            cache_ttl: std::time::Duration::from_secs(300), // 5 minutes
            enable_lazy_evaluation: true,
            enable_profiling: true,
            profile_sample_rate: 1.0, // Profile everything
            enable_memory_tracking: true,
            max_memory_usage: 200 * 1024 * 1024, // 200MB
            optimization_strategy: OptimizationStrategy::Balanced,
            enable_parallel_processing: false,
            worker_threads: 2,
        }
    }

    /// Configuration for production environments
    pub fn production() -> PerformanceConfig {
        PerformanceConfig {
            enable_caching: true,
            max_cache_size: 200 * 1024 * 1024, // 200MB
            cache_ttl: std::time::Duration::from_secs(1800), // 30 minutes
            enable_lazy_evaluation: true,
            enable_profiling: false,
            profile_sample_rate: 0.001, // Minimal profiling
            enable_memory_tracking: true,
            max_memory_usage: 1 * 1024 * 1024 * 1024, // 1GB
            optimization_strategy: OptimizationStrategy::Balanced,
            enable_parallel_processing: true,
            worker_threads: num_cpus::get(),
        }
    }
}
