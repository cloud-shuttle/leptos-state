# 📖 User Guide - leptos-state v0.2.2
## **Getting Started with State Management for Leptos**

> **🚧 Important Notice: Architectural Redesign in Progress**
> 
> We are currently undergoing a major architectural redesign to fix fundamental type system issues and create a more robust, maintainable library. The current v0.2.x version has some limitations with advanced features.
> 
> **For the latest information about the redesign, see:**
> - [🏗️ Architectural Redesign Plan](../development/ARCHITECTURAL_REDESIGN.md)
> - [🔧 Technical Specification](../development/TECHNICAL_SPECIFICATION.md)
> - [📅 Implementation Timeline](../development/IMPLEMENTATION_TIMELINE.md)

---

## 📋 **Current Status: v0.2.2**

### **✅ What Works Right Now**
- **Core State Machines** - Basic functionality compiles and tests pass
- **Simple Stores** - Basic store management works
- **Code Generation** - Actually generates working code in multiple languages
- **Testing Framework** - 90+ tests pass in isolation

### **⚠️ Current Limitations**
- **Advanced Features** - Some features don't work together due to type system issues
- **WASM-Only** - Examples can't run on native targets
- **Feature Flags** - Some advanced features don't compile properly

---

## 🚀 **Quick Start**

### **Installation**

Add `leptos-state` to your `Cargo.toml`:

```toml
[dependencies]
leptos-state = "0.2.2"
leptos = "0.8"
```

### **Basic Store Example**

```rust
use leptos::*;
use leptos_state::{create_store, use_store, Store};

#[derive(Clone, Debug, PartialEq)]
struct CounterState {
    count: i32,
    name: String,
}

impl CounterState {
    fn increment(&mut self) {
        self.count += 1;
    }
    
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

create_store!(
    CounterStore,
    CounterState,
    CounterState { count: 0, name: "Counter".to_string() }
);

#[component]
fn Counter() -> impl IntoView {
    let (state, set_state) = use_store::<CounterStore>();

    let increment = move |_| {
        set_state.update(|s| s.increment());
    };

    let set_name = move |ev| {
        let value = event_target_value(&ev);
        set_state.update(|s| s.set_name(value));
    };

    view! {
        <div class="counter">
            <h2>"Counter: " {move || state.get().count}</h2>
            <p>"Name: " {move || state.get().name}</p>
            <button on:click=increment>"Increment"</button>
            <input
                type="text"
                value=move || state.get().name
                on:input=set_name
                placeholder="Enter name"
            />
        </div>
    }
}
```

### **State Machine Example**

```rust
use leptos::*;
use leptos_state::{MachineBuilder, use_machine};

#[derive(Clone, Debug, PartialEq)]
enum TrafficLightEvent {
    Next,
    Emergency,
}

#[derive(Clone, Debug, PartialEq)]
struct TrafficContext {
    timer: i32,
    pedestrian_waiting: bool,
}

fn TrafficLight() -> impl IntoView {
    let machine = MachineBuilder::new()
        .state("red")
            .on(TrafficLightEvent::Next, "green")
        .state("green")
            .on(TrafficLightEvent::Next, "yellow")
        .state("yellow")
            .on(TrafficLightEvent::Next, "red")
        .initial("red")
        .build();
    
    let (state, send) = use_machine(machine);
    
    let next_light = move |_| send(TrafficLightEvent::Next);
    let emergency = move |_| send(TrafficLightEvent::Emergency);

    view! {
        <div class="traffic-light">
            <h2>"Traffic Light: " {move || state.get().value()}</h2>
            <div class="controls">
                <button on:click=next_light>"Next Light"</button>
                <button on:click=emergency>"Emergency"</button>
            </div>
        </div>
    }
}
```

---

## 🏪 **Store Management**

### **Store Creation**

Stores are the foundation of state management in `leptos-state`. They provide reactive state that automatically updates your UI when data changes.

```rust
// Define your state
#[derive(Clone, Debug, PartialEq)]
struct AppState {
    user: Option<User>,
    theme: Theme,
    settings: Settings,
}

// Create a store
create_store!(
    AppStore,
    AppState,
    AppState {
        user: None,
        theme: Theme::Light,
        settings: Settings::default(),
    }
);
```

### **Using Stores in Components**

