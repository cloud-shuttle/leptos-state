# 🎉 Leptos State v0.1.0 - Initial Release

We're excited to announce the initial release of **Leptos State**, a powerful state management library for Leptos applications with state machines, persistence, and DevTools!

## 🚀 What's New

### ✨ Core Features
- **State Machines**: XState-inspired finite state machines with hierarchical states
- **Stores**: Zustand-inspired reactive stores with middleware support
- **DevTools**: Built-in visualization and time-travel debugging
- **Performance**: Optimized for WASM and production use
- **Type Safety**: Strong compile-time guarantees powered by Rust

### 🏗️ Architecture
- **Modular Design**: Clear separation between stores and state machines
- **Extensible**: Middleware system for cross-cutting concerns
- **Composable**: Work seamlessly with Leptos reactive primitives
- **Predictable**: Explicit state transitions and side effects

### 📦 Package Stats
- **37 files** published
- **460.0KiB** total size (86.8KiB compressed)
- **90 tests** passing with 100% success rate
- **MIT license** with proper attribution

## 🎯 Key Features

### State Machines
```rust
use leptos_state::*;

let machine = MachineBuilder::<Context, Event>::new()
    .state("idle")
        .on(Event::Start, "active")
    .state("active")
        .on(Event::Stop, "idle")
    .initial("idle")
    .build();
```

### Reactive Stores
```rust
use leptos_state::*;

create_store!(AppStore, AppState, AppState { 
    count: 0, 
    user: None 
});

let (state, set_state) = use_store::<AppStore>();
```

### DevTools Integration
- Time-travel debugging
- State visualization
- Performance profiling
- Middleware inspection

## 📚 Examples Included

### 🎯 Todo App
Complete CRUD application demonstrating:
- Store management with reactive updates
- State machine for edit modes
- Persistence with localStorage
- Bulk operations

### 📊 Analytics Dashboard
Real-time data visualization showing:
- Complex state management
- Real-time updates
- Interactive charts
- Responsive design

### 🚦 Traffic Light
Simple state machine demonstrating:
- Timer-based transitions
- Visual state representation
- Event-driven architecture

## 🛠️ Getting Started

### Installation
```toml
[dependencies]
leptos = "0.6"
leptos-state = "0.1"
```

### Quick Start
```rust
use leptos::*;
use leptos_state::*;

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
```

## 🔧 Features

### State Machine Features
- ✅ Hierarchical states
- ✅ Parallel states
- ✅ Guards and conditions
- ✅ Actions and effects
- ✅ History states
- ✅ State persistence
- ✅ Visualization tools

### Store Features
- ✅ Reactive updates
- ✅ Middleware system
- ✅ Computed state
- ✅ Time-travel debugging
- ✅ Async store support
- ✅ DevTools integration

### Development Features
- ✅ Comprehensive testing
- ✅ Performance optimization
- ✅ Documentation generation
- ✅ Code generation
- ✅ Integration patterns

## 🧪 Testing

All features are thoroughly tested:
- **90 unit tests** passing
- **Integration tests** for complex scenarios
- **Property-based testing** for invariants
- **Performance benchmarks** included
- **WASM compatibility** verified

## 📖 Documentation

- **API Documentation**: https://docs.rs/leptos-state
- **GitHub Repository**: https://github.com/cloud-shuttle/leptos-state
- **Examples**: Complete working applications
- **Guide Book**: Comprehensive usage guide

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

### Development Setup
```bash
git clone https://github.com/cloud-shuttle/leptos-state.git
cd leptos-state
cargo build
cargo test
```

## 🎯 Roadmap

### v0.2.0 (Next Release)
- [ ] SSR compatibility improvements
- [ ] Advanced DevTools features
- [ ] More middleware options
- [ ] Performance optimizations
- [ ] Additional examples

### Future Releases
- [ ] Visual state machine editor
- [ ] Migration tools from Redux/MobX
- [ ] Advanced debugging features
- [ ] Plugin system
- [ ] TypeScript definitions export

## 🙏 Acknowledgments

- [Zustand](https://github.com/pmndrs/zustand) for store design inspiration
- [XState](https://xstate.js.org/) for state machine concepts
- [Leptos](https://leptos.dev/) for the reactive foundation
- The Rust community for excellent tooling and ecosystem

## 📄 License

This project is dual-licensed under either:
- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

---

**Built with ❤️ and 🦀 for the Leptos community**

---

## 🔗 Links

- **Crates.io**: https://crates.io/crates/leptos-state
- **Documentation**: https://docs.rs/leptos-state
- **GitHub**: https://github.com/cloud-shuttle/leptos-state
- **Examples**: https://github.com/cloud-shuttle/leptos-state/tree/main/examples

## 🎉 Try It Now!

```bash
cargo add leptos-state
```

Start building powerful, type-safe state management for your Leptos applications today!
