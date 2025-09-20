use leptos::prelude::*;
use leptos_state::v1::{StateMachineContext, StateMachineEvent, StateMachineState, Action, Guard, Context, ActionError, ContextError};

// =============================================================================
// Counter State Machine
// =============================================================================

/// Counter context
#[derive(Clone, Debug, Default, PartialEq)]
struct CounterContext {
    count: i32,
    max_count: i32,
    min_count: i32,
}

impl StateMachineContext for CounterContext {}

/// Counter events
#[derive(Clone, Debug, PartialEq)]
enum CounterEvent {
    #[allow(dead_code)]
    Increment,
    #[allow(dead_code)]
    Decrement,
    Reset,
    #[allow(dead_code)]
    SetValue(i32),
    #[allow(dead_code)]
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
#[allow(dead_code)]
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

// =============================================================================
// Counter Actions
// =============================================================================

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct IncrementAction;

impl Action<CounterContext> for IncrementAction {
    fn execute(&self, context: &mut CounterContext) -> Result<(), ActionError> {
        if context.count < context.max_count {
            context.count += 1;
            Ok(())
        } else {
            Err(ActionError::ValidationFailed("Cannot increment beyond max".to_string()))
        }
    }

    fn description(&self) -> &'static str {
        "Increment counter"
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct DecrementAction;

impl Action<CounterContext> for DecrementAction {
    fn execute(&self, context: &mut CounterContext) -> Result<(), ActionError> {
        if context.count > context.min_count {
            context.count -= 1;
            Ok(())
        } else {
            Err(ActionError::ValidationFailed("Cannot decrement below min".to_string()))
        }
    }

    fn description(&self) -> &'static str {
        "Decrement counter"
    }
}

#[derive(Clone, Debug)]
struct ResetAction;

impl Action<CounterContext> for ResetAction {
    fn execute(&self, context: &mut CounterContext) -> Result<(), ActionError> {
        context.count = 0;
        Ok(())
    }

    fn description(&self) -> &'static str {
        "Reset counter"
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct SetValueAction(i32);

impl Action<CounterContext> for SetValueAction {
    fn execute(&self, context: &mut CounterContext) -> Result<(), ActionError> {
        let value = self.0;
        if value >= context.min_count && value <= context.max_count {
            context.count = value;
            Ok(())
        } else {
            Err(ActionError::ValidationFailed(
                format!("Value {} is outside allowed range [{}, {}]", value, context.min_count, context.max_count)
            ))
        }
    }

    fn description(&self) -> &'static str {
        "Set counter value"
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct SetLimitsAction {
    min: i32,
    max: i32,
}

impl Action<CounterContext> for SetLimitsAction {
    fn execute(&self, context: &mut CounterContext) -> Result<(), ActionError> {
        if self.min <= self.max {
            context.min_count = self.min;
            context.max_count = self.max;
            
            // Adjust current count if it's outside the new range
            if context.count < self.min {
                context.count = self.min;
            } else if context.count > self.max {
                context.count = self.max;
            }
            
            Ok(())
        } else {
            Err(ActionError::ValidationFailed("Min must be less than or equal to max".to_string()))
        }
    }

    fn description(&self) -> &'static str {
        "Set counter limits"
    }
}

// =============================================================================
// Counter Guards
// =============================================================================

#[derive(Clone, Debug)]
#[allow(dead_code)]
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
#[allow(dead_code)]
struct CanDecrementGuard;

impl Guard<CounterContext, CounterEvent> for CanDecrementGuard {
    fn check(&self, context: &CounterContext, _event: &CounterEvent) -> bool {
        context.count > context.min_count
    }

    fn description(&self) -> &'static str {
        "Check if counter can be decremented"
    }
}

// =============================================================================
// Counter Store
// =============================================================================

/// Counter store that manages the counter state
#[derive(Clone)]
struct CounterStore {
    context: Context<CounterContext>,
}

impl CounterStore {
    fn new() -> Self {
        Self {
            context: Context::new(),
        }
    }

