use uuid::Uuid;

/// Notification types for styling
#[derive(Clone, Debug, PartialEq)]
pub enum NotificationType {
    Info,
    Success,
    Warning,
    Error,
}

impl Default for NotificationType {
    fn default() -> Self {
        NotificationType::Info
    }
}

/// Individual notification data
#[derive(Clone, Debug, PartialEq)]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub description: String,
    pub timeout: Option<u32>, // milliseconds
    pub notification_type: NotificationType,
    pub is_hovered: bool,
    pub window_blurred: bool,
    pub is_done: bool,
}

impl Notification {
    pub fn new(
        title: String,
        description: String,
        timeout: Option<u32>,
        notification_type: NotificationType,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            timeout,
            notification_type,
            is_hovered: false,
            window_blurred: false,
            is_done: false,
        }
    }

    pub fn should_show_progress(&self) -> bool {
        self.timeout.is_some() && !self.is_hovered && !self.window_blurred && !self.is_done
    }

    pub fn set_hovered(&mut self, hovered: bool) {
        self.is_hovered = hovered;
    }

    pub fn set_window_blurred(&mut self, blurred: bool) {
        self.window_blurred = blurred;
    }
}
