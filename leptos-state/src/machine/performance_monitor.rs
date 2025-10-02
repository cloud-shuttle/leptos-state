//! Performance monitoring for state management operations
//!
//! This module provides comprehensive performance monitoring capabilities,
//! including metrics collection, bottleneck identification, and performance
//! optimization recommendations.
//!
//! This is a simplified performance monitoring system that works with
//! the relaxed trait bounds of the main library.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};

/// Errors that can occur during performance monitoring
#[derive(Debug, Clone, thiserror::Error)]
pub enum PerformanceError {
    #[error("Performance monitoring not available")]
    NotAvailable,
    #[error("Metrics collection failed: {message}")]
    MetricsFailed { message: String },
    #[error("Serialization failed: {message}")]
    SerializationFailed { message: String },
    #[error("Performance threshold exceeded: {operation} took {duration:?}")]
    ThresholdExceeded { operation: String, duration: Duration },
}

/// Performance metrics collected for operations
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DurationStats {
    pub count: u64,
    pub total_duration: u64, // microseconds
    pub average_duration: u64, // microseconds
    pub min_duration: u64, // microseconds
    pub max_duration: u64, // microseconds
    pub p50_duration: u64, // microseconds (median)
    pub p95_duration: u64, // microseconds
    pub p99_duration: u64, // microseconds
}

/// Memory usage statistics
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MemoryStats {
    pub current_usage: u64, // bytes
    pub peak_usage: u64, // bytes
    pub allocations: u64,
    pub deallocations: u64,
}

/// Overall performance metrics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub operation_counts: HashMap<String, u64>,
    pub operation_durations: HashMap<String, DurationStats>,
    pub memory_usage: MemoryStats,
    pub error_counts: HashMap<String, u64>,
    pub last_updated: u64, // unix timestamp
    pub uptime: u64, // seconds
    pub total_operations: u64,
}

/// Performance configuration
#[derive(Clone, Debug)]
pub struct PerformanceConfig {
    pub enabled: bool,
    pub collect_detailed_stats: bool,
    pub max_metrics_history: usize,
    pub alert_thresholds: HashMap<String, Duration>,
    pub memory_tracking_enabled: bool,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collect_detailed_stats: true,
            max_metrics_history: 1000,
            alert_thresholds: HashMap::new(),
            memory_tracking_enabled: true,
        }
    }
}

/// Performance operation recorder
pub struct OperationRecorder {
    operation: String,
    start_time: SystemTime,
    metrics: Arc<RwLock<PerformanceMetrics>>,
    config: PerformanceConfig,
}

impl OperationRecorder {
    /// Create a new operation recorder
    pub fn new(
        operation: String,
        metrics: Arc<RwLock<PerformanceMetrics>>,
        config: PerformanceConfig,
    ) -> Self {
        Self {
            operation,
            start_time: SystemTime::now(),
            metrics,
            config,
        }
    }

    /// Complete the operation and record metrics
    pub fn complete(self) {
        if !self.config.enabled {
            return;
        }

        let duration = self.start_time
            .elapsed()
            .unwrap_or_default()
            .as_micros() as u64;

        let mut metrics = self.metrics.write().unwrap();

        // Update operation counts
        *metrics.operation_counts.entry(self.operation.clone()).or_insert(0) += 1;
        metrics.total_operations += 1;

        // Update duration stats
        let stats = metrics.operation_durations
            .entry(self.operation.clone())
            .or_insert_with(|| DurationStats {
                count: 0,
                total_duration: 0,
                average_duration: 0,
                min_duration: u64::MAX,
                max_duration: 0,
                p50_duration: 0,
                p95_duration: 0,
                p99_duration: 0,
            });

        stats.count += 1;
        stats.total_duration += duration;
        stats.average_duration = stats.total_duration / stats.count;
        stats.min_duration = stats.min_duration.min(duration);
        stats.max_duration = stats.max_duration.max(duration);

        // Update timestamps
        metrics.last_updated = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Check thresholds
        if let Some(threshold) = self.config.alert_thresholds.get(&self.operation) {
            if Duration::from_micros(duration) > *threshold {
                // In a real implementation, this would trigger alerts
                // For now, we'll just log it
                eprintln!("Performance threshold exceeded for {}: {}μs", self.operation, duration);
            }
        }
    }
}

