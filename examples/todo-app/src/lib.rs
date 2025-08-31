use wasm_bindgen::prelude::*;
use leptos::*;
use console_error_panic_hook;

mod simple_todo;
use simple_todo::SimpleTodoApp;

#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    
    mount_to_body(|| {
        view! {
            <SimpleTodoApp />
        }
    });
}
