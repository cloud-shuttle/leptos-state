//! # Compatibility Layer
//!
//! This module provides a simplified compatibility layer that re-exports
//! the most commonly used Leptos functions directly from the prelude.
//! This approach avoids the complexity of version-specific APIs and
//! provides a stable interface for users.

// Re-export the most commonly used Leptos functions directly
pub use leptos::prelude::{
    // Signals
    signal, memo, Effect,
    
    // Callbacks
    Callback,
    
    // Context
    provide_context, use_context,
    
    // View and mounting
    mount_to_body, view, View,
    
    // Types
    ReadSignal, WriteSignal, RwSignal, Memo,
};

// Re-export our own types
pub use super::{
    machine::*,
    store::*,
    hooks::*,
    utils::*,
};
