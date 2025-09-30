# Leptos State Minimal

A minimal, maintainable state management library for Leptos, designed with simplicity and usability in mind.

## Features

- **Simple trait bounds**: Only requires `Send + Sync + Clone + 'static` for reactive compatibility
- **Reactive stores**: Zustand-inspired state management with Leptos signals
- **State machines**: XState-inspired finite state machines with guards and actions
- **Leptos hooks**: Easy integration with Leptos components
- **Type-safe**: Compile-time guarantees for state transitions and updates

## Quick Start

```rust
use leptos::*;
use leptos_state_minimal::{use_store, State};

// Define your state
#[derive(Clone, Default)]
struct CounterState {
    count: i32,
    step: i32,
}

#[component]
fn Counter() -> impl IntoView {
    // Use the store hook
    let (state, actions) = use_store::<CounterState>();

    // Update state
    let increment = move |_| {
        actions.update(|s| s.count += s.step).unwrap();
    };

    view! {
        <div>
            <p>"Count: " {move || state.get().count}</p>
            <button on:click=increment>"Increment"</button>
        </div>
    }
}
```

## State Machines

```rust
use leptos_state_minimal::{use_machine, Event, State};

#[derive(Clone)]
enum TrafficEvent { Next }

#[derive(Clone, Default)]
struct TrafficContext;

fn create_machine() -> Machine<TrafficContext, TrafficEvent> {
    let mut machine = Machine::new("red", TrafficContext::default());

    // Add states and transitions
    let red_state = StateNode::new().on(TrafficEvent::Next, "green");
    machine.add_state("red", red_state);

    // ... add more states

    machine
}

#[component]
fn TrafficLight() -> impl IntoView {
    let (current_state, actions) = use_machine(create_machine());

    let next = move |_| {
        actions.send(TrafficEvent::Next).unwrap();
    };

    view! {
        <div>
            <p>"State: " {current_state}</p>
            <button on:click=next>"Next"</button>
        </div>
    }
}
```

## Philosophy

This library takes a different approach from the original leptos-state:

- **Minimal bounds**: Only requires what's necessary for reactivity
- **User-friendly**: Easy to understand and use
- **Progressive enhancement**: Core functionality works without advanced features
- **Maintainable**: Clean, simple code that's easy to extend

## Differences from Original

The original leptos-state library had overly restrictive trait bounds that caused compilation issues. This minimal version:

- Uses `Send + Sync + Clone + 'static` instead of complex bounds
- Provides the same core functionality in a simpler package
- Compiles without errors
- Is easier to maintain and extend

## Examples

See the `examples/` directory for working examples:
- `counter/`: Simple counter with reactive state
- More examples coming soon!

## Testing

```bash
cargo test
```

## Building Examples

```bash
cd examples/counter
cargo build --target wasm32-unknown-unknown
```

## License

MIT OR Apache-2.0
