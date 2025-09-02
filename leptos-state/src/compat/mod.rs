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
    machine::{Machine, MachineBuilder, MachineState, Transition, Event, Action, Guard},
    store::{Store, StoreSlice, StoreContext, use_store_slice},
    hooks::{use_machine, use_store as use_store_hook},
    utils::types::{StateResult, StateError},
};
