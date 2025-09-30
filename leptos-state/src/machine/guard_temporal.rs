//! Time-based guard implementations

use super::*;

/// Time-based guard - checks if enough time has passed
pub struct TimeGuard<C, E> {
    /// Minimum time that must have passed (in milliseconds)
    pub min_time_ms: u64,
    /// Maximum time that can have passed (in milliseconds, None for no limit)
    pub max_time_ms: Option<u64>,
    /// Time source function (returns current time in milliseconds)
    pub time_source: Box<dyn Fn() -> u64 + Send + Sync>,
    /// Start time tracker
    pub start_time: std::sync::Mutex<Option<u64>>,
    /// Description of the guard
    pub description: String,
}

impl<C, E> TimeGuard<C, E> {
    /// Create a new time guard with default time source
    pub fn new(min_time_ms: u64) -> Self {
        Self {
            min_time_ms,
            max_time_ms: None,
            time_source: Box::new(|| {
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64
            }),
            start_time: std::sync::Mutex::new(None),
            description: "Time Guard".to_string(),
        }
    }

    /// Set maximum time limit
    pub fn with_max_time(mut self, max_time_ms: u64) -> Self {
        self.max_time_ms = Some(max_time_ms);
        self
    }

    /// Set custom time source
    pub fn with_time_source<F>(mut self, time_source: F) -> Self
    where
        F: Fn() -> u64 + Send + Sync + 'static,
    {
        self.time_source = Box::new(time_source);
        self
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    /// Reset the timer
    pub fn reset(&self) {
        *self.start_time.lock().unwrap() = Some((self.time_source)());
    }
}

impl<C, E> GuardEvaluator<C, E> for TimeGuard<C, E> {
    fn check(&self, _context: &C, _event: &E) -> bool {
        let current_time = (self.time_source)();
        let mut start_time = self.start_time.lock().unwrap();

        // Initialize start time if not set
        if start_time.is_none() {
            *start_time = Some(current_time);
            return false; // Not enough time has passed yet
        }

        let elapsed = current_time - start_time.unwrap();

        // Check minimum time
        if elapsed < self.min_time_ms {
            return false;
        }

        // Check maximum time if set
        if let Some(max_time) = self.max_time_ms {
            if elapsed > max_time {
                return false;
            }
        }

        true
    }

    fn description(&self) -> String {
        let max_str = self
            .max_time_ms
            .map_or("∞".to_string(), |t| format!("{}ms", t));
        format!(
            "{} ({}ms to {})",
            self.description, self.min_time_ms, max_str
        )
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self {
            min_time_ms: self.min_time_ms,
            max_time_ms: self.max_time_ms,
            time_source: self.time_source.clone(),
            start_time: std::sync::Mutex::new(*self.start_time.lock().unwrap()),
            description: self.description.clone(),
        })
    }
}

/// Counter guard - limits the number of transitions
pub struct CounterGuard<C, E> {
    /// Maximum number of times this guard can pass
    pub max_count: usize,
    /// Current count
    pub current_count: std::sync::Mutex<usize>,
    /// Reset interval (in milliseconds, None for no reset)
    pub reset_interval: Option<u64>,
    /// Last reset time
    pub last_reset: std::sync::Mutex<Option<u64>>,
    /// Time source function
    pub time_source: Box<dyn Fn() -> u64 + Send + Sync>,
    /// Description of the guard
    pub description: String,
}

impl<C, E> CounterGuard<C, E> {
    /// Create a new counter guard
    pub fn new(max_count: usize) -> Self {
        Self {
            max_count,
            current_count: std::sync::Mutex::new(0),
            reset_interval: None,
            last_reset: std::sync::Mutex::new(None),
            time_source: Box::new(|| {
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64
            }),
            description: "Counter Guard".to_string(),
        }
    }

    /// Set reset interval
    pub fn with_reset_interval(mut self, interval_ms: u64) -> Self {
        self.reset_interval = Some(interval_ms);
        self
    }

    /// Set custom time source
    pub fn with_time_source<F>(mut self, time_source: F) -> Self
    where
        F: Fn() -> u64 + Send + Sync + 'static,
    {
        self.time_source = Box::new(time_source);
        self
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    /// Reset the counter
    pub fn reset_counter(&self) {
        *self.current_count.lock().unwrap() = 0;
        *self.last_reset.lock().unwrap() = Some((self.time_source)());
    }

    /// Get current count
    pub fn current_count(&self) -> usize {
        *self.current_count.lock().unwrap()
    }
}

impl<C, E> GuardEvaluator<C, E> for CounterGuard<C, E> {
    fn check(&self, _context: &C, _event: &E) -> bool {
        // Check if we need to reset
        if let Some(interval) = self.reset_interval {
            let current_time = (self.time_source)();
            let mut last_reset = self.last_reset.lock().unwrap();

            if let Some(reset_time) = *last_reset {
                if current_time - reset_time >= interval {
                    *self.current_count.lock().unwrap() = 0;
                    *last_reset = Some(current_time);
                }
            } else {
                *last_reset = Some(current_time);
            }
        }

        let mut count = self.current_count.lock().unwrap();
        if *count < self.max_count {
            *count += 1;
            true
        } else {
            false
        }
    }

