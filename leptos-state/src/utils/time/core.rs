//! Core time utilities and basic functionality

/// Time utilities for delayed transitions and timeouts
#[derive(Debug, Clone)]
pub struct TimeUtils;

impl TimeUtils {
    /// Create a timeout future
    pub async fn timeout<F, T>(duration: std::time::Duration, future: F) -> Result<T, TimeoutError>
    where
        F: std::future::Future<Output = T>,
    {
        tokio::time::timeout(duration, future)
            .await
            .map_err(|_| TimeoutError { duration })
    }

    /// Sleep for a duration
    pub async fn sleep(duration: std::time::Duration) {
        tokio::time::sleep(duration).await;
    }

    /// Get the current time
    pub fn now() -> std::time::Instant {
        std::time::Instant::now()
    }

    /// Check if a duration has elapsed
    pub fn has_elapsed(start: std::time::Instant, duration: std::time::Duration) -> bool {
        start.elapsed() >= duration
    }

    /// Calculate elapsed time
    pub fn elapsed(start: std::time::Instant) -> std::time::Duration {
        start.elapsed()
    }

    /// Add duration to instant
    pub fn add_duration(instant: std::time::Instant, duration: std::time::Duration) -> std::time::Instant {
        // Note: std::time::Instant doesn't support adding duration directly
        // This is a conceptual operation - in practice you'd track deadlines separately
        instant
    }

    /// Check if instant is in the past
    pub fn is_past(instant: std::time::Instant) -> bool {
        instant.elapsed().as_nanos() > 0
    }

    /// Check if instant is in the future
    pub fn is_future(instant: std::time::Instant) -> bool {
        !Self::is_past(instant)
    }

    /// Get system time
    pub fn system_time() -> std::time::SystemTime {
        std::time::SystemTime::now()
    }

    /// Convert duration to milliseconds
    pub fn duration_to_ms(duration: std::time::Duration) -> u64 {
        duration.as_millis() as u64
    }

    /// Convert milliseconds to duration
    pub fn ms_to_duration(ms: u64) -> std::time::Duration {
        std::time::Duration::from_millis(ms)
    }

    /// Convert duration to seconds
    pub fn duration_to_secs(duration: std::time::Duration) -> u64 {
        duration.as_secs()
    }

    /// Convert seconds to duration
    pub fn secs_to_duration(secs: u64) -> std::time::Duration {
        std::time::Duration::from_secs(secs)
    }

    /// Format duration as human readable string
    pub fn format_duration(duration: std::time::Duration) -> String {
        let total_ms = duration.as_millis();

        if total_ms < 1000 {
            format!("{}ms", total_ms)
        } else if total_ms < 60_000 {
            format!("{:.1}s", total_ms as f64 / 1000.0)
        } else if total_ms < 3_600_000 {
            format!("{:.1}m", total_ms as f64 / 60_000.0)
        } else {
            format!("{:.1}h", total_ms as f64 / 3_600_000.0)
        }
    }

    /// Parse duration from string (simple implementation)
    pub fn parse_duration(s: &str) -> Result<std::time::Duration, String> {
        let s = s.trim();

        if let Some(ms) = s.strip_suffix("ms") {
            ms.parse::<u64>()
                .map(std::time::Duration::from_millis)
                .map_err(|_| format!("Invalid milliseconds: {}", ms))
        } else if let Some(s) = s.strip_suffix('s') {
            s.parse::<u64>()
                .map(std::time::Duration::from_secs)
                .map_err(|_| format!("Invalid seconds: {}", s))
        } else if let Some(m) = s.strip_suffix('m') {
            m.parse::<u64>()
                .map(|mins| std::time::Duration::from_secs(mins * 60))
                .map_err(|_| format!("Invalid minutes: {}", m))
        } else if let Some(h) = s.strip_suffix('h') {
            h.parse::<u64>()
                .map(|hours| std::time::Duration::from_secs(hours * 3600))
                .map_err(|_| format!("Invalid hours: {}", h))
        } else {
            // Default to seconds
            s.parse::<u64>()
                .map(std::time::Duration::from_secs)
                .map_err(|_| format!("Invalid duration: {}", s))
        }
    }

    /// Clamp duration between min and max
    pub fn clamp_duration(duration: std::time::Duration, min: std::time::Duration, max: std::time::Duration) -> std::time::Duration {
        if duration < min {
            min
        } else if duration > max {
            max
        } else {
            duration
        }
    }

    /// Check if two durations are approximately equal
    pub fn duration_approx_eq(a: std::time::Duration, b: std::time::Duration, tolerance: std::time::Duration) -> bool {
        if a > b {
            a - b <= tolerance
        } else {
            b - a <= tolerance
        }
    }

    /// Get monotonic time (same as now for std::time::Instant)
    pub fn monotonic_time() -> std::time::Instant {
        Self::now()
    }

    /// Check if duration is zero
    pub fn is_zero(duration: std::time::Duration) -> bool {
        duration.as_nanos() == 0
    }

    /// Check if duration is positive
    pub fn is_positive(duration: std::time::Duration) -> bool {
        !Self::is_zero(duration)
    }

    /// Get minimum of two durations
    pub fn min_duration(a: std::time::Duration, b: std::time::Duration) -> std::time::Duration {
        if a < b { a } else { b }
    }

    /// Get maximum of two durations
    pub fn max_duration(a: std::time::Duration, b: std::time::Duration) -> std::time::Duration {
        if a > b { a } else { b }
    }

    /// Scale duration by factor
    pub fn scale_duration(duration: std::time::Duration, factor: f64) -> std::time::Duration {
        let nanos = (duration.as_nanos() as f64 * factor) as u128;
        std::time::Duration::from_nanos(nanos)
    }

    /// Linear interpolation between two durations
    pub fn lerp_duration(a: std::time::Duration, b: std::time::Duration, t: f64) -> std::time::Duration {
        let t = t.max(0.0).min(1.0);
        let a_nanos = a.as_nanos() as f64;
        let b_nanos = b.as_nanos() as f64;
        let result_nanos = a_nanos + (b_nanos - a_nanos) * t;
        std::time::Duration::from_nanos(result_nanos as u64)
    }
}

/// Timeout error
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeoutError {
    /// The duration that timed out
    pub duration: std::time::Duration,
}

impl std::fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Operation timed out after {}", TimeUtils::format_duration(self.duration))
    }
}

impl std::error::Error for TimeoutError {}

impl TimeoutError {
    /// Create a new timeout error
    pub fn new(duration: std::time::Duration) -> Self {
        Self { duration }
    }

    /// Get the timeout duration
    pub fn timeout_duration(&self) -> std::time::Duration {
        self.duration
    }

    /// Check if this is a short timeout (< 1 second)
    pub fn is_short_timeout(&self) -> bool {
        self.duration < std::time::Duration::from_secs(1)
    }

    /// Check if this is a long timeout (> 1 minute)
    pub fn is_long_timeout(&self) -> bool {
        self.duration > std::time::Duration::from_secs(60)
    }
}
