use leptos::prelude::*;
use wasm_bindgen::prelude::*;

mod components;
mod state_machine;
mod video_player;

use video_player::VideoPlayer;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init().expect("Failed to initialize logger");

    log::info!("Starting video player application");

    // Mount the video player component
    mount_to_body(VideoPlayer);

    log::info!("Mounted VideoPlayer component");
}
