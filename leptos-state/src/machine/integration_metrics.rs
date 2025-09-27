//! Integration metrics and monitoring

use super::*;

/// Integration metrics
#[derive(Debug, Clone)]
pub struct IntegrationMetrics {
    /// Total events sent
    pub events_sent: std::sync::atomic::AtomicU64,
    /// Total events received
    pub events_received: std::sync::atomic::AtomicU64,
    /// Total events filtered
    pub events_filtered: std::sync::atomic::AtomicU64,
    /// Total events unrouted
    pub events_unrouted: std::sync::atomic::AtomicU64,
    /// Total batches processed
    pub batches_processed: std::sync::atomic::AtomicU64,
    /// Total batches filtered
    pub batches_filtered: std::sync::atomic::AtomicU64,
    /// Total errors
    pub errors_total: std::sync::atomic::AtomicU64,
    /// Adapter-specific metrics
    pub adapter_metrics: std::sync::Mutex<std::collections::HashMap<String, AdapterMetrics>>,
    /// Start time
    pub start_time: std::time::Instant,
    /// Last reset time
    pub last_reset: std::sync::Mutex<Option<std::time::Instant>>,
}

impl IntegrationMetrics {
    /// Create a new metrics instance
    pub fn new() -> Self {
        Self {
            events_sent: std::sync::atomic::AtomicU64::new(0),
            events_received: std::sync::atomic::AtomicU64::new(0),
            events_filtered: std::sync::atomic::AtomicU64::new(0),
            events_unrouted: std::sync::atomic::AtomicU64::new(0),
            batches_processed: std::sync::atomic::AtomicU64::new(0),
            batches_filtered: std::sync::atomic::AtomicU64::new(0),
            errors_total: std::sync::atomic::AtomicU64::new(0),
            adapter_metrics: std::sync::Mutex::new(std::collections::HashMap::new()),
            start_time: std::time::Instant::now(),
            last_reset: std::sync::Mutex::new(None),
        }
    }

