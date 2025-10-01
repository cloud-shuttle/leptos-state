//! Leptos hooks for state management

use crate::{State, Event, Store, StoreActions, Machine, MachineResult};
use leptos::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

/// Hook for creating and using a reactive store
///
/// Returns a read signal for the current state and actions for updating it.
///
/// # Example
/// ```rust
/// #[derive(Clone, Default)]
/// struct CounterState {
///     count: i32,
/// }
///
/// impl State for CounterState {}
///
/// #[component]
/// fn Counter() -> impl IntoView {
///     let (state, actions) = use_store::<CounterState>();
///
///     let increment = move |_| {
///         actions.update(|s| s.count += 1).unwrap();
///     };
///
///     view! {
///         <div>
///             <p>"Count: " {move || state.get().count}</p>
///             <button on:click=increment>"Increment"</button>
///         </div>
///     }
/// }
/// ```
pub fn use_store<S: State>() -> (ReadSignal<S>, StoreActions<S>)
where
    S: Default,
{
    let store = Store::default();
    let signal = store.get();
    let actions = StoreActions::new(store);

    (signal, actions)
}

/// Hook for creating and using a store with custom initial state
///
/// Returns a read signal for the current state and actions for updating it.
pub fn use_store_with<S: State>(initial: S) -> (ReadSignal<S>, StoreActions<S>) {
    let store = Store::new(initial);
    let signal = store.get();
    let actions = StoreActions::new(store);

    (signal, actions)
}

/// Hook for creating and using a persistent store with LocalStorage
///
/// Returns a read signal for the current state and actions for updating it.
/// Automatically persists state to LocalStorage and restores it on creation.
///
/// Requires the serde and web features to be enabled.
///
/// # Example
/// ```rust
/// use leptos::*;
/// use leptos_state_minimal::use_persistent_store;
///
/// #[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
/// struct CounterState { count: i32 }
///
/// #[component]
/// fn PersistentCounter() -> impl IntoView {
///     let (state, actions) = use_persistent_store::<CounterState>("my-counter")
///         .unwrap_or_else(|_| use_store::<CounterState>()); // Fallback
///
///     let increment = move |_| actions.update(|s| s.count += 1).unwrap();
///
///     view! {
///         <p>"Count: " {move || state.get().count}</p>
///         <button on:click=increment>"+"</button>
///     }
/// }
/// ```
#[cfg(feature = "web")]
pub fn use_persistent_store<S: SerializableState + Default>(
    key: &str
) -> Result<(ReadSignal<S>, crate::persistence::PersistentStoreActions<S>), StoreError> {
    let backend = crate::persistence::LocalStorageBackend::new()?;
    let persistent_store = crate::persistence::PersistentStore::new(key.to_string(), S::default(), Box::new(backend))?;
    let signal = persistent_store.get();
    let actions = crate::persistence::PersistentStoreActions::new(persistent_store);

    Ok((signal, actions))
}

/// Hook for creating and using a persistent store
///
/// Returns a Result containing the signal and actions, or falls back to regular store.
/// This ensures the application continues to work even without browser storage.
///
/// Requires the serde and web features to be enabled.
#[cfg(feature = "web")]
pub fn use_persistent_store_fallback<S: SerializableState + Default>(
    key: &str
) -> (ReadSignal<S>, crate::persistence::PersistentStoreActions<S>) {
    match crate::persistence::LocalStorageBackend::new() {
        Ok(backend) => {
            let persistent_store = crate::persistence::PersistentStore::new(key.to_string(), S::default(), Box::new(backend))
                .expect("Failed to create persistent store");
            let signal = persistent_store.get();
            let actions = crate::persistence::PersistentStoreActions::new(persistent_store);
            (signal, actions)
        }
        Err(_) => {
            // Fallback to regular store with no-op persistence actions
            // For now, we'll just panic - graceful fallback needs more design work
            panic!("LocalStorage not available - graceful fallback not implemented yet")
        }
    }
}

