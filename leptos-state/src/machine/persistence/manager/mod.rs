//! Persistence manager and related structures

pub mod core;
pub mod info;
pub mod stats;
pub mod backup;
pub mod utils;

// Re-export the most commonly used items
pub use core::MachinePersistence;
pub use info::MachineInfo;
pub use stats::PersistenceStats;
pub use backup::{BackupManager, BackupEntry};
pub use utils::PersistenceUtils;
