//! Core integration metrics and collection

/// Integration metrics
#[derive(Debug)]
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
    pub adapter_metrics: std::sync::Mutex<std::collections::HashMap<String, super::adapter::AdapterMetrics>>,
    /// Start time
    pub start_time: std::time::Instant,
    /// Last reset time
    pub last_reset: std::sync::Mutex<Option<std::time::Instant>>,
}

impl Clone for IntegrationMetrics {
    fn clone(&self) -> Self {
        Self {
            events_sent: std::sync::atomic::AtomicU64::new(self.events_sent.load(std::sync::atomic::Ordering::SeqCst)),
            events_received: std::sync::atomic::AtomicU64::new(self.events_received.load(std::sync::atomic::Ordering::SeqCst)),
            events_filtered: std::sync::atomic::AtomicU64::new(self.events_filtered.load(std::sync::atomic::Ordering::SeqCst)),
            events_unrouted: std::sync::atomic::AtomicU64::new(self.events_unrouted.load(std::sync::atomic::Ordering::SeqCst)),
            batches_processed: std::sync::atomic::AtomicU64::new(self.batches_processed.load(std::sync::atomic::Ordering::SeqCst)),
            batches_filtered: std::sync::atomic::AtomicU64::new(self.batches_filtered.load(std::sync::atomic::Ordering::SeqCst)),
            errors_total: std::sync::atomic::AtomicU64::new(self.errors_total.load(std::sync::atomic::Ordering::SeqCst)),
            adapter_metrics: std::sync::Mutex::new(self.adapter_metrics.lock().unwrap().clone()),
            start_time: self.start_time,
            last_reset: std::sync::Mutex::new(*self.last_reset.lock().unwrap()),
        }
    }
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

