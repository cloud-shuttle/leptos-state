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
    println!("=== History State Machine Example ===");
    
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
    
    println!("Machine created with history state 'active_history'");
    
    // Test the state machine flow
    let initial = machine.initial_state();
    println!("1. Initial state: {:?}", initial.value());
    
    let active = machine.transition(&initial, TestEvent::Start);
    println!("2. After Start event: {:?}", active.value());
    
    let paused = machine.transition(&active, TestEvent::Pause);
    println!("3. After Pause event: {:?}", paused.value());
    
    let idle = machine.transition(&paused, TestEvent::Stop);
    println!("4. After Stop event: {:?}", idle.value());
    
    // Test history restoration
    println!("\n=== Testing History Restoration ===");
    if let Some(restored) = machine.transition_to_history("active_history") {
        println!("✓ History restored successfully!");
        println!("  Restored state: {:?}", restored.value());
        
        // Verify it restored to the paused state
        if let StateValue::Compound { parent, child } = &restored.value() {
            if parent == "active" && **child == StateValue::Simple("paused".to_string()) {
                println!("✓ Correctly restored to 'active.paused' state!");
            } else {
                println!("✗ Incorrect restoration - expected 'active.paused', got '{}.{:?}'", parent, **child);
            }
        } else {
            println!("✗ Expected compound state, got: {:?}", restored.value());
        }
    } else {
        println!("✗ History restoration failed!");
    }
    
    println!("\n=== History Test Complete ===");
}
