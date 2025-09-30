//! Performance monitoring for persistence

use crate::machine::persistence_core::PersistenceError;
use std::time::Instant;

/// Performance statistics for persistence operations
#[derive(Debug, Clone, Default)]
pub struct PersistenceStats {
    /// Total number of save operations
    pub saves_total: u64,
    /// Total number of load operations
    pub loads_total: u64,
    /// Total time spent on saves
    pub save_time_total: std::time::Duration,
    /// Total time spent on loads
    pub load_time_total: std::time::Duration,
    /// Number of save errors
    pub save_errors: u64,
    /// Number of load errors
    pub load_errors: u64,
    /// Average save time
    pub avg_save_time: std::time::Duration,
    /// Average load time
    pub avg_load_time: std::time::Duration,
    /// Last save time
    pub last_save_time: Option<Instant>,
    /// Last load time
    pub last_load_time: Option<Instant>,
    /// Data size statistics
    pub data_size_stats: DataSizeStats,
}

impl PersistenceStats {
    /// Record a save operation
    pub fn record_save(&mut self, duration: std::time::Duration, data_size: usize, success: bool) {
        self.saves_total += 1;
        self.save_time_total += duration;
        self.last_save_time = Some(Instant::now());

        if success {
            self.data_size_stats.record_save_size(data_size);
        } else {
            self.save_errors += 1;
        }

        self.update_averages();
    }

    /// Record a load operation
    pub fn record_load(&mut self, duration: std::time::Duration, data_size: usize, success: bool) {
        self.loads_total += 1;
        self.load_time_total += duration;
        self.last_load_time = Some(Instant::now());

        if success {
            self.data_size_stats.record_load_size(data_size);
        } else {
            self.load_errors += 1;
        }

        self.update_averages();
    }

    /// Update average times
    fn update_averages(&mut self) {
        if self.saves_total > 0 {
            let avg_nanos = self.save_time_total.as_nanos() / self.saves_total as u128;
            self.avg_save_time = std::time::Duration::from_nanos(avg_nanos as u64);
        }

        if self.loads_total > 0 {
            let avg_nanos = self.load_time_total.as_nanos() / self.loads_total as u128;
            self.avg_load_time = std::time::Duration::from_nanos(avg_nanos as u64);
        }
    }

    /// Get save success rate (0.0 to 1.0)
    pub fn save_success_rate(&self) -> f64 {
        if self.saves_total == 0 {
            0.0
        } else {
            (self.saves_total - self.save_errors) as f64 / self.saves_total as f64
        }
    }

    /// Get load success rate (0.0 to 1.0)
    pub fn load_success_rate(&self) -> f64 {
        if self.loads_total == 0 {
            0.0
        } else {
            (self.loads_total - self.load_errors) as f64 / self.loads_total as f64
        }
    }

    /// Get total operations
    pub fn total_operations(&self) -> u64 {
        self.saves_total + self.loads_total
    }

    /// Get total errors
    pub fn total_errors(&self) -> u64 {
        self.save_errors + self.load_errors
    }

    /// Get overall success rate
    pub fn overall_success_rate(&self) -> f64 {
        let total = self.total_operations();
        if total == 0 {
            0.0
        } else {
            (total - self.total_errors()) as f64 / total as f64
        }
    }

    /// Get time since last operation
    pub fn time_since_last_operation(&self) -> Option<std::time::Duration> {
        let last_save = self.last_save_time?;
        let last_load = self.last_load_time?;

        Some(last_save.max(last_load).elapsed())
    }

    /// Get performance summary
    pub fn summary(&self) -> String {
        format!(
            "PersistenceStats {{ saves: {}, loads: {}, success: {:.2}%, avg_save: {:.2}ms, avg_load: {:.2}ms }}",
            self.saves_total,
            self.loads_total,
            self.overall_success_rate() * 100.0,
            self.avg_save_time.as_millis(),
            self.avg_load_time.as_millis()
        )
    }
}

impl std::fmt::Display for PersistenceStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// Data size statistics
#[derive(Debug, Clone, Default)]
pub struct DataSizeStats {
    /// Total data saved
    pub total_saved: u64,
    /// Total data loaded
    pub total_loaded: u64,
    /// Average save size
    pub avg_save_size: f64,
    /// Average load size
    pub avg_load_size: f64,
    /// Maximum save size
    pub max_save_size: usize,
    /// Maximum load size
    pub max_load_size: usize,
    /// Minimum save size
    pub min_save_size: usize,
    /// Minimum load size
    pub min_load_size: usize,
    /// Save operations count
    pub save_count: u64,
    /// Load operations count
    pub load_count: u64,
}

