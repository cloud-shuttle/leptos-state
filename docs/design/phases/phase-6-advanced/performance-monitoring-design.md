# Performance Monitoring Design

## Overview
Implement comprehensive performance monitoring for state management operations, including metrics collection, alerting, profiling, and optimization recommendations.

## Current State
```rust
// No performance monitoring
impl<S: State> Store<S> {
    pub fn update<F>(&self, updater: F) -> Result<(), StoreError>
    where
        F: FnOnce(&mut S) + Send + 'static,
    {
        self.signal.update(updater);
        Ok(())
    }
}
```

## Proposed Enhancement
```rust
#[cfg(feature = "performance")]
impl<S: State> Store<S> {
    pub fn update_with_monitoring<F>(&self, updater: F) -> Result<(), StoreError>
    where
        F: FnOnce(&mut S) + Send + 'static,
    {
        let _monitor = self.performance_monitor.record_operation("update", || {
            self.signal.update(updater)
        });
        Ok(())
    }
}

// Global performance dashboard
console.performance.state.getReport();
console.performance.state.getBottlenecks();
```

## Motivation

### Performance Optimization
- **Bottleneck Identification**: Find slow state operations
- **Memory Leak Detection**: Monitor state size growth
- **Optimization Recommendations**: Automated performance suggestions
- **Regression Detection**: Alert on performance degradation

### Production Monitoring
- **Health Metrics**: Monitor state management system health
- **Usage Analytics**: Track operation patterns and frequencies
- **Capacity Planning**: Understand resource usage trends
- **SLA Compliance**: Ensure performance meets requirements

### Use Cases
- Identifying slow state updates in production
- Memory usage monitoring and leak detection
- Performance regression testing
- Real-time performance dashboards
- Automated performance optimization

## Implementation Details

