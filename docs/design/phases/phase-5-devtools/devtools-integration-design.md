# DevTools Integration Design

## Overview
Implement browser DevTools integration for real-time state inspection, debugging, and development workflow enhancement.

## Current State
```rust
// No DevTools integration
impl<S: State> Store<S> {
    pub fn new(initial: S) -> Self { /* ... */ }
}
```

## Proposed Enhancement
```rust
#[cfg(feature = "devtools")]
impl<S: State> Store<S> {
    pub fn with_devtools(self, name: &str) -> Self {
        // Enable DevTools integration
    }
}

// Browser console API
console.devtools.state.inspect("store_name");
console.devtools.state.timeTravel(5);
console.devtools.state.reset();
```

## Motivation

### Development Experience
- **Real-time Inspection**: View state changes as they happen
- **Interactive Debugging**: Modify state from DevTools console
- **Performance Monitoring**: Track state operation timing
- **Development Workflow**: Faster iteration and debugging

### Debugging Capabilities
- **State History**: View complete state change timeline
- **Time Travel**: Jump back to previous states
- **State Diffing**: See exactly what changed between states
- **Breakpoints**: Pause execution on state changes
- **Hot Reloading**: Update state without page refresh

### Use Cases
- Debugging state mutations during development
- Understanding complex state interactions
- Performance profiling of state operations
- Testing state changes in isolation
- Learning state management patterns

## Implementation Details

### DevTools Detection and Setup
```rust
#[cfg(feature = "web")]
pub struct DevToolsIntegration<S: State> {
    store_name: String,
    history: Vec<StateSnapshot<S>>,
    max_history_size: usize,
    current_index: usize,
    is_enabled: bool,
    listeners: Vec<Box<dyn Fn(&DevToolsMessage) + Send + Sync>>,
}

#[cfg(feature = "web")]
impl<S: State> DevToolsIntegration<S> {
    pub fn new(store_name: String) -> Result<Self, DevToolsError> {
        let integration = Self {
            store_name,
            history: Vec::new(),
            max_history_size: 50,
            current_index: 0,
            is_enabled: Self::is_devtools_available(),
            listeners: Vec::new(),
        };

        if integration.is_enabled {
            integration.setup_devtools_bridge()?;
        }

        Ok(integration)
    }

    fn is_devtools_available() -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            // Check if running in development environment
            // Look for __LEPTOS_DEVTOOLS__ global or similar
            js_sys::Reflect::get(&web_sys::window().unwrap(), &"__LEPTOS_DEVTOOLS__".into())
                .ok()
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            false
        }
    }

    fn setup_devtools_bridge(&self) -> Result<(), DevToolsError> {
        #[cfg(target_arch = "wasm32")]
        {
            // Register store with DevTools
            let store_data = serde_json::json!({
                "name": self.store_name,
                "type": "leptos-state-store",
                "features": ["inspect", "timeTravel", "reset"]
            });

            // Send to DevTools via custom event or global
            self.send_to_devtools("register", store_data)?;
        }

        Ok(())
    }

    fn send_to_devtools(&self, message_type: &str, data: serde_json::Value) -> Result<(), DevToolsError> {
        #[cfg(target_arch = "wasm32")]
        {
            let message = serde_json::json!({
                "type": message_type,
                "store": self.store_name,
                "data": data,
                "timestamp": Utc::now().timestamp()
            });

            // Send via custom event
            let event = web_sys::CustomEvent::new("leptos-devtools")?;
            event.init_custom_event_with_can_bubble_and_cancelable(
                "leptos-devtools",
                false,
                false
            );

            js_sys::Reflect::set(&event, &"detail".into(), &message.into())?;
            web_sys::window().unwrap().dispatch_event(&event)?;
        }

        Ok(())
    }
}
```

