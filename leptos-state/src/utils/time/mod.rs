//! Time utilities for delayed transitions and timeouts

pub mod core;
pub mod delayed;
pub mod timer;
pub mod triggers;
pub mod windows;
pub mod rate_limit;

// Re-export the most commonly used items
pub use core::{TimeUtils, TimeoutError};
pub use delayed::{DelayedAction, factories as delayed_factories};
pub use timer::{RepeatingTimer, TimerManager, factories as timer_factories};
pub use triggers::{TimeTrigger, TriggerManager, TriggerSchedule, factories as trigger_factories};
pub use windows::{TimeWindow, SlidingWindow, WindowManager, factories as window_factories};
pub use rate_limit::{RateLimiter, TokenBucket, RateLimiterManager, factories as rate_limit_factories};
