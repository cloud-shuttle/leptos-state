//! DevTools integration for debugging and time travel

use super::Store;
use crate::utils::{StateError, StateResult};
use serde_json::Value;
use std::collections::VecDeque;
use std::marker::PhantomData;

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

/// Time travel debugging support
pub struct TimeTravel<S: Store> {
    history: VecDeque<Snapshot<S::State>>,
    current_index: usize,
    max_history: usize,
    _phantom: PhantomData<S>,
}

/// State snapshot for time travel
#[derive(Debug, Clone)]
pub struct Snapshot<T> {
    pub state: T,
    pub action: String,
    pub timestamp: u64,
}

impl<T> Snapshot<T> {
    pub fn new(state: T, action: impl Into<String>) -> Self {
        Self {
            state,
            action: action.into(),
            timestamp: js_sys::Date::now() as u64,
        }
    }
}

impl<S: Store> TimeTravel<S> {
    pub fn new() -> Self {
        Self {
            history: VecDeque::new(),
            current_index: 0,
            max_history: 50, // Default to 50 snapshots
            _phantom: PhantomData,
        }
    }

    pub fn with_max_history(max_history: usize) -> Self {
        Self {
            history: VecDeque::new(),
            current_index: 0,
            max_history,
            _phantom: PhantomData,
        }
    }

    /// Record a new state snapshot
    pub fn record(&mut self, state: S::State, action: impl Into<String>) {
        let snapshot = Snapshot::new(state, action);

        // Remove any future history when recording new action
        self.history.truncate(self.current_index + 1);

        // Add new snapshot
        self.history.push_back(snapshot);

        // Remove old snapshots if we exceed max history
        if self.history.len() > self.max_history {
            self.history.pop_front();
        } else {
            self.current_index += 1;
        }
    }

    /// Undo to previous state
    pub fn undo(&mut self) -> Option<&S::State> {
        if self.can_undo() {
            self.current_index -= 1;
            self.history.get(self.current_index).map(|s| &s.state)
        } else {
            None
        }
    }

    /// Redo to next state
    pub fn redo(&mut self) -> Option<&S::State> {
        if self.can_redo() {
            self.current_index += 1;
            self.history.get(self.current_index).map(|s| &s.state)
        } else {
            None
        }
    }

    /// Jump to specific snapshot
    pub fn jump_to(&mut self, index: usize) -> Option<&S::State> {
        if index < self.history.len() {
            self.current_index = index;
            self.history.get(self.current_index).map(|s| &s.state)
        } else {
            None
        }
    }

    /// Check if undo is possible
    pub fn can_undo(&self) -> bool {
        self.current_index > 0
    }

    /// Check if redo is possible
    pub fn can_redo(&self) -> bool {
        self.current_index < self.history.len().saturating_sub(1)
    }

    /// Get current snapshot
    pub fn current(&self) -> Option<&Snapshot<S::State>> {
        self.history.get(self.current_index)
    }

    /// Get all snapshots
    pub fn snapshots(&self) -> impl Iterator<Item = &Snapshot<S::State>> {
        self.history.iter()
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.history.clear();
        self.current_index = 0;
    }
}

impl<S: Store> Default for TimeTravel<S> {
    fn default() -> Self {
        Self::new()
    }
}

/// WebSocket-based DevTools connector
#[cfg(target_arch = "wasm32")]
pub struct WebSocketConnector {
    url: String,
}

#[cfg(target_arch = "wasm32")]
impl WebSocketConnector {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}

#[cfg(target_arch = "wasm32")]
impl DevToolsConnector for WebSocketConnector {
    fn connect(&self) -> StateResult<DevToolsConnection> {
        // Placeholder for WebSocket connection
        tracing::info!("Connecting to DevTools at {}", self.url);
        Ok(DevToolsConnection::new(uuid::Uuid::new_v4().to_string()))
    }

    fn send_update(&self, update: StateUpdate) {
        // Placeholder for sending update via WebSocket
        tracing::debug!("DevTools update: {:?}", update.action_type);
    }

    fn is_available(&self) -> bool {
        // Check if WebSocket API is available
        true
    }
}

/// Console-based DevTools connector for debugging
pub struct ConsoleConnector {
    enabled: bool,
}

impl ConsoleConnector {
    pub fn new() -> Self {
        Self {
            enabled: cfg!(debug_assertions),
        }
    }

    pub fn with_enabled(enabled: bool) -> Self {
        Self { enabled }
    }
}

impl Default for ConsoleConnector {
    fn default() -> Self {
        Self::new()
    }
}

impl DevToolsConnector for ConsoleConnector {
    fn connect(&self) -> StateResult<DevToolsConnection> {
        tracing::info!("Console DevTools connector enabled");
        Ok(DevToolsConnection::new("console".to_string()))
    }

    fn send_update(&self, update: StateUpdate) {
        if self.enabled {
            web_sys::console::group_1(&format!("🔧 State Update: {}", update.action_type).into());
            // Convert serde_json::Value to JsValue for console logging
            let before_js = serde_json::to_string(&update.state_before)
                .map(|s| wasm_bindgen::JsValue::from_str(&s))
                .unwrap_or_else(|_| wasm_bindgen::JsValue::from_str("Error serializing state"));
            let after_js = serde_json::to_string(&update.state_after)
                .map(|s| wasm_bindgen::JsValue::from_str(&s))
                .unwrap_or_else(|_| wasm_bindgen::JsValue::from_str("Error serializing state"));
            let payload_js = serde_json::to_string(&update.payload)
                .map(|s| wasm_bindgen::JsValue::from_str(&s))
                .unwrap_or_else(|_| wasm_bindgen::JsValue::from_str("Error serializing payload"));

            web_sys::console::log_2(&"Before:".into(), &before_js);
            web_sys::console::log_2(&"After:".into(), &after_js);
            web_sys::console::log_2(&"Payload:".into(), &payload_js);
            web_sys::console::group_end();
        }
    }

    fn is_available(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create_store;

    #[derive(Clone, PartialEq, Debug)]
    struct TestState {
        count: i32,
    }

    create_store!(TestStore, TestState, TestState { count: 0 });

    #[test]
    fn time_travel_recording() {
        let mut time_travel = TimeTravel::<TestStore>::new();

        let state1 = TestState { count: 0 };
        let state2 = TestState { count: 1 };
        let state3 = TestState { count: 2 };

        time_travel.record(state1.clone(), "init");
        time_travel.record(state2.clone(), "increment");
        time_travel.record(state3.clone(), "increment");

        assert_eq!(time_travel.snapshots().count(), 3);
        assert_eq!(time_travel.current().unwrap().state.count, 2);
    }

    #[test]
    fn time_travel_undo_redo() {
        let mut time_travel = TimeTravel::<TestStore>::new();

        let state1 = TestState { count: 0 };
        let state2 = TestState { count: 1 };

        time_travel.record(state1.clone(), "init");
        time_travel.record(state2.clone(), "increment");

        assert!(time_travel.can_undo());
        let undone = time_travel.undo().unwrap();
        assert_eq!(undone.count, 0);

        assert!(time_travel.can_redo());
        let redone = time_travel.redo().unwrap();
        assert_eq!(redone.count, 1);
    }

    #[test]
    fn console_connector_creation() {
        let connector = ConsoleConnector::new();
        assert!(connector.is_available() == cfg!(debug_assertions));

        let connection = connector.connect();
        assert!(connection.is_ok());
    }
}
