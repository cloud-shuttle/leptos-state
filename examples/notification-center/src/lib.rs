use leptos::*;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

// use console_error_panic_hook;

mod components;
mod notification_machine;

/// Window focus/blur callback logic
pub fn setup_window_focus_listener<F, G>(
    on_focus: F,
    on_blur: G,
) -> Option<()>
where
    F: Fn(()) + 'static,
    G: Fn(()) + 'static,
{
    let window = web_sys::window()?;

    let focus_callback = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
        on_focus(());
    }) as Box<dyn Fn()>);

    let blur_callback = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
        on_blur(());
    }) as Box<dyn Fn()>);

    let _ = window.add_event_listener_with_callback("focus", focus_callback.as_ref().unchecked_ref());
    let _ = window.add_event_listener_with_callback("blur", blur_callback.as_ref().unchecked_ref());

    // Leak the closures to keep them alive
    focus_callback.forget();
    blur_callback.forget();

    Some(())
}

#[component]
pub fn App() -> impl IntoView {
    // Reactive signal for notifications
    let (notifications, set_notifications) = signal(Vec::<notification_machine::Notification>::new());

    // Window blur state
    let (window_blurred, set_window_blurred) = signal(false);

    // Set up window focus/blur listeners
    Effect::new(move |_| {
        setup_window_focus_listener(
            move |_| {
                set_window_blurred.set(false);
                // Update all notifications when window regains focus
                set_notifications.update(|nots| {
                    for notification in nots.iter_mut() {
                        notification.set_window_blurred(false);
                    }
                });
            },
            move |_| {
                set_window_blurred.set(true);
                // Update all notifications when window loses focus
                set_notifications.update(|nots| {
                    for notification in nots.iter_mut() {
                        notification.set_window_blurred(true);
                    }
                });
            },
        );
    });

    // Handle triggering new notifications
    let handle_trigger_notification = move |(title, description, timeout, notification_type): (String, String, Option<u32>, notification_machine::NotificationType)| {
        let mut notification = notification_machine::Notification::new(title, description, timeout, notification_type);
        notification.set_window_blurred(window_blurred.get());

        set_notifications.update(|nots| {
            nots.insert(0, notification); // Add to front for stacking
        });
    };

    // Handle closing notifications
    let handle_close_notification = move |id: String| {
        set_notifications.update(|nots| {
            nots.retain(|n| n.id != id);
        });
    };

    // Handle hover changes
    let handle_hover_change = move |(id, hovered): (String, bool)| {
        set_notifications.update(|nots| {
            if let Some(notification) = nots.iter_mut().find(|n| n.id == id) {
                notification.set_hovered(hovered);
            }
        });
    };

    view! {
        <div class="container">
            <div class="header">
                <h1>"🔔 Notification Center"</h1>
                <p>"Leptos Reactive State Management Demo - Inspired by React Toastify"</p>
            </div>

            <div class="content">
                <components::NotificationDemo on_trigger=handle_trigger_notification />

                <div class="demo-section">
                    <h2>"How it works"</h2>
                    <ul>
                        <li>"🔔 Each notification manages its own hover and window focus state"</li>
                        <li>"⏱️ CSS animations handle timeouts with progress bars"</li>
                        <li>"🎯 Window focus/blur detection pauses/resumes timers"</li>
                        <li>"🎨 Reactive UI updates with Leptos signals"</li>
                        <li>"🔄 Stacked notifications from most recent to oldest"</li>
                    </ul>

                    <p style="margin-top: 15px; color: #666;">
                        "Try hovering over notifications, switching browser tabs, and triggering different types!"
                    </p>
                </div>
            </div>
        </div>

        <components::NotificationContainer
            notifications=notifications
            on_close=handle_close_notification
            on_hover_change=handle_hover_change
        />
    }
}

#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(|| {
        view! {
            <App />
        }
    });
}