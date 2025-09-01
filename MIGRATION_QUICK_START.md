# 🚀 Leptos 0.8+ Migration Quick Start

## Getting Started

This guide helps you get started with the Leptos 0.8+ migration for `leptos-state`.

## Prerequisites

- Rust 1.70+ installed
- Familiarity with Leptos 0.6
- Understanding of Rust traits and bounds

## Step 1: Set Up Migration Environment

```bash
# Create migration branch from stable version
git checkout main
git checkout -b leptos-0.8-migration

# Update workspace dependencies
# Edit Cargo.toml to use leptos = "0.7" (stable 0.8+ alternative)
```

## Step 2: Start with Core Traits

Begin with the most critical changes in `leptos-state/src/machine/machine.rs`:

```rust
// Update StateMachine trait
pub trait StateMachine: Sized + 'static {
    type Context: Clone + PartialEq + Send + Sync + 'static;
    type Event: Clone + Send + Sync + 'static;
    type State: MachineState<Context = Self::Context> + Clone + Send + Sync + 'static;
    
    fn initial() -> Self::State;
    fn transition(state: &Self::State, event: Self::Event) -> Self::State;
}
```

## Step 3: Update Signal APIs

In `leptos-state/src/hooks/use_machine.rs`:

```rust
// Replace create_signal with create_rw_signal
let (state, set_state) = create_rw_signal(M::initial());

// Update callback usage
let send = Callback::new(move |event: M::Event| {
    set_state.update(|s| *s = M::transition(s, event));
});
```

## Step 4: Test Your Changes

```bash
# Check compilation
cargo check

# Run tests
cargo test

# Check specific package
cargo check -p leptos-state
```

## Common Issues & Solutions

### Issue 1: Thread Safety Errors
```
error[E0277]: `T` cannot be sent between threads safely
```

**Solution**: Add `Send + Sync` bounds to your types:
```rust
pub struct MyContext {
    // Your fields
}

// Add these derives
#[derive(Clone, PartialEq, Send, Sync)]
pub struct MyContext {
    // Your fields
}
```

### Issue 2: Signal Storage Errors
```
error[E0599]: the method `get` exists for struct `ReadSignal<...>`, but its trait bounds were not satisfied
```

**Solution**: Update signal creation and access:
```rust
// Use create_rw_signal instead of create_signal
let (state, set_state) = create_rw_signal(initial_value);

// Access signals directly
let value = state.get();
```

### Issue 3: Callback API Changes
```
error[E0599]: no method named `call` found for reference `&Callback<...>`
```

**Solution**: Use direct invocation:
```rust
// Before
callback.call(value);

// After
callback(value);
```

## Development Workflow

### 1. Incremental Changes
- Make small, focused changes
- Test after each change
- Commit frequently with descriptive messages

### 2. Testing Strategy
```bash
# Test specific components
cargo test -p leptos-state --lib

# Test examples
cargo test -p todo-app
cargo test -p analytics-dashboard

# Check for warnings
cargo check --all-targets
```

### 3. Debugging
```bash
# Verbose compilation
cargo check --verbose

# Show dependency tree
cargo tree -p leptos-state

# Check feature flags
cargo check --features "full"
```

## Migration Checklist

### Phase 1: Foundation ✅
- [ ] Update `StateMachine` trait bounds
- [ ] Update `MachineState` trait bounds
- [ ] Replace `create_signal` with `create_rw_signal`
- [ ] Update callback usage
- [ ] Fix basic compilation errors

### Phase 2: Store System
- [ ] Update `Store` trait bounds
- [ ] Fix `StoreContext` implementation
- [ ] Update async store features
- [ ] Fix store history system

### Phase 3: Machine System
- [ ] Update machine builder API
- [ ] Fix machine hooks
- [ ] Update machine features
- [ ] Test machine functionality

### Phase 4: Examples & Tests
- [ ] Update Todo App example
- [ ] Update Analytics Dashboard example
- [ ] Fix all tests
- [ ] Update documentation

## Useful Commands

```bash
# Check what needs to be updated
cargo check 2>&1 | grep -E "(error|warning)"

# Find all create_signal usages
grep -r "create_signal" src/

# Find all callback.call usages
grep -r "\.call(" src/

# Check for Send/Sync issues
cargo check 2>&1 | grep -E "Send|Sync"
```

## Getting Help

### Documentation
- [Leptos 0.8 Migration Guide](https://leptos.dev/book/0.8/migration.html)
- [Leptos 0.8 API Reference](https://docs.rs/leptos/0.8/)

### Community
- [Leptos Discord](https://discord.gg/YdRAhS7eQB)
- [Leptos GitHub Issues](https://github.com/leptos-rs/leptos/issues)

### Project Resources
- [Full Migration Roadmap](./LEPTOS_0_8_MIGRATION_ROADMAP.md)
- [Project Documentation](./docs/)
- [Examples](./examples/)

## Next Steps

1. **Start with Phase 1** - Core trait updates
2. **Test frequently** - Don't let errors accumulate
3. **Ask for help** - Use community resources
4. **Document your progress** - Help others follow

---

**Remember**: This is a significant migration. Take your time, test thoroughly, and don't hesitate to ask for help!
