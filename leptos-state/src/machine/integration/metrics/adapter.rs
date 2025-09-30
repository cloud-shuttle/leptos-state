//! Adapter-specific metrics and monitoring

/// Adapter-specific metrics
#[derive(Debug, Clone, Default)]
pub struct AdapterMetrics {
    /// Events sent by this adapter
    pub events_sent: u64,
    /// Events received by this adapter
    pub events_received: u64,
    /// Messages sent by this adapter
    pub messages_sent: u64,
    /// Messages received by this adapter
    pub messages_received: u64,
    /// Connection attempts
    pub connection_attempts: u64,
    /// Successful connections
    pub connections_successful: u64,
    /// Failed connections
    pub connections_failed: u64,
    /// Current connection status (0 = disconnected, 1 = connected)
    pub connection_status: u64,
    /// Total bytes sent
    pub bytes_sent: u64,
    /// Total bytes received
    pub bytes_received: u64,
    /// Response times (in nanoseconds) - last N measurements
    pub response_times: Vec<u64>,
    /// Error counts by type
    pub errors_by_type: std::collections::HashMap<String, u64>,
    /// Last activity timestamp
    pub last_activity: Option<std::time::Instant>,
    /// Uptime in seconds
    pub uptime_seconds: u64,
    /// Adapter-specific custom metrics
    pub custom_metrics: std::collections::HashMap<String, serde_json::Value>,
}

impl AdapterMetrics {
    /// Create new adapter metrics
    pub fn new() -> Self {
        Self::default()
    }

    /// Record an event sent
    pub fn record_event_sent(&mut self) {
        self.events_sent += 1;
        self.update_activity();
    }

    /// Record an event received
    pub fn record_event_received(&mut self) {
        self.events_received += 1;
        self.update_activity();
    }

    /// Record a message sent
    pub fn record_message_sent(&mut self, size_bytes: usize) {
        self.messages_sent += 1;
        self.bytes_sent += size_bytes as u64;
        self.update_activity();
    }

    /// Record a message received
    pub fn record_message_received(&mut self, size_bytes: usize) {
        self.messages_received += 1;
        self.bytes_received += size_bytes as u64;
        self.update_activity();
    }

    /// Record a connection attempt
    pub fn record_connection_attempt(&mut self) {
        self.connection_attempts += 1;
    }

    /// Record a successful connection
    pub fn record_connection_success(&mut self) {
        self.connections_successful += 1;
        self.connection_status = 1;
        self.update_activity();
    }

    /// Record a failed connection
    pub fn record_connection_failure(&mut self) {
        self.connections_failed += 1;
        self.connection_status = 0;
    }

    /// Record a response time
    pub fn record_response_time(&mut self, duration_ns: u64) {
        self.response_times.push(duration_ns);
        // Keep only last 100 measurements to avoid unbounded growth
        if self.response_times.len() > 100 {
            self.response_times.remove(0);
        }
        self.update_activity();
    }

    /// Record an error
    pub fn record_error(&mut self, error_type: &str) {
        *self.errors_by_type.entry(error_type.to_string()).or_insert(0) += 1;
        self.update_activity();
    }

    /// Set custom metric
    pub fn set_custom_metric(&mut self, key: String, value: serde_json::Value) {
        self.custom_metrics.insert(key, value);
    }

    /// Update activity timestamp
    fn update_activity(&mut self) {
        self.last_activity = Some(std::time::Instant::now());
    }

    /// Get connection success rate
    pub fn connection_success_rate(&self) -> f64 {
        if self.connection_attempts > 0 {
            self.connections_successful as f64 / self.connection_attempts as f64
        } else {
            0.0
        }
    }

    /// Get average response time
    pub fn average_response_time_ns(&self) -> Option<f64> {
        if self.response_times.is_empty() {
            None
        } else {
            let sum: u64 = self.response_times.iter().sum();
            Some(sum as f64 / self.response_times.len() as f64)
        }
    }

    /// Get median response time
    pub fn median_response_time_ns(&self) -> Option<u64> {
        if self.response_times.is_empty() {
            None
        } else {
            let mut sorted = self.response_times.clone();
            sorted.sort();
            let mid = sorted.len() / 2;
            Some(sorted[mid])
        }
    }

    /// Get 95th percentile response time
    pub fn p95_response_time_ns(&self) -> Option<u64> {
        if self.response_times.is_empty() {
            None
        } else {
            let mut sorted = self.response_times.clone();
            sorted.sort();
            let index = (sorted.len() as f64 * 0.95) as usize;
            sorted.get(index).copied()
        }
    }

    /// Get error rate (errors per message)
    pub fn error_rate(&self) -> f64 {
        let total_messages = self.messages_sent + self.messages_received;
        if total_messages > 0 {
            let total_errors: u64 = self.errors_by_type.values().sum();
            total_errors as f64 / total_messages as f64
        } else {
            0.0
        }
    }

    /// Get throughput (messages per second)
    pub fn messages_per_second(&self) -> f64 {
        if self.uptime_seconds > 0 {
            let total_messages = self.messages_sent + self.messages_received;
            total_messages as f64 / self.uptime_seconds as f64
        } else {
            0.0
        }
    }

