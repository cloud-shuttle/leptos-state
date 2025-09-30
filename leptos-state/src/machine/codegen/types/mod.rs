//! Code generation types and data structures

pub mod file;
pub mod transitions;
pub mod states;
pub mod events;
pub mod guards;
pub mod actions;
pub mod context;
pub mod machine;

// Re-export the most commonly used items
pub use file::GeneratedFile;
pub use transitions::TransitionInfo;
pub use states::StateGenInfo;
pub use events::EventGenInfo;
pub use guards::{GuardGenInfo, GuardType};
pub use actions::{ActionGenInfo, ActionType};
pub use context::{CodeGenContext, CodeGenOptions, IndentationStyle};
pub use machine::{MachineGenInfo, MachineType};
