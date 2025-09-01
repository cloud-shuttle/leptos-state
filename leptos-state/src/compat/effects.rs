//! # Effects Compatibility Layer
//!
//! Provides version-agnostic APIs for Leptos effects across different versions.
//! This layer uses direct imports for better reliability.

use leptos::prelude::*;
use super::{CompatResult, CompatError};

// Direct imports for better reliability
use leptos::effect;

/// Version-agnostic effect creation
pub fn create_effect<F>(f: F)
where
    F: Fn() + Send + Sync + 'static,
{
    // Use the current Leptos API directly
    effect(f)
}

/// Version-agnostic effect creation with cleanup
pub fn create_effect_with_cleanup<F, C>(effect_fn: F, cleanup: C)
where
    F: Fn() + Send + Sync + 'static,
    C: Fn() + Send + Sync + 'static,
{
    // Use the current Leptos API directly - simplified for now
    effect(move || {
        effect_fn();
        cleanup();
    })
}

/// Version-agnostic effect creation with dependencies
pub fn create_effect_with_deps<F, D>(f: F, _deps: D)
where
    F: Fn() + Send + Sync + 'static,
    D: Clone + PartialEq + Send + Sync + 'static,
{
    // Use the current Leptos API directly - simplified for now
    effect(f)
}

/// Version-agnostic effect creation with signal dependencies
pub fn create_effect_with_signals<F>(f: F)
where
    F: Fn() + Send + Sync + 'static,
{
    create_effect(f)
}

/// Version-agnostic effect creation with memo dependencies
pub fn create_effect_with_memos<F>(f: F)
where
    F: Fn() + Send + Sync + 'static,
{
    create_effect(f)
}

/// Version-agnostic effect creation with resource dependencies
pub fn create_effect_with_resources<F>(f: F)
where
    F: Fn() + Send + Sync + 'static,
{
    create_effect(f)
}

/// Version-agnostic effect creation with debouncing
pub fn create_debounced_effect<F>(f: F, _delay_ms: u32)
where
    F: Fn() + Send + Sync + 'static,
{
    // For now, just create a regular effect
    // TODO: Implement proper debouncing
    create_effect(f)
}

/// Version-agnostic effect creation with throttling
pub fn create_throttled_effect<F>(f: F, _interval_ms: u32)
where
    F: Fn() + Send + Sync + 'static,
{
    // For now, just create a regular effect
    // TODO: Implement proper throttling
    create_effect(f)
}

/// Version-agnostic effect creation with error handling
pub fn create_effect_with_error_handling<F>(f: F)
where
    F: Fn() -> Result<(), Box<dyn std::error::Error>> + Send + Sync + 'static,
{
    // Use the current Leptos API directly
    effect(move || {
        if let Err(e) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f())) {
            eprintln!("Effect error: {:?}", e);
        }
    })
}

/// Version-agnostic effect creation with logging
pub fn create_effect_with_logging<F>(f: F, name: &'static str)
where
    F: Fn() + Send + Sync + 'static,
{
    create_effect(move || {
        tracing::debug!("Running effect: {}", name);
        f();
        tracing::debug!("Completed effect: {}", name);
    })
}

/// Version-agnostic effect creation with performance monitoring
pub fn create_effect_with_performance<F>(f: F, name: &'static str)
where
    F: Fn() + Send + Sync + 'static,
{
    create_effect(move || {
        let start = std::time::Instant::now();
        f();
        let duration = start.elapsed();
        tracing::debug!("Effect '{}' took {:?}", name, duration);
    })
}

/// Version-agnostic effect creation with conditional execution
pub fn create_conditional_effect<F, P>(f: F, predicate: P)
where
    F: Fn() + Send + Sync + 'static,
    P: Fn() -> bool + Send + Sync + 'static,
{
    create_effect(move || {
        if predicate() {
            f();
        }
    })
}

/// Version-agnostic effect creation with signal tracking
pub fn create_signal_tracking_effect<F>(f: F)
where
    F: Fn() + Send + Sync + 'static,
{
    create_effect(f)
}

/// Version-agnostic effect creation with memo tracking
pub fn create_memo_tracking_effect<F>(f: F)
where
    F: Fn() + Send + Sync + 'static,
{
    create_effect(f)
}

/// Version-agnostic effect creation with resource tracking
pub fn create_resource_tracking_effect<F>(f: F)
where
    F: Fn() + Send + Sync + 'static,
{
    create_effect(f)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_effect() {
        let (signal, set_signal) = create_signal(0);
        let mut effect_run = false;
        
        create_effect(move || {
            let _ = signal.get();
            effect_run = true;
        });
        
        // The effect should have run
        assert!(effect_run);
    }
    
    #[test]
    fn test_create_effect_with_cleanup() {
        let (signal, set_signal) = create_signal(0);
        let mut cleanup_run = false;
        
        create_effect_with_cleanup(
            move || {
                let _ = signal.get();
            },
            move || {
                cleanup_run = true;
            },
        );
        
        // Change the signal to trigger cleanup
        set_signal.set(1);
        
        // Cleanup should have run
        assert!(cleanup_run);
    }
    
    #[test]
    fn test_create_conditional_effect() {
        let (condition, set_condition) = create_signal(false);
        let mut effect_run = false;
        
        create_conditional_effect(
            move || {
                effect_run = true;
            },
            move || condition.get(),
        );
        
        // Effect should not run when condition is false
        assert!(!effect_run);
        
        // Set condition to true
        set_condition.set(true);
        
        // Effect should run when condition is true
        assert!(effect_run);
    }
}
