//! Core DevTools types and traits

use super::Store;
use crate::utils::StateResult;
use serde_json::Value;

/// DevTools connector trait for different backends
pub trait DevToolsConnector {
    /// Connect to the DevTools backend
    fn connect(&self) -> StateResult<DevToolsConnection>;

    /// Send a state update to DevTools
    fn send_update(&self, update: StateUpdate);

    /// Check if DevTools are available
    fn is_available(&self) -> bool;
}

/// Connection to DevTools backend
pub struct DevToolsConnection {
    _connection_id: String,
}

impl DevToolsConnection {
    pub fn new(connection_id: String) -> Self {
        Self {
            _connection_id: connection_id,
        }
    }
}

/// State update event for DevTools
#[derive(Debug, Clone)]
pub struct StateUpdate {
    pub action_type: String,
    pub payload: Value,
    pub state_before: Value,
    pub state_after: Value,
    pub timestamp: u64,
}

impl StateUpdate {
    pub fn new(
        action_type: impl Into<String>,
        payload: Value,
        state_before: Value,
        state_after: Value,
    ) -> Self {
        Self {
            action_type: action_type.into(),
            payload,
            state_before,
            state_after,
            timestamp: js_sys::Date::now() as u64,
        }
    }
}
