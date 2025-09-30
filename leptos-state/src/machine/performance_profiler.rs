//! Performance profiler and monitoring

use super::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Performance profiler for state machines
#[derive(Debug)]
pub struct PerformanceProfiler {
    /// Start time of profiling
    pub start_time: Option<Instant>,
    /// End time of profiling
    pub end_time: Option<Instant>,
    /// Transition timing data
    pub transition_times: HashMap<String, Vec<Duration>>,
    /// State visit counts
    pub state_visits: HashMap<String, usize>,
    /// Event trigger counts
    pub event_triggers: HashMap<String, usize>,
    /// Memory usage samples
    pub memory_samples: Vec<(Instant, usize)>,
    /// Performance bottlenecks detected
    pub bottlenecks: Vec<PerformanceBottleneck>,
    /// Optimization suggestions
    pub suggestions: Vec<OptimizationSuggestion>,
}

impl PerformanceProfiler {
    /// Create a new performance profiler
    pub fn new() -> Self {
        Self {
            start_time: None,
            end_time: None,
            transition_times: HashMap::new(),
            state_visits: HashMap::new(),
            event_triggers: HashMap::new(),
            memory_samples: Vec::new(),
            bottlenecks: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Start profiling
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    /// Stop profiling
    pub fn stop(&mut self) {
        self.end_time = Some(Instant::now());
    }

    /// Record a transition timing
    pub fn record_transition(&mut self, from_state: &str, to_state: &str, duration: Duration) {
        let key = format!("{} -> {}", from_state, to_state);
        self.transition_times
            .entry(key)
            .or_insert_with(Vec::new)
            .push(duration);

        // Record state visits
        *self.state_visits.entry(from_state.to_string()).or_insert(0) += 1;
        *self.state_visits.entry(to_state.to_string()).or_insert(0) += 1;
    }

    /// Record an event trigger
    pub fn record_event(&mut self, event: &str) {
        *self.event_triggers.entry(event.to_string()).or_insert(0) += 1;
    }

    /// Record memory usage
    pub fn record_memory(&mut self, memory_usage: usize) {
        self.memory_samples.push((Instant::now(), memory_usage));
    }

    /// Analyze performance and generate bottlenecks
    pub fn analyze(&mut self) -> PerformanceAnalysis {
        let mut analysis = PerformanceAnalysis::new(self.get_metrics());

        // Analyze transition times for bottlenecks
        self.analyze_transition_times(&mut analysis);

        // Analyze memory usage for bottlenecks
        self.analyze_memory_usage(&mut analysis);

        // Analyze state distribution for bottlenecks
        self.analyze_state_distribution(&mut analysis);

        // Generate optimization suggestions
        self.generate_suggestions(&mut analysis);

        analysis
    }

    /// Get current performance metrics
    pub fn get_metrics(&self) -> PerformanceMetrics {
        let mut metrics = PerformanceMetrics::default();

        // Calculate average transition time
        let total_transitions: usize = self
            .transition_times
            .values()
            .map(|times| times.len())
            .sum();

        if total_transitions > 0 {
            let total_time: Duration = self.transition_times.values().flatten().sum();

            metrics.total_transitions = total_transitions;
            metrics.avg_transition_time = total_time / total_transitions as u32;

            metrics.max_transition_time = self
                .transition_times
                .values()
                .flatten()
                .max()
                .copied()
                .unwrap_or(Duration::from_nanos(0));

            metrics.min_transition_time = self
                .transition_times
                .values()
                .flatten()
                .min()
                .copied()
                .unwrap_or(Duration::from_nanos(0));
        }

        // Calculate memory usage
        if let Some((_, memory)) = self.memory_samples.last() {
            metrics.memory_usage = *memory;
        }

        if let Some((_, peak_memory)) = self.memory_samples.iter().max_by_key(|(_, mem)| *mem) {
            metrics.peak_memory_usage = *peak_memory;
        }

        metrics
    }

    /// Analyze transition times for bottlenecks
    fn analyze_transition_times(&mut self, analysis: &mut PerformanceAnalysis) {
        let avg_time = analysis.metrics.avg_transition_time;
        let max_time = analysis.metrics.max_transition_time;

        // Check for slow transitions
        if avg_time > Duration::from_millis(100) {
            let mut bottleneck = PerformanceBottleneck::new(
                BottleneckType::SlowTransition,
                (avg_time.as_millis() as f64 / 1000.0).min(1.0),
                format!("Average transition time is high: {:?}", avg_time),
                "Transition execution".to_string(),
            );

            bottleneck.add_suggestion(
                "Consider optimizing transition logic or adding caching".to_string(),
                0.3,
            );

            if max_time > Duration::from_secs(1) {
                bottleneck.add_suggestion(
                    "Investigate the slowest transition: {:?}",
                    max_time,
                    0.5,
                );
            }

            analysis.add_bottleneck(bottleneck);
        }
    }

    /// Analyze memory usage for bottlenecks
    fn analyze_memory_usage(&mut self, analysis: &mut PerformanceAnalysis) {
        let memory_usage = analysis.metrics.memory_usage;
        let peak_memory = analysis.metrics.peak_memory_usage;

        if peak_memory > 50 * 1024 * 1024 {
            // 50MB
            let severity = (peak_memory as f64 / (100 * 1024 * 1024) as f64).min(1.0);

            let mut bottleneck = PerformanceBottleneck::new(
                BottleneckType::MemoryUsage,
                severity,
                format!("High memory usage detected: {} bytes", peak_memory),
                "Memory management".to_string(),
            );

            bottleneck.add_suggestion(
                "Consider reducing cache size or implementing memory limits".to_string(),
                0.4,
            );

            analysis.add_bottleneck(bottleneck);
        }
    }

    /// Analyze state distribution for bottlenecks
    fn analyze_state_distribution(&mut self, analysis: &mut PerformanceAnalysis) {
        let total_visits: usize = self.state_visits.values().sum();
        let max_visits = self.state_visits.values().max().copied().unwrap_or(0);

        if total_visits > 0 {
            let concentration = max_visits as f64 / total_visits as f64;

            if concentration > 0.8 {
                let bottleneck = PerformanceBottleneck::new(
                    BottleneckType::Complexity,
                    concentration - 0.8,
                    format!("State machine execution is concentrated in few states ({:.1}% in one state)", concentration * 100.0),
                    "State distribution".to_string(),
                );

                analysis.add_bottleneck(bottleneck);
            }
        }
    }

    /// Generate optimization suggestions
    fn generate_suggestions(&mut self, analysis: &mut PerformanceAnalysis) {
        // Cache optimization suggestion
        if analysis.metrics.cache_hit_rate < 0.5 {
            analysis.add_suggestion(
                OptimizationSuggestion::new(
                    "Improve caching".to_string(),
                    "Increase cache hit rate by optimizing cache key strategy".to_string(),
                    0.2,
                    0.3,
                )
                .with_time(Duration::from_hours(4)),
            );
        }

        // Memory optimization suggestion
        if analysis.metrics.peak_memory_usage > 20 * 1024 * 1024 {
            analysis.add_suggestion(
                OptimizationSuggestion::new(
                    "Reduce memory usage".to_string(),
                    "Implement memory limits and optimize data structures".to_string(),
                    0.3,
                    0.4,
                )
                .with_time(Duration::from_hours(8)),
            );
        }

        // Transition optimization suggestion
        if analysis.metrics.avg_transition_time > Duration::from_millis(50) {
            analysis.add_suggestion(
                OptimizationSuggestion::new(
                    "Optimize transitions".to_string(),
                    "Review and optimize transition logic for better performance".to_string(),
                    0.4,
                    0.6,
                )
                .with_time(Duration::from_hours(12)),
            );
        }
    }

    /// Generate a performance report
    pub fn generate_report(&self) -> PerformanceReport {
        PerformanceReport {
            profiling_duration: self
                .start_time
                .and_then(|start| self.end_time.map(|end| end.duration_since(start))),
            total_transitions: self.transition_times.values().map(|v| v.len()).sum(),
            unique_transitions: self.transition_times.len(),
            total_state_visits: self.state_visits.values().sum(),
            unique_states_visited: self.state_visits.len(),
            total_events: self.event_triggers.values().sum(),
            unique_events: self.event_triggers.len(),
            bottlenecks_found: self.bottlenecks.len(),
            suggestions_made: self.suggestions.len(),
        }
    }

    /// Reset profiler data
    pub fn reset(&mut self) {
        self.start_time = None;
        self.end_time = None;
        self.transition_times.clear();
        self.state_visits.clear();
        self.event_triggers.clear();
        self.memory_samples.clear();
        self.bottlenecks.clear();
        self.suggestions.clear();
    }
}

/// Performance report
#[derive(Debug, Clone, PartialEq)]
pub struct PerformanceReport {
    /// Total profiling duration
    pub profiling_duration: Option<Duration>,
    /// Total number of transitions executed
    pub total_transitions: usize,
    /// Number of unique transitions
    pub unique_transitions: usize,
    /// Total state visits
    pub total_state_visits: usize,
    /// Number of unique states visited
    pub unique_states_visited: usize,
    /// Total events triggered
    pub total_events: usize,
    /// Number of unique events
    pub unique_events: usize,
    /// Number of bottlenecks found
    pub bottlenecks_found: usize,
    /// Number of suggestions made
    pub suggestions_made: usize,
}

impl PerformanceReport {
    /// Get a summary of the performance report
    pub fn summary(&self) -> String {
        format!(
            "Performance Report:\n\
            Duration: {:?}\n\
            Transitions: {} total, {} unique\n\
            States: {} visits, {} unique\n\
            Events: {} total, {} unique\n\
            Issues: {} bottlenecks, {} suggestions",
            self.profiling_duration.unwrap_or(Duration::from_nanos(0)),
            self.total_transitions,
            self.unique_transitions,
            self.total_state_visits,
            self.unique_states_visited,
            self.total_events,
            self.unique_events,
            self.bottlenecks_found,
            self.suggestions_made
        )
    }
}
