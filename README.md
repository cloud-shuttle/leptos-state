# Leptos State Management Library

A state management library for [Leptos](https://leptos.dev/) applications inspired by [Zustand's](https://github.com/pmndrs/zustand) simplicity and [XState's](https://xstate.js.org/) state machine capabilities.

## 🚀 Features

- **📦 Store Management**: Zustand-inspired stores with reactive updates
- **🤖 State Machines**: XState-inspired finite state machines with hierarchical states  
- **⚡ Leptos Integration**: First-class support for Leptos reactive primitives
- **🔒 Type Safety**: Ergonomic APIs with strong type safety powered by Rust
- **🏗️ Zero Boilerplate**: Minimal setup with powerful derive macros
- **🔄 Middleware Support**: Extensible middleware system for logging, persistence, etc.
- **🛠️ DevTools Ready**: Built-in support for time-travel debugging

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
leptos = "0.6"
leptos-state = "0.1"
```

## 🏃‍♂️ Quick Start

### Store Example (Zustand-style)

```rust
use leptos::*;
use leptos_state::*;

#[derive(Clone, PartialEq)]
struct AppState {
    count: i32,
    user: Option<String>,
}

create_store!(AppStore, AppState, AppState { 
    count: 0, 
    user: None 
});

#[component]
fn Counter() -> impl IntoView {
    let (state, set_state) = use_store::<AppStore>();
    
    let increment = move |_| {
        set_state.update(|s| s.count += 1);
    };
    
    view! {
        <div>
            <p>"Count: " {move || state.get().count}</p>
            <button on:click=increment>"Increment"</button>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    provide_store::<AppStore>(AppStore::create());
    view! { <Counter /> }
}
```

### State Machine Example (XState-style)

```rust
use leptos::*;
use leptos_state::*;

#[derive(Clone, PartialEq, Default)]
struct ToggleContext {
    count: i32,
}

#[derive(Clone, Debug)]
enum ToggleEvent {
    Toggle,
    Reset,
}

let machine = MachineBuilder::<ToggleContext, ToggleEvent>::new()
    .state("inactive")
        .on(ToggleEvent::Toggle, "active")
    .state("active") 
        .on(ToggleEvent::Toggle, "inactive")
        .on(ToggleEvent::Reset, "inactive")
    .initial("inactive")
    .build();

#[component]
fn ToggleButton() -> impl IntoView {
    let machine = use_machine::<ToggleMachine>();
    let is_active = machine.create_matcher("active".to_string());
    
    view! {
        <button on:click=move |_| machine.emit(ToggleEvent::Toggle)>
            {move || if is_active.get() { "ON" } else { "OFF" }}
        </button>
    }
}
```

## 📚 Core Concepts

### Stores

Stores are reactive containers for application state, inspired by Zustand:

- **Simple Creation**: Use the `create_store!` macro
- **Reactive Updates**: Built on Leptos signals for optimal performance  
- **Selectors**: Subscribe to specific slices of state
- **Middleware**: Extensible pipeline for cross-cutting concerns

### State Machines

State machines provide predictable state management with explicit transitions:

- **Finite States**: Define valid states and transitions
- **Guards**: Conditional transition logic
- **Actions**: Side effects during transitions
- **Hierarchical**: Nested states for complex workflows
- **Parallel**: Multiple simultaneous state machines

## 🏗️ Architecture

```
┌─────────────────────────────────────────┐
│          Application Layer              │
├─────────────────────────────────────────┤
│     Leptos Components & Hooks           │
├─────────────────────────────────────────┤
│         State Management API            │
├──────────────┬──────────────────────────┤
│  Store Layer │    Machine Layer         │
│  (Zustand)   │    (XState)              │
├──────────────┴──────────────────────────┤
│       Leptos Reactive Primitives        │
│    (Signals, Memos, Resources)          │
└─────────────────────────────────────────┘
```

## 🛠️ Advanced Features

### Middleware System

```rust
use leptos_state::*;

let store = StoreBuilder::new()
    .with_middleware(LoggerMiddleware::new("MyStore"))
    .with_middleware(PersistMiddleware::new("app_state"))
    .build();
```

### Time Travel Debugging

```rust
let history = use_store_history::<AppStore>();

// Undo/Redo functionality
if history.can_undo() {
    history.undo();
}

if history.can_redo() {
    history.redo();
}
```

### Computed State/Selectors

```rust
// Subscribe to computed values
let doubled_count = use_computed::<AppStore, _>(|state| state.count * 2);
let user_name = use_computed::<AppStore, _>(|state| {
    state.user.clone().unwrap_or("Guest".to_string())
});
```

## 🧪 Examples

Check out the `/examples` directory for complete applications:

- **[Counter](./examples/counter/)**: Basic store usage with selectors
- **[Traffic Light](./examples/traffic-light/)**: State machine with timer logic
- **[Todo App](./examples/todo/)**: Complex state with middleware
- **[Form Wizard](./examples/form-wizard/)**: Hierarchical state machines

## 📖 Documentation

- **[API Documentation](https://docs.rs/leptos-state)**: Complete API reference
- **[Guide Book](./docs/)**: Comprehensive usage guide
- **[Migration Guide](./docs/migration.md)**: From Redux/MobX patterns
- **[Performance Tips](./docs/performance.md)**: Optimization strategies

## 🧪 Testing

Run the test suite:

```bash
# Unit tests
cargo test

# Integration tests  
cargo test --test '*'

# WASM tests
wasm-pack test --headless --chrome
```

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

### Development Setup

1. Clone the repository
2. Install dependencies: `cargo build`
3. Run tests: `cargo test`
4. Check formatting: `cargo fmt`
5. Run lints: `cargo clippy`

## 📄 License

This project is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

## 🙏 Acknowledgments

- [Zustand](https://github.com/pmndrs/zustand) for store design inspiration
- [XState](https://xstate.js.org/) for state machine concepts
- [Leptos](https://leptos.dev/) for the reactive foundation
- The Rust community for excellent tooling and ecosystem

---

**Built with ❤️ and 🦀 for the Leptos community**