//! Performance metrics, bottlenecks, and optimization suggestions

use super::*;
use std::time::Duration;

/// Performance metrics for state machines
#[derive(Debug, Clone, PartialEq)]
pub struct PerformanceMetrics {
    /// Average transition time
    pub avg_transition_time: Duration,
    /// Maximum transition time
    pub max_transition_time: Duration,
    /// Minimum transition time
    pub min_transition_time: Duration,
    /// Total number of transitions
    pub total_transitions: usize,
    /// Cache hit rate (0.0 to 1.0)
    pub cache_hit_rate: f64,
    /// Memory usage in bytes
    pub memory_usage: usize,
    /// Peak memory usage
    pub peak_memory_usage: usize,
    /// Number of allocations
    pub allocations: usize,
    /// Number of deallocations
    pub deallocations: usize,
    /// CPU time spent in transitions
    pub cpu_time: Duration,
    /// I/O time spent (if applicable)
    pub io_time: Duration,
    /// Number of concurrent operations
    pub concurrent_operations: usize,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            avg_transition_time: Duration::from_nanos(0),
            max_transition_time: Duration::from_nanos(0),
            min_transition_time: Duration::from_nanos(0),
            total_transitions: 0,
            cache_hit_rate: 0.0,
            memory_usage: 0,
            peak_memory_usage: 0,
            allocations: 0,
            deallocations: 0,
            cpu_time: Duration::from_nanos(0),
            io_time: Duration::from_nanos(0),
            concurrent_operations: 0,
        }
    }
}

impl PerformanceMetrics {
    /// Create new performance metrics
    pub fn new() -> Self {
        Self::default()
    }

    /// Update metrics with a new transition time
    pub fn record_transition(&mut self, duration: Duration) {
        self.total_transitions += 1;
        self.cpu_time += duration;

        if self.total_transitions == 1 {
            self.avg_transition_time = duration;
            self.max_transition_time = duration;
            self.min_transition_time = duration;
        } else {
            self.avg_transition_time = Duration::from_nanos(
                ((self.avg_transition_time.as_nanos() * (self.total_transitions - 1) as u128)
                    + duration.as_nanos())
                    / self.total_transitions as u128,
            );

            if duration > self.max_transition_time {
                self.max_transition_time = duration;
            }

            if duration < self.min_transition_time {
                self.min_transition_time = duration;
            }
        }
    }

    /// Record memory usage
    pub fn record_memory_usage(&mut self, usage: usize) {
        self.memory_usage = usage;
        if usage > self.peak_memory_usage {
            self.peak_memory_usage = usage;
        }
    }

    /// Record allocation
    pub fn record_allocation(&mut self) {
        self.allocations += 1;
    }

    /// Record deallocation
    pub fn record_deallocation(&mut self) {
        self.deallocations += 1;
    }

    /// Record cache hit
    pub fn record_cache_hit(&mut self) {
        // This would need to be tracked separately
    }

    /// Record cache miss
    pub fn record_cache_miss(&mut self) {
        // This would need to be tracked separately
    }

    /// Get performance summary
    pub fn summary(&self) -> String {
        format!(
            "Performance Metrics:\n\
            Transitions: {}\n\
            Avg Time: {:?}\n\
            Max Time: {:?}\n\
            Min Time: {:?}\n\
            Memory: {} bytes (peak: {})\n\
            Allocations: {} / Deallocations: {}\n\
            Cache Hit Rate: {:.1}%",
            self.total_transitions,
            self.avg_transition_time,
            self.max_transition_time,
            self.min_transition_time,
            self.memory_usage,
            self.peak_memory_usage,
            self.allocations,
            self.deallocations,
            self.cache_hit_rate * 100.0
        )
    }
}

/// Performance bottleneck information
#[derive(Debug, Clone, PartialEq)]
pub struct PerformanceBottleneck {
    /// Type of bottleneck
    pub bottleneck_type: BottleneckType,
    /// Severity level (0.0 to 1.0)
    pub severity: f64,
    /// Description of the bottleneck
    pub description: String,
    /// Affected component
    pub affected_component: String,
    /// Suggested actions to resolve
    pub suggested_actions: Vec<String>,
    /// Estimated improvement if fixed
    pub estimated_improvement: f64,
}

impl PerformanceBottleneck {
    /// Create a new performance bottleneck
    pub fn new(
        bottleneck_type: BottleneckType,
        severity: f64,
        description: String,
        affected_component: String,
    ) -> Self {
        Self {
            bottleneck_type,
            severity,
            description,
            affected_component,
            suggested_actions: Vec::new(),
            estimated_improvement: 0.0,
        }
    }

