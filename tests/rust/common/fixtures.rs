//! Test fixtures and utilities for leptos-state tests
//!
//! This module provides reusable test fixtures, mock implementations,
//! and helper functions to reduce test boilerplate and improve consistency.

use leptos_state::*;
use leptos_state::machine::*;
use leptos_state::store::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Standard test context for state machines
#[derive(Clone, Debug, PartialEq, Default)]
pub struct TestContext {
    pub counter: i32,
    pub name: String,
    pub flags: HashMap<String, bool>,
    pub data: HashMap<String, serde_json::Value>,
}

impl TestContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_counter(counter: i32) -> Self {
        Self {
            counter,
            ..Default::default()
        }
    }

    pub fn with_name<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn set_flag<S: Into<String>>(&mut self, key: S, value: bool) {
        self.flags.insert(key.into(), value);
    }

    pub fn get_flag(&self, key: &str) -> bool {
        self.flags.get(key).copied().unwrap_or(false)
    }
}

/// Standard test events for state machines
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TestEvent {
    Start,
    Stop,
    Pause,
    Resume,
    Reset,
    Update(i32),
    SetName(String),
    ToggleFlag(String),
    Custom(String),
}

impl std::fmt::Display for TestEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestEvent::Start => write!(f, "Start"),
            TestEvent::Stop => write!(f, "Stop"),
            TestEvent::Pause => write!(f, "Pause"),
            TestEvent::Resume => write!(f, "Resume"),
            TestEvent::Reset => write!(f, "Reset"),
            TestEvent::Update(val) => write!(f, "Update({})", val),
            TestEvent::SetName(name) => write!(f, "SetName({})", name),
            TestEvent::ToggleFlag(flag) => write!(f, "ToggleFlag({})", flag),
            TestEvent::Custom(data) => write!(f, "Custom({})", data),
        }
    }
}

/// Test fixture for state machine testing
pub struct MachineTestFixture {
    pub machine: Machine<TestContext, TestEvent, TestContext>,
    pub initial_context: TestContext,
}

impl MachineTestFixture {
    /// Create a basic state machine for testing
    pub fn basic_machine() -> Self {
        let machine = machine!(create_machine_builder::<TestContext, TestEvent>(),
            state "idle"
                .on(TestEvent::Start) => "running",
            state "running"
                .on(TestEvent::Pause) => "paused"
                .on(TestEvent::Stop) => "stopped"
                .on_entry(|ctx: &mut TestContext| ctx.counter += 1),
            state "paused"
                .on(TestEvent::Resume) => "running"
                .on(TestEvent::Stop) => "stopped",
            state "stopped"
                .on(TestEvent::Reset) => "idle"
                .on_entry(|ctx: &mut TestContext| ctx.counter = 0),
            initial "idle"
        );

        Self {
            machine,
            initial_context: TestContext::new(),
        }
    }

    /// Create a machine with guards
    pub fn machine_with_guards() -> Self {
        let machine = machine!(create_machine_builder::<TestContext, TestEvent>(),
            state "locked"
                .on(TestEvent::Start) => "unlocked"
                    .guard(|ctx: &TestContext| ctx.get_flag("has_key")),
            state "unlocked"
                .on(TestEvent::Stop) => "locked"
                .on(TestEvent::Update(42)) => "special"
                    .guard(|ctx: &TestContext| ctx.counter > 5),
            state "special"
                .on(TestEvent::Reset) => "locked",
            initial "locked"
        );

        Self {
            machine,
            initial_context: TestContext::new(),
        }
    }

    /// Create a machine with actions
    pub fn machine_with_actions() -> Self {
        let machine = machine!(create_machine_builder::<TestContext, TestEvent>(),
            state "init"
                .on_entry(|ctx: &mut TestContext| {
                    ctx.set_flag("initialized", true);
                    ctx.name = "initialized".to_string();
                })
                .on(TestEvent::Start) => "active",
            state "active"
                .on_exit(|ctx: &mut TestContext| {
                    ctx.set_flag("was_active", true);
                })
                .on(TestEvent::SetName(name)) => "active"
                    .action(move |ctx: &mut TestContext| {
                        ctx.name = name.clone();
                    }),
            initial "init"
        );

        Self {
            machine,
            initial_context: TestContext::new(),
        }
    }