### Performance Metrics Collector
```rust
#[cfg(feature = "performance")]
pub struct PerformanceMonitor {
    metrics: Arc<RwLock<PerformanceMetrics>>,
    alerts: Vec<Box<dyn PerformanceAlert>>,
    reporters: Vec<Box<dyn MetricsReporter>>,
    config: PerformanceConfig,
}

#[derive(Clone, Debug)]
pub struct PerformanceMetrics {
    pub operation_counts: HashMap<String, u64>,
    pub operation_durations: HashMap<String, DurationStats>,
    pub memory_usage: MemoryStats,
    pub error_counts: HashMap<String, u64>,
    pub last_updated: DateTime<Utc>,
    pub uptime: Duration,
}

#[derive(Clone, Debug)]
pub struct DurationStats {
    pub count: u64,
    pub total_duration: Duration,
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub avg_duration: Duration,
    pub p95_duration: Duration,
    pub p99_duration: Duration,
}

#[derive(Clone, Debug)]
pub struct MemoryStats {
    pub current_usage: usize,
    pub peak_usage: usize,
    pub allocation_count: u64,
    pub deallocation_count: u64,
}

#[derive(Clone, Debug)]
pub struct PerformanceConfig {
    pub enable_operation_timing: bool,
    pub enable_memory_tracking: bool,
    pub enable_error_tracking: bool,
    pub slow_operation_threshold: Duration,
    pub memory_alert_threshold: usize,
    pub metrics_retention_period: Duration,
}

impl PerformanceMonitor {
    pub fn new(config: PerformanceConfig) -> Self {
        Self {
            metrics: Arc::new(RwLock::new(PerformanceMetrics {
                operation_counts: HashMap::new(),
                operation_durations: HashMap::new(),
                memory_usage: MemoryStats {
                    current_usage: 0,
                    peak_usage: 0,
                    allocation_count: 0,
                    deallocation_count: 0,
                },
                error_counts: HashMap::new(),
                last_updated: Utc::now(),
                uptime: Duration::from_secs(0),
            })),
            alerts: Vec::new(),
            reporters: Vec::new(),
            config,
        }
    }

    pub fn record_operation<F, R>(&self, operation_name: &str, operation: F) -> Result<R, PerformanceError>
    where
        F: FnOnce() -> R,
    {
        let start_time = Instant::now();
        let start_memory = if self.config.enable_memory_tracking {
            self.get_current_memory_usage()
        } else {
            0
        };

        let result = operation();

        let duration = start_time.elapsed();
        let end_memory = if self.config.enable_memory_tracking {
            self.get_current_memory_usage()
        } else {
            0
        };

        // Update metrics
        {
            let mut metrics = self.metrics.write().unwrap();

            // Operation counts
            *metrics.operation_counts.entry(operation_name.to_string()).or_insert(0) += 1;

            // Duration stats
            if self.config.enable_operation_timing {
                let stats = metrics.operation_durations
                    .entry(operation_name.to_string())
                    .or_insert_with(|| DurationStats {
                        count: 0,
                        total_duration: Duration::from_secs(0),
                        min_duration: Duration::from_secs(u64::MAX),
                        max_duration: Duration::from_secs(0),
                        avg_duration: Duration::from_secs(0),
                        p95_duration: Duration::from_secs(0),
                        p99_duration: Duration::from_secs(0),
                    });

                stats.count += 1;
                stats.total_duration += duration;
                stats.min_duration = stats.min_duration.min(duration);
                stats.max_duration = stats.max_duration.max(duration);
                stats.avg_duration = stats.total_duration / stats.count as u32;

                // Update percentiles (simplified)
                self.update_percentiles(stats);
            }

            // Memory stats
            if self.config.enable_memory_tracking {
                let memory_delta = end_memory as i64 - start_memory as i64;
                if memory_delta > 0 {
                    metrics.memory_usage.allocation_count += 1;
                } else if memory_delta < 0 {
                    metrics.memory_usage.deallocation_count += 1;
                }

                metrics.memory_usage.current_usage = end_memory;
                metrics.memory_usage.peak_usage = metrics.memory_usage.peak_usage.max(end_memory);
            }

            metrics.last_updated = Utc::now();
        }

        // Check for alerts
        self.check_alerts(operation_name, duration, end_memory)?;

        // Report metrics
        self.report_metrics()?;

        Ok(result)
    }

    pub fn record_error(&self, operation_name: &str, error: &str) {
        if self.config.enable_error_tracking {
            let mut metrics = self.metrics.write().unwrap();
            *metrics.error_counts.entry(format!("{}:{}", operation_name, error)).or_insert(0) += 1;
        }
    }

    fn get_current_memory_usage(&self) -> usize {
        // Platform-specific memory measurement
        // This would use platform-specific APIs
        0 // Placeholder
    }

    fn update_percentiles(&self, stats: &mut DurationStats) {
        // Simplified percentile calculation
        // In practice, you'd maintain a sorted list of durations
        stats.p95_duration = stats.avg_duration * 2; // Rough approximation
        stats.p99_duration = stats.avg_duration * 3; // Rough approximation
    }

    fn check_alerts(&self, operation_name: &str, duration: Duration, memory_usage: usize) -> Result<(), PerformanceError> {
        for alert in &self.alerts {
            if let Some(alert_msg) = alert.check(operation_name, duration, memory_usage) {
                // Trigger alert
                log::warn!("Performance alert: {}", alert_msg);
            }
        }
        Ok(())
    }

    fn report_metrics(&self) -> Result<(), PerformanceError> {
        for reporter in &self.reporters {
            let metrics = self.metrics.read().unwrap().clone();
            reporter.report(&metrics)?;
        }
        Ok(())
    }

    pub fn add_alert<A: PerformanceAlert + 'static>(mut self, alert: A) -> Self {
        self.alerts.push(Box::new(alert));
        self
    }

    pub fn add_reporter<R: MetricsReporter + 'static>(mut self, reporter: R) -> Self {
        self.reporters.push(Box::new(reporter));
        self
    }

    pub fn get_metrics(&self) -> PerformanceMetrics {
        self.metrics.read().unwrap().clone()
    }

    pub fn get_report(&self) -> PerformanceReport {
        let metrics = self.get_metrics();
        let bottlenecks = self.identify_bottlenecks(&metrics);
        let recommendations = self.generate_recommendations(&metrics, &bottlenecks);

        PerformanceReport {
            metrics,
            bottlenecks,
            recommendations,
            generated_at: Utc::now(),
        }
    }

    fn identify_bottlenecks(&self, metrics: &PerformanceMetrics) -> Vec<Bottleneck> {
        let mut bottlenecks = Vec::new();

        // Check for slow operations
        for (operation, stats) in &metrics.operation_durations {
            if stats.avg_duration > self.config.slow_operation_threshold {
                bottlenecks.push(Bottleneck {
                    category: BottleneckCategory::SlowOperation,
                    severity: if stats.p99_duration > self.config.slow_operation_threshold * 5 {
                        Severity::Critical
                    } else if stats.p95_duration > self.config.slow_operation_threshold * 2 {
                        Severity::High
                    } else {
                        Severity::Medium
                    },
                    description: format!("Operation '{}' is slow (avg: {:.2}ms)",
                                       operation, stats.avg_duration.as_millis()),
                    affected_operations: vec![operation.clone()],
                });
            }
        }

        // Check for memory issues
        if metrics.memory_usage.current_usage > self.config.memory_alert_threshold {
            bottlenecks.push(Bottleneck {
                category: BottleneckCategory::HighMemoryUsage,
                severity: Severity::High,
                description: format!("High memory usage: {} bytes", metrics.memory_usage.current_usage),
                affected_operations: vec!["all".to_string()],
            });
        }

        // Check for error rates
        let total_operations: u64 = metrics.operation_counts.values().sum();
        let total_errors: u64 = metrics.error_counts.values().sum();
        let error_rate = if total_operations > 0 {
            total_errors as f64 / total_operations as f64
        } else {
            0.0
        };

        if error_rate > 0.1 { // 10% error rate
            bottlenecks.push(Bottleneck {
                category: BottleneckCategory::HighErrorRate,
                severity: Severity::High,
                description: format!("High error rate: {:.1}%", error_rate * 100.0),
                affected_operations: metrics.error_counts.keys().cloned().collect(),
            });
        }

        bottlenecks
    }

    fn generate_recommendations(&self, metrics: &PerformanceMetrics, bottlenecks: &[Bottleneck]) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();

        for bottleneck in bottlenecks {
            match bottleneck.category {
                BottleneckCategory::SlowOperation => {
                    recommendations.push(Recommendation {
                        category: RecommendationCategory::Optimization,
                        priority: bottleneck.severity.clone().into(),
                        description: format!("Consider optimizing slow operations: {}",
                                           bottleneck.affected_operations.join(", ")),
                        implementation_effort: Effort::Medium,
                        expected_impact: Impact::High,
                    });
                }
                BottleneckCategory::HighMemoryUsage => {
                    recommendations.push(Recommendation {
                        category: RecommendationCategory::MemoryOptimization,
                        priority: Priority::High,
                        description: "Implement memory pooling or reduce state size".to_string(),
                        implementation_effort: Effort::High,
                        expected_impact: Impact::Medium,
                    });
                }
                BottleneckCategory::HighErrorRate => {
                    recommendations.push(Recommendation {
                        category: RecommendationCategory::ErrorHandling,
                        priority: Priority::Critical,
                        description: "Improve error handling and add retry logic".to_string(),
                        implementation_effort: Effort::Medium,
                        expected_impact: Impact::High,
                    });
                }
            }
        }

        recommendations
    }
}
```

