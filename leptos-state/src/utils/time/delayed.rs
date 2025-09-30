//! Delayed action execution

use super::core::TimeUtils;

/// Delayed action
pub struct DelayedAction<F> {
    /// Duration to delay
    pub duration: std::time::Duration,
    /// The action to execute
    pub action: Option<F>,
    /// Creation time
    created_at: std::time::Instant,
}

impl<F> DelayedAction<F>
where
    F: FnOnce() + Send + Sync + 'static,
{
    /// Create a new delayed action
    pub fn new(duration: std::time::Duration, action: F) -> Self {
        Self {
            duration,
            action: Some(action),
            created_at: TimeUtils::now(),
        }
    }

    /// Check if the delay has elapsed
    pub fn is_ready(&self) -> bool {
        TimeUtils::has_elapsed(self.created_at, self.duration)
    }

    /// Execute the action if ready
    pub fn execute_if_ready(mut self) -> Option<()> {
        if self.is_ready() {
            if let Some(action) = self.action.take() {
                action();
                Some(())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get remaining delay time
    pub fn remaining(&self) -> std::time::Duration {
        let elapsed = self.created_at.elapsed();
        if elapsed >= self.duration {
            std::time::Duration::from_nanos(0)
        } else {
            self.duration - elapsed
        }
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> std::time::Duration {
        self.created_at.elapsed()
    }

    /// Cancel the action (consume without executing)
    pub fn cancel(mut self) {
        self.action = None;
    }

    /// Check if action is cancelled
    pub fn is_cancelled(&self) -> bool {
        self.action.is_none()
    }

    /// Get the delay duration
    pub fn delay_duration(&self) -> std::time::Duration {
        self.duration
    }

    /// Get creation time
    pub fn created_at(&self) -> std::time::Instant {
        self.created_at
    }

    /// Clone the action (if the action is cloneable)
    pub fn clone_action(&self) -> Option<&F>
    where
        F: Clone,
    {
        self.action.as_ref()
    }
}

impl<F> std::fmt::Debug for DelayedAction<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DelayedAction")
            .field("duration", &self.duration)
            .field("ready", &self.is_ready())
            .field("cancelled", &self.is_cancelled())
            .field("remaining", &self.remaining())
            .finish()
    }
}

impl<F> std::fmt::Display for DelayedAction<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_cancelled() {
            write!(f, "DelayedAction(cancelled)")
        } else if self.is_ready() {
            write!(f, "DelayedAction(ready)")
        } else {
            write!(f, "DelayedAction(remaining: {})", TimeUtils::format_duration(self.remaining()))
        }
    }
}

/// Builder for delayed actions
pub struct DelayedActionBuilder<F> {
    duration: Option<std::time::Duration>,
    action: Option<F>,
}

impl<F> DelayedActionBuilder<F>
where
    F: FnOnce() + Send + Sync + 'static,
{
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            duration: None,
            action: None,
        }
    }

    /// Set the delay duration
    pub fn delay(mut self, duration: std::time::Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Set the action to execute
    pub fn action(mut self, action: F) -> Self {
        self.action = Some(action);
        self
    }

    /// Build the delayed action
    pub fn build(self) -> Result<DelayedAction<F>, String> {
        let duration = self.duration.ok_or("Delay duration not set".to_string())?;
        let action = self.action.ok_or("Action not set".to_string())?;

        Ok(DelayedAction::new(duration, action))
    }
}

impl<F> Default for DelayedActionBuilder<F> {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience functions for delayed actions
pub mod factories {
    use super::*;

    /// Create a delayed action with milliseconds
    pub fn delay_ms<F>(ms: u64, action: F) -> DelayedAction<F>
    where
        F: FnOnce() + Send + Sync + 'static,
    {
        DelayedAction::new(std::time::Duration::from_millis(ms), action)
    }

    /// Create a delayed action with seconds
    pub fn delay_secs<F>(secs: u64, action: F) -> DelayedAction<F>
    where
        F: FnOnce() + Send + Sync + 'static,
    {
        DelayedAction::new(std::time::Duration::from_secs(secs), action)
    }

    /// Create a delayed action with minutes
    pub fn delay_mins<F>(mins: u64, action: F) -> DelayedAction<F>
    where
        F: FnOnce() + Send + Sync + 'static,
    {
        DelayedAction::new(std::time::Duration::from_secs(mins * 60), action)
    }

    /// Create a delayed action with hours
    pub fn delay_hours<F>(hours: u64, action: F) -> DelayedAction<F>
    where
        F: FnOnce() + Send + Sync + 'static,
    {
        DelayedAction::new(std::time::Duration::from_secs(hours * 3600), action)
    }
}
