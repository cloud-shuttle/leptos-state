# 🔄 Migration Guide: v0.2.x to v1.0.0

This guide helps you migrate your leptos-state applications from v0.2.x to v1.0.0. The v1.0.0 release introduces significant architectural changes to improve type safety, performance, and maintainability.

## 🚨 **Breaking Changes Overview**

### **Major Changes**
- **Trait-first design**: All functionality is now trait-based
- **Explicit trait bounds**: Generic parameters have explicit constraints
- **New state machine API**: Completely redesigned state machine system
- **Store API changes**: New reactive store implementation
- **Feature flags**: Modular functionality with explicit feature gates

### **Removed Components**
- `MachineBuilder` (replaced with new builder pattern)
- `MachineStateImpl` (replaced with trait-based states)
- Old persistence system (replaced with new trait-based system)

## 📋 **Migration Checklist**

- [ ] **Update dependencies** to `leptos-state = "1.0.0-beta.1"`
- [ ] **Review feature flags** and enable required features
- [ ] **Update state machine implementations** to use new traits
- [ ] **Migrate store implementations** to new API
- [ ] **Update persistence code** to use new system
- [ ] **Test thoroughly** with new architecture
- [ ] **Update imports** to use `leptos_state::v1::*`

## 🔧 **Step-by-Step Migration**

### **Step 1: Update Dependencies**

#### **Before (v0.2.x)**
```toml
[dependencies]
leptos-state = "0.2"
```

#### **After (v1.0.0)**
```toml
[dependencies]
leptos-state = "1.0.0-beta.1"

# Enable required features
[features]
leptos-state = { version = "1.0.0-beta.1", features = ["persist", "devtools", "testing"] }
```

### **Step 2: Update Imports**

#### **Before (v0.2.x)**
```rust
use leptos_state::*;
use leptos_state::MachineBuilder;
use leptos_state::MachineStateImpl;
```

#### **After (v1.0.0)**
```rust
use leptos_state::v1::*;
use leptos_state::v1::builder::StateMachineBuilder;
use leptos_state::v1::traits::*;
```

### **Step 3: Migrate State Machine Implementations**

#### **Before (v0.2.x)**
```rust
#[derive(Clone, Debug, PartialEq)]
enum TrafficState {
    Red,
    Yellow,
    Green,
}

impl StateMachine for TrafficState {
    type Context = TrafficContext;
    type Event = TrafficEvent;
    
    fn initial_state() -> Self {
        TrafficState::Red
    }
    
    fn transition(state: &Self, event: Self::Event) -> Self {
        match (state, event) {
            (TrafficState::Red, TrafficEvent::Timer) => TrafficState::Green,
            (TrafficState::Green, TrafficEvent::Timer) => TrafficState::Yellow,
            (TrafficState::Yellow, TrafficEvent::Timer) => TrafficState::Red,
            _ => state.clone(),
        }
    }
}
```

#### **After (v1.0.0)**
```rust
#[derive(Clone, Debug, PartialEq, Default)]
enum TrafficState {
    #[default]
    Red,
    Yellow,
    Green,
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
            _ => state.clone(),
        }
    }
    
    fn can_transition(&self, state: &Self, event: Self::Event) -> bool {
        match (state, event) {
            (TrafficState::Red, TrafficEvent::Timer) => true,
            (TrafficState::Green, TrafficEvent::Timer) => true,
            (TrafficState::Yellow, TrafficEvent::Timer) => true,
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
    
    fn state_count(&self) -> usize { 3 }
    fn is_valid_state(&self, _state: &Self) -> bool { true }
    fn is_reachable(&self, _state: &Self) -> bool { true }
}
```

### **Step 4: Update State Machine Creation**

#### **Before (v0.2.x)**
```rust
let machine = MachineBuilder::new()
    .with_initial_state(TrafficState::Red)
    .with_context(TrafficContext::default())
    .build()?;
```

#### **After (v1.0.0)**
```rust
let context = TrafficContext::default();
let mut machine = Machine::new(TrafficState::Red, context);

// Add states with transitions
let red_state = StateNode::new("red")
    .with_value(StateValue::simple("red"))
    .with_transition(Transition::new(
        TrafficEvent::Timer,
        StateValue::simple("green")
    ));

let green_state = StateNode::new("green")
    .with_value(StateValue::simple("green"))
    .with_transition(Transition::new(
        TrafficEvent::Timer,
        StateValue::simple("yellow")
    ));

let yellow_state = StateNode::new("yellow")
    .with_value(StateValue::simple("yellow"))
    .with_transition(Transition::new(
        TrafficEvent::Timer,
        StateValue::simple("red")
    ));

machine.add_state(red_state)?;
machine.add_state(green_state)?;
machine.add_state(yellow_state)?;
```

