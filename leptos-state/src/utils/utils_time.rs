//! Time utilities for delayed transitions and timeouts

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

    /// Create a delayed action
    pub fn delay_action<F>(duration: std::time::Duration, action: F) -> DelayedAction<F>
    where
        F: FnOnce() + Send + Sync + 'static,
    {
        DelayedAction {
            duration,
            action: Some(action),
        }
    }

    /// Create a repeating timer
    pub fn repeating_timer<F>(interval: std::time::Duration, action: F) -> RepeatingTimer<F>
    where
        F: Fn() + Send + Sync + 'static,
    {
        RepeatingTimer {
            interval,
            action,
            running: std::sync::atomic::AtomicBool::new(false),
            handle: std::sync::Mutex::new(None),
        }
    }
}

/// Timeout error
#[derive(Debug, Clone, PartialEq)]
pub struct TimeoutError {
    /// The duration that timed out
    pub duration: std::time::Duration,
}

impl std::fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Operation timed out after {:?}", self.duration)
    }
}

impl std::error::Error for TimeoutError {}

/// Delayed action
pub struct DelayedAction<F> {
    /// Delay duration
    pub duration: std::time::Duration,
    /// The action to execute
    pub action: Option<F>,
}

impl<F> DelayedAction<F>
where
    F: FnOnce() + Send + Sync + 'static,
{
    /// Execute the delayed action
    pub async fn execute(mut self) {
        tokio::time::sleep(self.duration).await;
        if let Some(action) = self.action.take() {
            action();
        }
    }

    /// Cancel the delayed action
    pub fn cancel(mut self) {
        self.action = None;
    }
}

/// Repeating timer
pub struct RepeatingTimer<F> {
    /// Interval between executions
    pub interval: std::time::Duration,
    /// The action to execute
    pub action: F,
    /// Whether the timer is running
    pub running: std::sync::atomic::AtomicBool,
    /// Task handle
    pub handle: std::sync::Mutex<Option<tokio::task::JoinHandle<()>>>,
}

impl<F> RepeatingTimer<F>
where
    F: Fn() + Clone + Send + Sync + 'static,
{
    /// Start the timer
    pub fn start(&self) {
        if self.running.load(std::sync::atomic::Ordering::SeqCst) {
            return;
        }

        self.running
            .store(true, std::sync::atomic::Ordering::SeqCst);

        let interval = self.interval;
        let action = self.action.clone();
        let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(
            self.running.load(std::sync::atomic::Ordering::SeqCst)
        ));

        let handle = tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            interval_timer.tick().await; // Skip the first immediate tick

            while running.load(std::sync::atomic::Ordering::SeqCst) {
                interval_timer.tick().await;
                action();
            }
        });

        *self.handle.lock().unwrap() = Some(handle);
    }

    /// Stop the timer
    pub async fn stop(&self) {
        self.running
            .store(false, std::sync::atomic::Ordering::SeqCst);

        if let Some(handle) = self.handle.lock().unwrap().take() {
            handle.await.ok();
        }
    }

    /// Check if the timer is running
    pub fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::SeqCst)
    }

    /// Change the interval
    pub fn set_interval(&mut self, interval: std::time::Duration) {
        self.interval = interval;
    }
}

