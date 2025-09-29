use leptos::*;
use notification_center::App;

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(|| {
        view! {
            <App />
        }
    });
}
