//! Integration configuration structures

use super::*;

/// Integration configuration for state machines
#[derive(Debug, Clone, PartialEq)]
pub struct IntegrationConfig {
    /// Whether integration is enabled
    pub enabled: bool,
    /// Maximum concurrent integrations
    pub max_concurrent: usize,
    /// Timeout for integration operations
    pub timeout: std::time::Duration,
    /// Retry configuration
    pub retry_config: RetryConfig,
    /// Event routing configuration
    pub event_routing: EventRoutingConfig,
    /// Whether to collect metrics
    pub collect_metrics: bool,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_concurrent: 10,
            timeout: std::time::Duration::from_secs(30),
            retry_config: RetryConfig::default(),
            event_routing: EventRoutingConfig::default(),
            collect_metrics: true,
        }
    }
}

impl IntegrationConfig {
    /// Create a new integration config
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable or disable integration
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set maximum concurrent integrations
    pub fn max_concurrent(mut self, max: usize) -> Self {
        self.max_concurrent = max;
        self
    }

    /// Set timeout
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set retry configuration
    pub fn retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

    /// Set event routing configuration
    pub fn event_routing(mut self, routing: EventRoutingConfig) -> Self {
        self.event_routing = routing;
        self
    }

    /// Enable or disable metrics collection
    pub fn collect_metrics(mut self, collect: bool) -> Self {
        self.collect_metrics = collect;
        self
    }
}

/// Event routing configuration
#[derive(Debug, Clone, PartialEq)]
pub struct EventRoutingConfig {
    /// Whether event routing is enabled
    pub enabled: bool,
    /// Routing rules
    pub rules: Vec<RoutingRule>,
    /// Default destination for unrouted events
    pub default_destination: Option<String>,
    /// Whether to route internal events
    pub route_internal_events: bool,
}

impl Default for EventRoutingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: Vec::new(),
            default_destination: None,
            route_internal_events: false,
        }
    }
}

impl EventRoutingConfig {
    /// Create a new event routing config
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable or disable event routing
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Add a routing rule
    pub fn add_rule(mut self, rule: RoutingRule) -> Self {
        self.rules.push(rule);
        self
    }

    /// Set default destination
    pub fn default_destination(mut self, destination: String) -> Self {
        self.default_destination = Some(destination);
        self
    }

    /// Enable routing of internal events
    pub fn route_internal_events(mut self, route: bool) -> Self {
        self.route_internal_events = route;
        self
    }
}

/// Routing rule for events
#[derive(Debug, Clone, PartialEq)]
pub struct RoutingRule {
    /// Rule name
    pub name: String,
    /// Event pattern to match
    pub pattern: EventPattern,
    /// Destination for matched events
    pub destination: String,
    /// Transformation to apply
    pub transformation: Option<EventTransformation>,
    /// Whether the rule is enabled
    pub enabled: bool,
}

impl RoutingRule {
    /// Create a new routing rule
    pub fn new(name: String, pattern: EventPattern, destination: String) -> Self {
        Self {
            name,
            pattern,
            destination,
            transformation: None,
            enabled: true,
        }
    }

    /// Add transformation
    pub fn with_transformation(mut self, transformation: EventTransformation) -> Self {
        self.transformation = Some(transformation);
        self
    }

    /// Enable or disable the rule
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// Event pattern for routing
#[derive(Debug, Clone, PartialEq)]
pub struct EventPattern {
    /// Event type pattern (supports wildcards)
    pub event_type: String,
    /// Source pattern
    pub source: Option<String>,
    /// Priority filter
    pub priority: Option<EventPriority>,
    /// Custom match function
    pub custom_matcher: Option<Box<dyn Fn(&IntegrationEvent) -> bool + Send + Sync>>,
}

impl EventPattern {
    /// Create a new event pattern
    pub fn new(event_type: String) -> Self {
        Self {
            event_type,
            source: None,
            priority: None,
            custom_matcher: None,
        }
    }

    /// Set source pattern
    pub fn source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }

    /// Set priority filter
    pub fn priority(mut self, priority: EventPriority) -> Self {
        self.priority = Some(priority);
        self
    }

