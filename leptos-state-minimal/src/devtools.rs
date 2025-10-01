//! DevTools integration for browser-based debugging and state inspection
//!
//! This module provides browser DevTools integration for real-time state inspection,
//! debugging, and development workflow enhancement.

use crate::State;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Errors that can occur during DevTools operations
#[derive(Debug, Clone, thiserror::Error)]
pub enum DevToolsError {
    #[error("DevTools not available in this environment")]
    NotAvailable,
    #[error("DevTools bridge setup failed: {message}")]
    BridgeSetupFailed { message: String },
    #[error("Serialization failed: {message}")]
    SerializationFailed { message: String },
    #[error("Time travel operation failed: {message}")]
    TimeTravelFailed { message: String },
    #[error("State inspection failed: {message}")]
    InspectionFailed { message: String },
}

/// Messages sent between Rust and JavaScript DevTools
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum DevToolsMessage {
    /// Initialize DevTools for a store/machine
    Init { name: String, state_type: String },
    /// State has changed
    StateChanged { name: String, state: serde_json::Value, timestamp: u64 },
    /// Operation completed
    OperationCompleted { name: String, operation: String, duration_ms: u64 },
    /// Error occurred
    Error { name: String, error: String, operation: String },
    /// DevTools command from browser
    Command { command: DevToolsCommand },
}

/// Commands that can be sent from browser DevTools
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "command", content = "args")]
pub enum DevToolsCommand {
    /// Inspect current state
    Inspect,
    /// Get state history
    GetHistory { limit: Option<usize> },
    /// Time travel to specific index
    TimeTravel { index: usize },
    /// Reset to initial state
    Reset,
    /// Get performance metrics
    GetMetrics,
}

/// Core DevTools integration
#[cfg(target_arch = "wasm32")]
pub struct DevToolsIntegration<S: State> {
    store_name: String,
    is_enabled: bool,
    message_listeners: Vec<Box<dyn Fn(&DevToolsMessage) + Send + Sync>>,
}

#[cfg(target_arch = "wasm32")]
impl<S: State> DevToolsIntegration<S> {
    /// Create a new DevTools integration
    pub fn new(store_name: String) -> Result<Self, DevToolsError> {
        let is_enabled = Self::is_devtools_available();

        let integration = Self {
            store_name,
            is_enabled,
            message_listeners: Vec::new(),
        };

        if integration.is_enabled {
            integration.setup_devtools_bridge()?;
        }

        Ok(integration)
    }

    /// Check if DevTools are available in the current environment
    pub fn is_devtools_available() -> bool {
        // Check for development environment indicators
        // Look for __LEPTOS_DEVTOOLS__ global or similar
        js_sys::Reflect::get(&web_sys::window().unwrap(), &"__LEPTOS_DEVTOOLS__".into())
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }

