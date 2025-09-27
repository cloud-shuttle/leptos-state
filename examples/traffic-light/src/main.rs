use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild, Get, OnAttribute};
use leptos::*;
use leptos_state::machine::states::StateValue;
use leptos_state::*;

#[derive(Debug, Clone, PartialEq, Default)]
struct TrafficContext {
    timer: i32,
    pedestrian_waiting: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum TrafficEvent {
    Timer,
    PedestrianRequest,
    EmergencyStop,
    Reset,
}

impl Event for TrafficEvent {
    fn event_type(&self) -> &str {
        match self {
            TrafficEvent::Timer => "timer",
            TrafficEvent::PedestrianRequest => "pedestrian_request",
            TrafficEvent::EmergencyStop => "emergency_stop",
            TrafficEvent::Reset => "reset",
        }
    }
}

// For this example, we'll create a simplified machine implementation
#[derive(Debug, Clone, PartialEq)]
struct TrafficMachineState {
    current: StateValue,
    context: TrafficContext,
}

impl MachineState for TrafficMachineState {
    type Context = TrafficContext;

    fn value(&self) -> &StateValue {
        &self.current
    }

    fn context(&self) -> &Self::Context {
        &self.context
    }

    fn matches(&self, pattern: &str) -> bool {
        self.current.matches(pattern)
    }

    fn can_transition_to(&self, _target: &str) -> bool {
        true
    }
}

#[derive(Clone)]
struct TrafficMachine;

impl StateMachine for TrafficMachine {
    type Context = TrafficContext;
    type Event = TrafficEvent;
    type State = TrafficMachineState;

    fn initial() -> Self::State {
        TrafficMachineState {
            current: StateValue::simple("red"),
            context: TrafficContext::default(),
        }
    }

    fn transition(state: &Self::State, event: Self::Event) -> Self::State {
        let mut new_state = state.clone();

        match (&state.current, &event) {
            (current, TrafficEvent::Timer) => {
                new_state.current = match current {
                    StateValue::Simple(s) if s == "red" => StateValue::simple("green"),
                    StateValue::Simple(s) if s == "green" => StateValue::simple("yellow"),
                    StateValue::Simple(s) if s == "yellow" => StateValue::simple("red"),
                    _ => current.clone(),
                };
            }
            (_, TrafficEvent::PedestrianRequest) => {
                new_state.context.pedestrian_waiting = true;
            }
            (_, TrafficEvent::EmergencyStop) => {
                new_state.current = StateValue::simple("red");
                new_state.context.pedestrian_waiting = false;
            }
            (_, TrafficEvent::Reset) => {
                new_state = Self::initial();
            }
        }

        new_state
    }
}

#[component]
fn TrafficLight() -> impl IntoView {
    let machine = use_machine::<TrafficMachine>();

    let machine_current = machine.clone();
           let current_light = move || {
               let state = machine_current.current();
               match state {
                   StateValue::Simple(s) => {
                       match s.as_str() {
                           "red" => "Red".to_string(),
                           "yellow" => "Yellow".to_string(),
                           "green" => "Green".to_string(),
                           _ => s.to_string(),
                       }
                   },
                   _ => "unknown".to_string(),
               }
           };

    let is_red = machine.create_matcher("red".to_string());
    let is_yellow = machine.create_matcher("yellow".to_string());
    let is_green = machine.create_matcher("green".to_string());

    let machine_timer = machine.clone();
    let next_timer = move |_| machine_timer.emit(TrafficEvent::Timer);
    let machine_pedestrian = machine.clone();
    let request_pedestrian = move |_| machine_pedestrian.emit(TrafficEvent::PedestrianRequest);
    let machine_emergency = machine.clone();
    let emergency_stop = move |_| machine_emergency.emit(TrafficEvent::EmergencyStop);
    let machine_reset = machine.clone();
    let reset = move |_| machine_reset.emit(TrafficEvent::Reset);

    view! {
        <div class="traffic-light">
            <h1>"Traffic Light State Machine"</h1>

            <div class="light-display">
                <div
                    class="light red"
                    class:active=move || is_red.get()
                ></div>
                <div
                    class="light yellow"
                    class:active=move || is_yellow.get()
                ></div>
                <div
                    class="light green"
                    class:active=move || is_green.get()
                ></div>
            </div>

            <div class="status">
                <p>"Current State: " <strong data-testid="current-state">{current_light}</strong></p>
                <p>"Pedestrian Waiting: "
                    <strong data-testid="pedestrian-waiting">{move || if machine.get_context().pedestrian_waiting { "Yes" } else { "No" }}</strong>
                </p>
            </div>

            <div class="controls">
                <button data-testid="timer" on:click=next_timer>"Next (Timer)"</button>
                <button data-testid="pedestrian" on:click=request_pedestrian>"Pedestrian Request"</button>
                <button data-testid="emergency" on:click=emergency_stop>"Emergency Stop"</button>
                <button data-testid="reset" on:click=reset>"Reset"</button>
            </div>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="app">
            <TrafficLight />
            <style>
                "
                .traffic-light {
                    max-width: 400px;
                    margin: 2rem auto;
                    text-align: center;
                    font-family: Arial, sans-serif;
                }
                
                .light-display {
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    gap: 10px;
                    margin: 2rem 0;
                    padding: 20px;
                    background: #333;
                    border-radius: 20px;
                    width: 100px;
                    margin: 2rem auto;
                }
                
                .light {
                    width: 60px;
                    height: 60px;
                    border-radius: 50%;
                    opacity: 0.3;
                    transition: opacity 0.3s ease;
                }
                
                .light.active {
                    opacity: 1;
                }
                
                .light.red {
                    background: red;
                }
                
                .light.yellow {
                    background: yellow;
                }
                
                .light.green {
                    background: green;
                }
                
                .status {
                    margin: 2rem 0;
                }
                
                .controls {
                    display: flex;
                    flex-wrap: wrap;
                    gap: 10px;
                    justify-content: center;
                }
                
                button {
                    padding: 10px 15px;
                    border: none;
                    border-radius: 5px;
                    background: #007bff;
                    color: white;
                    cursor: pointer;
                    transition: background 0.2s;
                }
                
                button:hover {
                    background: #0056b3;
                }
                "
            </style>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
