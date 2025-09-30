//! Time window utilities for managing time ranges

use super::core::TimeUtils;

/// Time window representing a range of time
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeWindow {
    /// Start time of the window
    pub start: std::time::Instant,
    /// End time of the window
    pub end: std::time::Instant,
}

impl TimeWindow {
    /// Create a new time window
    pub fn new(start: std::time::Instant, end: std::time::Instant) -> Self {
        assert!(start <= end, "Start time must be before or equal to end time");
        Self { start, end }
    }

    /// Create a window starting now with given duration
    pub fn from_now(duration: std::time::Duration) -> Self {
        let start = TimeUtils::now();
        let end = start + duration;
        Self { start, end }
    }

    /// Create a window starting at given time with duration
    pub fn from_start(start: std::time::Instant, duration: std::time::Duration) -> Self {
        let end = start + duration;
        Self { start, end }
    }

    /// Create a window ending at given time with duration
    pub fn from_end(end: std::time::Instant, duration: std::time::Duration) -> Self {
        let start = end - duration;
        Self { start, end }
    }

    /// Get duration of the window
    pub fn duration(&self) -> std::time::Duration {
        self.end - self.start
    }

    /// Check if a time is within the window
    pub fn contains(&self, time: std::time::Instant) -> bool {
        time >= self.start && time <= self.end
    }

    /// Check if current time is within the window
    pub fn contains_now(&self) -> bool {
        self.contains(TimeUtils::now())
    }

    /// Check if window has started
    pub fn has_started(&self) -> bool {
        TimeUtils::now() >= self.start
    }

    /// Check if window has ended
    pub fn has_ended(&self) -> bool {
        TimeUtils::now() > self.end
    }

    /// Check if window is active (started but not ended)
    pub fn is_active(&self) -> bool {
        self.has_started() && !self.has_ended()
    }

    /// Check if window is in the future
    pub fn is_future(&self) -> bool {
        TimeUtils::now() < self.start
    }

    /// Check if window is in the past
    pub fn is_past(&self) -> bool {
        TimeUtils::now() > self.end
    }

    /// Get time until window starts
    pub fn time_until_start(&self) -> Option<std::time::Duration> {
        let now = TimeUtils::now();
        if now < self.start {
            Some(self.start - now)
        } else {
            None
        }
    }

    /// Get time until window ends
    pub fn time_until_end(&self) -> Option<std::time::Duration> {
        let now = TimeUtils::now();
        if now < self.end {
            Some(self.end - now)
        } else {
            None
        }
    }

    /// Get elapsed time within window
    pub fn elapsed(&self) -> std::time::Duration {
        let now = TimeUtils::now();
        if now <= self.start {
            std::time::Duration::from_nanos(0)
        } else if now >= self.end {
            self.duration()
        } else {
            now - self.start
        }
    }

    /// Get remaining time in window
    pub fn remaining(&self) -> std::time::Duration {
        let now = TimeUtils::now();
        if now >= self.end {
            std::time::Duration::from_nanos(0)
        } else if now <= self.start {
            self.duration()
        } else {
            self.end - now
        }
    }

    /// Get progress through window (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        let total = self.duration().as_nanos() as f64;
        if total == 0.0 {
            1.0
        } else {
            let elapsed = self.elapsed().as_nanos() as f64;
            (elapsed / total).min(1.0)
        }
    }

    /// Check if two windows overlap
    pub fn overlaps(&self, other: &TimeWindow) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    /// Get intersection of two windows
    pub fn intersection(&self, other: &TimeWindow) -> Option<TimeWindow> {
        if !self.overlaps(other) {
            return None;
        }

        let start = self.start.max(other.start);
        let end = self.end.min(other.end);

        Some(TimeWindow::new(start, end))
    }

    /// Get union of two windows (smallest window containing both)
    pub fn union(&self, other: &TimeWindow) -> TimeWindow {
        let start = self.start.min(other.start);
        let end = self.end.max(other.end);

        TimeWindow::new(start, end)
    }

    /// Extend window by duration on both sides
    pub fn extend(&self, duration: std::time::Duration) -> TimeWindow {
        TimeWindow::new(self.start - duration, self.end + duration)
    }

    /// Shrink window by duration on both sides
    pub fn shrink(&self, duration: std::time::Duration) -> TimeWindow {
        let new_start = self.start + duration;
        let new_end = self.end - duration;

        if new_start < new_end {
            TimeWindow::new(new_start, new_end)
        } else {
            // Window would be invalid, return zero-duration window at center
            let center = self.start + self.duration() / 2;
            TimeWindow::new(center, center)
        }
    }

    /// Split window into smaller windows of given duration
    pub fn split(&self, segment_duration: std::time::Duration) -> Vec<TimeWindow> {
        let mut windows = Vec::new();
        let mut current_start = self.start;

        while current_start < self.end {
            let current_end = (current_start + segment_duration).min(self.end);
            windows.push(TimeWindow::new(current_start, current_end));
            current_start = current_end;
        }

        windows
    }

    /// Check if window is valid
    pub fn is_valid(&self) -> bool {
        self.start <= self.end
    }
}

impl std::fmt::Display for TimeWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if self.is_past() {
            "past"
        } else if self.is_future() {
            "future"
        } else if self.is_active() {
            "active"
        } else {
            "unknown"
        };

        write!(
            f,
            "TimeWindow({}, {}, {})",
            TimeUtils::format_duration(self.duration()),
            status,
            TimeUtils::format_duration(self.remaining())
        )
    }
}

