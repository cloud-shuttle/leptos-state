//! Time travel debugging support

use super::Store;
use std::collections::VecDeque;
use std::marker::PhantomData;

/// Time travel debugging support
pub struct TimeTravel<S: Store> {
    history: VecDeque<Snapshot<S::State>>,
    current_index: usize,
    max_history: usize,
    _phantom: PhantomData<S>,
}

/// State snapshot for time travel
#[derive(Debug, Clone)]
pub struct Snapshot<T> {
    pub state: T,
    pub action: String,
    pub timestamp: u64,
}

impl<T> Snapshot<T> {
    pub fn new(state: T, action: impl Into<String>) -> Self {
        Self {
            state,
            action: action.into(),
            timestamp: js_sys::Date::now() as u64,
        }
    }
}

impl<S: Store> TimeTravel<S> {
    pub fn new() -> Self {
        Self {
            history: VecDeque::new(),
            current_index: 0,
            max_history: 50, // Default to 50 snapshots
            _phantom: PhantomData,
        }
    }

    pub fn with_max_history(max_history: usize) -> Self {
        Self {
            history: VecDeque::new(),
            current_index: 0,
            max_history,
            _phantom: PhantomData,
        }
    }

    /// Record a new state snapshot
    pub fn record(&mut self, state: S::State, action: impl Into<String>) {
        let snapshot = Snapshot::new(state, action);

        // Remove any future history when recording new action
        self.history.truncate(self.current_index + 1);

        // Add new snapshot
        self.history.push_back(snapshot);

        // Remove old snapshots if we exceed max history
        if self.history.len() > self.max_history {
            self.history.pop_front();
        } else {
            self.current_index += 1;
        }
    }

    /// Undo to previous state
    pub fn undo(&mut self) -> Option<&S::State> {
        if self.can_undo() {
            self.current_index -= 1;
            self.history.get(self.current_index).map(|s| &s.state)
        } else {
            None
        }
    }

    /// Redo to next state
    pub fn redo(&mut self) -> Option<&S::State> {
        if self.can_redo() {
            self.current_index += 1;
            self.history.get(self.current_index).map(|s| &s.state)
        } else {
            None
        }
    }

    /// Jump to specific snapshot
    pub fn jump_to(&mut self, index: usize) -> Option<&S::State> {
        if index < self.history.len() {
            self.current_index = index;
            self.history.get(self.current_index).map(|s| &s.state)
        } else {
            None
        }
    }

    /// Check if undo is possible
    pub fn can_undo(&self) -> bool {
        self.current_index > 0
    }

    /// Check if redo is possible
    pub fn can_redo(&self) -> bool {
        self.current_index < self.history.len().saturating_sub(1)
    }

    /// Get current snapshot
    pub fn current(&self) -> Option<&Snapshot<S::State>> {
        self.history.get(self.current_index)
    }

    /// Get all snapshots
    pub fn snapshots(&self) -> impl Iterator<Item = &Snapshot<S::State>> {
        self.history.iter()
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.history.clear();
        self.current_index = 0;
    }
}

impl<S: Store> Default for TimeTravel<S> {
    fn default() -> Self {
        Self::new()
    }
}
