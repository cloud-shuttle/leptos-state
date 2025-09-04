use leptos::*;
use leptos_state::v1::*;

// =============================================================================
// Counter State Machine
// =============================================================================

/// Counter context
#[derive(Clone, Debug, Default)]
struct CounterContext {
    count: i32,
    max_count: i32,
    min_count: i32,
}

impl StateMachineContext for CounterContext {}

/// Counter events
#[derive(Clone, Debug, PartialEq)]
enum CounterEvent {
    Increment,
    Decrement,
    Reset,
    SetValue(i32),
    SetLimits { min: i32, max: i32 },
}

impl StateMachineEvent for CounterEvent {}

impl Default for CounterEvent {
    fn default() -> Self {
        CounterEvent::Reset
    }
}

/// Counter states
#[derive(Clone, Debug, PartialEq)]
enum CounterState {
    Idle,
    AtMin,
    AtMax,
    InRange,
}

impl StateMachineState for CounterState {
    type Context = CounterContext;
    type Event = CounterEvent;
}

impl Default for CounterState {
    fn default() -> Self {
        CounterState::Idle
    }
}

impl StoreState for CounterState {}

// =============================================================================
// Counter Actions
// =============================================================================

#[derive(Clone, Debug)]
struct IncrementAction;

impl Action<CounterContext> for IncrementAction {
    fn execute(&self, context: &mut CounterContext) -> Result<(), error::ActionError> {
        if context.count < context.max_count {
            context.count += 1;
            Ok(())
        } else {
            Err(error::ActionError::ValidationFailed("Cannot increment beyond max".to_string()))
        }
    }

    fn description(&self) -> &'static str {
        "Increment counter"
    }
}

#[derive(Clone, Debug)]
struct DecrementAction;

impl Action<CounterContext> for DecrementAction {
    fn execute(&self, context: &mut CounterContext) -> Result<(), error::ActionError> {
        if context.count > context.min_count {
            context.count -= 1;
            Ok(())
        } else {
            Err(error::ActionError::ValidationFailed("Cannot decrement below min".to_string()))
        }
    }

    fn description(&self) -> &'static str {
        "Decrement counter"
    }
}

#[derive(Clone, Debug)]
struct ResetAction;

impl Action<CounterContext> for ResetAction {
    fn execute(&self, context: &mut CounterContext) -> Result<(), error::ActionError> {
        context.count = 0;
        Ok(())
    }

    fn description(&self) -> &'static str {
        "Reset counter to zero"
    }
}

#[derive(Clone, Debug)]
struct SetValueAction(i32);

impl Action<CounterContext> for SetValueAction {
    fn execute(&self, context: &mut CounterContext) -> Result<(), error::ActionError> {
        let value = self.0;
        if value >= context.min_count && value <= context.max_count {
            context.count = value;
            Ok(())
        } else {
            Err(error::ActionError::ValidationFailed(
                format!("Value {} is outside allowed range [{}, {}]", value, context.min_count, context.max_count)
            ))
        }
    }

    fn description(&self) -> &'static str {
        "Set counter to specific value"
    }
}

// =============================================================================
// Counter Guards
// =============================================================================

#[derive(Clone, Debug)]
struct CanIncrementGuard;

impl Guard<CounterContext, CounterEvent> for CanIncrementGuard {
    fn check(&self, context: &CounterContext, _event: &CounterEvent) -> bool {
        context.count < context.max_count
    }

    fn description(&self) -> &'static str {
        "Check if counter can be incremented"
    }
}

#[derive(Clone, Debug)]
struct CanDecrementGuard;

impl Guard<CounterContext, CounterEvent> for CanDecrementGuard {
    fn check(&self, context: &CounterContext, _event: &CounterEvent) -> bool {
        context.count > context.min_count
    }

    fn description(&self) -> &'static str {
        "Check if counter can be decremented"
    }
}

#[derive(Clone, Debug)]
struct ValueInRangeGuard;

impl Guard<CounterContext, CounterEvent> for ValueInRangeGuard {
    fn check(&self, context: &CounterContext, event: &CounterEvent) -> bool {
        match event {
            CounterEvent::SetValue(value) => {
                *value >= context.min_count && *value <= context.max_count
            }
            _ => true,
        }
    }

    fn description(&self) -> &'static str {
        "Check if value is within allowed range"
    }
}

// =============================================================================
// Counter Component
// =============================================================================

