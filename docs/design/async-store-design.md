# ⚡ Async Store Design

## Overview
Design document for asynchronous state management - handling async operations, loading states, error handling, and data fetching patterns.

## Core Async Principles

### 1. Reactive Async Operations
Async operations integrate seamlessly with Leptos reactivity system.

### 2. Comprehensive State Management
Track loading states, errors, and data with fine-grained control.

### 3. Optimistic Updates
Support optimistic UI updates with rollback capabilities.

### 4. Caching and Invalidation
Intelligent caching with configurable invalidation strategies.

## Async Store Architecture

### Core Async Store Interface

```rust
pub trait AsyncStore<T> {
    type Error: std::error::Error + 'static;
    type Params: Clone + 'static;
    
    /// Get current data
    fn data(&self) -> ReadSignal<Option<T>>;
    
    /// Get loading state
    fn loading(&self) -> ReadSignal<bool>;
    
    /// Get error state
    fn error(&self) -> ReadSignal<Option<Self::Error>>;
    
    /// Trigger refetch
    fn refetch(&self);
    
    /// Update parameters and refetch
    fn update_params(&self, params: Self::Params);
    
    /// Mutate data optimistically
    fn mutate(&self, updater: impl FnOnce(&mut T)) -> MutationHandle;
}
```

### Async Store Implementation

```rust
pub struct AsyncStoreImpl<T, P, E> {
    /// Current data state
    data: RwSignal<Option<T>>,
    
    /// Loading state
    loading: RwSignal<bool>,
    
    /// Error state
    error: RwSignal<Option<E>>,
    
    /// Parameters for the async operation
    params: RwSignal<P>,
    
    /// Fetcher function
    fetcher: Arc<dyn Fn(P) -> BoxFuture<'static, Result<T, E>> + Send + Sync>,
    
    /// Configuration
    config: AsyncStoreConfig,
    
    /// Cache
    cache: Arc<RwLock<LruCache<String, CachedEntry<T>>>>,
    
    /// Mutation queue
    mutations: Arc<Mutex<VecDeque<PendingMutation<T>>>>,
    
    /// Abort handle for current request
    abort_handle: RwSignal<Option<AbortHandle>>,
}

impl<T, P, E> AsyncStoreImpl<T, P, E> 
where 
    T: Clone + Send + Sync + 'static,
    P: Clone + Send + Sync + 'static,
    E: Clone + Send + Sync + 'static,
{
    pub fn new<F, Fut>(
        fetcher: F,
        initial_params: P,
        config: AsyncStoreConfig,
    ) -> Self 
    where 
        F: Fn(P) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<T, E>> + Send + 'static,
    {
        let fetcher = Arc::new(move |params: P| {
            let fut = fetcher(params);
            Box::pin(fut) as BoxFuture<'static, Result<T, E>>
        });
        
        let store = Self {
            data: create_rw_signal(None),
            loading: create_rw_signal(false),
            error: create_rw_signal(None),
            params: create_rw_signal(initial_params.clone()),
            fetcher,
            config,
            cache: Arc::new(RwLock::new(LruCache::new(config.cache_size))),
            mutations: Arc::new(Mutex::new(VecDeque::new())),
            abort_handle: create_rw_signal(None),
        };
        
        // Trigger initial fetch
        store.trigger_fetch(initial_params);
        
        store
    }
    
    fn trigger_fetch(&self, params: P) {
        // Cancel any ongoing request
        if let Some(handle) = self.abort_handle.get() {
            handle.abort();
        }
        
        // Check cache first
        if let Some(cached) = self.get_from_cache(&params) {
            self.data.set(Some(cached));
            return;
        }
        
        // Set loading state
        self.loading.set(true);
        self.error.set(None);
        
        let fetcher = Arc::clone(&self.fetcher);
        let data_signal = self.data;
        let loading_signal = self.loading;
        let error_signal = self.error;
        let cache = Arc::clone(&self.cache);
        let config = self.config.clone();
        
        let (abort_handle, abort_registration) = AbortHandle::new_pair();
        self.abort_handle.set(Some(abort_handle));
        
        spawn_local(async move {
            let future = fetcher(params.clone());
            let abortable_future = Abortable::new(future, abort_registration);
            
            match abortable_future.await {
                Ok(Ok(result)) => {
                    data_signal.set(Some(result.clone()));
                    loading_signal.set(false);
                    
                    // Cache the result
                    if config.cache_enabled {
                        let mut cache = cache.write().unwrap();
                        cache.put(
                            cache_key(&params),
                            CachedEntry {
                                data: result,
                                timestamp: std::time::Instant::now(),
                                expires_at: Some(std::time::Instant::now() + config.cache_ttl),
                            }
                        );
                    }
                }
                Ok(Err(err)) => {
                    error_signal.set(Some(err));
                    loading_signal.set(false);
                }
                Err(_aborted) => {
                    // Request was aborted, don't update state
                }
            }
        });
    }
}
```