```rust
#[component]
fn AppHeader() -> impl IntoView {
    let (state, set_state) = use_store::<AppStore>();
    
    let toggle_theme = move |_| {
        set_state.update(|s| {
            s.theme = match s.theme {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
            };
        });
    };

    view! {
        <header class=move || format!("header-{}", state.get().theme.as_str())>
            <h1>"My App"</h1>
            <button on:click=toggle_theme>
                "Toggle Theme"
            </button>
        </header>
    }
}
```

### **Store Slices and Computed Values**

```rust
#[component]
fn UserProfile() -> impl IntoView {
    // Get a slice of the store
    let user = use_store_slice::<AppStore, _>(|state| state.user.clone());
    
    // Create computed values
    let is_authenticated = use_computed::<AppStore, _>(|state| state.user.is_some());
    let user_name = use_computed::<AppStore, _>(|state| {
        state.user.as_ref().map(|u| u.name.clone()).unwrap_or_default()
    });

    view! {
        <div class="user-profile">
            {move || if is_authenticated.get() {
                view! {
                    <div>
                        <h3>"Welcome, " {user_name}</h3>
                        <p>"User ID: " {move || user.get().as_ref().map(|u| u.id).unwrap_or(0)}</p>
                    </div>
                }
            } else {
                view! {
                    <div>
                        <p>"Please log in"</p>
                        <button>"Login"</button>
                    </div>
                }
            }}
        </div>
    }
}
```

---

## 🤖 **State Machines**

### **Basic State Machine**

State machines provide a structured way to manage complex application state with well-defined transitions and actions.

```rust
#[derive(Clone, Debug, PartialEq)]
enum AppEvent {
    Login { username: String, password: String },
    Logout,
    Navigate { route: String },
    Error { message: String },
}

#[derive(Clone, Debug, PartialEq)]
struct AppContext {
    user: Option<User>,
    current_route: String,
    error_count: u32,
}

fn AppStateMachine() -> impl IntoView {
    let machine = MachineBuilder::new()
        .state("loading")
            .on(AppEvent::Login { username: _, password: _ }, "authenticated")
            .on(AppEvent::Error { message: _ }, "error")
        .state("authenticated")
            .on(AppEvent::Logout, "loading")
            .on(AppEvent::Navigate { route: _ }, "authenticated")
            .on(AppEvent::Error { message: _ }, "error")
        .state("error")
            .on(AppEvent::Login { username: _, password: _ }, "authenticated")
            .on(AppEvent::Logout, "loading")
        .initial("loading")
        .build();
    
    let (state, send) = use_machine(machine);
    
    let login = move |username: String, password: String| {
        send(AppEvent::Login { username, password });
    };
    
    let logout = move |_| send(AppEvent::Logout);
    let navigate = move |route: String| send(AppEvent::Navigate { route });

    view! {
        <div class="app">
            {move || match state.get().value().as_str() {
                "loading" => view! { <div>"Loading..."</div> },
                "authenticated" => view! {
                    <div>
                        <h2>"Welcome!"</h2>
                        <button on:click=move |_| logout()>"Logout"</button>
                        <button on:click=move |_| navigate("profile".to_string())>"Profile"</button>
                    </div>
                },
                "error" => view! {
                    <div>
                        <h2>"Error occurred"</h2>
                        <button on:click=move |_| logout()>"Try Again"</button>
                    </div>
                },
                _ => view! { <div>"Unknown state"</div> }
            }}
        </div>
    }
}
```

### **State Machine with Guards**

Guards allow you to conditionally allow or block transitions based on the current state and context.

```rust
#[derive(Clone, Debug, PartialEq)]
enum PaymentEvent {
    SubmitPayment { amount: f64 },
    ValidatePayment,
    ProcessPayment,
    CompletePayment,
}

fn PaymentStateMachine() -> impl IntoView {
    let machine = MachineBuilder::new()
        .state("idle")
            .on(PaymentEvent::SubmitPayment { amount: _ }, "validating")
        .state("validating")
            .on(PaymentEvent::ValidatePayment, "processing")
            .on(PaymentEvent::Error { message: _ }, "error")
        .state("processing")
            .on(PaymentEvent::ProcessPayment, "completed")
            .on(PaymentEvent::Error { message: _ }, "error")
        .state("completed")
            .on(PaymentEvent::SubmitPayment { amount: _ }, "validating")
        .state("error")
            .on(PaymentEvent::SubmitPayment { amount: _ }, "validating")
        .initial("idle")
        .build();
    
    let (state, send) = use_machine(machine);
    
    // Implementation...
}
```

---

## 🔧 **Advanced Features (When Available)**

### **Persistence**