    /// Record a sent event
    pub fn record_sent_event(&self) {
        self.events_sent.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    /// Record received events
    pub fn record_received_events(&self, count: usize) {
        self.events_received.fetch_add(count as u64, std::sync::atomic::Ordering::Relaxed);
    }

    /// Record a filtered event
    pub fn record_filtered_event(&self) {
        self.events_filtered.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    /// Record a filtered batch
    pub fn record_filtered_batch(&self, size: usize) {
        self.batches_filtered.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.events_filtered.fetch_add(size as u64, std::sync::atomic::Ordering::Relaxed);
    }

    /// Record an unrouted event
    pub fn record_unrouted_event(&self) {
        self.events_unrouted.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    /// Record a processed batch
    pub fn record_processed_batch(&self) {
        self.batches_processed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    /// Record an error
    pub fn record_error(&self) {
        self.errors_total.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    /// Get adapter metrics
    pub fn get_adapter_metrics(&self, adapter_name: &str) -> AdapterMetrics {
        let mut metrics = self.adapter_metrics.lock().unwrap();
        metrics.entry(adapter_name.to_string())
            .or_insert_with(AdapterMetrics::new)
            .clone()
    }

    /// Update adapter metrics
    pub fn update_adapter_metrics(&self, adapter_name: &str, metrics: AdapterMetrics) {
        let mut adapter_metrics = self.adapter_metrics.lock().unwrap();
        adapter_metrics.insert(adapter_name.to_string(), metrics);
    }

    /// Get summary statistics
    pub fn get_summary(&self) -> MetricsSummary {
        let events_sent = self.events_sent.load(std::sync::atomic::Ordering::Relaxed);
        let events_received = self.events_received.load(std::sync::atomic::Ordering::Relaxed);
        let events_filtered = self.events_filtered.load(std::sync::atomic::Ordering::Relaxed);
        let events_unrouted = self.events_unrouted.load(std::sync::atomic::Ordering::Relaxed);
        let batches_processed = self.batches_processed.load(std::sync::atomic::Ordering::Relaxed);
        let batches_filtered = self.batches_filtered.load(std::sync::atomic::Ordering::Relaxed);
        let errors_total = self.errors_total.load(std::sync::atomic::Ordering::Relaxed);

        let total_events = events_sent + events_received;
        let success_rate = if total_events > 0 {
            ((total_events - errors_total) as f64 / total_events as f64) * 100.0
        } else {
            0.0
        };

        MetricsSummary {
            events_sent,
            events_received,
            events_filtered,
            events_unrouted,
            batches_processed,
            batches_filtered,
            errors_total,
            success_rate,
            uptime: self.start_time.elapsed(),
            adapter_count: self.adapter_metrics.lock().unwrap().len(),
        }
    }

    /// Reset all metrics
    pub fn reset(&self) {
        self.events_sent.store(0, std::sync::atomic::Ordering::Relaxed);
        self.events_received.store(0, std::sync::atomic::Ordering::Relaxed);
        self.events_filtered.store(0, std::sync::atomic::Ordering::Relaxed);
        self.events_unrouted.store(0, std::sync::atomic::Ordering::Relaxed);
        self.batches_processed.store(0, std::sync::atomic::Ordering::Relaxed);
        self.batches_filtered.store(0, std::sync::atomic::Ordering::Relaxed);
        self.errors_total.store(0, std::sync::atomic::Ordering::Relaxed);

        *self.adapter_metrics.lock().unwrap() = std::collections::HashMap::new();
        *self.last_reset.lock().unwrap() = Some(std::time::Instant::now());
    }

    /// Export metrics in Prometheus format
    pub fn to_prometheus(&self) -> String {
        let summary = self.get_summary();
        let mut output = String::new();

        output.push_str("# HELP integration_events_sent_total Total number of events sent\n");
        output.push_str("# TYPE integration_events_sent_total counter\n");
        output.push_str(&format!("integration_events_sent_total {}\n", summary.events_sent));

        output.push_str("# HELP integration_events_received_total Total number of events received\n");
        output.push_str("# TYPE integration_events_received_total counter\n");
        output.push_str(&format!("integration_events_received_total {}\n", summary.events_received));

        output.push_str("# HELP integration_events_filtered_total Total number of events filtered\n");
        output.push_str("# TYPE integration_events_filtered_total counter\n");
        output.push_str(&format!("integration_events_filtered_total {}\n", summary.events_filtered));

        output.push_str("# HELP integration_errors_total Total number of errors\n");
        output.push_str("# TYPE integration_errors_total counter\n");
        output.push_str(&format!("integration_errors_total {}\n", summary.errors_total));

        output.push_str("# HELP integration_success_rate Success rate percentage\n");
        output.push_str("# TYPE integration_success_rate gauge\n");
        output.push_str(&format!("integration_success_rate {}\n", summary.success_rate));

        output
    }
}

/// Adapter-specific metrics
#[derive(Debug, Clone)]
pub struct AdapterMetrics {
    /// Events sent by this adapter
    pub events_sent: u64,
    /// Events received by this adapter
    pub events_received: u64,
    /// Errors encountered by this adapter
    pub errors: u64,
    /// Average processing time
    pub avg_processing_time: std::time::Duration,
    /// Last activity timestamp
    pub last_activity: Option<std::time::Instant>,
    /// Health status
    pub health_status: HealthStatus,
}

impl AdapterMetrics {
    /// Create new adapter metrics
    pub fn new() -> Self {
        Self {
            events_sent: 0,
            events_received: 0,
            errors: 0,
            avg_processing_time: std::time::Duration::from_nanos(0),
            last_activity: None,
            health_status: HealthStatus::Unknown,
        }
    }

    /// Record a sent event
    pub fn record_sent_event(&mut self, processing_time: std::time::Duration) {
        self.events_sent += 1;
        self.update_avg_processing_time(processing_time);
        self.last_activity = Some(std::time::Instant::now());
    }

    /// Record a received event
    pub fn record_received_event(&mut self) {
        self.events_received += 1;
        self.last_activity = Some(std::time::Instant::now());
    }

    /// Record an error
    pub fn record_error(&mut self) {
        self.errors += 1;
        self.last_activity = Some(std::time::Instant::now());
    }

    /// Update average processing time
    pub fn update_avg_processing_time(&mut self, new_time: std::time::Duration) {
        let total_events = self.events_sent + self.events_received;
        if total_events > 0 {
            let current_total = self.avg_processing_time * (total_events - 1) as u32;
            self.avg_processing_time = (current_total + new_time) / total_events as u32;
        } else {
            self.avg_processing_time = new_time;
        }
    }

    /// Update health status
    pub fn update_health(&mut self, status: HealthStatus) {
        self.health_status = status;
    }

    /// Check if adapter is active (has activity within last 5 minutes)
    pub fn is_active(&self) -> bool {
        self.last_activity
            .map(|time| time.elapsed() < std::time::Duration::from_secs(300))
            .unwrap_or(false)
    }

    /// Get success rate for this adapter
    pub fn success_rate(&self) -> f64 {
        let total_operations = self.events_sent + self.events_received;
        if total_operations > 0 {
            ((total_operations - self.errors) as f64 / total_operations as f64) * 100.0
        } else {
            0.0
        }
    }
}

/// Metrics summary
#[derive(Debug, Clone)]
pub struct MetricsSummary {
    /// Total events sent
    pub events_sent: u64,
    /// Total events received
    pub events_received: u64,
    /// Total events filtered
    pub events_filtered: u64,
    /// Total events unrouted
    pub events_unrouted: u64,
    /// Total batches processed
    pub batches_processed: u64,
    /// Total batches filtered
    pub batches_filtered: u64,
    /// Total errors
    pub errors_total: u64,
    /// Success rate percentage
    pub success_rate: f64,
    /// System uptime
    pub uptime: std::time::Duration,
    /// Number of active adapters
    pub adapter_count: usize,
}

impl MetricsSummary {
    /// Get events per second rate
    pub fn events_per_second(&self) -> f64 {
        let uptime_secs = self.uptime.as_secs_f64();
        if uptime_secs == 0.0 {
            0.0
        } else {
            (self.events_sent + self.events_received) as f64 / uptime_secs
        }
    }

    /// Get error rate (errors per minute)
    pub fn error_rate_per_minute(&self) -> f64 {
        let uptime_mins = self.uptime.as_secs_f64() / 60.0;
        if uptime_mins == 0.0 {
            0.0
        } else {
            self.errors_total as f64 / uptime_mins
        }
    }

    /// Check if system is healthy
    pub fn is_healthy(&self) -> bool {
        self.success_rate >= 95.0 && self.errors_total < 100
    }
}

/// Performance monitoring
pub struct PerformanceMonitor {
    /// Response time percentiles
    pub response_times: std::collections::HashMap<String, Percentiles>,
    /// Throughput measurements
    pub throughput: std::collections::HashMap<String, ThroughputMetrics>,
    /// Resource usage
    pub resource_usage: ResourceUsage,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        Self {
            response_times: std::collections::HashMap::new(),
            throughput: std::collections::HashMap::new(),
            resource_usage: ResourceUsage::new(),
        }
    }

    /// Record response time
    pub fn record_response_time(&mut self, operation: &str, duration: std::time::Duration) {
        let percentiles = self.response_times.entry(operation.to_string())
            .or_insert_with(Percentiles::new);
        percentiles.add_sample(duration);
    }

    /// Record throughput
    pub fn record_throughput(&mut self, operation: &str, count: u64) {
        let throughput = self.throughput.entry(operation.to_string())
            .or_insert_with(ThroughputMetrics::new);
        throughput.record_operations(count);
    }

    /// Update resource usage
    pub fn update_resource_usage(&mut self) {
        self.resource_usage.update();
    }

    /// Get performance report
    pub fn get_report(&self) -> PerformanceReport {
        PerformanceReport {
            response_times: self.response_times.clone(),
            throughput: self.throughput.clone(),
            resource_usage: self.resource_usage.clone(),
            generated_at: std::time::Instant::now(),
        }
    }
}

/// Response time percentiles
#[derive(Debug, Clone)]
pub struct Percentiles {
    samples: Vec<std::time::Duration>,
}

impl Percentiles {
    /// Create new percentiles tracker
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
        }
    }

    /// Add a sample
    pub fn add_sample(&mut self, duration: std::time::Duration) {
        self.samples.push(duration);

        // Keep only last 1000 samples
        if self.samples.len() > 1000 {
            self.samples.remove(0);
        }
    }

    /// Get p50 (median)
    pub fn p50(&self) -> std::time::Duration {
        self.percentile(50.0)
    }

    /// Get p95
    pub fn p95(&self) -> std::time::Duration {
        self.percentile(95.0)
    }

    /// Get p99
    pub fn p99(&self) -> std::time::Duration {
        self.percentile(99.0)
    }

    /// Calculate percentile
    fn percentile(&self, p: f64) -> std::time::Duration {
        if self.samples.is_empty() {
            return std::time::Duration::from_nanos(0);
        }

        let mut sorted = self.samples.clone();
        sorted.sort();

        let index = ((p / 100.0) * (sorted.len() - 1) as f64) as usize;
        sorted[index]
    }
}

/// Throughput metrics
#[derive(Debug, Clone)]
pub struct ThroughputMetrics {
    total_operations: u64,
    start_time: std::time::Instant,
}

impl ThroughputMetrics {
    /// Create new throughput metrics
    pub fn new() -> Self {
        Self {
            total_operations: 0,
            start_time: std::time::Instant::now(),
        }
    }

    /// Record operations
    pub fn record_operations(&mut self, count: u64) {
        self.total_operations += count;
    }

    /// Get operations per second
    pub fn ops_per_second(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed == 0.0 {
            0.0
        } else {
            self.total_operations as f64 / elapsed
        }
    }
}

/// Resource usage metrics
#[derive(Debug, Clone)]
pub struct ResourceUsage {
    /// CPU usage percentage
    pub cpu_percent: f64,
    /// Memory usage in bytes
    pub memory_bytes: u64,
    /// Disk I/O operations per second
    pub disk_iops: f64,
    /// Network bytes per second
    pub network_bps: f64,
    /// Last updated
    pub last_updated: std::time::Instant,
}

impl ResourceUsage {
    /// Create new resource usage tracker
    pub fn new() -> Self {
        Self {
            cpu_percent: 0.0,
            memory_bytes: 0,
            disk_iops: 0.0,
            network_bps: 0.0,
            last_updated: std::time::Instant::now(),
        }
    }

    /// Update resource usage (simplified implementation)
    pub fn update(&mut self) {
        // In a real implementation, this would query system metrics
        // For now, use placeholder values
        self.cpu_percent = 25.0; // 25% CPU usage
        self.memory_bytes = 100 * 1024 * 1024; // 100 MB
        self.disk_iops = 150.0; // 150 IOPS
        self.network_bps = 1024.0 * 1024.0; // 1 MB/s
        self.last_updated = std::time::Instant::now();
    }
}

/// Performance report
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    /// Response time percentiles by operation
    pub response_times: std::collections::HashMap<String, Percentiles>,
    /// Throughput metrics by operation
    pub throughput: std::collections::HashMap<String, ThroughputMetrics>,
    /// Current resource usage
    pub resource_usage: ResourceUsage,
    /// Report generation time
    pub generated_at: std::time::Instant,
}