    /// Reset machine to initial state
    pub fn reset(&mut self) {
        // Create a new machine instance (since Machine doesn't implement Clone)
        let new_machine = match self.machine.current_state().name.as_str() {
            "idle" | "locked" | "init" => Self::basic_machine().machine,
            _ => Self::basic_machine().machine,
        };
        self.machine = new_machine;
    }

    /// Get current state name
    pub fn current_state(&self) -> &str {
        &self.machine.current_state().name
    }

    /// Get current context
    pub fn context(&self) -> &TestContext {
        self.machine.get_context()
    }
}

/// Test fixture for store testing
pub struct StoreTestFixture<T: Clone + PartialEq + std::fmt::Debug + Default + 'static> {
    pub store: StoreContext<T>,
    pub initial_state: T,
}

impl<T: Clone + PartialEq + std::fmt::Debug + Default + Send + Sync + 'static> StoreTestFixture<T> {
    pub fn new(initial_state: T) -> Self {
        let store = create_store(initial_state.clone());
        Self {
            store,
            initial_state,
        }
    }

    pub fn basic_store() -> StoreTestFixture<TestState> {
        let initial_state = TestState::default();
        Self::new(initial_state)
    }
}

/// Standard test state for stores
#[derive(Clone, Debug, PartialEq, Default)]
pub struct TestState {
    pub counter: i32,
    pub name: String,
    pub items: Vec<String>,
    pub settings: HashMap<String, serde_json::Value>,
}

impl TestState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_counter(counter: i32) -> Self {
        Self {
            counter,
            ..Default::default()
        }
    }

    pub fn add_item<S: Into<String>>(&mut self, item: S) {
        self.items.push(item.into());
    }

    pub fn set_setting<K: Into<String>, V: Into<serde_json::Value>>(&mut self, key: K, value: V) {
        self.settings.insert(key.into(), value.into());
    }
}

/// Mock adapter for testing integration
pub struct MockAdapter {
    pub received_events: Arc<Mutex<Vec<super::super::super::machine::integration::events::IntegrationEvent>>>,
    pub send_behavior: MockSendBehavior,
    pub health_status: Arc<Mutex<bool>>,
}

#[derive(Clone, Debug)]
pub enum MockSendBehavior {
    AlwaysSuccess,
    AlwaysFail(String),
    Conditional(Box<dyn Fn(&super::super::super::machine::integration::events::IntegrationEvent) -> Result<(), super::super::super::machine::integration::IntegrationError> + Send + Sync>),
}

impl MockAdapter {
    pub fn new() -> Self {
        Self {
            received_events: Arc::new(Mutex::new(Vec::new())),
            send_behavior: MockSendBehavior::AlwaysSuccess,
            health_status: Arc::new(Mutex::new(true)),
        }
    }

    pub fn with_send_behavior(mut self, behavior: MockSendBehavior) -> Self {
        self.send_behavior = behavior;
        self
    }

    pub fn failing(error_message: &str) -> Self {
        Self::new().with_send_behavior(MockSendBehavior::AlwaysFail(error_message.to_string()))
    }

    pub fn get_received_events(&self) -> Vec<super::super::super::machine::integration::events::IntegrationEvent> {
        self.received_events.lock().unwrap().clone()
    }

    pub fn clear_events(&self) {
        self.received_events.lock().unwrap().clear();
    }

    pub fn set_health(&self, healthy: bool) {
        *self.health_status.lock().unwrap() = healthy;
    }
}

