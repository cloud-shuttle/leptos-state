//! DevTools connector implementations

use super::devtools_core::{DevToolsConnection, DevToolsConnector, StateUpdate};
use crate::utils::StateResult;

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