    fn increment(&mut self) -> Result<(), ActionError> {
        let action = IncrementAction;
        self.context.update(|context| {
            action.execute(context).map_err(|e| match e {
                ActionError::ValidationFailed(msg) => ContextError::ValidationFailed(msg),
                ActionError::ExecutionFailed(msg) => ContextError::UpdateFailed(msg),
                ActionError::MissingContext => ContextError::AccessDenied,
                ActionError::InsufficientPermissions => ContextError::AccessDenied,
                ActionError::Timeout => ContextError::UpdateFailed("Action timeout".to_string()),
                ActionError::DependencyNotSatisfied(msg) => ContextError::UpdateFailed(msg),
                ActionError::InvalidStateChange => ContextError::InvalidState(context.clone()),
            })
        }).map_err(|_| ActionError::ExecutionFailed("Failed to update context".to_string()))
    }

    fn decrement(&mut self) -> Result<(), ActionError> {
        let action = DecrementAction;
        self.context.update(|context| {
            action.execute(context).map_err(|e| match e {
                ActionError::ValidationFailed(msg) => ContextError::ValidationFailed(msg),
                ActionError::ExecutionFailed(msg) => ContextError::UpdateFailed(msg),
                ActionError::MissingContext => ContextError::AccessDenied,
                ActionError::InsufficientPermissions => ContextError::AccessDenied,
                ActionError::Timeout => ContextError::UpdateFailed("Action timeout".to_string()),
                ActionError::DependencyNotSatisfied(msg) => ContextError::UpdateFailed(msg),
                ActionError::InvalidStateChange => ContextError::InvalidState(context.clone()),
            })
        }).map_err(|_| ActionError::ExecutionFailed("Failed to update context".to_string()))
    }

    fn reset(&mut self) -> Result<(), ActionError> {
        let action = ResetAction;
        self.context.update(|context| {
            action.execute(context).map_err(|e| match e {
                ActionError::ValidationFailed(msg) => ContextError::ValidationFailed(msg),
                ActionError::ExecutionFailed(msg) => ContextError::UpdateFailed(msg),
                ActionError::MissingContext => ContextError::AccessDenied,
                ActionError::InsufficientPermissions => ContextError::AccessDenied,
                ActionError::Timeout => ContextError::UpdateFailed("Action timeout".to_string()),
                ActionError::DependencyNotSatisfied(msg) => ContextError::UpdateFailed(msg),
                ActionError::InvalidStateChange => ContextError::InvalidState(context.clone()),
            })
        }).map_err(|_| ActionError::ExecutionFailed("Failed to update context".to_string()))
    }

    fn set_value(&mut self, value: i32) -> Result<(), ActionError> {
        let action = SetValueAction(value);
        self.context.update(|context| {
            action.execute(context).map_err(|e| match e {
                ActionError::ValidationFailed(msg) => ContextError::ValidationFailed(msg),
                ActionError::ExecutionFailed(msg) => ContextError::UpdateFailed(msg),
                ActionError::MissingContext => ContextError::AccessDenied,
                ActionError::InsufficientPermissions => ContextError::AccessDenied,
                ActionError::Timeout => ContextError::UpdateFailed("Action timeout".to_string()),
                ActionError::DependencyNotSatisfied(msg) => ContextError::UpdateFailed(msg),
                ActionError::InvalidStateChange => ContextError::InvalidState(context.clone()),
            })
        }).map_err(|_| ActionError::ExecutionFailed("Failed to update context".to_string()))
    }

    fn set_limits(&mut self, min: i32, max: i32) -> Result<(), ActionError> {
        let action = SetLimitsAction { min, max };
        self.context.update(|context| {
            action.execute(context).map_err(|e| match e {
                ActionError::ValidationFailed(msg) => ContextError::ValidationFailed(msg),
                ActionError::ExecutionFailed(msg) => ContextError::UpdateFailed(msg),
                ActionError::MissingContext => ContextError::AccessDenied,
                ActionError::InsufficientPermissions => ContextError::AccessDenied,
                ActionError::Timeout => ContextError::UpdateFailed("Action timeout".to_string()),
                ActionError::DependencyNotSatisfied(msg) => ContextError::UpdateFailed(msg),
                ActionError::InvalidStateChange => ContextError::InvalidState(context.clone()),
            })
        }).map_err(|_| ActionError::ExecutionFailed("Failed to update context".to_string()))
    }