### State History and Time Travel
```rust
#[cfg(feature = "web")]
impl<S: State + Clone> DevToolsIntegration<S> {
    pub fn record_state_change(&mut self, new_state: &S, operation: &str) {
        if !self.is_enabled {
            return;
        }

        let snapshot = StateSnapshot {
            state: new_state.clone(),
            timestamp: Utc::now(),
            operation: operation.to_string(),
            index: self.history.len(),
        };

        // Remove future history if we're not at the end
        self.history.truncate(self.current_index + 1);

        // Add new snapshot
        self.history.push(snapshot);
        self.current_index = self.history.len() - 1;

        // Limit history size
        if self.history.len() > self.max_history_size {
            self.history.remove(0);
            self.current_index -= 1;
        }

        // Notify DevTools
        self.notify_devtools("stateChanged", &serde_json::json!({
            "currentIndex": self.current_index,
            "totalStates": self.history.len(),
            "operation": operation
        }));
    }

    pub fn time_travel_to(&mut self, index: usize) -> Result<&S, DevToolsError> {
        if index >= self.history.len() {
            return Err(DevToolsError::InvalidTimeTravelIndex);
        }

        self.current_index = index;

        // Notify DevTools
        self.notify_devtools("timeTravel", &serde_json::json!({
            "index": index,
            "totalStates": self.history.len()
        }));

        Ok(&self.history[index].state)
    }

    pub fn get_history(&self) -> &[StateSnapshot<S>] {
        &self.history
    }

    pub fn get_current_state(&self) -> Option<&S> {
        self.history.get(self.current_index).map(|s| &s.state)
    }

    pub fn can_time_travel_back(&self) -> bool {
        self.current_index > 0
    }

    pub fn can_time_travel_forward(&self) -> bool {
        self.current_index < self.history.len() - 1
    }

    pub fn time_travel_back(&mut self) -> Result<&S, DevToolsError> {
        if !self.can_time_travel_back() {
            return Err(DevToolsError::CannotTimeTravelBack);
        }
        self.time_travel_to(self.current_index - 1)
    }

    pub fn time_travel_forward(&mut self) -> Result<&S, DevToolsError> {
        if !self.can_time_travel_forward() {
            return Err(DevToolsError::CannotTimeTravelForward);
        }
        self.time_travel_to(self.current_index + 1)
    }
}

#[derive(Clone, Debug)]
pub struct StateSnapshot<S> {
    pub state: S,
    pub timestamp: DateTime<Utc>,
    pub operation: String,
    pub index: usize,
}
```

### DevTools Message System
```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DevToolsMessage {
    RegisterStore {
        name: String,
        features: Vec<String>,
    },
    StateChanged {
        store: String,
        current_index: usize,
        total_states: usize,
        operation: String,
    },
    TimeTravel {
        store: String,
        index: usize,
        total_states: usize,
    },
    InspectState {
        store: String,
        state: serde_json::Value,
    },
    ResetStore {
        store: String,
    },
    Error {
        store: String,
        message: String,
    },
}

#[cfg(feature = "web")]
impl<S: State> DevToolsIntegration<S> {
    pub fn add_message_listener<F>(&mut self, listener: F)
    where
        F: Fn(&DevToolsMessage) + Send + Sync + 'static,
    {
        self.listeners.push(Box::new(listener));
    }

    pub fn notify_devtools(&self, message_type: &str, data: &serde_json::Value) {
        let message = DevToolsMessage::StateChanged {
            store: self.store_name.clone(),
            current_index: self.current_index,
            total_states: self.history.len(),
            operation: message_type.to_string(),
        };

        for listener in &self.listeners {
            listener(&message);
        }
    }

    pub fn handle_devtools_command(&mut self, command: DevToolsCommand) -> Result<(), DevToolsError> {
        match command {
            DevToolsCommand::Inspect => {
                if let Some(state) = self.get_current_state() {
                    let json = serde_json::to_value(state)?;
                    self.notify_devtools("inspect", &json);
                }
            }
            DevToolsCommand::TimeTravelBack => {
                self.time_travel_back()?;
            }
            DevToolsCommand::TimeTravelForward => {
                self.time_travel_forward()?;
            }
            DevToolsCommand::Reset => {
                self.current_index = 0;
                self.notify_devtools("reset", &serde_json::json!({}));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DevToolsCommand {
    Inspect,
    TimeTravelBack,
    TimeTravelForward,
    Reset,
}
```

