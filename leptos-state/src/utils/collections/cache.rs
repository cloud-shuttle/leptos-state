//! Caching and priority queue utilities

use std::collections::BinaryHeap;
use std::time::{Duration, Instant};

/// Cache for expensive operations
#[derive(Debug)]
pub struct Cache<K, V> {
    /// Cache entries
    entries: std::collections::HashMap<K, CacheEntry<V>>,
    /// Maximum number of entries
    max_size: usize,
    /// Time-to-live for entries
    ttl: Option<Duration>,
    /// Cache statistics
    stats: CacheStats,
}

#[derive(Debug, Clone)]
pub struct CacheEntry<V> {
    /// Cached value
    value: V,
    /// When this entry was created
    created_at: Instant,
    /// Last access time
    last_accessed: Instant,
    /// Number of times accessed
    access_count: u64,
}

impl<V> CacheEntry<V> {
    /// Create a new cache entry
    pub fn new(value: V) -> Self {
        let now = Instant::now();
        Self {
            value,
            created_at: now,
            last_accessed: now,
            access_count: 1,
        }
    }

    /// Get the cached value
    pub fn value(&self) -> &V {
        &self.value
    }

    /// Get the cached value mutably
    pub fn value_mut(&mut self) -> &mut V {
        self.last_accessed = Instant::now();
        self.access_count += 1;
        &mut self.value
    }

    /// Check if this entry is expired
    pub fn is_expired(&self, ttl: Duration) -> bool {
        self.created_at.elapsed() > ttl
    }

    /// Get the age of this entry
    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }

    /// Get time since last access
    pub fn time_since_access(&self) -> Duration {
        self.last_accessed.elapsed()
    }

    /// Get access count
    pub fn access_count(&self) -> u64 {
        self.access_count
    }
}

impl<K, V> Cache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
{
    /// Create a new cache with unlimited size and no TTL
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
            max_size: usize::MAX,
            ttl: None,
            stats: CacheStats::default(),
        }
    }

    /// Create a new cache with maximum size
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            entries: std::collections::HashMap::new(),
            max_size,
            ttl: None,
            stats: CacheStats::default(),
        }
    }

    /// Create a new cache with TTL
    pub fn with_ttl(ttl: Duration) -> Self {
        Self {
            entries: std::collections::HashMap::new(),
            max_size: usize::MAX,
            ttl: Some(ttl),
            stats: CacheStats::default(),
        }
    }

    /// Create a new cache with size and TTL
    pub fn with_config(max_size: usize, ttl: Option<Duration>) -> Self {
        Self {
            entries: std::collections::HashMap::new(),
            max_size,
            ttl,
            stats: CacheStats::default(),
        }
    }

    /// Get a value from the cache
    pub fn get(&mut self, key: &K) -> Option<&V> {
        // Check TTL first
        if let Some(ttl) = self.ttl {
            if let Some(entry) = self.entries.get(key) {
                if entry.is_expired(ttl) {
                    self.stats.misses += 1;
                    self.entries.remove(key);
                    return None;
                }
            }
        }

        match self.entries.get_mut(key) {
            Some(entry) => {
                self.stats.hits += 1;
                entry.last_accessed = Instant::now();
                entry.access_count += 1;
                Some(&entry.value)
            }
            None => {
                self.stats.misses += 1;
                None
            }
        }
    }

    /// Get a mutable reference to a value in the cache
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        // Check TTL first
        if let Some(ttl) = self.ttl {
            if let Some(entry) = self.entries.get(key) {
                if entry.is_expired(ttl) {
                    self.stats.misses += 1;
                    self.entries.remove(key);
                    return None;
                }
            }
        }

        match self.entries.get_mut(key) {
            Some(entry) => {
                self.stats.hits += 1;
                entry.last_accessed = Instant::now();
                entry.access_count += 1;
                Some(&mut entry.value)
            }
            None => {
                self.stats.misses += 1;
                None
            }
        }
    }

    /// Insert a value into the cache
    pub fn insert(&mut self, key: K, value: V) {
        // Evict if at capacity
        if self.entries.len() >= self.max_size {
            // Simple LRU: remove least recently used
            let mut oldest_key = None;
            let mut oldest_time = Instant::now();

            for (k, entry) in &self.entries {
                if entry.last_accessed < oldest_time {
                    oldest_time = entry.last_accessed;
                    oldest_key = Some(k.clone());
                }
            }

            if let Some(key_to_remove) = oldest_key {
                self.entries.remove(&key_to_remove);
                self.stats.evictions += 1;
            }
        }

        let entry = CacheEntry::new(value);
        self.entries.insert(key, entry);
        self.stats.size = self.entries.len();
    }

    /// Remove a value from the cache
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.entries.remove(key).map(|entry| entry.value)
    }

    /// Check if a key exists in the cache
    pub fn contains(&self, key: &K) -> bool {
        self.entries.contains_key(key)
    }

    /// Clear all entries from the cache
    pub fn clear(&mut self) {
        self.entries.clear();
        self.stats = CacheStats::default();
    }

    /// Get the number of entries in the cache
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get cache statistics
    pub fn stats(&self) -> &CacheStats {
        &self.stats
    }

    /// Clean up expired entries
    pub fn cleanup_expired(&mut self) {
        if let Some(ttl) = self.ttl {
            let mut to_remove = Vec::new();

            for (key, entry) in &self.entries {
                if entry.is_expired(ttl) {
                    to_remove.push(key.clone());
                }
            }

            for key in to_remove {
                self.entries.remove(&key);
                self.stats.evictions += 1;
            }

            self.stats.size = self.entries.len();
        }
    }

    /// Iterate over cache entries
    pub fn iter(&self) -> std::collections::hash_map::Iter<K, CacheEntry<V>> {
        self.entries.iter()
    }

    /// Iterate over cache entries mutably
    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<K, CacheEntry<V>> {
        self.entries.iter_mut()
    }
}