    /// Add custom matcher
    pub fn custom_matcher<F>(mut self, matcher: F) -> Self
    where
        F: Fn(&IntegrationEvent) -> bool + Send + Sync + 'static,
    {
        self.custom_matcher = Some(Box::new(matcher));
        self
    }

    /// Check if pattern matches an event
    pub fn matches(&self, event: &IntegrationEvent) -> bool {
        // Check event type (with wildcard support)
        if !self.matches_event_type(&event.event_type) {
            return false;
        }

        // Check source
        if let Some(ref source_pattern) = self.source {
            if !self.matches_source(source_pattern, &event.source) {
                return false;
            }
        }

        // Check priority
        if let Some(priority_filter) = &self.priority {
            if event.priority != priority_filter {
                return false;
            }
        }

        // Check custom matcher
        if let Some(ref matcher) = self.custom_matcher {
            if !matcher(event) {
                return false;
            }
        }

        true
    }

    /// Check if event type matches pattern
    fn matches_event_type(&self, event_type: &str) -> bool {
        if self.event_type.contains('*') {
            // Simple wildcard matching
            let pattern = self.event_type.replace('*', ".*");
            regex::Regex::new(&pattern)
                .map(|re| re.is_match(event_type))
                .unwrap_or(false)
        } else {
            self.event_type == event_type
        }
    }

    /// Check if source matches pattern
    fn matches_source(&self, pattern: &str, source: &str) -> bool {
        if pattern.contains('*') {
            let pattern = pattern.replace('*', ".*");
            regex::Regex::new(&pattern)
                .map(|re| re.is_match(source))
                .unwrap_or(false)
        } else {
            pattern == source
        }
    }
}

/// Event transformation for routing
#[derive(Debug, Clone)]
pub struct EventTransformation {
    /// New event type
    pub new_event_type: Option<String>,
    /// New source
    pub new_source: Option<String>,
    /// New priority
    pub new_priority: Option<EventPriority>,
    /// Data transformation function
    pub data_transformer: Option<Box<dyn Fn(serde_json::Value) -> serde_json::Value + Send + Sync>>,
}

impl EventTransformation {
    /// Create a new event transformation
    pub fn new() -> Self {
        Self {
            new_event_type: None,
            new_source: None,
            new_priority: None,
            data_transformer: None,
        }
    }

    /// Set new event type
    pub fn event_type(mut self, event_type: String) -> Self {
        self.new_event_type = Some(event_type);
        self
    }

    /// Set new source
    pub fn source(mut self, source: String) -> Self {
        self.new_source = Some(source);
        self
    }

    /// Set new priority
    pub fn priority(mut self, priority: EventPriority) -> Self {
        self.new_priority = Some(priority);
        self
    }

    /// Set data transformer
    pub fn data_transformer<F>(mut self, transformer: F) -> Self
    where
        F: Fn(serde_json::Value) -> serde_json::Value + Send + Sync + 'static,
    {
        self.data_transformer = Some(Box::new(transformer));
        self
    }

    /// Apply transformation to an event
    pub fn apply(&self, mut event: IntegrationEvent) -> IntegrationEvent {
        if let Some(ref new_type) = self.new_event_type {
            event.event_type = new_type.clone();
        }

        if let Some(ref new_source) = self.new_source {
            event.source = new_source.clone();
        }

        if let Some(new_priority) = &self.new_priority {
            event.priority = new_priority;
        }

        if let Some(ref transformer) = self.data_transformer {
            event.data = transformer(event.data);
        }

        event
    }
}

/// Retry configuration
#[derive(Debug, Clone, PartialEq)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: usize,
    /// Initial delay between retries
    pub initial_delay: std::time::Duration,
    /// Maximum delay between retries
    pub max_delay: std::time::Duration,
    /// Backoff multiplier
    pub backoff_multiplier: f64,
    /// Whether to use exponential backoff
    pub exponential_backoff: bool,
    /// Jitter factor for randomization
    pub jitter_factor: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: std::time::Duration::from_millis(100),
            max_delay: std::time::Duration::from_secs(30),
            backoff_multiplier: 2.0,
            exponential_backoff: true,
            jitter_factor: 0.1,
        }
    }
}

