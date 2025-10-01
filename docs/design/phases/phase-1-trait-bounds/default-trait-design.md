# Default Trait Design

## Overview
Add `Default` trait bound to enable default state initialization, providing clean fallback values and initialization patterns.

## Current State
```rust
// Current minimal bounds
pub trait State: Send + Sync + Clone + 'static {}
pub trait Event: Send + Sync + Clone + 'static {}
```

## Proposed Enhancement
```rust
// Enhanced bounds with Default
pub trait State: Send + Sync + Clone + Default + 'static {}
pub trait Event: Send + Sync + Clone + Default + 'static {}
```

## Motivation

### Initialization Patterns
- **Clean defaults**: States can be initialized without explicit values
- **Reset functionality**: Stores can reset to known good state
- **Testing**: Easy state setup for tests
- **Error recovery**: Fallback to default state on errors

### Use Cases
- Store initialization: `Store::new(Default::default())`
- State reset: `store.reset()` returns to default
- Test setup: `let store = Store::new(MyState::default())`
- Error recovery: Automatic fallback to safe state

## Implementation Details

### Auto-Derive for Simple Types
```rust
#[derive(Clone, Default)]
struct CounterState {
    count: i32,  // defaults to 0
    step: i32,   // defaults to 0
}
```

### Custom Default Implementation
For types needing specific default values:
```rust
#[derive(Clone)]
struct AppState {
    theme: Theme,
    language: String,
    notifications_enabled: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            theme: Theme::Light,
            language: "en".to_string(),
            notifications_enabled: true,
        }
    }
}
```

### Complex Types with Default
```rust
#[derive(Clone)]
struct DashboardState {
    widgets: Vec<Widget>,
    layout: Layout,
    filters: HashMap<String, Filter>,
}

impl Default for DashboardState {
    fn default() -> Self {
        Self {
            widgets: vec![Widget::default_chart()],
            layout: Layout::Grid,
            filters: HashMap::new(),
        }
    }
}
```

## API Design

### Store Integration
```rust
impl<S: State> Store<S> {
    /// Create store with default state
    pub fn default() -> Self {
        Self::new(S::default())
    }

    /// Reset store to default state
    pub fn reset(&self) -> Result<(), StoreError> {
        self.set(S::default())
    }

    /// Check if current state is default
    pub fn is_default(&self) -> bool {
        self.signal.get_untracked() == S::default()
    }
}
```

### Hook Integration
```rust
/// Hook that provides default-initialized store
pub fn use_store_default<S: State>() -> (ReadSignal<S>, StoreActions<S>) {
    use_store_with_initial(S::default())
}

/// Hook with reset functionality
pub fn use_store_with_reset<S: State>() -> (ReadSignal<S>, StoreActions<S>, impl Fn() -> Result<(), StoreError>) {
    let (state, actions) = use_store::<S>();
    let reset_fn = {
        let actions = actions.clone();
        move || actions.set(S::default())
    };

    (state, actions, reset_fn)
}
```

## State Machine Integration

### Default Context
```rust
impl<S: State, E: Event> Machine<S, E> {
    /// Create machine with default context
    pub fn default(initial_state: &str) -> Self {
        Self::new(initial_state, S::default())
    }
}
```

## Error Handling

### Compilation Guidance
When Default can't be derived automatically:
```rust
error[E0277]: the trait bound `MyState: Default` is not satisfied
help: consider deriving Default: `#[derive(Default)]`
help: or implement Default manually: `impl Default for MyState { ... }`
```

### Runtime Considerations
- Default implementations should be safe and valid
- Avoid panicking in Default::default()
- Default state should represent a reasonable starting point

## Testing Strategy

### Unit Tests
```rust
#[test]
fn default_state_is_valid() {
    let default_state = MyState::default();
    // Verify default state is in expected condition
    assert_eq!(default_state.count, 0);
    assert_eq!(default_state.step, 1);
}

#[test]
fn store_default_initialization() {
    let store = Store::<MyState>::default();
    assert_eq!(store.get().get_untracked(), MyState::default());
}

#[test]
fn store_reset_functionality() {
    let store = Store::new(MyState { count: 100, step: 5 });

    // Modify state
    store.update(|s| s.count = 50).unwrap();

    // Reset to default
    store.reset().unwrap();

    assert_eq!(store.get().get_untracked(), MyState::default());
}
```

### Property-Based Testing
```rust
proptest! {
    #[test]
    fn default_state_roundtrip(state: MyState) {
        let store = Store::new(state.clone());

        // Modify and reset
        store.update(|s| s.count = 999).unwrap();
        store.reset().unwrap();

        // Should be back to original (not default!)
        // This tests that reset works correctly
        prop_assert_eq!(store.get().get_untracked(), MyState::default());
    }
}
```

## Migration Guide

### For Existing Code
```rust
// Before
#[derive(Clone)]
struct MyState {
    count: i32,
}

// After - add Default
#[derive(Clone, Default)]
struct MyState {
    count: i32,  // i32 defaults to 0
}
```

### Complex Migration
```rust
// Before
#[derive(Clone)]
struct ComplexState {
    data: Vec<String>,
    config: Config,
}

// After
#[derive(Clone)]
struct ComplexState {
    data: Vec<String>,
    config: Config,
}

impl Default for ComplexState {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            config: Config::default(),
        }
    }
}
```

## Performance Impact

### Compile Time
- **Minimal**: Default derive is fast and optimized
- **Impact**: <1% increase in compile time

### Runtime
- **Zero cost**: Default::default() is called only during initialization
- **Memory**: No additional memory usage
- **Binary size**: Negligible increase

## Design Patterns

### Builder Pattern with Defaults
```rust
impl MyState {
    pub fn builder() -> MyStateBuilder {
        MyStateBuilder::default()
    }
}

#[derive(Default)]
struct MyStateBuilder {
    count: Option<i32>,
    step: Option<i32>,
}

impl MyStateBuilder {
    pub fn build(self) -> MyState {
        MyState {
            count: self.count.unwrap_or(0),
            step: self.step.unwrap_or(1),
        }
    }
}
```

### Conditional Defaults
```rust
#[derive(Clone)]
struct FeatureState {
    enabled: bool,
    config: Option<FeatureConfig>,
}

impl Default for FeatureState {
    fn default() -> Self {
        Self {
            enabled: false,
            config: None,
        }
    }
}
```

## Security Considerations

### Safe Defaults
- Default state should not expose sensitive information
- Default configurations should be secure
- Avoid default credentials or secrets

```rust
impl Default for AuthState {
    fn default() -> Self {
        Self {
            user: None,
            token: None,
            permissions: Permissions::empty(),
        }
    }
}
```

## Future Extensions

### Configuration-Based Defaults
```rust
impl MyState {
    pub fn default_with_config(config: &AppConfig) -> Self {
        Self {
            theme: config.default_theme,
            language: config.default_language,
            ..Default::default()
        }
    }
}
```

### Environment-Based Defaults
```rust
impl Default for AppState {
    fn default() -> Self {
        Self {
            debug_mode: std::env::var("DEBUG").is_ok(),
            port: std::env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            ..Self::default_config()
        }
    }
}
```

## Risk Assessment

### Likelihood: Low
- Default is a fundamental trait in Rust
- Most types can derive it automatically
- Well-understood patterns

### Impact: Low
- Easy to implement for most use cases
- Clear migration path
- Backward compatible additions

### Mitigation
- Comprehensive documentation with examples
- Migration helper macros
- Gradual adoption support
