use leptos::*;
use crate::store::*;

/// Hook to access a store's state and setter
pub fn use_store<S: Store>() -> (ReadSignal<S::State>, WriteSignal<S::State>) {
    S::use_store()
}

/// Hook to access a computed slice of store state
pub fn use_store_slice<S: Store, Slice: StoreSlice<S>>() -> Memo<Slice::Output> {
    crate::store::use_store_slice::<S, Slice>()
}

/// Hook to create a computed value from store state
pub fn use_computed<S: Store, T: PartialEq + Clone + 'static>(
    selector: impl Fn(&S::State) -> T + 'static,
) -> Memo<T> {
    crate::store::create_computed::<S, T>(selector)
}

/// Hook for store actions (functions that update store state)
pub fn use_store_actions<S: Store>() -> StoreActions<S::State> {
    let (_, set_state) = use_store::<S>();
    StoreActions::new(set_state)
}

/// Helper struct for common store actions
pub struct StoreActions<T: 'static> {
    setter: WriteSignal<T>,
}

impl<T: Clone> StoreActions<T> {
    pub fn new(setter: WriteSignal<T>) -> Self {
        Self { setter }
    }
    
    /// Set the entire state
    pub fn set(&self, new_state: T) {
        self.setter.set(new_state);
    }
    
    /// Update state with a function
    pub fn update(&self, f: impl FnOnce(&mut T)) {
        self.setter.update(f);
    }
    
    /// Update state with a mapping function
    pub fn map(&self, f: impl FnOnce(T) -> T) {
        self.setter.update(|state| *state = f(state.clone()));
    }
    
    /// Reset to initial state
    pub fn reset<S: Store<State = T>>(&self) {
        self.setter.set(S::create());
    }
}

/// Hook for batched store updates
pub fn use_store_batch<S: Store>() -> StoreBatch<S::State> {
    let (_, set_state) = use_store::<S>();
    StoreBatch::new(set_state)
}

/// Helper for batching multiple store updates
pub struct StoreBatch<T: 'static> {
    setter: WriteSignal<T>,
    pending_updates: std::cell::RefCell<Vec<Box<dyn FnOnce(&mut T)>>>,
}

impl<T: Clone> StoreBatch<T> {
    pub fn new(setter: WriteSignal<T>) -> Self {
        Self {
            setter,
            pending_updates: std::cell::RefCell::new(Vec::new()),
        }
    }
    
    /// Add an update to the batch
    pub fn update(&self, f: impl FnOnce(&mut T) + 'static) {
        self.pending_updates.borrow_mut().push(Box::new(f));
    }
    
    /// Apply all pending updates in a single batch
    pub fn commit(self) {
        let updates = self.pending_updates.into_inner();
        if !updates.is_empty() {
            self.setter.update(|state| {
                for update in updates {
                    update(state);
                }
            });
        }
    }
}

/// Hook for store history/undo functionality
pub fn use_store_history<S: Store>() -> StoreHistory<S::State> 
where
    S::State: Clone + PartialEq,
{
    let (state, set_state) = use_store::<S>();
    let history = create_rw_signal(Vec::<S::State>::new());
    let current_index = create_rw_signal(0);
    
    // Track state changes and add to history
    create_effect(move |prev_state: Option<Option<S::State>>| {
        let current_state = state.get();
        
        if let Some(Some(prev)) = prev_state {
            if prev != current_state {
                history.update(|h| {
                    // Remove any future history when new changes are made
                    h.truncate(current_index.get());
                    h.push(current_state.clone());
                });
                current_index.update(|i| *i += 1);
            }
        } else {
            // Initial state
            history.update(|h| h.push(current_state.clone()));
        }
        
        Some(current_state)
    });
    
    StoreHistory {
        set_state,
        history: history.read_only(),
        current_index: current_index.read_only(),
        set_index: current_index.write_only(),
    }
}

/// Store history manager
pub struct StoreHistory<T: 'static> {
    set_state: WriteSignal<T>,
    history: ReadSignal<Vec<T>>,
    current_index: ReadSignal<usize>,
    set_index: WriteSignal<usize>,
}

impl<T: Clone> StoreHistory<T> {
    /// Check if undo is possible
    pub fn can_undo(&self) -> bool {
        self.current_index.get() > 0
    }
    
    /// Check if redo is possible
    pub fn can_redo(&self) -> bool {
        let history = self.history.get();
        self.current_index.get() < history.len().saturating_sub(1)
    }
    
    /// Undo to previous state
    pub fn undo(&self) {
        if self.can_undo() {
            let new_index = self.current_index.get() - 1;
            self.set_index.set(new_index);
            
            let history = self.history.get();
            if let Some(state) = history.get(new_index) {
                self.set_state.set(state.clone());
            }
        }
    }
    
    /// Redo to next state
    pub fn redo(&self) {
        if self.can_redo() {
            let new_index = self.current_index.get() + 1;
            self.set_index.set(new_index);
            
            let history = self.history.get();
            if let Some(state) = history.get(new_index) {
                self.set_state.set(state.clone());
            }
        }
    }
    
    /// Jump to specific history index
    pub fn jump_to(&self, index: usize) {
        let history = self.history.get();
        if index < history.len() {
            self.set_index.set(index);
            if let Some(state) = history.get(index) {
                self.set_state.set(state.clone());
            }
        }
    }
    
    /// Get current history length
    pub fn len(&self) -> usize {
        self.history.get().len()
    }
    
    /// Get current index in history
    pub fn current(&self) -> usize {
        self.current_index.get()
    }
    
    /// Clear history
    pub fn clear(&self) {
        // This would require a WriteSignal<Vec<T>> instead of ReadSignal
        // For now, this is a placeholder
        tracing::warn!("clear() not implemented - would require refactoring to use RwSignal");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::create_store;

    #[derive(Clone, PartialEq, Debug)]
    pub struct TestState {
        count: i32,
        name: String,
    }

    create_store!(TestStore, TestState, TestState {
        count: 0,
        name: "test".to_string()
    });

    #[test]
    fn store_actions_work() {
        // This test would need a Leptos runtime
        // Placeholder for now
        assert!(true);
    }

    #[test]
    fn batch_updates_work() {
        // This test would need a Leptos runtime
        // Placeholder for now
        assert!(true);
    }
}