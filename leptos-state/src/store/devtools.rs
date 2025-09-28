//! DevTools integration for debugging and time travel
//!
//! The devtools module has been split into specialized modules for better organization:
//! - `devtools_core`: Core traits, connection, and state update types
//! - `devtools_connectors`: Connector implementations (WebSocket, Console)
//! - `devtools_timeline`: Time travel debugging support

// Re-export all devtools functionality from the split modules
pub use super::devtools_core::*;
pub use super::devtools_connectors::*;
pub use super::devtools_timeline::*;
