# ⚡ Performance Optimization Guide - leptos-state v1.0.0

This guide covers performance optimization techniques and best practices for leptos-state applications.

## 📊 Performance Monitoring

### Built-in Performance Tracking

leptos-state includes comprehensive performance monitoring out of the box:

```rust
use leptos_state::v1::*;
use std::time::Duration;

fn monitor_performance() {
    let benchmark = PerformanceBenchmark::new()
        .with_thresholds(PerformanceThresholds {
            max_transition_time: Duration::from_millis(10),
            max_memory_usage: 1024 * 1024, // 1MB
            min_performance_score: 80.0,
        });
    
    // Benchmark state machine transitions
    let result = benchmark.benchmark_operation("state_transition", || {
        // Your operation here
        0 // Return memory usage
    }, 1000);
    
    println!("Performance score: {}", result.performance_score);
    
    // Get optimization suggestions
    let suggestions = benchmark.get_suggestions();
    for suggestion in suggestions {
        println!("Suggestion: {}", suggestion.description);
    }
}
```

### Performance Metrics

The performance system tracks:

- **Execution Time**: Average, minimum, and maximum operation times
- **Memory Usage**: Memory consumption per operation
- **Performance Score**: 0-100 score based on thresholds
- **Optimization Suggestions**: Automated recommendations

## 🚀 Store Optimization

### Minimize Store Updates

```rust
// ❌ Bad: Multiple small updates
fn inefficient_counter() -> impl IntoView {
    let (store, set_store) = use_store::<CounterStore>();
    
    let increment = move |_| {
        set_store.update(|state| state.count += 1);
    };
    
    let decrement = move |_| {
        set_store.update(|state| state.count -= 1);
    };
    
    let reset = move |_| {
        set_store.update(|state| state.count = 0);
    };
    
    // ... view
}

// ✅ Good: Batch updates
fn efficient_counter() -> impl IntoView {
    let (store, set_store) = use_store::<CounterStore>();
    
    let batch_update = move |action: &str| {
        set_store.update(|state| {
            match action {
                "increment" => state.count += 1,
                "decrement" => state.count -= 1,
                "reset" => state.count = 0,
                _ => {}
            }
        });
    };
    
    // ... view
}
```

### Use Selective Updates

```rust
// ❌ Bad: Update entire store
set_store.update(|state| {
    state.count += 1;
    state.name = "Updated".to_string(); // Unnecessary update
});

// ✅ Good: Update only what changed
set_store.update(|state| {
    state.count += 1;
    // Only update what actually changed
});
```

### Implement Smart Cloning

```rust
#[derive(Clone, PartialEq, Debug, Default)]
struct OptimizedStore {
    count: i32,
    data: Arc<Vec<String>>, // Use Arc for large, immutable data
    metadata: Arc<HashMap<String, String>>,
}

impl StoreState for OptimizedStore {}

impl Store for OptimizedStore {
    fn create() -> Self {
        Self {
            count: 0,
            data: Arc::new(Vec::new()),
            metadata: Arc::new(HashMap::new()),
        }
    }
    
    // ... other methods
}
```

## 🎯 State Machine Optimization

### Optimize State Transitions

```rust
// ❌ Bad: Complex transition logic
impl StateMachine for TrafficState {
    fn transition(&self, state: &Self, event: Self::Event) -> Self {
        match (state, event) {
            (TrafficState::Red, TrafficEvent::Timer) => {
                // Complex logic here
                if some_condition() {
                    TrafficState::Green
                } else {
                    TrafficState::Yellow
                }
            },
            // ... many more cases
        }
    }
}

// ✅ Good: Simple, fast transitions
impl StateMachine for TrafficState {
    fn transition(&self, state: &Self, event: Self::Event) -> Self {
        match (state, event) {
            (TrafficState::Red, TrafficEvent::Timer) => TrafficState::Green,
            (TrafficState::Green, TrafficEvent::Timer) => TrafficState::Yellow,
            (TrafficState::Yellow, TrafficEvent::Timer) => TrafficState::Red,
            _ => state.clone(),
        }
    }
}
```

### Use Guards Efficiently

