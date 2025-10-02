//! # Leptos State - Advanced State Management for Leptos
//!
//! Leptos State provides powerful state machines and reactive stores for Leptos applications,
//! inspired by XState and Zustand.
//!
//! ## Quick Start
//!
//! ```rust
//! use leptos_state::*;
//!
//! // Define your context and events
//! #[derive(Clone, Debug, PartialEq)]
//! struct AppContext {
//!     user: Option<String>,
//!     is_loading: bool,
//! }
//!
//! #[derive(Clone, Debug, PartialEq)]
//! enum AppEvent {
//!     Login(String),
//!     Logout,
//!     SetLoading(bool),
//! }
//!
//! // Create a state machine
//! let machine = Machine::builder()
//!     .state("idle", |s| s.on(AppEvent::Login(_), "loading"))
//!     .state("loading", |s| s.on(AppEvent::SetLoading(false), "authenticated"))
//!     .state("authenticated", |s| s.on(AppEvent::Logout, "idle"))
//!     .build("idle");
//! ```
//!
//! ## Understanding Trait Bounds
//!
//! Leptos State requires certain traits for important functionality:
//!
//! - **`Debug`**: Error reporting, logging, and debugging
//! - **`PartialEq`**: State comparison and change detection
//! - **`Clone`**: Creating state snapshots and copies
//! - **`Send + Sync`**: Thread-safe operations in async contexts
//!
//! ## Usage Patterns
//!
//! ### Basic Usage (Most Common)
//!
//! For standard Rust types, just derive the required traits:
//!
//! ```rust
//! #[derive(Clone, Debug, PartialEq)]
//! struct MyContext { /* fields */ }
//!
//! #[derive(Clone, Debug, PartialEq)]
//! enum MyEvent { /* variants */ }
//! ```
//!
//! ### Working with Complex Types
//!
//! For types that can't implement traits directly, use newtype wrappers:
//!
//! ```rust
//! // If you have a complex type
//! struct ComplexData { /* fields */ }
//!
//! // Create a wrapper
//! #[derive(Clone, Debug, PartialEq)]
//! struct WrappedData(ComplexData);
//!
//! // Implement Deref/DerefMut for convenience
//! impl std::ops::Deref for WrappedData {
//!     type Target = ComplexData;
//!     fn deref(&self) -> &ComplexData { &self.0 }
//! }
//!
//! // Now use WrappedData in your state machines
//! #[derive(Clone, Debug, PartialEq)]
//! struct AppState {
//!     data: WrappedData,
//! }
//! ```
//!
//! ### Why These Bounds Are Required
//!
//! Phase 8 analysis revealed that these bounds serve critical purposes:
//!
//! - **Change Detection**: `PartialEq` enables efficient reactivity
//! - **Error Reporting**: `Debug` provides meaningful error messages
//! - **Performance**: Bounds enable optimizations like deduplication
//! - **Thread Safety**: `Send + Sync` ensure async compatibility
//!
//! ## Feature Flags
//!
//! - `serde` (default): Serialization support
//! - `yaml`: YAML serialization
//! - `debug_bounds`: Enhanced debugging features
//! - `full`: All features enabled
//!
//! ## Architecture
//!
//! - **State Machines**: XState-inspired finite state machines with guards and actions
//! - **Reactive Stores**: Zustand-like global state with middleware
//! - **Performance Monitoring**: Built-in metrics and bottleneck detection
//! - **Visualization**: DOT and Mermaid diagram generation
//! - **Testing**: Property-based testing utilities

#![allow(clippy::type_complexity)]
#![allow(clippy::useless_format)]
#![allow(clippy::single_char_add_str)]
#![allow(clippy::redundant_pattern_matching)]
#![allow(clippy::format_in_format_args)]
#![allow(clippy::new_without_default)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::module_inception)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::should_implement_trait)]
#![allow(dead_code)]
#![allow(clippy::assertions_on_constants)]
#![allow(clippy::bool_assert_comparison)]
#![allow(clippy::absurd_extreme_comparisons)]
#![allow(unused_comparisons)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::manual_map)]
#![allow(clippy::clone_on_copy)]
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
    create_store, LoggerMiddleware, MiddlewareChain, Store, StoreContext, StoreSlice,
    ValidationMiddleware,
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

