//! Performance analysis and reporting structures

/// Response time percentiles
#[derive(Debug, Clone, Default)]
pub struct Percentiles {
    /// 50th percentile (median)
    pub p50: u64,
    /// 75th percentile
    pub p75: u64,
    /// 90th percentile
    pub p90: u64,
    /// 95th percentile
    pub p95: u64,
    /// 99th percentile
    pub p99: u64,
    /// 99.9th percentile
    pub p999: u64,
    /// Minimum value
    pub min: u64,
    /// Maximum value
    pub max: u64,
    /// Sample count
    pub count: usize,
}

impl Percentiles {
    /// Calculate percentiles from response times
    pub fn from_response_times(response_times: &[u64]) -> Self {
        if response_times.is_empty() {
            return Self::default();
        }

        let mut sorted = response_times.to_vec();
        sorted.sort();

        let len = sorted.len();
        let p50 = sorted[len / 2];
        let p75 = sorted[(len * 3) / 4];
        let p90 = sorted[(len * 9) / 10];
        let p95 = sorted[(len * 95) / 100];
        let p99 = sorted[(len * 99) / 100];
        let p999 = sorted[(len * 999) / 1000];
        let min = sorted[0];
        let max = sorted[len - 1];

        Self {
            p50,
            p75,
            p90,
            p95,
            p99,
            p999,
            min,
            max,
            count: len,
        }
    }

    /// Get percentile by value (0.0 to 1.0)
    pub fn percentile(&self, p: f64) -> Option<u64> {
        if !(0.0..=1.0).contains(&p) {
            return None;
        }

        match p {
            0.5 => Some(self.p50),
            0.75 => Some(self.p75),
            0.9 => Some(self.p90),
            0.95 => Some(self.p95),
            0.99 => Some(self.p99),
            0.999 => Some(self.p999),
            0.0 => Some(self.min),
            1.0 => Some(self.max),
            _ => None, // Would need interpolation for other values
        }
    }

    /// Get formatted percentile string
    pub fn format_percentile(&self, p: f64) -> String {
        self.percentile(p)
            .map(|ns| format!("{:.2}ms", ns as f64 / 1_000_000.0))
            .unwrap_or("N/A".to_string())
    }

    /// Get summary
    pub fn summary(&self) -> String {
        format!(
            "Percentiles {{ p50: {}, p95: {}, p99: {}, min: {}, max: {}, count: {} }}",
            self.format_percentile(0.5),
            self.format_percentile(0.95),
            self.format_percentile(0.99),
            self.format_percentile(0.0),
            self.format_percentile(1.0),
            self.count
        )
    }
}

impl std::fmt::Display for Percentiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Throughput metrics
#[derive(Debug, Clone, Default)]
pub struct ThroughputMetrics {
    /// Events per second
    pub events_per_second: f64,
    /// Messages per second
    pub messages_per_second: f64,
    /// Bytes per second
    pub bytes_per_second: f64,
    /// Requests per second
    pub requests_per_second: f64,
    /// Measurement period in seconds
    pub period_seconds: u64,
    /// Peak events per second
    pub peak_events_per_second: f64,
    /// Peak messages per second
    pub peak_messages_per_second: f64,
    /// Peak bytes per second
    pub peak_bytes_per_second: f64,
    /// Sustained rate (95th percentile)
    pub sustained_events_per_second: f64,
}

impl ThroughputMetrics {
    /// Create from metrics data
    pub fn from_data(
        total_events: u64,
        total_messages: u64,
        total_bytes: u64,
        total_requests: u64,
        period_seconds: u64,
    ) -> Self {
        let events_per_second = if period_seconds > 0 {
            total_events as f64 / period_seconds as f64
        } else {
            0.0
        };

        let messages_per_second = if period_seconds > 0 {
            total_messages as f64 / period_seconds as f64
        } else {
            0.0
        };

        let bytes_per_second = if period_seconds > 0 {
            total_bytes as f64 / period_seconds as f64
        } else {
            0.0
        };

        let requests_per_second = if period_seconds > 0 {
            total_requests as f64 / period_seconds as f64
        } else {
            0.0
        };

        Self {
            events_per_second,
            messages_per_second,
            bytes_per_second,
            requests_per_second,
            period_seconds,
            peak_events_per_second: events_per_second, // Simplified
            peak_messages_per_second: messages_per_second,
            peak_bytes_per_second: bytes_per_second,
            sustained_events_per_second: events_per_second * 0.95, // Simplified
        }
    }

