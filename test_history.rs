use leptos_state::machine::*;

#[derive(Debug, Clone, PartialEq, Default)]
struct TestContext {
    count: i32,
}

#[derive(Debug, Clone, PartialEq)]
enum TestEvent {
    Start,
    Stop,
    Pause,
    Resume,
}

impl Event for TestEvent {
    fn event_type(&self) -> &str {
        match self {
            TestEvent::Start => "start",
            TestEvent::Stop => "stop",
            TestEvent::Pause => "pause",
            TestEvent::Resume => "resume",
        }
    }
}

fn main() {
    println!("Testing history functionality...");
    
    // Create a simple machine with history
    let base_machine = MachineBuilder::<TestContext, TestEvent>::new()
        .state("idle")
            .on(TestEvent::Start, "active")
        .state("active")
            .child_state("running")
                .on(TestEvent::Pause, "paused")
                .parent()
            .child_state("paused")
                .on(TestEvent::Resume, "running")
                .parent()
            .initial_child("running")
            .on(TestEvent::Stop, "idle")
        .initial("idle")
        .build();
    
    let history_state = HistoryState::shallow("active").with_default("running");
    
    let machine = HistoryMachine::new(base_machine)
        .add_history_state("active_history".to_string(), history_state);
    
    // Test basic functionality
    let initial = machine.initial_state();
    println!("Initial state: {:?}", initial.value());
    
    let active = machine.transition(&initial, TestEvent::Start);
    println!("After Start: {:?}", active.value());
    
    let paused = machine.transition(&active, TestEvent::Pause);
    println!("After Pause: {:?}", paused.value());
    
    let idle = machine.transition(&paused, TestEvent::Stop);
    println!("After Stop: {:?}", idle.value());
    
    // Test history restoration
    if let Some(restored) = machine.transition_to_history("active_history") {
        println!("History restored: {:?}", restored.value());
        println!("History test PASSED!");
    } else {
        println!("History test FAILED!");
    }
}
