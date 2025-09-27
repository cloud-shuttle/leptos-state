//! # Leptos State Management Library
//!
//! A state management library for Leptos applications inspired by Zustand's simplicity
//! and XState's state machine capabilities.
//!
//! ## Features
//!
//! - **Store Management**: Zustand-inspired stores with reactive updates
//! - **State Machines**: XState-inspired finite state machines
//! - **Leptos Integration**: First-class support for Leptos reactive primitives
//! - **TypeScript-like DX**: Ergonomic APIs with strong type safety
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use leptos::*;
//! use leptos_state::*;
//!
//! // Create a store
//! #[derive(Clone, PartialEq)]
//! struct AppState {
//!     count: i32,
//! }
//!
//! create_store!(AppStore, AppState, AppState { count: 0 });
//!
//! // Use in components
//! #[component]
//! fn Counter() -> impl IntoView {
//!     let (state, set_state) = use_store::<AppStore>();
//!     
//!     view! {
//!         <button on:click=move |_| set_state.update(|s| s.count += 1)>
//!             "Count: " {move || state.get().count}
//!         </button>
//!     }
//! }
//! ```

pub mod compat;
pub mod hooks;
pub mod machine;
pub mod store;
pub mod utils;

// Re-export commonly used items
// Store types
pub use store::{
    create_computed, create_store, provide_store, use_store_slice, LoggerMiddleware,
    MiddlewareChain, Store, StoreContext, StoreSlice, ValidationMiddleware,
};
// Machine types
pub use machine::{Machine, MachineBuilder, MachineState, StateMachine};
// Hook types
pub use hooks::{
    use_machine, use_machine_history, use_machine_with_instance, use_store, use_store_with_actions,
};
// Utility types
pub use utils::{LogLevel, StateError, StateResult};
// Compatibility layer
pub use compat::*;
