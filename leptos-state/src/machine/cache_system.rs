//! Cache system for performance optimization

use super::*;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant};

/// Cache statistics
#[derive(Debug, Clone, PartialEq)]
pub struct CacheStats {
    /// Total cache hits
    pub hits: usize,
    /// Total cache misses
    pub misses: usize,
    /// Total entries in cache
    pub entries: usize,
    /// Cache size in bytes
    pub size_bytes: usize,
    /// Hit rate (0.0 to 1.0)
    pub hit_rate: f64,
    /// Average access time
    pub avg_access_time: Duration,
    /// Cache evictions
    pub evictions: usize,
}

impl Default for CacheStats {
    fn default() -> Self {
        Self {
            hits: 0,
            misses: 0,
            entries: 0,
            size_bytes: 0,
            hit_rate: 0.0,
            avg_access_time: Duration::from_nanos(0),
            evictions: 0,
        }
    }
}

impl CacheStats {
    /// Record a cache hit
    pub fn record_hit(&mut self) {
        self.hits += 1;
        self.update_hit_rate();
    }

    /// Record a cache miss
    pub fn record_miss(&mut self) {
        self.misses += 1;
        self.update_hit_rate();
    }

    /// Record a cache eviction
    pub fn record_eviction(&mut self) {
        self.evictions += 1;
    }

    /// Update hit rate
    fn update_hit_rate(&mut self) {
        let total = self.hits + self.misses;
        if total > 0 {
            self.hit_rate = self.hits as f64 / total as f64;
        }
    }

    /// Get cache summary
    pub fn summary(&self) -> String {
        format!(
            "Cache Stats: {} hits, {} misses, {:.1}% hit rate, {} entries, {} bytes",
            self.hits,
            self.misses,
            self.hit_rate * 100.0,
            self.entries,
            self.size_bytes
        )
    }
}

/// Memory usage tracker
#[derive(Debug, Clone)]
pub struct MemoryTracker {
    /// Current memory usage
    pub current_usage: usize,
    /// Peak memory usage
    pub peak_usage: usize,
    /// Memory allocations
    pub allocations: usize,
    /// Memory deallocations
    pub deallocations: usize,
    /// Allocation size samples
    pub allocation_sizes: Vec<usize>,
}

impl MemoryTracker {
    /// Create a new memory tracker
    pub fn new() -> Self {
        Self {
            current_usage: 0,
            peak_usage: 0,
            allocations: 0,
            deallocations: 0,
            allocation_sizes: Vec::new(),
        }
    }

    /// Record an allocation
    pub fn record_allocation(&mut self, size: usize) {
        self.current_usage += size;
        self.allocations += 1;
        self.allocation_sizes.push(size);

        if self.current_usage > self.peak_usage {
            self.peak_usage = self.current_usage;
        }
    }

    /// Record a deallocation
    pub fn record_deallocation(&mut self, size: usize) {
        self.current_usage = self.current_usage.saturating_sub(size);
        self.deallocations += 1;
    }

    /// Get average allocation size
    pub fn average_allocation_size(&self) -> usize {
        if self.allocation_sizes.is_empty() {
            0
        } else {
            self.allocation_sizes.iter().sum::<usize>() / self.allocation_sizes.len()
        }
    }

    /// Check if memory usage is above threshold
    pub fn is_above_threshold(&self, threshold: usize) -> bool {
        self.current_usage > threshold
    }

    /// Get memory usage summary
    pub fn summary(&self) -> String {
        format!(
            "Memory: {} current, {} peak, {} allocations, {} deallocations, {} avg size",
            self.current_usage,
            self.peak_usage,
            self.allocations,
            self.deallocations,
            self.average_allocation_size()
        )
    }
}

/// Transition cache for performance optimization
pub struct TransitionCache<C: Send + Sync + Clone + 'static, E> {
    /// Cache storage
    cache: HashMap<CacheKey<C, E>, CachedTransition<C>>,
    /// Cache statistics
    stats: CacheStats,
    /// Memory tracker
    memory_tracker: MemoryTracker,
    /// Maximum cache size
    max_size: usize,
    /// Cache TTL
    ttl: Duration,
    /// Cache creation time
    created_at: Instant,
}

