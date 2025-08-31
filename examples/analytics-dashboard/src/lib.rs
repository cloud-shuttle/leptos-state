use wasm_bindgen::prelude::*;
use leptos::*;
use console_error_panic_hook;

mod dashboard;
use dashboard::AnalyticsDashboard;

#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    
    mount_to_body(|| {
        view! {
            <AnalyticsDashboard />
        }
    });
}
