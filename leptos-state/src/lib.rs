//! # leptos-state v1.0.0 - Redesigned Architecture
//!
//! This module contains the completely redesigned architecture that fixes
//! the fundamental type system issues present in v0.2.x.
//!
//! ## Design Philosophy
//!
//! 1. **Trait-first design** with proper bounds
//! 2. **Feature flags that actually work** independently and together
//! 3. **Zero-cost abstractions** where possible
//! 4. **WASM-first but native-compatible**
//! 5. **Leptos v0.8+ integration** from day one

pub mod v1;
pub mod hooks;
pub mod machine;
pub mod store;
pub mod utils;

// Re-export main types for easy access
pub use v1::*;
pub use hooks::*;
pub use machine::*;