    fn get_count(&self) -> i32 {
        self.context.get().map(|ctx| ctx.count).unwrap_or(0)
    }

    fn get_limits(&self) -> (i32, i32) {
        self.context.get().map(|ctx| (ctx.min_count, ctx.max_count)).unwrap_or((-100, 100))
    }
}

// =============================================================================
// Leptos Component
// =============================================================================

#[component]
fn Counter() -> impl IntoView {
    let (store, set_store) = signal(CounterStore::new());
    let (count, set_count) = signal(0);
    let (min_limit, set_min_limit) = signal(-100);
    let (max_limit, set_max_limit) = signal(100);

    // Update count when store changes
    Effect::new(move |_| {
        let current_count = store.get().get_count();
        set_count.set(current_count);
        
        let (min, max) = store.get().get_limits();
        set_min_limit.set(min);
        set_max_limit.set(max);
    });

    let increment = move |_| {
        let mut new_store = store.get();
        if let Err(e) = new_store.increment() {
            println!("Failed to increment: {:?}", e);
        }
        set_store.set(new_store);
    };

    let decrement = move |_| {
        let mut new_store = store.get();
        if let Err(e) = new_store.decrement() {
            println!("Failed to decrement: {:?}", e);
        }
        set_store.set(new_store);
    };

    let reset = move |_| {
        let mut new_store = store.get();
        if let Err(e) = new_store.reset() {
            println!("Failed to reset: {:?}", e);
        }
        set_store.set(new_store);
    };

    let set_value = move |value: i32| {
        let mut new_store = store.get();
        if let Err(e) = new_store.set_value(value) {
            println!("Failed to set value: {:?}", e);
        }
        set_store.set(new_store);
    };

    let set_limits = move |min: i32, max: i32| {
        let mut new_store = store.get();
        if let Err(e) = new_store.set_limits(min, max) {
            println!("Failed to set limits: {:?}", e);
        }
        set_store.set(new_store);
    };

    view! {
        <div class="counter-container">
            <h2>"Counter Example - v1.0.0"</h2>
            
            <div class="counter-display">
                <h3>"Current Count: " {count}</h3>
                <p>"Limits: [" {min_limit} " to " {max_limit} "]"</p>
            </div>
            
            <div class="counter-controls">
                <button on:click=decrement disabled=move || count.get() <= min_limit.get()>
                    "Decrement"
                </button>
                
                <button on:click=increment disabled=move || count.get() >= max_limit.get()>
                    "Increment"
                </button>
                
                <button on:click=reset>
                    "Reset"
                </button>
            </div>
            
            <div class="counter-inputs">
                <div class="input-group">
                    <label>"Set Value:"</label>
                    <input 
                        type="number" 
                        value=count
                        on:change=move |ev| {
                            let value = event_target_value(&ev).parse::<i32>().unwrap_or(0);
                            set_value(value);
                        }
                    />
                </div>
                
                <div class="input-group">
                    <label>"Min Limit:"</label>
                    <input 
                        type="number" 
                        value=min_limit
                        on:change=move |ev| {
                            let value = event_target_value(&ev).parse::<i32>().unwrap_or(-100);
                            set_limits(value, max_limit.get());
                        }
                    />
                </div>
                
                <div class="input-group">
                    <label>"Max Limit:"</label>
                    <input 
                        type="number" 
                        value=max_limit
                        on:change=move |ev| {
                            let value = event_target_value(&ev).parse::<i32>().unwrap_or(100);
                            set_limits(min_limit.get(), value);
                        }
                    />
                </div>
            </div>
            
            <div class="counter-info">
                <p>"This example demonstrates the new v1.0.0 API with:"</p>
                <ul>
                    <li>"Proper trait implementations"</li>
                    <li>"Context management"</li>
                    <li>"Action and Guard patterns"</li>
                    <li>"Leptos integration"</li>
                </ul>
            </div>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="app">
            <h1>"leptos-state v1.0.0 Demo"</h1>
            <Counter />
        </div>
    }
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
    mount_to_body(|| view! { <App /> });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    mount_to_body(|| view! { <App /> });
}
