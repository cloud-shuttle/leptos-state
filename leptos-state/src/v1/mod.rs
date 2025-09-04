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

pub mod traits;
pub mod machine;
pub mod store;
pub mod builder;
pub mod error;
pub mod state;
pub mod event;
pub mod context;
pub mod devtools;
pub mod performance;

// Re-export main types for easy access
pub use traits::*;
pub use machine::*;
pub use store::*;
pub use builder::*;
pub use error::*;
pub use state::*;
pub use event::*;
pub use context::*;
pub use devtools::*;
pub use performance::*;

#[cfg(test)]
mod tests {


    #[test]
    fn test_module_compilation() {
        // Basic test to ensure the module compiles
        assert!(true);
    }
}
