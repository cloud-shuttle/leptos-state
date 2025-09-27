use leptos::prelude::*;

/// Context for the video player state machine
#[derive(Debug, Clone, PartialEq)]
pub struct VideoPlayerContext {
    pub video_src: String,
    pub video_poster: String,
    pub current_video_src: Option<String>,
    pub video_duration: Option<f64>,
    pub video_current_time: f64,
    pub volume: f64,
    pub muted: bool,
    pub animation_action_timestamp: String,
    pub is_touch_device: bool,
}

impl Default for VideoPlayerContext {
    fn default() -> Self {
        Self {
            video_src: String::new(),
            video_poster: String::new(),
            current_video_src: None,
            video_duration: None,
            video_current_time: 0.0,
            volume: 1.0,
            muted: false,
            animation_action_timestamp: String::new(),
            is_touch_device: false,
        }
    }
}

/// Video player state using Leptos signals
#[derive(Debug, Clone, PartialEq)]
pub enum VideoPlayerState {
    Stopped,
    Loading,
    Playing,
    Paused,
    Ended,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnimationType {
    Playing,
    Paused,
    Backward,
    Forward,
}

/// Video player store using Leptos signals
#[derive(Clone, Copy)]
pub struct VideoPlayerStore {
    pub state: ReadSignal<VideoPlayerState>,
    pub set_state: WriteSignal<VideoPlayerState>,
    pub context: ReadSignal<VideoPlayerContext>,
    pub set_context: WriteSignal<VideoPlayerContext>,
    pub is_loading: ReadSignal<bool>,
    pub set_loading: WriteSignal<bool>,
    pub show_controls: ReadSignal<bool>,
    pub set_show_controls: WriteSignal<bool>,
    pub animation: ReadSignal<Option<AnimationType>>,
    pub set_animation: WriteSignal<Option<AnimationType>>,
    pub is_fullscreen: ReadSignal<bool>,
    pub set_fullscreen: WriteSignal<bool>,
}

impl VideoPlayerStore {
    pub fn new(initial_context: VideoPlayerContext) -> Self {
        let (state, set_state) = signal(VideoPlayerState::Stopped);
        let (context, set_context) = signal(initial_context);
        let (is_loading, set_loading) = signal(false);
        let (show_controls, set_show_controls) = signal(false);
        let (animation, set_animation) = signal(None);
        let (is_fullscreen, set_fullscreen) = signal(false);
        
        Self {
            state,
            set_state,
            context,
            set_context,
            is_loading,
            set_loading,
            show_controls,
            set_show_controls,
            animation,
            set_animation,
            is_fullscreen,
            set_fullscreen,
        }
    }
    
    pub fn play(&self) {
        self.set_state.set(VideoPlayerState::Playing);
        self.set_loading.set(false);
    }
    
    pub fn pause(&self) {
        self.set_state.set(VideoPlayerState::Paused);
    }
    
    pub fn stop(&self) {
        self.set_state.set(VideoPlayerState::Stopped);
    }
    
    pub fn set_loading(&self, loading: bool) {
        self.set_loading.set(loading);
        if loading {
            self.set_state.set(VideoPlayerState::Loading);
        }
    }
    
    pub fn update_time(&self, current_time: f64) {
        self.set_context.update(|ctx| {
            ctx.video_current_time = current_time;
        });
    }
    
    pub fn update_duration(&self, duration: f64) {
        self.set_context.update(|ctx| {
            ctx.video_duration = Some(duration);
        });
    }
    
    pub fn set_volume(&self, volume: f64) {
        self.set_context.update(|ctx| {
            ctx.volume = volume;
        });
    }
    
    pub fn toggle_mute(&self) {
        self.set_context.update(|ctx| {
            ctx.muted = !ctx.muted;
        });
    }
    
    pub fn animate(&self, animation: AnimationType) {
        self.set_animation.set(Some(animation));
        // Clear animation after a delay
        set_timeout(
            move || {
                // Animation will be cleared by the component
            },
            std::time::Duration::from_millis(600),
        );
    }
    
    pub fn toggle_fullscreen(&self) {
        self.set_fullscreen.set(!self.is_fullscreen.get());
    }
}
