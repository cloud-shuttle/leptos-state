//! Registry functionality for managing collections of items

use super::super::utils_traits::WithId;

/// Registry for managing collections of items
#[derive(Debug)]
pub struct Registry<T: Clone + WithId> {
    /// Items stored in the registry
    items: std::collections::HashMap<String, T>,
    /// Next available ID
    next_id: u64,
}

impl<T: Clone + WithId> Registry<T> {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            items: std::collections::HashMap::new(),
            next_id: 1,
        }
    }

    /// Register a new item and return its ID
    pub fn register(&mut self, mut item: T) -> String {
        let id = format!("item_{}", self.next_id);
        self.next_id += 1;

        // Set the ID on the item if it implements WithId
        item.set_id(id.clone());
        self.items.insert(id.clone(), item);
        id
    }

    /// Get an item by ID
    pub fn get(&self, id: &str) -> Option<&T> {
        self.items.get(id)
    }

    /// Get a mutable reference to an item by ID
    pub fn get_mut(&mut self, id: &str) -> Option<&mut T> {
        self.items.get_mut(id)
    }

    /// Remove an item by ID
    pub fn remove(&mut self, id: &str) -> Option<T> {
        self.items.remove(id)
    }

    /// Check if an item exists
    pub fn contains(&self, id: &str) -> bool {
        self.items.contains_key(id)
    }

    /// Get all item IDs
    pub fn ids(&self) -> Vec<String> {
        self.items.keys().cloned().collect()
    }

    /// Get all items
    pub fn items(&self) -> Vec<&T> {
        self.items.values().collect()
    }

    /// Get all items mutably
    pub fn items_mut(&mut self) -> Vec<&mut T> {
        self.items.values_mut().collect()
    }

    /// Get the number of items
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Clear all items
    pub fn clear(&mut self) {
        self.items.clear();
        self.next_id = 1;
    }

    /// Find items by predicate
    pub fn find<F>(&self, predicate: F) -> Vec<&T>
    where
        F: Fn(&T) -> bool,
    {
        self.items.values().filter(|item| predicate(item)).collect()
    }

    /// Find the first item matching a predicate
    pub fn find_first<F>(&self, predicate: F) -> Option<&T>
    where
        F: Fn(&T) -> bool,
    {
        self.items.values().find(|item| predicate(item))
    }

    /// Update an item by ID
    pub fn update<F>(&mut self, id: &str, updater: F) -> bool
    where
        F: Fn(&mut T),
    {
        if let Some(item) = self.items.get_mut(id) {
            updater(item);
            true
        } else {
            false
        }
    }

    /// Iterate over items
    pub fn iter(&self) -> std::collections::hash_map::Iter<String, T> {
        self.items.iter()
    }

    /// Iterate over items mutably
    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<String, T> {
        self.items.iter_mut()
    }

    /// Convert to iterator
    pub fn into_iter(self) -> std::collections::hash_map::IntoIter<String, T> {
        self.items.into_iter()
    }
}

impl<T: Clone + WithId> Default for Registry<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Observable registry that notifies on changes
#[derive(Debug)]
pub struct ObservableRegistry<T: Clone + WithId> {
    /// Underlying registry
    registry: Registry<T>,
    /// Change listeners
    listeners: Vec<Box<dyn Fn(&RegistryEvent<T>) + Send + Sync>>,
}

impl<T: Clone + WithId> ObservableRegistry<T> {
    /// Create a new observable registry
    pub fn new() -> Self {
        Self {
            registry: Registry::new(),
            listeners: Vec::new(),
        }
    }

    /// Add a change listener
    pub fn add_listener<F>(&mut self, listener: F)
    where
        F: Fn(&RegistryEvent<T>) + Send + Sync + 'static,
    {
        self.listeners.push(Box::new(listener));
    }

    /// Remove all listeners
    pub fn clear_listeners(&mut self) {
        self.listeners.clear();
    }

    /// Register a new item
    pub fn register(&mut self, item: T) -> String {
        let id = self.registry.register(item.clone());
        self.notify_listeners(&RegistryEvent::Added(id.clone(), item));
        id
    }

    /// Remove an item
    pub fn remove(&mut self, id: &str) -> Option<T> {
        let item = self.registry.remove(id);
        if let Some(ref item) = item {
            self.notify_listeners(&RegistryEvent::Removed(id.to_string(), item.clone()));
        }
        item
    }

    /// Update an item
    pub fn update<F>(&mut self, id: &str, updater: F) -> bool
    where
        F: Fn(&mut T),
    {
        let old_item = self.registry.get(id).cloned();
        let updated = self.registry.update(id, updater);

        if updated {
            if let Some(new_item) = self.registry.get(id) {
                if let Some(old_item) = old_item {
                    self.notify_listeners(&RegistryEvent::Updated(
                        id.to_string(),
                        old_item,
                        new_item.clone(),
                    ));
                }
            }
        }

        updated
    }

    /// Clear all items
    pub fn clear(&mut self) {
        let old_items: Vec<_> = self.registry.items().into_iter().cloned().collect();
        self.registry.clear();
        self.notify_listeners(&RegistryEvent::Cleared(old_items));
    }

    fn notify_listeners(&self, event: &RegistryEvent<T>) {
        for listener in &self.listeners {
            listener(event);
        }
    }
}

/// Events that can occur in an observable registry
#[derive(Debug, Clone)]
pub enum RegistryEvent<T> {
    /// Item was added
    Added(String, T),
    /// Item was removed
    Removed(String, T),
    /// Item was updated
    Updated(String, T, T),
    /// All items were cleared
    Cleared(Vec<T>),
}

impl<T: Clone + WithId> std::ops::Deref for ObservableRegistry<T> {
    type Target = Registry<T>;

    fn deref(&self) -> &Self::Target {
        &self.registry
    }
}

impl<T: Clone + WithId> Default for ObservableRegistry<T> {
    fn default() -> Self {
        Self::new()
    }
}
