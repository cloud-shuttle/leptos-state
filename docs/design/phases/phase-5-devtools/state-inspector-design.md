# State Inspector Design

## Overview
Implement a comprehensive state inspection system for runtime debugging, monitoring, and development workflow enhancement.

## Current State
```rust
// Basic state access only
impl<S: State> Store<S> {
    pub fn get(&self) -> ReadSignal<S> {
        self.signal.read_only()
    }
}
```

## Proposed Enhancement
```rust
pub struct StateInspector<S: State> {
    store: Store<S>,
    watch_list: Vec<Box<dyn WatchExpression<S>>>,
    change_detectors: Vec<Box<dyn ChangeDetector<S>>>,
    metrics: InspectorMetrics,
}

impl<S: State> Store<S> {
    pub fn with_inspector(self) -> (Self, StateInspectorHandle<S>) {
        // Enable state inspection
    }
}
```

## Motivation

### Development Debugging
- **Runtime Inspection**: Examine state at any point in execution
- **Change Tracking**: Monitor how and when state changes
- **Watch Expressions**: Track specific state properties
- **Breakpoints**: Pause execution on state conditions
- **Performance Monitoring**: Track state operation timing

### Quality Assurance
- **State Validation**: Ensure state invariants are maintained
- **Change Auditing**: Log all state modifications
- **Performance Profiling**: Identify slow state operations
- **Memory Leak Detection**: Monitor state lifecycle

### Use Cases
- Debugging complex state interactions
- Performance optimization of state operations
- Testing state management correctness
- Learning state change patterns
- Monitoring application health

## Implementation Details

### State Inspector Core
```rust
pub struct StateInspector<S: State> {
    store_name: String,
    current_state: S,
    previous_state: Option<S>,
    change_history: Vec<StateChange<S>>,
    watch_expressions: Vec<Box<dyn WatchExpression<S>>>,
    change_detectors: Vec<Box<dyn ChangeDetector<S>>>,
    metrics: InspectorMetrics,
    max_history_size: usize,
    is_recording: bool,
}

#[derive(Clone, Debug)]
pub struct StateChange<S> {
    pub timestamp: DateTime<Utc>,
    pub operation: String,
    pub old_state: Option<S>,
    pub new_state: S,
    pub changed_fields: Vec<String>,
    pub duration: Duration,
}

#[derive(Clone, Debug, Default)]
pub struct InspectorMetrics {
    pub total_operations: u64,
    pub total_changes: u64,
    pub average_operation_time: Duration,
    pub slowest_operation: Duration,
    pub memory_usage: usize,
    pub last_inspection: DateTime<Utc>,
}

impl<S: State> StateInspector<S> {
    pub fn new(store_name: String, initial_state: S) -> Self {
        Self {
            store_name,
            current_state: initial_state,
            previous_state: None,
            change_history: Vec::new(),
            watch_expressions: Vec::new(),
            change_detectors: Vec::new(),
            metrics: InspectorMetrics::default(),
            max_history_size: 1000,
            is_recording: true,
        }
    }

    pub fn record_operation<F>(&mut self, operation_name: &str, operation: F) -> Result<(), InspectorError>
    where
        F: FnOnce() -> S,
    {
        let start_time = Instant::now();

        let new_state = operation();

        let duration = start_time.elapsed();

        // Update metrics
        self.metrics.total_operations += 1;
        self.metrics.average_operation_time = Duration::from_nanos(
            ((self.metrics.average_operation_time.as_nanos() as u128 * (self.metrics.total_operations - 1) as u128)
             + duration.as_nanos() as u128) as u128 / self.metrics.total_operations as u128
        );
        self.metrics.slowest_operation = self.metrics.slowest_operation.max(duration);
        self.metrics.last_inspection = Utc::now();

        if self.is_recording {
            let changed_fields = self.detect_changed_fields(&new_state);

            if !changed_fields.is_empty() {
                self.metrics.total_changes += 1;

                let change = StateChange {
                    timestamp: Utc::now(),
                    operation: operation_name.to_string(),
                    old_state: self.previous_state.clone(),
                    new_state: new_state.clone(),
                    changed_fields,
                    duration,
                };

                self.change_history.push(change);

                // Limit history size
                if self.change_history.len() > self.max_history_size {
                    self.change_history.remove(0);
                }

                // Run watch expressions
                for watch in &self.watch_expressions {
                    watch.evaluate(&self.previous_state, &new_state);
                }

                // Run change detectors
                for detector in &self.change_detectors {
                    detector.detect(&self.previous_state, &new_state);
                }
            }
        }

        self.previous_state = Some(self.current_state.clone());
        self.current_state = new_state;

        Ok(())
    }

    fn detect_changed_fields(&self, new_state: &S) -> Vec<String>
    where
        S: PartialEq,
    {
        // This is a simplified implementation
        // In practice, you'd need reflection or derive macros
        // to detect specific field changes
        if Some(new_state) != self.previous_state.as_ref() {
            vec!["state".to_string()] // Placeholder
        } else {
            Vec::new()
        }
    }

    pub fn get_current_state(&self) -> &S {
        &self.current_state
    }

    pub fn get_change_history(&self) -> &[StateChange<S>] {
        &self.change_history
    }

    pub fn get_metrics(&self) -> &InspectorMetrics {
        &self.metrics
    }

    pub fn clear_history(&mut self) {
        self.change_history.clear();
    }

    pub fn set_recording(&mut self, recording: bool) {
        self.is_recording = recording;
    }
}
```