impl super::super::super::machine::integration::adapters::traits::IntegrationAdapterTrait for MockAdapter {
    fn adapter_type(&self) -> super::super::super::machine::integration::adapters::traits::AdapterType {
        super::super::super::machine::integration::adapters::traits::AdapterType::Custom("mock".to_string())
    }

    fn name(&self) -> String {
        "Mock Adapter".to_string()
    }

    fn supports_feature(&self, _feature: super::super::super::machine::integration::adapters::traits::AdapterFeature) -> bool {
        true // Mock supports everything
    }

    async fn health_check(&self) -> Result<(), super::super::super::machine::integration::IntegrationError> {
        if *self.health_status.lock().unwrap() {
            Ok(())
        } else {
            Err(super::super::super::machine::integration::IntegrationError::ConnectionError("Mock adapter unhealthy".to_string()))
        }
    }

    async fn send_event(&self, event: &super::super::super::machine::integration::events::IntegrationEvent) -> Result<(), super::super::super::machine::integration::IntegrationError> {
        self.received_events.lock().unwrap().push(event.clone());

        match &self.send_behavior {
            MockSendBehavior::AlwaysSuccess => Ok(()),
            MockSendBehavior::AlwaysFail(msg) => Err(super::super::super::machine::integration::IntegrationError::SendError(msg.clone())),
            MockSendBehavior::Conditional(checker) => checker(event),
        }
    }

    async fn receive_events(&self) -> Result<Vec<super::super::super::machine::integration::events::IntegrationEvent>, super::super::super::machine::integration::IntegrationError> {
        // Mock adapter doesn't receive events by default
        Ok(Vec::new())
    }

    fn stats(&self) -> super::super::super::machine::integration::adapters::traits::AdapterStats {
        let events_sent = self.received_events.lock().unwrap().len() as u64;
        super::super::super::machine::integration::adapters::traits::AdapterStats {
            events_sent,
            events_received: 0,
            errors: 0,
            last_success: Some(std::time::SystemTime::now()),
            last_error: None,
            healthy: *self.health_status.lock().unwrap(),
        }
    }
}

/// Test utilities for timing and performance
pub struct TestTimer {
    start: std::time::Instant,
}

impl TestTimer {
    pub fn new() -> Self {
        Self {
            start: std::time::Instant::now(),
        }
    }

    pub fn elapsed(&self) -> std::time::Duration {
        self.start.elapsed()
    }

    pub fn assert_under(&self, limit: std::time::Duration, operation: &str) {
        let elapsed = self.elapsed();
        assert!(elapsed < limit,
            "{} took {:?}, exceeded limit of {:?}", operation, elapsed, limit);
    }

    pub fn print_elapsed(&self, operation: &str) {
        println!("{} took {:?}", operation, self.elapsed());
    }
}

/// Macro for creating test machines with less boilerplate
#[macro_export]
macro_rules! test_machine {
    ($($body:tt)*) => {{
        machine!(create_machine_builder::<$crate::common::fixtures::TestContext, $crate::common::fixtures::TestEvent>(),
            $($body)*
        )
    }};
}

/// Macro for asserting state machine state
#[macro_export]
macro_rules! assert_machine_state {
    ($machine:expr, $expected_state:expr) => {
        assert_eq!($machine.current_state().name, $expected_state,
            "Expected state '{}', got '{}'", $expected_state, $machine.current_state().name);
    };
}

/// Macro for asserting context values
#[macro_export]
macro_rules! assert_context {
    ($machine:expr, $field:ident == $expected:expr) => {
        assert_eq!($machine.get_context().$field, $expected,
            "Expected context.{:?} == {:?}, got {:?}", stringify!($field), $expected, $machine.get_context().$field);
    };
}

/// Helper for creating temporary config files for testing
pub struct TempConfigFile {
    file: tempfile::NamedTempFile,
}

impl TempConfigFile {
    pub fn new(content: &str) -> Result<Self, std::io::Error> {
        let mut file = tempfile::NamedTempFile::new()?;
        std::io::Write::write_all(&mut file, content.as_bytes())?;
        Ok(Self { file })
    }