### Alert System
```rust
pub trait PerformanceAlert: Send + Sync {
    fn check(&self, operation_name: &str, duration: Duration, memory_usage: usize) -> Option<String>;
    fn name(&self) -> &'static str;
}

pub struct SlowOperationAlert {
    pub threshold: Duration,
}

impl PerformanceAlert for SlowOperationAlert {
    fn check(&self, operation_name: &str, duration: Duration, _memory_usage: usize) -> Option<String> {
        if duration > self.threshold {
            Some(format!("Operation '{}' took {:.2}ms (threshold: {:.2}ms)",
                        operation_name, duration.as_millis(), self.threshold.as_millis()))
        } else {
            None
        }
    }

    fn name(&self) -> &'static str {
        "slow_operation"
    }
}

pub struct MemoryUsageAlert {
    pub threshold: usize,
}

impl PerformanceAlert for MemoryUsageAlert {
    fn check(&self, _operation_name: &str, _duration: Duration, memory_usage: usize) -> Option<String> {
        if memory_usage > self.threshold {
            Some(format!("Memory usage exceeded threshold: {} bytes (threshold: {} bytes)",
                        memory_usage, self.threshold))
        } else {
            None
        }
    }

    fn name(&self) -> &'static str {
        "memory_usage"
    }
}

pub struct ErrorRateAlert {
    pub threshold: f64, // Error rate as fraction
}

impl PerformanceAlert for ErrorRateAlert {
    fn check(&self, operation_name: &str, _duration: Duration, _memory_usage: usize) -> Option<String> {
        // This would need access to error counts
        // Implementation would track error rates per operation
        None // Placeholder
    }

    fn name(&self) -> &'static str {
        "error_rate"
    }
}
```

