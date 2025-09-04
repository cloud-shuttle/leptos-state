# 🚀 Quickstart Guide - leptos-state v1.0.0

Welcome to **leptos-state v1.0.0**! This guide will get you up and running with the new architecture in minutes.

## 📦 Installation

Add leptos-state to your `Cargo.toml`:

```toml
[dependencies]
leptos-state = "1.0.0-beta.1"
leptos = "0.8"
```

### Feature Flags

Enable the features you need:

```toml
[dependencies]
leptos-state = { version = "1.0.0-beta.1", features = ["persist", "devtools", "testing"] }
```

**Available Features:**
- `persist` - State persistence (LocalStorage, Memory)
- `devtools` - Browser DevTools integration
- `testing` - Testing framework and utilities
- `migration` - Migration tools for v0.2.x
- `wasm` - WebAssembly optimizations
- `ssr` - Server-side rendering support

## 🏪 Getting Started with Stores

### Basic Store

```rust
use leptos_state::v1::*;
use leptos::*;

#[derive(Clone, PartialEq, Debug, Default)]
struct CounterStore {
    count: i32,
    name: String,
}

impl StoreState for CounterStore {}

impl Store for CounterStore {
    fn create() -> Self {
        Self { count: 0, name: "Counter".to_string() }
    }
    
    fn create_with_state(state: Self) -> Self {
        state
    }
    
    fn update<F>(&mut self, f: F) 
    where 
        F: FnOnce(&mut Self) {
        f(self);
    }
    
    fn get(&self) -> &Self {
        self
    }
    
    fn get_mut(&mut self) -> &mut Self {
        self
    }
}

fn Counter() -> impl IntoView {
    let (store, set_store) = use_store::<CounterStore>();
    
    let increment = move |_| {
        set_store.update(|state| state.count += 1);
    };
    
    let set_name = move |name: String| {
        set_store.update(|state| state.name = name);
    };
    
    view! {
        <div>
            <h2>"Counter: " {move || store.get().count}</h2>
            <p>"Name: " {move || store.get().name}</p>
            <button on:click=increment>
                "Increment"
            </button>
            <input 
                placeholder="Enter name"
                on:change=move |ev| {
                    let name = event_target_value(&ev);
                    set_name(name);
                }
            />
        </div>
    }
}
```

### Store with Middleware

```rust
use leptos_state::v1::*;

#[derive(Clone, PartialEq, Debug, Default)]
struct LoggedStore {
    count: i32,
}

impl StoreState for LoggedStore {}

impl Store for LoggedStore {
    // ... same implementation as above
}

fn LoggedCounter() -> impl IntoView {
    let store = create_store_with_middleware::<LoggedStore>()
        .with_logging() // Add logging middleware
        .with_persistence("counter") // Add persistence
        .with_history(10); // Keep last 10 states
    
    let (state, set_state) = use_store_with_middleware(store);
    
    // ... rest of component
}
```

## 🎯 State Machines

### Basic State Machine