### Store Integration
```rust
#[cfg(feature = "web")]
impl<S: State + Clone> Store<S> {
    pub fn with_devtools(mut self, name: &str) -> Result<Self, DevToolsError> {
        let devtools = DevToolsIntegration::new(name.to_string())?;

        // Store initial state
        devtools.record_state_change(&self.signal.get_untracked(), "init");

        self.devtools = Some(devtools);
        Ok(self)
    }

    pub fn update_with_devtools<F>(&self, updater: F) -> Result<(), StoreError>
    where
        F: FnOnce(&mut S) + Send + 'static,
    {
        let old_state = self.signal.get_untracked();
        self.signal.update(updater);
        let new_state = self.signal.get_untracked();

        // Record in DevTools
        if let Some(ref mut devtools) = self.devtools {
            devtools.record_state_change(&new_state, "update");
        }

        Ok(())
    }

    pub fn time_travel_to(&self, index: usize) -> Result<(), StoreError> {
        if let Some(ref mut devtools) = self.devtools {
            let target_state = devtools.time_travel_to(index)?;
            self.signal.set(target_state.clone());
            Ok(())
        } else {
            Err(StoreError::DevToolsNotEnabled)
        }
    }

    pub fn get_devtools_history(&self) -> Option<&[StateSnapshot<S>]> {
        self.devtools.as_ref().map(|dt| dt.get_history())
    }
}
```

### Browser Console API
```rust
#[cfg(target_arch = "wasm32")]
pub mod console_api {
    use super::*;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        fn log(s: &str);
    }

    #[wasm_bindgen(start)]
    pub fn setup_devtools_console_api() {
        // Register global DevTools functions
        js_sys::Reflect::set(
            &web_sys::window().unwrap(),
            &"__leptos_devtools__".into(),
            &serde_wasm_bindgen::to_value(&DevToolsConsoleApi::new()).unwrap()
        ).unwrap();
    }

    #[wasm_bindgen]
    pub struct DevToolsConsoleApi {
        stores: HashMap<String, Box<dyn Any + Send + Sync>>,
    }

    #[wasm_bindgen]
    impl DevToolsConsoleApi {
        pub fn new() -> Self {
            Self {
                stores: HashMap::new(),
            }
        }

        pub fn register_store(&mut self, name: String, store: Box<dyn Any + Send + Sync>) {
            self.stores.insert(name, store);
        }

        pub fn inspect(&self, store_name: &str) -> Result<String, JsValue> {
            if let Some(store) = self.stores.get(store_name) {
                // This would need type-safe access to the store
                // Implementation would depend on how stores are registered
                Ok(format!("Inspecting store: {}", store_name))
            } else {
                Err(format!("Store '{}' not found", store_name).into())
            }
        }

        pub fn time_travel(&self, store_name: &str, index: usize) -> Result<(), JsValue> {
            // Implementation for time travel via console
            log(&format!("Time traveling store '{}' to index {}", store_name, index));
            Ok(())
        }

        pub fn list_stores(&self) -> Vec<String> {
            self.stores.keys().cloned().collect()
        }
    }
}
```

## Error Handling

### DevTools Errors
```rust
#[derive(Debug, Clone, thiserror::Error)]
pub enum DevToolsError {
    #[error("DevTools not available in this environment")]
    NotAvailable,

    #[error("Failed to setup DevTools bridge")]
    SetupFailed,

    #[error("Invalid time travel index: {0}")]
    InvalidTimeTravelIndex(usize),

    #[error("Cannot time travel back from beginning")]
    CannotTimeTravelBack,

    #[error("Cannot time travel forward from end")]
    CannotTimeTravelForward,

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("JavaScript error: {0}")]
    JavaScript(String),
}
```

### Graceful Degradation
```rust
#[cfg(feature = "web")]
impl<S: State> Store<S> {
    pub fn with_devtools_graceful(self, name: &str) -> Self {
        match self.with_devtools(name) {
            Ok(store) => store,
            Err(_) => {
                // DevTools not available, return store without DevTools
                log::debug!("DevTools not available, continuing without DevTools integration");
                self
            }
        }
    }
}
```

## Testing Strategy

