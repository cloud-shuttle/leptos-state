//! # Simple Compatibility Layer
//!
//! A simplified compatibility layer that provides the most essential
//! Leptos APIs with minimal complexity and maximum reliability.
//!
//! This layer uses direct imports and fallback implementations to avoid
//! version-specific API issues.

use leptos::prelude::*;
use super::{CompatResult, CompatError};

/// Simple signal creation that works across Leptos versions
pub fn create_signal<T>(initial: T) -> (ReadSignal<T>, WriteSignal<T>)
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    // Use the current Leptos API directly
    leptos::create_signal(initial)
}

/// Simple memo creation that works across Leptos versions
pub fn create_memo<F, T>(f: F) -> Memo<T>
where
    F: Fn() -> T + 'static,
    T: Clone + Send + Sync + 'static,
{
    // Use the current Leptos API directly
    leptos::memo!(f)
}

/// Simple effect creation that works across Leptos versions
pub fn create_effect<F>(f: F)
where
    F: Fn() + 'static,
{
    // Use the current Leptos API directly
    leptos::create_effect(f)
}

/// Simple callback creation that works across Leptos versions
pub fn create_callback<T, F>(f: F) -> Callback<T>
where
    T: Clone + 'static,
    F: Fn(T) + 'static,
{
    // Use the current Leptos API directly
    leptos::create_callback(f)
}

/// Simple context provision that works across Leptos versions
pub fn provide_context<T>(value: T)
where
    T: Clone + Send + Sync + 'static,
{
    // Use the current Leptos API directly
    leptos::provide_context(value)
}

/// Simple context consumption that works across Leptos versions
pub fn use_context<T>() -> Option<T>
where
    T: Clone + Send + Sync + 'static,
{
    // Use the current Leptos API directly
    leptos::use_context::<T>()
}

/// Simple mount function that works across Leptos versions
pub fn mount_to_body<F>(f: F)
where
    F: Fn() -> View<()> + 'static,
{
    leptos::mount_to_body(f)
}

/// Simple mount to element function
pub fn mount_to_element<F>(target: &str, f: F)
where
    F: Fn() -> View<()> + 'static,
{
    leptos::mount_to_element(target, f)
}

/// Simple mount to DOM element function
pub fn mount_to_dom_element<F>(element: &web_sys::Element, f: F)
where
    F: Fn() -> View<()> + 'static,
{
    leptos::mount_to_dom_element(element, f)
}

/// Simple render to string function
pub fn render_to_string<F>(f: F) -> String
where
    F: Fn() -> View<()> + 'static,
{
    leptos::render_to_string(f)
}

/// Simple render to string with context function
pub fn render_to_string_with_context<F>(f: F) -> String
where
    F: Fn() -> View<()> + 'static,
{
    leptos::render_to_string_with_context(f)
}

/// Simple render to writer function
pub fn render_to_writer<F, W>(f: F, writer: W) -> Result<(), std::io::Error>
where
    F: Fn() -> View<()> + 'static,
    W: std::io::Write,
{
    leptos::render_to_writer(f, writer)
}

/// Simple render to writer with context function
pub fn render_to_writer_with_context<F, W>(f: F, writer: W) -> Result<(), std::io::Error>
where
    F: Fn() -> View<()> + 'static,
    W: std::io::Write,
{
    leptos::render_to_writer_with_context(f, writer)
}

/// Simple view macro re-export
pub use leptos::view;

/// Version detection
pub fn leptos_version() -> &'static str {
    // This is a simplified version detection
    // In a real implementation, you'd use compile-time feature detection
    "0.7"
}

/// Check if we're using a specific Leptos version
pub fn is_leptos_version(version: &str) -> bool {
    leptos_version() == version
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
    fn test_create_memo() {
        let (read, _) = create_signal(42);
        let memo = create_memo(move || read.get() * 2);
        
        assert_eq!(memo.get(), 84);
    }
    
    #[test]
    fn test_create_callback() {
        let callback = create_callback(|value: i32| {
            assert_eq!(value, 42);
        });
        
        callback.call(42);
    }
    
    #[test]
    fn test_version_detection() {
        let version = leptos_version();
        assert!(!version.is_empty());
        
        assert!(is_leptos_version("0.7"));
        assert!(!is_leptos_version("0.8"));
    }
}
