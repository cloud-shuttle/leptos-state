# 📖 Leptos State User Guide

Welcome to the comprehensive user guide for **leptos-state**! This guide will walk you through everything you need to know to build powerful, reactive applications with state management.

## 🗂️ Guide Structure

### 🚀 **Getting Started**
- **[Installation](./installation.md)**: Setup and configuration
- **[Quick Start](./quick-start.md)**: Your first leptos-state application
- **[Basic Concepts](./concepts.md)**: Core concepts and terminology

### 📦 **Store Management**
- **[Creating Stores](./stores/creating-stores.md)**: Store creation and configuration
- **[State Updates](./stores/state-updates.md)**: Modifying state reactively
- **[Computed State](./stores/computed-state.md)**: Derived state and selectors
- **[Middleware](./stores/middleware.md)**: Extending store functionality

### 🤖 **State Machines**
- **[State Machine Basics](./machines/basics.md)**: Introduction to state machines
- **[Transitions](./machines/transitions.md)**: State transitions and events
- **[Guards](./machines/guards.md)**: Conditional transitions
- **[Actions](./machines/actions.md)**: Side effects and behaviors
- **[Hierarchical States](./machines/hierarchical.md)**: Complex state structures

### 🪝 **Hooks and Reactivity**
- **[use_store](./hooks/use-store.md)**: Store subscription hook
- **[use_machine](./hooks/use-machine.md)**: State machine hook
- **[use_computed](./hooks/use-computed.md)**: Computed state hook
- **[use_effect](./hooks/use-effect.md)**: Side effects and subscriptions

### 🔧 **Advanced Features**
- **[Persistence](./advanced/persistence.md)**: State persistence and serialization
- **[DevTools](./advanced/devtools.md)**: Debugging and visualization
- **[Performance](./advanced/performance.md)**: Optimization strategies
- **[Testing](./advanced/testing.md)**: Testing state management

### 🌐 **Web and WASM**
- **[WASM Setup](./web/wasm-setup.md)**: WebAssembly configuration
- **[Browser Integration](./web/browser.md)**: Browser-specific features
- **[SSR Support](./web/ssr.md)**: Server-side rendering

## 🎯 Learning Path

### **Beginner** 🟢
1. **Installation**: Set up your development environment
2. **Quick Start**: Build your first application
3. **Basic Concepts**: Understand core principles
4. **Creating Stores**: Learn state management basics

### **Intermediate** 🟡
1. **State Updates**: Master reactive state changes
2. **Computed State**: Work with derived state
3. **State Machines**: Build complex state logic
4. **Middleware**: Extend functionality

### **Advanced** 🔴
1. **Hierarchical States**: Complex state structures
2. **Performance**: Optimization techniques
3. **DevTools**: Advanced debugging
4. **Testing**: Comprehensive testing strategies

## 🚀 Quick Start

### **Installation**
```toml
[dependencies]
leptos = "0.8"
leptos-state = "0.2"
```

### **Basic Store**
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

### **State Machine**
```rust
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
```

## 🔗 Related Resources

- **[Examples](../examples/)**: Working code samples
- **[API Reference](../api-reference/)**: Complete API documentation
- **[Migration Guide](../migration/)**: Upgrade from other solutions
- **[Contributing](../contributing/)**: Help improve the project

## 🆘 Need Help?

### **Common Issues**
- **Build Problems**: Check Rust and Leptos versions
- **Runtime Errors**: Verify state management patterns
- **Performance Issues**: Review optimization strategies
- **Migration Problems**: Consult migration guides

### **Getting Support**
- **GitHub Issues**: [Report bugs](https://github.com/cloud-shuttle/leptos-state/issues)
- **Discussions**: [Ask questions](https://github.com/cloud-shuttle/leptos-state/discussions)
- **Documentation**: This comprehensive guide

---

*Ready to build amazing applications with leptos-state? Let's get started! 🚀✨*