impl<C: Send + Sync + Clone + 'static, E> TransitionCache<C, E> {
    /// Create a new transition cache
    pub fn new(max_size: usize, ttl: Duration) -> Self {
        Self {
            cache: HashMap::new(),
            stats: CacheStats::default(),
            memory_tracker: MemoryTracker::new(),
            max_size,
            ttl,
            created_at: Instant::now(),
        }
    }

    /// Get a cached transition result
    pub fn get(&mut self, key: &CacheKey<C, E>) -> Option<&CachedTransition<C>> {
        if let Some(cached) = self.cache.get(key) {
            // Check if cache entry is expired
            if self.created_at.elapsed() > self.ttl {
                self.stats.record_miss();
                None
            } else {
                self.stats.record_hit();
                Some(cached)
            }
        } else {
            self.stats.record_miss();
            None
        }
    }

    /// Insert a transition result into the cache
    pub fn insert(&mut self, key: CacheKey<C, E>, value: CachedTransition<C>) {
        // Check if we need to evict entries
        if self.cache.len() >= self.max_size {
            // Simple LRU eviction - remove oldest entry
            if let Some(key_to_remove) = self.cache.keys().next().cloned() {
                if let Some(removed) = self.cache.remove(&key_to_remove) {
                    self.memory_tracker.record_deallocation(removed.size_bytes());
                    self.stats.record_eviction();
                }
            }
        }

        let size = value.size_bytes();
        self.memory_tracker.record_allocation(size);
        self.cache.insert(key, value);
        self.stats.entries = self.cache.len();
        self.stats.size_bytes = self.memory_tracker.current_usage;
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.stats = CacheStats::default();
        self.memory_tracker = MemoryTracker::new();
    }

    /// Get cache statistics
    pub fn stats(&self) -> &CacheStats {
        &self.stats
    }

    /// Get memory usage
    pub fn memory_usage(&self) -> usize {
        self.memory_tracker.current_usage
    }

    /// Check if cache should be cleaned up (expired entries)
    pub fn needs_cleanup(&self) -> bool {
        self.created_at.elapsed() > self.ttl
    }

    /// Clean up expired entries
    pub fn cleanup(&mut self) {
        let now = Instant::now();
        let ttl = self.ttl;

        self.cache.retain(|_, cached| {
            (now - cached.created_at) < ttl
        });

        self.stats.entries = self.cache.len();
    }
}

/// Cache key for transitions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheKey<C: Send + Sync + Clone + 'static, E> {
    /// Current state value
    pub state_value: String,
    /// Event that triggered the transition
    pub event: String,
    /// Context hash (simplified)
    pub context_hash: u64,
}

impl<C: Send + Sync + Clone + 'static, E> CacheKey<C, E> {
    /// Create a new cache key
    pub fn new(state_value: String, event: E, context: &C) -> Self
    where
        E: Hash,
        C: Hash,
    {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        context.hash(&mut hasher);
        let context_hash = hasher.finish();

        let event_hash = {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            event.hash(&mut hasher);
            hasher.finish()
        };

        Self {
            state_value,
            event: format!("{:?}", event_hash), // Simplified event representation
            context_hash,
        }
    }
}

/// Cached transition result
#[derive(Debug, Clone)]
pub struct CachedTransition<C: Send + Sync + Clone + 'static> {
    /// Resulting state
    pub result_state: MachineStateImpl<C>,
    /// Time when this entry was cached
    pub created_at: Instant,
    /// Access count
    pub access_count: usize,
    /// Last accessed time
    pub last_accessed: Instant,
}

impl<C: Send + Sync + Clone + 'static> CachedTransition<C> {
    /// Create a new cached transition
    pub fn new(result_state: MachineStateImpl<C>) -> Self {
        let now = Instant::now();
        Self {
            result_state,
            created_at: now,
            access_count: 1,
            last_accessed: now,
        }
    }

    /// Record access
    pub fn record_access(&mut self) {
        self.access_count += 1;
        self.last_accessed = Instant::now();
    }

    /// Get approximate size in bytes
    pub fn size_bytes(&self) -> usize {
        // Rough estimate - this would be more accurate with actual measurement
        std::mem::size_of::<Self>() + self.result_state.value.to_string().len()
    }

    /// Check if entry is expired
    pub fn is_expired(&self, ttl: Duration) -> bool {
        self.created_at.elapsed() > ttl
    }
}