### **Step 5: Migrate Store Implementations**

#### **Before (v0.2.x)**
```rust
#[derive(Clone, Debug, PartialEq)]
struct CounterStore {
    count: i32,
}

impl Store for CounterStore {
    fn new() -> Self {
        Self { count: 0 }
    }
    
    fn update(&mut self, action: &str, payload: Option<serde_json::Value>) -> Result<(), String> {
        match action {
            "increment" => self.count += 1,
            "decrement" => self.count -= 1,
            _ => return Err("Unknown action".to_string()),
        }
        Ok(())
    }
}
```

#### **After (v1.0.0)**
```rust
#[derive(Clone, Debug, PartialEq, Default)]
struct CounterStore {
    count: i32,
}

impl StoreState for CounterStore {}

impl Store for CounterStore {
    fn create() -> Self {
        Self { count: 0 }
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

// Usage in components
fn Counter() -> impl IntoView {
    let (store, set_store) = use_store::<CounterStore>();
    
    let increment = move |_| {
        set_store.update(|state| state.count += 1);
    };
    
    let decrement = move |_| {
        set_store.update(|state| state.count -= 1);
    };
    
    view! {
        <div>
            <h2>"Counter: " {move || store.get().count}</h2>
            <button on:click=increment>"Increment"</button>
            <button on:click=decrement>"Decrement"</button>
        </div>
    }
}
```

### **Step 6: Update Persistence Code**

#### **Before (v0.2.x)**
```rust
let store = Store::new()
    .with_persistence("counter")
    .with_backend(LocalStorageBackend::new());
```

#### **After (v1.0.0)**
```rust
let store = create_store_with_persistence::<CounterStore>("counter");
let (state, set_state) = use_store_with_persistence(store);
```

### **Step 7: Update Hooks Usage**

#### **Before (v0.2.x)**
```rust
let (machine, set_machine) = use_machine::<TrafficState>();
```

#### **After (v1.0.0)**
```rust
let machine = use_machine_with_context(TrafficState::Red, TrafficContext::default());
```

## 🛠️ **Migration Tools**

leptos-state v1.0.0 includes automated migration tools to help with the transition:

### **Migration Analyzer**
```rust
use leptos_state::v1::migration::*;

let analyzer = MigrationAnalyzer::new();
let issues = analyzer.analyze_code("old_code.rs");
let suggestions = analyzer.generate_suggestions(&issues);

for suggestion in suggestions {
    println!("Suggestion: {}", suggestion.description);
    println!("Priority: {:?}", suggestion.priority);
}
```

### **Code Transformer**
```rust
let transformer = CodeTransformer::new();
let new_code = transformer.transform("old_code.rs");
println!("Transformed code: {}", new_code);
```

### **Migration Helper**
```rust
let helper = MigrationHelper::new();
let guide = helper.generate_migration_guide(&issues);
println!("Migration guide: {}", guide);
```

## 🔍 **Common Migration Issues**

### **Issue 1: Missing Default Implementation**
**Error**: `the trait bound 'YourType: Default' is not satisfied`

**Solution**: Add `#[derive(Default)]` and `#[default]` attribute to your state/event enums:

```rust
#[derive(Clone, Debug, PartialEq, Default)]
enum YourState {
    #[default]
    Initial,
    // ... other variants
}
```

### **Issue 2: Trait Bounds Not Satisfied**
**Error**: `the trait bound 'YourType: StateMachineContext' is not satisfied`

**Solution**: Implement the required traits:

```rust
#[derive(Clone, PartialEq, Debug, Default)]
struct YourContext {
    // ... fields
}

impl StateMachineContext for YourContext {}
```

### **Issue 3: Missing Import**
**Error**: `cannot find trait 'StateMachine' in this scope`

**Solution**: Update your imports:

```rust
use leptos_state::v1::*;
use leptos_state::v1::traits::*;
```

### **Issue 4: Feature Flag Issues**
**Error**: `feature 'persist' is required`

**Solution**: Enable required features in Cargo.toml:

```toml
[dependencies]
leptos-state = { version = "1.0.0-beta.1", features = ["persist"] }
```

## 📚 **Migration Examples**

### **Complete State Machine Migration**

