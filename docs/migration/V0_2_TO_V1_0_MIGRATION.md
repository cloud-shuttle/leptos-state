# 🔄 Migration Guide: v0.2.x → v1.0.0
## **Complete Migration from Current to New Architecture**

> **Status**: 🚧 In Development  
> **Target Release**: December 2025  
> **Breaking Changes**: Yes, but with migration tools

---

## 📋 **Overview**

This guide covers the complete migration from `leptos-state` v0.2.x to v1.0.0. The new version introduces a completely redesigned architecture that fixes fundamental type system issues and provides a more robust, maintainable foundation.

### **Why Migrate?**
- **Fix Type System Issues** - No more compilation errors with feature combinations
- **Feature Independence** - All features work together without conflicts
- **Better Performance** - Optimized for modern Rust and WASM
- **Future-Proof** - Architecture that can grow with your needs

---

## 🚨 **Breaking Changes Summary**

### **1. Trait Bounds**
- **Before**: Minimal bounds (`Send + Sync`)
- **After**: Proper bounds (`Clone + Debug + Default + Send + Sync`)

### **2. Builder Pattern**
- **Before**: Basic builder with limited validation
- **After**: Type-safe builder with strict validation

### **3. Feature Flags**
- **Before**: Features don't work together
- **After**: Features work independently and together

### **4. API Changes**
- **Before**: Some methods have insufficient bounds
- **After**: All methods have proper bounds and work correctly

---

## 🔧 **Automatic Migration Tools**

### **Migration Command**

```bash
# Install migration tool
cargo install leptos-state-migrate

# Run migration on your project
leptos-state-migrate --project-path ./my-project

# Preview changes without applying
leptos-state-migrate --project-path ./my-project --dry-run
```

### **Migration Tool Features**
- **Automatic Code Updates** - Converts v0.2.x code to v1.0.0
- **Trait Bound Fixes** - Adds missing trait implementations
- **API Updates** - Updates method calls to new signatures
- **Validation** - Checks that migration was successful

---

## 📝 **Manual Migration Steps**

### **Step 1: Update Dependencies**

```toml
# Cargo.toml
[dependencies]
# Before
leptos-state = "0.2.2"

# After
leptos-state = "1.0.0"
```

### **Step 2: Fix Trait Bounds**

#### **Before (v0.2.x)**
```rust
#[derive(Clone, PartialEq)]
struct MyState {
    count: i32,
    name: String,
}
```

#### **After (v1.0.0)**
```rust
#[derive(Clone, Debug, Default, PartialEq)]
struct MyState {
    count: i32,
    name: String,
}

impl Default for MyState {
    fn default() -> Self {
        Self {
            count: 0,
            name: String::new(),
        }
    }
}
```

### **Step 3: Update Store Definitions**

#### **Before (v0.2.x)**
```rust
create_store!(
    MyStore,
    MyState,
    MyState { count: 0, name: "Default".to_string() }
);
```

#### **After (v1.0.0)**
```rust
create_store!(
    MyStore,
    MyState,
    MyState::default()
);
```

### **Step 4: Update State Machine Definitions**

#### **Before (v0.2.x)**
```rust
let machine = MachineBuilder::new()
    .state("idle")
        .on(MyEvent::Start, "active")
    .state("active")
        .on(MyEvent::Stop, "idle")
    .initial("idle")
    .build();
```

#### **After (v1.0.0)**
```rust
let machine = MachineBuilder::new()
    .state("idle")
        .on(MyEvent::Start, "active")
    .state("active")
        .on(MyEvent::Stop, "idle")
    .initial("idle")
    .build()
    .expect("Failed to build state machine");
```

### **Step 5: Update Hooks Usage**

#### **Before (v0.2.x)**
```rust
let (state, set_state) = use_store::<MyStore>();
```

#### **After (v1.0.0)**
```rust
let (state, set_state) = use_store::<MyStore>();
// Same API, but now works with all features
```

---

## 🏗️ **Architecture Changes**

### **New Trait Hierarchy**

#### **Before (v0.2.x)**
```rust
pub struct Machine<C: Send + Sync, E> {
    // Limited bounds
}
```

#### **After (v1.0.0)**
```rust
pub trait StateMachineContext: 
    Clone + Debug + Default + Send + Sync + 'static {}

pub trait StateMachineEvent: 
    Clone + Debug + PartialEq + Send + Sync + 'static {}

pub struct Machine<C, E, S>
where
    C: StateMachineContext,
    E: StateMachineEvent,
    S: StateMachineState<Context = C, Event = E>,
{
    // Proper bounds and type safety
}
```

### **New Builder Pattern**

#### **Before (v0.2.x)**
```rust
impl<C: Send + Sync, E> MachineBuilder<C, E> {
    fn build(self) -> Machine<C, E> { /* ... */ }
}
```