/// Main performance monitor
pub struct PerformanceMonitor {
    metrics: Arc<RwLock<PerformanceMetrics>>,
    config: PerformanceConfig,
    start_time: SystemTime,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        Self::with_config(PerformanceConfig::default())
    }

    /// Create a performance monitor with custom configuration
    pub fn with_config(config: PerformanceConfig) -> Self {
        let metrics = Arc::new(RwLock::new(PerformanceMetrics {
            operation_counts: HashMap::new(),
            operation_durations: HashMap::new(),
            memory_usage: MemoryStats::default(),
            error_counts: HashMap::new(),
            last_updated: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            uptime: 0,
            total_operations: 0,
        }));

        Self {
            metrics,
            config,
            start_time: SystemTime::now(),
        }
    }

    /// Record an operation with timing
    pub fn record_operation<F, R>(&self, operation: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let recorder = OperationRecorder::new(
            operation.to_string(),
            Arc::clone(&self.metrics),
            self.config.clone(),
        );

        let result = f();
        recorder.complete();
        result
    }

    /// Record an error for an operation
    pub fn record_error(&self, operation: &str, error_type: &str) {
        if !self.config.enabled {
            return;
        }

        let mut metrics = self.metrics.write().unwrap();
        *metrics.error_counts.entry(error_type.to_string()).or_insert(0) += 1;
    }

    /// Get current performance metrics
    pub fn get_metrics(&self) -> PerformanceMetrics {
        let mut metrics = self.metrics.read().unwrap().clone();
        metrics.uptime = self.start_time.elapsed().unwrap_or_default().as_secs();
        metrics
    }

    /// Get metrics as JSON string
    pub fn get_metrics_json(&self) -> Result<String, PerformanceError> {
        let metrics = self.get_metrics();
        serde_json::to_string(&metrics)
            .map_err(|e| PerformanceError::SerializationFailed {
                message: e.to_string(),
            })
    }

    /// Reset all metrics
    pub fn reset(&self) {
        let mut metrics = self.metrics.write().unwrap();
        *metrics = PerformanceMetrics {
            operation_counts: HashMap::new(),
            operation_durations: HashMap::new(),
            memory_usage: MemoryStats::default(),
            error_counts: HashMap::new(),
            last_updated: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            uptime: 0,
            total_operations: 0,
        };
    }

    /// Get operations that exceed performance thresholds
    pub fn get_bottlenecks(&self) -> Vec<(String, DurationStats)> {
        let metrics = self.metrics.read().unwrap();
        let mut bottlenecks = Vec::new();

        for (operation, stats) in &metrics.operation_durations {
            // Consider operations slow if average > 1ms or p95 > 5ms
            if stats.average_duration > 1000 || stats.p95_duration > 5000 {
                bottlenecks.push((operation.clone(), stats.clone()));
            }
        }

        bottlenecks.sort_by(|a, b| b.1.average_duration.cmp(&a.1.average_duration));
        bottlenecks
    }

    /// Generate performance recommendations
    pub fn get_recommendations(&self) -> Vec<String> {
        let bottlenecks = self.get_bottlenecks();
        let metrics = self.metrics.read().unwrap();
        let mut recommendations = Vec::new();

        if !bottlenecks.is_empty() {
            recommendations.push(format!(
                "Found {} performance bottlenecks. Consider optimizing slow operations.",
                bottlenecks.len()
            ));

            for (operation, stats) in bottlenecks.into_iter().take(5) {
                recommendations.push(format!(
                    "Operation '{}' is slow (avg: {}μs, p95: {}μs). Consider memoization or reducing complexity.",
                    operation, stats.average_duration, stats.p95_duration
                ));
            }
        }

        if metrics.total_operations > 10000 {
            recommendations.push(
                "High operation volume detected. Consider batching operations or using middleware.".to_string()
            );
        }

        if metrics.memory_usage.peak_usage > 100 * 1024 * 1024 { // 100MB
            recommendations.push(
                "High memory usage detected. Consider implementing state cleanup or using smaller state types.".to_string()
            );
        }

        recommendations
    }

    /// Check if performance monitoring is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Enable or disable performance monitoring
    pub fn set_enabled(&mut self, enabled: bool) {
        self.config.enabled = enabled;
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for PerformanceMonitor {
    fn clone(&self) -> Self {
        Self {
            metrics: Arc::clone(&self.metrics),
            config: self.config.clone(),
            start_time: self.start_time,
        }
    }
}