#[component]
fn Counter() -> impl IntoView {
    // Create state machine
    let machine = create_memo(move |_| {
        let mut builder = MachineBuilder::new();
        
        builder
            .with_state("idle")
            .with_state("at_min")
            .with_state("at_max")
            .with_state("in_range")
            .initial("idle")
            .with_context_factory(|| CounterContext {
                count: 0,
                max_count: 100,
                min_count: -50,
            })
            .build()
            .expect("Failed to build counter machine")
    });

    // Create store for UI state
    let store = create_memo(move |_| {
        StateStore::new()
            .with_initial_state(CounterState::Idle)
            .with_persistence(store::PersistenceConfig {
                enabled: true,
                format: store::SerializationFormat::Json,
                key: "counter_state".to_string(),
            })
    });

    // Reactive signals
    let (count, set_count) = create_signal(0);
    let (state, set_state) = create_signal(CounterState::Idle);
    let (can_increment, set_can_increment) = create_signal(true);
    let (can_decrement, set_can_decrement) = create_signal(true);

    // Event handlers
    let increment = move |_| {
        let mut machine = machine.get();
        let event = CounterEvent::Increment;
        
        if let Ok(new_state) = machine.transition(event) {
            set_count.set(machine.context().count);
            set_state.set(new_state);
            set_can_increment.set(machine.context().count < machine.context().max_count);
            set_can_decrement.set(machine.context().count > machine.context().min_count);
        }
    };

    let decrement = move |_| {
        let mut machine = machine.get();
        let event = CounterEvent::Decrement;
        
        if let Ok(new_state) = machine.transition(event) {
            set_count.set(machine.context().count);
            set_state.set(new_state);
            set_can_increment.set(machine.context().count < machine.context().max_count);
            set_can_decrement.set(machine.context().count > machine.context().min_count);
        }
    };

    let reset = move |_| {
        let mut machine = machine.get();
        let event = CounterEvent::Reset;
        
        if let Ok(new_state) = machine.transition(event) {
            set_count.set(machine.context().count);
            set_state.set(new_state);
            set_can_increment.set(machine.context().count < machine.context().max_count);
            set_can_decrement.set(machine.context().count > machine.context().min_count);
        }
    };

    let set_value = move |value: i32| {
        let mut machine = machine.get();
        let event = CounterEvent::SetValue(value);
        
        if let Ok(new_state) = machine.transition(event) {
            set_count.set(machine.context().count);
            set_state.set(new_state);
            set_can_increment.set(machine.context().count < machine.context().max_count);
            set_can_decrement.set(machine.context().count > machine.context().min_count);
        }
    };

    // Initialize state
    create_effect(move |_| {
        let machine = machine.get();
        set_count.set(machine.context().count);
        set_can_increment.set(machine.context().count < machine.context().max_count);
        set_can_decrement.set(machine.context().count > machine.context().min_count);
    });

    view! {
        <div class="counter-container">
            <h2>"Counter Example"</h2>
            
            <div class="counter-display">
                <span class="count">{count}</span>
                <span class="state">"State: {format!("{:?}", state)}"</span>
            </div>
            
            <div class="counter-controls">
                <button 
                    on:click=decrement 
                    disabled=move || !can_decrement.get()
                    class="counter-btn decrement"
                >
                    "Decrement"
                </button>
                
                <button 
                    on:click=reset 
                    class="counter-btn reset"
                >
                    "Reset"
                </button>
                
                <button 
                    on:click=increment 
                    disabled=move || !can_increment.get()
                    class="counter-btn increment"
                >
                    "Increment"
                </button>
            </div>
            
            <div class="counter-actions">
                <button 
                    on:click=move |_| set_value(10)
                    class="action-btn"
                >
                    "Set to 10"
                </button>
                <button 
                    on:click=move |_| set_value(-10)
                    class="action-btn"
                >
                    "Set to -10"
                </button>
                <button 
                    on:click=move |_| set_value(50)
                    class="action-btn"
                >
                    "Set to 50"
                </button>
            </div>
            
            <div class="counter-info">
                <p>"Range: -50 to 100"</p>
                <p>"Can increment: {can_increment}"</p>
                <p>"Can decrement: {can_decrement}"</p>
            </div>
        </div>
    }
}

// =============================================================================
// Main App
// =============================================================================

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="app">
            <h1>"Leptos State - Counter Example"</h1>
            <Counter />
        </div>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App /> });
}

// =============================================================================
// CSS Styles
// =============================================================================

#[cfg(target_arch = "wasm32")]
mod styles {
    use wasm_bindgen::prelude::*;
    use web_sys::window;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        fn log(s: &str);
    }

    pub fn inject_styles() {
        let window = window().expect("No window found");
        let document = window.document().expect("No document found");
        let head = document.head().expect("No head found");
        
        let style = document.create_element("style").expect("Failed to create style element");
        style.set_inner_html(include_str!("styles.css"));
        head.append_child(&style).expect("Failed to append style");
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    styles::inject_styles();
    leptos::mount_to_body(|| view! { <App /> });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    leptos::mount_to_body(|| view! { <App /> });
}