### Metrics Reporters
```rust
pub trait MetricsReporter: Send + Sync {
    fn report(&self, metrics: &PerformanceMetrics) -> Result<(), PerformanceError>;
    fn name(&self) -> &'static str;
}

pub struct ConsoleReporter;

impl MetricsReporter for ConsoleReporter {
    fn report(&self, metrics: &PerformanceMetrics) -> Result<(), PerformanceError> {
        log::info!("Performance Metrics:");
        log::info!("  Operations: {}", metrics.operation_counts.len());
        log::info!("  Total Operations: {}", metrics.operation_counts.values().sum::<u64>());
        log::info!("  Memory Usage: {} bytes", metrics.memory_usage.current_usage);
        log::info!("  Errors: {}", metrics.error_counts.values().sum::<u64>());
        Ok(())
    }

    fn name(&self) -> &'static str {
        "console"
    }
}

pub struct JsonFileReporter {
    file_path: PathBuf,
}

impl JsonFileReporter {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }
}

impl MetricsReporter for JsonFileReporter {
    fn report(&self, metrics: &PerformanceMetrics) -> Result<(), PerformanceError> {
        let json = serde_json::to_string_pretty(metrics)?;
        std::fs::write(&self.file_path, json)?;
        Ok(())
    }

    fn name(&self) -> &'static str {
        "json_file"
    }
}

#[cfg(feature = "web")]
pub struct WebReporter {
    endpoint: String,
}

#[cfg(feature = "web")]
impl MetricsReporter for WebReporter {
    fn report(&self, metrics: &PerformanceMetrics) -> Result<(), PerformanceError> {
        // Send metrics to web endpoint
        todo!()
    }

    fn name(&self) -> &'static str {
        "web"
    }
}
```

### Performance Report Generation
```rust
#[derive(Clone, Debug)]
pub struct PerformanceReport {
    pub metrics: PerformanceMetrics,
    pub bottlenecks: Vec<Bottleneck>,
    pub recommendations: Vec<Recommendation>,
    pub generated_at: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct Bottleneck {
    pub category: BottleneckCategory,
    pub severity: Severity,
    pub description: String,
    pub affected_operations: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum BottleneckCategory {
    SlowOperation,
    HighMemoryUsage,
    HighErrorRate,
    ResourceContention,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl From<Severity> for Priority {
    fn from(severity: Severity) -> Self {
        match severity {
            Severity::Low => Priority::Low,
            Severity::Medium => Priority::Medium,
            Severity::High => Priority::High,
            Severity::Critical => Priority::Critical,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Recommendation {
    pub category: RecommendationCategory,
    pub priority: Priority,
    pub description: String,
    pub implementation_effort: Effort,
    pub expected_impact: Impact,
}

#[derive(Clone, Debug)]
pub enum RecommendationCategory {
    Optimization,
    MemoryOptimization,
    ErrorHandling,
    ArchitectureImprovement,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Clone, Debug)]
pub enum Effort {
    Low,
    Medium,
    High,
}

#[derive(Clone, Debug)]
pub enum Impact {
    Low,
    Medium,
    High,
}

impl PerformanceReport {
    pub fn to_json(&self) -> Result<String, PerformanceError> {
        serde_json::to_string_pretty(self).map_err(PerformanceError::Serialization)
    }

    pub fn to_markdown(&self) -> String {
        let mut output = format!("# Performance Report\n\nGenerated: {}\n\n", self.generated_at);

        output.push_str("## Metrics Summary\n\n");
        output.push_str(&format!("- Total Operations: {}\n",
            self.metrics.operation_counts.values().sum::<u64>()));
        output.push_str(&format!("- Memory Usage: {} bytes\n", self.metrics.memory_usage.current_usage));
        output.push_str(&format!("- Total Errors: {}\n", self.metrics.error_counts.values().sum::<u64>()));

        output.push_str("\n## Bottlenecks\n\n");
        for bottleneck in &self.bottlenecks {
            output.push_str(&format!("### {} ({:?})\n{}\n\n",
                bottleneck.category, bottleneck.severity, bottleneck.description));
        }

        output.push_str("## Recommendations\n\n");
        for recommendation in &self.recommendations {
            output.push_str(&format!("### {} ({:?})\n", recommendation.category, recommendation.priority));
            output.push_str(&format!("**Effort:** {:?} | **Impact:** {:?}\n\n", 
                recommendation.implementation_effort, recommendation.expected_impact));
            output.push_str(&format!("{}\n\n", recommendation.description));
        }

        output
    }
}
```

