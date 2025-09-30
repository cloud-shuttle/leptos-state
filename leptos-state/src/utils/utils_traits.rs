//! Common utility traits

/// Helper trait for creating unique identifiers
pub trait WithId {
    /// Get the unique identifier
    fn id(&self) -> &str;

    /// Set the unique identifier
    fn set_id(&mut self, id: String);

    /// Generate a new unique identifier
    fn generate_id() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    /// Check if this has the same ID as another
    fn has_same_id(&self, other: &impl WithId) -> bool {
        self.id() == other.id()
    }
}

/// Helper trait for validation
pub trait Validate {
    /// Validate this instance
    fn validate(&self) -> Result<(), String>;

    /// Check if this instance is valid
    fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }
}

/// Helper trait for serialization
pub trait Serialize {
    /// Serialize this instance to JSON
    fn to_json(&self) -> Result<String, Box<dyn std::error::Error>>;

    /// Serialize this instance to a pretty JSON string
    fn to_json_pretty(&self) -> Result<String, Box<dyn std::error::Error>>;
}

/// Helper trait for deserialization
pub trait Deserialize<T> {
    /// Deserialize from JSON string
    fn from_json(json: &str) -> Result<T, Box<dyn std::error::Error>>;
}

/// Subscription handle for cleanup
pub struct SubscriptionHandle {
    /// Cleanup function
    pub cleanup: Option<Box<dyn FnOnce() + Send + Sync>>,
}

impl SubscriptionHandle {
    /// Create a new subscription handle
    pub fn new<F>(cleanup: F) -> Self
    where
        F: FnOnce() + Send + Sync + 'static,
    {
        Self {
            cleanup: Some(Box::new(cleanup)),
        }
    }

    /// Create a handle with no cleanup
    pub fn empty() -> Self {
        Self { cleanup: None }
    }

    /// Manually cleanup the subscription
    pub fn cleanup(mut self) {
        if let Some(cleanup) = self.cleanup.take() {
            cleanup();
        }
    }

    /// Check if this handle has cleanup
    pub fn has_cleanup(&self) -> bool {
        self.cleanup.is_some()
    }
}

impl Drop for SubscriptionHandle {
    fn drop(&mut self) {
        if let Some(cleanup) = self.cleanup.take() {
            cleanup();
        }
    }
}

/// Extension trait for creating subscription handles
pub trait Subscribable {
    /// Create a subscription handle
    fn subscribe<F>(&self, cleanup: F) -> SubscriptionHandle
    where
        F: FnOnce() + Send + Sync + 'static;
}

/// Observable pattern implementation
pub trait Observable<T> {
    /// Subscribe to changes
    fn subscribe<F>(&self, callback: F) -> SubscriptionHandle
    where
        F: Fn(&T) + Send + Sync + 'static;

    /// Notify all subscribers of a change
    fn notify(&self, value: &T);

    /// Get the current value
    fn get(&self) -> T;

    /// Set the value and notify subscribers
    fn set(&mut self, value: T) {
        // Implementation would call notify here
        let _ = value; // Placeholder
    }
}

/// Cloneable trait for dynamic dispatch
pub trait Cloneable: std::fmt::Debug {
    /// Clone this instance
    fn clone_box(&self) -> Box<dyn Cloneable>;
}

impl<T: Clone + std::fmt::Debug + 'static> Cloneable for T {
    fn clone_box(&self) -> Box<dyn Cloneable> {
        Box::new(self.clone())
    }
}

/// Type-erased observable
pub struct ErasedObservable {
    /// The observable implementation
    pub inner: Box<dyn std::any::Any + Send + Sync>,
    /// Clone function
    pub clone_fn: Box<
        dyn Fn(&Box<dyn std::any::Any + Send + Sync>) -> Box<dyn std::any::Any + Send + Sync>
            + Send
            + Sync,
    >,
}

impl ErasedObservable {
    /// Create a new erased observable
    pub fn new<T: Clone + Send + Sync + 'static>(value: T) -> Self {
        let clone_fn = |inner: &Box<dyn std::any::Any + Send + Sync>| {
            inner
                .downcast_ref::<T>()
                .map(|v| Box::new(v.clone()) as Box<dyn std::any::Any + Send + Sync>)
                .unwrap_or_else(|| Box::new(()) as Box<dyn std::any::Any + Send + Sync>)
        };

        Self {
            inner: Box::new(value),
            clone_fn: Box::new(clone_fn),
        }
    }

    /// Try to get the value as a specific type
    pub fn as_ref<T: 'static>(&self) -> Option<&T> {
        self.inner.downcast_ref()
    }
}

/// Builder pattern trait
pub trait Builder<T> {
    /// Build the final object
    fn build(self) -> Result<T, String>;

    /// Validate the builder state
    fn validate(&self) -> Result<(), String>;
}

