//! # View Compatibility Layer
//!
//! Provides version-agnostic APIs for Leptos view mounting and rendering across different versions.
//! This layer uses direct imports for better reliability.

use leptos::prelude::*;
use super::{CompatResult, CompatError};

/// Version-agnostic view macro re-export
/// This is a simple re-export since the view! macro is generally stable
pub use leptos::view;

/// Version-agnostic mount to body
pub fn mount_to_body<F>(f: F)
where
    F: Fn() -> View<()> + 'static,
{
    leptos::mount_to_body(f)
}

/// Version-agnostic mount to element
pub fn mount_to_element<F>(target: &str, f: F)
where
    F: Fn() -> View<()> + 'static,
{
    // Use the current Leptos API directly - this function exists in the prelude
    leptos::prelude::mount_to_element(target, f)
}

/// Version-agnostic mount to DOM element
pub fn mount_to_dom_element<F>(element: &web_sys::Element, f: F)
where
    F: Fn() -> View<()> + 'static,
{
    // Use the current Leptos API directly - this function exists in the prelude
    leptos::prelude::mount_to_dom_element(element, f)
}

/// Version-agnostic render to string
pub fn render_to_string<F>(f: F) -> String
where
    F: Fn() -> View<()> + 'static,
{
    leptos::render_to_string(f)
}

/// Version-agnostic render to string with context
pub fn render_to_string_with_context<F>(f: F) -> String
where
    F: Fn() -> View<()> + 'static,
{
    leptos::render_to_string_with_context(f)
}

/// Version-agnostic render to writer
pub fn render_to_writer<F, W>(f: F, writer: W) -> Result<(), std::io::Error>
where
    F: Fn() -> View<()> + 'static,
    W: std::io::Write,
{
    leptos::render_to_writer(f, writer)
}

/// Version-agnostic render to writer with context
pub fn render_to_writer_with_context<F, W>(f: F, writer: W) -> Result<(), std::io::Error>
where
    F: Fn() -> View<()> + 'static,
    W: std::io::Write,
{
    leptos::render_to_writer_with_context(f, writer)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mount_to_body() {
        // This test would need to run in a browser environment
        // For now, we'll just test that the function compiles
        let _ = mount_to_body::<fn() -> leptos::View>;
    }
    
    #[test]
    fn test_mount_to_element() {
        // This test would need to run in a browser environment
        // For now, we'll just test that the function compiles
        let _ = mount_to_element::<fn() -> leptos::View>;
    }
    
    #[test]
    fn test_render_to_string() {
        // This test would need to run in a Leptos component context
        // For now, we'll just test that the function compiles
        let _ = render_to_string::<fn() -> leptos::View>;
    }
}
