//! Performance-optimized selectors with statistics

use crate::store::Store;
use std::time::Instant;

/// Performance-optimized selector with statistics
pub struct PerformanceSelector<T: Store, O> {
    /// The selector function
    selector: Box<dyn Fn(&T::State) -> O + Send + Sync>,
    /// Cache for the last computed value
    cache: std::sync::Mutex<Option<(T::State, O)>>,
    /// Performance statistics
    stats: std::sync::Mutex<SelectorStats>,
}

impl<T: Store, O: Clone + PartialEq + 'static> PerformanceSelector<T, O> {
    /// Create a new performance selector
    pub fn new<F>(selector: F) -> Self
    where
        F: Fn(&T::State) -> O + Send + Sync + 'static,
    {
        Self {
            selector: Box::new(selector),
            cache: std::sync::Mutex::new(None),
            stats: std::sync::Mutex::new(SelectorStats::new()),
        }
    }

    /// Get the selected value with performance tracking
    pub fn select(&self, state: &T::State) -> O {
        let start_time = Instant::now();
        let mut stats = self.stats.lock().unwrap();

        stats.total_calls += 1;

        let mut cache = self.cache.lock().unwrap();

        if let Some((ref cached_state, ref cached_value)) = *cache {
            if cached_state == state {
                stats.cache_hits += 1;
                let duration = start_time.elapsed();
                stats.total_time += duration;
                return cached_value.clone();
            }
        }

        // Cache miss - compute value
        stats.cache_misses += 1;
        let compute_start = Instant::now();
        let value = (self.selector)(state);
        let compute_time = compute_start.elapsed();

        stats.computation_time += compute_time;
        stats.total_time += start_time.elapsed();

        *cache = Some((state.clone(), value.clone()));
        value
    }

    /// Clear the memoization cache
    pub fn clear_cache(&self) {
        *self.cache.lock().unwrap() = None;
    }

    /// Get performance statistics
    pub fn stats(&self) -> SelectorStats {
        self.stats.lock().unwrap().clone()
    }

    /// Reset statistics
    pub fn reset_stats(&self) {
        *self.stats.lock().unwrap() = SelectorStats::new();
    }

    /// Get cache hit rate
    pub fn cache_hit_rate(&self) -> f64 {
        let stats = self.stats.lock().unwrap();
        if stats.total_calls == 0 {
            0.0
        } else {
            (stats.cache_hits as f64 / stats.total_calls as f64) * 100.0
        }
    }

    /// Get average computation time
    pub fn avg_computation_time(&self) -> std::time::Duration {
        let stats = self.stats.lock().unwrap();
        if stats.cache_misses == 0 {
            std::time::Duration::from_secs(0)
        } else {
            stats.computation_time / stats.cache_misses as u32
        }
    }

    /// Get average total time
    pub fn avg_total_time(&self) -> std::time::Duration {
        let stats = self.stats.lock().unwrap();
        if stats.total_calls == 0 {
            std::time::Duration::from_secs(0)
        } else {
            stats.total_time / stats.total_calls as u32
        }
    }

    /// Check if performance is optimal
    pub fn is_performance_optimal(&self) -> bool {
        let hit_rate = self.cache_hit_rate();
        let avg_time = self.avg_total_time();

        // Consider optimal if hit rate > 80% and avg time < 1ms
        hit_rate > 80.0 && avg_time.as_millis() < 1
    }
}

impl<T: Store, O: Clone + PartialEq + 'static> std::fmt::Debug for PerformanceSelector<T, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PerformanceSelector")
            .field("stats", &self.stats())
            .finish()
    }
}

impl<T: Store, O: Clone + PartialEq + 'static> std::fmt::Display for PerformanceSelector<T, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hit_rate = self.cache_hit_rate();
        write!(f, "PerformanceSelector(hit_rate: {:.1}%)", hit_rate)
    }
}

/// Selector statistics
#[derive(Debug, Clone, Default)]
pub struct SelectorStats {
    /// Total number of calls
    pub total_calls: u64,
    /// Number of cache hits
    pub cache_hits: u64,
    /// Number of cache misses
    pub cache_misses: u64,
    /// Total computation time
    pub computation_time: std::time::Duration,
    /// Total time (including cache checks)
    pub total_time: std::time::Duration,
}

impl SelectorStats {
    /// Create new statistics
    pub fn new() -> Self {
        Self::default()
    }

    /// Get cache hit rate as percentage
    pub fn cache_hit_rate(&self) -> f64 {
        if self.total_calls == 0 {
            0.0
        } else {
            (self.cache_hits as f64 / self.total_calls as f64) * 100.0
        }
    }

