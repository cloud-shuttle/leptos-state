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

/// Action trait for side effects during transitions
pub trait Action<C, E> {
    /// Execute the action, potentially modifying the context
    fn execute(&self, context: &mut C, event: &E);
}

/// Function-based action implementation
pub struct FunctionAction<C, E, F> {
    func: F,
    _phantom: std::marker::PhantomData<(C, E)>,
}

impl<C, E, F> FunctionAction<C, E, F>
where
    F: Fn(&mut C, &E),
{
    pub fn new(func: F) -> Self {
        Self {
            func,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<C, E, F> Action<C, E> for FunctionAction<C, E, F>
where
    F: Fn(&mut C, &E),
{
    fn execute(&self, context: &mut C, event: &E) {
        (self.func)(context, event);
    }
}

/// Assign action that updates context fields
pub struct AssignAction<C, E, T, F> {
    field_updater: F,
    _phantom: std::marker::PhantomData<(C, E, T)>,
}

impl<C, E, T, F> AssignAction<C, E, T, F>
where
    F: Fn(&mut C, &E, T),
{
    pub fn new(field_updater: F) -> Self {
        Self {
            field_updater,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<C, E, T, F> Action<C, E> for AssignAction<C, E, T, F>
where
    F: Fn(&mut C, &E, T),
    T: Default,
{
    fn execute(&self, context: &mut C, event: &E) {
        (self.field_updater)(context, event, T::default());
    }
}

/// Log action for debugging
pub struct LogAction {
    message: String,
}

impl LogAction {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl<C, E> Action<C, E> for LogAction
where
    C: fmt::Debug,
    E: fmt::Debug,
{
    fn execute(&self, context: &mut C, event: &E) {
        tracing::info!(
            "{} - Context: {:?}, Event: {:?}",
            self.message,
            context,
            event
        );
    }
}

/// Pure action that doesn't modify context
pub struct PureAction<F> {
    func: F,
}

impl<F> PureAction<F>
where
    F: Fn(),
{
    pub fn new(func: F) -> Self {
        Self { func }
    }
}

impl<C, E, F> Action<C, E> for PureAction<F>
where
    F: Fn(),
{
    fn execute(&self, _context: &mut C, _event: &E) {
        (self.func)();
    }
}

/// Spawn action for async operations (placeholder)
pub struct SpawnAction<F> {
    _future_factory: F,
}

impl<F> SpawnAction<F> {
    pub fn new(future_factory: F) -> Self {
        Self {
            _future_factory: future_factory,
        }
    }
}

impl<C, E, F, Fut> Action<C, E> for SpawnAction<F>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = ()> + 'static,
{
    fn execute(&self, _context: &mut C, _event: &E) {
        // In a real implementation, this would spawn the future
        // For now, it's a placeholder
        tracing::debug!("SpawnAction executed (placeholder)");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    #[derive(Default)]
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