#### **Before (v0.2.x)**
```rust
use leptos_state::*;

#[derive(Clone, Debug, PartialEq)]
enum GameState {
    Idle,
    Playing,
    Paused,
    GameOver,
}

#[derive(Clone, Debug, PartialEq)]
enum GameEvent {
    Start,
    Pause,
    Resume,
    End,
}

impl StateMachine for GameState {
    type Context = GameContext;
    type Event = GameEvent;
    
    fn initial_state() -> Self {
        GameState::Idle
    }
    
    fn transition(state: &Self, event: Self::Event) -> Self {
        match (state, event) {
            (GameState::Idle, GameEvent::Start) => GameState::Playing,
            (GameState::Playing, GameEvent::Pause) => GameState::Paused,
            (GameState::Paused, GameEvent::Resume) => GameState::Playing,
            (GameState::Playing, GameEvent::End) => GameState::GameOver,
            _ => state.clone(),
        }
    }
}

fn Game() -> impl IntoView {
    let (machine, set_machine) = use_machine::<GameState>();
    
    view! {
        <div>
            <p>"State: " {move || format!("{:?}", machine.current_state())}</p>
            <button on:click=move |_| set_machine.transition(GameEvent::Start)>"Start"</button>
        </div>
    }
}
```

#### **After (v1.0.0)**
```rust
use leptos_state::v1::*;

#[derive(Clone, Debug, PartialEq, Default)]
enum GameState {
    #[default]
    Idle,
    Playing,
    Paused,
    GameOver,
}

#[derive(Clone, Debug, PartialEq, Default)]
enum GameEvent {
    #[default]
    Start,
    Pause,
    Resume,
    End,
}

#[derive(Clone, Debug, PartialEq, Default)]
struct GameContext {
    score: u32,
    level: u32,
}

impl StateMachineContext for GameContext {}
impl StateMachineEvent for GameEvent {}

impl StateMachineState for GameState {
    type Context = GameContext;
    type Event = GameEvent;
}

impl StateMachine for GameState {
    fn initial_state(&self) -> Self {
        GameState::Idle
    }
    
    fn transition(&self, state: &Self, event: Self::Event) -> Self {
        match (state, event) {
            (GameState::Idle, GameEvent::Start) => GameState::Playing,
            (GameState::Playing, GameEvent::Pause) => GameState::Paused,
            (GameState::Paused, GameEvent::Resume) => GameState::Playing,
            (GameState::Playing, GameEvent::End) => GameState::GameOver,
            _ => state.clone(),
        }
    }
    
    fn can_transition(&self, state: &Self, event: Self::Event) -> bool {
        match (state, event) {
            (GameState::Idle, GameEvent::Start) => true,
            (GameState::Playing, GameEvent::Pause) => true,
            (GameState::Paused, GameEvent::Resume) => true,
            (GameState::Playing, GameEvent::End) => true,
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

fn Game() -> impl IntoView {
    let initial_context = GameContext::default();
    let machine = use_machine_with_context(GameState::Idle, initial_context);
    
    view! {
        <div>
            <p>"State: " {move || format!("{:?}", machine.state())}</p>
            <p>"Score: " {move || machine.context().score}</p>
            <button on:click=move |_| machine.send(GameEvent::Start)>"Start"</button>
            <button on:click=move |_| machine.send(GameEvent::Pause)>"Pause"</button>
            <button on:click=move |_| machine.send(GameEvent::Resume)>"Resume"</button>
            <button on:click=move |_| machine.send(GameEvent::End)>"End"</button>
        </div>
    }
}
```

## ✅ **Migration Validation**

After completing the migration, validate your changes:

1. **Compilation**: Ensure the code compiles without errors
2. **Functionality**: Test all state transitions and store operations
3. **Performance**: Verify performance characteristics are maintained
4. **Integration**: Test with your Leptos application

## 🆘 **Getting Help**

If you encounter issues during migration:

1. **Check the error messages** for specific trait bound issues
2. **Review the examples** in the Quickstart Guide
3. **Use migration tools** to analyze your code
4. **Consult the API Reference** for detailed interface information
5. **Open an issue** on GitHub with your specific problem

## 🎯 **Next Steps**

After successful migration:

1. **Explore new features** like performance monitoring and DevTools
2. **Optimize performance** using the built-in benchmarking tools
3. **Add persistence** to your state machines and stores
4. **Implement testing** using the new testing framework

---

**The migration to v1.0.0 opens up new possibilities for building robust, performant state management in your Leptos applications. Take advantage of the improved type safety and new features!** 🚀