#### **After (v1.0.0)**
```rust
impl<C, E, S> MachineBuilder<C, E, S>
where
    C: StateMachineContext,
    E: StateMachineEvent,
    S: StateMachineState<Context = C, Event = E>,
{
    fn build(self) -> Result<Machine<C, E, S>, BuildError> { /* ... */ }
}
```

---

## 🔄 **Feature Migration**

### **Persistence Features**

#### **Before (v0.2.x)**
```rust
// This didn't work due to type system issues
#[cfg(feature = "persist")]
let machine = MachineBuilder::new()
    .state("idle")
    .build_with_persistence(config); // Compilation error
```

#### **After (v1.0.0)**
```rust
#[cfg(feature = "persist")]
let machine = MachineBuilder::new()
    .state("idle")
    .build()
    .expect("Failed to build")
    .with_persistence(PersistenceConfig {
        enabled: true,
        storage_key: "my_machine".to_string(),
        auto_save: true,
        ..Default::default()
    });
```

### **Visualization Features**

#### **Before (v0.2.x)**
```rust
// This didn't work due to type system issues
#[cfg(feature = "visualization")]
let diagram = machine.generate_mermaid(); // Compilation error
```

#### **After (v1.0.0)**
```rust
#[cfg(feature = "visualization")]
let diagram = machine.generate_mermaid();
println!("{}", diagram);
```

### **Testing Features**

#### **Before (v0.2.x)**
```rust
// Limited testing capabilities
let machine = MachineBuilder::new()
    .state("idle")
    .build();
// Basic testing only
```

#### **After (v1.0.0)**
```rust
let machine = MachineBuilder::new()
    .state("idle")
    .build()
    .expect("Failed to build");

#[cfg(feature = "testing")]
let tester = machine.create_tester();
let result = tester.property_test(|machine, events| {
    // Property-based testing
    true
});
```

---

## 🧪 **Testing Your Migration**

### **Migration Validation**

```bash
# Run tests to ensure migration was successful
cargo test

# Check compilation with all features
cargo check --features persist,visualization,testing,codegen

# Run examples to verify functionality
cargo run --example counter
cargo run --example traffic-light
```

### **Common Migration Issues**

#### **1. Missing Trait Implementations**
```rust
// Error: the trait bound `MyState: Default` is not satisfied
// Solution: Implement Default trait
impl Default for MyState {
    fn default() -> Self {
        Self { /* ... */ }
    }
}
```

#### **2. Builder Pattern Changes**
```rust
// Error: method `build` returns Result, not Machine
// Solution: Handle the Result
let machine = MachineBuilder::new()
    .state("idle")
    .build()
    .expect("Failed to build state machine");
```

#### **3. Feature Flag Issues**
```rust
// Error: feature combinations don't work
// Solution: All features now work together in v1.0.0
cargo check --features persist,visualization,testing
```

---

## 📊 **Migration Checklist**

### **Pre-Migration**
- [ ] **Backup your code** - Create a git branch for migration
- [ ] **Update Rust toolchain** - Ensure you have Rust 1.70+
- [ ] **Check dependencies** - Update Leptos to v0.8+
- [ ] **Run current tests** - Ensure everything works before migration

### **Migration Process**
- [ ] **Update Cargo.toml** - Change leptos-state version to 1.0.0
- [ ] **Run automatic migration** - Use migration tools
- [ ] **Fix trait bounds** - Add missing trait implementations
- [ ] **Update API calls** - Handle Result types from builders
- [ ] **Test compilation** - Ensure code compiles without errors

### **Post-Migration**
- [ ] **Run all tests** - Verify functionality is preserved
- [ ] **Test with features** - Ensure all features work together
- [ ] **Performance testing** - Verify no performance regressions
- [ ] **Update documentation** - Reflect new API usage

---

## 🚀 **Performance Improvements**

### **What's Faster in v1.0.0**

1. **Compilation Time** - Better type system reduces compilation overhead
2. **Runtime Performance** - Optimized data structures and algorithms
3. **Memory Usage** - More efficient memory management
4. **WASM Size** - Smaller binary size for web deployment

### **Benchmarking Your Migration**

```rust
#[cfg(test)]
mod benchmarks {
    use super::*;
    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    fn benchmark_state_transitions(c: &mut Criterion) {
        let machine = create_test_machine();
        let mut state = machine.initial_state();
        
        c.bench_function("state_transitions", |b| {
            b.iter(|| {
                let event = TestEvent::Next;
                black_box(machine.transition(&state, event));
            });
        });
    }

    criterion_group!(benches, benchmark_state_transitions);
    criterion_main!(benches);
}
```

---

## 🔍 **Debugging Migration Issues**

### **Common Error Messages**