## Async Store Types

### 1. Resource-Based Store
```rust
pub fn create_async_resource<T, P, E, F, Fut>(
    fetcher: F,
    params: impl Fn() -> P + 'static,
) -> AsyncResource<T, E>
where 
    T: Clone + 'static,
    P: Clone + PartialEq + 'static,
    E: Clone + 'static,
    F: Fn(P) -> Fut + 'static,
    Fut: Future<Output = Result<T, E>> + 'static,
{
    let resource = create_resource(params, fetcher);
    
    AsyncResource {
        data: create_memo(move |_| {
            resource.get().and_then(|r| r.ok())
        }).into(),
        loading: resource.loading(),
        error: create_memo(move |_| {
            resource.get().and_then(|r| r.err())
        }).into(),
        refetch: move || resource.refetch(),
    }
}

pub struct AsyncResource<T, E> {
    pub data: ReadSignal<Option<T>>,
    pub loading: ReadSignal<bool>,
    pub error: ReadSignal<Option<E>>,
    pub refetch: Box<dyn Fn()>,
}
```

### 2. Query Store (React Query-like)
```rust
pub struct QueryStore<T, P, E> {
    key: String,
    fetcher: Arc<dyn Fn(P) -> BoxFuture<'static, Result<T, E>> + Send + Sync>,
    params: RwSignal<P>,
    config: QueryConfig,
    state: QueryState<T, E>,
}

#[derive(Clone)]
pub struct QueryState<T, E> {
    pub data: Option<T>,
    pub error: Option<E>,
    pub status: QueryStatus,
    pub last_updated: Option<std::time::Instant>,
    pub stale_time: std::time::Duration,
}

#[derive(Clone, PartialEq)]
pub enum QueryStatus {
    Idle,
    Loading,
    Success,
    Error,
}

impl<T, P, E> QueryStore<T, P, E> 
where 
    T: Clone + Send + Sync + 'static,
    P: Clone + Send + Sync + 'static,
    E: Clone + Send + Sync + 'static,
{
    pub fn new(
        key: impl Into<String>,
        fetcher: impl Fn(P) -> BoxFuture<'static, Result<T, E>> + Send + Sync + 'static,
        config: QueryConfig,
    ) -> Self {
        Self {
            key: key.into(),
            fetcher: Arc::new(fetcher),
            params: create_rw_signal(config.initial_params.clone()),
            config,
            state: QueryState {
                data: None,
                error: None,
                status: QueryStatus::Idle,
                last_updated: None,
                stale_time: config.stale_time,
            },
        }
    }
    
    pub fn use_query(&self, params: P) -> QueryResult<T, E> {
        // Update params if changed
        if self.params.get() != params {
            self.params.set(params.clone());
            self.invalidate();
        }
        
        // Check if data is stale
        if self.is_stale() {
            self.fetch_if_needed();
        }
        
        QueryResult {
            data: self.state.data.clone(),
            error: self.state.error.clone(),
            is_loading: self.state.status == QueryStatus::Loading,
            is_error: self.state.status == QueryStatus::Error,
            is_stale: self.is_stale(),
            refetch: Box::new(|| self.refetch()),
        }
    }
    
    fn is_stale(&self) -> bool {
        if let Some(last_updated) = self.state.last_updated {
            last_updated.elapsed() > self.state.stale_time
        } else {
            true
        }
    }
}

pub struct QueryResult<T, E> {
    pub data: Option<T>,
    pub error: Option<E>,
    pub is_loading: bool,
    pub is_error: bool,
    pub is_stale: bool,
    pub refetch: Box<dyn Fn()>,
}
```

