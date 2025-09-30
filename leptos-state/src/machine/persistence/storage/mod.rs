//! Storage backends for machine persistence

pub mod core;
pub mod local;
pub mod memory;
pub mod filesystem;
pub mod factory;

// Re-export the most commonly used items
pub use core::{MachineStorage, StorageInfo};
pub use local::LocalStorage;
pub use memory::MemoryStorage;
pub use filesystem::FileSystemStorage;
pub use factory::{StorageFactory, StorageConfig};
