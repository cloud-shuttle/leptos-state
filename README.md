# 🚀 **leptos-state** - Powerful State Management for Leptos

[![Crates.io](https://img.shields.io/crates/v/leptos-state)](https://crates.io/crates/leptos-state)
[![Documentation](https://img.shields.io/docsrs/leptos-state)](https://docs.rs/leptos-state)
[![License](https://img.shields.io/crates/l/leptos-state)](https://github.com/cloud-shuttle/leptos-state/blob/main/LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)

**The definitive state management solution for Leptos applications** - featuring stores, state machines, middleware, and DevTools integration.

> 🎉 **v1.0.0-alpha.1 is here!** This is a major architectural redesign with a trait-first approach, improved type safety, and enhanced Leptos v0.8+ integration. See the [migration guide](docs/migration/v1.0.0.md) for upgrading from v0.2.x.

## ✨ **Features**

- 🏪 **Reactive Stores** - Zustand-inspired API with Leptos integration
- 🎯 **State Machines** - XState-like state machines with guards and actions
- 🔌 **Middleware System** - Extensible middleware for logging, validation, and more
- 🛠️ **DevTools Integration** - Browser DevTools for state inspection and debugging
- 💾 **Persistence** - Automatic state persistence with multiple storage backends
- 📊 **Visualization** - State machine diagrams and transition tracking
- 🧪 **Testing Framework** - Comprehensive testing utilities for state machines
- ⚡ **Performance Optimized** - Minimal overhead with smart reactivity
- 🌐 **WASM Ready** - Full WebAssembly support for web applications

## 🚀 **Quick Start**

### Installation

```toml
[dependencies]
leptos-state = "1.0.0-alpha.1"
leptos = "0.8"
```

### Simple Store (v1.0.0-alpha.1)

```rust
use leptos_state::v1::*;
use leptos_state::{use_store, provide_store};

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
    
    fn update(&mut self, action: &str, payload: Option<serde_json::Value>) -> Result<(), String> {
        match action {
            "increment" => self.count += 1,
            "set_name" => if let Some(payload) = payload {
                self.name = payload.as_str().unwrap_or("Counter").to_string();
            },
            _ => return Err("Unknown action".to_string()),
        }
        Ok(())
    }
}

fn Counter() -> impl IntoView {
    let (store, set_store) = use_store::<CounterStore>();
    
    let increment = move |_| {
        set_store.update(|state| state.count += 1);
    };
    
    view! {
        <div>
            <h2>"Counter: " {move || store.get().count}</h2>
            <p>"Name: " {move || store.get().name}</p>
            <button on:click=increment>
                "Increment"
            </button>
        </div>
    }
}
```

### State Machine (v1.0.0-alpha.1)

```rust
use leptos_state::v1::*;
use leptos_state::use_machine_with_context;

#[derive(Clone, PartialEq, Debug, Default)]
struct TrafficContext {
    timer: u32,
}

impl StateMachineContext for TrafficContext {}

#[derive(Clone, PartialEq, Debug)]
enum TrafficEvent {
    Timer,
    EmergencyStop,
}

impl StateMachineEvent for TrafficEvent {}
impl Default for TrafficEvent {
    fn default() -> Self { TrafficEvent::Timer }
}

#[derive(Clone, PartialEq, Debug)]
enum TrafficState {
    Red,
    Yellow,
    Green,
}

impl StateMachineState for TrafficState {
    type Context = TrafficContext;
    type Event = TrafficEvent;
}

impl Default for TrafficState {
    fn default() -> Self { TrafficState::Red }
}

impl StateMachine for TrafficState {
    fn initial_state(&self) -> Self { TrafficState::Red }
    
    fn transitions(&self) -> Vec<Transition<Self::Context, Self::Event, Self>> {
        vec![
            Transition::new(TrafficState::Red, TrafficEvent::Timer, TrafficState::Green),
            Transition::new(TrafficState::Green, TrafficEvent::Timer, TrafficState::Yellow),
            Transition::new(TrafficState::Yellow, TrafficEvent::Timer, TrafficState::Red),
        ]
    }
}

fn TrafficLight() -> impl IntoView {
    let initial_context = TrafficContext::default();
    let machine = use_machine_with_context(TrafficState::Red, initial_context);
    
    let current_light = move || {
        match machine.state() {
            TrafficState::Red => "red",
            TrafficState::Yellow => "yellow", 
            TrafficState::Green => "green",
        }
    };
    
    let next_timer = move |_| machine.send(TrafficEvent::Timer);
    
    view! {
        <div>
            <h2>"Traffic Light: " {current_light}</h2>
            <button on:click=next_timer>
                "Next Light"
            </button>
        </div>
    }
}
```

## 📚 **Documentation**

- **[📖 User Guide](https://github.com/cloud-shuttle/leptos-state/tree/main/docs/user-guide)** - Comprehensive usage guide
- **[🔧 API Reference](https://docs.rs/leptos-state)** - Complete API documentation
- **[📝 Examples](https://github.com/cloud-shuttle/leptos-state/tree/main/examples)** - Working code samples
- **[🔄 Migration Guide](https://github.com/cloud-shuttle/leptos-state/tree/main/docs/migration)** - Upgrade from v0.1.0

## 🎯 **Why leptos-state?**

### **For Leptos Developers**
- **First-class Leptos integration** - Built specifically for Leptos applications
- **Reactive by design** - Automatic updates when state changes
- **WASM optimized** - Designed for web applications

### **For State Management**
- **Familiar APIs** - Inspired by Zustand and XState
- **Type safety** - Full Rust type safety and compile-time guarantees
- **Performance** - Minimal runtime overhead with smart optimizations

### **For Production Apps**
- **Middleware ecosystem** - Extensible architecture for enterprise needs
- **DevTools support** - Professional debugging and monitoring
- **Testing utilities** - Comprehensive testing framework included

## 🔧 **Advanced Features**

### Middleware System

```rust
use leptos_state::{LoggerMiddleware, ValidationMiddleware, MiddlewareChain};

let store = create_store::<MyStore>()
    .with_middleware(
        MiddlewareChain::new()
            .add(LoggerMiddleware::new())
            .add(ValidationMiddleware::new())
    );
```

### Persistence

```rust
let machine = MachineBuilder::new()
    .state("idle")
    .build_with_persistence(PersistenceConfig {
        enabled: true,
        storage_key: "my_machine".to_string(),
        auto_save: true,
        ..Default::default()
    });
```

### Code Generation

```rust
let generator = machine.build_with_code_generation(CodeGenConfig {
    target_languages: vec![ProgrammingLanguage::Rust, ProgrammingLanguage::TypeScript],
    output_directory: "generated".to_string(),
    ..Default::default()
});

generator.generate_code()?;
```

## 🌟 **Examples**

Check out our comprehensive examples:

- **[📱 Todo App](https://github.com/cloud-shuttle/leptos-state/tree/main/examples/todo-app)** - Full-featured todo application
- **[🚦 Traffic Light](https://github.com/cloud-shuttle/leptos-state/tree/main/examples/traffic-light)** - State machine basics
- **[📊 Analytics Dashboard](https://github.com/cloud-shuttle/leptos-state/tree/main/examples/analytics-dashboard)** - Complex state management
- **[🔧 Code Generation](https://github.com/cloud-shuttle/leptos-state/tree/main/examples/codegen)** - Multi-language code generation

## 🚀 **Getting Started**

1. **Add to your project:**
   ```bash
   cargo add leptos-state
   ```

2. **Check out the examples:**
   ```bash
   git clone https://github.com/cloud-shuttle/leptos-state.git
   cd leptos-state/examples
   cargo run --bin counter
   ```

3. **Read the documentation:**
   - [User Guide](https://github.com/cloud-shuttle/leptos-state/tree/main/docs/user-guide)
   - [API Reference](https://docs.rs/leptos-state)

## 🤝 **Contributing**

We welcome contributions! Please see our [Contributing Guide](https://github.com/cloud-shuttle/leptos-state/tree/main/docs/contributing) for details.

- 🐛 **Report bugs** on [GitHub Issues](https://github.com/cloud-shuttle/leptos-state/issues)
- 💡 **Request features** via [GitHub Discussions](https://github.com/cloud-shuttle/leptos-state/discussions)
- 📝 **Submit PRs** for bug fixes and improvements

## 📄 **License**

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## 🙏 **Acknowledgments**

- Built with ❤️ for the [Leptos](https://github.com/leptos-rs/leptos) community
- Inspired by [Zustand](https://github.com/pmndrs/zustand) and [XState](https://github.com/statelyai/xstate)
- Part of the [Cloud Shuttle](https://cloud-shuttle.com) ecosystem

---

**Ready to build amazing Leptos applications?** [Get started now!](https://github.com/cloud-shuttle/leptos-state)