    /// Format bytes per second
    pub fn bytes_per_second_formatted(&self) -> String {
        format_bytes_per_second(self.bytes_per_second)
    }

    /// Check if throughput is within acceptable range
    pub fn is_within_range(&self, min_eps: f64, max_eps: f64) -> bool {
        self.events_per_second >= min_eps && self.events_per_second <= max_eps
    }

    /// Get throughput efficiency (sustained / peak)
    pub fn efficiency(&self) -> f64 {
        if self.peak_events_per_second > 0.0 {
            self.sustained_events_per_second / self.peak_events_per_second
        } else {
            0.0
        }
    }

    /// Get summary
    pub fn summary(&self) -> String {
        format!(
            "Throughput {{ {:.1} evt/s, {:.1} msg/s, {}, {:.1} req/s }}",
            self.events_per_second,
            self.messages_per_second,
            self.bytes_per_second_formatted(),
            self.requests_per_second
        )
    }
}

impl std::fmt::Display for ThroughputMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Resource usage metrics
#[derive(Debug, Clone, Default)]
pub struct ResourceUsage {
    /// CPU usage percentage (0.0 to 100.0)
    pub cpu_usage_percent: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// Peak memory usage in bytes
    pub peak_memory_bytes: u64,
    /// Disk I/O bytes per second
    pub disk_io_bytes_per_sec: f64,
    /// Network I/O bytes per second
    pub network_io_bytes_per_sec: f64,
    /// Thread count
    pub thread_count: usize,
    /// File descriptor count
    pub fd_count: usize,
    /// Connection count
    pub connection_count: usize,
    /// Uptime in seconds
    pub uptime_seconds: u64,
}

impl ResourceUsage {
    /// Create from system metrics
    pub fn from_system() -> Self {
        // This would integrate with system monitoring libraries
        // For now, return default values
        Self::default()
    }

    /// Update with current values
    pub fn update(&mut self) {
        // This would update from system monitoring
        // For now, just increment uptime
        self.uptime_seconds += 1;
    }

    /// Get memory usage formatted
    pub fn memory_usage_formatted(&self) -> String {
        format_bytes(self.memory_usage_bytes)
    }

    /// Get peak memory usage formatted
    pub fn peak_memory_formatted(&self) -> String {
        format_bytes(self.peak_memory_bytes)
    }

    /// Check if resource usage is within limits
    pub fn within_limits(&self, cpu_limit: f64, memory_limit_bytes: u64) -> bool {
        self.cpu_usage_percent <= cpu_limit && self.memory_usage_bytes <= memory_limit_bytes
    }

    /// Get resource efficiency score (0.0 to 1.0, higher is better)
    pub fn efficiency_score(&self) -> f64 {
        let cpu_efficiency = 1.0 - (self.cpu_usage_percent / 100.0);
        let memory_efficiency = if self.peak_memory_bytes > 0 {
            1.0 - (self.memory_usage_bytes as f64 / self.peak_memory_bytes as f64)
        } else {
            1.0
        };

        (cpu_efficiency + memory_efficiency) / 2.0
    }

    /// Get summary
    pub fn summary(&self) -> String {
        format!(
            "Resources {{ CPU: {:.1}%, Mem: {}, Threads: {}, Conns: {} }}",
            self.cpu_usage_percent,
            self.memory_usage_formatted(),
            self.thread_count,
            self.connection_count
        )
    }
}

impl std::fmt::Display for ResourceUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Performance report
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    /// Report timestamp
    pub timestamp: std::time::SystemTime,
    /// Metrics summary
    pub summary: super::summary::MetricsSummary,
    /// Percentiles
    pub percentiles: Percentiles,
    /// Throughput metrics
    pub throughput: ThroughputMetrics,
    /// Resource usage
    pub resources: ResourceUsage,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Alerts
    pub alerts: Vec<String>,
}

impl PerformanceReport {
    /// Create a new performance report
    pub fn new(
        summary: super::summary::MetricsSummary,
        percentiles: Percentiles,
        throughput: ThroughputMetrics,
        resources: ResourceUsage,
    ) -> Self {
        let mut report = Self {
            timestamp: std::time::SystemTime::now(),
            summary,
            percentiles,
            throughput,
            resources,
            recommendations: Vec::new(),
            alerts: Vec::new(),
        };

        report.generate_recommendations();
        report.generate_alerts();

        report
    }

