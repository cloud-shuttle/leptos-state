//! Collection utilities for managing multiple stores/machines

/// Collection utilities for managing multiple stores/machines
#[derive(Debug)]
pub struct CollectionUtils;

impl CollectionUtils {
    /// Group items by a key function
    pub fn group_by<T, K, F>(items: Vec<T>, key_fn: F) -> std::collections::HashMap<K, Vec<T>>
    where
        K: Eq + std::hash::Hash,
        F: Fn(&T) -> K,
    {
        let mut groups = std::collections::HashMap::new();

        for item in items {
            let key = key_fn(&item);
            groups.entry(key).or_insert_with(Vec::new).push(item);
        }

        groups
    }

    /// Filter items by a predicate
    pub fn filter<T, F>(items: Vec<T>, predicate: F) -> Vec<T>
    where
        F: Fn(&T) -> bool,
    {
        items.into_iter().filter(predicate).collect()
    }

    /// Map items using a function
    pub fn map<T, U, F>(items: Vec<T>, mapper: F) -> Vec<U>
    where
        F: Fn(T) -> U,
    {
        items.into_iter().map(mapper).collect()
    }

    /// Find the first item matching a predicate
    pub fn find<T, F>(items: &[T], predicate: F) -> Option<&T>
    where
        F: Fn(&T) -> bool,
    {
        items.iter().find(|item| predicate(item))
    }

    /// Check if any item matches a predicate
    pub fn any<T, F>(items: &[T], predicate: F) -> bool
    where
        F: Fn(&T) -> bool,
    {
        items.iter().any(predicate)
    }

    /// Check if all items match a predicate
    pub fn all<T, F>(items: &[T], predicate: F) -> bool
    where
        F: Fn(&T) -> bool,
    {
        items.iter().all(predicate)
    }

    /// Get unique items
    pub fn unique<T: Eq + std::hash::Hash + Clone>(items: Vec<T>) -> Vec<T> {
        let mut seen = std::collections::HashSet::new();
        items.into_iter()
            .filter(|item| seen.insert(item.clone()))
            .collect()
    }

    /// Sort items by a comparison function
    pub fn sort_by<T, F>(mut items: Vec<T>, compare: F) -> Vec<T>
    where
        F: Fn(&T, &T) -> std::cmp::Ordering,
    {
        items.sort_by(compare);
        items
    }

    /// Partition items into two groups
    pub fn partition<T, F>(items: Vec<T>, predicate: F) -> (Vec<T>, Vec<T>)
    where
        F: Fn(&T) -> bool,
    {
        let mut matching = Vec::new();
        let mut non_matching = Vec::new();

        for item in items {
            if predicate(&item) {
                matching.push(item);
            } else {
                non_matching.push(item);
            }
        }

        (matching, non_matching)
    }
}

/// Registry for managing collections of items
pub struct Registry<T: Clone + WithId> {
    /// Items in the registry
    pub items: std::sync::RwLock<std::collections::HashMap<String, T>>,
}