### Watch Expressions
```rust
pub trait WatchExpression<S: State>: Send + Sync {
    fn name(&self) -> &'static str;
    fn evaluate(&self, old_state: &Option<S>, new_state: &S);
    fn is_active(&self) -> bool;
}

pub struct PropertyWatch<S, F, T> {
    name: String,
    extractor: F,
    last_value: Option<T>,
    on_change: Box<dyn Fn(&T, &T) + Send + Sync>,
}

impl<S, F, T> PropertyWatch<S, F, T>
where
    S: State,
    F: Fn(&S) -> T + Send + Sync,
    T: Clone + PartialEq + Debug + Send + Sync + 'static,
{
    pub fn new(name: String, extractor: F, on_change: Box<dyn Fn(&T, &T) + Send + Sync>) -> Self {
        Self {
            name,
            extractor,
            last_value: None,
            on_change,
        }
    }
}

impl<S, F, T> WatchExpression<S> for PropertyWatch<S, F, T>
where
    S: State,
    F: Fn(&S) -> T + Send + Sync,
    T: Clone + PartialEq + Debug + Send + Sync + 'static,
{
    fn name(&self) -> &'static str {
        &self.name
    }

    fn evaluate(&self, _old_state: &Option<S>, new_state: &S) {
        let new_value = (self.extractor)(new_state);

        if let Some(ref last_value) = self.last_value {
            if &new_value != last_value {
                (self.on_change)(last_value, &new_value);
            }
        }

        self.last_value = Some(new_value);
    }

    fn is_active(&self) -> bool {
        true
    }
}

// Usage example
let count_watch = PropertyWatch::new(
    "count_watch".to_string(),
    |state: &CounterState| state.count,
    Box::new(|old, new| {
        log::info!("Count changed from {} to {}", old, new);
    })
);

inspector.add_watch_expression(Box::new(count_watch));
```