/// Default implementation for builders
pub trait BuilderExt<T>: Builder<T> + Sized {
    /// Build and unwrap, panicking on error
    fn build_unwrap(self) -> T {
        self.build().unwrap()
    }

    /// Build with a default on error
    fn build_or_default(self, default: T) -> T {
        self.build().unwrap_or(default)
    }
}

impl<T, B: Builder<T>> BuilderExt<T> for B {}

/// Factory pattern trait
pub trait Factory<T> {
    /// Create a new instance
    fn create(&self) -> T;

    /// Create a new instance with configuration
    fn create_with_config(&self, config: &Config) -> T;
}

// Re-export Config for the Factory trait
use super::utils_config::Config;

/// Named factory for creating named instances
pub trait NamedFactory<T>: Factory<T> {
    /// Get the factory name
    fn name(&self) -> &str;

    /// Create a named instance
    fn create_named(&self, name: &str) -> T;
}

/// Registry for factories
pub struct FactoryRegistry<T, F: Factory<T>> {
    /// Registered factories
    pub factories: std::collections::HashMap<String, Box<F>>,
}

impl<T, F: Factory<T>> FactoryRegistry<T, F> {
    /// Create a new registry
    pub fn new() -> Self {
        Self {
            factories: std::collections::HashMap::new(),
        }
    }

    /// Register a factory
    pub fn register(&mut self, name: String, factory: Box<F>) {
        self.factories.insert(name, factory);
    }

    /// Get a factory by name
    pub fn get(&self, name: &str) -> Option<&F> {
        self.factories.get(name).map(|f| f.as_ref())
    }

    /// Create an instance using a named factory
    pub fn create(&self, name: &str) -> Option<T> {
        self.get(name).map(|f| f.create())
    }

    /// Create an instance with configuration using a named factory
    pub fn create_with_config(&self, name: &str, config: &Config) -> Option<T> {
        self.get(name).map(|f| f.create_with_config(config))
    }

    /// List all registered factory names
    pub fn list_names(&self) -> Vec<String> {
        self.factories.keys().cloned().collect()
    }
}

/// Event-driven trait
pub trait EventDriven<E> {
    /// Handle an event
    fn handle_event(&mut self, event: &E) -> Result<(), String>;

    /// Check if this can handle a particular event type
    fn can_handle(&self, event: &E) -> bool;
}

/// State machine trait for objects that have state
pub trait HasState<S> {
    /// Get the current state
    fn state(&self) -> &S;

    /// Set the state
    fn set_state(&mut self, state: S);

    /// Check if the current state matches
    fn is_state(&self, state: &S) -> bool
    where
        S: PartialEq,
    {
        self.state() == state
    }
}

/// Lifecycle trait for objects with lifecycle management
pub trait Lifecycle {
    /// Initialize the object
    fn initialize(&mut self) -> Result<(), String> {
        Ok(())
    }

    /// Start the object
    fn start(&mut self) -> Result<(), String> {
        Ok(())
    }

    /// Stop the object
    fn stop(&mut self) -> Result<(), String> {
        Ok(())
    }

    /// Shutdown the object
    fn shutdown(&mut self) -> Result<(), String> {
        self.stop()
    }

    /// Check if the object is running
    fn is_running(&self) -> bool {
        false
    }

    /// Get the lifecycle state
    fn lifecycle_state(&self) -> LifecycleState {
        if self.is_running() {
            LifecycleState::Running
        } else {
            LifecycleState::Stopped
        }
    }
}

/// Lifecycle states
#[derive(Debug, Clone, PartialEq)]
pub enum LifecycleState {
    /// Object has been created but not initialized
    Created,
    /// Object is initializing
    Initializing,
    /// Object is initialized but not started
    Initialized,
    /// Object is starting
    Starting,
    /// Object is running
    Running,
    /// Object is stopping
    Stopping,
    /// Object is stopped
    Stopped,
    /// Object has encountered an error
    Error(String),
}

/// Resource management trait
pub trait Resource: Lifecycle {
    /// Get the resource usage
    fn resource_usage(&self) -> ResourceUsage;

    /// Check if the resource is healthy
    fn is_healthy(&self) -> bool;

    /// Get resource metrics
    fn metrics(&self) -> ResourceMetrics;
}

/// Resource usage information
#[derive(Debug, Clone)]
pub struct ResourceUsage {
    /// CPU usage percentage
    pub cpu_percent: f64,
    /// Memory usage in bytes
    pub memory_bytes: u64,
    /// Disk usage in bytes
    pub disk_bytes: u64,
    /// Network usage in bytes
    pub network_bytes: u64,
}

/// Resource metrics
#[derive(Debug, Clone)]
pub struct ResourceMetrics {
    /// Requests per second
    pub requests_per_second: f64,
    /// Error rate
    pub error_rate: f64,
    /// Average response time
    pub avg_response_time: std::time::Duration,
    /// Total requests served
    pub total_requests: u64,
}
