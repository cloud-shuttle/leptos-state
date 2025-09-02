use super::Store;
use std::marker::PhantomData;

/// Trait for store middleware
pub trait Middleware<S: Store> {
    /// Wrap the next middleware/store operation
    fn wrap(&self, next: Box<dyn Fn(&S::State) -> S::State>) -> Box<dyn Fn(&S::State) -> S::State>;
}

/// Chain multiple middleware together
pub struct MiddlewareChain<S: Store> {
    middlewares: Vec<Box<dyn Middleware<S>>>,
}

impl<S: Store> MiddlewareChain<S> {
    pub fn new() -> Self {
        Self {
            middlewares: Vec::new(),
        }
    }

    pub fn add(mut self, middleware: Box<dyn Middleware<S>>) -> Self {
        self.middlewares.push(middleware);
        self
    }

    pub fn apply(
        &self,
        base: impl Fn(&S::State) -> S::State + 'static,
    ) -> impl Fn(&S::State) -> S::State {
        let mut result: Box<dyn Fn(&S::State) -> S::State> = Box::new(base);

        for middleware in self.middlewares.iter().rev() {
            result = middleware.wrap(result);
        }

        move |state| result(state)
    }
}

impl<S: Store> Default for MiddlewareChain<S> {
    fn default() -> Self {
        Self::new()
    }
}

/// Logger middleware for debugging state changes
pub struct LoggerMiddleware<S: Store> {
    prefix: String,
    _phantom: PhantomData<S>,
}

impl<S: Store> LoggerMiddleware<S> {
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            _phantom: PhantomData,
        }
    }
}

impl<S: Store> Middleware<S> for LoggerMiddleware<S>
where
    S::State: std::fmt::Debug,
{
    fn wrap(&self, next: Box<dyn Fn(&S::State) -> S::State>) -> Box<dyn Fn(&S::State) -> S::State> {
        let prefix = self.prefix.clone();
        Box::new(move |state| {
            tracing::debug!("{}: Previous state: {:?}", prefix, state);
            let new_state = next(state);
            tracing::debug!("{}: New state: {:?}", prefix, new_state);
            new_state
        })
    }
}

/// Persistence middleware for storing state in localStorage
#[cfg(feature = "persist")]
pub struct PersistMiddleware<S: Store> {
    key: String,
    _phantom: PhantomData<S>,
}

#[cfg(feature = "persist")]
impl<S: Store> PersistMiddleware<S> {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            _phantom: PhantomData,
        }
    }
}

#[cfg(feature = "persist")]
impl<S: Store> Middleware<S> for PersistMiddleware<S>
where
    S::State: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    fn wrap(&self, next: Box<dyn Fn(&S::State) -> S::State>) -> Box<dyn Fn(&S::State) -> S::State> {
        let key = self.key.clone();
        Box::new(move |state| {
            let new_state = next(state);

            // Save to localStorage using the utility function
            if let Err(e) = crate::store::save_to_storage(&key, &new_state) {
                tracing::warn!("Failed to persist state: {:?}", e);
            }

            new_state
        })
    }
}

/// Validation middleware to ensure state integrity
pub struct ValidationMiddleware<S: Store, F> {
    validator: F,
    _phantom: PhantomData<S>,
}

impl<S: Store, F> ValidationMiddleware<S, F>
where
    F: Fn(&S::State) -> bool,
{
    pub fn new(validator: F) -> Self {
        Self {
            validator,
            _phantom: PhantomData,
        }
    }
}

impl<S: Store, F> Middleware<S> for ValidationMiddleware<S, F>
where
    F: Fn(&S::State) -> bool + Clone + 'static,
{
    fn wrap(&self, next: Box<dyn Fn(&S::State) -> S::State>) -> Box<dyn Fn(&S::State) -> S::State> {
        let validator = self.validator.clone();
        Box::new(move |state| {
            let new_state = next(state);

            if validator(&new_state) {
                new_state
            } else {
                tracing::warn!("State validation failed, keeping previous state");
                state.clone()
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create_store;

    #[derive(Clone, PartialEq, Debug)]
    pub struct TestState {
        count: i32,
    }

    create_store!(TestStore, TestState, TestState { count: 0 });

    #[test]
    fn middleware_chain_applies_in_reverse_order() {
        let chain = MiddlewareChain::<TestStore>::new()
            .add(Box::new(LoggerMiddleware::new("first")))
            .add(Box::new(LoggerMiddleware::new("second")));

        // Test that the chain can be created and applied
        let wrapped = chain.apply(|state| TestState {
            count: state.count + 1,
        });
        let result = wrapped(&TestState { count: 0 });
        assert_eq!(result.count, 1);
    }

    #[test]
    fn validation_middleware_prevents_invalid_states() {
        let validator: ValidationMiddleware<TestStore, _> =
            ValidationMiddleware::new(|state: &TestState| state.count >= 0);

        let wrapped = validator.wrap(Box::new(|_| TestState { count: -1 }));
        let result: TestState = wrapped(&TestState { count: 5 });

        // Should keep the original state since -1 is invalid
        assert_eq!(result.count, 5);
    }
}
