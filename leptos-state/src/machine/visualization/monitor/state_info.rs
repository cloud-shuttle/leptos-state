//! Real-time state information and status tracking

use crate::machine::{Machine, MachineStateImpl};

/// Real-time state information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StateInfo<C: Send + Sync + std::fmt::Debug, E: std::fmt::Debug + PartialEq> {
    /// Machine ID
    pub machine_id: String,
    /// Current state
    pub state: MachineStateImpl<C>,
    /// State status
    pub status: StateStatus,
    /// Creation time
    pub created_at: std::time::SystemTime,
    /// Last updated time
    pub last_updated: std::time::SystemTime,
    /// Transition count
    pub transition_count: u64,
    /// Error count
    pub error_count: u64,
    /// Metadata
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

impl<C: Send + Sync + std::fmt::Debug + Clone + 'static, E: std::fmt::Debug + PartialEq + Clone + Send + Sync + 'static> StateInfo<C, E> {
    /// Create new state info from a machine
    pub fn from_machine(machine: &Machine<C, E, C>) -> Self {
        let now = std::time::SystemTime::now();
        Self {
            machine_id: machine.id().to_string(),
            state: machine.initial_state(),
            status: StateStatus::Active,
            created_at: now,
            last_updated: now,
            transition_count: 0,
            error_count: 0,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Update with new state
    pub fn update_state(&mut self, new_state: MachineStateImpl<C>) {
        self.state = new_state;
        self.last_updated = std::time::SystemTime::now();
        self.transition_count += 1;
    }

    /// Record an error
    pub fn record_error(&mut self) {
        self.error_count += 1;
        if self.error_count > 10 {
            self.status = StateStatus::Error;
        }
    }

    /// Set status
    pub fn set_status(&mut self, status: StateStatus) {
        self.status = status;
        self.last_updated = std::time::SystemTime::now();
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: String, value: serde_json::Value) {
        self.metadata.insert(key, value);
    }

    /// Get metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&serde_json::Value> {
        self.metadata.get(key)
    }

    /// Get uptime
    pub fn uptime(&self) -> std::time::Duration {
        self.created_at.elapsed().unwrap_or_default()
    }

    /// Get time since last update
    pub fn time_since_update(&self) -> std::time::Duration {
        self.last_updated.elapsed().unwrap_or_default()
    }

    /// Check if state is stale
    pub fn is_stale(&self, max_age: std::time::Duration) -> bool {
        self.time_since_update() > max_age
    }

    /// Get health score (0-100)
    pub fn health_score(&self) -> u8 {
        match self.status {
            StateStatus::Active => {
                if self.error_count == 0 {
                    100
                } else {
                    (90 - self.error_count.min(10) * 5) as u8
                }
            }
            StateStatus::Idle => 80,
            StateStatus::Error => (50 - self.error_count.min(5) * 5) as u8,
            StateStatus::Terminated => 0,
        }
    }

    /// Generate summary
    pub fn summary(&self) -> String {
        format!(
            "StateInfo(machine: {}, state: {}, status: {}, transitions: {}, errors: {}, health: {})",
            self.machine_id,
            self.state.value(),
            self.status,
            self.transition_count,
            self.error_count,
            self.health_score()
        )
    }

    /// Export to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

impl<C: Send + Sync + std::fmt::Debug, E: std::fmt::Debug + PartialEq> std::fmt::Display for StateInfo<C, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}

/// State status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StateStatus {
    /// Machine is actively processing
    Active,
    /// Machine is idle
    Idle,
    /// Machine has encountered errors
    Error,
    /// Machine has been terminated
    Terminated,
}

impl StateStatus {
    /// Check if status indicates active operation
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }

    /// Check if status indicates idle operation
    pub fn is_idle(&self) -> bool {
        matches!(self, Self::Idle)
    }

    /// Check if status indicates error state
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    /// Check if status indicates terminated state
    pub fn is_terminated(&self) -> bool {
        matches!(self, Self::Terminated)
    }

    /// Get status as string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Idle => "idle",
            Self::Error => "error",
            Self::Terminated => "terminated",
        }
    }

    /// Get status color for UI
    pub fn color(&self) -> &'static str {
        match self {
            Self::Active => "green",
            Self::Idle => "blue",
            Self::Error => "red",
            Self::Terminated => "gray",
        }
    }

    /// Can transition from this status
    pub fn can_transition(&self) -> bool {
        !matches!(self, Self::Terminated)
    }

    /// Is operational (can perform work)
    pub fn is_operational(&self) -> bool {
        matches!(self, Self::Active | Self::Idle)
    }
}

