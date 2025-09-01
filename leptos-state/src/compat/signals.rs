//! # Signals Compatibility Layer
//!
//! Provides version-agnostic APIs for Leptos signals across different versions.
//! This layer uses direct imports for better reliability.

use leptos::prelude::*;
use super::{CompatResult, CompatError};

/// Version-agnostic signal creation
pub fn create_signal<T>(initial: T) -> (ReadSignal<T>, WriteSignal<T>)
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    // Use the current Leptos API directly
    leptos::prelude::create_signal(initial)
}

/// Version-agnostic read-write signal creation
pub fn create_rw_signal<T>(initial: T) -> RwSignal<T>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    // Use the current Leptos API directly
    leptos::prelude::signal(initial)
}

/// Version-agnostic memo creation
pub fn create_memo<T, F>(f: F) -> Memo<T>
where
    T: Clone + PartialEq + Send + Sync + 'static,
    F: Fn() -> T + 'static,
{
    // Use the current Leptos API directly
    leptos::prelude::memo!(f)
}

/// Version-agnostic effect creation
pub fn create_effect<F>(f: F)
where
    F: Fn() + 'static,
{
    // Use the current Leptos API directly
    leptos::prelude::create_effect(f)
}

/// Version-agnostic resource creation
pub fn create_resource<T: Send + Sync, U: Send + Sync, F>(
    source: Signal<T>,
    fetcher: F,
) -> Resource<T, U>
where
    T: Clone + PartialEq + 'static + Send + Sync,
    U: Clone + PartialEq + 'static + Send + Sync,
    F: Fn(T) -> futures::future::BoxFuture<'static, U> + Send + Sync + 'static,
{
    // Use the current Leptos API directly - create_resource exists in 0.7
    leptos::prelude::create_resource(source, fetcher)
}

/// Version-agnostic signal conversion
pub fn signal<T>(initial: T) -> RwSignal<T>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    // Use the current Leptos API directly
    leptos::prelude::signal(initial)
}

/// Version-agnostic signal splitting
pub fn split_signal<T, U, F>(
    signal: ReadSignal<T>,
    splitter: F,
) -> (ReadSignal<U>, WriteSignal<U>)
where
    T: Clone + PartialEq + Send + Sync + 'static,
    U: Clone + PartialEq + Send + Sync + 'static,
    F: Fn(&T) -> U + Send + Sync + 'static,
{
    let (read, write) = create_signal(splitter(&signal.get()));
    
    create_effect(move || {
        let new_value = splitter(&signal.get());
        write.set(new_value);
    });
    
    (read, write)
}

/// Version-agnostic signal mapping
pub fn map_signal<T, U, F>(
    signal: ReadSignal<T>,
    mapper: F,
) -> ReadSignal<U>
where
    T: Clone + PartialEq + Send + Sync + 'static,
    U: Clone + PartialEq + Send + Sync + 'static,
    F: Fn(&T) -> U + Send + Sync + 'static,
{
    let (mapped, set_mapped) = create_signal(mapper(&signal.get()));
    
    create_effect(move || {
        let value = signal.get();
        let new_value = mapper(&value);
        set_mapped.set(new_value);
    });
    
    mapped
}

/// Version-agnostic signal filtering
pub fn filter_signal<T, F>(
    signal: ReadSignal<T>,
    predicate: F,
) -> ReadSignal<Option<T>>
where
    T: Clone + PartialEq + Send + Sync + 'static,
    F: Fn(&T) -> bool + Send + Sync + 'static,
{
    let (filtered, set_filtered) = create_signal({
        let value = signal.get();
        if predicate(&value) {
            Some(value)
        } else {
            None
        }
    });
    
    create_effect(move || {
        let value = signal.get();
        let new_value = if predicate(&value) {
            Some(value)
        } else {
            None
        };
        set_filtered.set(new_value);
    });
    
    filtered
}

/// Version-agnostic signal debouncing
pub fn debounce_signal<T>(
    signal: ReadSignal<T>,
    delay_ms: u32,
) -> ReadSignal<T>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    let (debounced, set_debounced) = create_signal(signal.get());
    
    create_effect(move || {
        let value = signal.get();
        let setter = set_debounced.clone();
        
        // In a real implementation, you'd use a timer here
        // For now, we'll just pass through the value
        setter.set(value);
    });
    
    debounced
}

/// Version-agnostic signal throttling
pub fn throttle_signal<T>(
    signal: ReadSignal<T>,
    interval_ms: u32,
) -> ReadSignal<T>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    let (throttled, set_throttled) = create_signal(signal.get());
    
    create_effect(move || {
        let value = signal.get();
        let setter = set_throttled.clone();
        
        // In a real implementation, you'd use a timer here
        // For now, we'll just pass through the value
        setter.set(value);
    });
    
    throttled
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_signal() {
        let (read, write) = create_signal(42);
        assert_eq!(read.get(), 42);
        
        write.set(100);
        assert_eq!(read.get(), 100);
    }
    
    #[test]
    fn test_create_rw_signal() {
        let signal = create_rw_signal(42);
        assert_eq!(signal.get(), 42);
        
        signal.set(100);
        assert_eq!(signal.get(), 100);
    }
    
    #[test]
    fn test_create_memo() {
        let (read, _) = create_signal(42);
        let memo = create_memo(move || read.get() * 2);
        
        assert_eq!(memo.get(), 84);
    }
    
    #[test]
    fn test_map_signal() {
        let (read, write) = create_signal(42);
        let mapped = map_signal(read, |x| x * 2);
        
        assert_eq!(mapped.get(), 84);
        
        write.set(100);
        assert_eq!(mapped.get(), 200);
    }
}
