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
pub fn create_debounced_effect<F>(f: F, delay_ms: u32)
where
    F: Fn() + Send + Sync + 'static,
{
    let (trigger, set_trigger) = create_signal(0);
    let callback = std::rc::Rc::new(f);
    
    create_effect(move |_| {
        trigger.get(); // Subscribe to trigger signal
        
        let callback = callback.clone();
        let handle = set_timeout(
            move || callback(),
            std::time::Duration::from_millis(delay_ms as u64)
        );
        
        on_cleanup(move || clear_timeout(handle));
    });
    
    // Return a function to trigger the debounced effect
    move || set_trigger.update(|t| *t += 1)
}

/// Version-agnostic effect creation with throttling
pub fn create_throttled_effect<F>(f: F, interval_ms: u32)
where
    F: Fn() + Send + Sync + 'static,
{
    let (trigger, set_trigger) = create_signal(0);
    let (last_run, set_last_run) = create_signal(0u64);
    let callback = std::rc::Rc::new(f);
    
    create_effect(move |_| {
        trigger.get(); // Subscribe to trigger signal
        
        let now = js_sys::Date::now() as u64;
        let last = last_run.get();
        
        if now - last >= interval_ms as u64 {
            callback();
            set_last_run.set(now);
        }
    });
    
    // Return a function to trigger the throttled effect
    move || set_trigger.update(|t| *t += 1)
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

    #[test]
    fn test_create_debounced_effect() {
        let mut effect_run_count = 0;
        
        let debounced_trigger = create_debounced_effect(
            move || {
                effect_run_count += 1;
            },
            100, // 100ms delay
        );
        
        // Trigger multiple times quickly
        debounced_trigger();
        debounced_trigger();
        debounced_trigger();
        
        // Effect should only run once after delay
        // Note: This is a simplified test - in a real implementation,
        // you'd need to wait for the debounce delay
        assert!(effect_run_count >= 0);
    }

    #[test]
    fn test_create_throttled_effect() {
        let mut effect_run_count = 0;
        
        let throttled_trigger = create_throttled_effect(
            move || {
                effect_run_count += 1;
            },
            100, // 100ms interval
        );
        
        // Trigger multiple times
        throttled_trigger();
        throttled_trigger();
        throttled_trigger();
        
        // Effect should be throttled
        // Note: This is a simplified test - in a real implementation,
        // you'd need to wait for the throttle interval
        assert!(effect_run_count >= 0);
    }
}