### Unit Tests
```rust
#[cfg(feature = "web")]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn devtools_history_recording() {
        let mut devtools = DevToolsIntegration::<TestState>::new("test_store".to_string()).unwrap();

        let state1 = TestState { count: 0 };
        let state2 = TestState { count: 1 };
        let state3 = TestState { count: 2 };

        devtools.record_state_change(&state1, "init");
        devtools.record_state_change(&state2, "increment");
        devtools.record_state_change(&state3, "increment");

        assert_eq!(devtools.get_history().len(), 3);
        assert_eq!(devtools.get_current_state().unwrap().count, 2);
    }

    #[wasm_bindgen_test]
    async fn time_travel_functionality() {
        let mut devtools = DevToolsIntegration::<TestState>::new("test_store".to_string()).unwrap();

        // Record some states
        for i in 0..5 {
            let state = TestState { count: i };
            devtools.record_state_change(&state, &format!("set_{}", i));
        }

        // Time travel back
        let state = devtools.time_travel_to(2).unwrap();
        assert_eq!(state.count, 2);

        // Time travel forward
        let state = devtools.time_travel_to(4).unwrap();
        assert_eq!(state.count, 4);
    }
}
```

### Integration Tests
```rust
#[cfg(feature = "web")]
#[wasm_bindgen_test]
async fn store_with_devtools_integration() {
    let store = Store::new(TestState { count: 0 })
        .with_devtools("integration_test")
        .unwrap();

    // Perform some operations
    store.update_with_devtools(|s| s.count = 5).unwrap();
    store.update_with_devtools(|s| s.count = 10).unwrap();

    // Check history
    let history = store.get_devtools_history().unwrap();
    assert_eq!(history.len(), 3); // init + 2 updates

    // Time travel
    store.time_travel_to(1).unwrap();
    assert_eq!(store.get().get_untracked().count, 5);
}
```

## Performance Impact

### Development-Only Overhead
- **Zero in Production**: DevTools only enabled in development builds
- **Memory**: History storage (configurable limit)
- **CPU**: State diffing and serialization during development
- **Network**: Minimal DevTools message passing

### Optimization Strategies
```rust
#[cfg(feature = "web")]
impl<S: State> DevToolsIntegration<S> {
    pub fn with_compressed_history(mut self) -> Self {
        // Compress old history entries to save memory
        self.compression_enabled = true;
        self
    }

    pub fn with_sampling(mut self, sample_rate: f64) -> Self {
        // Only record a percentage of state changes
        self.sampling_rate = sample_rate;
        self
    }

    pub fn with_async_recording(mut self) -> Self {
        // Record state changes asynchronously to avoid blocking
        self.async_recording = true;
        self
    }
}
```

## Security Considerations

### Development-Only Access
- DevTools only available in development builds
- Production builds have no DevTools overhead or access
- Clear separation between development and production code

### Sensitive Data Protection
```rust
#[cfg(feature = "web")]
impl<S: State> DevToolsIntegration<S> {
    pub fn with_data_filter<F>(mut self, filter: F) -> Self
    where
        F: Fn(&S) -> S + Send + Sync + 'static,
    {
        // Filter sensitive data before sending to DevTools
        self.data_filter = Some(Box::new(filter));
        self
    }
}

// Usage
let store = Store::new(user_state)
    .with_devtools("user_store")
    .unwrap()
    .with_data_filter(|state| {
        // Remove sensitive fields for DevTools
        UserState {
            password_hash: "[FILTERED]".to_string(),
            ..state.clone()
        }
    });
```

### Cross-Origin Protection
- DevTools messages only sent to same-origin
- No external communication allowed
- All DevTools functionality scoped to current page

## Future Extensions

### Advanced Debugging Features
```rust
#[cfg(feature = "web")]
impl<S: State> DevToolsIntegration<S> {
    pub fn add_breakpoint<F>(&mut self, condition: F)
    where
        F: Fn(&S, &S) -> bool + Send + Sync + 'static, // (old_state, new_state) -> should_break
    {
        // Pause execution when condition is met
        todo!()
    }

    pub fn add_watch_expression<F, R>(&mut self, expression: F)
    where
        F: Fn(&S) -> R + Send + Sync + 'static,
        R: Debug,
    {
        // Watch specific expressions and log changes
        todo!()
    }

    pub fn record_performance_metrics(&mut self, enable: bool) {
        // Record timing information for state operations
        todo!()
    }
}
```