/// Sliding time window for rate limiting and metrics
#[derive(Debug, Clone)]
pub struct SlidingWindow {
    /// Window duration
    pub duration: std::time::Duration,
    /// Events in the window
    events: Vec<std::time::Instant>,
}

impl SlidingWindow {
    /// Create a new sliding window
    pub fn new(duration: std::time::Duration) -> Self {
        Self {
            duration,
            events: Vec::new(),
        }
    }

    /// Add an event to the window
    pub fn add_event(&mut self) {
        let now = TimeUtils::now();
        self.events.push(now);
        self.cleanup();
    }

    /// Add multiple events
    pub fn add_events(&mut self, count: usize) {
        let now = TimeUtils::now();
        for _ in 0..count {
            self.events.push(now);
        }
        self.cleanup();
    }

    /// Get event count in current window
    pub fn event_count(&mut self) -> usize {
        self.cleanup();
        self.events.len()
    }

    /// Get events per second rate
    pub fn events_per_second(&mut self) -> f64 {
        self.cleanup();
        if self.events.is_empty() {
            0.0
        } else {
            let count = self.events.len() as f64;
            let duration_secs = self.duration.as_secs_f64();
            count / duration_secs
        }
    }

    /// Check if rate limit exceeded
    pub fn exceeds_limit(&mut self, max_events: usize) -> bool {
        self.event_count() >= max_events
    }

    /// Get time until can add another event without exceeding limit
    pub fn time_until_can_add(&mut self, max_events: usize) -> Option<std::time::Duration> {
        if self.event_count() < max_events {
            return Some(std::time::Duration::from_nanos(0));
        }

        // Find oldest event that would be kept if we add one more
        let current_count = self.events.len();
        if current_count < max_events {
            return Some(std::time::Duration::from_nanos(0));
        }

        // Need to wait until oldest event expires
        let oldest_to_keep = self.events.get(current_count - max_events)?;
        let now = TimeUtils::now();
        if now >= *oldest_to_keep + self.duration {
            Some(std::time::Duration::from_nanos(0))
        } else {
            Some(*oldest_to_keep + self.duration - now)
        }
    }

    /// Cleanup expired events
    fn cleanup(&mut self) {
        let cutoff = TimeUtils::now() - self.duration;
        self.events.retain(|&time| time > cutoff);
    }

    /// Clear all events
    pub fn clear(&mut self) {
        self.events.clear();
    }

    /// Get window duration
    pub fn window_duration(&self) -> std::time::Duration {
        self.duration
    }
}

impl std::fmt::Display for SlidingWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SlidingWindow({}, {} events)",
            TimeUtils::format_duration(self.duration),
            self.events.len()
        )
    }
}

/// Time window manager for coordinating multiple windows
pub struct WindowManager {
    windows: std::collections::HashMap<String, SlidingWindow>,
}

impl WindowManager {
    /// Create a new window manager
    pub fn new() -> Self {
        Self {
            windows: std::collections::HashMap::new(),
        }
    }

    /// Get or create a window
    pub fn get_window(&mut self, name: &str, duration: std::time::Duration) -> &mut SlidingWindow {
        self.windows
            .entry(name.to_string())
            .or_insert_with(|| SlidingWindow::new(duration))
    }

    /// Add event to window
    pub fn add_event(&mut self, window_name: &str, duration: std::time::Duration) {
        self.get_window(window_name, duration).add_event();
    }

    /// Check rate limit for window
    pub fn check_rate_limit(&mut self, window_name: &str, duration: std::time::Duration, max_events: usize) -> bool {
        !self.get_window(window_name, duration).exceeds_limit(max_events)
    }

    /// Get event count for window
    pub fn event_count(&mut self, window_name: &str, duration: std::time::Duration) -> usize {
        self.get_window(window_name, duration).event_count()
    }

    /// Clear window
    pub fn clear_window(&mut self, window_name: &str) {
        if let Some(window) = self.windows.get_mut(window_name) {
            window.clear();
        }
    }

    /// Clear all windows
    pub fn clear_all(&mut self) {
        self.windows.clear();
    }

    /// Get window names
    pub fn window_names(&self) -> Vec<&str> {
        self.windows.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience functions for time windows
pub mod factories {
    use super::*;

    /// Create a window for the last N milliseconds
    pub fn last_ms(ms: u64) -> TimeWindow {
        TimeWindow::from_now(std::time::Duration::from_millis(ms))
    }

    /// Create a window for the last N seconds
    pub fn last_secs(secs: u64) -> TimeWindow {
        TimeWindow::from_now(std::time::Duration::from_secs(secs))
    }

    /// Create a window for the last N minutes
    pub fn last_mins(mins: u64) -> TimeWindow {
        TimeWindow::from_now(std::time::Duration::from_secs(mins * 60))
    }

    /// Create a window for the next N milliseconds
    pub fn next_ms(ms: u64) -> TimeWindow {
        let start = TimeUtils::now();
        let end = start + std::time::Duration::from_millis(ms);
        TimeWindow::new(start, end)
    }

    /// Create a window for the next N seconds
    pub fn next_secs(secs: u64) -> TimeWindow {
        let start = TimeUtils::now();
        let end = start + std::time::Duration::from_secs(secs);
        TimeWindow::new(start, end)
    }

    /// Create a sliding window for rate limiting
    pub fn sliding_window(duration: std::time::Duration) -> SlidingWindow {
        SlidingWindow::new(duration)
    }
}