## Error Handling

### Performance Errors
```rust
#[derive(Debug, Clone, thiserror::Error)]
pub enum PerformanceError {
    #[error("Metrics collection failed: {message}")]
    MetricsError { message: String },

    #[error("Alert check failed: {message}")]
    AlertError { message: String },

    #[error("Reporting failed: {message}")]
    ReportingError { message: String },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Platform-specific error: {message}")]
    PlatformError { message: String },
}
```

### Safe Monitoring
```rust
impl PerformanceMonitor {
    pub fn record_operation_safe<F, R>(
        &self,
        operation_name: &str,
        operation: F
    ) -> Result<R, PerformanceError>
    where
        F: FnOnce() -> R,
    {
        std::panic::catch_unwind(|| self.record_operation(operation_name, operation))
            .map_err(|_| PerformanceError::MetricsError {
                message: "Metrics recording panicked".to_string(),
            })?
    }
}
```

## Testing Strategy

### Unit Tests
```rust
#[cfg(feature = "performance")]
mod tests {
    use super::*;

    #[test]
    fn performance_monitor_records_operations() {
        let config = PerformanceConfig {
            enable_operation_timing: true,
            enable_memory_tracking: false,
            enable_error_tracking: true,
            slow_operation_threshold: Duration::from_millis(100),
            memory_alert_threshold: 1024 * 1024,
            metrics_retention_period: Duration::from_secs(3600),
        };

        let monitor = PerformanceMonitor::new(config);

        // Record some operations
        monitor.record_operation("test_op", || {
            std::thread::sleep(Duration::from_millis(10));
            42
        }).unwrap();

        let metrics = monitor.get_metrics();
        assert_eq!(*metrics.operation_counts.get("test_op").unwrap(), 1);

        let duration_stats = metrics.operation_durations.get("test_op").unwrap();
        assert!(duration_stats.total_duration >= Duration::from_millis(10));
    }

    #[test]
    fn bottleneck_detection_works() {
        let config = PerformanceConfig {
            enable_operation_timing: true,
            slow_operation_threshold: Duration::from_millis(50),
            ..Default::default()
        };

        let monitor = PerformanceMonitor::new(config);

        // Record slow operation
        monitor.record_operation("slow_op", || {
            std::thread::sleep(Duration::from_millis(100));
            ()
        }).unwrap();

        let report = monitor.get_report();
        assert!(!report.bottlenecks.is_empty());

        let bottleneck = &report.bottlenecks[0];
        assert_eq!(bottleneck.category, BottleneckCategory::SlowOperation);
    }

    #[test]
    fn alert_system_works() {
        let config = PerformanceConfig {
            enable_operation_timing: true,
            slow_operation_threshold: Duration::from_millis(1),
            ..Default::default()
        };

        let monitor = PerformanceMonitor::new(config)
            .add_alert(SlowOperationAlert {
                threshold: Duration::from_millis(50),
            });

        // This should trigger an alert (assuming the alert system logs warnings)
        monitor.record_operation("slow_op", || {
            std::thread::sleep(Duration::from_millis(100));
            ()
        }).unwrap();
    }
}
```

