//! Simple store implementation

use super::*;

/// A simple store implementation that stores the initial value
pub struct SimpleStore<T> {
    /// The current state
    pub value: std::rc::Rc<std::cell::RefCell<T>>,
}

impl<T> SimpleStore<T> {
    /// Create a new simple store with an initial value
    pub fn new(initial: T) -> Self {
        Self {
            value: std::rc::Rc::new(std::cell::RefCell::new(initial)),
        }
    }
}

impl<T> Clone for SimpleStore<T> {
    fn clone(&self) -> Self {
        Self {
            value: std::rc::Rc::clone(&self.value),
        }
    }
}

impl<T> Store for SimpleStore<T>
where
    T: Clone + PartialEq + 'static,
{
    type State = T;

    fn get(&self) -> Self::State {
        self.value.borrow().clone()
    }

    fn set(&self, state: Self::State) {
        *self.value.borrow_mut() = state;
    }

    fn update<F>(&self, f: F)
    where
        F: FnOnce(Self::State) -> Self::State,
    {
        let current = self.get();
        let new_state = f(current);
        self.set(new_state);
    }
}

/// Macro for creating store implementations
#[macro_export]
macro_rules! create_store_type {
    ($name:ident, $state_type:ty, $initial:expr) => {
        #[derive(Clone)]
        pub struct $name {
            store: $crate::store::SimpleStore<$state_type>,
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    store: $crate::store::SimpleStore::new($initial),
                }
            }

            pub fn get(&self) -> $state_type {
                self.store.get()
            }

            pub fn set(&self, state: $state_type) {
                self.store.set(state);
            }

            pub fn update<F>(&self, f: F)
            where
                F: FnOnce($state_type) -> $state_type,
            {
                self.store.update(f);
            }
        }

        impl $crate::store::Store for $name {
            type State = $state_type;

            fn get(&self) -> Self::State {
                self.store.get()
            }

            fn set(&self, state: Self::State) {
                self.store.set(state);
            }

            fn update<F>(&self, f: F)
            where
                F: FnOnce(Self::State) -> Self::State,
            {
                self.store.update(f);
            }
        }
    };
}

/// Reactive store that integrates with Leptos signals
pub struct ReactiveStore<T: Clone + PartialEq + 'static> {
    /// The Leptos signal for the state
    pub signal: leptos::RwSignal<T>,
}

impl<T: Clone + PartialEq + 'static> ReactiveStore<T> {
    /// Create a new reactive store
    pub fn new(initial: T) -> Self {
        Self {
            signal: leptos::create_rw_signal(initial),
        }
    }

    /// Get a read-only signal
    pub fn read_signal(&self) -> leptos::ReadSignal<T> {
        self.signal.read_only()
    }

    /// Get a write signal
    pub fn write_signal(&self) -> leptos::WriteSignal<T> {
        self.signal.write_only()
    }
}

impl<T: Clone + PartialEq + 'static> Store for ReactiveStore<T> {
    type State = T;

    fn get(&self) -> Self::State {
        self.signal.get()
    }

    fn set(&self, state: Self::State) {
        self.signal.set(state);
    }

    fn update<F>(&self, f: F)
    where
        F: FnOnce(Self::State) -> Self::State,
    {
        self.signal.update(f);
    }
}

/// Async store for handling asynchronous state updates
pub struct AsyncStore<T: Clone + PartialEq + 'static> {
    /// The underlying store
    pub store: SimpleStore<T>,
    /// Pending operations counter
    pub pending_ops: std::sync::atomic::AtomicU32,
}

impl<T: Clone + PartialEq + 'static> AsyncStore<T> {
    /// Create a new async store
    pub fn new(initial: T) -> Self {
        Self {
            store: SimpleStore::new(initial),
            pending_ops: std::sync::atomic::AtomicU32::new(0),
        }
    }

    /// Update the store asynchronously
    pub async fn update_async<F, Fut>(&self, f: F)
    where
        F: FnOnce(T) -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        self.pending_ops.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        let current = self.store.get();
        let new_state = f(current).await;

        self.store.set(new_state);
        self.pending_ops.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
    }

    /// Check if there are pending operations
    pub fn has_pending_ops(&self) -> bool {
        self.pending_ops.load(std::sync::atomic::Ordering::SeqCst) > 0
    }

    /// Get the number of pending operations
    pub fn pending_ops_count(&self) -> u32 {
        self.pending_ops.load(std::sync::atomic::Ordering::SeqCst)
    }
}

impl<T: Clone + PartialEq + 'static> Store for AsyncStore<T> {
    type State = T;

    fn get(&self) -> Self::State {
        self.store.get()
    }

    fn set(&self, state: Self::State) {
        self.store.set(state);
    }

    fn update<F>(&self, f: F)
    where
        F: FnOnce(Self::State) -> Self::State,
    {
        self.store.update(f);
    }
}

/// Middleware-enabled store
pub struct MiddlewareStore<T: Clone + PartialEq + 'static, M> {
    /// The underlying store
    pub store: SimpleStore<T>,
    /// The middleware
    pub middleware: M,
}

impl<T: Clone + PartialEq + 'static, M> MiddlewareStore<T, M> {
    /// Create a new middleware store
    pub fn new(initial: T, middleware: M) -> Self {
        Self {
            store: SimpleStore::new(initial),
            middleware,
        }
    }
}

impl<T: Clone + PartialEq + 'static, M> Store for MiddlewareStore<T, M>
where
    M: StoreMiddleware<T>,
{
    type State = T;

    fn get(&self) -> Self::State {
        let value = self.store.get();
        self.middleware.on_get(&value);
        value
    }

    fn set(&self, state: Self::State) {
        self.middleware.on_set(&self.store.get(), &state);
        self.store.set(state);
    }

    fn update<F>(&self, f: F)
    where
        F: FnOnce(Self::State) -> Self::State,
    {
        let current = self.store.get();
        let new_state = f(current.clone());
        self.middleware.on_update(&current, &new_state);
        self.store.update(f);
    }
}

/// Store middleware trait
pub trait StoreMiddleware<T> {
    /// Called when getting the state
    fn on_get(&self, state: &T) {}

    /// Called when setting the state
    fn on_set(&self, old_state: &T, new_state: &T) {}

    /// Called when updating the state
    fn on_update(&self, old_state: &T, new_state: &T) {}
}

/// Logging middleware
pub struct LoggingMiddleware;

impl<T> StoreMiddleware<T> for LoggingMiddleware {
    fn on_set(&self, old_state: &T, new_state: &T) {
        eprintln!("Store state changed");
    }

    fn on_update(&self, old_state: &T, new_state: &T) {
        eprintln!("Store state updated");
    }
}

/// Validation middleware
pub struct ValidationMiddleware<T, F> {
    /// Validation function
    pub validator: F,
    /// Phantom data
    _phantom: std::marker::PhantomData<T>,
}

impl<T, F> ValidationMiddleware<T, F>
where
    F: Fn(&T) -> Result<(), String>,
{
    /// Create a new validation middleware
    pub fn new(validator: F) -> Self {
        Self {
            validator,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, F> StoreMiddleware<T> for ValidationMiddleware<T, F>
where
    F: Fn(&T) -> Result<(), String>,
{
    fn on_set(&self, _old_state: &T, new_state: &T) {
        if let Err(error) = (self.validator)(new_state) {
            eprintln!("Store validation failed: {}", error);
        }
    }

    fn on_update(&self, _old_state: &T, new_state: &T) {
        if let Err(error) = (self.validator)(new_state) {
            eprintln!("Store validation failed: {}", error);
        }
    }
}
