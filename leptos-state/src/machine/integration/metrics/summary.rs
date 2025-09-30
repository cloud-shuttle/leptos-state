//! Metrics summary and aggregation functionality

use super::core::MetricsSnapshot;

/// Metrics summary
#[derive(Debug, Clone)]
pub struct MetricsSummary {
    /// Total events processed
    pub total_events: u64,
    /// Events per second
    pub events_per_second: f64,
    /// Error rate
    pub error_rate: f64,
    /// Filtering rate
    pub filtering_rate: f64,
    /// Average response time (nanoseconds)
    pub avg_response_time_ns: Option<f64>,
    /// 95th percentile response time (nanoseconds)
    pub p95_response_time_ns: Option<u64>,
    /// Total bytes transferred
    pub total_bytes: u64,
    /// Throughput (bytes per second)
    pub throughput_bytes_per_sec: f64,
    /// Active connections
    pub active_connections: u64,
    /// Connection success rate
    pub connection_success_rate: f64,
    /// Uptime in seconds
    pub uptime_seconds: u64,
    /// Health score (0.0 to 1.0)
    pub health_score: f64,
    /// Top error types
    pub top_errors: Vec<(String, u64)>,
    /// Adapter health scores
    pub adapter_health_scores: std::collections::HashMap<String, f64>,
}

impl MetricsSummary {
    /// Create summary from metrics snapshot
    pub fn from_snapshot(snapshot: &MetricsSnapshot) -> Self {
        let total_events = snapshot.events_sent + snapshot.events_received;
        let events_per_second = snapshot.event_processing_rate();
        let error_rate = snapshot.error_rate();
        let filtering_rate = snapshot.filtering_efficiency();

        // Calculate response times across all adapters
        let mut all_response_times = Vec::new();
        let mut total_bytes = 0u64;
        let mut active_connections = 0u64;
        let mut total_connections = 0u64;
        let mut successful_connections = 0u64;
        let mut adapter_health_scores = std::collections::HashMap::new();
        let mut all_errors = std::collections::HashMap::new();

        for (adapter_name, metrics) in &snapshot.adapter_metrics {
            // Collect response times
            all_response_times.extend_from_slice(&metrics.response_times);

            // Sum bytes
            total_bytes += metrics.bytes_sent + metrics.bytes_received;

            // Count connections
            if metrics.connection_status > 0 {
                active_connections += 1;
            }
            total_connections += metrics.connection_attempts;
            successful_connections += metrics.connections_successful;

            // Store health scores
            adapter_health_scores.insert(adapter_name.clone(), metrics.health_score());

            // Collect errors
            for (error_type, count) in &metrics.errors_by_type {
                *all_errors.entry(error_type.clone()).or_insert(0) += count;
            }
        }

        let connection_success_rate = if total_connections > 0 {
            successful_connections as f64 / total_connections as f64
        } else {
            0.0
        };

        // Calculate response time statistics
        let avg_response_time_ns = if !all_response_times.is_empty() {
            let sum: u64 = all_response_times.iter().sum();
            Some(sum as f64 / all_response_times.len() as f64)
        } else {
            None
        };

        let p95_response_time_ns = if !all_response_times.is_empty() {
            let mut sorted = all_response_times;
            sorted.sort();
            let index = (sorted.len() as f64 * 0.95) as usize;
            sorted.get(index.min(sorted.len() - 1)).copied()
        } else {
            None
        };

        let throughput_bytes_per_sec = if snapshot.uptime_seconds > 0 {
            total_bytes as f64 / snapshot.uptime_seconds as f64
        } else {
            0.0
        };

        // Calculate overall health score
        let event_health = if total_events > 0 { 1.0 - error_rate } else { 1.0 };
        let connection_health = connection_success_rate;
        let adapter_avg_health = if !adapter_health_scores.is_empty() {
            let sum: f64 = adapter_health_scores.values().sum();
            sum / adapter_health_scores.len() as f64
        } else {
            1.0
        };

        let health_score = (event_health * 0.4) + (connection_health * 0.3) + (adapter_avg_health * 0.3);

        // Get top 5 error types
        let mut top_errors: Vec<_> = all_errors.into_iter().collect();
        top_errors.sort_by(|a, b| b.1.cmp(&a.1));
        top_errors.truncate(5);

        Self {
            total_events,
            events_per_second,
            error_rate,
            filtering_rate,
            avg_response_time_ns,
            p95_response_time_ns,
            total_bytes,
            throughput_bytes_per_sec,
            active_connections,
            connection_success_rate,
            uptime_seconds: snapshot.uptime_seconds,
            health_score,
            top_errors,
            adapter_health_scores,
        }
    }

