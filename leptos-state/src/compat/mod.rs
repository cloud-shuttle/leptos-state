//! # Compatibility Layer
//!
//! This module provides a simplified compatibility layer that re-exports
//! the most commonly used Leptos functions directly from the prelude.
//! This approach avoids the complexity of version-specific APIs and
//! provides a stable interface for users.

// Re-export the most commonly used Leptos functions directly
pub use leptos::prelude::{
    memo,
    // View and mounting
    mount_to_body,
    // Context
    provide_context,
    // Signals
    signal,
    use_context,

    view,
    // Callbacks
    Callback,

    Effect,

    Memo,
    // Types
    ReadSignal,
    RwSignal,
    View,

    WriteSignal,
};

// Re-export our own types
pub use super::{
    hooks::{use_machine, use_store as use_store_hook},
    machine::{Action, Event, Guard, Machine, MachineBuilder, MachineState, Transition},
    store::{use_store_slice, Store, StoreContext, StoreSlice},
    utils::types::{StateError, StateResult},
};