    pub fn path(&self) -> &std::path::Path {
        self.file.path()
    }

    pub fn path_str(&self) -> String {
        self.path().to_string_lossy().to_string()
    }
}

impl Drop for TempConfigFile {
    fn drop(&mut self) {
        // Temp file will be automatically cleaned up
    }
}

/// Helper for setting up environment variables for testing
pub struct TestEnvironment {
    vars: HashMap<String, String>,
}

impl TestEnvironment {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn set<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.vars.insert(key.into(), value.into());
        self
    }

    pub fn apply(&self) {
        for (key, value) in &self.vars {
            std::env::set_var(key, value);
        }
    }

    pub fn cleanup(&self) {
        for key in self.vars.keys() {
            std::env::remove_var(key);
        }
    }
}

impl Drop for TestEnvironment {
    fn drop(&mut self) {
        self.cleanup();
    }
}

/// Async test helper for running tests with timeouts
pub async fn with_timeout<F, Fut, T>(future: F, timeout: std::time::Duration) -> Result<T, String>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = T>,
{
    let timeout_future = tokio::time::timeout(timeout, future());
    match timeout_future.await {
        Ok(result) => Ok(result),
        Err(_) => Err(format!("Test timed out after {:?}", timeout)),
    }
}

/// Performance benchmark helper
pub fn benchmark<F, R>(iterations: usize, operation: F) -> (std::time::Duration, Vec<R>)
where
    F: Fn() -> R,
{
    let mut results = Vec::with_capacity(iterations);
    let start = std::time::Instant::now();

    for _ in 0..iterations {
        results.push(operation());
    }

    let total_time = start.elapsed();
    (total_time, results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine_fixture_basic() {
        let fixture = MachineTestFixture::basic_machine();

        assert_eq!(fixture.current_state(), "idle");
        assert_eq!(fixture.context().counter, 0);
    }

    #[test]
    fn test_machine_fixture_transition() {
        let fixture = MachineTestFixture::basic_machine();

        fixture.machine.transition(TestEvent::Start).unwrap();
        assert_eq!(fixture.machine.current_state().name, "running");
    }

    #[test]
    fn test_store_fixture() {
        let fixture = StoreTestFixture::basic_store();

        assert_eq!(fixture.store.get().counter, 0);
        assert_eq!(fixture.store.get().name, "");
    }

    #[test]
    fn test_mock_adapter() {
        let adapter = MockAdapter::new();

        // Initially no events
        assert_eq!(adapter.get_received_events().len(), 0);

        // Adapter is healthy by default
        let stats = adapter.stats();
        assert!(stats.healthy);
        assert_eq!(stats.events_sent, 0);
    }

    #[test]
    fn test_test_environment() {
        let env = TestEnvironment::new()
            .set("TEST_VAR", "test_value")
            .set("ANOTHER_VAR", "another_value");

        // Variables not set yet
        assert_eq!(std::env::var("TEST_VAR"), Err(std::env::VarError::NotPresent));

        env.apply();

        // Variables should be set
        assert_eq!(std::env::var("TEST_VAR"), Ok("test_value".to_string()));
        assert_eq!(std::env::var("ANOTHER_VAR"), Ok("another_value".to_string()));

        // Cleanup happens automatically in Drop
    }

    #[test]
    fn test_temp_config_file() {
        let content = r#"{"test": "config"}"#;
        let temp_file = TempConfigFile::new(content).unwrap();

        // File should exist and contain content
        assert!(temp_file.path().exists());
        let read_content = std::fs::read_to_string(temp_file.path()).unwrap();
        assert_eq!(read_content, content);

        // File is cleaned up automatically
    }

    #[test]
    fn test_benchmark_helper() {
        let (duration, results) = benchmark(100, || 42);

        assert!(duration > std::time::Duration::from_nanos(0));
        assert_eq!(results.len(), 100);
        assert!(results.iter().all(|&x| x == 42));
    }
}
