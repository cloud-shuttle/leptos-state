/// Machine configuration options
#[derive(Clone, Debug)]
pub struct MachineConfig {
    pub strict_mode: bool,
    pub auto_cleanup: bool,
    pub max_history_size: usize,
    pub enable_guards: bool,
    pub enable_actions: bool,
}

impl Default for MachineConfig {
    fn default() -> Self {
        Self {
            strict_mode: false,
            auto_cleanup: true,
            max_history_size: 100,
            enable_guards: true,
            enable_actions: true,
        }
    }
}

/// Event routing configuration
#[derive(Clone, Debug)]
pub struct EventRoutingConfig {
    pub priority: i32,
    pub async_processing: bool,
    pub retry_count: u32,
    pub timeout_ms: u64,
}

impl Default for EventRoutingConfig {
    fn default() -> Self {
        Self {
            priority: 0,
            async_processing: false,
            retry_count: 3,
            timeout_ms: 5000,
        }
    }
}

/// State validation configuration
#[derive(Clone, Debug)]
pub struct StateValidationConfig {
    pub validate_on_entry: bool,
    pub validate_on_exit: bool,
    pub validate_transitions: bool,
    pub strict_validation: bool,
}

impl Default for StateValidationConfig {
    fn default() -> Self {
        Self {
            validate_on_entry: true,
            validate_on_exit: true,
            validate_transitions: true,
            strict_validation: false,
        }
    }
}

/// Performance monitoring configuration
#[derive(Clone, Debug)]
pub struct PerformanceConfig {
    pub enable_metrics: bool,
    pub enable_profiling: bool,
    pub metrics_interval_ms: u64,
    pub max_samples: usize,
    pub optimization_strategy: crate::machine::performance_config::OptimizationStrategy,
    pub worker_threads: usize,
    pub profile_sample_rate: f64,
    pub max_memory_usage: u64,
    pub max_cache_size: usize,
    pub enable_parallel_processing: bool,
    pub enable_memory_tracking: bool,
    pub enable_lazy_evaluation: bool,
    pub enable_caching: bool,
    pub cache_ttl: std::time::Duration,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_metrics: false,
            enable_profiling: false,
            metrics_interval_ms: 1000,
            max_samples: 1000,
            optimization_strategy: crate::machine::performance_config::OptimizationStrategy::Balanced,
            worker_threads: num_cpus::get(),
            profile_sample_rate: 0.1,
            max_memory_usage: 500 * 1024 * 1024, // 500MB
            max_cache_size: 100 * 1024 * 1024, // 100MB
            enable_parallel_processing: false,
            enable_memory_tracking: true,
            enable_lazy_evaluation: false,
            enable_caching: true,
            cache_ttl: std::time::Duration::from_secs(3600), // 1 hour
        }
    }
}

/// Integration configuration for external systems
#[derive(Clone, Debug)]
pub struct IntegrationConfig {
    pub enable_external_events: bool,
    pub enable_state_sync: bool,
    pub sync_interval_ms: u64,
    pub event_routing: EventRoutingConfig,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            enable_external_events: false,
            enable_state_sync: false,
            sync_interval_ms: 1000,
            event_routing: EventRoutingConfig::default(),
        }
    }
}

/// Machine configuration combining all settings
#[derive(Clone, Debug)]
pub struct CompleteMachineConfig {
    pub machine: MachineConfig,
    pub validation: StateValidationConfig,
    pub performance: PerformanceConfig,
    pub integration: IntegrationConfig,
}

impl Default for CompleteMachineConfig {
    fn default() -> Self {
        Self {
            machine: MachineConfig::default(),
            validation: StateValidationConfig::default(),
            performance: PerformanceConfig::default(),
            integration: IntegrationConfig::default(),
        }
    }
}
