//! Memoized selectors and advanced selection patterns

pub mod basic;
pub mod dependency;
pub mod combined;
pub mod performance;
pub mod lazy;
pub mod factory;

// Re-export the most commonly used items
pub use basic::{MemoizedSelector, CacheStrategy, SelectorConfig, MemoizedSelectorBuilder};
pub use dependency::{DependencyTrackedSelector, DependencyTracker};
pub use combined::{CombinedSelector, CombinationStrategy, SelectorPipeline};
pub use performance::{PerformanceSelector, SelectorStats, SelectorPerformanceMonitor};
pub use lazy::{LazySelector, LazyEvaluationManager, LazySelectorStats};
pub use factory::{SelectorFactory, presets};