/// Cache statistics
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Number of cache hits
    pub hits: u64,
    /// Number of cache misses
    pub misses: u64,
    /// Current number of entries
    pub size: usize,
    /// Number of evictions
    pub evictions: u64,
}

impl CacheStats {
    /// Calculate hit rate
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }

    /// Get total number of accesses
    pub fn total_accesses(&self) -> u64 {
        self.hits + self.misses
    }
}

/// Priority queue for managing prioritized items
#[derive(Debug)]
pub struct PriorityQueue<T> {
    /// Internal binary heap
    heap: BinaryHeap<PriorityItem<T>>,
    /// Next available ID for tie-breaking
    next_id: u64,
}

/// Item in the priority queue
#[derive(Debug)]
struct PriorityItem<T> {
    /// Item value
    value: T,
    /// Priority (higher = more important)
    priority: i64,
    /// ID for tie-breaking
    id: u64,
}

impl<T> PartialEq for PriorityItem<T> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.id == other.id
    }
}

impl<T> Eq for PriorityItem<T> {}

impl<T> PartialOrd for PriorityItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for PriorityItem<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Higher priority first, then lower ID first (FIFO for same priority)
        other.priority.cmp(&self.priority)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl<T> PriorityQueue<T> {
    /// Create a new priority queue
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            next_id: 0,
        }
    }

    /// Push an item with default priority (0)
    pub fn push(&mut self, value: T) {
        self.push_with_priority(value, 0);
    }

    /// Push an item with specified priority
    pub fn push_with_priority(&mut self, value: T, priority: i64) {
        let item = PriorityItem {
            value,
            priority,
            id: self.next_id,
        };
        self.next_id += 1;
        self.heap.push(item);
    }

    /// Pop the highest priority item
    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|item| item.value)
    }

    /// Peek at the highest priority item without removing it
    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|item| &item.value)
    }

    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Get the number of items in the queue
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Clear all items from the queue
    pub fn clear(&mut self) {
        self.heap.clear();
        self.next_id = 0;
    }

    /// Change the priority of an item (requires removing and re-inserting)
    pub fn change_priority<F>(&mut self, predicate: F, new_priority: i64)
    where
        F: Fn(&T) -> bool,
    {
        let mut items_to_reinsert = Vec::new();

        // Remove matching items
        while let Some(item) = self.heap.pop() {
            if predicate(&item.value) {
                items_to_reinsert.push(item.value);
            } else {
                // Keep non-matching items in a temp vec
                // This is inefficient but necessary for the heap structure
                // In a real implementation, you'd want a more sophisticated approach
                items_to_reinsert.insert(0, item.value);
            }
        }

        // Re-insert all items with new priorities where applicable
        for value in items_to_reinsert {
            self.push_with_priority(value, new_priority);
        }
    }
}

impl<T> Default for PriorityQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}