    /// Set up the JavaScript bridge for DevTools communication
    fn setup_devtools_bridge(&self) -> Result<(), DevToolsError> {
        // Set up global JavaScript functions for DevTools access
        // This creates console.devtools.state.* APIs

        let window = web_sys::window().unwrap();

        // Create a global leptos devtools object
        let devtools_obj = js_sys::Object::new();
        let state_obj = js_sys::Object::new();

        // Set up console.devtools.state.inspect(storeName)
        let inspect_fn = js_sys::Function::new_with_args("storeName", r#"
            console.log("Leptos DevTools: Inspecting store '" + storeName + "'");
            return { storeName: storeName, message: "Store inspection not yet implemented" };
        "#);

        // Set up console.devtools.state.timeTravel(storeName, index)
        let time_travel_fn = js_sys::Function::new_with_args("storeName, index", r#"
            console.log("Leptos DevTools: Time travel for store '" + storeName + "' to index " + index);
            return { storeName: storeName, index: index, message: "Time travel not yet implemented" };
        "#);

        // Set up console.devtools.state.reset(storeName)
        let reset_fn = js_sys::Function::new_with_args("storeName", r#"
            console.log("Leptos DevTools: Resetting store '" + storeName + "'");
            return { storeName: storeName, message: "Store reset not yet implemented" };
        "#);

        // Set up console.devtools.state.list()
        let list_fn = js_sys::Function::new_no_args(r#"
            console.log("Leptos DevTools: Available stores - listing not yet implemented");
            return { message: "Store listing not yet implemented" };
        "#);

        // Attach functions to state object
        js_sys::Reflect::set(&state_obj, &"inspect".into(), &inspect_fn)
            .map_err(|_| DevToolsError::BridgeSetupFailed {
                message: "Failed to set up inspect function".to_string(),
            })?;

        js_sys::Reflect::set(&state_obj, &"timeTravel".into(), &time_travel_fn)
            .map_err(|_| DevToolsError::BridgeSetupFailed {
                message: "Failed to set up timeTravel function".to_string(),
            })?;

        js_sys::Reflect::set(&state_obj, &"reset".into(), &reset_fn)
            .map_err(|_| DevToolsError::BridgeSetupFailed {
                message: "Failed to set up reset function".to_string(),
            })?;

        js_sys::Reflect::set(&state_obj, &"list".into(), &list_fn)
            .map_err(|_| DevToolsError::BridgeSetupFailed {
                message: "Failed to set up list function".to_string(),
            })?;

        // Attach state object to devtools object
        js_sys::Reflect::set(&devtools_obj, &"state".into(), &state_obj)
            .map_err(|_| DevToolsError::BridgeSetupFailed {
                message: "Failed to set up state object".to_string(),
            })?;

        // Set up console.devtools object
        js_sys::Reflect::set(&window, &"__leptos_devtools".into(), &devtools_obj)
            .map_err(|_| DevToolsError::BridgeSetupFailed {
                message: "Failed to set up DevTools bridge".to_string(),
            })?;

        // Also set up console.devtools for convenience
        if let Ok(console) = js_sys::Reflect::get(&window, &"console".into()) {
            if let Ok(console_obj) = console.dyn_into::<js_sys::Object>() {
                js_sys::Reflect::set(&console_obj, &"devtools".into(), &devtools_obj)
                    .map_err(|_| DevToolsError::BridgeSetupFailed {
                        message: "Failed to set up console.devtools".to_string(),
                    })?;
            }
        }

        // Log that DevTools are ready
        web_sys::console::log_1(&"Leptos DevTools initialized. Use console.devtools.state.* commands".into());

        Ok(())
    }

    /// Send a message to DevTools
    pub fn send_message(&self, message: DevToolsMessage) -> Result<(), DevToolsError> {
        if !self.is_enabled {
            return Ok(());
        }

        // Serialize the message and send it to JavaScript
        let message_json = serde_json::to_string(&message)
            .map_err(|e| DevToolsError::SerializationFailed {
                message: e.to_string(),
            })?;

        // Send to JavaScript DevTools bridge
        // This would call a JavaScript function that handles the message
        self.send_to_javascript(&message_json)?;

        // Notify local listeners
        for listener in &self.message_listeners {
            listener(&message);
        }

        Ok(())
    }

    /// Send JSON message to JavaScript
    fn send_to_javascript(&self, json_message: &str) -> Result<(), DevToolsError> {
        // Call JavaScript function to handle DevTools message
        // This is a placeholder - actual implementation would use web_sys
        // to call a JavaScript function that handles DevTools integration

        // For now, we'll just log to console as a demonstration
        web_sys::console::log_1(&format!("DevTools: {}", json_message).into());

        Ok(())
    }

    /// Add a message listener
    pub fn add_message_listener<F>(&mut self, listener: F)
    where
        F: Fn(&DevToolsMessage) + Send + Sync + 'static,
    {
        self.message_listeners.push(Box::new(listener));
    }

    /// Check if DevTools integration is enabled
    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    /// Get the store name
    pub fn store_name(&self) -> &str {
        &self.store_name
    }
}

/// Fallback implementation for non-WASM targets
#[cfg(not(target_arch = "wasm32"))]
pub struct DevToolsIntegration {
    store_name: String,
    is_enabled: bool,
    message_listeners: Vec<Box<dyn Fn(&DevToolsMessage) + Send + Sync>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl DevToolsIntegration {
    pub fn new(_store_name: String) -> Result<Self, DevToolsError> {
        Ok(Self {
            store_name: _store_name,
            is_enabled: false,
            message_listeners: Vec::new(),
        })
    }

    pub fn is_devtools_available() -> bool {
        false
    }

    pub fn send_message(&self, _message: DevToolsMessage) -> Result<(), DevToolsError> {
        Ok(()) // No-op on non-WASM targets
    }

    pub fn add_message_listener<F>(&mut self, _listener: F)
    where
        F: Fn(&DevToolsMessage) + Send + Sync + 'static,
    {
        // No-op on non-WASM targets
    }

    pub fn is_enabled(&self) -> bool {
        false
    }

    pub fn store_name(&self) -> &str {
        &self.store_name
    }
}

/// State change record for history tracking
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StateChange<S> {
    pub timestamp: u64,
    pub operation: String,
    pub old_state: Option<S>,
    pub new_state: S,
    pub changed_fields: Vec<String>,
    pub duration: u64, // microseconds
}

/// Metrics for state inspection
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct InspectorMetrics {
    pub total_operations: u64,
    pub total_duration: u64, // microseconds
    pub average_operation_time: u64, // microseconds
    pub max_operation_time: u64, // microseconds
    pub min_operation_time: u64, // microseconds
}

/// Runtime state inspector for debugging and monitoring
pub struct StateInspector<S: State> {
    store_name: String,
    current_state: S,
    previous_state: Option<S>,
    change_history: Vec<StateChange<S>>,
    metrics: InspectorMetrics,
    max_history_size: usize,
    is_recording: bool,
    #[cfg(all(target_arch = "wasm32", feature = "devtools"))]
    devtools: Option<DevToolsIntegration>,
}

impl<S: State + Clone> StateInspector<S> {
    /// Create a new state inspector
    pub fn new(store_name: String, initial_state: S) -> Self {
        Self {
            store_name,
            current_state: initial_state.clone(),
            previous_state: None,
            change_history: Vec::new(),
            metrics: InspectorMetrics::default(),
            max_history_size: 50,
            is_recording: true,
            #[cfg(all(target_arch = "wasm32", feature = "devtools"))]
            devtools: None,
        }
    }

    /// Create a state inspector with DevTools integration
    #[cfg(all(target_arch = "wasm32", feature = "devtools"))]
    pub fn with_devtools(store_name: String, initial_state: S) -> Result<Self, DevToolsError> {
        let devtools = DevToolsIntegration::new(store_name.clone())?;

        Ok(Self {
            store_name,
            current_state: initial_state.clone(),
            previous_state: None,
            change_history: Vec::new(),
            metrics: InspectorMetrics::default(),
            max_history_size: 50,
            is_recording: true,
            devtools: Some(devtools),
        })
    }

    /// Record a state change
    pub fn record_change(&mut self, operation: &str, new_state: S) {
        if !self.is_recording {
            return;
        }

        let start_time = SystemTime::now();
        let old_state = self.previous_state.clone();

        // Calculate duration (placeholder - in real implementation this would be passed in)
        let duration = start_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() as u64;

        // Detect changed fields (simplified - would need more sophisticated diffing)
        let changed_fields = vec!["state".to_string()]; // Placeholder

        let change = StateChange {
            timestamp: start_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            operation: operation.to_string(),
            old_state,
            new_state: new_state.clone(),
            changed_fields,
            duration,
        };

        // Update history
        self.change_history.push(change);
        if self.change_history.len() > self.max_history_size {
            self.change_history.remove(0);
        }

        // Update metrics
        self.metrics.total_operations += 1;
        self.metrics.total_duration += duration;
        self.metrics.average_operation_time = self.metrics.total_duration / self.metrics.total_operations;
        self.metrics.max_operation_time = self.metrics.max_operation_time.max(duration);
        self.metrics.min_operation_time = if self.metrics.min_operation_time == 0 {
            duration
        } else {
            self.metrics.min_operation_time.min(duration)
        };

        // Update state
        self.previous_state = Some(self.current_state.clone());
        self.current_state = new_state;

        // Send to DevTools if available
        #[cfg(all(target_arch = "wasm32", feature = "devtools"))]
        if let Some(ref devtools) = self.devtools {
            let _ = devtools.send_message(DevToolsMessage::StateChanged {
                name: self.store_name.clone(),
                state: serde_json::to_value(&self.current_state).unwrap_or_default(),
                timestamp: start_time
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            });
        }
    }

    /// Get current state
    pub fn current_state(&self) -> &S {
        &self.current_state
    }

    /// Get previous state
    pub fn previous_state(&self) -> Option<&S> {
        self.previous_state.as_ref()
    }

    /// Get change history
    pub fn change_history(&self) -> &[StateChange<S>] {
        &self.change_history
    }

    /// Get metrics
    pub fn metrics(&self) -> &InspectorMetrics {
        &self.metrics
    }

    /// Set maximum history size
    pub fn set_max_history_size(&mut self, size: usize) {
        self.max_history_size = size;
        // Trim history if needed
        while self.change_history.len() > self.max_history_size {
            self.change_history.remove(0);
        }
    }

    /// Enable/disable recording
    pub fn set_recording(&mut self, enabled: bool) {
        self.is_recording = enabled;
    }

    /// Check if recording is enabled
    pub fn is_recording(&self) -> bool {
        self.is_recording
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.change_history.clear();
    }

    /// Get change at specific index
    pub fn get_change(&self, index: usize) -> Option<&StateChange<S>> {
        self.change_history.get(index)
    }

    /// Get number of recorded changes
    pub fn change_count(&self) -> usize {
        self.change_history.len()
    }
}

/// Snapshot of state with metadata for time travel
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StateSnapshot<S> {
    pub state: S,
    pub timestamp: u64,
    pub operation: String,
    pub operation_index: usize,
    pub metadata: serde_json::Value,
    pub stack_trace: Option<String>,
}

/// Time travel debugger for navigating state history
pub struct TimeTravelDebugger<S: State> {
    store_name: String,
    history: Vec<StateSnapshot<S>>,
    current_index: usize,
    bookmarks: std::collections::HashMap<String, usize>,
    replay_mode: bool,
    max_history_size: usize,
    auto_snapshot: bool,
    #[cfg(all(target_arch = "wasm32", feature = "devtools"))]
    devtools: Option<DevToolsIntegration>,
}

impl<S: State + Clone> TimeTravelDebugger<S> {
    /// Create a new time travel debugger
    pub fn new(store_name: String, initial_state: S) -> Self {
        let initial_snapshot = StateSnapshot {
            state: initial_state,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            operation: "initial".to_string(),
            operation_index: 0,
            metadata: serde_json::Value::Null,
            stack_trace: None,
        };

        Self {
            store_name,
            history: vec![initial_snapshot],
            current_index: 0,
            bookmarks: std::collections::HashMap::new(),
            replay_mode: false,
            max_history_size: 100,
            auto_snapshot: true,
            #[cfg(all(target_arch = "wasm32", feature = "devtools"))]
            devtools: None,
        }
    }

    /// Create a time travel debugger with DevTools integration
    #[cfg(all(target_arch = "wasm32", feature = "devtools"))]
    pub fn with_devtools(store_name: String, initial_state: S) -> Result<Self, DevToolsError> {
        let devtools = DevToolsIntegration::new(store_name.clone())?;

        let initial_snapshot = StateSnapshot {
            state: initial_state,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            operation: "initial".to_string(),
            operation_index: 0,
            metadata: serde_json::Value::Null,
            stack_trace: None,
        };

        Ok(Self {
            store_name,
            history: vec![initial_snapshot],
            current_index: 0,
            bookmarks: std::collections::HashMap::new(),
            replay_mode: false,
            max_history_size: 100,
            auto_snapshot: true,
            devtools: Some(devtools),
        })
    }

    /// Record a new state snapshot
    pub fn record_snapshot(&mut self, state: S, operation: &str, metadata: serde_json::Value) {
        if !self.auto_snapshot {
            return;
        }

        let snapshot = StateSnapshot {
            state,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            operation: operation.to_string(),
            operation_index: self.history.len(),
            metadata,
            stack_trace: None, // Would need to implement stack trace capture
        };

        // Add to history
        self.history.push(snapshot);

        // Trim history if needed
        if self.history.len() > self.max_history_size {
            self.history.remove(0);
        }

        // Update current index if not in replay mode
        if !self.replay_mode {
            self.current_index = self.history.len() - 1;
        }

        // Send to DevTools if available
        #[cfg(all(target_arch = "wasm32", feature = "devtools"))]
        if let Some(ref devtools) = self.devtools {
            let _ = devtools.send_message(DevToolsMessage::StateChanged {
                name: self.store_name.clone(),
                state: serde_json::to_value(&self.history.last().unwrap().state).unwrap_or_default(),
                timestamp: self.history.last().unwrap().timestamp,
            });
        }
    }

    /// Time travel to a specific snapshot index
    pub fn time_travel(&mut self, index: usize) -> Result<&S, DevToolsError> {
        if index >= self.history.len() {
            return Err(DevToolsError::TimeTravelFailed {
                message: format!("Index {} out of bounds (max: {})", index, self.history.len() - 1),
            });
        }

        self.current_index = index;
        self.replay_mode = true;

        // Send command to DevTools
        #[cfg(all(target_arch = "wasm32", feature = "devtools"))]
        if let Some(ref devtools) = self.devtools {
            let _ = devtools.send_message(DevToolsMessage::Command(
                DevToolsCommand::TimeTravel { index }
            ));
        }

        Ok(&self.history[self.current_index].state)
    }

    /// Go to the next snapshot
    pub fn next(&mut self) -> Option<&S> {
        if self.current_index < self.history.len() - 1 {
            self.current_index += 1;
            Some(&self.history[self.current_index].state)
        } else {
            None
        }
    }

    /// Go to the previous snapshot
    pub fn previous(&mut self) -> Option<&S> {
        if self.current_index > 0 {
            self.current_index -= 1;
            Some(&self.history[self.current_index].state)
        } else {
            None
        }
    }

    /// Get current snapshot
    pub fn current(&self) -> &StateSnapshot<S> {
        &self.history[self.current_index]
    }

    /// Get current state
    pub fn current_state(&self) -> &S {
        &self.history[self.current_index].state
    }

    /// Check if currently in replay mode
    pub fn is_replay_mode(&self) -> bool {
        self.replay_mode
    }

    /// Exit replay mode and go to latest state
    pub fn exit_replay_mode(&mut self) {
        self.replay_mode = false;
        self.current_index = self.history.len() - 1;
    }

    /// Get all snapshots
    pub fn history(&self) -> &[StateSnapshot<S>] {
        &self.history
    }

    /// Get snapshot at index
    pub fn get_snapshot(&self, index: usize) -> Option<&StateSnapshot<S>> {
        self.history.get(index)
    }

    /// Create a bookmark at current position
    pub fn bookmark(&mut self, name: &str) {
        self.bookmarks.insert(name.to_string(), self.current_index);
    }

    /// Jump to a bookmarked position
    pub fn goto_bookmark(&mut self, name: &str) -> Result<&S, DevToolsError> {
        if let Some(&index) = self.bookmarks.get(name) {
            self.time_travel(index)
        } else {
            Err(DevToolsError::TimeTravelFailed {
                message: format!("Bookmark '{}' not found", name),
            })
        }
    }

    /// Get all bookmarks
    pub fn bookmarks(&self) -> &std::collections::HashMap<String, usize> {
        &self.bookmarks
    }

    /// Clear all bookmarks
    pub fn clear_bookmarks(&mut self) {
        self.bookmarks.clear();
    }

    /// Set maximum history size
    pub fn set_max_history_size(&mut self, size: usize) {
        self.max_history_size = size;
        // Trim history if needed
        while self.history.len() > self.max_history_size {
            self.history.remove(0);
        }
    }

    /// Enable/disable auto snapshot recording
    pub fn set_auto_snapshot(&mut self, enabled: bool) {
        self.auto_snapshot = enabled;
    }

    /// Check if auto snapshot is enabled
    pub fn auto_snapshot(&self) -> bool {
        self.auto_snapshot
    }

    /// Clear history (keep only current state)
    pub fn clear_history(&mut self) {
        if !self.history.is_empty() {
            let current = self.history[self.current_index].clone();
            self.history = vec![current];
            self.current_index = 0;
            self.bookmarks.clear();
        }
    }

    /// Get history size
    pub fn history_size(&self) -> usize {
        self.history.len()
    }

    /// Get current index
    pub fn current_index(&self) -> usize {
        self.current_index
    }
}
