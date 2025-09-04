use leptos::*;
use leptos::prelude::{ElementChild, ClassAttribute, OnAttribute};
use leptos_state::v1::*;
use leptos_state::use_machine_with_context;

#[derive(Debug, Clone, PartialEq, Default)]
struct TrafficContext {
    timer: i32,
    pedestrian_waiting: bool,
}

impl StateMachineContext for TrafficContext {}

#[derive(Debug, Clone, PartialEq, Default)]
enum TrafficEvent {
    #[default]
    Timer,
    EmergencyStop,
}

impl StateMachineEvent for TrafficEvent {}

#[derive(Debug, Clone, PartialEq)]
enum TrafficState {
    Red,
    Yellow,
    Green,
}

impl StateMachineState for TrafficState {
    type Context = TrafficContext;
    type Event = TrafficEvent;
}

impl Default for TrafficState {
    fn default() -> Self {
        TrafficState::Red
    }
}

impl StateMachine for TrafficState {
    fn initial_state(&self) -> Self {
        TrafficState::Red
    }

    fn transition(&self, state: &Self, event: TrafficEvent) -> Self {
        match (state, event) {
            (TrafficState::Red, TrafficEvent::Timer) => TrafficState::Green,
            (TrafficState::Green, TrafficEvent::Timer) => TrafficState::Yellow,
            (TrafficState::Yellow, TrafficEvent::Timer) => TrafficState::Red,
            (_, TrafficEvent::EmergencyStop) => TrafficState::Red,
        }
    }

    fn can_transition(&self, state: &Self, event: TrafficEvent) -> bool {
        match (state, event) {
            (TrafficState::Red, TrafficEvent::Timer) => true,
            (TrafficState::Green, TrafficEvent::Timer) => true,
            (TrafficState::Yellow, TrafficEvent::Timer) => true,
            (_, TrafficEvent::EmergencyStop) => true,
        }
    }

    fn try_transition(&self, state: &Self, event: TrafficEvent) -> Result<Self, TransitionError<TrafficEvent>> {
        if self.can_transition(state, event.clone()) {
            Ok(self.transition(state, event))
        } else {
            Err(TransitionError::InvalidTransition(event))
        }
    }

    fn state_count(&self) -> usize {
        3
    }

    fn is_valid_state(&self, state: &Self) -> bool {
        matches!(state, TrafficState::Red | TrafficState::Yellow | TrafficState::Green)
    }

    fn is_reachable(&self, state: &Self) -> bool {
        self.is_valid_state(state)
    }
}

#[component]
fn TrafficLight() -> impl IntoView {
    let initial_context = TrafficContext::default();
    let machine = use_machine_with_context(TrafficState::Red, initial_context);

    let machine_clone1 = machine.clone();
    let current_light = move || {
        let state = machine_clone1.state();
        match state {
            TrafficState::Red => "red",
            TrafficState::Yellow => "yellow",
            TrafficState::Green => "green",
        }
    };

    let machine_clone2 = machine.clone();
    let is_red = move || machine_clone2.state() == TrafficState::Red;
    let machine_clone3 = machine.clone();
    let is_yellow = move || machine_clone3.state() == TrafficState::Yellow;
    let machine_clone4 = machine.clone();
    let is_green = move || machine_clone4.state() == TrafficState::Green;

    let machine_clone5 = machine.clone();
    let next_timer = move |_| machine_clone5.send(TrafficEvent::Timer);
    let machine_clone6 = machine.clone();
    let emergency_stop = move |_| machine_clone6.send(TrafficEvent::EmergencyStop);

    view! {
        <div class="traffic-light">
            <h1>"Traffic Light State Machine"</h1>

            <div class="light-display">
                <div
                    class="light red"
                    class:active=is_red
                ></div>
                <div
                    class="light yellow"
                    class:active=is_yellow
                ></div>
                <div
                    class="light green"
                    class:active=is_green
                ></div>
            </div>

            <div class="status">
                <p>"Current State: " <strong>{current_light}</strong></p>
                <p>"Pedestrian Waiting: "
                    <strong>{move || if machine.context().pedestrian_waiting { "Yes" } else { "No" }}</strong>
                </p>
            </div>

            <div class="controls">
                <button on:click=next_timer>"Next (Timer)"</button>
                <button on:click=emergency_stop>"Emergency Stop"</button>
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
    leptos::mount::mount_to_body(App);
}