impl RetryConfig {
    /// Create a new retry config
    pub fn new() -> Self {
        Self::default()
    }

    /// Set maximum attempts
    pub fn max_attempts(mut self, attempts: usize) -> Self {
        self.max_attempts = attempts;
        self
    }

    /// Set initial delay
    pub fn initial_delay(mut self, delay: std::time::Duration) -> Self {
        self.initial_delay = delay;
        self
    }

    /// Set maximum delay
    pub fn max_delay(mut self, delay: std::time::Duration) -> Self {
        self.max_delay = delay;
        self
    }

    /// Set backoff multiplier
    pub fn backoff_multiplier(mut self, multiplier: f64) -> Self {
        self.backoff_multiplier = multiplier;
        self
    }

    /// Enable or disable exponential backoff
    pub fn exponential_backoff(mut self, enabled: bool) -> Self {
        self.exponential_backoff = enabled;
        self
    }

    /// Set jitter factor
    pub fn jitter_factor(mut self, factor: f64) -> Self {
        self.jitter_factor = factor;
        self
    }

    /// Calculate delay for a given attempt number
    pub fn calculate_delay(&self, attempt: usize) -> std::time::Duration {
        if attempt == 0 {
            return self.initial_delay;
        }

        let base_delay = if self.exponential_backoff {
            self.initial_delay * (self.backoff_multiplier.powi(attempt as i32) as u32)
        } else {
            self.initial_delay * (attempt as u32)
        };

        let delay = std::cmp::min(base_delay, self.max_delay);

        // Add jitter
        if self.jitter_factor > 0.0 {
            let jitter = (delay.as_millis() as f64 * self.jitter_factor) as u64;
            let jitter_offset = (rand::random::<u64>() % (jitter * 2)).saturating_sub(jitter);
            std::time::Duration::from_millis(delay.as_millis() as u64 + jitter_offset)
        } else {
            delay
        }
    }
}

/// Connection configuration for adapters
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    /// Connection URL
    pub url: String,
    /// Authentication credentials
    pub credentials: Option<Credentials>,
    /// Connection timeout
    pub timeout: std::time::Duration,
    /// Maximum connections
    pub max_connections: usize,
    /// Connection pool settings
    pub pool_config: PoolConfig,
}

impl ConnectionConfig {
    /// Create a new connection config
    pub fn new(url: String) -> Self {
        Self {
            url,
            credentials: None,
            timeout: std::time::Duration::from_secs(30),
            max_connections: 10,
            pool_config: PoolConfig::default(),
        }
    }

    /// Set credentials
    pub fn credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    /// Set timeout
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set max connections
    pub fn max_connections(mut self, max: usize) -> Self {
        self.max_connections = max;
        self
    }

    /// Set pool config
    pub fn pool_config(mut self, config: PoolConfig) -> Self {
        self.pool_config = config;
        self
    }
}

/// Authentication credentials
#[derive(Debug, Clone)]
pub enum Credentials {
    /// Basic authentication
    Basic { username: String, password: String },
    /// Bearer token
    Bearer { token: String },
    /// API key
    ApiKey { key: String, header_name: String },
    /// Custom credentials
    Custom { headers: std::collections::HashMap<String, String> },
}

/// Connection pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Minimum idle connections
    pub min_idle: usize,
    /// Maximum idle connections
    pub max_idle: usize,
    /// Connection idle timeout
    pub idle_timeout: std::time::Duration,
    /// Maximum lifetime of a connection
    pub max_lifetime: Option<std::time::Duration>,
    /// Health check interval
    pub health_check_interval: std::time::Duration,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            min_idle: 1,
            max_idle: 10,
            idle_timeout: std::time::Duration::from_secs(300),
            max_lifetime: Some(std::time::Duration::from_secs(3600)),
            health_check_interval: std::time::Duration::from_secs(60),
        }
    }
}
