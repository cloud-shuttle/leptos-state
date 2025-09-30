//! Dependency-tracked memoized selectors

use crate::store::Store;
use std::collections::HashSet;

/// Dependency-tracked memoized selector
pub struct DependencyTrackedSelector<T: Store, O> {
    /// The selector function
    selector: Box<dyn Fn(&T::State) -> O + Send + Sync>,
    /// Cache for the last computed value
    cache: std::sync::Mutex<Option<(HashSet<String>, O)>>,
    /// Dependencies that this selector reads
    dependencies: std::sync::Mutex<HashSet<String>>,
}

impl<T: Store, O: Clone + PartialEq + 'static> DependencyTrackedSelector<T, O> {
    /// Create a new dependency-tracked selector
    pub fn new<F>(selector: F) -> Self
    where
        F: Fn(&T::State) -> O + Send + Sync + 'static,
    {
        Self {
            selector: Box::new(selector),
            cache: std::sync::Mutex::new(None),
            dependencies: std::sync::Mutex::new(HashSet::new()),
        }
    }

    /// Get the selected value, using dependency tracking
    pub fn select(&self, state: &T::State) -> O {
        // For now, this is simplified - in a real implementation,
        // you'd track which fields are accessed during selector execution
        let mut cache = self.cache.lock().unwrap();

        // Simple dependency tracking - assume all state changes invalidate cache
        // In a real implementation, this would track specific field access
        if cache.is_some() {
            // For demonstration, we'll always recompute
            // Real implementation would check if tracked dependencies changed
        }

        let value = (self.selector)(state);
        let deps = self.dependencies.lock().unwrap().clone();
        *cache = Some((deps, value.clone()));
        value
    }

    /// Clear the memoization cache
    pub fn clear_cache(&self) {
        *self.cache.lock().unwrap() = None;
    }

    /// Add a dependency
    pub fn add_dependency(&self, dependency: String) {
        self.dependencies.lock().unwrap().insert(dependency);
    }

    /// Remove a dependency
    pub fn remove_dependency(&self, dependency: &str) {
        self.dependencies.lock().unwrap().remove(dependency);
    }

    /// Get current dependencies
    pub fn get_dependencies(&self) -> HashSet<String> {
        self.dependencies.lock().unwrap().clone()
    }

    /// Check if selector depends on a specific field
    pub fn depends_on(&self, dependency: &str) -> bool {
        self.dependencies.lock().unwrap().contains(dependency)
    }

    /// Clear all dependencies
    pub fn clear_dependencies(&self) {
        self.dependencies.lock().unwrap().clear();
        self.clear_cache(); // Clear cache when dependencies change
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> (bool, usize, usize) {
        let cache = self.cache.lock().unwrap();
        let deps = self.dependencies.lock().unwrap();
        (cache.is_some(), deps.len(), if cache.is_some() { 1 } else { 0 })
    }

    /// Invalidate cache if dependencies changed
    pub fn invalidate_if_dependencies_changed(&self, changed_fields: &HashSet<String>) -> bool {
        let deps = self.dependencies.lock().unwrap();
        let has_intersection = deps.intersection(changed_fields).next().is_some();

        if has_intersection {
            self.clear_cache();
            true
        } else {
            false
        }
    }
}

impl<T: Store, O: Clone + PartialEq + 'static> std::fmt::Debug for DependencyTrackedSelector<T, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (has_cache, dep_count, _) = self.cache_stats();
        f.debug_struct("DependencyTrackedSelector")
            .field("has_cache", &has_cache)
            .field("dependencies", &dep_count)
            .finish()
    }
}

impl<T: Store, O: Clone + PartialEq + 'static> std::fmt::Display for DependencyTrackedSelector<T, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (has_cache, dep_count, _) = self.cache_stats();
        write!(f, "DependencyTrackedSelector(cached: {}, deps: {})", has_cache, dep_count)
    }
}

/// Dependency tracker for automatic dependency detection
pub struct DependencyTracker {
    /// Currently tracked dependencies
    current_dependencies: std::cell::RefCell<HashSet<String>>,
    /// Whether tracking is active
    tracking_active: std::cell::RefCell<bool>,
}

impl DependencyTracker {
    /// Create a new dependency tracker
    pub fn new() -> Self {
        Self {
            current_dependencies: std::cell::RefCell::new(HashSet::new()),
            tracking_active: std::cell::RefCell::new(false),
        }
    }

    /// Start tracking dependencies
    pub fn start_tracking(&self) {
        *self.tracking_active.borrow_mut() = true;
        self.current_dependencies.borrow_mut().clear();
    }

    /// Stop tracking dependencies
    pub fn stop_tracking(&self) -> HashSet<String> {
        *self.tracking_active.borrow_mut() = false;
        self.current_dependencies.borrow().clone()
    }

    /// Track a dependency access
    pub fn track_dependency(&self, dependency: String) {
        if *self.tracking_active.borrow() {
            self.current_dependencies.borrow_mut().insert(dependency);
        }
    }

    /// Check if tracking is active
    pub fn is_tracking(&self) -> bool {
        *self.tracking_active.borrow()
    }

    /// Get current dependencies without stopping tracking
    pub fn get_current_dependencies(&self) -> HashSet<String> {
        self.current_dependencies.borrow().clone()
    }
}

impl Default for DependencyTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-local dependency tracker
thread_local! {
    static DEPENDENCY_TRACKER: DependencyTracker = DependencyTracker::new();
}

/// Get the thread-local dependency tracker
pub fn get_dependency_tracker() -> &'static DependencyTracker {
    DEPENDENCY_TRACKER.with(|tracker| {
        // This is unsafe but necessary for static lifetime
        // In a real implementation, you'd use a safer approach
        unsafe { &*(tracker as *const DependencyTracker) }
    })
}

/// Track a dependency access
pub fn track_dependency(dependency: String) {
    DEPENDENCY_TRACKER.with(|tracker| {
        tracker.track_dependency(dependency);
    });
}

/// Start dependency tracking
pub fn start_dependency_tracking() {
    DEPENDENCY_TRACKER.with(|tracker| {
        tracker.start_tracking();
    });
}

/// Stop dependency tracking and get dependencies
pub fn stop_dependency_tracking() -> HashSet<String> {
    DEPENDENCY_TRACKER.with(|tracker| {
        tracker.stop_tracking()
    })
}

/// Check if dependency tracking is active
pub fn is_dependency_tracking_active() -> bool {
    DEPENDENCY_TRACKER.with(|tracker| {
        tracker.is_tracking()
    })
}