```rust
use leptos_state::v1::*;

#[derive(Clone, Debug, PartialEq, Default)]
struct TrafficContext {
    timer: u32,
    emergency_mode: bool,
}

impl StateMachineContext for TrafficContext {}

#[derive(Clone, Debug, PartialEq)]
enum TrafficEvent {
    Timer,
    EmergencyStop,
    EmergencyClear,
}

impl StateMachineEvent for TrafficEvent {}

#[derive(Clone, Debug, PartialEq)]
enum TrafficState {
    Red,
    Yellow,
    Green,
    Emergency,
}

impl StateMachineState for TrafficState {
    type Context = TrafficContext;
    type Event = TrafficEvent;
}

impl StateMachine for TrafficState {
    fn initial_state(&self) -> Self {
        TrafficState::Red
    }
    
    fn transition(&self, state: &Self, event: Self::Event) -> Self {
        match (state, event) {
            (TrafficState::Red, TrafficEvent::Timer) => TrafficState::Green,
            (TrafficState::Green, TrafficEvent::Timer) => TrafficState::Yellow,
            (TrafficState::Yellow, TrafficEvent::Timer) => TrafficState::Red,
            (_, TrafficEvent::EmergencyStop) => TrafficState::Emergency,
            (TrafficState::Emergency, TrafficEvent::EmergencyClear) => TrafficState::Red,
            _ => state.clone(),
        }
    }
    
    fn can_transition(&self, state: &Self, event: Self::Event) -> bool {
        match (state, event) {
            (TrafficState::Red, TrafficEvent::Timer) => true,
            (TrafficState::Green, TrafficEvent::Timer) => true,
            (TrafficState::Yellow, TrafficEvent::Timer) => true,
            (_, TrafficEvent::EmergencyStop) => true,
            (TrafficState::Emergency, TrafficEvent::EmergencyClear) => true,
            _ => false,
        }
    }
    
    fn try_transition(&self, state: &Self, event: Self::Event) -> Result<Self, TransitionError<Self::Event>> {
        if self.can_transition(state, event.clone()) {
            Ok(self.transition(state, event))
        } else {
            Err(TransitionError::InvalidTransition(event))
        }
    }
    
    fn state_count(&self) -> usize { 4 }
    fn is_valid_state(&self, _state: &Self) -> bool { true }
    fn is_reachable(&self, _state: &Self) -> bool { true }
}

fn TrafficLight() -> impl IntoView {
    let initial_context = TrafficContext::default();
    let machine = use_machine_with_context(TrafficState::Red, initial_context);
    
    let advance_timer = move |_| {
        machine.send(TrafficEvent::Timer);
    };
    
    let emergency_stop = move |_| {
        machine.send(TrafficEvent::EmergencyStop);
    };
    
    let clear_emergency = move |_| {
        machine.send(TrafficEvent::EmergencyClear);
    };
    
    view! {
        <div>
            <div class="traffic-light">
                <div class="light red" class:active={move || machine.state() == TrafficState::Red}></div>
                <div class="light yellow" class:active={move || machine.state() == TrafficState::Yellow}></div>
                <div class="light green" class:active={move || machine.state() == TrafficState::Green}></div>
                <div class="light emergency" class:active={move || machine.state() == TrafficState::Emergency}></div>
            </div>
            
            <div class="controls">
                <button on:click=advance_timer>"Advance Timer"</button>
                <button on:click=emergency_stop>"Emergency Stop"</button>
                <button on:click=clear_emergency>"Clear Emergency"</button>
            </div>
            
            <p>"Current State: " {move || format!("{:?}", machine.state())}</p>
            <p>"Timer: " {move || machine.context().timer}</p>
        </div>
    }
}
```

### State Machine with Guards and Actions

```rust
use leptos_state::v1::*;
use std::sync::Arc;

// Guards
struct CanIncrementGuard;

impl Guard<TrafficContext, TrafficEvent> for CanIncrementGuard {
    fn check(&self, context: &TrafficContext, _event: &TrafficEvent) -> bool {
        !context.emergency_mode
    }
    
    fn description(&self) -> &'static str {
        "Check if increment is allowed (not in emergency mode)"
    }
}

// Actions
struct LogTransitionAction;

impl Action<TrafficContext> for LogTransitionAction {
    fn execute(&self, context: &mut TrafficContext) -> Result<(), ActionError> {
        println!("State transition logged at timer: {}", context.timer);
        context.timer += 1;
        Ok(())
    }
    
    fn description(&self) -> &'static str {
        "Log the state transition and increment timer"
    }
}

// Enhanced state machine with guards and actions
fn create_enhanced_traffic_machine() -> Machine<TrafficContext, TrafficEvent, TrafficState> {
    let context = TrafficContext::default();
    let mut machine = Machine::new(TrafficState::Red, context);
    
    let red_state = StateNode::new("red")
        .with_value(StateValue::simple("red"))
        .with_transition(
            Transition::new(TrafficEvent::Timer, StateValue::simple("green"))
                .with_guard(Arc::new(CanIncrementGuard))
                .with_action(Arc::new(LogTransitionAction))
        );
    
    let green_state = StateNode::new("green")
        .with_value(StateValue::simple("green"))
        .with_transition(
            Transition::new(TrafficEvent::Timer, StateValue::simple("yellow"))
                .with_guard(Arc::new(CanIncrementGuard))
                .with_action(Arc::new(LogTransitionAction))
        );
    
    let yellow_state = StateNode::new("yellow")
        .with_value(StateValue::simple("yellow"))
        .with_transition(
            Transition::new(TrafficEvent::Timer, StateValue::simple("red"))
                .with_guard(Arc::new(CanIncrementGuard))
                .with_action(Arc::new(LogTransitionAction))
        );
    
    machine.add_state(red_state);
    machine.add_state(green_state);
    machine.add_state(yellow_state);
    
    machine
}
```

## 💾 Persistence

### Basic Persistence

