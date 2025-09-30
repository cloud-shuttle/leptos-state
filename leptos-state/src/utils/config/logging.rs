//! Log levels and logging configuration

use std::fmt;

/// Log levels for debugging
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum LogLevel {
    /// No logging
    Off,
    /// Error messages only
    Error,
    /// Warning and error messages
    Warn,
    /// Info, warning, and error messages
    Info,
    /// Debug, info, warning, and error messages
    Debug,
    /// All messages including trace
    Trace,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Off => write!(f, "off"),
            Self::Error => write!(f, "error"),
            Self::Warn => write!(f, "warn"),
            Self::Info => write!(f, "info"),
            Self::Debug => write!(f, "debug"),
            Self::Trace => write!(f, "trace"),
        }
    }
}

impl LogLevel {
    /// Check if this log level includes the given level
    pub fn includes(&self, other: LogLevel) -> bool {
        *self >= other
    }

    /// Check if this log level should log errors
    pub fn should_log_error(&self) -> bool {
        self.includes(LogLevel::Error)
    }

    /// Check if this log level should log warnings
    pub fn should_log_warn(&self) -> bool {
        self.includes(LogLevel::Warn)
    }

    /// Check if this log level should log info
    pub fn should_log_info(&self) -> bool {
        self.includes(LogLevel::Info)
    }

    /// Check if this log level should log debug messages
    pub fn should_log_debug(&self) -> bool {
        self.includes(LogLevel::Debug)
    }

    /// Check if this log level should log trace messages
    pub fn should_log_trace(&self) -> bool {
        self.includes(LogLevel::Trace)
    }

    /// Get the log level as an uppercase string
    pub fn as_uppercase(&self) -> &'static str {
        match self {
            Self::Off => "OFF",
            Self::Error => "ERROR",
            Self::Warn => "WARN",
            Self::Info => "INFO",
            Self::Debug => "DEBUG",
            Self::Trace => "TRACE",
        }
    }

    /// Get the log level as a colored string (for terminals)
    pub fn as_colored(&self) -> String {
        match self {
            Self::Off => format!("\x1b[90m{}\x1b[0m", self.as_uppercase()), // Gray
            Self::Error => format!("\x1b[91m{}\x1b[0m", self.as_uppercase()), // Red
            Self::Warn => format!("\x1b[93m{}\x1b[0m", self.as_uppercase()), // Yellow
            Self::Info => format!("\x1b[94m{}\x1b[0m", self.as_uppercase()), // Blue
            Self::Debug => format!("\x1b[95m{}\x1b[0m", self.as_uppercase()), // Magenta
            Self::Trace => format!("\x1b[96m{}\x1b[0m", self.as_uppercase()), // Cyan
        }
    }

    /// Get all log levels
    pub fn all() -> Vec<Self> {
        vec![
            Self::Off,
            Self::Error,
            Self::Warn,
            Self::Info,
            Self::Debug,
            Self::Trace,
        ]
    }

    /// Parse from string (case-insensitive)
    pub fn from_str_ignore_case(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "off" => Some(Self::Off),
            "error" => Some(Self::Error),
            "warn" => Some(Self::Warn),
            "info" => Some(Self::Info),
            "debug" => Some(Self::Debug),
            "trace" => Some(Self::Trace),
            _ => None,
        }
    }

    /// Get the numeric value of the log level
    pub fn as_usize(&self) -> usize {
        match self {
            Self::Off => 0,
            Self::Error => 1,
            Self::Warn => 2,
            Self::Info => 3,
            Self::Debug => 4,
            Self::Trace => 5,
        }
    }

    /// Create from numeric value
    pub fn from_usize(value: usize) -> Option<Self> {
        match value {
            0 => Some(Self::Off),
            1 => Some(Self::Error),
            2 => Some(Self::Warn),
            3 => Some(Self::Info),
            4 => Some(Self::Debug),
            5 => Some(Self::Trace),
            _ => None,
        }
    }

    /// Check if this is the most verbose level
    pub fn is_most_verbose(&self) -> bool {
        matches!(self, Self::Trace)
    }

    /// Check if this is the least verbose level
    pub fn is_least_verbose(&self) -> bool {
        matches!(self, Self::Off)
    }

    /// Get the next more verbose level
    pub fn more_verbose(&self) -> Option<Self> {
        Self::from_usize(self.as_usize() + 1)
    }

    /// Get the next less verbose level
    pub fn less_verbose(&self) -> Option<Self> {
        if self.as_usize() > 0 {
            Self::from_usize(self.as_usize() - 1)
        } else {
            None
        }
    }

    /// Get the emoji representation
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Off => "🔇",
            Self::Error => "🚨",
            Self::Warn => "⚠️",
            Self::Info => "ℹ️",
            Self::Debug => "🐛",
            Self::Trace => "🔍",
        }
    }
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