    /// Get bandwidth (bytes per second)
    pub fn bytes_per_second(&self) -> f64 {
        if self.uptime_seconds > 0 {
            let total_bytes = self.bytes_sent + self.bytes_received;
            total_bytes as f64 / self.uptime_seconds as f64
        } else {
            0.0
        }
    }

    /// Check if adapter is currently active
    pub fn is_active(&self) -> bool {
        self.connection_status > 0
    }

    /// Check if adapter has recent activity
    pub fn has_recent_activity(&self, threshold_seconds: u64) -> bool {
        self.last_activity
            .map(|t| t.elapsed().as_secs() < threshold_seconds)
            .unwrap_or(false)
    }

    /// Get most common error type
    pub fn most_common_error(&self) -> Option<(&String, u64)> {
        self.errors_by_type
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(error_type, count)| (error_type, *count))
    }

    /// Get total error count
    pub fn total_errors(&self) -> u64 {
        self.errors_by_type.values().sum()
    }

    /// Get health score (0.0 to 1.0, higher is better)
    pub fn health_score(&self) -> f64 {
        let connection_score = if self.connection_attempts > 0 {
            self.connection_success_rate()
        } else {
            1.0
        };

        let error_score = 1.0 - self.error_rate().min(1.0);

        let activity_score = if self.has_recent_activity(300) { 1.0 } else { 0.0 };

        // Weighted average
        (connection_score * 0.4) + (error_score * 0.4) + (activity_score * 0.2)
    }

    /// Reset metrics
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Get metrics summary
    pub fn summary(&self) -> String {
        format!(
            "AdapterMetrics {{ sent: {}, received: {}, errors: {}, health: {:.2}, active: {} }}",
            self.events_sent + self.messages_sent,
            self.events_received + self.messages_received,
            self.total_errors(),
            self.health_score(),
            self.is_active()
        )
    }

    /// Merge with another adapter metrics instance
    pub fn merge(&mut self, other: &AdapterMetrics) {
        self.events_sent += other.events_sent;
        self.events_received += other.events_received;
        self.messages_sent += other.messages_sent;
        self.messages_received += other.messages_received;
        self.connection_attempts += other.connection_attempts;
        self.connections_successful += other.connections_successful;
        self.connections_failed += other.connections_failed;
        self.bytes_sent += other.bytes_sent;
        self.bytes_received += other.bytes_received;
        self.response_times.extend_from_slice(&other.response_times);
        self.uptime_seconds += other.uptime_seconds;

        // Merge error counts
        for (error_type, count) in &other.errors_by_type {
            *self.errors_by_type.entry(error_type.clone()).or_insert(0) += count;
        }

        // Update connection status and activity
        if other.connection_status > 0 {
            self.connection_status = 1;
        }

        if let Some(other_activity) = other.last_activity {
            match self.last_activity {
                Some(current) if other_activity > current => {
                    self.last_activity = Some(other_activity);
                }
                None => {
                    self.last_activity = Some(other_activity);
                }
                _ => {}
            }
        }
    }
}

impl std::fmt::Display for AdapterMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Adapter metrics aggregator for multiple adapters
pub struct AdapterMetricsAggregator {
    adapters: std::collections::HashMap<String, AdapterMetrics>,
}

impl AdapterMetricsAggregator {
    /// Create a new aggregator
    pub fn new() -> Self {
        Self {
            adapters: std::collections::HashMap::new(),
        }
    }

    /// Get or create metrics for an adapter
    pub fn metrics_for(&mut self, adapter_name: &str) -> &mut AdapterMetrics {
        self.adapters
            .entry(adapter_name.to_string())
            .or_insert_with(AdapterMetrics::new)
    }

    /// Get metrics for an adapter (read-only)
    pub fn get_metrics(&self, adapter_name: &str) -> Option<&AdapterMetrics> {
        self.adapters.get(adapter_name)
    }

    /// Get all adapter names
    pub fn adapter_names(&self) -> Vec<&str> {
        self.adapters.keys().map(|s| s.as_str()).collect()
    }

    /// Get total metrics across all adapters
    pub fn total_metrics(&self) -> AdapterMetrics {
        let mut total = AdapterMetrics::new();

        for metrics in self.adapters.values() {
            total.merge(metrics);
        }

        total
    }

    /// Get adapter with highest error rate
    pub fn highest_error_rate_adapter(&self) -> Option<(&str, f64)> {
        self.adapters
            .iter()
            .map(|(name, metrics)| (name.as_str(), metrics.error_rate()))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .filter(|(_, rate)| *rate > 0.0)
    }

    /// Get adapter with lowest health score
    pub fn lowest_health_adapter(&self) -> Option<(&str, f64)> {
        self.adapters
            .iter()
            .map(|(name, metrics)| (name.as_str(), metrics.health_score()))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .filter(|(_, score)| *score < 1.0)
    }

    /// Reset all adapter metrics
    pub fn reset_all(&mut self) {
        for metrics in self.adapters.values_mut() {
            metrics.reset();
        }
    }

    /// Remove inactive adapters (no recent activity)
    pub fn remove_inactive(&mut self, threshold_seconds: u64) {
        self.adapters.retain(|_, metrics| metrics.has_recent_activity(threshold_seconds));
    }
}

impl Default for AdapterMetricsAggregator {
    fn default() -> Self {
        Self::new()
    }
}