    /// Create an empty summary
    pub fn empty() -> Self {
        Self {
            total_events: 0,
            events_per_second: 0.0,
            error_rate: 0.0,
            filtering_rate: 0.0,
            avg_response_time_ns: None,
            p95_response_time_ns: None,
            total_bytes: 0,
            throughput_bytes_per_sec: 0.0,
            active_connections: 0,
            connection_success_rate: 0.0,
            uptime_seconds: 0,
            health_score: 1.0,
            top_errors: Vec::new(),
            adapter_health_scores: std::collections::HashMap::new(),
        }
    }

    /// Check if summary indicates healthy system
    pub fn is_healthy(&self) -> bool {
        self.health_score >= 0.8 && self.error_rate < 0.05
    }

    /// Check if summary indicates degraded performance
    pub fn is_degraded(&self) -> bool {
        self.health_score >= 0.5 && self.health_score < 0.8
    }

    /// Check if summary indicates unhealthy system
    pub fn is_unhealthy(&self) -> bool {
        self.health_score < 0.5 || self.error_rate >= 0.1
    }

    /// Get performance grade
    pub fn performance_grade(&self) -> &'static str {
        match self.health_score {
            h if h >= 0.9 => "A",
            h if h >= 0.8 => "B",
            h if h >= 0.7 => "C",
            h if h >= 0.6 => "D",
            _ => "F",
        }
    }

    /// Get formatted uptime
    pub fn uptime_formatted(&self) -> String {
        let secs = self.uptime_seconds;
        if secs < 60 {
            format!("{}s", secs)
        } else if secs < 3600 {
            format!("{}m {}s", secs / 60, secs % 60)
        } else if secs < 86400 {
            format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
        } else {
            format!("{}d {}h", secs / 86400, (secs % 86400) / 3600)
        }
    }

    /// Get formatted throughput
    pub fn throughput_formatted(&self) -> String {
        const UNITS: &[&str] = &["B/s", "KB/s", "MB/s", "GB/s"];
        let mut size = self.throughput_bytes_per_sec;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        format!("{:.1} {}", size, UNITS[unit_index])
    }

    /// Get summary text
    pub fn summary_text(&self) -> String {
        format!(
            "Integration Summary: {} events ({:.1}/s), {:.1}% errors, {} uptime, grade {}",
            self.total_events,
            self.events_per_second,
            self.error_rate * 100.0,
            self.uptime_formatted(),
            self.performance_grade()
        )
    }

    /// Get detailed report
    pub fn detailed_report(&self) -> String {
        let mut report = format!("Integration Metrics Report\n");
        report.push_str(&format!("={}\n", "=".repeat(40)));
        report.push_str(&format!("Total Events: {}\n", self.total_events));
        report.push_str(&format!("Events/Second: {:.2}\n", self.events_per_second));
        report.push_str(&format!("Error Rate: {:.2}%\n", self.error_rate * 100.0));
        report.push_str(&format!("Filtering Rate: {:.2}%\n", self.filtering_rate * 100.0));
        report.push_str(&format!("Active Connections: {}\n", self.active_connections));
        report.push_str(&format!("Connection Success: {:.1}%\n", self.connection_success_rate * 100.0));
        report.push_str(&format!("Uptime: {}\n", self.uptime_formatted()));
        report.push_str(&format!("Throughput: {}\n", self.throughput_formatted()));
        report.push_str(&format!("Health Score: {:.2}/1.0\n", self.health_score));
        report.push_str(&format!("Performance Grade: {}\n", self.performance_grade()));

        if let Some(avg_rt) = self.avg_response_time_ns {
            report.push_str(&format!("Avg Response Time: {:.2}ms\n", avg_rt / 1_000_000.0));
        }

        if let Some(p95_rt) = self.p95_response_time_ns {
            report.push_str(&format!("95th Percentile RT: {:.2}ms\n", p95_rt as f64 / 1_000_000.0));
        }

        if !self.top_errors.is_empty() {
            report.push_str(&format!("\nTop Errors:\n"));
            for (error_type, count) in &self.top_errors {
                report.push_str(&format!("  {}: {}\n", error_type, count));
            }
        }

        if !self.adapter_health_scores.is_empty() {
            report.push_str(&format!("\nAdapter Health Scores:\n"));
            for (adapter, score) in &self.adapter_health_scores {
                report.push_str(&format!("  {}: {:.2}\n", adapter, score));
            }
        }

        report
    }
}

impl std::fmt::Display for MetricsSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary_text())
    }
}

impl Default for MetricsSummary {
    fn default() -> Self {
        Self::empty()
    }
}
