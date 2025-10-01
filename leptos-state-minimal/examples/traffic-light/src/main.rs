use leptos::*;
use leptos::prelude::*;
use leptos_state_minimal::{use_machine, Machine, StateNode};

/// Traffic light events
#[derive(Clone)]
enum TrafficEvent {
    Next,
}

/// Traffic light context
#[derive(Clone, Default)]
struct TrafficContext {
    cycle_count: i32,
}

impl TrafficContext {
    fn increment_cycle(&mut self) {
        self.cycle_count += 1;
    }
}

fn create_traffic_light_machine() -> Machine<TrafficContext, TrafficEvent> {
    let mut machine = Machine::new("red", TrafficContext::default());

    // Red state - transitions to green
    let red_state = StateNode::new()
        .on_entry(|ctx: &mut TrafficContext, _| {
            println!("Entering RED state, cycle: {}", ctx.cycle_count);
        })
        .on_exit(|ctx: &mut TrafficContext, _| {
            ctx.increment_cycle();
            println!("Exiting RED state");
        })
        .on(TrafficEvent::Next, "green");

    machine.add_state("red", red_state);

    // Green state - transitions to yellow
    let green_state = StateNode::new()
        .on_entry(|ctx: &mut TrafficContext, _| {
            println!("Entering GREEN state, cycle: {}", ctx.cycle_count);
        })
        .on_exit(|ctx: &mut TrafficContext, _| {
            println!("Exiting GREEN state");
        })
        .on(TrafficEvent::Next, "yellow");

    machine.add_state("green", green_state);

    // Yellow state - transitions back to red
    let yellow_state = StateNode::new()
        .on_entry(|ctx: &mut TrafficContext, _| {
            println!("Entering YELLOW state, cycle: {}", ctx.cycle_count);
        })
        .on_exit(|ctx: &mut TrafficContext, _| {
            println!("Exiting YELLOW state");
        })
        .on(TrafficEvent::Next, "red");

    machine.add_state("yellow", yellow_state);

    machine
}

#[component]
fn TrafficLight() -> impl IntoView {
    // Use the machine hook
    let machine = create_traffic_light_machine();
    let (current_state, actions) = use_machine(machine);

    // Next handler
    let next = move |_| {
        if let Err(e) = actions.send(TrafficEvent::Next) {
            log::error!("Failed to send event: {:?}", e);
        }
    };

    // Get state color and display text
    let state_info = move || {
        match current_state.get().as_str() {
            "red" => ("#ff4444", "🔴 RED"),
            "yellow" => ("#ffaa44", "🟡 YELLOW"),
            "green" => ("#44ff44", "🟢 GREEN"),
            _ => ("#666666", "❓ UNKNOWN"),
        }
    };
    let state_color = move || state_info().0;
    let state_text = move || state_info().1;

    // Get cycle count
    let cycle_count = move || {
        actions.context().cycle_count
    };

    view! {
        <div style="text-align: center; padding: 20px;">
            <h1>"Traffic Light State Machine"</h1>

            <div style="margin: 40px 0;">
                <div style=format!("width: 200px; height: 200px; border-radius: 50%; background-color: {}; margin: 0 auto; display: flex; align-items: center; justify-content: center; font-size: 24px; color: white; text-shadow: 2px 2px 4px rgba(0,0,0,0.5); transition: background-color 0.3s ease;", state_color())>
                    {state_text}
                </div>
            </div>

            <div style="margin: 30px 0;">
                <button
                    on:click=next
                    style="font-size: 20px; padding: 15px 30px; background-color: #007bff; color: white; border: none; border-radius: 8px; cursor: pointer; transition: background-color 0.2s;"
                    on:mouseenter=move |_| {}
                    on:mouseleave=move |_| {}
                >
                    "Next →"
                </button>
            </div>

            <div style="margin: 20px 0; padding: 20px; background-color: #f8f9fa; border-radius: 8px;">
                <h3>"State Machine Info"</h3>
                <p><strong>"Current State:"</strong> {current_state}</p>
                <p><strong>"Cycle Count:"</strong> {cycle_count}</p>
                <p><strong>"Possible Transitions:"</strong> {
                    move || actions.possible_transitions().join(", ")
                }</p>
            </div>

            <div style="margin-top: 40px; padding: 20px; background-color: #e9ecef; border-radius: 8px;">
                <h3>"Leptos State Minimal - State Machine Demo"</h3>
                <p>"This traffic light demonstrates finite state machine functionality with:"</p>
                <ul style="text-align: left; display: inline-block;">
                    <li>"State transitions with entry/exit actions"</li>
                    <li>"Context data persistence across states"</li>
                    <li>"Type-safe event handling"</li>
                    <li>"Reactive UI updates"</li>
                    <li>"Minimal trait bounds (Send + Sync + Clone + 'static)"</li>
                </ul>
            </div>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <TrafficLight />
    }
}

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(App);
}