```rust
use leptos_state::v1::*;

fn PersistentCounter() -> impl IntoView {
    let store = create_store_with_persistence::<CounterStore>("counter");
    let (state, set_state) = use_store_with_persistence(store);
    
    // State automatically persists to LocalStorage (WASM) or Memory (native)
    
    view! {
        <div>
            <h2>"Persistent Counter: " {move || state.get().count}</h2>
            <button on:click=move |_| set_state.update(|s| s.count += 1)>
                "Increment"
            </button>
        </div>
    }
}
```

### Custom Persistence Configuration

```rust
use leptos_state::v1::*;

fn CustomPersistentStore() -> impl IntoView {
    let store = create_store_with_persistence::<CounterStore>("custom_counter")
        .with_prefix("my_app") // Custom storage prefix
        .with_serialization_format(SerializationFormat::Json) // JSON serialization
        .with_compression(true); // Enable compression
    
    // ... rest of component
}
```

## 🛠️ DevTools Integration

### Enable DevTools

```rust
use leptos_state::v1::*;

fn App() -> impl IntoView {
    // Enable DevTools in development
    #[cfg(debug_assertions)]
    enable_devtools();
    
    view! {
        <div>
            <h1>"My App"</h1>
            <Counter />
            <TrafficLight />
        </div>
    }
}
```

### Custom DevTools Configuration

```rust
use leptos_state::v1::*;

fn configure_devtools() {
    let devtools = DevTools::new()
        .with_history_limit(100)
        .with_performance_tracking(true)
        .with_export_format(ExportFormat::Json);
    
    enable_devtools_with_config(devtools);
}
```

## 🧪 Testing

### Unit Testing

```rust
use leptos_state::v1::*;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_counter_increment() {
        let mut store = CounterStore::create();
        store.update(|state| state.count += 1);
        assert_eq!(store.get().count, 1);
    }
    
    #[test]
    fn test_traffic_light_transition() {
        let context = TrafficContext::default();
        let machine = create_enhanced_traffic_machine();
        
        assert!(machine.can_transition(&TrafficState::Red, &TrafficEvent::Timer));
        
        let result = machine.transition(TrafficEvent::Timer);
        assert!(result.is_ok());
    }
}
```

### Property-Based Testing

```rust
use leptos_state::v1::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_store_properties(count in 0..1000i32) {
        let mut store = CounterStore::create();
        store.update(|state| state.count = count);
        assert_eq!(store.get().count, count);
    }
}
```

## 🔄 Migration from v0.2.x

### Using Migration Tools

```rust
use leptos_state::v1::*;

fn migrate_old_code() {
    let analyzer = MigrationAnalyzer::new();
    let helper = MigrationHelper::new();
    
    // Analyze old code
    let issues = analyzer.analyze_code("old_code.rs");
    
    // Get migration suggestions
    let suggestions = helper.generate_suggestions(&issues);
    
    // Transform code
    let transformer = CodeTransformer::new();
    let new_code = transformer.transform("old_code.rs");
    
    println!("Migration suggestions: {:?}", suggestions);
    println!("Transformed code: {}", new_code);
}
```

## 📊 Performance Monitoring

### Built-in Performance Tracking

```rust
use leptos_state::v1::*;

fn monitor_performance() {
    let benchmark = PerformanceBenchmark::new()
        .with_thresholds(PerformanceThresholds {
            max_transition_time: Duration::from_millis(10),
            max_memory_usage: 1024 * 1024, // 1MB
            min_performance_score: 80.0,
        });
    
    // Benchmark operations
    let result = benchmark.benchmark_operation("state_transition", || {
        // Your operation here
        0 // Return memory usage
    }, 1000);
    
    println!("Performance score: {}", result.performance_score);
    
    // Get optimization suggestions
    let suggestions = benchmark.get_suggestions();
    for suggestion in suggestions {
        println!("Suggestion: {}", suggestion.description);
    }
}
```

## 🚀 Next Steps

1. **Explore Examples**: Check out the `examples/` directory for more complex use cases
2. **Read API Reference**: Dive into the detailed API documentation
3. **Join Community**: Get help and share your experiences
4. **Contribute**: Help improve leptos-state with your feedback and contributions

## 🆘 Need Help?

- **Documentation**: [docs.rs/leptos-state](https://docs.rs/leptos-state)
- **Examples**: [GitHub Examples](https://github.com/cloud-shuttle/leptos-state/tree/main/examples)
- **Issues**: [GitHub Issues](https://github.com/cloud-shuttle/leptos-state/issues)
- **Discussions**: [GitHub Discussions](https://github.com/cloud-shuttle/leptos-state/discussions)

---

**Happy coding with leptos-state v1.0.0! 🎉**