### 3. Mutation Store
```rust
pub struct MutationStore<T, P, E> {
    mutator: Arc<dyn Fn(P) -> BoxFuture<'static, Result<T, E>> + Send + Sync>,
    state: RwSignal<MutationState<T, E>>,
    config: MutationConfig,
}

#[derive(Clone)]
pub struct MutationState<T, E> {
    pub data: Option<T>,
    pub error: Option<E>,
    pub status: MutationStatus,
    pub variables: Option<String>, // Serialized parameters
}

#[derive(Clone, PartialEq)]
pub enum MutationStatus {
    Idle,
    Loading,
    Success,
    Error,
}

impl<T, P, E> MutationStore<T, P, E> 
where 
    T: Clone + Send + Sync + 'static,
    P: Clone + Send + Sync + 'static,
    E: Clone + Send + Sync + 'static,
{
    pub fn new(
        mutator: impl Fn(P) -> BoxFuture<'static, Result<T, E>> + Send + Sync + 'static,
        config: MutationConfig,
    ) -> Self {
        Self {
            mutator: Arc::new(mutator),
            state: create_rw_signal(MutationState {
                data: None,
                error: None,
                status: MutationStatus::Idle,
                variables: None,
            }),
            config,
        }
    }
    
    pub fn mutate(&self, params: P) -> MutationHandle<T, E> {
        self.state.update(|state| {
            state.status = MutationStatus::Loading;
            state.error = None;
            state.variables = Some(format!("{:?}", params)); // Better serialization needed
        });
        
        let mutator = Arc::clone(&self.mutator);
        let state_signal = self.state;
        let config = self.config.clone();
        
        let (tx, rx) = oneshot::channel();
        
        spawn_local(async move {
            let result = mutator(params).await;
            
            state_signal.update(|state| {
                match &result {
                    Ok(data) => {
                        state.data = Some(data.clone());
                        state.status = MutationStatus::Success;
                    }
                    Err(error) => {
                        state.error = Some(error.clone());
                        state.status = MutationStatus::Error;
                    }
                }
            });
            
            let _ = tx.send(result);
        });
        
        MutationHandle { receiver: rx }
    }
}

pub struct MutationHandle<T, E> {
    receiver: oneshot::Receiver<Result<T, E>>,
}

impl<T, E> MutationHandle<T, E> {
    pub async fn wait(self) -> Result<T, E> {
        self.receiver.await.unwrap_or_else(|_| {
            panic!("Mutation was cancelled")
        })
    }
}
```

## Configuration and Caching

### Async Store Configuration

```rust
#[derive(Clone)]
pub struct AsyncStoreConfig {
    /// Enable caching
    pub cache_enabled: bool,
    
    /// Cache size (number of entries)
    pub cache_size: usize,
    
    /// Cache time-to-live
    pub cache_ttl: std::time::Duration,
    
    /// Stale time (when to consider data stale)
    pub stale_time: std::time::Duration,
    
    /// Retry configuration
    pub retry: RetryConfig,
    
    /// Optimistic updates
    pub optimistic_updates: bool,
    
    /// Background refetch
    pub background_refetch: bool,
    
    /// Refetch on window focus
    pub refetch_on_focus: bool,
    
    /// Dedupe identical requests
    pub dedupe_requests: bool,
}

impl Default for AsyncStoreConfig {
    fn default() -> Self {
        Self {
            cache_enabled: true,
            cache_size: 1000,
            cache_ttl: std::time::Duration::from_secs(300), // 5 minutes
            stale_time: std::time::Duration::from_secs(60), // 1 minute
            retry: RetryConfig::default(),
            optimistic_updates: false,
            background_refetch: true,
            refetch_on_focus: true,
            dedupe_requests: true,
        }
    }
}

#[derive(Clone)]
pub struct RetryConfig {
    pub enabled: bool,
    pub max_attempts: u32,
    pub initial_delay: std::time::Duration,
    pub max_delay: std::time::Duration,
    pub backoff_factor: f64,
    pub retry_condition: Option<Arc<dyn Fn(&dyn std::error::Error) -> bool + Send + Sync>>,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_attempts: 3,
            initial_delay: std::time::Duration::from_millis(1000),
            max_delay: std::time::Duration::from_millis(30000),
            backoff_factor: 2.0,
            retry_condition: None,
        }
    }
}
```

### Cache Implementation

