//! Event priority levels and management

/// Event priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum EventPriority {
    /// Lowest priority
    Lowest = 0,
    /// Low priority
    Low = 1,
    /// Normal priority
    Normal = 2,
    /// High priority
    High = 3,
    /// Critical priority (highest)
    Critical = 4,
}

impl EventPriority {
    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            EventPriority::Lowest => "lowest",
            EventPriority::Low => "low",
            EventPriority::Normal => "normal",
            EventPriority::High => "high",
            EventPriority::Critical => "critical",
        }
    }

    /// Get numeric value
    pub fn value(&self) -> i32 {
        *self as i32
    }

    /// Check if priority is above normal
    pub fn is_above_normal(&self) -> bool {
        self.value() > EventPriority::Normal.value()
    }

    /// Check if priority is below normal
    pub fn is_below_normal(&self) -> bool {
        self.value() < EventPriority::Normal.value()
    }

    /// Check if priority is urgent (high or critical)
    pub fn is_urgent(&self) -> bool {
        matches!(self, EventPriority::High | EventPriority::Critical)
    }

    /// Get the next higher priority
    pub fn escalate(&self) -> Self {
        match self {
            EventPriority::Lowest => EventPriority::Low,
            EventPriority::Low => EventPriority::Normal,
            EventPriority::Normal => EventPriority::High,
            EventPriority::High => EventPriority::Critical,
            EventPriority::Critical => EventPriority::Critical,
        }
    }

    /// Get the next lower priority
    pub fn deescalate(&self) -> Self {
        match self {
            EventPriority::Lowest => EventPriority::Lowest,
            EventPriority::Low => EventPriority::Lowest,
            EventPriority::Normal => EventPriority::Low,
            EventPriority::High => EventPriority::Normal,
            EventPriority::Critical => EventPriority::High,
        }
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "lowest" => Some(EventPriority::Lowest),
            "low" => Some(EventPriority::Low),
            "normal" => Some(EventPriority::Normal),
            "high" => Some(EventPriority::High),
            "critical" => Some(EventPriority::Critical),
            _ => None,
        }
    }

    /// Get all priorities in order
    pub fn all() -> Vec<Self> {
        vec![
            EventPriority::Lowest,
            EventPriority::Low,
            EventPriority::Normal,
            EventPriority::High,
            EventPriority::Critical,
        ]
    }

    /// Get priorities above a threshold
    pub fn above(threshold: Self) -> Vec<Self> {
        Self::all().into_iter().filter(|p| *p > threshold).collect()
    }

    /// Get priorities below a threshold
    pub fn below(threshold: Self) -> Vec<Self> {
        Self::all().into_iter().filter(|p| *p < threshold).collect()
    }

    /// Get default priority
    pub fn default() -> Self {
        EventPriority::Normal
    }
}

impl Default for EventPriority {
    fn default() -> Self {
        Self::default()
    }
}

impl std::fmt::Display for EventPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for EventPriority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s).ok_or_else(|| format!("Invalid priority: {}", s))
    }
}

impl From<i32> for EventPriority {
    fn from(value: i32) -> Self {
        match value {
            0 => EventPriority::Lowest,
            1 => EventPriority::Low,
            2 => EventPriority::Normal,
            3 => EventPriority::High,
            4 => EventPriority::Critical,
            _ if value < 0 => EventPriority::Lowest,
            _ => EventPriority::Critical,
        }
    }
}

impl From<EventPriority> for i32 {
    fn from(priority: EventPriority) -> Self {
        priority.value()
    }
}
