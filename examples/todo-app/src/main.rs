use leptos::*;


mod simple_todo;

use simple_todo::SimpleTodoApp;

fn main() {
    // Set up console error handling for better debugging
    console_error_panic_hook::set_once();
    
    // Initialize logging
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logging");
    
    log::info!("Starting Simple Todo App...");
    
    // Mount the app
    leptos::mount::mount_to_body(|| {
        view! {
            <SimpleTodoApp />
        }
    });
}
