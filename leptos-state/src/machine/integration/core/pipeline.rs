//! Integration pipeline for complex workflows

use crate::machine::integration::events::IntegrationEvent;
use crate::StateResult;

/// Integration pipeline for complex workflows
pub struct IntegrationPipeline<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + std::hash::Hash + Eq + 'static> {
    /// Pipeline name
    name: String,
    /// Pipeline steps
    steps: Vec<Box<dyn IntegrationStep<C, E> + Send + Sync>>,
    /// Pipeline configuration
    config: PipelineConfig,
    /// Execution statistics
    stats: PipelineStats,
}

impl<C: Send + Sync + Clone + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::hash::Hash + Eq + std::fmt::Debug + 'static>
    IntegrationPipeline<C, E>
{
    /// Create a new integration pipeline
    pub fn new(name: String, config: PipelineConfig) -> Self {
        Self {
            name,
            steps: Vec::new(),
            config,
            stats: PipelineStats::new(),
        }
    }

    /// Add a step to the pipeline
    pub fn add_step(&mut self, step: Box<dyn IntegrationStep<C, E> + Send + Sync>) {
        self.steps.push(step);
    }

    /// Execute the pipeline with an event
    pub async fn execute(&mut self, event: IntegrationEvent) -> Result<IntegrationEvent, String> {
        let start_time = std::time::Instant::now();

        let mut current_event = event;
        let mut step_results = Vec::new();

        for (index, step) in self.steps.iter_mut().enumerate() {
            let step_start = std::time::Instant::now();

            match step.execute(&current_event).await {
                Ok(result_event) => {
                    current_event = result_event;
                    let step_duration = step_start.elapsed();
                    step_results.push(StepResult::Success {
                        step_index: index,
                        duration: step_duration,
                    });
                }
                Err(error) => {
                    let step_duration = step_start.elapsed();
                    step_results.push(StepResult::Failure {
                        step_index: index,
                        error: error.clone(),
                        duration: step_duration,
                    });

                    if self.config.fail_fast {
                        return Err(format!("Pipeline step {} failed: {}", index, error));
                    }
                }
            }
        }

        let total_duration = start_time.elapsed();
        self.stats.record_execution(total_duration, step_results);

        Ok(current_event)
    }

    /// Get pipeline name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get number of steps
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Get pipeline configuration
    pub fn config(&self) -> &PipelineConfig {
        &self.config
    }

    /// Get execution statistics
    pub fn stats(&self) -> &PipelineStats {
        &self.stats
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = PipelineStats::new();
    }

    /// Check if pipeline is valid
    pub fn validate(&self) -> Result<(), String> {
        if self.steps.is_empty() {
            return Err("Pipeline must have at least one step".to_string());
        }

        for (index, step) in self.steps.iter().enumerate() {
            if let Err(error) = step.validate() {
                return Err(format!("Step {} validation failed: {}", index, error));
            }
        }

        Ok(())
    }
}

impl<C: Send + Sync + Clone + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::hash::Hash + Eq + std::fmt::Debug + 'static>
    std::fmt::Debug for IntegrationPipeline<C, E>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IntegrationPipeline")
            .field("name", &self.name)
            .field("steps", &self.steps.len())
            .field("config", &self.config)
            .finish()
    }
}

/// Integration step trait
#[async_trait::async_trait]
pub trait IntegrationStep<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + std::hash::Hash + Eq + 'static>: Send + Sync {
    /// Execute the step
    async fn execute(&mut self, event: &IntegrationEvent) -> Result<IntegrationEvent, String>;

    /// Validate the step configuration
    fn validate(&self) -> Result<(), String>;

    /// Get step name
    fn name(&self) -> &str;

    /// Get step description
    fn description(&self) -> &str {
        self.name()
    }
}

/// Pipeline configuration
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineConfig {
    /// Whether to fail fast on first error
    pub fail_fast: bool,
    /// Maximum execution time
    pub max_execution_time: std::time::Duration,
    /// Retry configuration
    pub retry_config: Option<RetryConfig>,
    /// Whether to enable metrics collection
    pub enable_metrics: bool,
    /// Custom configuration
    pub custom_config: std::collections::HashMap<String, serde_json::Value>,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            fail_fast: true,
            max_execution_time: std::time::Duration::from_secs(30),
            retry_config: None,
            enable_metrics: true,
            custom_config: std::collections::HashMap::new(),
        }
    }
}

impl PipelineConfig {
    /// Create a new pipeline configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set fail fast behavior
    pub fn fail_fast(mut self, fail_fast: bool) -> Self {
        self.fail_fast = fail_fast;
        self
    }

    /// Set maximum execution time
    pub fn max_execution_time(mut self, duration: std::time::Duration) -> Self {
        self.max_execution_time = duration;
        self
    }

    /// Set retry configuration
    pub fn retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = Some(config);
        self
    }

    /// Enable or disable metrics
    pub fn enable_metrics(mut self, enable: bool) -> Self {
        self.enable_metrics = enable;
        self
    }

    /// Add custom configuration
    pub fn custom_config<K: Into<String>, V: Into<serde_json::Value>>(mut self, key: K, value: V) -> Self {
        self.custom_config.insert(key.into(), value.into());
        self
    }
}

/// Retry configuration for pipeline steps
#[derive(Debug, Clone, PartialEq)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Delay between retries
    pub delay: std::time::Duration,
    /// Backoff multiplier
    pub backoff_multiplier: f64,
    /// Maximum delay
    pub max_delay: std::time::Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            delay: std::time::Duration::from_millis(100),
            backoff_multiplier: 2.0,
            max_delay: std::time::Duration::from_secs(10),
        }
    }
}

