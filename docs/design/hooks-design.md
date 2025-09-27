# 🪝 Hooks Design

## Overview
Reactive hooks for integrating stores and machines with Leptos components.

## Architecture

### Core Components
```
hooks/
├── store_hooks.rs     # use_store, create_store hooks (150 lines)
├── machine_hooks.rs   # use_machine, use_transition hooks (120 lines)
├── async_hooks.rs     # use_async_store, use_resource hooks (100 lines)
└── mod.rs             # Hook exports and utilities (50 lines)
```

## Store Hooks

```rust
pub fn use_store<T>() -> (ReadSignal<T>, StoreActions<T>)
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    let store = create_store(T::default());
    let (signal, set_signal) = create_signal(store.get().get());

    create_effect(move |_| {
        let current = store.get().get();
        if signal.get() != current {
            set_signal.set(current);
        }
    });

    let actions = StoreActions::new(store.clone());
    (signal, actions)
}

pub struct StoreActions<T> {
    store: Store<T>,
}

impl<T> StoreActions<T>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    pub fn new(store: Store<T>) -> Self {
        Self { store }
    }

    pub fn update<F>(&self, updater: F)
    where
        F: FnOnce(&mut T) + Send + 'static,
    {
        self.store.update(updater);
    }

    pub fn set(&self, new_value: T) {
        self.store.set(new_value);
    }

    pub fn reset(&self) {
        self.store.reset();
    }
}

pub fn create_store<T>(initial: T) -> Store<T>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    Store::new(initial)
}
```

## Machine Hooks

```rust
pub fn use_machine<M>(machine: M) -> (ReadSignal<M::State>, MachineActions<M::Event>)
where
    M: MachineTrait + 'static,
    M::State: Clone + Send + Sync + 'static,
    M::Event: Clone + Send + Sync + 'static,
{
    let (state_signal, set_state) = create_signal(machine.initial());
    let machine_ref = Arc::new(RwLock::new(machine));

    let actions = MachineActions::new(machine_ref.clone());

    // Subscribe to state changes
    let machine_clone = machine_ref.clone();
    create_effect(move |_| {
        // This would need to be implemented based on how the machine exposes state changes
        // For now, this is a placeholder
    });

    (state_signal, actions)
}

pub struct MachineActions<E> {
    machine: Arc<RwLock<dyn MachineTrait<Event = E>>>,
}

impl<E> MachineActions<E>
where
    E: Clone + Send + Sync + 'static,
{
    pub fn new(machine: Arc<RwLock<dyn MachineTrait<Event = E>>>) -> Self {
        Self { machine }
    }

    pub fn send(&self, event: E) {
        let mut machine = self.machine.write().unwrap();
        let _ = machine.transition(event);
        // Update state signal would need to be implemented
    }

    pub fn can_send(&self, event: &E) -> bool {
        let machine = self.machine.read().unwrap();
        // Check if event can be sent from current state
        true // Placeholder
    }
}

pub fn use_machine_with_instance<M>(machine: M) -> (ReadSignal<M::State>, Callback<M::Event>)
where
    M: MachineTrait + Clone + 'static,
    M::State: Clone + Send + Sync + 'static,
    M::Event: Clone + Send + Sync + 'static,
{
    let (state_signal, set_state) = create_signal(machine.initial());
    let machine_ref = create_rw_signal(machine);

    let send_callback = Callback::new(move |event: M::Event| {
        let mut machine = machine_ref.get();
        let _ = machine.transition(event);
        machine_ref.set(machine);
        // Update state signal
    });

    (state_signal, send_callback)
}
```

## Async Store Hooks

```rust
pub fn use_async_store<T, F, Fut>(fetcher: F) -> AsyncStore<T>
where
    T: Clone + Send + Sync + 'static,
    F: Fn() -> Fut + 'static,
    Fut: Future<Output = T> + 'static,
{
    let resource = create_resource(|| (), move |_| fetcher());
    AsyncStore::new(resource)
}

pub struct AsyncStore<T> {
    resource: Resource<(), T>,
    state: Option<T>,
    error: Option<ResourceError>,
}

impl<T> AsyncStore<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new(resource: Resource<(), T>) -> Self {
        Self {
            resource,
            state: None,
            error: None,
        }
    }

    pub fn data(&self) -> Option<&T> {
        self.state.as_ref()
    }

    pub fn is_loading(&self) -> bool {
        matches!(self.resource.read(), Some(ResourceState::Loading))
    }

    pub fn error(&self) -> Option<&ResourceError> {
        self.error.as_ref()
    }

    pub fn refetch(&self) {
        // Trigger resource refetch
        // Implementation depends on resource API
    }
}
```

## Hook Composition

```rust
pub fn use_store_with_machine<T, M>(
    store: Store<T>,
    machine: M,
) -> (ReadSignal<T>, ReadSignal<M::State>, StoreActions<T>, MachineActions<M::Event>)
where
    T: Clone + PartialEq + Send + Sync + 'static,
    M: MachineTrait + 'static,
    M::State: Clone + Send + Sync + 'static,
    M::Event: Clone + Send + Sync + 'static,
{
    let (store_signal, store_actions) = use_store_with_store(store);
    let (machine_signal, machine_actions) = use_machine(machine);

    (store_signal, machine_signal, store_actions, machine_actions)
}

pub fn use_store_with_store<T>(store: Store<T>) -> (ReadSignal<T>, StoreActions<T>)
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    let (signal, set_signal) = create_signal(store.get().get());

    create_effect(move |_| {
        let current = store.get().get();
        if signal.get() != current {
            set_signal.set(current);
        }
    });

    let actions = StoreActions::new(store);
    (signal, actions)
}
```

## Type Definitions

```rust
pub type HookResult<T> = (ReadSignal<T>, WriteSignal<T>);

pub trait StoreHook<T>: Send + Sync {
    fn get_store(&self) -> &Store<T>;
    fn update_store<F>(&self, updater: F)
    where
        F: FnOnce(&mut T) + Send + 'static;
}

pub trait MachineHook<E>: Send + Sync {
    fn send_event(&self, event: E);
    fn can_send(&self, event: &E) -> bool;
    fn get_current_state(&self) -> String;
}
```

## Testing Strategy

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn use_store_returns_correct_types() {
        let (signal, actions) = use_store::<CounterState>();
        assert!(signal.get().count == 0);
        // Test actions work
    }

    #[test]
    fn use_machine_handles_transitions() {
        let machine = create_traffic_light_machine();
        let (state, actions) = use_machine(machine);

        assert_eq!(state.get(), "red");
        actions.send(TrafficLightEvent::Next);
        // Verify state changed
    }

    #[test]
    fn async_store_handles_loading_states() {
        let store = use_async_store(|| async { 42 });
        assert!(store.is_loading());
        // Wait for completion and verify data
    }
}
```

## Performance Considerations

- **Signal Creation:** Minimize signal overhead in hooks
- **Effect Dependencies:** Optimize effect dependencies
- **Memory Management:** Clean up subscriptions on unmount
- **Update Batching:** Batch multiple hook updates

## Future Extensions

- [ ] Suspense integration
- [ ] Error boundaries
- [ ] Hook composition utilities
- [ ] DevTools integration