### Change Detectors
```rust
pub trait ChangeDetector<S: State>: Send + Sync {
    fn name(&self) -> &'static str;
    fn detect(&self, old_state: &Option<S>, new_state: &S) -> Option<ChangeDetection>;
    fn is_active(&self) -> bool;
}

#[derive(Clone, Debug)]
pub struct ChangeDetection {
    pub detector_name: String,
    pub severity: ChangeSeverity,
    pub message: String,
    pub suggestion: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ChangeSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

pub struct ValidationChangeDetector<S, F> {
    name: String,
    validator: F,
}

impl<S, F> ValidationChangeDetector<S, F>
where
    S: State,
    F: Fn(&S) -> Option<ChangeDetection> + Send + Sync,
{
    pub fn new(name: String, validator: F) -> Self {
        Self { name, validator }
    }
}

impl<S, F> ChangeDetector<S> for ValidationChangeDetector<S, F>
where
    S: State,
    F: Fn(&S) -> Option<ChangeDetection> + Send + Sync,
{
    fn name(&self) -> &'static str {
        &self.name
    }

    fn detect(&self, _old_state: &Option<S>, new_state: &S) -> Option<ChangeDetection> {
        (self.validator)(new_state)
    }

    fn is_active(&self) -> bool {
        true
    }
}

// Usage example
let negative_count_detector = ValidationChangeDetector::new(
    "negative_count_detector".to_string(),
    |state: &CounterState| {
        if state.count < 0 {
            Some(ChangeDetection {
                detector_name: "negative_count_detector".to_string(),
                severity: ChangeSeverity::Error,
                message: format!("Count became negative: {}", state.count),
                suggestion: Some("Ensure count never goes below 0".to_string()),
            })
        } else {
            None
        }
    }
);

inspector.add_change_detector(Box::new(negative_count_detector));
```

### Performance Profiling
```rust
pub struct PerformanceProfiler<S: State> {
    operation_times: HashMap<String, Vec<Duration>>,
    slow_operation_threshold: Duration,
    on_slow_operation: Box<dyn Fn(&str, Duration) + Send + Sync>,
}

impl<S: State> PerformanceProfiler<S> {
    pub fn new(threshold: Duration, on_slow: Box<dyn Fn(&str, Duration) + Send + Sync>) -> Self {
        Self {
            operation_times: HashMap::new(),
            slow_operation_threshold: threshold,
            on_slow_operation: on_slow,
        }
    }

    pub fn record_operation(&mut self, operation_name: &str, duration: Duration) {
        self.operation_times
            .entry(operation_name.to_string())
            .or_insert_with(Vec::new)
            .push(duration);

        if duration > self.slow_operation_threshold {
            (self.on_slow_operation)(operation_name, duration);
        }
    }

    pub fn get_operation_stats(&self, operation_name: &str) -> Option<OperationStats> {
        self.operation_times.get(operation_name).map(|times| {
            let total: Duration = times.iter().sum();
            let count = times.len();
            let avg = total / count as u32;
            let min = times.iter().min().unwrap();
            let max = times.iter().max().unwrap();

            OperationStats {
                operation_name: operation_name.to_string(),
                call_count: count,
                total_time: total,
                average_time: avg,
                min_time: *min,
                max_time: *max,
            }
        })
    }

    pub fn get_all_stats(&self) -> Vec<OperationStats> {
        self.operation_times.keys()
            .filter_map(|name| self.get_operation_stats(name))
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct OperationStats {
    pub operation_name: String,
    pub call_count: usize,
    pub total_time: Duration,
    pub average_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
}
```

### Store Integration
```rust
pub struct InspectorHandle<S: State> {
    inspector: Arc<Mutex<StateInspector<S>>>,
}

impl<S: State> InspectorHandle<S> {
    pub fn record_operation<F>(&self, operation_name: &str, operation: F) -> Result<(), InspectorError>
    where
        F: FnOnce() -> S,
    {
        self.inspector.lock().unwrap().record_operation(operation_name, operation)
    }

    pub fn add_watch_expression(&self, watch: Box<dyn WatchExpression<S>>) {
        self.inspector.lock().unwrap().watch_expressions.push(watch);
    }

    pub fn add_change_detector(&self, detector: Box<dyn ChangeDetector<S>>) {
        self.inspector.lock().unwrap().change_detectors.push(detector);
    }

    pub fn get_current_state(&self) -> S
    where
        S: Clone,
    {
        self.inspector.lock().unwrap().get_current_state().clone()
    }

    pub fn get_change_history(&self) -> Vec<StateChange<S>>
    where
        S: Clone,
    {
        self.inspector.lock().unwrap().get_change_history().to_vec()
    }

    pub fn get_metrics(&self) -> InspectorMetrics {
        self.inspector.lock().unwrap().get_metrics().clone()
    }

    pub fn export_report(&self) -> InspectorReport<S>
    where
        S: Clone,
    {
        let inspector = self.inspector.lock().unwrap();
        InspectorReport {
            store_name: inspector.store_name.clone(),
            current_state: inspector.current_state.clone(),
            change_history: inspector.change_history.clone(),
            metrics: inspector.metrics.clone(),
            watch_expressions: inspector.watch_expressions.len(),
            change_detectors: inspector.change_detectors.len(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InspectorReport<S> {
    pub store_name: String,
    pub current_state: S,
    pub change_history: Vec<StateChange<S>>,
    pub metrics: InspectorMetrics,
    pub watch_expressions: usize,
    pub change_detectors: usize,
}
```