### Integration Tests
```rust
#[cfg(feature = "performance")]
#[test]
fn store_with_performance_monitoring() {
    let config = PerformanceConfig {
        enable_operation_timing: true,
        ..Default::default()
    };

    let monitor = PerformanceMonitor::new(config);
    let mut store = Store::new(TestState { count: 0 });

    // Perform operations with monitoring
    for i in 0..10 {
        monitor.record_operation("store_update", || {
            store.update(|s| s.count = i).unwrap();
        }).unwrap();
    }

    let metrics = monitor.get_metrics();
    assert_eq!(*metrics.operation_counts.get("store_update").unwrap(), 10);

    let report = monitor.get_report();
    assert!(report.recommendations.is_empty()); // Should be fast operations
}
```

## Performance Impact

### Monitoring Overhead
- **Minimal Baseline**: Basic operation counting
- **Timing Overhead**: High-precision timing impact
- **Memory Tracking**: Platform-specific memory APIs
- **Configurable**: Can disable expensive features

### Optimization Strategies
```rust
impl PerformanceMonitor {
    pub fn optimized_for_production(mut self) -> Self {
        // Disable expensive features for production
        self.config.enable_memory_tracking = false;
        self.config.enable_operation_timing = false;
        self
    }

    pub fn with_sampling(mut self, sample_rate: f64) -> Self {
        // Only monitor a percentage of operations
        self.sample_rate = sample_rate;
        self
    }

    pub fn with_async_reporting(mut self) -> Self {
        // Report metrics asynchronously
        self.async_reporting = true;
        self
    }
}
```

## Security Considerations

### Information Disclosure
- Performance metrics may reveal application internals
- Filter sensitive operation names in reports
- Control access to performance monitoring features

### Resource Consumption
- Performance monitoring itself consumes resources
- Prevent monitoring from impacting application performance
- Rate limiting for metric collection

## Future Extensions

### Distributed Monitoring
```rust
#[cfg(feature = "distributed")]
pub struct DistributedPerformanceMonitor {
    local_monitor: PerformanceMonitor,
    cluster_coordinator: ClusterCoordinator,
}

#[cfg(feature = "distributed")]
impl DistributedPerformanceMonitor {
    pub async fn aggregate_cluster_metrics(&self) -> Result<ClusterPerformanceReport, PerformanceError> {
        // Collect metrics from all nodes in cluster
        todo!()
    }

    pub async fn detect_cluster_bottlenecks(&self) -> Result<Vec<ClusterBottleneck>, PerformanceError> {
        // Find bottlenecks across the distributed system
        todo!()
    }
}
```

### Predictive Analytics
```rust
#[cfg(feature = "predictive")]
pub struct PredictivePerformanceMonitor {
    monitor: PerformanceMonitor,
    predictor: Box<dyn PerformancePredictor>,
}

#[cfg(feature = "predictive")]
impl PredictivePerformanceMonitor {
    pub fn predict_future_bottlenecks(&self) -> Result<Vec<FutureBottleneck>, PerformanceError> {
        // Use historical data to predict future performance issues
        todo!()
    }

    pub fn recommend_preemptive_optimizations(&self) -> Result<Vec<Optimization>, PerformanceError> {
        // Suggest optimizations before problems occur
        todo!()
    }
}
```

### Real-time Dashboards
```rust
#[cfg(all(feature = "performance", feature = "web"))]
pub struct RealtimeDashboard {
    monitor: PerformanceMonitor,
    update_interval: Duration,
    web_socket_server: WebSocketServer,
}

#[cfg(all(feature = "performance", feature = "web"))]
impl RealtimeDashboard {
    pub async fn serve_dashboard(&self, port: u16) -> Result<(), PerformanceError> {
        // Serve real-time performance dashboard over WebSocket
        todo!()
    }

    pub async fn broadcast_metrics(&self) -> Result<(), PerformanceError> {
        // Send metrics updates to connected dashboard clients
        todo!()
    }
}
```

## Migration Guide

