//! Monitoring statistics and metrics

/// Monitoring statistics
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct MonitoringStats {
    /// Number of state changes
    pub state_changes: u64,
    /// Number of transitions
    pub transitions: u64,
    /// Number of actions executed
    pub actions: u64,
    /// Number of guards evaluated
    pub guards: u64,
    /// Number of errors
    pub errors: u64,
    /// Number of performance events
    pub performance_events: u64,
    /// Total monitoring time
    pub total_time: std::time::Duration,
    /// Average event processing time
    pub avg_processing_time: std::time::Duration,
}

impl MonitoringStats {
    /// Create new statistics
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a processing time
    pub fn record_processing_time(&mut self, duration: std::time::Duration) {
        self.total_time += duration;
        let total_events = self.state_changes + self.transitions + self.actions + self.guards;
        if total_events > 0 {
            self.avg_processing_time = self.total_time / total_events as u32;
        }
    }

    /// Get total events count
    pub fn total_events(&self) -> u64 {
        self.state_changes + self.transitions + self.actions + self.guards
    }

    /// Get events per second rate
    pub fn events_per_second(&self) -> f64 {
        let total_seconds = self.total_time.as_secs_f64();
        if total_seconds > 0.0 {
            self.total_events() as f64 / total_seconds
        } else {
            0.0
        }
    }

    /// Get error rate as percentage
    pub fn error_rate(&self) -> f64 {
        let total = self.total_events();
        if total > 0 {
            (self.errors as f64 / total as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Get performance score (higher is better)
    pub fn performance_score(&self) -> f64 {
        let events_per_sec = self.events_per_second();
        let error_rate = self.error_rate();

        // Base score from throughput
        let throughput_score = if events_per_sec > 1000.0 {
            100.0
        } else if events_per_sec > 100.0 {
            75.0
        } else if events_per_sec > 10.0 {
            50.0
        } else {
            25.0
        };

        // Penalty for errors
        let error_penalty = error_rate * 2.0;

        (throughput_score - error_penalty).max(0.0)
    }

    /// Check if statistics indicate good performance
    pub fn is_performing_well(&self) -> bool {
        self.performance_score() > 60.0
    }

    /// Generate summary report
    pub fn summary(&self) -> String {
        format!(
            "Monitoring: {} events ({} state changes, {} transitions, {} actions, {} guards), {} errors, {:.1} EPS",
            self.total_events(),
            self.state_changes,
            self.transitions,
            self.actions,
            self.guards,
            self.errors,
            self.events_per_second()
        )
    }

    /// Generate detailed report
    pub fn detailed_report(&self) -> String {
        let mut report = format!("Monitoring Statistics Report\n");
        report.push_str(&format!("={}\n", "=".repeat(30)));
        report.push_str(&format!("Total Events: {}\n", self.total_events()));
        report.push_str(&format!("  State Changes: {}\n", self.state_changes));
        report.push_str(&format!("  Transitions: {}\n", self.transitions));
        report.push_str(&format!("  Actions: {}\n", self.actions));
        report.push_str(&format!("  Guards: {}\n", self.guards));
        report.push_str(&format!("Errors: {} ({:.1}%)\n", self.errors, self.error_rate()));
        report.push_str(&format!("Performance Events: {}\n", self.performance_events));
        report.push_str(&format!("Total Time: {:.2}s\n", self.total_time.as_secs_f64()));
        report.push_str(&format!("Avg Processing Time: {:.2}ms\n", self.avg_processing_time.as_millis()));
        report.push_str(&format!("Events/Second: {:.1}\n", self.events_per_second()));
        report.push_str(&format!("Performance Score: {:.1}/100\n", self.performance_score()));
        report
    }

    /// Reset statistics
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Merge with another statistics instance
    pub fn merge(&mut self, other: &MonitoringStats) {
        self.state_changes += other.state_changes;
        self.transitions += other.transitions;
        self.actions += other.actions;
        self.guards += other.guards;
        self.errors += other.errors;
        self.performance_events += other.performance_events;
        self.total_time += other.total_time;

        // Recalculate average
        let total_events = self.total_events();
        if total_events > 0 {
            self.avg_processing_time = self.total_time / total_events as u32;
        }
    }

    /// Export to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Import from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl std::fmt::Display for MonitoringStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

impl std::ops::Add for MonitoringStats {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.merge(&rhs);
        self
    }
}

impl std::ops::AddAssign for MonitoringStats {
    fn add_assign(&mut self, rhs: Self) {
        self.merge(&rhs);
    }
}

/// Statistics aggregator for multiple monitors
pub struct StatisticsAggregator {
    /// Collected statistics from multiple sources
    stats: Vec<MonitoringStats>,
}

impl StatisticsAggregator {
    /// Create a new aggregator
    pub fn new() -> Self {
        Self {
            stats: Vec::new(),
        }
    }

    /// Add statistics
    pub fn add_stats(&mut self, stats: MonitoringStats) {
        self.stats.push(stats);
    }

    /// Get aggregated statistics
    pub fn aggregated(&self) -> MonitoringStats {
        let mut total = MonitoringStats::new();
        for stat in &self.stats {
            total.merge(stat);
        }
        total
    }

    /// Get statistics count
    pub fn count(&self) -> usize {
        self.stats.len()
    }

    /// Clear all statistics
    pub fn clear(&mut self) {
        self.stats.clear();
    }

    /// Get individual statistics
    pub fn individual_stats(&self) -> &[MonitoringStats] {
        &self.stats
    }
}

impl Default for StatisticsAggregator {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for StatisticsAggregator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let aggregated = self.aggregated();
        write!(f, "StatisticsAggregator({} sources): {}", self.count(), aggregated)
    }
}