## Error Handling

### Inspector Errors
```rust
#[derive(Debug, Clone, thiserror::Error)]
pub enum InspectorError {
    #[error("Inspector operation failed: {message}")]
    OperationFailed { message: String },

    #[error("Watch expression error: {expression} - {message}")]
    WatchExpressionError { expression: String, message: String },

    #[error("Change detector error: {detector} - {message}")]
    ChangeDetectorError { detector: String, message: String },

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Performance profiling error: {0}")]
    Profiling(String),
}
```

### Safe Inspection
```rust
impl<S: State> StateInspector<S> {
    pub fn inspect_safely<F, R>(&self, inspector: F) -> Result<R, InspectorError>
    where
        F: FnOnce(&Self) -> Result<R, InspectorError>,
    {
        std::panic::catch_unwind(|| inspector(self))
            .map_err(|_| InspectorError::OperationFailed {
                message: "Inspector operation panicked".to_string(),
            })?
    }
}
```

## Testing Strategy

### Unit Tests
```rust
#[test]
fn inspector_records_state_changes() {
    let mut inspector = StateInspector::new("test_store".to_string(), TestState { count: 0 });

    inspector.record_operation("increment", || TestState { count: 1 }).unwrap();
    inspector.record_operation("increment", || TestState { count: 2 }).unwrap();

    let history = inspector.get_change_history();
    assert_eq!(history.len(), 2);
    assert_eq!(history[0].operation, "increment");
    assert_eq!(history[1].new_state.count, 2);
}

#[test]
fn watch_expression_triggers_on_change() {
    let mut inspector = StateInspector::new("test_store".to_string(), TestState { count: 0 });

    let mut change_count = 0;
    let watch = PropertyWatch::new(
        "count_watch".to_string(),
        |state| state.count,
        Box::new(|_old, _new| change_count += 1)
    );

    inspector.watch_expressions.push(Box::new(watch));

    inspector.record_operation("change", || TestState { count: 5 }).unwrap();
    assert_eq!(change_count, 1);

    inspector.record_operation("no_change", || TestState { count: 5 }).unwrap();
    assert_eq!(change_count, 1); // Should not trigger on same value
}

#[test]
fn performance_profiler_tracks_operations() {
    let mut profiler = PerformanceProfiler::new(
        Duration::from_millis(100),
        Box::new(|name, duration| {
            log::warn!("Slow operation: {} took {:?}", name, duration);
        })
    );

    profiler.record_operation("fast_op", Duration::from_millis(10));
    profiler.record_operation("slow_op", Duration::from_millis(200));

    let stats = profiler.get_operation_stats("fast_op").unwrap();
    assert_eq!(stats.call_count, 1);
    assert_eq!(stats.average_time, Duration::from_millis(10));
}
```

