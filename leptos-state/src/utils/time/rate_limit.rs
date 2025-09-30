//! Rate limiting utilities based on time windows

use super::windows::{SlidingWindow, WindowManager};
use super::core::TimeUtils;

/// Rate limiter using sliding window
#[derive(Debug)]
pub struct RateLimiter {
    window: SlidingWindow,
    max_events: usize,
    name: String,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(name: String, window_duration: std::time::Duration, max_events: usize) -> Self {
        Self {
            window: SlidingWindow::new(window_duration),
            max_events,
            name,
        }
    }

    /// Try to acquire permission (record an event)
    pub fn try_acquire(&mut self) -> bool {
        if self.window.event_count() < self.max_events {
            self.window.add_event();
            true
        } else {
            false
        }
    }

    /// Check if can acquire without recording
    pub fn can_acquire(&mut self) -> bool {
        self.window.event_count() < self.max_events
    }

    /// Force acquire (always succeeds, but may exceed limit)
    pub fn force_acquire(&mut self) {
        self.window.add_event();
    }

    /// Get current event count
    pub fn current_count(&mut self) -> usize {
        self.window.event_count()
    }

    /// Get remaining capacity
    pub fn remaining_capacity(&mut self) -> usize {
        self.max_events.saturating_sub(self.window.event_count())
    }

    /// Get time until can acquire again
    pub fn time_until_can_acquire(&mut self) -> Option<std::time::Duration> {
        self.window.time_until_can_add(self.max_events)
    }

    /// Check if currently rate limited
    pub fn is_rate_limited(&mut self) -> bool {
        !self.can_acquire()
    }

    /// Get rate limiter name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get window duration
    pub fn window_duration(&self) -> std::time::Duration {
        self.window.duration
    }

    /// Get max events
    pub fn max_events(&self) -> usize {
        self.max_events
    }

    /// Get current rate (events per second)
    pub fn current_rate(&mut self) -> f64 {
        self.window.events_per_second()
    }

    /// Reset the rate limiter
    pub fn reset(&mut self) {
        self.window.clear();
    }

    /// Set new max events
    pub fn set_max_events(&mut self, max_events: usize) {
        self.max_events = max_events;
    }

    /// Set new window duration
    pub fn set_window_duration(&mut self, duration: std::time::Duration) {
        self.window = SlidingWindow::new(duration);
    }
}

impl std::fmt::Display for RateLimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RateLimiter '{}' ({}/{} in {})",
            self.name,
            self.window.events.len(), // Approximate current count
            self.max_events,
            TimeUtils::format_duration(self.window.duration)
        )
    }
}

/// Token bucket rate limiter
#[derive(Debug)]
pub struct TokenBucket {
    /// Bucket capacity (max tokens)
    capacity: usize,
    /// Current token count
    tokens: usize,
    /// Token refill rate (tokens per second)
    refill_rate: f64,
    /// Last refill time
    last_refill: std::time::Instant,
    /// Bucket name
    name: String,
}

impl TokenBucket {
    /// Create a new token bucket
    pub fn new(name: String, capacity: usize, refill_rate: f64) -> Self {
        Self {
            capacity,
            tokens: capacity, // Start full
            refill_rate,
            last_refill: TimeUtils::now(),
            name,
        }
    }

    /// Try to consume tokens
    pub fn try_consume(&mut self, tokens: usize) -> bool {
        self.refill();
        if self.tokens >= tokens {
            self.tokens -= tokens;
            true
        } else {
            false
        }
    }

    /// Try to consume one token
    pub fn try_consume_one(&mut self) -> bool {
        self.try_consume(1)
    }

    /// Force consume tokens (may go negative)
    pub fn force_consume(&mut self, tokens: usize) {
        self.refill();
        self.tokens = self.tokens.saturating_sub(tokens);
    }

    /// Add tokens to bucket
    pub fn add_tokens(&mut self, tokens: usize) {
        self.tokens = (self.tokens + tokens).min(self.capacity);
    }

    /// Get current token count
    pub fn current_tokens(&mut self) -> usize {
        self.refill();
        self.tokens
    }

    /// Get bucket capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get refill rate
    pub fn refill_rate(&self) -> f64 {
        self.refill_rate
    }

    /// Get time until bucket is full
    pub fn time_until_full(&mut self) -> std::time::Duration {
        self.refill();
        let tokens_needed = self.capacity - self.tokens;
        if tokens_needed == 0 {
            std::time::Duration::from_nanos(0)
        } else {
            let seconds_needed = tokens_needed as f64 / self.refill_rate;
            std::time::Duration::from_secs_f64(seconds_needed)
        }
    }

    /// Get fill percentage (0.0 to 1.0)
    pub fn fill_percentage(&mut self) -> f64 {
        self.current_tokens() as f64 / self.capacity as f64
    }

    /// Reset bucket to full
    pub fn reset(&mut self) {
        self.tokens = self.capacity;
        self.last_refill = TimeUtils::now();
    }

    /// Refill tokens based on elapsed time
    fn refill(&mut self) {
        let now = TimeUtils::now();
        let elapsed = now - self.last_refill;
        let elapsed_secs = elapsed.as_secs_f64();

        let tokens_to_add = (elapsed_secs * self.refill_rate) as usize;
        if tokens_to_add > 0 {
            self.tokens = (self.tokens + tokens_to_add).min(self.capacity);
            self.last_refill = now;
        }
    }