```rust
// ❌ Bad: Expensive guard operations
struct ExpensiveGuard;

impl Guard<TrafficContext, TrafficEvent> for ExpensiveGuard {
    fn check(&self, context: &TrafficContext, _event: &TrafficEvent) -> bool {
        // Expensive database query or network call
        expensive_operation(context)
    }
}

// ✅ Good: Cache expensive guard results
struct CachedGuard {
    cache: Arc<Mutex<HashMap<String, bool>>>,
}

impl Guard<TrafficContext, TrafficEvent> for CachedGuard {
    fn check(&self, context: &TrafficContext, _event: &TrafficEvent) -> bool {
        let cache_key = format!("{:?}", context);
        
        if let Ok(cache) = self.cache.lock() {
            if let Some(&result) = cache.get(&cache_key) {
                return result;
            }
        }
        
        // Expensive operation only when needed
        let result = expensive_operation(context);
        
        // Cache the result
        if let Ok(mut cache) = self.cache.lock() {
            cache.insert(cache_key, result);
        }
        
        result
    }
}
```

### Optimize Action Execution

```rust
// ❌ Bad: Synchronous expensive actions
struct ExpensiveAction;

impl Action<TrafficContext> for ExpensiveAction {
    fn execute(&self, context: &mut TrafficContext) -> Result<(), ActionError> {
        // Blocking operation
        std::thread::sleep(Duration::from_millis(100));
        context.timer += 1;
        Ok(())
    }
}

// ✅ Good: Async actions for expensive operations
struct AsyncAction;

impl Action<TrafficContext> for AsyncAction {
    fn execute(&self, context: &mut TrafficContext) -> Result<(), ActionError> {
        // Non-blocking operation
        spawn_local(async move {
            let result = expensive_async_operation().await;
            // Handle result asynchronously
        });
        
        context.timer += 1;
        Ok(())
    }
}
```

## 💾 Persistence Optimization

### Batch Persistence Operations

```rust
// ❌ Bad: Persist on every update
fn inefficient_persistence() {
    let store = create_store_with_persistence::<CounterStore>("counter");
    
    // This persists on every update
    for i in 0..1000 {
        store.update(|state| state.count = i);
    }
}

// ✅ Good: Batch persistence
fn efficient_persistence() {
    let store = create_store_with_persistence::<CounterStore>("counter");
    
    // Batch updates
    store.update(|state| {
        for i in 0..1000 {
            state.count = i;
        }
    });
    
    // Persist once at the end
    store.persist();
}
```

### Use Appropriate Storage Backends

```rust
// For small, frequently updated data
let fast_store = create_store_with_persistence::<SmallStore>("fast")
    .with_backend(StorageBackend::Memory); // In-memory for speed

// For large, infrequently updated data
let persistent_store = create_store_with_persistence::<LargeStore>("persistent")
    .with_backend(StorageBackend::LocalStorage) // Persistent storage
    .with_compression(true); // Enable compression
```

### Implement Smart Serialization

```rust
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
struct OptimizedStore {
    #[serde(skip_serializing_if = "Option::is_none")]
    optional_field: Option<String>,
    
    #[serde(default)]
    default_field: String,
    
    #[serde(skip_serializing_if = "Vec::is_empty")]
    empty_vector: Vec<String>,
}
```

## 🧪 Testing Performance

### Performance Regression Testing

```rust
#[test]
fn test_performance_regression() {
    let benchmark = PerformanceBenchmark::new()
        .with_thresholds(PerformanceThresholds {
            max_transition_time: Duration::from_millis(5),
            max_memory_usage: 512 * 1024, // 512KB
            min_performance_score: 90.0,
        });
    
    let result = benchmark.benchmark_operation("state_transition", || {
        // Your operation here
        0
    }, 1000);
    
    assert!(
        benchmark.meets_thresholds(&result),
        "Performance regression detected: {:?}",
        result
    );
}
```

### Benchmark Critical Paths