impl std::fmt::Display for StateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for StateStatus {
    fn default() -> Self {
        Self::Idle
    }
}

impl std::str::FromStr for StateStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(Self::Active),
            "idle" => Ok(Self::Idle),
            "error" => Ok(Self::Error),
            "terminated" => Ok(Self::Terminated),
            _ => Err(format!("Invalid state status: {}", s)),
        }
    }
}

/// State information collector
pub struct StateInfoCollector<C: Send + Sync + std::fmt::Debug, E: std::fmt::Debug + PartialEq> {
    /// Collected state information
    states: std::collections::HashMap<String, StateInfo<C, E>>,
    /// Collection statistics
    stats: CollectionStats,
}

impl<C: Send + Sync + std::fmt::Debug + Clone + 'static, E: std::fmt::Debug + PartialEq + Clone + Send + Sync + 'static> StateInfoCollector<C, E> {
    /// Create a new collector
    pub fn new() -> Self {
        Self {
            states: std::collections::HashMap::new(),
            stats: CollectionStats::new(),
        }
    }

    /// Add state information
    pub fn add_state_info(&mut self, info: StateInfo<C, E>) {
        let machine_id = info.machine_id.clone();
        self.states.insert(machine_id, info);
        self.stats.total_states = self.states.len() as u64;
    }

    /// Get state information by machine ID
    pub fn get_state_info(&self, machine_id: &str) -> Option<&StateInfo<C, E>> {
        self.states.get(machine_id)
    }

    /// Remove state information
    pub fn remove_state_info(&mut self, machine_id: &str) -> Option<StateInfo<C, E>> {
        let result = self.states.remove(machine_id);
        if result.is_some() {
            self.stats.total_states = self.states.len() as u64;
        }
        result
    }

    /// Get all machine IDs
    pub fn machine_ids(&self) -> Vec<String> {
        self.states.keys().cloned().collect()
    }

    /// Get all state information
    pub fn all_states(&self) -> Vec<&StateInfo<C, E>> {
        self.states.values().collect()
    }

    /// Get states by status
    pub fn states_by_status(&self, status: StateStatus) -> Vec<&StateInfo<C, E>> {
        self.states.values().filter(|info| info.status == status).collect()
    }

    /// Get healthy states (health score > 80)
    pub fn healthy_states(&self) -> Vec<&StateInfo<C, E>> {
        self.states.values().filter(|info| info.health_score() > 80).collect()
    }

    /// Get unhealthy states (health score <= 50)
    pub fn unhealthy_states(&self) -> Vec<&StateInfo<C, E>> {
        self.states.values().filter(|info| info.health_score() <= 50).collect()
    }

    /// Update statistics
    pub fn update_stats(&mut self) {
        let mut active = 0;
        let mut idle = 0;
        let mut error = 0;
        let mut terminated = 0;
        let mut avg_health = 0.0;

        for state in self.states.values() {
            match state.status {
                StateStatus::Active => active += 1,
                StateStatus::Idle => idle += 1,
                StateStatus::Error => error += 1,
                StateStatus::Terminated => terminated += 1,
            }
            avg_health += state.health_score() as f64;
        }

        if !self.states.is_empty() {
            avg_health /= self.states.len() as f64;
        }

        self.stats.active_states = active;
        self.stats.idle_states = idle;
        self.stats.error_states = error;
        self.stats.terminated_states = terminated;
        self.stats.average_health_score = avg_health;
    }

    /// Get collection statistics
    pub fn stats(&self) -> &CollectionStats {
        &self.stats
    }

    /// Clear all state information
    pub fn clear(&mut self) {
        self.states.clear();
        self.stats = CollectionStats::new();
    }
}

impl<C: Send + Sync + std::fmt::Debug + Clone + 'static, E: std::fmt::Debug + PartialEq + Clone + Send + Sync + 'static> Default for StateInfoCollector<C, E> {
    fn default() -> Self {
        Self::new()
    }
}

/// Collection statistics
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CollectionStats {
    /// Total number of states being tracked
    pub total_states: u64,
    /// Number of active states
    pub active_states: u64,
    /// Number of idle states
    pub idle_states: u64,
    /// Number of error states
    pub error_states: u64,
    /// Number of terminated states
    pub terminated_states: u64,
    /// Average health score
    pub average_health_score: f64,
}

impl std::fmt::Display for CollectionStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CollectionStats(total: {}, active: {}, idle: {}, errors: {}, avg_health: {:.1})",
            self.total_states,
            self.active_states,
            self.idle_states,
            self.error_states,
            self.average_health_score
        )
    }
}