impl<T: Clone + WithId> Registry<T> {
    /// Create a new registry
    pub fn new() -> Self {
        Self {
            items: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// Register an item
    pub fn register(&self, item: T) -> Result<(), String> {
        let id = item.id().to_string();
        let mut items = self.items.write().unwrap();

        if items.contains_key(&id) {
            return Err(format!("Item with ID '{}' already exists", id));
        }

        items.insert(id, item);
        Ok(())
    }

    /// Unregister an item
    pub fn unregister(&self, id: &str) -> Result<T, String> {
        let mut items = self.items.write().unwrap();
        items.remove(id)
            .ok_or_else(|| format!("Item with ID '{}' not found", id))
    }

    /// Get an item by ID
    pub fn get(&self, id: &str) -> Option<T> {
        self.items.read().unwrap().get(id).cloned()
    }

    /// Check if an item exists
    pub fn contains(&self, id: &str) -> bool {
        self.items.read().unwrap().contains_key(id)
    }

    /// Get all items
    pub fn all(&self) -> Vec<T> {
        self.items.read().unwrap().values().cloned().collect()
    }

    /// Get all item IDs
    pub fn ids(&self) -> Vec<String> {
        self.items.read().unwrap().keys().cloned().collect()
    }

    /// Get the number of items
    pub fn len(&self) -> usize {
        self.items.read().unwrap().len()
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.items.read().unwrap().is_empty()
    }

    /// Clear all items
    pub fn clear(&self) {
        self.items.write().unwrap().clear();
    }

    /// Filter items by a predicate
    pub fn filter<F>(&self, predicate: F) -> Vec<T>
    where
        F: Fn(&T) -> bool,
    {
        self.items.read().unwrap().values()
            .filter(|item| predicate(item))
            .cloned()
            .collect()
    }

    /// Find the first item matching a predicate
    pub fn find<F>(&self, predicate: F) -> Option<T>
    where
        F: Fn(&T) -> bool,
    {
        self.items.read().unwrap().values()
            .find(|item| predicate(item))
            .cloned()
    }
}

// Re-export WithId for the Registry
use super::utils_traits::WithId;

/// Observable registry that notifies on changes
pub struct ObservableRegistry<T: Clone + WithId> {
    /// The underlying registry
    pub registry: Registry<T>,
    /// Change listeners
    pub listeners: std::sync::RwLock<Vec<Box<dyn Fn(&RegistryChange<T>) + Send + Sync>>>,
}

#[derive(Debug, Clone)]
pub enum RegistryChange<T> {
    /// Item was added
    Added(T),
    /// Item was removed
    Removed(String),
    /// Item was updated
    Updated(T),
    /// Registry was cleared
    Cleared,
}

impl<T: Clone + WithId> ObservableRegistry<T> {
    /// Create a new observable registry
    pub fn new() -> Self {
        Self {
            registry: Registry::new(),
            listeners: std::sync::RwLock::new(Vec::new()),
        }
    }

    /// Add a change listener
    pub fn add_listener<F>(&self, listener: F)
    where
        F: Fn(&RegistryChange<T>) + Send + Sync + 'static,
    {
        self.listeners.write().unwrap().push(Box::new(listener));
    }

    /// Register an item and notify listeners
    pub fn register(&self, item: T) -> Result<(), String> {
        self.registry.register(item.clone())?;
        self.notify(RegistryChange::Added(item));
        Ok(())
    }

    /// Unregister an item and notify listeners
    pub fn unregister(&self, id: &str) -> Result<T, String> {
        let item = self.registry.unregister(id)?;
        self.notify(RegistryChange::Removed(id.to_string()));
        Ok(item)
    }

    /// Update an item and notify listeners
    pub fn update<F>(&self, id: &str, updater: F) -> Result<(), String>
    where
        F: FnOnce(&mut T),
    {
        let mut item = self.registry.get(id)
            .ok_or_else(|| format!("Item with ID '{}' not found", id))?;

        updater(&mut item);

        // Re-register with updated item
        let mut items = self.registry.items.write().unwrap();
        items.insert(id.to_string(), item.clone());

        self.notify(RegistryChange::Updated(item));
        Ok(())
    }

    /// Clear all items and notify listeners
    pub fn clear(&self) {
        self.registry.clear();
        self.notify(RegistryChange::Cleared);
    }

    /// Notify all listeners of a change
    fn notify(&self, change: RegistryChange<T>) {
        let listeners = self.listeners.read().unwrap();
        for listener in listeners.iter() {
            listener(&change);
        }
    }

    /// Get the underlying registry
    pub fn inner(&self) -> &Registry<T> {
        &self.registry
    }
}

impl<T: Clone + WithId> std::ops::Deref for ObservableRegistry<T> {
    type Target = Registry<T>;

    fn deref(&self) -> &Self::Target {
        &self.registry
    }
}

/// Cache for expensive operations
pub struct Cache<K, V> {
    /// Cache storage
    pub storage: std::sync::RwLock<std::collections::HashMap<K, CacheEntry<V>>>,
    /// Maximum cache size
    pub max_size: usize,
    /// Default TTL
    pub default_ttl: Option<std::time::Duration>,
}

#[derive(Debug, Clone)]
pub struct CacheEntry<V> {
    /// Cached value
    pub value: V,
    /// Insertion time
    pub inserted_at: std::time::Instant,
    /// Time to live
    pub ttl: Option<std::time::Duration>,
    /// Access count
    pub access_count: usize,
}

impl<V> CacheEntry<V> {
    /// Check if this entry is expired
    pub fn is_expired(&self, current_time: std::time::Instant) -> bool {
        if let Some(ttl) = self.ttl {
            current_time.duration_since(self.inserted_at) >= ttl
        } else {
            false
        }
    }
}

impl<K, V> Cache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    /// Create a new cache
    pub fn new(max_size: usize) -> Self {
        Self {
            storage: std::sync::RwLock::new(std::collections::HashMap::new()),
            max_size,
            default_ttl: None,
        }
    }

    /// Create a cache with TTL
    pub fn with_ttl(max_size: usize, default_ttl: std::time::Duration) -> Self {
        Self {
            storage: std::sync::RwLock::new(std::collections::HashMap::new()),
            max_size,
            default_ttl: Some(default_ttl),
        }
    }

    /// Get a value from the cache
    pub fn get(&self, key: &K) -> Option<V> {
        let mut storage = self.storage.write().unwrap();
        let current_time = std::time::Instant::now();

        if let Some(entry) = storage.get_mut(key) {
            if entry.is_expired(current_time) {
                storage.remove(key);
                return None;
            }

            entry.access_count += 1;
            Some(entry.value.clone())
        } else {
            None
        }
    }

    /// Insert a value into the cache
    pub fn insert(&self, key: K, value: V) {
        self.insert_with_ttl(key, value, self.default_ttl);
    }

    /// Insert a value with custom TTL
    pub fn insert_with_ttl(&self, key: K, value: V, ttl: Option<std::time::Duration>) {
        let mut storage = self.storage.write().unwrap();
        let entry = CacheEntry {
            value,
            inserted_at: std::time::Instant::now(),
            ttl,
            access_count: 0,
        };

        storage.insert(key, entry);

        // Enforce max size (simple LRU-like eviction)
        if storage.len() > self.max_size {
            // Remove least recently accessed item
            let mut items: Vec<_> = storage.iter().collect();
            items.sort_by_key(|(_, entry)| entry.access_count);
            if let Some((key_to_remove, _)) = items.first() {
                storage.remove(key_to_remove);
            }
        }
    }

    /// Remove a value from the cache
    pub fn remove(&self, key: &K) -> Option<V> {
        self.storage.write().unwrap().remove(key).map(|entry| entry.value)
    }

    /// Clear the cache
    pub fn clear(&self) {
        self.storage.write().unwrap().clear();
    }

    /// Get cache size
    pub fn len(&self) -> usize {
        self.storage.read().unwrap().len()
    }

    /// Check if cache contains a key
    pub fn contains(&self, key: &K) -> bool {
        let storage = self.storage.read().unwrap();
        let current_time = std::time::Instant::now();

        if let Some(entry) = storage.get(key) {
            !entry.is_expired(current_time)
        } else {
            false
        }
    }

    /// Cleanup expired entries
    pub fn cleanup(&self) {
        let mut storage = self.storage.write().unwrap();
        let current_time = std::time::Instant::now();

        storage.retain(|_, entry| !entry.is_expired(current_time));
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let storage = self.storage.read().unwrap();
        let total_entries = storage.len();
        let expired_entries = storage.values()
            .filter(|entry| entry.is_expired(std::time::Instant::now()))
            .count();
        let total_accesses: usize = storage.values().map(|entry| entry.access_count).sum();

        CacheStats {
            total_entries,
            expired_entries,
            total_accesses,
            hit_rate: if total_accesses > 0 {
                (total_accesses - expired_entries) as f64 / total_accesses as f64
            } else {
                0.0
            },
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Total entries in cache
    pub total_entries: usize,
    /// Number of expired entries
    pub expired_entries: usize,
    /// Total access count
    pub total_accesses: usize,
    /// Cache hit rate
    pub hit_rate: f64,
}

/// Priority queue for managing prioritized items
pub struct PriorityQueue<T> {
    /// Items organized by priority
    pub queues: std::sync::RwLock<std::collections::BTreeMap<i32, Vec<T>>>,
}

impl<T> PriorityQueue<T> {
    /// Create a new priority queue
    pub fn new() -> Self {
        Self {
            queues: std::sync::RwLock::new(std::collections::BTreeMap::new()),
        }
    }

    /// Push an item with priority
    pub fn push(&self, item: T, priority: i32) {
        self.queues.write().unwrap()
            .entry(priority)
            .or_insert_with(Vec::new)
            .push(item);
    }

    /// Pop the highest priority item
    pub fn pop(&self) -> Option<T> {
        let mut queues = self.queues.write().unwrap();

        if let Some((_, queue)) = queues.iter_mut().next() {
            if let Some(item) = queue.pop() {
                // Remove empty priority levels
                if queue.is_empty() {
                    let priority_to_remove = *queues.keys().next().unwrap();
                    queues.remove(&priority_to_remove);
                }
                return Some(item);
            }
        }

        None
    }

    /// Peek at the highest priority item
    pub fn peek(&self) -> Option<&T> {
        let queues = self.queues.read().unwrap();

        if let Some((_, queue)) = queues.iter().next() {
            queue.last()
        } else {
            None
        }
    }

    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.queues.read().unwrap().is_empty()
    }

    /// Get the number of items
    pub fn len(&self) -> usize {
        self.queues.read().unwrap().values().map(|q| q.len()).sum()
    }

    /// Clear all items
    pub fn clear(&self) {
        self.queues.write().unwrap().clear();
    }
}

/// Event bus for decoupled communication
pub struct EventBus<E> {
    /// Event listeners
    pub listeners: std::sync::RwLock<std::collections::HashMap<String, Vec<Box<dyn Fn(&E) + Send + Sync>>>>,
}

impl<E> EventBus<E> {
    /// Create a new event bus
    pub fn new() -> Self {
        Self {
            listeners: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }

    /// Subscribe to an event type
    pub fn subscribe<F>(&self, event_type: &str, listener: F)
    where
        F: Fn(&E) + Send + Sync + 'static,
    {
        self.listeners.write().unwrap()
            .entry(event_type.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(listener));
    }

    /// Unsubscribe from an event type
    pub fn unsubscribe(&self, event_type: &str) {
        self.listeners.write().unwrap().remove(event_type);
    }

    /// Publish an event
    pub fn publish(&self, event_type: &str, event: &E) {
        if let Some(listeners) = self.listeners.read().unwrap().get(event_type) {
            for listener in listeners {
                listener(event);
            }
        }
    }

    /// Get the number of listeners for an event type
    pub fn listener_count(&self, event_type: &str) -> usize {
        self.listeners.read().unwrap()
            .get(event_type)
            .map(|listeners| listeners.len())
            .unwrap_or(0)
    }

    /// Clear all listeners
    pub fn clear(&self) {
        self.listeners.write().unwrap().clear();
    }
}