impl std::str::FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_ignore_case(s)
            .ok_or_else(|| format!("Invalid log level: {}", s))
    }
}

/// Logger configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoggerConfig {
    /// Minimum log level
    pub level: LogLevel,
    /// Enable timestamps
    pub timestamps: bool,
    /// Enable colored output
    pub colored: bool,
    /// Include thread IDs
    pub thread_ids: bool,
    /// Include file and line information
    pub file_info: bool,
    /// Maximum message length (0 = unlimited)
    pub max_message_length: usize,
    /// Custom format string
    pub format: Option<String>,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            timestamps: true,
            colored: true,
            thread_ids: false,
            file_info: false,
            max_message_length: 0,
            format: None,
        }
    }
}

impl LoggerConfig {
    /// Create a new logger configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the minimum log level
    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    /// Enable or disable timestamps
    pub fn with_timestamps(mut self, timestamps: bool) -> Self {
        self.timestamps = timestamps;
        self
    }

    /// Enable or disable colored output
    pub fn with_colored(mut self, colored: bool) -> Self {
        self.colored = colored;
        self
    }

    /// Enable or disable thread IDs
    pub fn with_thread_ids(mut self, thread_ids: bool) -> Self {
        self.thread_ids = thread_ids;
        self
    }

    /// Enable or disable file information
    pub fn with_file_info(mut self, file_info: bool) -> Self {
        self.file_info = file_info;
        self
    }

    /// Set maximum message length
    pub fn with_max_message_length(mut self, length: usize) -> Self {
        self.max_message_length = length;
        self
    }

    /// Set custom format string
    pub fn with_format<S: Into<String>>(mut self, format: S) -> Self {
        self.format = Some(format.into());
        self
    }

    /// Create a development logger configuration
    pub fn development() -> Self {
        Self::new()
            .with_level(LogLevel::Debug)
            .with_timestamps(true)
            .with_colored(true)
            .with_thread_ids(true)
            .with_file_info(true)
    }

    /// Create a production logger configuration
    pub fn production() -> Self {
        Self::new()
            .with_level(LogLevel::Warn)
            .with_timestamps(true)
            .with_colored(false)
            .with_thread_ids(false)
            .with_file_info(false)
    }

    /// Create a test logger configuration
    pub fn test() -> Self {
        Self::new()
            .with_level(LogLevel::Debug)
            .with_timestamps(false)
            .with_colored(false)
            .with_thread_ids(false)
            .with_file_info(false)
    }

    /// Format a log message
    pub fn format_message(&self, level: LogLevel, message: &str, file: Option<&str>, line: Option<u32>) -> String {
        let mut formatted = String::new();

        // Add timestamp if enabled
        if self.timestamps {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default();
            formatted.push_str(&format!("[{}] ", now.as_secs()));
        }

        // Add thread ID if enabled
        if self.thread_ids {
            formatted.push_str(&format!("[{:?}] ", std::thread::current().id()));
        }

        // Add level
        if self.colored {
            formatted.push_str(&level.as_colored());
        } else {
            formatted.push_str(level.as_uppercase());
        }
        formatted.push_str(": ");

        // Add file and line info if enabled
        if self.file_info {
            if let (Some(file), Some(line)) = (file, line) {
                formatted.push_str(&format!("{}:{}: ", file, line));
            }
        }

        // Add message
        let message = if self.max_message_length > 0 && message.len() > self.max_message_length {
            format!("{}...", &message[..self.max_message_length.saturating_sub(3)])
        } else {
            message.to_string()
        };
        formatted.push_str(&message);

        formatted
    }

    /// Check if a message should be logged at the given level
    pub fn should_log(&self, level: LogLevel) -> bool {
        self.level.includes(level)
    }
}

impl std::fmt::Display for LoggerConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LoggerConfig(level: {}, timestamps: {}, colored: {}, thread_ids: {}, file_info: {})",
            self.level, self.timestamps, self.colored, self.thread_ids, self.file_info
        )
    }
}
