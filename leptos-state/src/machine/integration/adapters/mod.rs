//! Modular integration adapters
//!
//! This module provides various integration adapter implementations
//! that can connect to different external systems for event handling.

pub mod traits;
pub mod http;
pub mod database;
pub mod message_queue;
pub mod file_system;
pub mod websocket;

// Re-export commonly used types for convenience
pub use traits::{
    IntegrationAdapterTrait, AdapterType, AdapterStats, ConnectionConfig,
    AdapterFeature, BoxedAdapter, boxed
};
pub use http::HttpApiAdapter;
pub use database::DatabaseAdapter;
pub use message_queue::MessageQueueAdapter;
pub use file_system::{FileSystemAdapter, FileFormat};
pub use websocket::WebSocketAdapter;

// Legacy aliases for backward compatibility
pub type RestApiAdapter = HttpApiAdapter;