impl RetryConfig {
    /// Create a new retry configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set maximum attempts
    pub fn max_attempts(mut self, attempts: u32) -> Self {
        self.max_attempts = attempts;
        self
    }

    /// Set initial delay
    pub fn delay(mut self, delay: std::time::Duration) -> Self {
        self.delay = delay;
        self
    }

    /// Set backoff multiplier
    pub fn backoff_multiplier(mut self, multiplier: f64) -> Self {
        self.backoff_multiplier = multiplier;
        self
    }

    /// Set maximum delay
    pub fn max_delay(mut self, delay: std::time::Duration) -> Self {
        self.max_delay = delay;
        self
    }

    /// Calculate delay for a specific attempt
    pub fn delay_for_attempt(&self, attempt: u32) -> std::time::Duration {
        let base_delay_ms = self.delay.as_millis() as f64;
        let multiplier = self.backoff_multiplier.powi(attempt.saturating_sub(1) as i32);
        let calculated_ms = base_delay_ms * multiplier;
        let max_ms = self.max_delay.as_millis() as f64;
        let final_ms = calculated_ms.min(max_ms) as u64;

        std::time::Duration::from_millis(final_ms)
    }
}

/// Step execution result
#[derive(Debug, Clone)]
pub enum StepResult {
    /// Step executed successfully
    Success {
        /// Step index
        step_index: usize,
        /// Execution duration
        duration: std::time::Duration,
    },
    /// Step failed
    Failure {
        /// Step index
        step_index: usize,
        /// Error message
        error: String,
        /// Execution duration
        duration: std::time::Duration,
    },
}

impl StepResult {
    /// Check if the result is successful
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success { .. })
    }

    /// Check if the result is a failure
    pub fn is_failure(&self) -> bool {
        matches!(self, Self::Failure { .. })
    }

    /// Get execution duration
    pub fn duration(&self) -> std::time::Duration {
        match self {
            Self::Success { duration, .. } | Self::Failure { duration, .. } => *duration,
        }
    }

    /// Get step index
    pub fn step_index(&self) -> usize {
        match self {
            Self::Success { step_index, .. } | Self::Failure { step_index, .. } => *step_index,
        }
    }
}

/// Pipeline execution statistics
#[derive(Debug, Clone)]
pub struct PipelineStats {
    /// Total executions
    pub total_executions: u64,
    /// Successful executions
    pub successful_executions: u64,
    /// Failed executions
    pub failed_executions: u64,
    /// Total execution time
    pub total_execution_time: std::time::Duration,
    /// Average execution time
    pub avg_execution_time: std::time::Duration,
    /// Step results from last execution
    pub last_step_results: Vec<StepResult>,
}

impl PipelineStats {
    /// Create new statistics
    pub fn new() -> Self {
        Self {
            total_executions: 0,
            successful_executions: 0,
            failed_executions: 0,
            total_execution_time: std::time::Duration::from_secs(0),
            avg_execution_time: std::time::Duration::from_secs(0),
            last_step_results: Vec::new(),
        }
    }

    /// Record an execution
    pub fn record_execution(&mut self, duration: std::time::Duration, step_results: Vec<StepResult>) {
        self.total_executions += 1;
        self.total_execution_time += duration;
        self.last_step_results = step_results;

        let has_failures = self.last_step_results.iter().any(|r| r.is_failure());
        if has_failures {
            self.failed_executions += 1;
        } else {
            self.successful_executions += 1;
        }

        self.avg_execution_time = self.total_execution_time / self.total_executions as u32;
    }

    /// Get success rate as percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_executions == 0 {
            0.0
        } else {
            (self.successful_executions as f64 / self.total_executions as f64) * 100.0
        }
    }

    /// Get failure rate as percentage
    pub fn failure_rate(&self) -> f64 {
        100.0 - self.success_rate()
    }

    /// Reset statistics
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

impl Default for PipelineStats {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for PipelineStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PipelineStats {{ executions: {}, success: {:.1}%, avg_time: {:.2}ms }}",
            self.total_executions,
            self.success_rate(),
            self.avg_execution_time.as_millis()
        )
    }
}

/// Pipeline builder for fluent construction
pub struct PipelineBuilder<C: Send + Sync + Clone + 'static, E: Clone + Send + Sync + std::hash::Hash + Eq + 'static> {
    name: String,
    config: PipelineConfig,
    steps: Vec<Box<dyn IntegrationStep<C, E> + Send + Sync>>,
}

impl<C: Send + Sync + Clone + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::hash::Hash + Eq + std::fmt::Debug + 'static>
    PipelineBuilder<C, E>
{
    /// Create a new pipeline builder
    pub fn new(name: String) -> Self {
        Self {
            name,
            config: PipelineConfig::default(),
            steps: Vec::new(),
        }
    }

    /// Set pipeline configuration
    pub fn config(mut self, config: PipelineConfig) -> Self {
        self.config = config;
        self
    }

    /// Add a step to the pipeline
    pub fn step(mut self, step: Box<dyn IntegrationStep<C, E> + Send + Sync>) -> Self {
        self.steps.push(step);
        self
    }

    /// Build the pipeline
    pub fn build(self) -> Result<IntegrationPipeline<C, E>, String> {
        let mut pipeline = IntegrationPipeline::new(self.name, self.config);

        for step in self.steps {
            pipeline.add_step(step);
        }

        pipeline.validate()?;
        Ok(pipeline)
    }
}