> **Note**: Persistence features are currently limited due to architectural issues. They will be fully functional in v1.0.0.

```rust
// This will work properly in v1.0.0
#[cfg(feature = "persist")]
let machine = MachineBuilder::new()
    .state("idle")
    .build_with_persistence(PersistenceConfig {
        enabled: true,
        storage_key: "my_machine".to_string(),
        auto_save: true,
        ..Default::default()
    });
```

### **Visualization**

> **Note**: Visualization features are currently limited due to architectural issues. They will be fully functional in v1.0.0.

```rust
// This will work properly in v1.0.0
#[cfg(feature = "visualization")]
let diagram = machine.generate_mermaid();
println!("{}", diagram);
```

---

## 🧪 **Testing Your State Management**

### **Unit Testing Stores**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_store_creation() {
        let store = CounterStore::create();
        assert_eq!(store.count, 0);
        assert_eq!(store.name, "Counter");
    }

    #[test]
    fn test_store_actions() {
        let mut store = CounterStore::create();
        store.increment();
        assert_eq!(store.count, 1);
        
        store.set_name("New Name".to_string());
        assert_eq!(store.name, "New Name");
    }
}
```

### **Testing State Machines**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_machine_transitions() {
        let machine = MachineBuilder::new()
            .state("red")
                .on(TrafficLightEvent::Next, "green")
            .state("green")
                .on(TrafficLightEvent::Next, "yellow")
            .state("yellow")
                .on(TrafficLightEvent::Next, "red")
            .initial("red")
            .build();
        
        let mut state = machine.initial_state();
        assert_eq!(state.value(), "red");
        
        state = machine.transition(&state, TrafficLightEvent::Next);
        assert_eq!(state.value(), "green");
        
        state = machine.transition(&state, TrafficLightEvent::Next);
        assert_eq!(state.value(), "yellow");
        
        state = machine.transition(&state, TrafficLightEvent::Next);
        assert_eq!(state.value(), "red");
    }
}
```

---

## 🚨 **Known Issues and Workarounds**

### **Feature Flag Problems**

Some advanced features don't compile due to type system issues. Here are workarounds:

1. **Use Basic Features Only**: Stick to core state machines and stores
2. **Avoid Feature Combinations**: Don't try to use multiple advanced features together
3. **Check Compilation**: Test your code with `cargo check` before running

### **WASM-Only Limitations**

Examples can't run on native targets. To test:

1. **Use WASM Target**: `cargo build --target wasm32-unknown-unknown`
2. **Test in Browser**: Use `wasm-pack` or similar tools
3. **Wait for v1.0.0**: Native compatibility will be available

---

## 🔄 **Migration to v1.0.0**

When v1.0.0 is released, you'll need to update your code. The migration will include:

1. **Trait Bounds**: All types must implement required traits
2. **Builder Pattern**: New builder with stricter type checking
3. **Feature Flags**: Features will work independently and together
4. **API Changes**: Some method signatures will change

**Migration tools will be provided to automate most of this process.**

---

## 📚 **Additional Resources**

### **Current Documentation**
- **[🔧 API Reference](../api-reference/)** - Current API documentation
- **[📝 Examples](../examples/)** - Working code samples
- **[🔄 Migration Guide](../migration/)** - Upgrade instructions

### **Future Documentation (v1.0.0)**
- **[🏗️ Architectural Redesign Plan](../development/ARCHITECTURAL_REDESIGN.md)** - Complete redesign overview
- **[🔧 Technical Specification](../development/TECHNICAL_SPECIFICATION.md)** - Implementation details
- **[📅 Implementation Timeline](../development/IMPLEMENTATION_TIMELINE.md)** - Development timeline

---

## 🤝 **Getting Help**

- **GitHub Issues**: [Report bugs and request features](https://github.com/cloud-shuttle/leptos-state/issues)
- **Discussions**: [Join community discussions](https://github.com/cloud-shuttle/leptos-state/discussions)
- **Documentation**: This comprehensive guide

---

## 🚀 **What's Next?**

1. **Start with Basic Stores**: Get familiar with simple state management
2. **Build Simple State Machines**: Learn the basic patterns
3. **Follow the Redesign**: Keep up with v1.0.0 development
4. **Prepare for Migration**: Plan your upgrade path

---

*This user guide covers the current v0.2.2 functionality. For information about the upcoming v1.0.0 redesign, see the [Architectural Redesign Plan](../development/ARCHITECTURAL_REDESIGN.md). Last updated: September 4, 2025*