### Adding Performance Monitoring
```rust
// Before - no monitoring
let store = Store::new(initial_state);
store.update(|s| s.count += 1).unwrap();

// After - with monitoring
#[cfg(feature = "performance")]
let monitor = PerformanceMonitor::new(PerformanceConfig::default());

#[cfg(feature = "performance")]
let store = Store::new(initial_state);
monitor.record_operation("update", || {
    store.update(|s| s.count += 1).unwrap();
}).unwrap();

// Or wrap the store
let monitored_store = MonitoredStore::new(store, monitor);
monitored_store.update(|s| s.count += 1).unwrap();
```

### Configuration-Based Monitoring
```rust
#[derive(Deserialize)]
pub struct MonitoringConfig {
    pub enable_performance_monitoring: bool,
    pub enable_alerts: bool,
    pub slow_operation_threshold_ms: u64,
    pub memory_alert_threshold_mb: usize,
    pub metrics_reporting_interval_seconds: u64,
}

pub fn create_monitored_store<S: State>(
    initial: S,
    config: &MonitoringConfig
) -> (Store<S>, Option<PerformanceMonitor>) {
    let store = Store::new(initial);

    if !config.enable_performance_monitoring {
        return (store, None);
    }

    let monitor_config = PerformanceConfig {
        enable_operation_timing: true,
        enable_memory_tracking: true,
        enable_error_tracking: true,
        slow_operation_threshold: Duration::from_millis(config.slow_operation_threshold_ms),
        memory_alert_threshold: config.memory_alert_threshold_mb * 1024 * 1024,
        metrics_retention_period: Duration::from_secs(3600),
    };

    let mut monitor = PerformanceMonitor::new(monitor_config);

    if config.enable_alerts {
        monitor = monitor
            .add_alert(SlowOperationAlert {
                threshold: Duration::from_millis(config.slow_operation_threshold_ms),
            })
            .add_alert(MemoryUsageAlert {
                threshold: config.memory_alert_threshold_mb * 1024 * 1024,
            });
    }

    // Add periodic reporter
    monitor = monitor.add_reporter(PeriodicReporter::new(
        Duration::from_secs(config.metrics_reporting_interval_seconds)
    ));

    (store, Some(monitor))
}
```

### Production vs Development Setup
```rust
pub fn setup_performance_monitoring<S: State>(store: Store<S>) -> MonitoredStore<S> {
    #[cfg(debug_assertions)]
    {
        // Development: Full monitoring
        let config = PerformanceConfig {
            enable_operation_timing: true,
            enable_memory_tracking: true,
            enable_error_tracking: true,
            slow_operation_threshold: Duration::from_millis(10),
            memory_alert_threshold: 50 * 1024 * 1024, // 50MB
            metrics_retention_period: Duration::from_secs(3600),
        };

        let monitor = PerformanceMonitor::new(config)
            .add_alert(SlowOperationAlert { threshold: Duration::from_millis(10) })
            .add_reporter(ConsoleReporter);

        MonitoredStore::new(store, monitor)
    }

    #[cfg(not(debug_assertions))]
    {
        // Production: Minimal monitoring
        let config = PerformanceConfig {
            enable_operation_timing: false,
            enable_memory_tracking: false,
            enable_error_tracking: true,
            slow_operation_threshold: Duration::from_millis(1000),
            memory_alert_threshold: 500 * 1024 * 1024, // 500MB
            metrics_retention_period: Duration::from_secs(86400), // 24 hours
        };

        let monitor = PerformanceMonitor::new(config)
            .add_reporter(JsonFileReporter::new("performance_metrics.json".into()));

        MonitoredStore::new(store, monitor)
    }
}
```

## Risk Assessment

### Likelihood: Low
- Performance monitoring is opt-in and generally safe
- Memory overhead is bounded and configurable
- Alert system can be configured to prevent false positives

### Impact: Low
- Monitoring can be disabled in production if needed
- Resource usage is minimal with default settings
- Clear error boundaries prevent monitoring failures from affecting application

### Mitigation
- Comprehensive configuration options for different environments
- Performance monitoring of the monitoring system itself
- Graceful degradation when monitoring fails
- Clear documentation on resource usage expectations
- Opt-in features with sensible production defaults