#### **1. Trait Bound Errors**
```
error[E0277]: the trait bound `MyState: Default` is not satisfied
```
**Solution**: Implement the `Default` trait for your state types.

#### **2. Builder Pattern Errors**
```
error[E0599]: the method `build` exists for struct `MachineBuilder<C, E>`, but its trait bounds were not satisfied
```
**Solution**: Ensure your types implement all required traits.

#### **3. Feature Flag Errors**
```
error[E0599]: the method `with_persistence` exists for struct `Machine<C, E>`, but its trait bounds were not satisfied
```
**Solution**: This will be fixed in v1.0.0 - all features work together.

### **Getting Help**

- **Migration Issues**: [GitHub Issues](https://github.com/cloud-shuttle/leptos-state/issues)
- **Community Support**: [GitHub Discussions](https://github.com/cloud-shuttle/leptos-state/discussions)
- **Documentation**: [User Guide](../user-guide/README.md)

---

## 📚 **Migration Examples**

### **Complete Migration Example**

#### **Before (v0.2.x)**
```rust
use leptos::*;
use leptos_state::*;

#[derive(Clone, PartialEq)]
struct AppState {
    count: i32,
    user: Option<String>,
}

create_store!(
    AppStore,
    AppState,
    AppState { count: 0, user: None }
);

#[derive(Clone, Debug)]
enum AppEvent {
    Increment,
    SetUser(String),
}

fn App() -> impl IntoView {
    let (state, set_state) = use_store::<AppStore>();
    
    let increment = move |_| {
        set_state.update(|s| s.count += 1);
    };
    
    let set_user = move |ev| {
        let value = event_target_value(&ev);
        set_state.update(|s| s.user = Some(value));
    };

    view! {
        <div>
            <h1>"Count: " {move || state.get().count}</h1>
            <button on:click=increment>"Increment"</button>
            <input
                type="text"
                placeholder="Enter user name"
                on:input=set_user
            />
            <p>"User: " {move || state.get().user.as_ref().unwrap_or(&"None".to_string())}</p>
        </div>
    }
}
```

#### **After (v1.0.0)**
```rust
use leptos::*;
use leptos_state::*;

#[derive(Clone, Debug, Default, PartialEq)]
struct AppState {
    count: i32,
    user: Option<String>,
}

create_store!(
    AppStore,
    AppState,
    AppState::default()
);

#[derive(Clone, Debug, PartialEq)]
enum AppEvent {
    Increment,
    SetUser(String),
}

fn App() -> impl IntoView {
    let (state, set_state) = use_store::<AppStore>();
    
    let increment = move |_| {
        set_state.update(|s| s.count += 1);
    };
    
    let set_user = move |ev| {
        let value = event_target_value(&ev);
        set_state.update(|s| s.user = Some(value));
    };

    view! {
        <div>
            <h1>"Count: " {move || state.get().count}</h1>
            <button on:click=increment>"Increment"</button>
            <input
                type="text"
                placeholder="Enter user name"
                on:input=set_user
            />
            <p>"User: " {move || state.get().user.as_ref().unwrap_or(&"None".to_string())}</p>
        </div>
    }
}
```

---

## 🎯 **Migration Timeline**

### **Phase 1: Preparation (September 2025)**
- **Migration Tools Development** - Automatic migration tools
- **Documentation Updates** - Complete migration guides
- **Community Outreach** - Announce migration plans

### **Phase 2: Early Access (October 2025)**
- **Alpha Release** - v1.0.0-alpha for early adopters
- **Migration Testing** - Community testing of migration tools
- **Feedback Collection** - Gather migration experience

### **Phase 3: General Release (December 2025)**
- **v1.0.0 Release** - Stable release with migration tools
- **Community Migration** - Support community migration
- **Documentation Updates** - Final migration guides

---

## 🤝 **Getting Help with Migration**

### **Migration Support Channels**

1. **GitHub Issues** - Report migration problems
2. **GitHub Discussions** - Ask migration questions
3. **Migration Tools** - Use automatic migration tools
4. **Documentation** - Comprehensive migration guides

### **Community Resources**

- **Migration Examples** - Real-world migration examples
- **Video Tutorials** - Step-by-step migration videos
- **Community Support** - Help from other developers
- **Migration Workshops** - Live migration assistance

---

## 📚 **Additional Resources**

- **[🏗️ Architectural Redesign Plan](../development/ARCHITECTURAL_REDESIGN.md)** - Complete redesign overview
- **[🔧 Technical Specification](../development/TECHNICAL_SPECIFICATION.md)** - Implementation details
- **[📅 Implementation Timeline](../development/IMPLEMENTATION_TIMELINE.md)** - Development timeline
- **[📖 User Guide](../user-guide/README.md)** - Current usage documentation

---

*This migration guide will be updated as v1.0.0 development progresses. Last updated: September 4, 2025*