### Integration Tests
```rust
#[test]
fn store_with_inspector_integration() {
    let (store, inspector) = create_store_with_inspector(TestState { count: 0 });

    // Perform operations through the store
    store.update(|s| s.count = 10).unwrap();
    store.update(|s| s.count = 20).unwrap();

    // Check inspector recorded the changes
    let history = inspector.get_change_history();
    assert_eq!(history.len(), 2);

    let metrics = inspector.get_metrics();
    assert_eq!(metrics.total_operations, 2);
    assert_eq!(metrics.total_changes, 2);

    // Export report
    let report = inspector.export_report();
    assert_eq!(report.store_name, "test_store");
    assert_eq!(report.current_state.count, 20);
}
```

## Performance Impact

### Inspection Overhead
- **Memory**: History storage, watch expressions, change detectors
- **CPU**: Field change detection, watch expression evaluation
- **Configurable**: Can disable recording, limit history size
- **Async Option**: Move heavy operations to background threads

### Optimization Strategies
```rust
impl<S: State> StateInspector<S> {
    pub fn with_async_processing(mut self) -> Self {
        // Process watch expressions and detectors asynchronously
        self.async_processing = true;
        self
    }

    pub fn with_compressed_history(mut self) -> Self {
        // Compress old history entries
        self.compression_enabled = true;
        self
    }

    pub fn with_sampling(mut self, sample_rate: f64) -> Self {
        // Only inspect a percentage of operations
        self.sampling_rate = sample_rate;
        self
    }

    pub fn optimize_for_performance(mut self) -> Self {
        self.max_history_size = 100; // Smaller history
        self.watch_expressions.clear(); // No watches
        self.change_detectors.clear(); // No detectors
        self.is_recording = false; // No recording
        self
    }
}
```

## Security Considerations

### Information Disclosure
- Inspectors may expose sensitive state data
- Filter sensitive information in reports
- Access control for inspector operations

```rust
impl<S: State> StateInspector<S> {
    pub fn with_data_filter<F>(mut self, filter: F) -> Self
    where
        F: Fn(&S) -> S + Send + Sync + 'static,
    {
        self.data_filter = Some(Box::new(filter));
        self
    }

    fn filter_state(&self, state: &S) -> S
    where
        S: Clone,
    {
        if let Some(ref filter) = self.data_filter {
            filter(state)
        } else {
            state.clone()
        }
    }
}
```

### Access Control
```rust
pub trait InspectorAccessControl {
    fn can_inspect(&self, user: &str, operation: &str) -> bool;
    fn can_modify(&self, user: &str, operation: &str) -> bool;
    fn audit_access(&self, user: &str, operation: &str, allowed: bool);
}

impl<S: State> StateInspector<S> {
    pub fn with_access_control(mut self, access_control: Box<dyn InspectorAccessControl + Send + Sync>) -> Self {
        self.access_control = Some(access_control);
        self
    }
}
```

## Future Extensions

### Advanced Analytics
```rust
pub struct StateAnalytics<S: State> {
    state_patterns: Vec<StatePattern<S>>,
    anomaly_detectors: Vec<Box<dyn AnomalyDetector<S>>>,
    predictive_models: Vec<Box<dyn PredictiveModel<S>>>,
}

impl<S: State> StateAnalytics<S> {
    pub fn detect_patterns(&self, history: &[StateChange<S>]) -> Vec<DetectedPattern> {
        // Detect common state change patterns
        todo!()
    }

    pub fn predict_next_state(&self, current_state: &S) -> Vec<(S, f64)> {
        // Predict likely next states with probabilities
        todo!()
    }

    pub fn detect_anomalies(&self, change: &StateChange<S>) -> Vec<Anomaly> {
        // Detect unusual state changes
        todo!()
    }
}
```

### Remote Inspection
```rust
#[cfg(feature = "remote-inspection")]
pub struct RemoteInspectorClient {
    server_url: String,
    auth_token: String,
}

#[cfg(feature = "remote-inspection")]
impl RemoteInspectorClient {
    pub async fn connect(&self) -> Result<(), InspectorError> {
        // Connect to remote inspection server
        todo!()
    }

    pub async fn send_report(&self, report: &InspectorReport<impl State>) -> Result<(), InspectorError> {
        // Send inspection report to remote server
        todo!()
    }

    pub async fn receive_commands(&self) -> Result<Vec<InspectorCommand>, InspectorError> {
        // Receive commands from remote server
        todo!()
    }
}
```

