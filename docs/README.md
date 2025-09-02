# 📚 Leptos State Documentation

Welcome to the comprehensive documentation for the **leptos-state** library - a powerful state management solution for Leptos applications.

## 🗂️ Documentation Structure

### 📖 [User Guide](./user-guide/)
- **Getting Started**: Quick start guides and tutorials
- **Examples**: Working examples and code samples
- **HTML Documentation**: Interactive documentation pages

### 🔧 [API Reference](./api-reference/)
- **Core APIs**: Complete API documentation
- **Types**: Type definitions and interfaces
- **Macros**: Macro usage and examples

### 🚀 [Examples](./examples/)
- **Basic Examples**: Simple state management patterns
- **Advanced Examples**: Complex use cases and patterns
- **Real-world Applications**: Complete application examples

### 🔄 [Migration](./migration/)
- **Leptos 0.8+ Migration**: Complete migration guide
- **Compatibility**: Version compatibility information
- **Migration Tools**: Tools and utilities for migration

### 🛠️ [Development](./development/)
- **Architecture**: System design and architecture
- **Testing Strategy**: Testing approaches and tools
- **Implementation Guide**: Development guidelines

### 📋 [Contributing](./contributing/)
- **Contributing Guide**: How to contribute to the project
- **Code of Conduct**: Community guidelines
- **Development Setup**: Local development environment

### 🏷️ [Releases](./releases/)
- **Changelog**: Complete version history
- **Release Notes**: Detailed release announcements
- **Migration Guides**: Version-specific migration information

## 🚀 Quick Start

### Installation

```toml
[dependencies]
leptos = "0.8"
leptos-state = "0.2"
```

### Basic Usage

```rust
use leptos::*;
use leptos_state::*;

#[derive(Clone, PartialEq)]
struct AppState {
    count: i32,
}

create_store!(AppStore, AppState, AppState { count: 0 });

#[component]
fn Counter() -> impl IntoView {
    let (state, set_state) = use_store::<AppStore>();
    
    view! {
        <div>
            <p>"Count: " {move || state.get().count}</p>
            <button on:click=move |_| set_state.update(|s| s.count += 1)>
                "Increment"
            </button>
        </div>
    }
}
```

## 🎯 Key Features

- **📦 Store Management**: Zustand-inspired stores with reactive updates
- **🤖 State Machines**: XState-inspired finite state machines
- **⚡ Leptos Integration**: First-class support for Leptos 0.8+
- **🔒 Type Safety**: Strong compile-time guarantees
- **🛠️ DevTools**: Built-in debugging and visualization
- **🌐 WASM Support**: Full WebAssembly compatibility

## 📖 Getting Started

1. **Installation**: Add the dependency to your `Cargo.toml`
2. **Basic Store**: Start with simple state management
3. **State Machines**: Build complex state logic
4. **Examples**: Explore working examples
5. **Advanced Features**: Dive into advanced patterns

## 🔗 Quick Links

- **[User Guide](./user-guide/)**: Start here for tutorials
- **[Examples](./examples/)**: Working code samples
- **[Migration Guide](./migration/)**: Upgrade to Leptos 0.8+
- **[API Reference](./api-reference/)**: Complete API docs
- **[Contributing](./contributing/)**: Help improve the project
- **[Roadmap](./ROADMAP.md)**: Future development plans

## 🆘 Need Help?

- **GitHub Issues**: [Report bugs and request features](https://github.com/cloud-shuttle/leptos-state/issues)
- **Discussions**: [Join community discussions](https://github.com/cloud-shuttle/leptos-state/discussions)
- **Documentation**: This comprehensive guide

---

*Welcome to leptos-state - the modern state management solution for Leptos applications! 🚀*