impl<F> Drop for RepeatingTimer<F> {
    fn drop(&mut self) {
        self.running
            .store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

/// Time-based triggers
pub mod triggers {
    use super::*;

    /// Time-based trigger
    pub trait TimeTrigger {
        /// Check if the trigger should fire
        fn should_fire(&self, current_time: std::time::Instant) -> bool;

        /// Get the next fire time
        fn next_fire_time(&self, current_time: std::time::Instant) -> Option<std::time::Instant>;
    }

    /// One-time trigger
    pub struct OneTimeTrigger {
        /// Fire time
        pub fire_time: std::time::Instant,
        /// Whether it has fired
        pub fired: std::sync::atomic::AtomicBool,
    }

    impl OneTimeTrigger {
        /// Create a new one-time trigger
        pub fn new(fire_time: std::time::Instant) -> Self {
            Self {
                fire_time,
                fired: std::sync::atomic::AtomicBool::new(false),
            }
        }

        /// Create a trigger that fires after a delay
        pub fn after(delay: std::time::Duration) -> Self {
            Self::new(std::time::Instant::now() + delay)
        }
    }

    impl TimeTrigger for OneTimeTrigger {
        fn should_fire(&self, current_time: std::time::Instant) -> bool {
            !self.fired.load(std::sync::atomic::Ordering::SeqCst) && current_time >= self.fire_time
        }

        fn next_fire_time(&self, _current_time: std::time::Instant) -> Option<std::time::Instant> {
            if self.fired.load(std::sync::atomic::Ordering::SeqCst) {
                None
            } else {
                Some(self.fire_time)
            }
        }
    }

    /// Periodic trigger
    pub struct PeriodicTrigger {
        /// Interval between fires
        pub interval: std::time::Duration,
        /// Last fire time
        pub last_fire: std::sync::Mutex<Option<std::time::Instant>>,
    }

    impl PeriodicTrigger {
        /// Create a new periodic trigger
        pub fn new(interval: std::time::Duration) -> Self {
            Self {
                interval,
                last_fire: std::sync::Mutex::new(None),
            }
        }
    }

    impl TimeTrigger for PeriodicTrigger {
        fn should_fire(&self, current_time: std::time::Instant) -> bool {
            let mut last_fire = self.last_fire.lock().unwrap();

            match *last_fire {
                None => {
                    *last_fire = Some(current_time);
                    true
                }
                Some(last) => {
                    if current_time.duration_since(last) >= self.interval {
                        *last_fire = Some(current_time);
                        true
                    } else {
                        false
                    }
                }
            }
        }

        fn next_fire_time(&self, current_time: std::time::Instant) -> Option<std::time::Instant> {
            let last_fire = self.last_fire.lock().unwrap();
            match *last_fire {
                None => Some(current_time),
                Some(last) => Some(last + self.interval),
            }
        }
    }

    /// Cron-like trigger
    pub struct CronTrigger {
        /// Cron expression
        pub expression: String,
        /// Last fire time
        pub last_fire: std::sync::Mutex<Option<std::time::Instant>>,
    }

    impl CronTrigger {
        /// Create a new cron trigger
        pub fn new(expression: String) -> Self {
            Self {
                expression,
                last_fire: std::sync::Mutex::new(None),
            }
        }
    }

    impl TimeTrigger for CronTrigger {
        fn should_fire(&self, _current_time: std::time::Instant) -> bool {
            // Simplified implementation - in practice, this would parse cron expressions
            // For now, just fire once
            let mut last_fire = self.last_fire.lock().unwrap();
            if last_fire.is_none() {
                *last_fire = Some(_current_time);
                true
            } else {
                false
            }
        }

        fn next_fire_time(&self, _current_time: std::time::Instant) -> Option<std::time::Instant> {
            // Simplified - would calculate next cron time
            None
        }
    }
}

/// Time window utilities
pub mod windows {
    use super::*;

    /// Time window
    #[derive(Debug, Clone)]
    pub struct TimeWindow {
        /// Start time
        pub start: std::time::Instant,
        /// End time
        pub end: std::time::Instant,
    }

    impl TimeWindow {
        /// Create a new time window
        pub fn new(start: std::time::Instant, end: std::time::Instant) -> Self {
            Self { start, end }
        }

        /// Create a time window of a specific duration
        pub fn of_duration(duration: std::time::Duration) -> Self {
            let now = std::time::Instant::now();
            Self {
                start: now,
                end: now + duration,
            }
        }

        /// Create a sliding time window
        pub fn sliding(duration: std::time::Duration) -> SlidingWindow {
            SlidingWindow {
                duration,
                last_reset: std::sync::Mutex::new(std::time::Instant::now()),
            }
        }

        /// Check if a time is within this window
        pub fn contains(&self, time: std::time::Instant) -> bool {
            time >= self.start && time <= self.end
        }

        /// Get the duration of this window
        pub fn duration(&self) -> std::time::Duration {
            self.end.duration_since(self.start)
        }

        /// Check if this window has expired
        pub fn is_expired(&self, current_time: std::time::Instant) -> bool {
            current_time > self.end
        }
    }

    /// Sliding time window
    #[derive(Debug)]
    pub struct SlidingWindow {
        /// Window duration
        pub duration: std::time::Duration,
        /// Last reset time
        pub last_reset: std::sync::Mutex<std::time::Instant>,
    }

    impl SlidingWindow {
        /// Check if the window should slide (reset)
        pub fn should_slide(&self, current_time: std::time::Instant) -> bool {
            let last_reset = *self.last_reset.lock().unwrap();
            current_time.duration_since(last_reset) >= self.duration
        }

        /// Slide the window
        pub fn slide(&self, current_time: std::time::Instant) {
            *self.last_reset.lock().unwrap() = current_time;
        }

        /// Get the current window
        pub fn current_window(&self) -> TimeWindow {
            let last_reset = *self.last_reset.lock().unwrap();
            TimeWindow {
                start: last_reset,
                end: last_reset + self.duration,
            }
        }
    }

    /// Tumbling time window
    pub struct TumblingWindow {
        /// Window duration
        pub duration: std::time::Duration,
        /// Windows
        pub windows: std::sync::Mutex<Vec<TimeWindow>>,
    }

    impl TumblingWindow {
        /// Create a new tumbling window
        pub fn new(duration: std::time::Duration) -> Self {
            Self {
                duration,
                windows: std::sync::Mutex::new(Vec::new()),
            }
        }

        /// Add a new window
        pub fn add_window(&self, start_time: std::time::Instant) {
            let window = TimeWindow {
                start: start_time,
                end: start_time + self.duration,
            };
            self.windows.lock().unwrap().push(window);
        }

        /// Get active windows
        pub fn active_windows(&self, current_time: std::time::Instant) -> Vec<TimeWindow> {
            let mut windows = self.windows.lock().unwrap();
            windows.retain(|w| !w.is_expired(current_time));
            windows.clone()
        }

        /// Cleanup expired windows
        pub fn cleanup(&self, current_time: std::time::Instant) {
            self.windows
                .lock()
                .unwrap()
                .retain(|w| !w.is_expired(current_time));
        }
    }
}

/// Rate limiting utilities
pub mod rate_limiting {
    use super::*;

    /// Token bucket rate limiter
    pub struct TokenBucket {
        /// Maximum tokens
        pub capacity: usize,
        /// Current tokens
        pub tokens: std::sync::atomic::AtomicUsize,
        /// Refill rate (tokens per second)
        pub refill_rate: f64,
        /// Last refill time
        pub last_refill: std::sync::Mutex<std::time::Instant>,
    }

    impl TokenBucket {
        /// Create a new token bucket
        pub fn new(capacity: usize, refill_rate: f64) -> Self {
            Self {
                capacity,
                tokens: std::sync::atomic::AtomicUsize::new(capacity),
                refill_rate,
                last_refill: std::sync::Mutex::new(std::time::Instant::now()),
            }
        }

        /// Try to consume a token
        pub fn try_consume(&self) -> bool {
            self.refill();

            let current_tokens = self.tokens.load(std::sync::atomic::Ordering::SeqCst);
            if current_tokens > 0 {
                self.tokens
                    .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                true
            } else {
                false
            }
        }

        /// Consume a token, blocking if necessary
        pub async fn consume(&self) {
            loop {
                self.refill();

                let current_tokens = self.tokens.load(std::sync::atomic::Ordering::SeqCst);
                if current_tokens > 0 {
                    self.tokens
                        .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                    break;
                }

                // Wait for refill
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            }
        }

        /// Refill tokens based on elapsed time
        fn refill(&self) {
            let mut last_refill = self.last_refill.lock().unwrap();
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(*last_refill).as_secs_f64();

            let tokens_to_add = (elapsed * self.refill_rate) as usize;
            if tokens_to_add > 0 {
                let current_tokens = self.tokens.load(std::sync::atomic::Ordering::SeqCst);
                let new_tokens = std::cmp::min(current_tokens + tokens_to_add, self.capacity);
                self.tokens
                    .store(new_tokens, std::sync::atomic::Ordering::SeqCst);
                *last_refill = now;
            }
        }

        /// Get current token count
        pub fn available_tokens(&self) -> usize {
            self.refill();
            self.tokens.load(std::sync::atomic::Ordering::SeqCst)
        }
    }

    /// Fixed window rate limiter
    pub struct FixedWindow {
        /// Window duration
        pub window_duration: std::time::Duration,
        /// Maximum requests per window
        pub max_requests: usize,
        /// Current window start
        pub window_start: std::sync::Mutex<std::time::Instant>,
        /// Request count in current window
        pub request_count: std::sync::atomic::AtomicUsize,
    }

    impl FixedWindow {
        /// Create a new fixed window rate limiter
        pub fn new(window_duration: std::time::Duration, max_requests: usize) -> Self {
            Self {
                window_duration,
                max_requests,
                window_start: std::sync::Mutex::new(std::time::Instant::now()),
                request_count: std::sync::atomic::AtomicUsize::new(0),
            }
        }

        /// Try to allow a request
        pub fn try_allow(&self) -> bool {
            let now = std::time::Instant::now();
            let mut window_start = self.window_start.lock().unwrap();

            // Check if we need to start a new window
            if now.duration_since(*window_start) >= self.window_duration {
                *window_start = now;
                self.request_count
                    .store(1, std::sync::atomic::Ordering::SeqCst);
                true
            } else {
                let current_count = self.request_count.load(std::sync::atomic::Ordering::SeqCst);
                if current_count < self.max_requests {
                    self.request_count
                        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    true
                } else {
                    false
                }
            }
        }
    }
}
