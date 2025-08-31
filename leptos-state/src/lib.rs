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

pub mod store;
pub mod machine;
pub mod hooks;
pub mod utils;

// Re-export commonly used items
// Store types
pub use store::{Store, StoreSlice, use_store_slice, StoreContext};
// Machine types
pub use machine::{StateMachine, MachineState, MachineBuilder, Machine};
// Hook types
pub use hooks::{use_machine, use_machine_history};
// Utility types
pub use utils::{StateResult, StateError, LogLevel};