use crate::state_machine::*;
use leptos::ev::MouseEvent;
use leptos::prelude::*;

#[component]
pub fn LoadingOverlay() -> impl IntoView {
    view! {
        <div class="loading-overlay">
            <div class="loading-spinner"></div>
            <p>"Loading video..."</p>
        </div>
    }
}

#[component]
pub fn ControlsOverlay(
    store: crate::state_machine::VideoPlayerStore,
    on_play: impl Fn(MouseEvent) + 'static + Copy,
    on_pause: impl Fn(MouseEvent) + 'static + Copy,
    _on_toggle: impl Fn(MouseEvent) + 'static + Copy,
    on_seek: impl Fn(f64) + 'static + Copy,
    on_volume_change: impl Fn(f64) + 'static + Copy,
    on_volume_mute: impl Fn(MouseEvent) + 'static + Copy,
    on_fullscreen: impl Fn(MouseEvent) + 'static + Copy,
) -> impl IntoView {
    let current_time = move || store.context.get().video_current_time;
    let duration = move || store.context.get().video_duration.unwrap_or(1.0);
    let progress = move || {
        let current = current_time();
        let dur = duration();
        if dur > 0.0 {
            current / dur
        } else {
            0.0
        }
    };

    view! {
        <div class="controls-overlay">
            <div class="controls-top">
                <div class="video-title">"Big Buck Bunny"</div>
            </div>

            <div class="controls-center">
                <button
                    class="play-button"
                    on:click=move |ev| {
                        if store.state.get() == crate::state_machine::VideoPlayerState::Playing {
                            on_pause(ev)
                        } else {
                            on_play(ev)
                        }
                    }
                >
                    {move || if store.state.get() == crate::state_machine::VideoPlayerState::Playing { "⏸️" } else { "▶️" }}
                </button>
            </div>

            <div class="controls-bottom">
                <div class="progress-container">
                    <div class="progress-bar">
                        <div
                            class="progress-fill"
                            style=move || format!("width: {}%", progress() * 100.0)
                        ></div>
                        <div
                            class="progress-handle"
                            style=move || format!("left: {}%", progress() * 100.0)
                            on:mousedown=move |_| {
                                // Handle seeking
                                on_seek(progress());
                            }
                        ></div>
                    </div>
                </div>

                <div class="time-display">
                    <span>{move || format_time(current_time())}</span>
                    " / "
                    <span>{move || format_time(duration())}</span>
                </div>

                <div class="volume-controls">
                    <button
                        class="volume-button"
                        on:click=on_volume_mute
                    >
                        {move || if store.context.get().muted { "🔇" } else { "🔊" }}
                    </button>
                    <input
                        type="range"
                        class="volume-slider"
                        min="0"
                        max="1"
                        step="0.1"
                        value=move || store.context.get().volume
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<f64>().unwrap_or(1.0);
                            on_volume_change(value);
                        }
                    />
                </div>

                <button
                    class="fullscreen-button"
                    on:click=on_fullscreen
                >
                    "⛶"
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn AnimationOverlay(animation_type: AnimationType) -> impl IntoView {
    let animation_class = match animation_type {
        AnimationType::Playing => "animate-playing",
        AnimationType::Paused => "animate-paused",
        AnimationType::Backward => "animate-backward",
        AnimationType::Forward => "animate-forward",
    };

    view! {
        <div class=format!("animation-overlay {}", animation_class)>
            <div class="animation-icon">
                {match animation_type {
                    AnimationType::Playing => "▶️",
                    AnimationType::Paused => "⏸️",
                    AnimationType::Backward => "⏪",
                    AnimationType::Forward => "⏩",
                }}
            </div>
        </div>
    }
}

fn format_time(seconds: f64) -> String {
    let minutes = (seconds / 60.0) as u32;
    let seconds = (seconds % 60.0) as u32;
    format!("{:02}:{:02}", minutes, seconds)
}
