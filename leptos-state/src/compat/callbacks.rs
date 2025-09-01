//! # Callbacks Compatibility Layer
//!
//! Provides version-agnostic APIs for Leptos callbacks across different versions.
//! This layer uses direct imports for better reliability.

use leptos::prelude::*;
use super::{CompatResult, CompatError};

/// Version-agnostic callback creation
pub fn create_callback<T, F>(f: F) -> Callback<T>
where
    T: Clone + 'static,
    F: Fn(T) + 'static,
{
    // Use the current Leptos API directly
    leptos::prelude::create_callback(f)
}

/// Version-agnostic callback creation with no parameters
pub fn create_callback_0<F>(f: F) -> Callback<()>
where
    F: Fn() + 'static,
{
    // Use the current Leptos API directly
    leptos::prelude::create_callback(move |_: ()| f())
}

/// Version-agnostic callback creation with event parameter
pub fn create_event_callback<T, F>(f: F) -> Callback<T>
where
    T: Clone + 'static,
    F: Fn(T) + 'static,
{
    leptos::prelude::create_callback(f)
}

/// Version-agnostic callback invocation
pub fn call_callback<T>(callback: &Callback<T>, value: T)
where
    T: Clone + 'static,
{
    // Use the current Leptos API directly - call the callback as a function
    callback.call(value)
}

/// Version-agnostic callback invocation with no parameters
pub fn call_callback_0(callback: &Callback<()>) {
    call_callback(callback, ())
}

/// Version-agnostic callback cloning
pub fn clone_callback<T>(callback: &Callback<T>) -> Callback<T>
where
    T: Clone + 'static,
{
    callback.clone()
}

/// Version-agnostic callback composition
pub fn compose_callbacks<T, U, V, F, G>(
    _first: Callback<T>,
    second: Callback<U>,
    composer: F,
) -> Callback<T>
where
    T: Clone + 'static,
    U: Clone + 'static,
    V: Clone + 'static,
    F: Fn(T) -> U + 'static,
    G: Fn(U) -> V + 'static,
{
    leptos::prelude::create_callback(move |value: T| {
        let intermediate = composer(value);
        call_callback(&second, intermediate);
    })
}

/// Version-agnostic callback chaining
pub fn chain_callbacks<T>(
    callbacks: Vec<Callback<T>>,
) -> Callback<T>
where
    T: Clone + 'static,
{
    leptos::prelude::create_callback(move |value: T| {
        for callback in &callbacks {
            call_callback(callback, value.clone());
        }
    })
}

/// Version-agnostic callback with error handling
pub fn create_callback_with_error<T, E, F>(
    f: F,
) -> Callback<Result<T, E>>
where
    T: Clone + 'static,
    E: Clone + 'static,
    F: Fn(T) + 'static,
{
    create_callback(move |result: Result<T, E>| {
        if let Ok(value) = result {
            f(value);
        }
    })
}

/// Version-agnostic callback with optional value
pub fn create_callback_with_option<T, F>(
    f: F,
) -> Callback<Option<T>>
where
    T: Clone + 'static,
    F: Fn(T) + 'static,
{
    create_callback(move |option: Option<T>| {
        if let Some(value) = option {
            f(value);
        }
    })
}

/// Version-agnostic callback with debouncing
pub fn create_debounced_callback<T, F>(
    f: F,
    delay_ms: u32,
) -> Callback<T>
where
    T: Clone + 'static,
    F: Fn(T) + 'static,
{
    // In a real implementation, you'd use a timer here
    // For now, we'll just pass through the callback
    create_callback(f)
}

/// Version-agnostic callback with throttling
pub fn create_throttled_callback<T, F>(
    f: F,
    interval_ms: u32,
) -> Callback<T>
where
    T: Clone + 'static,
    F: Fn(T) + 'static,
{
    // In a real implementation, you'd use a timer here
    // For now, we'll just pass through the callback
    create_callback(f)
}

/// Version-agnostic callback with signal integration
pub fn create_signal_callback<T, U, F>(
    signal: ReadSignal<T>,
    f: F,
) -> Callback<U>
where
    T: Clone + PartialEq + Send + Sync + 'static,
    U: Clone + 'static,
    F: Fn(T, U) + 'static,
{
    create_callback(move |value: U| {
        let signal_value = signal.get();
        f(signal_value, value);
    })
}

/// Version-agnostic callback with memo integration
pub fn create_memo_callback<T, U, F>(
    memo: Memo<T>,
    f: F,
) -> Callback<U>
where
    T: Clone + PartialEq + Send + Sync + 'static,
    U: Clone + 'static,
    F: Fn(T, U) + 'static,
{
    create_callback(move |value: U| {
        let memo_value = memo.get();
        f(memo_value, value);
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_callback() {
        let callback = create_callback(|value: i32| {
            assert_eq!(value, 42);
        });
        
        call_callback(&callback, 42);
    }
    
    #[test]
    fn test_create_callback_0() {
        let callback = create_callback_0(|| {
            // This should be called
        });
        
        call_callback_0(&callback);
    }
    
    #[test]
    fn test_chain_callbacks() {
        let mut call_count = 0;
        
        let callback1 = create_callback(move |_: i32| {
            call_count += 1;
        });
        
        let callback2 = create_callback(move |_: i32| {
            call_count += 1;
        });
        
        let chained = chain_callbacks(vec![callback1, callback2]);
        call_callback(&chained, 42);
        
        assert_eq!(call_count, 2);
    }
    
    #[test]
    fn test_create_callback_with_option() {
        let callback = create_callback_with_option(|value: i32| {
            assert_eq!(value, 42);
        });
        
        call_callback(&callback, Some(42));
        call_callback(&callback, None); // Should not panic
    }
}