    /// Get bucket name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl std::fmt::Display for TokenBucket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TokenBucket '{}' ({}/{})",
            self.name, self.tokens, self.capacity
        )
    }
}

/// Rate limiter manager for coordinating multiple limiters
pub struct RateLimiterManager {
    sliding_window_limiters: std::collections::HashMap<String, RateLimiter>,
    token_buckets: std::collections::HashMap<String, TokenBucket>,
}

impl RateLimiterManager {
    /// Create a new rate limiter manager
    pub fn new() -> Self {
        Self {
            sliding_window_limiters: std::collections::HashMap::new(),
            token_buckets: std::collections::HashMap::new(),
        }
    }

    /// Create or get a sliding window rate limiter
    pub fn sliding_window_limiter(
        &mut self,
        name: &str,
        window_duration: std::time::Duration,
        max_events: usize,
    ) -> &mut RateLimiter {
        self.sliding_window_limiters
            .entry(name.to_string())
            .or_insert_with(|| RateLimiter::new(name.to_string(), window_duration, max_events))
    }

    /// Create or get a token bucket
    pub fn token_bucket(
        &mut self,
        name: &str,
        capacity: usize,
        refill_rate: f64,
    ) -> &mut TokenBucket {
        self.token_buckets
            .entry(name.to_string())
            .or_insert_with(|| TokenBucket::new(name.to_string(), capacity, refill_rate))
    }

    /// Try to acquire from sliding window limiter
    pub fn try_acquire_sliding(&mut self, name: &str) -> Option<bool> {
        self.sliding_window_limiters.get_mut(name).map(|limiter| limiter.try_acquire())
    }

    /// Try to acquire from token bucket
    pub fn try_acquire_token(&mut self, name: &str) -> Option<bool> {
        self.token_buckets.get_mut(name).map(|bucket| bucket.try_consume_one())
    }

    /// Check if any rate limiter would block
    pub fn can_acquire(&mut self, sliding_names: &[&str], token_names: &[&str]) -> bool {
        // Check sliding window limiters
        for name in sliding_names {
            if let Some(limiter) = self.sliding_window_limiters.get_mut(*name) {
                if !limiter.can_acquire() {
                    return false;
                }
            }
        }

        // Check token buckets
        for name in token_names {
            if let Some(bucket) = self.token_buckets.get_mut(*name) {
                if bucket.current_tokens() == 0 {
                    return false;
                }
            }
        }

        true
    }

    /// Acquire from all specified limiters
    pub fn acquire_all(&mut self, sliding_names: &[&str], token_names: &[&str]) -> bool {
        if !self.can_acquire(sliding_names, token_names) {
            return false;
        }

        // Actually acquire
        for name in sliding_names {
            if let Some(limiter) = self.sliding_window_limiters.get_mut(*name) {
                limiter.force_acquire();
            }
        }

        for name in token_names {
            if let Some(bucket) = self.token_buckets.get_mut(*name) {
                let _ = bucket.try_consume_one();
            }
        }

        true
    }

    /// Get limiter names
    pub fn limiter_names(&self) -> Vec<&str> {
        let mut names = Vec::new();
        names.extend(self.sliding_window_limiters.keys().map(|s| s.as_str()));
        names.extend(self.token_buckets.keys().map(|s| s.as_str()));
        names
    }

    /// Reset all limiters
    pub fn reset_all(&mut self) {
        for limiter in self.sliding_window_limiters.values_mut() {
            limiter.reset();
        }
        for bucket in self.token_buckets.values_mut() {
            bucket.reset();
        }
    }

    /// Remove a limiter
    pub fn remove_limiter(&mut self, name: &str) {
        self.sliding_window_limiters.remove(name);
        self.token_buckets.remove(name);
    }
}

impl Default for RateLimiterManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience functions for rate limiting
pub mod factories {
    use super::*;

    /// Create a rate limiter that allows N events per second
    pub fn per_second(name: String, events_per_second: usize) -> RateLimiter {
        RateLimiter::new(
            name,
            std::time::Duration::from_secs(1),
            events_per_second,
        )
    }

    /// Create a rate limiter that allows N events per minute
    pub fn per_minute(name: String, events_per_minute: usize) -> RateLimiter {
        RateLimiter::new(
            name,
            std::time::Duration::from_secs(60),
            events_per_minute,
        )
    }

    /// Create a rate limiter that allows N events per hour
    pub fn per_hour(name: String, events_per_hour: usize) -> RateLimiter {
        RateLimiter::new(
            name,
            std::time::Duration::from_secs(3600),
            events_per_hour,
        )
    }

    /// Create a token bucket with 1 token per second refill rate
    pub fn token_bucket_per_second(name: String, capacity: usize) -> TokenBucket {
        TokenBucket::new(name, capacity, 1.0)
    }

    /// Create a token bucket with custom refill rate
    pub fn token_bucket_custom(name: String, capacity: usize, tokens_per_second: f64) -> TokenBucket {
        TokenBucket::new(name, capacity, tokens_per_second)
    }
}
