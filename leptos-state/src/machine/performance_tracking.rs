//! Performance tracking for state machine tests

use super::*;
use std::time::{Duration, Instant};

/// Performance tracker for tests
pub struct PerformanceTracker {
    /// Start time of the test
    pub start_time: Option<Instant>,
    /// End time of the test
    pub end_time: Option<Instant>,
    /// Transition times
    pub transition_times: Vec<Duration>,
    /// Memory usage samples
    pub memory_samples: Vec<usize>,
    /// Allocation counts
    pub allocation_counts: Vec<usize>,
}

impl PerformanceTracker {
    /// Create a new performance tracker
    pub fn new() -> Self {
        Self {
            start_time: None,
            end_time: None,
            transition_times: Vec::new(),
            memory_samples: Vec::new(),
            allocation_counts: Vec::new(),
        }
    }

    /// Start tracking performance
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    /// Stop tracking performance
    pub fn stop(&mut self) {
        self.end_time = Some(Instant::now());
    }

    /// Record a transition time
    pub fn record_transition_time(&mut self, duration: Duration) {
        self.transition_times.push(duration);
    }

    /// Record memory usage
    pub fn record_memory_usage(&mut self, usage: usize) {
        self.memory_samples.push(usage);
    }

    /// Record allocation count
    pub fn record_allocation_count(&mut self, count: usize) {
        self.allocation_counts.push(count);
    }

    /// Get total execution time
    pub fn total_execution_time(&self) -> Option<Duration> {
        match (self.start_time, self.end_time) {
            (Some(start), Some(end)) => Some(end.duration_since(start)),
            _ => None,
        }
    }

    /// Get average transition time
    pub fn average_transition_time(&self) -> Duration {
        if self.transition_times.is_empty() {
            Duration::from_nanos(0)
        } else {
            let total: Duration = self.transition_times.iter().sum();
            total / self.transition_times.len() as u32
        }
    }

    /// Get maximum transition time
    pub fn max_transition_time(&self) -> Duration {
        self.transition_times.iter()
            .max()
            .copied()
            .unwrap_or(Duration::from_nanos(0))
    }

    /// Get minimum transition time
    pub fn min_transition_time(&self) -> Duration {
        self.transition_times.iter()
            .min()
            .copied()
            .unwrap_or(Duration::from_nanos(0))
    }

    /// Get average memory usage
    pub fn average_memory_usage(&self) -> usize {
        if self.memory_samples.is_empty() {
            0
        } else {
            self.memory_samples.iter().sum::<usize>() / self.memory_samples.len()
        }
    }

    /// Get maximum memory usage
    pub fn max_memory_usage(&self) -> usize {
        self.memory_samples.iter()
            .max()
            .copied()
            .unwrap_or(0)
    }

    /// Get minimum memory usage
    pub fn min_memory_usage(&self) -> usize {
        self.memory_samples.iter()
            .min()
            .copied()
            .unwrap_or(0)
    }

    /// Get average allocation count
    pub fn average_allocation_count(&self) -> usize {
        if self.allocation_counts.is_empty() {
            0
        } else {
            self.allocation_counts.iter().sum::<usize>() / self.allocation_counts.len()
        }
    }

    /// Get maximum allocation count
    pub fn max_allocation_count(&self) -> usize {
        self.allocation_counts.iter()
            .max()
            .copied()
            .unwrap_or(0)
    }

    /// Get minimum allocation count
    pub fn min_allocation_count(&self) -> usize {
        self.allocation_counts.iter()
            .min()
            .copied()
            .unwrap_or(0)
    }

    /// Get performance metrics
    pub fn get_metrics(&self) -> PerformanceMetrics {
        PerformanceMetrics {
            avg_transition_time: self.average_transition_time(),
            max_transition_time: self.max_transition_time(),
            memory_usage: self.average_memory_usage(),
            allocations: self.average_allocation_count(),
        }
    }

    /// Generate performance report
    pub fn generate_report(&self) -> PerformanceReport {
        PerformanceReport {
            total_execution_time: self.total_execution_time(),
            transition_count: self.transition_times.len(),
            average_transition_time: self.average_transition_time(),
            max_transition_time: self.max_transition_time(),
            min_transition_time: self.min_transition_time(),
            average_memory_usage: self.average_memory_usage(),
            max_memory_usage: self.max_memory_usage(),
            min_memory_usage: self.min_memory_usage(),
            average_allocation_count: self.average_allocation_count(),
            max_allocation_count: self.max_allocation_count(),
            min_allocation_count: self.min_allocation_count(),
        }
    }
}

/// Performance report
#[derive(Debug, Clone, PartialEq)]
pub struct PerformanceReport {
    /// Total execution time
    pub total_execution_time: Option<Duration>,
    /// Number of transitions
    pub transition_count: usize,
    /// Average transition time
    pub average_transition_time: Duration,
    /// Maximum transition time
    pub max_transition_time: Duration,
    /// Minimum transition time
    pub min_transition_time: Duration,
    /// Average memory usage
    pub average_memory_usage: usize,
    /// Maximum memory usage
    pub max_memory_usage: usize,
    /// Minimum memory usage
    pub min_memory_usage: usize,
    /// Average allocation count
    pub average_allocation_count: usize,
    /// Maximum allocation count
    pub max_allocation_count: usize,
    /// Minimum allocation count
    pub min_allocation_count: usize,
}

impl PerformanceReport {
    /// Check if performance meets requirements
    pub fn meets_requirements(&self, max_transition_time: Duration, max_memory_usage: usize) -> bool {
        self.max_transition_time <= max_transition_time && self.max_memory_usage <= max_memory_usage
    }

    /// Get performance summary
    pub fn summary(&self) -> String {
        format!(
            "Performance Summary:\n\
            Total Time: {:?}\n\
            Transitions: {}\n\
            Avg Transition Time: {:?}\n\
            Max Transition Time: {:?}\n\
            Min Transition Time: {:?}\n\
            Avg Memory Usage: {} bytes\n\
            Max Memory Usage: {} bytes\n\
            Min Memory Usage: {} bytes\n\
            Avg Allocations: {}\n\
            Max Allocations: {}\n\
            Min Allocations: {}",
            self.total_execution_time.unwrap_or(Duration::from_nanos(0)),
            self.transition_count,
            self.average_transition_time,
            self.max_transition_time,
            self.min_transition_time,
            self.average_memory_usage,
            self.max_memory_usage,
            self.min_memory_usage,
            self.average_allocation_count,
            self.max_allocation_count,
            self.min_allocation_count
        )
    }
}
