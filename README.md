# 🚀 leptos-state

[![Crates.io](https://img.shields.io/crates/v/leptos-state)](https://crates.io/crates/leptos-state)
[![Documentation](https://img.shields.io/docsrs/leptos-state)](https://docs.rs/leptos-state)
[![License](https://img.shields.io/crates/l/leptos-state)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

**Advanced state management for [Leptos](https://github.com/leptos-rs/leptos) applications with state machines, reactive stores, and persistence.**

## ✨ **Features**

### 🎯 **State Machines**
- **XState-inspired API** with guards, actions, and nested states
- **Type-safe transitions** with compile-time validation
- **Context management** for complex state logic
- **Visualization tools** for debugging and documentation

### 🗄️ **Reactive Stores**
- **Zustand-inspired API** with Leptos integration
- **Automatic reactivity** using Leptos signals
- **Middleware support** for logging, persistence, and more
- **DevTools integration** for state inspection

### 💾 **Persistence**
- **Multiple backends** (LocalStorage, Memory, IndexedDB)
- **Serialization formats** (JSON, YAML, MessagePack)
- **Automatic state restoration** on page reload
- **Migration support** for schema changes

### 🧪 **Testing Framework**
- **Property-based testing** with `proptest`
- **State machine testing** utilities
- **Performance benchmarking** with `criterion`
- **Test case generation** for complex scenarios

### 🚀 **Performance**
- **WASM-first design** for web applications
- **Native Rust support** for server-side usage
- **Memory optimization** with efficient data structures
- **Performance monitoring** and optimization tools

### 🔧 **Developer Experience**
- **Comprehensive error handling** with actionable messages
- **Type-safe APIs** with explicit trait bounds
- **Feature flags** for modular functionality
- **Migration tools** from v0.2.x

## 📦 **Installation**

### **Basic Installation**
```toml
[dependencies]
leptos-state = "1.0.0-rc.1"
leptos = "0.8"
```

### **With Feature Flags**
```toml
[dependencies]
leptos-state = { version = "1.0.0-rc.1", features = ["persist", "devtools", "testing"] }
```

### **Available Features**
- `persist` - Persistence system with multiple backends
- `devtools` - Browser DevTools integration
- `testing` - Testing framework and utilities
- `codegen` - Code generation for state machines

## 🚀 **Quick Start**

### **Basic State Machine**
```rust
use leptos::*;
use leptos_state::v1::*;

#[derive(Clone, Debug, PartialEq, Default)]
enum TrafficState {
    #[default]
    Red,
    Yellow,
    Green,
}

#[derive(Clone, Debug, PartialEq, Default)]
enum TrafficEvent {
    #[default]
    Timer,
}

#[derive(Clone, Debug, PartialEq, Default)]
struct TrafficContext {
    duration: u32,
}

impl StateMachineContext for TrafficContext {}
impl StateMachineEvent for TrafficEvent {}

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

fn TrafficLight() -> impl IntoView {
    let initial_context = TrafficContext::default();
    let machine = use_machine_with_context(TrafficState::Red, initial_context);
    
    view! {
        <div>
            <h2>"Traffic Light: " {move || format!("{:?}", machine.state())}</h2>
            <button on:click=move |_| machine.send(TrafficEvent::Timer)>"Next"</button>
        </div>
    }
}
```

### **Reactive Store**
```rust
use leptos::*;
use leptos_state::v1::*;

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

### **With Persistence**
```rust
use leptos_state::v1::*;

// Create a store with persistence
let store = create_store_with_persistence::<CounterStore>("counter");
let (state, set_state) = use_store_with_persistence(store);

// State automatically persists to LocalStorage
// and restores on page reload
```

## 📚 **Documentation**

### **User Guides**
- **[Quickstart Guide](docs/user-guide/QUICKSTART.md)** - Get started in minutes
- **[Performance Guide](docs/user-guide/PERFORMANCE.md)** - Optimize your applications
- **[Migration Guide](docs/migration/V0_2_TO_V1_0_MIGRATION.md)** - Migrate from v0.2.x

### **API Reference**
- **[Complete API Reference](docs/api-reference/API_REFERENCE.md)** - All public APIs
- **[Examples](examples/)** - Working examples and demos
- **[Changelog](docs/CHANGELOG.md)** - Version history and changes

### **Development**
- **[Architecture Overview](docs/development/ARCHITECTURE.md)** - System design and principles
- **[Contributing Guide](CONTRIBUTING.md)** - How to contribute
- **[Testing Guide](docs/development/TESTING.md)** - Testing best practices

## 🔧 **Examples**

### **Basic Examples**
- **[Counter](examples/counter/)** - Simple state management
- **[Todo App](examples/todo/)** - CRUD operations with persistence
- **[Traffic Light](examples/traffic-light/)** - State machine basics

### **Advanced Examples**
- **[E-commerce Cart](examples/ecommerce/)** - Complex state with persistence
- **[Game State](examples/game/)** - Nested state machines
- **[Form Management](examples/forms/)** - Form state with validation

### **Integration Examples**
- **[Leptos SSR](examples/ssr/)** - Server-side rendering
- **[WASM Web](examples/wasm/)** - WebAssembly deployment
- **[Native App](examples/native/)** - Desktop application

## 🚀 **Performance**

### **Benchmarks**
```bash
# Run performance benchmarks
cargo bench --features "testing,persist"

# Run specific benchmarks
cargo bench --bench performance_benchmarks
```

### **Performance Features**
- **Lazy loading** for large state trees
- **Connection pooling** for persistence backends
- **Memory optimization** with efficient data structures
- **Performance monitoring** with built-in tools

## 🧪 **Testing**

### **Unit Testing**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos_state::v1::testing::*;
    
    #[test]
    fn test_traffic_light_transitions() {
        let machine = TrafficState::default();
        let context = TrafficContext::default();
        
        let result = machine.try_transition(&TrafficState::Red, TrafficEvent::Timer);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), TrafficState::Green);
    }
}
```

### **Property-Based Testing**
```rust
#[test]
fn test_traffic_light_properties() {
    let tester = StateMachineTester::new(TrafficState::default());
    let result = tester.property_test(|machine, events| {
        // Test that all transitions are valid
        events.iter().all(|event| {
            machine.can_transition(&machine.current_state(), event.clone())
        })
    });
    
    assert!(result.is_ok());
}
```

## 🔄 **Migration from v0.2.x**

leptos-state v1.0.0 is a complete rewrite with breaking changes. We provide comprehensive migration tools:

### **Migration Tools**
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

### **Migration Steps**
1. **Update dependencies** to `leptos-state = "1.0.0-rc.1"`
2. **Run migration analysis** to identify issues
3. **Apply automatic transformations** where possible
4. **Manually update** remaining code patterns
5. **Test thoroughly** with new architecture

See the [Migration Guide](docs/migration/V0_2_TO_V1_0_MIGRATION.md) for detailed instructions.

## 🤝 **Contributing**

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### **Development Setup**
```bash
# Clone the repository
git clone https://github.com/cloud-shuttle/leptos-state.git
cd leptos-state

# Install dependencies
cargo build

# Run tests
cargo test --features "testing,persist,devtools"

# Run benchmarks
cargo bench --features "testing,persist"
```

### **Areas for Contribution**
- **Documentation** - Improve guides and examples
- **Testing** - Add test coverage and benchmarks
- **Performance** - Optimize algorithms and data structures
- **Features** - Implement new functionality
- **Examples** - Create real-world use cases

## 📄 **License**

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 **Acknowledgments**

- **[Leptos](https://github.com/leptos-rs/leptos)** - The amazing Rust web framework
- **[XState](https://xstate.js.org/)** - Inspiration for state machine design
- **[Zustand](https://github.com/pmndrs/zustand)** - Inspiration for store API
- **[Rust Community](https://www.rust-lang.org/community)** - For the excellent ecosystem

## 📞 **Support**

### **Getting Help**
- **[GitHub Issues](https://github.com/cloud-shuttle/leptos-state/issues)** - Report bugs and request features
- **[GitHub Discussions](https://github.com/cloud-shuttle/leptos-state/discussions)** - Ask questions and share ideas
- **[Documentation](https://docs.rs/leptos-state)** - Comprehensive API reference

### **Community**
- **Discord**: Join our community server
- **Twitter**: Follow for updates and announcements
- **Blog**: Read about new features and best practices

---

**Built with ❤️ by the CloudShuttle team and contributors**

*Ready to build amazing state management for your Leptos applications? [Get started now!](docs/user-guide/QUICKSTART.md)* 🚀