    /// Add a suggested action
    pub fn add_suggestion(&mut self, action: String, improvement: f64) {
        self.suggested_actions.push(action);
        self.estimated_improvement = self.estimated_improvement.max(improvement);
    }
}

/// Types of performance bottlenecks
#[derive(Debug, Clone, PartialEq)]
pub enum BottleneckType {
    /// Slow transition execution
    SlowTransition,
    /// High memory usage
    MemoryUsage,
    /// Cache inefficiency
    CacheInefficiency,
    /// I/O bound operations
    IoBound,
    /// CPU bound operations
    CpuBound,
    /// Lock contention
    LockContention,
    /// Excessive allocations
    ExcessiveAllocations,
    /// State machine complexity
    Complexity,
}

impl BottleneckType {
    /// Get a description of the bottleneck type
    pub fn description(&self) -> &str {
        match self {
            BottleneckType::SlowTransition => "Slow transition execution",
            BottleneckType::MemoryUsage => "High memory usage",
            BottleneckType::CacheInefficiency => "Cache inefficiency",
            BottleneckType::IoBound => "I/O bound operations",
            BottleneckType::CpuBound => "CPU bound operations",
            BottleneckType::LockContention => "Lock contention",
            BottleneckType::ExcessiveAllocations => "Excessive allocations",
            BottleneckType::Complexity => "State machine complexity",
        }
    }
}

/// Optimization suggestion
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationSuggestion {
    /// Type of optimization
    pub optimization_type: String,
    /// Description of the optimization
    pub description: String,
    /// Expected performance improvement (0.0 to 1.0)
    pub expected_improvement: f64,
    /// Difficulty level (0.0 = easy, 1.0 = hard)
    pub difficulty: f64,
    /// Estimated implementation time
    pub estimated_time: Duration,
    /// Prerequisites for implementing
    pub prerequisites: Vec<String>,
}

impl OptimizationSuggestion {
    /// Create a new optimization suggestion
    pub fn new(
        optimization_type: String,
        description: String,
        expected_improvement: f64,
        difficulty: f64,
    ) -> Self {
        Self {
            optimization_type,
            description,
            expected_improvement,
            difficulty,
            estimated_time: Duration::from_hours(1),
            prerequisites: Vec::new(),
        }
    }

    /// Set estimated implementation time
    pub fn with_time(mut self, time: Duration) -> Self {
        self.estimated_time = time;
        self
    }

    /// Add a prerequisite
    pub fn add_prerequisite(mut self, prereq: String) -> Self {
        self.prerequisites.push(prereq);
        self
    }
}

/// Performance analysis result
#[derive(Debug, Clone, PartialEq)]
pub struct PerformanceAnalysis {
    /// Current performance metrics
    pub metrics: PerformanceMetrics,
    /// Identified bottlenecks
    pub bottlenecks: Vec<PerformanceBottleneck>,
    /// Optimization suggestions
    pub suggestions: Vec<OptimizationSuggestion>,
    /// Overall performance score (0.0 to 1.0)
    pub performance_score: f64,
    /// Analysis timestamp
    pub timestamp: std::time::Instant,
}

impl PerformanceAnalysis {
    /// Create a new performance analysis
    pub fn new(metrics: PerformanceMetrics) -> Self {
        Self {
            metrics,
            bottlenecks: Vec::new(),
            suggestions: Vec::new(),
            performance_score: 0.0,
            timestamp: std::time::Instant::now(),
        }
    }

    /// Add a bottleneck
    pub fn add_bottleneck(&mut self, bottleneck: PerformanceBottleneck) {
        self.bottlenecks.push(bottleneck);
        self.update_score();
    }

    /// Add an optimization suggestion
    pub fn add_suggestion(&mut self, suggestion: OptimizationSuggestion) {
        self.suggestions.push(suggestion);
    }

    /// Update the performance score based on bottlenecks
    fn update_score(&mut self) {
        let bottleneck_penalty: f64 = self.bottlenecks.iter().map(|b| b.severity).sum();

        self.performance_score = (1.0 - bottleneck_penalty).max(0.0);
    }

    /// Get the most critical bottleneck
    pub fn critical_bottleneck(&self) -> Option<&PerformanceBottleneck> {
        self.bottlenecks
            .iter()
            .max_by(|a, b| a.severity.partial_cmp(&b.severity).unwrap())
    }

    /// Get summary report
    pub fn summary(&self) -> String {
        format!(
            "Performance Analysis:\n\
            Score: {:.1}%\n\
            Bottlenecks: {}\n\
            Suggestions: {}\n\
            Critical Issue: {}",
            self.performance_score * 100.0,
            self.bottlenecks.len(),
            self.suggestions.len(),
            self.critical_bottleneck()
                .map(|b| b.description.clone())
                .unwrap_or_else(|| "None".to_string())
        )
    }
}