    /// Record an event sent
    pub fn record_event_sent(&self) {
        self.events_sent.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    /// Record an event received
    pub fn record_event_received(&self) {
        self.events_received.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    /// Record an event filtered
    pub fn record_event_filtered(&self) {
        self.events_filtered.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    /// Record an event unrouted
    pub fn record_event_unrouted(&self) {
        self.events_unrouted.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    /// Record a batch processed
    pub fn record_batch_processed(&self) {
        self.batches_processed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    /// Record a batch filtered
    pub fn record_batch_filtered(&self) {
        self.batches_filtered.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    /// Record an error
    pub fn record_error(&self) {
        self.errors_total.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    /// Get adapter metrics (create if doesn't exist)
    pub fn adapter_metrics(&self, adapter_name: &str) -> super::adapter::AdapterMetrics {
        let mut metrics = self.adapter_metrics.lock().unwrap();
        metrics.entry(adapter_name.to_string()).or_insert_with(super::adapter::AdapterMetrics::new).clone()
    }

    /// Update adapter metrics
    pub fn update_adapter_metrics<F>(&self, adapter_name: &str, updater: F)
    where
        F: FnOnce(&mut super::adapter::AdapterMetrics),
    {
        let mut metrics = self.adapter_metrics.lock().unwrap();
        let adapter_metrics = metrics.entry(adapter_name.to_string()).or_insert_with(super::adapter::AdapterMetrics::new);
        updater(adapter_metrics);
    }

    /// Get current metrics snapshot
    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            events_sent: self.events_sent.load(std::sync::atomic::Ordering::SeqCst),
            events_received: self.events_received.load(std::sync::atomic::Ordering::SeqCst),
            events_filtered: self.events_filtered.load(std::sync::atomic::Ordering::SeqCst),
            events_unrouted: self.events_unrouted.load(std::sync::atomic::Ordering::SeqCst),
            batches_processed: self.batches_processed.load(std::sync::atomic::Ordering::SeqCst),
            batches_filtered: self.batches_filtered.load(std::sync::atomic::Ordering::SeqCst),
            errors_total: self.errors_total.load(std::sync::atomic::Ordering::SeqCst),
            adapter_metrics: self.adapter_metrics.lock().unwrap().clone(),
            uptime_seconds: self.start_time.elapsed().as_secs(),
            last_reset_seconds: self.last_reset.lock().unwrap().map(|t| t.elapsed().as_secs()),
        }
    }

    /// Reset all metrics
    pub fn reset(&self) {
        self.events_sent.store(0, std::sync::atomic::Ordering::SeqCst);
        self.events_received.store(0, std::sync::atomic::Ordering::SeqCst);
        self.events_filtered.store(0, std::sync::atomic::Ordering::SeqCst);
        self.events_unrouted.store(0, std::sync::atomic::Ordering::SeqCst);
        self.batches_processed.store(0, std::sync::atomic::Ordering::SeqCst);
        self.batches_filtered.store(0, std::sync::atomic::Ordering::SeqCst);
        self.errors_total.store(0, std::sync::atomic::Ordering::SeqCst);
        *self.adapter_metrics.lock().unwrap() = std::collections::HashMap::new();
        *self.last_reset.lock().unwrap() = Some(std::time::Instant::now());
    }

    /// Get uptime
    pub fn uptime(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }

    /// Get time since last reset
    pub fn time_since_reset(&self) -> Option<std::time::Duration> {
        self.last_reset.lock().unwrap().map(|t| t.elapsed())
    }

    /// Calculate event processing rate (events per second)
    pub fn event_processing_rate(&self) -> f64 {
        let uptime_secs = self.uptime().as_secs_f64();
        if uptime_secs > 0.0 {
            let total_events = self.events_sent.load(std::sync::atomic::Ordering::SeqCst) +
                              self.events_received.load(std::sync::atomic::Ordering::SeqCst);
            total_events as f64 / uptime_secs
        } else {
            0.0
        }
    }

    /// Calculate error rate (errors per total events)
    pub fn error_rate(&self) -> f64 {
        let total_events = self.events_sent.load(std::sync::atomic::Ordering::SeqCst) +
                          self.events_received.load(std::sync::atomic::Ordering::SeqCst);
        if total_events > 0 {
            let errors = self.errors_total.load(std::sync::atomic::Ordering::SeqCst);
            errors as f64 / total_events as f64
        } else {
            0.0
        }
    }

    /// Calculate filtering rate (filtered events per total events)
    pub fn filtering_rate(&self) -> f64 {
        let total_events = self.events_received.load(std::sync::atomic::Ordering::SeqCst);
        if total_events > 0 {
            let filtered = self.events_filtered.load(std::sync::atomic::Ordering::SeqCst);
            filtered as f64 / total_events as f64
        } else {
            0.0
        }
    }

    /// Get metrics summary
    pub fn summary(&self) -> String {
        let snapshot = self.snapshot();
        format!(
            "IntegrationMetrics {{ sent: {}, received: {}, filtered: {}, errors: {}, rate: {:.1} evt/s, error_rate: {:.3} }}",
            snapshot.events_sent,
            snapshot.events_received,
            snapshot.events_filtered,
            snapshot.errors_total,
            self.event_processing_rate(),
            self.error_rate()
        )
    }
}

impl std::fmt::Display for IntegrationMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

impl Default for IntegrationMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Metrics snapshot for thread-safe access
#[derive(Debug, Clone)]
pub struct MetricsSnapshot {
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
    /// Adapter-specific metrics
    pub adapter_metrics: std::collections::HashMap<String, super::adapter::AdapterMetrics>,
    /// Uptime in seconds
    pub uptime_seconds: u64,
    /// Time since last reset in seconds
    pub last_reset_seconds: Option<u64>,
}

impl MetricsSnapshot {
    /// Calculate derived metrics
    pub fn event_processing_rate(&self) -> f64 {
        if self.uptime_seconds > 0 {
            let total_events = self.events_sent + self.events_received;
            total_events as f64 / self.uptime_seconds as f64
        } else {
            0.0
        }
    }

    /// Calculate error rate
    pub fn error_rate(&self) -> f64 {
        let total_events = self.events_sent + self.events_received;
        if total_events > 0 {
            self.errors_total as f64 / total_events as f64
        } else {
            0.0
        }
    }

    /// Calculate filtering efficiency
    pub fn filtering_efficiency(&self) -> f64 {
        if self.events_received > 0 {
            self.events_filtered as f64 / self.events_received as f64
        } else {
            0.0
        }
    }

    /// Get summary
    pub fn summary(&self) -> String {
        format!(
            "Snapshot {{ sent: {}, received: {}, filtered: {}, errors: {}, rate: {:.1} evt/s }}",
            self.events_sent,
            self.events_received,
            self.events_filtered,
            self.errors_total,
            self.event_processing_rate()
        )
    }
}

impl std::fmt::Display for MetricsSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}
