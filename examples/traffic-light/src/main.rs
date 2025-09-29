use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq)]
enum TrafficLightState {
    Red,
    Yellow,
    Green,
}

#[component]
fn TrafficLight() -> impl IntoView {
    let (current_state, set_current_state) = signal(TrafficLightState::Red);
    let (pedestrian_waiting, set_pedestrian_waiting) = signal(false);

    let current_light = move || match current_state.get() {
        TrafficLightState::Red => "Red",
        TrafficLightState::Yellow => "Yellow",
        TrafficLightState::Green => "Green",
    };

    let is_red = move || current_state.get() == TrafficLightState::Red;
    let is_yellow = move || current_state.get() == TrafficLightState::Yellow;
    let is_green = move || current_state.get() == TrafficLightState::Green;

    let next_timer = move |_| {
        set_current_state.update(|state| {
            let next_state = match state {
                TrafficLightState::Red => TrafficLightState::Green,
                TrafficLightState::Green => TrafficLightState::Yellow,
                TrafficLightState::Yellow => TrafficLightState::Red,
            };
            *state = next_state;
        });

        // If pedestrians are waiting and we're at red, clear the waiting state
        // (simulating that pedestrians can now cross)
        if current_state.get() == TrafficLightState::Red && pedestrian_waiting.get() {
            set_pedestrian_waiting.set(false);
        }
    };

    let request_pedestrian = move |_| {
        set_pedestrian_waiting.set(true);
    };

    let emergency_stop = move |_| {
        set_current_state.set(TrafficLightState::Red);
        set_pedestrian_waiting.set(false);
    };

    let reset = move |_| {
        set_current_state.set(TrafficLightState::Red);
        set_pedestrian_waiting.set(false);
    };

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
                <p>"Current State: " <strong data-testid="current-state">{current_light}</strong></p>
                <p>"Pedestrian Waiting: "
                    <strong data-testid="pedestrian-waiting">{move || if pedestrian_waiting.get() { "Yes" } else { "No" }}</strong>
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
    leptos::mount::mount_to_body(|| {
        view! {
            <App />
        }
    });
}
