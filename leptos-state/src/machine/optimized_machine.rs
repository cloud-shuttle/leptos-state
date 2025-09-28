//! Performance-optimized state machine

use super::*;
use std::collections::HashMap;
use std::hash::Hash;

/// Performance-optimized state machine
pub struct OptimizedMachine<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> {
    /// Base machine
    base_machine: Machine<C, E, C>,
    /// Transition cache
    cache: TransitionCache<C, E>,
    /// Performance profiler
    profiler: PerformanceProfiler,
    /// Lazy evaluators for expensive operations
    lazy_evaluators: HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
    /// Optimization level
    optimization_level: OptimizationLevel,
}

impl<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> OptimizedMachine<C, E> {
    /// Create a new optimized machine
    pub fn new(base_machine: Machine<C, E, C>, config: PerformanceConfig) -> Self {
        Self {
            cache: TransitionCache::new(config.max_cache_size, config.cache_ttl),
            profiler: PerformanceProfiler::new(),
            base_machine,
            lazy_evaluators: HashMap::new(),
            optimization_level: OptimizationLevel::from_strategy(&config.optimization_strategy),
        }
    }

    /// Perform an optimized transition
    pub fn transition(&mut self, current: &MachineStateImpl<C>, event: E) -> MachineStateImpl<C> {
        let start_time = std::time::Instant::now();

        // Create cache key
        let cache_key = CacheKey::new(
            current.value.to_string(),
            event.clone(),
            &current.context,
        );

        // Try cache first
        if let Some(cached) = self.cache.get(&cache_key) {
            let result = cached.result_state.clone();
            let duration = start_time.elapsed();

            // Record performance
            self.profiler.record_transition(
                &current.value.to_string(),
                &result.value.to_string(),
                duration,
            );

            return result;
        }

        // Perform actual transition
        let transition_start = std::time::Instant::now();
        // This would need to be implemented properly
        // For now, return a placeholder
        let result = MachineStateImpl {
            value: current.value.clone(),
            context: current.context.clone(),
        };
        let transition_duration = transition_start.elapsed();

        // Cache the result
        let cached_transition = CachedTransition::new(result.clone());
        self.cache.insert(cache_key, cached_transition);

        let total_duration = start_time.elapsed();

        // Record performance
        self.profiler.record_transition(
            &current.value.to_string(),
            &result.value.to_string(),
            total_duration,
        );

        result
    }

    /// Get initial state
    pub fn initial_state(&self) -> MachineStateImpl<C>
    where
        C: Default,
    {
        self.base_machine.initial_state()
    }

    /// Get all states
    pub fn get_states(&self) -> Vec<String> {
        self.base_machine.get_states()
    }

    /// Get performance metrics
    pub fn get_metrics(&self) -> PerformanceMetrics {
        let mut metrics = self.profiler.get_metrics();
        metrics.cache_hit_rate = self.cache.stats().hit_rate;
        metrics.memory_usage = self.cache.memory_usage();
        metrics
    }

    /// Analyze performance
    pub fn analyze_performance(&mut self) -> PerformanceAnalysis {
        self.profiler.analyze()
    }

    /// Optimize the machine based on performance analysis
    pub fn optimize(&mut self) {
        let analysis = self.profiler.analyze();

        // Apply optimizations based on bottlenecks
        for bottleneck in &analysis.bottlenecks {
            match bottleneck.bottleneck_type {
                BottleneckType::SlowTransition => {
                    // Increase cache size or enable lazy evaluation
                    // This would be implemented based on specific optimization strategies
                }
                BottleneckType::MemoryUsage => {
                    // Reduce cache size or implement memory limits
                }
                BottleneckType::CacheInefficiency => {
                    // Improve cache strategy
                }
                _ => {}
            }
        }
    }

    /// Enable profiling
    pub fn enable_profiling(&mut self) {
        self.profiler.start();
    }

    /// Disable profiling
    pub fn disable_profiling(&mut self) {
        self.profiler.stop();
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> &CacheStats {
        self.cache.stats()
    }

    /// Clear cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Set optimization level
    pub fn set_optimization_level(&mut self, level: OptimizationLevel) {
        self.optimization_level = level;
    }

    /// Get optimization level
    pub fn optimization_level(&self) -> &OptimizationLevel {
        &self.optimization_level
    }

    /// Add a lazy evaluator
    pub fn add_lazy_evaluator<T, F>(&mut self, name: String, evaluator: LazyEvaluator<T, F>)
    where
        T: 'static,
        F: 'static,
    {
        self.lazy_evaluators.insert(name, Box::new(evaluator));
    }

    /// Get a lazy evaluator
    pub fn get_lazy_evaluator<T>(&self, name: &str) -> Option<&LazyEvaluator<T, Box<dyn Fn() -> T>>>
    where
        T: 'static,
    {
        self.lazy_evaluators.get(name)?
            .downcast_ref::<LazyEvaluator<T, Box<dyn Fn() -> T>>>()
    }
}

/// Optimization levels
#[derive(Debug, Clone, PartialEq)]
pub enum OptimizationLevel {
    /// No optimization
    None,
    /// Basic optimizations
    Basic,
    /// Advanced optimizations
    Advanced,
    /// Aggressive optimizations
    Aggressive,
}

impl OptimizationLevel {
    /// Create optimization level from strategy
    pub fn from_strategy(strategy: &OptimizationStrategy) -> Self {
        match strategy {
            OptimizationStrategy::Speed => OptimizationLevel::Advanced,
            OptimizationStrategy::Balanced => OptimizationLevel::Basic,
            OptimizationStrategy::Memory => OptimizationLevel::Basic,
            OptimizationStrategy::Aggressive => OptimizationLevel::Aggressive,
            OptimizationStrategy::Conservative => OptimizationLevel::None,
            OptimizationStrategy::Custom(_) => OptimizationLevel::Basic,
        }
    }
}

/// Extension trait for adding performance optimization to machines
pub trait MachinePerformanceExt<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> {
    /// Create an optimized version of this machine
    fn optimize(self, config: PerformanceConfig) -> OptimizedMachine<C, E>;
}

impl<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> MachinePerformanceExt<C, E> for Machine<C, E, C> {
    fn optimize(self, config: PerformanceConfig) -> OptimizedMachine<C, E> {
        OptimizedMachine::new(self, config)
    }
}
