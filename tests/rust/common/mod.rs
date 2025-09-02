//! Common testing utilities and helpers for leptos-state tests

use leptos_state::{
    machine::{MachineBuilder, StateMachine},
    store::{Store, StoreContext},
    utils::StateResult,
};
use std::sync::Arc;
use std::time::Duration;

/// Test context for state machines
#[derive(Debug, Clone, PartialEq)]
pub struct TestContext {
    pub counter: i32,
    pub name: String,
    pub is_active: bool,
}

impl Default for TestContext {
    fn default() -> Self {
        Self {
            counter: 0,
            name: "test".to_string(),
            is_active: false,
        }
    }
}

/// Test events for state machines
#[derive(Debug, Clone, PartialEq)]
pub enum TestEvent {
    Start,
    Stop,
    Pause,
    Resume,
    Complete,
    Reset,
    Increment,
    Decrement,
    SetName(String),
    ToggleActive,
}

/// Test state machine implementation
pub struct TestMachine;

impl StateMachine for TestMachine {
    type Context = TestContext;
    type Event = TestEvent;
    type State = TestState;

    fn initial() -> Self::State {
        TestState::Idle
    }

    fn transition(state: &Self::State, event: Self::Event) -> Self::State {
        match (state, event) {
            (TestState::Idle, TestEvent::Increment) => TestState::Counting,
            (TestState::Counting, TestEvent::Decrement) => TestState::Idle,
            (TestState::Counting, TestEvent::Increment) => TestState::Counting,
            (TestState::Idle, TestEvent::Decrement) => TestState::Idle,
            (_, TestEvent::Reset) => TestState::Idle,
            _ => state.clone(),
        }
    }
}

/// Test states
#[derive(Debug, Clone, PartialEq)]
pub enum TestState {
    Idle,
    Active,
    Paused,
    Finished,
    Counting,
    Counter(i32),
    Error,
}

impl TestState {
    pub fn is_idle(&self) -> bool {
        matches!(self, TestState::Idle)
    }

    pub fn is_counting(&self) -> bool {
        matches!(self, TestState::Counting)
    }

    pub fn is_error(&self) -> bool {
        matches!(self, TestState::Error)
    }
}

/// Test store implementation
#[derive(Clone)]
pub struct TestStore;

impl Store for TestStore {
    type State = TestState;

    fn create() -> Self::State {
        TestState::Idle
    }
}

/// Test utilities
pub mod utils {
    use super::*;
    use leptos_state::machine::MachineStateImpl;

    /// Create a test machine with builder
    pub fn create_test_machine() -> impl StateMachine<Context = TestContext, Event = TestEvent> {
        MachineBuilder::new()
            .state("idle")
                .on(TestEvent::Increment, "counting")
                .on(TestEvent::Decrement, "idle")
            .state("counting")
                .on(TestEvent::Increment, "counting")
                .on(TestEvent::Decrement, "idle")
                .on(TestEvent::Reset, "idle")
            .initial("idle")
            .build()
    }

    /// Create a test machine state
    pub fn create_test_state(state: TestState, context: TestContext) -> MachineStateImpl<TestContext> {
        MachineStateImpl {
            value: match state {
                TestState::Idle => leptos_state::machine::states::StateValue::Simple("idle".to_string()),
                TestState::Counting => leptos_state::machine::states::StateValue::Simple("counting".to_string()),
                TestState::Error => leptos_state::machine::states::StateValue::Simple("error".to_string()),
            },
            context,
        }
    }

    /// Assert state machine transitions
    pub fn assert_transition<M: StateMachine>(
        machine: &M,
        from_state: &M::State,
        event: M::Event,
        expected_state: &M::State,
    ) where
        M::State: PartialEq + std::fmt::Debug,
        M::Event: Clone,
    {
        let result = M::transition(from_state, event.clone());
        assert_eq!(
            result, *expected_state,
            "Transition from {:?} with {:?} should result in {:?}, but got {:?}",
            from_state, event, expected_state, result
        );
    }

    /// Wait for async operations (for testing)
    pub async fn wait_for_async(duration: Duration) {
        tokio::time::sleep(duration).await;
    }

    /// Create a test context with custom values
    pub fn test_context(counter: i32, name: &str, is_active: bool) -> TestContext {
        TestContext {
            counter,
            name: name.to_string(),
            is_active,
        }
    }
}

/// Test macros
#[macro_export]
macro_rules! assert_state {
    ($state:expr, $expected:pat) => {
        assert!(matches!($state, $expected), "Expected {:?} to match pattern", $state);
    };
}

#[macro_export]
macro_rules! assert_context {
    ($context:expr, $field:ident, $expected:expr) => {
        assert_eq!($context.$field, $expected, "Expected {} to be {:?}", stringify!($field), $expected);
    };
}

/// Performance testing utilities
pub mod performance {
    use super::*;
    use std::time::Instant;

    /// Measure execution time of a function
    pub fn measure_time<F, R>(f: F) -> (R, Duration)
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        (result, duration)
    }

    /// Benchmark a function multiple times
    pub fn benchmark<F, R>(f: F, iterations: usize) -> Vec<Duration>
    where
        F: Fn() -> R,
    {
        (0..iterations)
            .map(|_| {
                let start = Instant::now();
                f();
                start.elapsed()
            })
            .collect()
    }

    /// Calculate statistics from benchmark results
    pub fn benchmark_stats(durations: &[Duration]) -> (Duration, Duration, Duration) {
        let total: Duration = durations.iter().sum();
        let avg = total / durations.len() as u32;
        let min = durations.iter().min().unwrap_or(&Duration::ZERO);
        let max = durations.iter().max().unwrap_or(&Duration::ZERO);
        (avg, *min, *max)
    }
}

/// Memory testing utilities
pub mod memory {
    use std::alloc::{alloc, dealloc, Layout};
    use std::ptr;

    /// Track memory allocations (basic implementation)
    pub struct MemoryTracker {
        allocations: std::collections::HashMap<*mut u8, usize>,
        total_allocated: usize,
    }

    impl MemoryTracker {
        pub fn new() -> Self {
            Self {
                allocations: std::collections::HashMap::new(),
                total_allocated: 0,
            }
        }

        pub fn allocate(&mut self, size: usize) -> *mut u8 {
            let layout = Layout::from_size_align(size, 8).unwrap();
            let ptr = unsafe { alloc(layout) };
            if !ptr.is_null() {
                self.allocations.insert(ptr, size);
                self.total_allocated += size;
            }
            ptr
        }

        pub fn deallocate(&mut self, ptr: *mut u8) {
            if let Some(size) = self.allocations.remove(&ptr) {
                self.total_allocated -= size;
                let layout = Layout::from_size_align(size, 8).unwrap();
                unsafe { dealloc(ptr, layout) };
            }
        }

        pub fn total_allocated(&self) -> usize {
            self.total_allocated
        }

        pub fn allocation_count(&self) -> usize {
            self.allocations.len()
        }
    }

    impl Drop for MemoryTracker {
        fn drop(&mut self) {
            // Clean up any remaining allocations
            let ptrs: Vec<*mut u8> = self.allocations.keys().copied().collect();
            for ptr in ptrs {
                self.deallocate(ptr);
            }
        }
    }
}
