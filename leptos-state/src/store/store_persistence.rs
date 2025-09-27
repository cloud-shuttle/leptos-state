//! Store persistence functionality

use super::*;

/// Load state from localStorage
pub fn load_from_local_storage<T: serde::de::DeserializeOwned>(key: &str) -> Option<T> {
    use leptos::window;

    #[cfg(feature = "hydrate")]
    {
        let window = window();
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(value)) = storage.get_item(key) {
                if let Ok(parsed) = serde_json::from_str(&value) {
                    return Some(parsed);
                }
            }
        }
    }

    #[cfg(not(feature = "hydrate"))]
    {
        // In SSR mode, return None
        return None;
    }

    None
}

/// Save state to localStorage
pub fn save_to_local_storage<T: serde::Serialize>(key: &str, value: &T) -> Result<(), String> {
    use leptos::window;

    #[cfg(feature = "hydrate")]
    {
        let window = window();
        match window.local_storage() {
            Ok(Some(storage)) => {
                match serde_json::to_string(value) {
                    Ok(json) => {
                        storage.set_item(key, &json)
                            .map_err(|e| format!("Failed to save to localStorage: {:?}", e))
                    }
                    Err(e) => Err(format!("Failed to serialize state: {}", e)),
                }
            }
            _ => Err("localStorage not available".to_string()),
        }
    }

    #[cfg(not(feature = "hydrate"))]
    {
        // In SSR mode, do nothing
        Ok(())
    }
}

/// Create an effect that persists store changes to localStorage
pub fn persist_to_local_storage<T: Clone + PartialEq + serde::Serialize + 'static>(
    key: &str,
    store: StoreContext<T>,
) {
    use leptos::create_effect;

    let key = key.to_string();

    create_effect(move |_| {
        let _current = store.get(); // This will trigger the effect when state changes
        let value = store.get();

        if let Err(e) = save_to_local_storage(&key, &value) {
            leptos::logging::error!("Failed to persist store: {}", e);
        }
    });
}

/// Clear a key from localStorage
pub fn clear_from_local_storage(key: &str) -> Result<(), String> {
    use leptos::window;

    #[cfg(feature = "hydrate")]
    {
        let window = window();
        match window.local_storage() {
            Ok(Some(storage)) => {
                storage.remove_item(key)
                    .map_err(|e| format!("Failed to clear from localStorage: {:?}", e))
            }
            _ => Err("localStorage not available".to_string()),
        }
    }

    #[cfg(not(feature = "hydrate"))]
    {
        // In SSR mode, do nothing
        Ok(())
    }
}

/// Check if localStorage is available
pub fn is_local_storage_available() -> bool {
    #[cfg(feature = "hydrate")]
    {
        use leptos::window;
        window.local_storage().is_ok()
    }

    #[cfg(not(feature = "hydrate"))]
    {
        false
    }
}

/// Persistence middleware that automatically saves changes
pub struct PersistenceMiddleware<T: Clone + PartialEq + serde::Serialize + 'static> {
    store: StoreContext<T>,
    key: String,
}

impl<T: Clone + PartialEq + serde::Serialize + 'static> PersistenceMiddleware<T> {
    /// Create a new persistence middleware
    pub fn new(store: StoreContext<T>, key: String) -> Self {
        // Set up the persistence effect
        persist_to_local_storage(&key, store.clone());

        Self { store, key }
    }

    /// Get the store
    pub fn store(&self) -> &StoreContext<T> {
        &self.store
    }

    /// Manually save the current state
    pub fn save(&self) -> Result<(), String> {
        let value = self.store.get();
        save_to_local_storage(&self.key, &value)
    }

    /// Manually load and set the state
    pub fn load(&self) -> Result<(), String> {
        if let Some(value) = load_from_local_storage(&self.key) {
            self.store.set(value);
            Ok(())
        } else {
            Err("No saved state found".to_string())
        }
    }

    /// Clear the saved state
    pub fn clear(&self) -> Result<(), String> {
        clear_from_local_storage(&self.key)
    }
}

/// Migration function type for handling schema changes
pub type MigrationFn<T> = Box<dyn Fn(T) -> T + Send + Sync>;

/// Migration manager for handling store schema changes
pub struct MigrationManager<T: Clone + PartialEq + 'static> {
    migrations: Vec<MigrationFn<T>>,
    current_version: usize,
}

impl<T: Clone + PartialEq + 'static> MigrationManager<T> {
    /// Create a new migration manager
    pub fn new() -> Self {
        Self {
            migrations: Vec::new(),
            current_version: 0,
        }
    }

    /// Add a migration function
    pub fn add_migration<F>(&mut self, migration: F)
    where
        F: Fn(T) -> T + Send + Sync + 'static,
    {
        self.migrations.push(Box::new(migration));
        self.current_version = self.migrations.len();
    }

    /// Apply migrations to loaded state
    pub fn apply_migrations(&self, mut state: T, stored_version: usize) -> T {
        for (i, migration) in self.migrations.iter().enumerate() {
            if i >= stored_version {
                state = migration(state);
            }
        }
        state
    }

    /// Get the current schema version
    pub fn current_version(&self) -> usize {
        self.current_version
    }
}

/// Versioned persistent store
pub struct VersionedPersistentStore<T: Clone + PartialEq + serde::Serialize + 'static> {
    store: StoreContext<T>,
    key: String,
    migrations: MigrationManager<T>,
}

impl<T: Clone + PartialEq + serde::Serialize + 'static> VersionedPersistentStore<T> {
    /// Create a new versioned persistent store
    pub fn new(initial: T, key: String) -> Self {
        let migrations = MigrationManager::new();

        // Try to load with migrations
        let loaded_state = load_from_local_storage(&format!("{}_data", key))
            .map(|data: T| data)
            .unwrap_or(initial);

        let store = create_store(loaded_state);

        // Set up persistence
        persist_to_local_storage(&format!("{}_data", key), store.clone());

        Self {
            store,
            key,
            migrations,
        }
    }

    /// Add a migration
    pub fn add_migration<F>(&mut self, migration: F)
    where
        F: Fn(T) -> T + Send + Sync + 'static,
    {
        self.migrations.add_migration(migration);
    }

    /// Get the store context
    pub fn store(&self) -> &StoreContext<T> {
        &self.store
    }

    /// Get the current schema version
    pub fn schema_version(&self) -> usize {
        self.migrations.current_version()
    }

    /// Force a schema migration
    pub fn migrate_schema(&self) -> Result<(), String> {
        // This would load old data, apply migrations, and save
        // Implementation depends on specific versioning strategy
        Ok(())
    }
}