```rust
#[derive(Clone)]
pub struct CachedEntry<T> {
    pub data: T,
    pub timestamp: std::time::Instant,
    pub expires_at: Option<std::time::Instant>,
    pub stale_at: Option<std::time::Instant>,
}

impl<T> CachedEntry<T> {
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            std::time::Instant::now() > expires_at
        } else {
            false
        }
    }
    
    pub fn is_stale(&self) -> bool {
        if let Some(stale_at) = self.stale_at {
            std::time::Instant::now() > stale_at
        } else {
            false
        }
    }
}

pub struct QueryCache<T> {
    entries: Arc<RwLock<LruCache<String, CachedEntry<T>>>>,
    config: CacheConfig,
}

impl<T> QueryCache<T> 
where T: Clone
{
    pub fn new(config: CacheConfig) -> Self {
        Self {
            entries: Arc::new(RwLock::new(LruCache::new(config.max_size))),
            config,
        }
    }
    
    pub fn get(&self, key: &str) -> Option<T> {
        let entries = self.entries.read().unwrap();
        
        if let Some(entry) = entries.peek(key) {
            if !entry.is_expired() {
                return Some(entry.data.clone());
            }
        }
        
        None
    }
    
    pub fn set(&self, key: String, data: T) {
        let mut entries = self.entries.write().unwrap();
        
        let entry = CachedEntry {
            data,
            timestamp: std::time::Instant::now(),
            expires_at: Some(std::time::Instant::now() + self.config.ttl),
            stale_at: Some(std::time::Instant::now() + self.config.stale_time),
        };
        
        entries.put(key, entry);
    }
    
    pub fn invalidate(&self, key: &str) {
        let mut entries = self.entries.write().unwrap();
        entries.pop(key);
    }
    
    pub fn invalidate_all(&self) {
        let mut entries = self.entries.write().unwrap();
        entries.clear();
    }
}
```

## Optimistic Updates

### Optimistic Update System

```rust
pub struct OptimisticUpdater<T> {
    original_data: Arc<Mutex<Option<T>>>,
    pending_mutations: Arc<Mutex<Vec<PendingMutation<T>>>>,
}

#[derive(Clone)]
pub struct PendingMutation<T> {
    pub id: String,
    pub updater: Arc<dyn Fn(&mut T) + Send + Sync>,
    pub rollback: Arc<dyn Fn(&mut T) + Send + Sync>,
    pub timestamp: std::time::Instant,
}

impl<T> OptimisticUpdater<T> 
where T: Clone
{
    pub fn new() -> Self {
        Self {
            original_data: Arc::new(Mutex::new(None)),
            pending_mutations: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    pub fn apply_optimistic_update<F, R>(
        &self,
        data: &mut T,
        updater: F,
        rollback: R,
    ) -> OptimisticHandle
    where 
        F: Fn(&mut T) + Send + Sync + 'static,
        R: Fn(&mut T) + Send + Sync + 'static,
    {
        // Store original data if this is the first optimistic update
        {
            let mut original = self.original_data.lock().unwrap();
            if original.is_none() {
                *original = Some(data.clone());
            }
        }
        
        // Apply the optimistic update
        updater(data);
        
        // Record the mutation
        let mutation_id = uuid::Uuid::new_v4().to_string();
        let mutation = PendingMutation {
            id: mutation_id.clone(),
            updater: Arc::new(updater),
            rollback: Arc::new(rollback),
            timestamp: std::time::Instant::now(),
        };
        
        {
            let mut pending = self.pending_mutations.lock().unwrap();
            pending.push(mutation);
        }
        
        OptimisticHandle {
            mutation_id,
            updater: Arc::clone(&self),
        }
    }
    
    pub fn commit_mutation(&self, mutation_id: &str, data: &mut T) {
        let mut pending = self.pending_mutations.lock().unwrap();
        pending.retain(|m| m.id != mutation_id);
        
        // If no more pending mutations, clear original data
        if pending.is_empty() {
            let mut original = self.original_data.lock().unwrap();
            *original = None;
        }
    }
    
    pub fn rollback_mutation(&self, mutation_id: &str, data: &mut T) {
        let mut pending = self.pending_mutations.lock().unwrap();
        
        if let Some(index) = pending.iter().position(|m| m.id == mutation_id) {
            let mutation = pending.remove(index);
            
            // Rollback this specific mutation
            (mutation.rollback)(data);
            
            // Re-apply all remaining mutations
            for remaining in &pending[index..] {
                (remaining.updater)(data);
            }
        }
    }
    
    pub fn rollback_all(&self, data: &mut T) {
        if let Some(original) = self.original_data.lock().unwrap().take() {
            *data = original;
        }
        
        let mut pending = self.pending_mutations.lock().unwrap();
        pending.clear();
    }
}

pub struct OptimisticHandle {
    mutation_id: String,
    updater: Arc<OptimisticUpdater<T>>,
}

impl OptimisticHandle {
    pub fn commit(self, data: &mut T) {
        self.updater.commit_mutation(&self.mutation_id, data);
    }
    
    pub fn rollback(self, data: &mut T) {
        self.updater.rollback_mutation(&self.mutation_id, data);
    }
}
```

