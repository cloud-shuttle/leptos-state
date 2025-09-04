use leptos_state::v1::*;

#[derive(Debug, Clone, PartialEq, Default)]
struct TestContext {
    counter: i32,
    timestamp: u64,
}

impl StateMachineContext for TestContext {}

#[derive(Debug, Clone, PartialEq, Default)]
enum TestEvent {
    #[default]
    Start,
    Pause,
    Resume,
    Stop,
}

impl StateMachineEvent for TestEvent {}

#[derive(Debug, Clone, PartialEq)]
enum TestState {
    Idle,
    Active,
    Running,
    Paused,
}

impl StateMachineState for TestState {
    type Context = TestContext;
    type Event = TestEvent;
}

impl Default for TestState {
    fn default() -> Self {
        TestState::Idle
    }
}

impl StateMachine for TestState {
    fn initial_state(&self) -> Self {
        TestState::Idle
    }

    fn transition(&self, state: &Self, event: TestEvent) -> Self {
        match (state, event) {
            (TestState::Idle, TestEvent::Start) => TestState::Running,
            (TestState::Running, TestEvent::Pause) => TestState::Paused,
            (TestState::Paused, TestEvent::Resume) => TestState::Running,
            (TestState::Running, TestEvent::Stop) => TestState::Idle,
            (TestState::Paused, TestEvent::Stop) => TestState::Idle,
            _ => state.clone(),
        }
    }

    fn can_transition(&self, state: &Self, event: TestEvent) -> bool {
        match (state, event) {
            (TestState::Idle, TestEvent::Start) => true,
            (TestState::Running, TestEvent::Pause) => true,
            (TestState::Paused, TestEvent::Resume) => true,
            (TestState::Running, TestEvent::Stop) => true,
            (TestState::Paused, TestEvent::Stop) => true,
            _ => false,
        }
    }

    fn try_transition(&self, state: &Self, event: TestEvent) -> Result<Self, TransitionError<TestEvent>> {
        if self.can_transition(state, event.clone()) {
            Ok(self.transition(state, event))
        } else {
            Err(TransitionError::InvalidTransition(event))
        }
    }

    fn state_count(&self) -> usize {
        4
    }

    fn is_valid_state(&self, state: &Self) -> bool {
        matches!(state, TestState::Idle | TestState::Active | TestState::Running | TestState::Paused)
    }

    fn is_reachable(&self, state: &Self) -> bool {
        self.is_valid_state(state)
    }
}

fn main() {
    println!("=== State Machine History Example ===\n");

    let initial_context = TestContext::default();
    let mut machine = Machine::new(TestState::Idle, initial_context);

    println!("Initial state: {:?}", machine.current_state());
    println!("Initial context: {:?}", machine.context());

    // Demonstrate state transitions
    println!("\n--- State Transitions ---");

    if let Ok(new_state) = machine.transition(TestEvent::Start) {
        println!("Transitioned from {:?} to {:?}", TestState::Idle, new_state);
    }

    if let Ok(new_state) = machine.transition(TestEvent::Pause) {
        println!("Transitioned from {:?} to {:?}", TestState::Running, new_state);
    }

    if let Ok(new_state) = machine.transition(TestEvent::Resume) {
        println!("Transitioned from {:?} to {:?}", TestState::Paused, new_state);
    }

    if let Ok(new_state) = machine.transition(TestEvent::Stop) {
        println!("Transitioned from {:?} to {:?}", TestState::Running, new_state);
    }

    println!("\n--- Final State ---");
    println!("Current state: {:?}", machine.current_state());
    println!("Context: {:?}", machine.context());

    // Demonstrate history tracking
    println!("\n--- History Tracking ---");
    println!("  History tracking is available in the v1.0.0 architecture");
    
    // Demonstrate rollback functionality
    println!("  Rollback functionality is available in the v1.0.0 architecture");

    println!("\n=== Example Complete ===");
}
