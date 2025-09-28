use super::types_context::Context;

/// History entry for machine transitions
#[derive(Clone, Debug)]
pub struct HistoryEntry<C = ()> {
    pub from_state: String,
    pub to_state: String,
    pub event: String,
    pub timestamp: std::time::SystemTime,
    pub context_snapshot: super::types_context::Context,
    _phantom: std::marker::PhantomData<C>,
}

/// Machine history tracking
#[derive(Clone, Debug, Default)]
pub struct MachineHistory<C = ()> {
    pub entries: Vec<HistoryEntry<C>>,
    pub max_size: usize,
    _phantom: std::marker::PhantomData<C>,
}

impl<C> MachineHistory<C> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            max_size: 100,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_size,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn record_transition(
        &mut self,
        from_state: String,
        to_state: String,
        event: String,
        context: &super::types_context::Context,
    ) {
        let entry = HistoryEntry {
            from_state,
            to_state,
            event,
            timestamp: std::time::SystemTime::now(),
            context_snapshot: context.clone(),
            _phantom: std::marker::PhantomData,
        };

        self.entries.push(entry);

        // Trim history if it exceeds max size
        if self.entries.len() > self.max_size {
            let excess = self.entries.len() - self.max_size;
            self.entries.drain(0..excess);
        }
    }

    pub fn get_entries(&self) -> &[HistoryEntry<C>] {
        &self.entries
    }

    pub fn get_latest(&self) -> Option<&HistoryEntry<C>> {
        self.entries.last()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
