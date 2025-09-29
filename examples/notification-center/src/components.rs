use leptos::*;
use leptos::prelude::*;
use crate::notification_machine::*;
use web_sys::AnimationEvent;

/// Individual notification component
#[component]
pub fn NotificationItem<F, G>(
    notification: ReadSignal<Notification>,
    on_close: F,
    on_hover_change: G,
) -> impl IntoView
where
    F: Fn(String) + Send + Clone + 'static,
    G: Fn((String, bool)) + Send + Clone + 'static,
{
    let css_class = Memo::new(move |_| {
        let notification_data = notification.get();
        let base_class = "notification";

        match notification_data.notification_type {
            NotificationType::Success => format!("{} success", base_class),
            NotificationType::Error => format!("{} error", base_class),
            NotificationType::Warning => format!("{} warning", base_class),
            NotificationType::Info => format!("{} info", base_class),
        }
    });

    let progress_animation_duration = Memo::new(move |_| {
        let notification_data = notification.get();
        if let Some(timeout) = notification_data.timeout {
            format!("{}ms", timeout)
        } else {
            "0ms".to_string()
        }
    });

    let should_show_progress = Memo::new(move |_| {
        notification.get().should_show_progress()
    });

    let on_close_clone = on_close.clone();
    let on_hover_change_clone = on_hover_change.clone();

    let handle_close = move |_| {
        let id = notification.get().id.clone();
        on_close_clone(id);
    };

    let on_hover_change_clone2 = on_hover_change.clone();
    let handle_mouse_enter = move |_| {
        let id = notification.get().id.clone();
        on_hover_change_clone((id, true));
    };

    let _on_hover_change_clone3 = on_hover_change.clone();
    let handle_mouse_leave = move |_| {
        let id = notification.get().id.clone();
        on_hover_change_clone2((id, false));
    };

    let on_close_clone2 = on_close.clone();
    let handle_animation_end = move |event: AnimationEvent| {
        // Only handle progress bar animation end
        if event.animation_name() == "countdown" {
            let id = notification.get().id.clone();
            on_close_clone2(id);
        }
    };

    view! {
        <div
            class=move || css_class.get()
            on:mouseenter=handle_mouse_enter
            on:mouseleave=handle_mouse_leave
        >
            <div class="notification-content">
                <div class="notification-title">
                    {move || notification.get().title.clone()}
                </div>
                <div class="notification-description">
                    {move || notification.get().description.clone()}
                </div>
                <button
                    class="notification-close"
                    on:click=handle_close
                    aria-label="Close notification"
                >
                    "×"
                </button>
            </div>
            <div class="progress-bar" style=move || {
                let notification_data = notification.get();
                if notification_data.timeout.is_some() {
                    "height: 4px; background: #e9ecef;"
                } else {
                    "display: none;"
                }
            }>
                <div
                    class="progress-fill"
                    style=move || {
                        let notification_data = notification.get();
                        if notification_data.timeout.is_some() {
                            let duration = progress_animation_duration.get();
                            let play_state = if should_show_progress.get() { "running" } else { "paused" };
                            format!("animation-duration: {}; animation-play-state: {}", duration, play_state)
                        } else {
                            "display: none;".to_string()
                        }
                    }
                    on:animationend=handle_animation_end
                ></div>
            </div>
        </div>
    }
}

/// Notification container component
#[component]
pub fn NotificationContainer<F, G>(
    notifications: ReadSignal<Vec<Notification>>,
    on_close: F,
    on_hover_change: G,
) -> impl IntoView
where
    F: Fn(String) + Send + Clone + 'static,
    G: Fn((String, bool)) + Send + Clone + 'static,
{
    view! {
        <div class="notifications-container">
            <For
                each=move || notifications.get()
                key=|notification| notification.id.clone()
                children=move |notification| {
                    let on_close_clone = on_close.clone();
                    let on_hover_change_clone = on_hover_change.clone();
                    let notification_signal = signal(notification);
                    view! {
                        <NotificationItem
                            notification=notification_signal.0
                            on_close=on_close_clone
                            on_hover_change=on_hover_change_clone
                        />
                    }
                }
            />
        </div>
    }
}

/// Demo form component
#[component]
pub fn NotificationDemo<F>(
    on_trigger: F,
) -> impl IntoView
where
    F: Fn((String, String, Option<u32>, NotificationType)) + Send + Clone + 'static,
{
    let (title, set_title) = signal("Successfully saved!".to_string());
    let (description, set_description) = signal("Your XState machine has been saved 👌".to_string());
    let (timeout, set_timeout) = signal(Some(5000u32));
    let (notification_type, set_notification_type) = signal(NotificationType::Success);

    let handle_trigger = move |_| {
        on_trigger((
            title.get(),
            description.get(),
            timeout.get(),
            notification_type.get(),
        ));
    };

    let handle_timeout_change = move |ev| {
        let value = event_target_value(&ev);
        let timeout_value = match value.as_str() {
            "none" => None,
            "5000" => Some(5000),
            "10000" => Some(10000),
            _ => Some(5000),
        };
        set_timeout.set(timeout_value);
    };

    let handle_type_change = move |ev| {
        let value = event_target_value(&ev);
        let notification_type = match value.as_str() {
            "success" => NotificationType::Success,
            "error" => NotificationType::Error,
            "warning" => NotificationType::Warning,
            "info" => NotificationType::Info,
            _ => NotificationType::Info,
        };
        set_notification_type.set(notification_type);
    };

    view! {
        <div class="demo-section">
            <h2>"Trigger a notification"</h2>

            <div class="form-group">
                <label>"Title"</label>
                <input
                    type="text"
                    prop:value=title
                    on:input=move |ev| set_title.set(event_target_value(&ev))
                />
            </div>

            <div class="form-group">
                <label>"Description"</label>
                <textarea
                    prop:value=description
                    on:input=move |ev| set_description.set(event_target_value(&ev))
                ></textarea>
            </div>

            <div class="form-group">
                <label>"Timeout"</label>
                <select on:change=handle_timeout_change>
                    <option value="none" selected=move || timeout.get().is_none()>"No timeout"</option>
                    <option value="5000" selected=move || timeout.get() == Some(5000)>"5s"</option>
                    <option value="10000" selected=move || timeout.get() == Some(10000)>"10s"</option>
                </select>
            </div>

            <div class="form-group">
                <label>"Notification method"</label>
                <select on:change=handle_type_change>
                    <option value="info" selected=move || matches!(notification_type.get(), NotificationType::Info)>"Info"</option>
                    <option value="success" selected=move || matches!(notification_type.get(), NotificationType::Success)>"Success"</option>
                    <option value="warning" selected=move || matches!(notification_type.get(), NotificationType::Warning)>"Warning"</option>
                    <option value="error" selected=move || matches!(notification_type.get(), NotificationType::Error)>"Error"</option>
                </select>
            </div>

            <div class="button-group">
                <button class="btn btn-primary" on:click=handle_trigger>
                    "Trigger"
                </button>
            </div>
        </div>
    }
}