```rust
#[test]
fn benchmark_critical_operations() {
    let mut benchmark = PerformanceBenchmark::new();
    
    // Benchmark state machine creation
    let creation_result = benchmark.benchmark_operation("machine_creation", || {
        let context = TrafficContext::default();
        let _machine = Machine::new(TrafficState::Red, context);
        0
    }, 100);
    
    // Benchmark transitions
    let transition_result = benchmark.benchmark_operation("state_transition", || {
        let context = TrafficContext::default();
        let mut machine = Machine::new(TrafficState::Red, context);
        let _result = machine.transition(TrafficEvent::Timer);
        0
    }, 100);
    
    println!("Creation: {:?}", creation_result);
    println!("Transition: {:?}", transition_result);
}
```

## 🔧 Advanced Optimization Techniques

### Use Lazy Loading

```rust
#[derive(Clone, PartialEq, Debug)]
struct LazyStore {
    count: i32,
    expensive_data: Option<Arc<ExpensiveData>>,
}

impl Store for LazyStore {
    fn get_expensive_data(&self) -> Option<Arc<ExpensiveData>> {
        if self.expensive_data.is_none() {
            // Load data only when needed
            let data = load_expensive_data();
            self.expensive_data = Some(Arc::new(data));
        }
        self.expensive_data.clone()
    }
}
```

### Implement Connection Pooling

```rust
struct ConnectionPool {
    connections: Arc<Mutex<Vec<Connection>>>,
    max_connections: usize,
}

impl ConnectionPool {
    fn get_connection(&self) -> Option<Connection> {
        if let Ok(mut connections) = self.connections.lock() {
            connections.pop()
        } else {
            None
        }
    }
    
    fn return_connection(&self, connection: Connection) {
        if let Ok(mut connections) = self.connections.lock() {
            if connections.len() < self.max_connections {
                connections.push(connection);
            }
        }
    }
}
```

### Use Smart Caching

```rust
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

struct SmartCache<K, V> {
    cache: Arc<Mutex<HashMap<K, (V, Instant)>>>,
    ttl: Duration,
}

impl<K, V> SmartCache<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    fn get(&self, key: &K) -> Option<V> {
        if let Ok(cache) = self.cache.lock() {
            if let Some((value, timestamp)) = cache.get(key) {
                if timestamp.elapsed() < self.ttl {
                    return Some(value.clone());
                }
            }
        }
        None
    }
    
    fn set(&self, key: K, value: V) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.insert(key, (value, Instant::now()));
        }
    }
}
```

## 📈 Performance Monitoring in Production

### Real-time Performance Tracking

```rust
fn enable_production_monitoring() {
    let devtools = DevTools::new()
        .with_performance_tracking(true)
        .with_metrics_export(true)
        .with_alerting(true);
    
    enable_devtools_with_config(devtools);
}
```

### Performance Alerts

```rust
fn setup_performance_alerts() {
    let thresholds = PerformanceThresholds {
        max_transition_time: Duration::from_millis(50),
        max_memory_usage: 10 * 1024 * 1024, // 10MB
        min_performance_score: 70.0,
    };
    
    // Set up alerting when thresholds are exceeded
    spawn_local(async move {
        monitor_performance_thresholds(thresholds).await;
    });
}
```

## 🎯 Best Practices Summary

1. **Minimize Updates**: Batch store updates and only update what changed
2. **Smart Cloning**: Use `Arc` for large, immutable data
3. **Efficient Transitions**: Keep state machine transitions simple and fast
4. **Cache Guards**: Cache expensive guard operations
5. **Async Actions**: Use async actions for expensive operations
6. **Batch Persistence**: Group persistence operations
7. **Lazy Loading**: Load expensive data only when needed
8. **Connection Pooling**: Reuse expensive resources
9. **Smart Caching**: Cache frequently accessed data
10. **Monitor Performance**: Use built-in performance tracking

## 🔍 Performance Debugging

### Enable Debug Logging

```rust
use tracing::Level;

fn enable_debug_logging() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
}
```

### Profile Specific Operations

```rust
use tracing::{info, instrument};

#[instrument(skip(self))]
fn expensive_operation(&self) -> Result<(), Error> {
    info!("Starting expensive operation");
    
    // Your operation here
    
    info!("Completed expensive operation");
    Ok(())
}
```

---

**Remember: Profile first, optimize second! Use the built-in performance tools to identify bottlenecks before applying optimizations.** 🚀
