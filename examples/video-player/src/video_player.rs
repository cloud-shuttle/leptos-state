use leptos::prelude::*;
// use web_sys;

use crate::components::*;
use crate::state_machine::*;

#[component]
pub fn VideoPlayer() -> impl IntoView {
    log::info!("VideoPlayer component is rendering");

    // Sample video URL - using Big Buck Bunny as in the XState example
    let video_src =
        "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4";
    let video_poster =
        "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/images/BigBuckBunny.jpg";

    // Create the video player context
    let context = VideoPlayerContext {
        video_src: video_src.to_string(),
        video_poster: video_poster.to_string(),
        current_video_src: Some(video_src.to_string()),
        video_duration: None,
        video_current_time: 0.0,
        volume: 1.0,
        muted: false,
        animation_action_timestamp: String::new(),
        is_touch_device: is_touch_device(),
    };

    // Create the video player store
    let store = VideoPlayerStore::new(context);

    // Video element reference
    let video_ref = NodeRef::<leptos::html::Video>::new();

    // Handle video events
    let handle_video_events = move |event: web_sys::Event| {
        let event_type = event.type_();
        match event_type.as_str() {
            "loadedmetadata" => {
                if let Some(video) = video_ref.get() {
                    let duration = video.duration();
                    store.update_duration(duration);
                }
            }
            "timeupdate" => {
                if let Some(video) = video_ref.get() {
                    let current_time = video.current_time();
                    store.update_time(current_time);
                }
            }
            "waiting" => {
                store.set_loading(true);
            }
            "canplay" => {
                store.set_loading(false);
            }
            "canplaythrough" => {
                store.set_loading(false);
            }
            "ended" => {
                store.stop();
            }
            _ => {}
        }
    };

    // Handle video click
    let handle_video_click = move |_| {
        let current_state = store.state.get();
        match current_state {
            VideoPlayerState::Playing => {
                store.pause();
                store.animate(AnimationType::Paused);
            }
            VideoPlayerState::Paused => {
                store.play();
                store.animate(AnimationType::Playing);
            }
            VideoPlayerState::Stopped => {
                store.play();
            }
            _ => {}
        }
    };

    // Handle mouse events
    let handle_mouse_enter = move |_| {
        if !store.context.get().is_touch_device {
            store.set_show_controls.set(true);
        }
    };

    let handle_mouse_leave = move |_| {
        if !store.context.get().is_touch_device {
            store.set_show_controls.set(false);
        }
    };

    // Handle keyboard events
    let handle_keydown = move |event: web_sys::KeyboardEvent| {
        let key = event.key();
        leptos::logging::log!("Key pressed: {}", key);

        match key.as_str() {
            " " | "k" | "K" => {
                leptos::logging::log!("Play/pause triggered by key: {}", key);
                event.prevent_default();
                let current_state = store.state.get();
                match current_state {
                    VideoPlayerState::Playing => {
                        store.pause();
                        store.animate(AnimationType::Paused);
                    }
                    VideoPlayerState::Paused => {
                        store.play();
                        store.animate(AnimationType::Playing);
                    }
                    _ => {}
                }
            }
            "f" => {
                event.prevent_default();
                store.toggle_fullscreen();
            }
            "ArrowLeft" => {
                event.prevent_default();
                if let Some(video) = video_ref.get() {
                    let current_time = video.current_time();
                    let new_time = (current_time - 10.0).max(0.0);
                    leptos::logging::log!("Skip backward: {} -> {} (diff: {})", current_time, new_time, new_time - current_time);
                    video.set_current_time(new_time);
                    store.animate(AnimationType::Backward);
                }
            }
            "ArrowRight" => {
                event.prevent_default();
                if let Some(video) = video_ref.get() {
                    let current_time = video.current_time();
                    let duration = video.duration();
                    let new_time = (current_time + 10.0).min(duration);
                    leptos::logging::log!("Skip forward: {} -> {} (diff: {})", current_time, new_time, new_time - current_time);
                    video.set_current_time(new_time);
                    store.animate(AnimationType::Forward);
                }
            }
            _ => {}
        }
    };

    // Handle state changes
    Effect::new(move |_| {
        let state = store.state.get();
        if let Some(video) = video_ref.get() {
            match state {
                VideoPlayerState::Playing => {
                    let _ = video.play().unwrap();
                }
                VideoPlayerState::Paused => {
                    video.pause().unwrap();
                }
                _ => {}
            }
        }
    });

    // Auto-hide controls after delay
    Effect::new(move |_| {
        if store.show_controls.get() && !store.context.get().is_touch_device {
            set_timeout(
                move || {
                    store.set_show_controls.set(false);
                },
                std::time::Duration::from_millis(2000),
            );
        }
    });

    // Clear animation after delay
    Effect::new(move |_| {
        if store.animation.get().is_some() {
            set_timeout(
                move || {
                    store.set_animation.set(None);
                },
                std::time::Duration::from_millis(600),
            );
        }
    });

    view! {
        <div class="video-player-container" on:keydown=handle_keydown tabindex="0">
            <div class="video-wrapper" on:mouseenter=handle_mouse_enter on:mouseleave=handle_mouse_leave>
                <video
                    node_ref=video_ref
                    src=move || store.context.get().current_video_src.clone().unwrap_or_default()
                    poster=move || store.context.get().video_poster.clone()
                    controls=false
                    on:click=handle_video_click
                    on:loadedmetadata=handle_video_events
                    on:timeupdate=handle_video_events
                    on:waiting=handle_video_events
                    on:canplay=handle_video_events
                    on:canplaythrough=handle_video_events
                    on:ended=handle_video_events
                    class="video-element"
                />

                // Loading overlay
                {move || {
                    if store.is_loading.get() {
                        Some(view! { <LoadingOverlay /> })
                    } else {
                        None
                    }
                }}

                // Controls overlay
                {move || {
                    let show_controls = store.show_controls.get();
                    let state = store.state.get();
                    let is_touch_device = store.context.get().is_touch_device;

                    if show_controls || state == VideoPlayerState::Paused || is_touch_device {
                        Some(view! {
                            <ControlsOverlay
                                store=store
                                on_play=move |_| { store.play(); }
                                on_pause=move |_| { store.pause(); }
                                _on_toggle=move |_| {
                                    let current_state = store.state.get();
                                    match current_state {
                                        VideoPlayerState::Playing => store.pause(),
                                        VideoPlayerState::Paused => store.play(),
                                        _ => store.play(),
                                    }
                                }
                                on_seek=move |percentage| {
                                    if let Some(video) = video_ref.get() {
                                        let duration = video.duration();
                                        let new_time = duration * percentage / 100.0;
                                        video.set_current_time(new_time);
                                    }
                                }
                                on_volume_change=move |volume| { store.set_volume(volume); }
                                on_volume_mute=move |_| { store.toggle_mute(); }
                                on_fullscreen=move |_| { store.toggle_fullscreen(); }
                            />
                        })
                    } else {
                        None
                    }
                }}

                // Animation overlay
                {move || {
                    store.animation.get().map(|animation| view! {
                        <AnimationOverlay animation_type=animation />
                    })
                }}
            </div>
        </div>
    }
}

fn is_touch_device() -> bool {
    // Simple touch device detection - in a real app you'd use proper media queries
    // For now, we'll assume it's not a touch device
    false
}