    fn description(&self) -> String {
        let reset_str = self
            .reset_interval
            .map_or("never".to_string(), |i| format!("every {}ms", i));
        format!(
            "{} (max {} times, reset {})",
            self.description, self.max_count, reset_str
        )
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self {
            max_count: self.max_count,
            current_count: std::sync::Mutex::new(*self.current_count.lock().unwrap()),
            reset_interval: self.reset_interval,
            last_reset: std::sync::Mutex::new(*self.last_reset.lock().unwrap()),
            time_source: self.time_source.clone(),
            description: self.description.clone(),
        })
    }
}

/// Rate limit guard - limits the rate of transitions
pub struct RateLimitGuard<C, E> {
    /// Maximum number of transitions per time window
    pub max_per_window: usize,
    /// Time window in milliseconds
    pub window_ms: u64,
    /// Event timestamps
    pub timestamps: std::sync::Mutex<Vec<u64>>,
    /// Time source function
    pub time_source: Box<dyn Fn() -> u64 + Send + Sync>,
    /// Description of the guard
    pub description: String,
}

impl<C, E> RateLimitGuard<C, E> {
    /// Create a new rate limit guard
    pub fn new(max_per_window: usize, window_ms: u64) -> Self {
        Self {
            max_per_window,
            window_ms,
            timestamps: std::sync::Mutex::new(Vec::new()),
            time_source: Box::new(|| {
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64
            }),
            description: "Rate Limit Guard".to_string(),
        }
    }

    /// Set custom time source
    pub fn with_time_source<F>(mut self, time_source: F) -> Self
    where
        F: Fn() -> u64 + Send + Sync + 'static,
    {
        self.time_source = Box::new(time_source);
        self
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
}

impl<C, E> GuardEvaluator<C, E> for RateLimitGuard<C, E> {
    fn check(&self, _context: &C, _event: &E) -> bool {
        let current_time = (self.time_source)();
        let mut timestamps = self.timestamps.lock().unwrap();

        // Remove timestamps outside the current window
        let window_start = current_time.saturating_sub(self.window_ms);
        timestamps.retain(|&t| t >= window_start);

        // Check if we can add another timestamp
        if timestamps.len() < self.max_per_window {
            timestamps.push(current_time);
            true
        } else {
            false
        }
    }

    fn description(&self) -> String {
        format!(
            "{} (max {} per {}ms window)",
            self.description, self.max_per_window, self.window_ms
        )
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self {
            max_per_window: self.max_per_window,
            window_ms: self.window_ms,
            timestamps: std::sync::Mutex::new(self.timestamps.lock().unwrap().clone()),
            time_source: self.time_source.clone(),
            description: self.description.clone(),
        })
    }
}

/// Cooldown guard - prevents rapid successive transitions
pub struct CooldownGuard<C, E> {
    /// Cooldown period in milliseconds
    pub cooldown_ms: u64,
    /// Last activation time
    pub last_activation: std::sync::Mutex<Option<u64>>,
    /// Time source function
    pub time_source: Box<dyn Fn() -> u64 + Send + Sync>,
    /// Description of the guard
    pub description: String,
}

impl<C, E> CooldownGuard<C, E> {
    /// Create a new cooldown guard
    pub fn new(cooldown_ms: u64) -> Self {
        Self {
            cooldown_ms,
            last_activation: std::sync::Mutex::new(None),
            time_source: Box::new(|| {
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64
            }),
            description: "Cooldown Guard".to_string(),
        }
    }

    /// Set custom time source
    pub fn with_time_source<F>(mut self, time_source: F) -> Self
    where
        F: Fn() -> u64 + Send + Sync + 'static,
    {
        self.time_source = Box::new(time_source);
        self
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
}

impl<C, E> GuardEvaluator<C, E> for CooldownGuard<C, E> {
    fn check(&self, _context: &C, _event: &E) -> bool {
        let current_time = (self.time_source)();
        let mut last_activation = self.last_activation.lock().unwrap();

        if let Some(last_time) = *last_activation {
            if current_time - last_time < self.cooldown_ms {
                return false; // Still in cooldown
            }
        }

        // Update last activation time
        *last_activation = Some(current_time);
        true
    }

    fn description(&self) -> String {
        format!("{} ({}ms cooldown)", self.description, self.cooldown_ms)
    }

    fn clone_guard(&self) -> Box<dyn GuardEvaluator<C, E>> {
        Box::new(Self {
            cooldown_ms: self.cooldown_ms,
            last_activation: std::sync::Mutex::new(*self.last_activation.lock().unwrap()),
            time_source: self.time_source.clone(),
            description: self.description.clone(),
        })
    }
}