impl DataSizeStats {
    /// Record a save operation size
    pub fn record_save_size(&mut self, size: usize) {
        self.total_saved += size as u64;
        self.save_count += 1;
        self.max_save_size = self.max_save_size.max(size);

        if self.min_save_size == 0 || size < self.min_save_size {
            self.min_save_size = size;
        }

        self.update_save_average();
    }

    /// Record a load operation size
    pub fn record_load_size(&mut self, size: usize) {
        self.total_loaded += size as u64;
        self.load_count += 1;
        self.max_load_size = self.max_load_size.max(size);

        if self.min_load_size == 0 || size < self.min_load_size {
            self.min_load_size = size;
        }

        self.update_load_average();
    }

    /// Update save average
    fn update_save_average(&mut self) {
        if self.save_count > 0 {
            self.avg_save_size = self.total_saved as f64 / self.save_count as f64;
        }
    }

    /// Update load average
    fn update_load_average(&mut self) {
        if self.load_count > 0 {
            self.avg_load_size = self.total_loaded as f64 / self.load_count as f64;
        }
    }

    /// Get compression ratio (saved vs loaded)
    pub fn compression_ratio(&self) -> Option<f64> {
        if self.total_loaded > 0 {
            Some(self.total_saved as f64 / self.total_loaded as f64)
        } else {
            None
        }
    }

    /// Get size efficiency summary
    pub fn efficiency_summary(&self) -> String {
        let ratio = self.compression_ratio()
            .map(|r| format!("{:.2}", r))
            .unwrap_or("N/A".to_string());

        format!(
            "DataSizeStats {{ saved: {} bytes (avg: {:.0}), loaded: {} bytes (avg: {:.0}), ratio: {} }}",
            self.total_saved,
            self.avg_save_size,
            self.total_loaded,
            self.avg_load_size,
            ratio
        )
    }
}

/// Performance monitor for persistence operations
pub struct PersistenceMonitor {
    stats: std::sync::RwLock<PersistenceStats>,
}

impl PersistenceMonitor {
    /// Create a new persistence monitor
    pub fn new() -> Self {
        Self {
            stats: std::sync::RwLock::new(PersistenceStats::default()),
        }
    }

    /// Record a save operation
    pub fn record_save(&self, duration: std::time::Duration, data_size: usize, success: bool) {
        if let Ok(mut stats) = self.stats.write() {
            stats.record_save(duration, data_size, success);
        }
    }

    /// Record a load operation
    pub fn record_load(&self, duration: std::time::Duration, data_size: usize, success: bool) {
        if let Ok(mut stats) = self.stats.write() {
            stats.record_load(duration, data_size, success);
        }
    }

    /// Get current statistics
    pub fn stats(&self) -> PersistenceStats {
        self.stats.read().unwrap().clone()
    }

    /// Reset statistics
    pub fn reset(&self) {
        *self.stats.write().unwrap() = PersistenceStats::default();
    }

    /// Get performance report
    pub fn report(&self) -> PersistenceReport {
        let stats = self.stats();

        PersistenceReport {
            timestamp: std::time::SystemTime::now(),
            stats,
            recommendations: self.generate_recommendations(&stats),
        }
    }

    /// Generate performance recommendations
    fn generate_recommendations(&self, stats: &PersistenceStats) -> Vec<String> {
        let mut recommendations = Vec::new();

        if stats.save_success_rate() < 0.95 {
            recommendations.push("Consider implementing retry logic for save operations".to_string());
        }

        if stats.load_success_rate() < 0.95 {
            recommendations.push("Consider implementing retry logic for load operations".to_string());
        }

        if stats.avg_save_time > std::time::Duration::from_millis(100) {
            recommendations.push("Consider optimizing save operations or implementing compression".to_string());
        }

        if stats.avg_load_time > std::time::Duration::from_millis(50) {
            recommendations.push("Consider optimizing load operations or implementing caching".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("Persistence performance is within acceptable ranges".to_string());
        }

        recommendations
    }
}

impl Default for PersistenceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance report
#[derive(Debug, Clone)]
pub struct PersistenceReport {
    /// Report timestamp
    pub timestamp: std::time::SystemTime,
    /// Performance statistics
    pub stats: PersistenceStats,
    /// Performance recommendations
    pub recommendations: Vec<String>,
}

impl PersistenceReport {
    /// Convert to human-readable format
    pub fn to_string(&self) -> String {
        let mut report = format!("Persistence Performance Report ({})\n", self.timestamp.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs());
        report.push_str(&format!("Statistics: {}\n", self.stats.summary()));
        report.push_str("Recommendations:\n");

        for recommendation in &self.recommendations {
            report.push_str(&format!("  - {}\n", recommendation));
        }

        report
    }
}