    /// Generate performance recommendations
    fn generate_recommendations(&mut self) {
        if self.summary.error_rate > 0.05 {
            self.recommendations.push(
                "High error rate detected. Consider implementing circuit breakers or retry logic.".to_string()
            );
        }

        if self.summary.events_per_second < 10.0 {
            self.recommendations.push(
                "Low throughput detected. Consider optimizing event processing pipeline.".to_string()
            );
        }

        if let Some(avg_rt) = self.percentiles.avg_response_time_ns {
            if avg_rt > 100_000_000 { // 100ms
                self.recommendations.push(
                    "High response times detected. Consider optimizing I/O operations.".to_string()
                );
            }
        }

        if self.resources.cpu_usage_percent > 80.0 {
            self.recommendations.push(
                "High CPU usage detected. Consider scaling or optimizing compute-intensive operations.".to_string()
            );
        }

        if self.resources.memory_usage_bytes > 1_000_000_000 { // 1GB
            self.recommendations.push(
                "High memory usage detected. Consider optimizing memory management.".to_string()
            );
        }

        if self.recommendations.is_empty() {
            self.recommendations.push("Performance is within acceptable ranges.".to_string());
        }
    }

    /// Generate alerts
    fn generate_alerts(&mut self) {
        if self.summary.error_rate > 0.1 {
            self.alerts.push(format!("CRITICAL: Error rate is {:.1}%", self.summary.error_rate * 100.0));
        }

        if self.summary.health_score < 0.5 {
            self.alerts.push(format!("CRITICAL: Health score is {:.2}", self.summary.health_score));
        }

        if self.resources.cpu_usage_percent > 95.0 {
            self.alerts.push(format!("CRITICAL: CPU usage is {:.1}%", self.resources.cpu_usage_percent));
        }

        if self.resources.memory_usage_bytes > 2_000_000_000 { // 2GB
            self.alerts.push(format!("WARNING: Memory usage is {}", self.resources.memory_usage_formatted()));
        }

        if self.summary.active_connections == 0 {
            self.alerts.push("WARNING: No active connections detected".to_string());
        }
    }

    /// Check if report has critical issues
    pub fn has_critical_issues(&self) -> bool {
        !self.alerts.is_empty() && self.alerts.iter().any(|alert| alert.starts_with("CRITICAL"))
    }

    /// Check if report has warnings
    pub fn has_warnings(&self) -> bool {
        !self.alerts.is_empty() && self.alerts.iter().any(|alert| alert.starts_with("WARNING"))
    }

    /// Get overall status
    pub fn status(&self) -> &'static str {
        if self.has_critical_issues() {
            "CRITICAL"
        } else if self.has_warnings() {
            "WARNING"
        } else if self.summary.is_healthy() {
            "HEALTHY"
        } else {
            "DEGRADED"
        }
    }

    /// Export to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Generate text report
    pub fn text_report(&self) -> String {
        let mut report = format!("Performance Report - {}\n", self.status());
        report.push_str(&format!("Generated: {}\n", self.timestamp.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs()));
        report.push_str(&format!("={}\n", "=".repeat(50)));

        report.push_str(&format!("Summary:\n  {}\n\n", self.summary.summary_text()));

        report.push_str(&format!("Response Times:\n  {}\n\n", self.percentiles.summary()));

        report.push_str(&format!("Throughput:\n  {}\n\n", self.throughput.summary()));

        report.push_str(&format!("Resources:\n  {}\n\n", self.resources.summary()));

        if !self.alerts.is_empty() {
            report.push_str("Alerts:\n");
            for alert in &self.alerts {
                report.push_str(&format!("  ⚠️  {}\n", alert));
            }
            report.push('\n');
        }

        if !self.recommendations.is_empty() {
            report.push_str("Recommendations:\n");
            for rec in &self.recommendations {
                report.push_str(&format!("  💡 {}\n", rec));
            }
            report.push('\n');
        }

        report
    }
}

impl std::fmt::Display for PerformanceReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text_report())
    }
}

/// Helper function to format bytes
fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, UNITS[unit_index])
}

/// Helper function to format bytes per second
fn format_bytes_per_second(bps: f64) -> String {
    format!("{}/s", format_bytes(bps as u64))
}