### Visual Debugging
```rust
#[cfg(feature = "visual-debugging")]
pub struct VisualDebugger<S: State> {
    canvas_renderer: Box<dyn CanvasRenderer>,
    state_visualizers: Vec<Box<dyn StateVisualizer<S>>>,
}

#[cfg(feature = "visual-debugging")]
impl<S: State> VisualDebugger<S> {
    pub fn render_state_graph(&self, history: &[StateChange<S>]) -> Result<String, InspectorError> {
        // Render state change graph as SVG
        todo!()
    }

    pub fn render_state_timeline(&self, history: &[StateChange<S>]) -> Result<String, InspectorError> {
        // Render timeline of state changes
        todo!()
    }
}
```

## Migration Guide

### Adding Inspection to Existing Stores
```rust
// Before - basic store
let store = Store::new(initial_state);

// After - with inspection
let (store, inspector) = store.with_inspector("my_store");

// Add some basic monitoring
inspector.add_change_detector(Box::new(ValidationChangeDetector::new(
    "basic_validation".to_string(),
    |state| {
        // Basic validation logic
        None // No issues
    }
)));
```

### Gradual Adoption
```rust
// Phase 1: Basic inspection
let (store, inspector) = create_store_with_basic_inspection(initial_state);

// Phase 2: Add watch expressions
inspector.add_watch_expression(Box::new(PropertyWatch::new(
    "important_property".to_string(),
    |state| state.important_property,
    Box::new(|old, new| {
        log::info!("Important property changed: {:?} -> {:?}", old, new);
    })
)));

// Phase 3: Add performance monitoring
let profiler = PerformanceProfiler::new(
    Duration::from_millis(50),
    Box::new(|op, duration| {
        log::warn!("Slow operation: {} took {:?}", op, duration);
    })
);

// Phase 4: Add validation
inspector.add_change_detector(Box::new(ValidationChangeDetector::new(
    "business_rules".to_string(),
    |state| validate_business_rules(state)
)));
```

### Configuration-Based Inspection
```rust
#[derive(Deserialize)]
pub struct InspectorConfig {
    pub enable_inspection: bool,
    pub max_history_size: usize,
    pub enable_performance_monitoring: bool,
    pub slow_operation_threshold_ms: u64,
    pub watch_expressions: Vec<WatchExpressionConfig>,
}

pub fn create_store_with_inspection<S: State>(
    initial: S,
    config: &InspectorConfig
) -> (Store<S>, Option<InspectorHandle<S>>) {
    let store = Store::new(initial);

    if !config.enable_inspection {
        return (store, None);
    }

    let (store, inspector) = store.with_inspector("configured_store");

    // Configure based on config
    inspector.set_max_history_size(config.max_history_size);

    if config.enable_performance_monitoring {
        let profiler = PerformanceProfiler::new(
            Duration::from_millis(config.slow_operation_threshold_ms),
            Box::new(|op, duration| {
                log::warn!("Slow operation detected: {} took {:?}", op, duration);
            })
        );
        inspector.set_performance_profiler(profiler);
    }

    for watch_config in &config.watch_expressions {
        let watch = create_watch_expression_from_config(watch_config);
        inspector.add_watch_expression(watch);
    }

    (store, Some(inspector))
}
```

## Risk Assessment

### Likelihood: Medium
- Inspection adds complexity and potential performance overhead
- Watch expressions and detectors can have bugs
- Memory usage can grow with history size

### Impact: Medium
- Performance impact can be significant if not configured properly
- Memory leaks possible if history not managed
- False positives from change detectors

### Mitigation
- Comprehensive testing of inspection features
- Configurable limits and sampling
- Graceful degradation when limits exceeded
- Clear documentation on performance implications
- Access controls and data filtering
- Async processing for heavy operations
