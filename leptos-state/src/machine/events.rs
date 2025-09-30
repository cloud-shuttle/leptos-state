use crate::machine::core_actions::Action;
use std::fmt;

/// Trait for machine events
pub trait Event: Clone + fmt::Debug {
    /// Get the event type as a string
    fn event_type(&self) -> &str;
}

/// Basic string-based event
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringEvent(pub String);

impl Event for StringEvent {
    fn event_type(&self) -> &str {
        &self.0
    }
}

impl From<&str> for StringEvent {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for StringEvent {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// Event with optional data payload
#[derive(Debug, Clone)]
pub struct DataEvent<T> {
    pub event_type: String,
    pub data: Option<T>,
}

impl<T> DataEvent<T> {
    pub fn new(event_type: impl Into<String>) -> Self {
        Self {
            event_type: event_type.into(),
            data: None,
        }
    }

    pub fn with_data(event_type: impl Into<String>, data: T) -> Self {
        Self {
            event_type: event_type.into(),
            data: Some(data),
        }
    }
}

impl<T: Clone + fmt::Debug> Event for DataEvent<T> {
    fn event_type(&self) -> &str {
        &self.event_type
    }
}

// Action trait moved to core_actions.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestContext {
        count: i32,
        message: String,
    }

    #[derive(Debug, Clone, PartialEq)]
    enum TestEvent {
        Increment,
        SetMessage(String),
    }

    impl Event for TestEvent {
        fn event_type(&self) -> &str {
            match self {
                TestEvent::Increment => "increment",
                TestEvent::SetMessage(_) => "set_message",
            }
        }
    }

    #[test]
    fn string_event_creation() {
        let event = StringEvent::from("test_event");
        assert_eq!(event.event_type(), "test_event");
    }

    #[test]
    fn data_event_creation() {
        let event: DataEvent<String> = DataEvent::new("test");
        assert_eq!(event.event_type(), "test");
        assert!(event.data.is_none());

        let event_with_data = DataEvent::with_data("test", 42);
        assert_eq!(event_with_data.event_type(), "test");
        assert_eq!(event_with_data.data, Some(42));
    }

    #[test]
    fn function_action_execution() {
        let action = FunctionAction::new(|ctx: &mut TestContext, _event: &TestEvent| {
            ctx.count += 1;
        });

        let mut context = TestContext {
            count: 0,
            message: "test".to_string(),
        };

        action.execute(&mut context, &TestEvent::Increment);
        assert_eq!(context.count, 1);
    }

    #[test]
    fn log_action_execution() {
        let action = LogAction::new("Test action");
        let mut context = TestContext {
            count: 0,
            message: "test".to_string(),
        };

        // This should not panic
        action.execute(&mut context, &TestEvent::Increment);
    }

    #[test]
    fn pure_action_execution() {
        let called = std::cell::RefCell::new(false);
        let action = PureAction::new(|| {
            *called.borrow_mut() = true;
        });

        let mut context = TestContext {
            count: 0,
            message: "test".to_string(),
        };

        action.execute(&mut context, &TestEvent::Increment);
        assert!(*called.borrow());
    }
}