    /// Get average computation time
    pub fn avg_computation_time(&self) -> std::time::Duration {
        if self.cache_misses == 0 {
            std::time::Duration::from_secs(0)
        } else {
            self.computation_time / self.cache_misses as u32
        }
    }

    /// Get average total time
    pub fn avg_total_time(&self) -> std::time::Duration {
        if self.total_calls == 0 {
            std::time::Duration::from_secs(0)
        } else {
            self.total_time / self.total_calls as u32
        }
    }

    /// Get performance score (higher is better)
    pub fn performance_score(&self) -> f64 {
        let hit_rate = self.cache_hit_rate();
        let avg_time_ms = self.avg_total_time().as_secs_f64() * 1000.0;

        // Score based on hit rate and speed
        // Max score: 100 (100% hit rate + < 0.1ms avg time)
        // Min score: 0
        let hit_rate_score = hit_rate;
        let speed_score = if avg_time_ms < 0.1 {
            100.0
        } else if avg_time_ms < 1.0 {
            75.0
        } else if avg_time_ms < 10.0 {
            50.0
        } else {
            25.0
        };

        (hit_rate_score + speed_score) / 2.0
    }

    /// Check if statistics indicate good performance
    pub fn is_performing_well(&self) -> bool {
        self.performance_score() > 75.0
    }

    /// Generate performance report
    pub fn performance_report(&self) -> String {
        format!(
            "Selector Performance Report:\n\
             Total Calls: {}\n\
             Cache Hits: {} ({:.1}%)\n\
             Cache Misses: {}\n\
             Avg Computation Time: {:.2}ms\n\
             Avg Total Time: {:.2}ms\n\
             Performance Score: {:.1}/100",
            self.total_calls,
            self.cache_hits,
            self.cache_hit_rate(),
            self.cache_misses,
            self.avg_computation_time().as_secs_f64() * 1000.0,
            self.avg_total_time().as_secs_f64() * 1000.0,
            self.performance_score()
        )
    }
}

impl std::fmt::Display for SelectorStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "calls: {}, hits: {} ({:.1}%), avg_time: {:.2}ms",
            self.total_calls,
            self.cache_hits,
            self.cache_hit_rate(),
            self.avg_total_time().as_secs_f64() * 1000.0
        )
    }
}

impl std::ops::Add for SelectorStats {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            total_calls: self.total_calls + rhs.total_calls,
            cache_hits: self.cache_hits + rhs.cache_hits,
            cache_misses: self.cache_misses + rhs.cache_misses,
            computation_time: self.computation_time + rhs.computation_time,
            total_time: self.total_time + rhs.total_time,
        }
    }
}

/// Performance monitor for tracking selector performance
pub struct SelectorPerformanceMonitor {
    /// Statistics by selector name
    stats: std::sync::Mutex<std::collections::HashMap<String, SelectorStats>>,
}

impl SelectorPerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        Self {
            stats: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }

    /// Record statistics for a selector
    pub fn record_stats(&self, selector_name: String, stats: SelectorStats) {
        self.stats.lock().unwrap().insert(selector_name, stats);
    }

    /// Get statistics for a selector
    pub fn get_stats(&self, selector_name: &str) -> Option<SelectorStats> {
        self.stats.lock().unwrap().get(selector_name).cloned()
    }

    /// Get all selector names
    pub fn selector_names(&self) -> Vec<String> {
        self.stats.lock().unwrap().keys().cloned().collect()
    }

    /// Get overall performance statistics
    pub fn overall_stats(&self) -> SelectorStats {
        let stats = self.stats.lock().unwrap();
        let mut total = SelectorStats::new();

        for stat in stats.values() {
            total = total + stat.clone();
        }

        total
    }

    /// Generate comprehensive performance report
    pub fn comprehensive_report(&self) -> String {
        let stats = self.stats.lock().unwrap();
        let mut report = format!("Selector Performance Monitor Report\n");
        report.push_str(&format!("={}\n", "=".repeat(40)));
        report.push_str(&format!("Total Selectors: {}\n\n", stats.len()));

        let overall = self.overall_stats();
        report.push_str("Overall Statistics:\n");
        report.push_str(&overall.performance_report());
        report.push_str("\n\n");

        if !stats.is_empty() {
            report.push_str("Per-Selector Statistics:\n");
            for (name, stat) in stats.iter() {
                report.push_str(&format!("{}: {}\n", name, stat));
            }
        }

        report
    }

    /// Clear all statistics
    pub fn clear_all(&self) {
        self.stats.lock().unwrap().clear();
    }
}

impl Default for SelectorPerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}
