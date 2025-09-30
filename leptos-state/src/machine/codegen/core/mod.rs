//! Core code generation functionality

pub mod generator;
pub mod structure;
pub mod transitions;
pub mod guards_actions;
pub mod tests;
pub mod files;
pub mod stats;

// Re-export the most commonly used items
pub use generator::CodeGenerator;
pub use stats::GenerationStats;
