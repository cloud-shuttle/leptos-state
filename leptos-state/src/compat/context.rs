//! # Context Compatibility Layer
//!
//! Provides version-agnostic APIs for Leptos context across different versions.
//! This layer uses direct imports for better reliability.

use leptos::prelude::*;
use super::{CompatResult, CompatError};

// Direct imports for better reliability
use leptos::{
    signal, memo, effect, create_callback, create_local_resource,
};

/// Version-agnostic context provision
pub fn provide_context<T>(value: T)
where
    T: Clone + 'static,
{
    // Use the current Leptos API directly
    leptos::provide_context(value)
}

/// Version-agnostic context consumption
pub fn use_context<T>() -> Option<T>
where
    T: Clone + 'static,
{
    // Use the current Leptos API directly
    leptos::use_context::<T>()
}

/// Version-agnostic context consumption with default
pub fn use_context_with_default<T>(default: T) -> T
where
    T: Clone + 'static,
{
    use_context::<T>().unwrap_or(default)
}

/// Version-agnostic context consumption with signal
pub fn use_context_signal<T>() -> ReadSignal<Option<T>>
where
    T: Clone + Send + Sync + 'static,
{
    // Use the current Leptos API directly
    signal(use_context::<T>()).0
}

/// Create a memo that uses a single context value
pub fn use_context_memo<T, U, F>(f: F) -> Memo<Option<U>>
where
    T: Clone + Send + Sync + 'static,
    U: Clone + Send + Sync + PartialEq + 'static,
    F: Fn(&T) -> U + Send + Sync + 'static,
{
    // Use the current Leptos API directly
    let context = use_context::<T>();
    match context {
        Some(context) => memo!(move || Some(f(&context))),
        None => memo!(move || None::<U>),
    }
}

/// Create a memo that uses multiple context values
pub fn use_contexts_memo<T1, T2, U, F>(f: F) -> Memo<Option<U>>
where
    T1: Clone + Send + Sync + 'static,
    T2: Clone + Send + Sync + 'static,
    U: Clone + Send + Sync + PartialEq + 'static,
    F: Fn(&T1, &T2) -> U + Send + Sync + 'static,
{
    // Use the current Leptos API directly
    let context1 = use_context::<T1>();
    let context2 = use_context::<T2>();
    match (context1, context2) {
        (Some(context1), Some(context2)) => memo!(move || Some(f(&context1, &context2))),
        _ => memo!(move || None::<U>),
    }
}

/// Create an effect that uses a single context value
pub fn use_context_effect<T, F>(f: F)
where
    T: Clone + Send + Sync + 'static,
    F: Fn(&T) + Send + Sync + 'static,
{
    // Use the current Leptos API directly
    let context = use_context::<T>();
    if let Some(context) = context {
        effect(move || f(&context));
    }
}

/// Create an effect that uses multiple context values
pub fn use_contexts_effect<T1, T2, F>(f: F)
where
    T1: Clone + Send + Sync + 'static,
    T2: Clone + Send + Sync + 'static,
    F: Fn(&T1, &T2) + Send + Sync + 'static,
{
    // Use the current Leptos API directly
    let context1 = use_context::<T1>();
    let context2 = use_context::<T2>();
    if let (Some(context1), Some(context2)) = (context1, context2) {
        effect(move || f(&context1, &context2));
    }
}

/// Create a callback that uses a single context value
pub fn use_context_callback<T, U, F>(f: F) -> Callback<U>
where
    T: Clone + Send + Sync + 'static,
    U: Clone + 'static,
    F: Fn(&T, U) + Send + Sync + 'static,
{
    // Use the current Leptos API directly
    let context = use_context::<T>();
    match context {
        Some(context) => create_callback(move |value| f(&context, value)),
        None => create_callback(move |_| {}),
    }
}

/// Create a callback that uses multiple context values
pub fn use_contexts_callback<T1, T2, U, F>(f: F) -> Callback<U>
where
    T1: Clone + Send + Sync + 'static,
    T2: Clone + Send + Sync + 'static,
    U: Clone + 'static,
    F: Fn(&T1, &T2, U) + Send + Sync + 'static,
{
    // Use the current Leptos API directly
    let context1 = use_context::<T1>();
    let context2 = use_context::<T2>();
    match (context1, context2) {
        (Some(context1), Some(context2)) => create_callback(move |value| f(&context1, &context2, value)),
        _ => create_callback(move |_| {}),
    }
}

/// Create a simple callback that uses a single context value
pub fn use_context_callback_simple<T, F>(f: F) -> Callback<()>
where
    T: Clone + Send + Sync + Default + 'static,
    F: Fn(&T) + Send + Sync + 'static,
{
    // Use the current Leptos API directly
    let context = use_context::<T>().unwrap_or_default();
    create_callback(move |_| f(&context))
}

/// Create a simple callback that uses multiple context values
pub fn use_contexts_callback_simple<T1, T2, F>(f: F) -> Callback<()>
where
    T1: Clone + Send + Sync + Default + 'static,
    T2: Clone + Send + Sync + Default + 'static,
    F: Fn(&T1, &T2) + Send + Sync + 'static,
{
    // Use the current Leptos API directly
    let context1 = use_context::<T1>().unwrap_or_default();
    let context2 = use_context::<T2>().unwrap_or_default();
    create_callback(move |_| f(&context1, &context2))
}

/// Create a resource that uses a single context value
pub fn use_context_resource<T, U, F>(f: F) -> Resource<(), U>
where
    T: Clone + Send + Sync + 'static,
    U: Clone + Send + Sync + 'static,
    F: Fn(&T) -> U + Send + Sync + 'static,
{
    // Use the current Leptos API directly
    let context = use_context::<T>();
    match context {
        Some(context) => create_local_resource(move || (), move |_| f(&context)),
        None => create_local_resource(move || (), move |_| panic!("Context not available")),
    }
}

/// Create a resource that uses multiple context values
pub fn use_contexts_resource<T1, T2, U, F>(f: F) -> Resource<(), U>
where
    T1: Clone + Send + Sync + 'static,
    T2: Clone + Send + Sync + 'static,
    U: Clone + Send + Sync + 'static,
    F: Fn(&T1, &T2) -> U + Send + Sync + 'static,
{
    // Use the current Leptos API directly
    let context1 = use_context::<T1>();
    let context2 = use_context::<T2>();
    match (context1, context2) {
        (Some(context1), Some(context2)) => create_local_resource(move || (), move |_| f(&context1, &context2)),
        _ => create_local_resource(move || (), move |_| panic!("Contexts not available")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_provide_and_use_context() {
        // This test would need to run in a Leptos component context
        // For now, we'll just test that the functions compile
        let _ = provide_context::<i32>;
        let _ = use_context::<i32>;
    }
    
    #[test]
    fn test_use_context_with_default() {
        // This test would need to run in a Leptos component context
        // For now, we'll just test that the function compiles
        let _ = use_context_with_default::<i32>;
    }
}