### Collaborative Debugging
```rust
#[cfg(feature = "web")]
impl<S: State> DevToolsIntegration<S> {
    pub async fn connect_to_remote_debugger(&mut self, url: &str) -> Result<(), DevToolsError> {
        // Connect to remote debugging session
        // Sync state changes across multiple developers
        todo!()
    }

    pub fn share_debugging_session(&self) -> String {
        // Generate shareable debugging session URL
        todo!()
    }
}
```

### State Machine Visualization
```rust
#[cfg(feature = "web")]
impl<E: Event> DevToolsIntegration<MachineState> {
    pub fn visualize_state_machine(&self) -> Result<String, DevToolsError> {
        // Generate DOT graph for state machine visualization
        // Return as string for DevTools to render
        todo!()
    }

    pub fn highlight_current_state(&self) {
        // Highlight current state in DevTools visualization
        todo!()
    }
}
```

## Migration Guide

### Adding DevTools to Existing Stores
```rust
// Before - no DevTools
let store = Store::new(initial_state);

// After - with DevTools
let store = Store::new(initial_state)
    .with_devtools("my_store")
    .unwrap_or_else(|_| {
        // Fallback if DevTools not available
        Store::new(initial_state)
    });
```

### Conditional DevTools
```rust
// Only enable DevTools in development
#[cfg(debug_assertions)]
let store = Store::new(initial_state)
    .with_devtools("my_store")
    .unwrap();

#[cfg(not(debug_assertions))]
let store = Store::new(initial_state);
```

### Custom DevTools Configuration
```rust
let store = Store::new(initial_state)
    .with_devtools("my_store")
    .unwrap()
    .with_devtools_config(DevToolsConfig {
        max_history_size: 100,
        enable_time_travel: true,
        enable_state_diffing: true,
        enable_performance_monitoring: true,
    });
```

## Browser DevTools Extension

### Extension Architecture
```javascript
// DevTools panel script
class LeptosStateDevTools {
    constructor() {
        this.stores = new Map();
        this.setupEventListeners();
    }

    setupEventListeners() {
        window.addEventListener('leptos-devtools', (event) => {
            this.handleDevToolsMessage(event.detail);
        });
    }

    handleDevToolsMessage(message) {
        switch (message.type) {
            case 'register':
                this.registerStore(message.store, message.data);
                break;
            case 'stateChanged':
                this.updateStoreState(message.store, message.data);
                break;
            case 'timeTravel':
                this.handleTimeTravel(message.store, message.data);
                break;
        }
    }

    registerStore(name, data) {
        this.stores.set(name, {
            name,
            features: data.features,
            history: [],
            currentIndex: 0
        });
        this.updateUI();
    }

    // Additional UI and interaction methods...
}
```

### Console API Integration
```javascript
// Make DevTools available in console
window.leptosState = {
    stores: () => {
        return Array.from(devToolsPanel.stores.keys());
    },

    inspect: (storeName) => {
        const store = devToolsPanel.stores.get(storeName);
        if (store) {
            console.table(store.history[store.currentIndex]);
        }
    },

    timeTravel: (storeName, index) => {
        // Send time travel command to store
        const event = new CustomEvent('leptos-devtools-command', {
            detail: {
                command: 'timeTravel',
                store: storeName,
                index
            }
        });
        window.dispatchEvent(event);
    }
};
```

## Risk Assessment

### Likelihood: Low
- DevTools are development-only features
- Graceful degradation when not available
- Well-established browser DevTools patterns

### Impact: Low
- Zero production impact (feature-gated)
- Memory overhead bounded and configurable
- Clear separation from application logic

### Mitigation
- Comprehensive testing in browser environments
- Graceful degradation strategies
- Security-conscious data filtering
- Performance monitoring and optimization
- Clear documentation and usage guidelines