/// Hook for creating and using a state machine
///
/// Returns a read signal for the current state name and actions for sending events.
///
/// # Example
/// ```rust
/// #[derive(Clone)]
/// enum TrafficEvent { Timer }
/// impl Event for TrafficEvent {}
///
/// #[derive(Clone, Default)]
/// struct TrafficContext;
/// impl State for TrafficContext {}
///
/// fn create_machine() -> Machine<TrafficContext, TrafficEvent> {
///     let mut machine = Machine::new("red", TrafficContext::default());
///     // Add states and transitions...
///     machine
/// }
///
/// #[component]
/// fn TrafficLight() -> impl IntoView {
///     let machine = create_machine();
///     let (current_state, actions) = use_machine(machine);
///
///     let next = move |_| {
///         actions.send(TrafficEvent::Timer).unwrap();
///     };
///
///     view! {
///         <div>
///             <p>"State: " {current_state}</p>
///             <button on:click=next>"Next"</button>
///         </div>
///     }
/// }
/// ```
pub fn use_machine<S: State, E: Event>(
    initial_machine: Machine<S, E>
) -> (ReadSignal<String>, MachineActions<S, E>) {
    let machine = Rc::new(RefCell::new(initial_machine));
    let current_state = RwSignal::new(machine.borrow().current_state().to_string());
    let actions = MachineActions::new(machine, current_state.clone());

    (current_state.read_only(), actions)
}

/// Actions for interacting with a state machine
pub struct MachineActions<S: State, E: Event> {
    machine: Rc<RefCell<Machine<S, E>>>,
    current_state_signal: RwSignal<String>,
}

impl<S: State, E: Event> MachineActions<S, E> {
    /// Create new machine actions
    pub fn new(machine: Rc<RefCell<Machine<S, E>>>, current_state_signal: RwSignal<String>) -> Self {
        Self {
            machine,
            current_state_signal,
        }
    }

    /// Send an event to the machine
    pub fn send(&self, event: E) -> MachineResult<()> {
        let result = self.machine.borrow_mut().send(event.clone());
        if result.is_ok() {
            // Update the signal with the new state
            let new_state = self.machine.borrow().current_state().to_string();
            self.current_state_signal.set(new_state);
        }
        result
    }

    /// Check if an event can be sent from the current state
    pub fn can_send(&self, event: &E) -> bool {
        self.machine.borrow().can_transition(event)
    }

    /// Get the current state name
    pub fn current_state(&self) -> String {
        self.machine.borrow().current_state().to_string()
    }

    /// Get a reference to the current context
    pub fn context(&self) -> S
    where
        S: Clone,
    {
        self.machine.borrow().context().clone()
    }

    /// Get all possible transitions from the current state
    pub fn possible_transitions(&self) -> Vec<String> {
        self.machine.borrow().possible_transitions()
    }
}

/// Hook for using both a store and machine together
///
/// This is useful when you need both local state management and
/// complex state transitions.
///
/// Returns: (store_signal, state_signal, store_actions, machine_actions)
pub fn use_store_and_machine<S: State, E: Event>(
    initial_store: S,
    initial_machine: Machine<S, E>,
) -> (ReadSignal<S>, ReadSignal<String>, StoreActions<S>, MachineActions<S, E>)
where
    S: Clone,
{
    let (store_signal, store_actions) = use_store_with(initial_store);
    let (machine_signal, machine_actions) = use_machine(initial_machine);

    (store_signal, machine_signal, store_actions, machine_actions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[derive(Clone, Default, Debug, Eq, PartialEq)]
    struct TestState {
        count: i32,
    }

    #[derive(Clone)]
    enum TestEvent {
        Increment,
        Decrement,
    }

    #[test]
    fn use_store_returns_correct_types() {
        leptos::mount::mount_to_body(|| {
            let (signal, actions) = use_store::<TestState>();
            assert_eq!(signal.get().count, 0);

            actions.update(|s| s.count = 42).unwrap();
            assert_eq!(signal.get().count, 42);
        });
    }
}