## Error Handling and Retry Logic

### Retry Implementation

```rust
pub struct RetryManager<T, E> {
    config: RetryConfig,
    current_attempt: Arc<AtomicU32>,
}

impl<T, E> RetryManager<T, E> 
where E: std::error::Error + 'static
{
    pub fn new(config: RetryConfig) -> Self {
        Self {
            config,
            current_attempt: Arc::new(AtomicU32::new(0)),
        }
    }
    
    pub async fn execute_with_retry<F, Fut>(
        &self,
        operation: F,
    ) -> Result<T, RetryError<E>>
    where 
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, E>>,
    {
        let mut delay = self.config.initial_delay;
        
        loop {
            let attempt = self.current_attempt.load(std::sync::atomic::Ordering::SeqCst);
            
            match operation().await {
                Ok(result) => {
                    self.current_attempt.store(0, std::sync::atomic::Ordering::SeqCst);
                    return Ok(result);
                }
                Err(error) => {
                    if attempt >= self.config.max_attempts {
                        return Err(RetryError::MaxAttemptsExceeded(error));
                    }
                    
                    // Check if error is retryable
                    if let Some(ref condition) = self.config.retry_condition {
                        if !condition(&error) {
                            return Err(RetryError::NonRetryableError(error));
                        }
                    }
                    
                    // Wait before retry
                    sleep(delay).await;
                    
                    // Update delay for next attempt
                    delay = std::cmp::min(
                        delay.mul_f64(self.config.backoff_factor),
                        self.config.max_delay,
                    );
                    
                    self.current_attempt.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RetryError<E> {
    #[error("Maximum retry attempts exceeded: {0}")]
    MaxAttemptsExceeded(E),
    
    #[error("Non-retryable error: {0}")]
    NonRetryableError(E),
}
```

## Leptos Integration Hooks

### Primary Async Hooks

```rust
pub fn use_async_query<T, P, E, F, Fut>(
    key: impl Into<String>,
    params: impl Fn() -> P + 'static,
    fetcher: F,
) -> QueryResult<T, E>
where 
    T: Clone + 'static,
    P: Clone + PartialEq + 'static,
    E: Clone + 'static,
    F: Fn(P) -> Fut + 'static,
    Fut: Future<Output = Result<T, E>> + 'static,
{
    let query_store = create_query_store(key, fetcher, QueryConfig::default());
    let current_params = create_memo(params);
    
    query_store.use_query(current_params.get())
}

pub fn use_mutation<T, P, E, F, Fut>(
    mutator: F,
) -> (MutationResult<T, E>, impl Fn(P) -> MutationHandle<T, E>)
where 
    T: Clone + 'static,
    P: Clone + 'static,
    E: Clone + 'static,
    F: Fn(P) -> Fut + 'static,
    Fut: Future<Output = Result<T, E>> + 'static,
{
    let mutation_store = create_mutation_store(mutator, MutationConfig::default());
    
    let result = MutationResult {
        data: mutation_store.state.with(|s| s.data.clone()),
        error: mutation_store.state.with(|s| s.error.clone()),
        is_loading: mutation_store.state.with(|s| s.status == MutationStatus::Loading),
        is_error: mutation_store.state.with(|s| s.status == MutationStatus::Error),
    };
    
    let mutate = move |params: P| mutation_store.mutate(params);
    
    (result, mutate)
}

pub fn use_infinite_query<T, P, E, F, Fut>(
    key: impl Into<String>,
    params: impl Fn() -> P + 'static,
    fetcher: F,
) -> InfiniteQueryResult<T, E>
where 
    T: Clone + 'static,
    P: Clone + PartialEq + 'static,
    E: Clone + 'static,
    F: Fn(P) -> Fut + 'static,
    Fut: Future<Output = Result<PaginatedData<T>, E>> + 'static,
{
    // Implementation for infinite/paginated queries
    todo!("Implement infinite query logic")
}
```

This async store design provides comprehensive async state management with sophisticated caching, error handling, and optimistic update capabilities, all while maintaining seamless integration with Leptos reactivity.
