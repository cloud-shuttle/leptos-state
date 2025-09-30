//! Generation statistics and metrics

/// Generation statistics
#[derive(Debug, Clone, Default)]
pub struct GenerationStats {
    /// Total files generated
    pub total_files_generated: u64,
    /// Total lines of code generated
    pub total_lines_generated: u64,
    /// Total generation time
    pub total_generation_time: std::time::Duration,
    /// Average generation time per file
    pub avg_generation_time: std::time::Duration,
    /// Last generation time
    pub last_generation_time: Option<std::time::Instant>,
    /// Generation errors count
    pub generation_errors: u64,
    /// Cache hits
    pub cache_hits: u64,
    /// Cache misses
    pub cache_misses: u64,
}

impl GenerationStats {
    /// Record a generation operation
    pub fn record_generation(&mut self, lines: usize, duration: std::time::Duration) {
        self.total_files_generated += 1;
        self.total_lines_generated += lines as u64;
        self.total_generation_time += duration;
        self.last_generation_time = Some(std::time::Instant::now());

        if self.total_files_generated > 0 {
            self.avg_generation_time = self.total_generation_time / self.total_files_generated as u32;
        }
    }

    /// Record a generation error
    pub fn record_error(&mut self) {
        self.generation_errors += 1;
    }

    /// Record a cache hit
    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
    }

    /// Record a cache miss
    pub fn record_cache_miss(&mut self) {
        self.cache_misses += 1;
    }

    /// Get cache hit rate as percentage
    pub fn cache_hit_rate(&self) -> f64 {
        let total_cache_accesses = self.cache_hits + self.cache_misses;
        if total_cache_accesses == 0 {
            0.0
        } else {
            (self.cache_hits as f64 / total_cache_accesses as f64) * 100.0
        }
    }

    /// Get error rate as percentage
    pub fn error_rate(&self) -> f64 {
        if self.total_files_generated == 0 {
            0.0
        } else {
            (self.generation_errors as f64 / self.total_files_generated as f64) * 100.0
        }
    }

    /// Get lines per second generation rate
    pub fn lines_per_second(&self) -> f64 {
        let total_seconds = self.total_generation_time.as_secs_f64();
        if total_seconds == 0.0 {
            0.0
        } else {
            self.total_lines_generated as f64 / total_seconds
        }
    }

    /// Get files per second generation rate
    pub fn files_per_second(&self) -> f64 {
        let total_seconds = self.total_generation_time.as_secs_f64();
        if total_seconds == 0.0 {
            0.0
        } else {
            self.total_files_generated as f64 / total_seconds
        }
    }

    /// Reset statistics
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Get summary string
    pub fn summary(&self) -> String {
        format!(
            "Generated {} files ({} lines) in {:.2}s, {:.0} lines/s, {:.1}% cache hit rate",
            self.total_files_generated,
            self.total_lines_generated,
            self.total_generation_time.as_secs_f64(),
            self.lines_per_second(),
            self.cache_hit_rate()
        )
    }

    /// Get detailed statistics
    pub fn detailed_stats(&self) -> String {
        let mut stats = String::new();
        stats.push_str("Code Generation Statistics\n");
        stats.push_str("==========================\n");
        stats.push_str(&format!("Total files generated: {}\n", self.total_files_generated));
        stats.push_str(&format!("Total lines generated: {}\n", self.total_lines_generated));
        stats.push_str(&format!("Total generation time: {:.2}s\n", self.total_generation_time.as_secs_f64()));
        stats.push_str(&format!("Average generation time: {:.2}ms\n", self.avg_generation_time.as_millis()));
        stats.push_str(&format!("Lines per second: {:.0}\n", self.lines_per_second()));
        stats.push_str(&format!("Files per second: {:.2}\n", self.files_per_second()));
        stats.push_str(&format!("Cache hits: {}\n", self.cache_hits));
        stats.push_str(&format!("Cache misses: {}\n", self.cache_misses));
        stats.push_str(&format!("Cache hit rate: {:.1}%\n", self.cache_hit_rate()));
        stats.push_str(&format!("Generation errors: {}\n", self.generation_errors));
        stats.push_str(&format!("Error rate: {:.1}%\n", self.error_rate()));
        stats
    }

    /// Check if statistics indicate healthy generation
    pub fn is_healthy(&self) -> bool {
        self.error_rate() < 5.0 && self.lines_per_second() > 100.0
    }

    /// Get health status
    pub fn health_status(&self) -> &'static str {
        if self.is_healthy() {
            "healthy"
        } else if self.error_rate() > 10.0 {
            "unhealthy"
        } else {
            "degraded"
        }
    }

    /// Export statistics as JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Import statistics from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl std::fmt::Display for GenerationStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

impl std::ops::Add for GenerationStats {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            total_files_generated: self.total_files_generated + other.total_files_generated,
            total_lines_generated: self.total_lines_generated + other.total_lines_generated,
            total_generation_time: self.total_generation_time + other.total_generation_time,
            avg_generation_time: std::time::Duration::from_millis(
                ((self.avg_generation_time.as_millis() + other.avg_generation_time.as_millis()) / 2) as u64
            ),
            last_generation_time: self.last_generation_time.or(other.last_generation_time),
            generation_errors: self.generation_errors + other.generation_errors,
            cache_hits: self.cache_hits + other.cache_hits,
            cache_misses: self.cache_misses + other.cache_misses,
        }
    }
}
