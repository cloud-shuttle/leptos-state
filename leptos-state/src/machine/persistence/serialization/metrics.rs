//! Serialization metrics and analysis

/// Complexity metrics for analysis
#[derive(Debug, Clone, Default)]
pub struct ComplexityMetrics {
    /// Total number of states
    pub state_count: usize,
    /// Total number of transitions
    pub transition_count: usize,
    /// Number of compound states
    pub compound_state_count: usize,
    /// Number of parallel states
    pub parallel_state_count: usize,
    /// Maximum state depth
    pub max_depth: usize,
    /// Average transitions per state
    pub avg_transitions_per_state: f64,
    /// Number of states with guards
    pub states_with_guards: usize,
    /// Number of states with actions
    pub states_with_actions: usize,
    /// Total number of guards
    pub total_guards: usize,
    /// Total number of actions
    pub total_actions: usize,
    /// Cyclomatic complexity
    pub cyclomatic_complexity: usize,
    /// Serialization size estimate (bytes)
    pub estimated_size: usize,
}

impl ComplexityMetrics {
    /// Create new complexity metrics
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate metrics from serialized machine
    pub fn from_machine<C, E, S>(machine: &super::core::SerializedMachine<C, E, S>) -> Self {
        let mut metrics = Self::new();

        metrics.state_count = machine.states.len();
        metrics.transition_count = machine.transitions.len();

        // Count state types and analyze structure
        for state in &machine.states {
            match state.state_type {
                super::core::StateType::Compound => {
                    metrics.compound_state_count += 1;
                    metrics.max_depth = metrics.max_depth.max(Self::calculate_depth(state, machine, 1));
                }
                super::core::StateType::Parallel => {
                    metrics.parallel_state_count += 1;
                }
                super::core::StateType::Simple => {}
            }

            if state.has_children() {
                metrics.states_with_actions += 1;
            }
        }

        // Analyze transitions
        for transition in &machine.transitions {
            if transition.has_guards() {
                metrics.states_with_guards += 1;
                metrics.total_guards += transition.guards.len();
            }
            if transition.has_actions() {
                metrics.states_with_actions += 1;
                metrics.total_actions += transition.actions.len();
            }
        }

        // Calculate averages
        if metrics.state_count > 0 {
            metrics.avg_transitions_per_state = metrics.transition_count as f64 / metrics.state_count as f64;
        }

        // Calculate cyclomatic complexity (simplified)
        metrics.cyclomatic_complexity = metrics.transition_count + metrics.total_guards + 1;

        // Estimate serialization size
        metrics.estimated_size = Self::estimate_size(machine);

        metrics
    }

    /// Calculate state depth recursively
    fn calculate_depth<C, E, S>(state: &super::core::SerializedState<C>, machine: &super::core::SerializedMachine<C, E, S>, current_depth: usize) -> usize {
        let mut max_child_depth = current_depth;

        for child_id in &state.child_states {
            if let Some(child) = machine.get_state(child_id) {
                if child.is_compound() {
                    max_child_depth = max_child_depth.max(Self::calculate_depth(child, machine, current_depth + 1));
                }
            }
        }

        max_child_depth
    }

    /// Estimate serialization size in bytes
    fn estimate_size<C, E, S>(machine: &super::core::SerializedMachine<C, E, S>) -> usize {
        let mut size = 0;

        // Basic structure overhead
        size += std::mem::size_of::<super::core::SerializedMachine<C, E, S>>();

        // States
        for state in &machine.states {
            size += state.id.len() + std::mem::size_of::<super::core::SerializedState<C>>();
            size += state.child_states.iter().map(|s| s.len()).sum::<usize>();
            size += state.entry_actions.iter().map(|s| s.len()).sum::<usize>();
            size += state.exit_actions.iter().map(|s| s.len()).sum::<usize>();
        }

        // Transitions
        for transition in &machine.transitions {
            size += transition.source.len() + transition.target.len();
            size += std::mem::size_of::<super::core::SerializedTransition<E>>();
            size += transition.guards.iter().map(|s| s.len()).sum::<usize>();
            size += transition.actions.iter().map(|s| s.len()).sum::<usize>();
        }

        // Metadata
        size += machine.metadata.estimate_size();

        size
    }

    /// Get complexity score (higher = more complex)
    pub fn complexity_score(&self) -> f64 {
        let depth_factor = self.max_depth as f64;
        let state_factor = self.state_count as f64;
        let transition_factor = self.transition_count as f64;
        let guard_factor = self.total_guards as f64;

        depth_factor * 2.0 + state_factor * 0.5 + transition_factor * 0.3 + guard_factor * 1.0
    }

    /// Check if machine is considered complex
    pub fn is_complex(&self) -> bool {
        self.complexity_score() > 20.0
    }

    /// Get complexity level as string
    pub fn complexity_level(&self) -> &'static str {
        let score = self.complexity_score();
        match score {
            s if s < 5.0 => "simple",
            s if s < 15.0 => "moderate",
            s if s < 30.0 => "complex",
            _ => "very complex",
        }
    }

    /// Get recommended optimization strategies
    pub fn recommended_optimizations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        if self.max_depth > 3 {
            recommendations.push("Consider flattening deep state hierarchies".to_string());
        }

        if self.avg_transitions_per_state > 5.0 {
            recommendations.push("Consider consolidating transition logic".to_string());
        }

        if self.total_guards > self.transition_count * 2 {
            recommendations.push("Consider simplifying guard conditions".to_string());
        }

        if self.estimated_size > 1024 * 1024 { // 1MB
            recommendations.push("Consider compression for large state machines".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("Machine complexity is within acceptable ranges".to_string());
        }

        recommendations
    }
}

/// Serialization format information
#[derive(Debug, Clone)]
pub struct SerializationFormat {
    /// Format name
    pub name: String,
    /// Format version
    pub version: String,
    /// MIME type
    pub mime_type: String,
    /// File extension
    pub file_extension: String,
    /// Whether format supports compression
    pub supports_compression: bool,
    /// Whether format supports encryption
    pub supports_encryption: bool,
    /// Maximum recommended size
    pub max_recommended_size: Option<u64>,
}

impl SerializationFormat {
    /// Create JSON format
    pub fn json() -> Self {
        Self {
            name: "JSON".to_string(),
            version: "1.0".to_string(),
            mime_type: "application/json".to_string(),
            file_extension: "json".to_string(),
            supports_compression: true,
            supports_encryption: true,
            max_recommended_size: None,
        }
    }

    /// Create MessagePack format
    pub fn messagepack() -> Self {
        Self {
            name: "MessagePack".to_string(),
            version: "1.0".to_string(),
            mime_type: "application/msgpack".to_string(),
            file_extension: "msgpack".to_string(),
            supports_compression: true,
            supports_encryption: true,
            max_recommended_size: None,
        }
    }

    /// Create binary format
    pub fn binary() -> Self {
        Self {
            name: "Binary".to_string(),
            version: "1.0".to_string(),
            mime_type: "application/octet-stream".to_string(),
            file_extension: "bin".to_string(),
            supports_compression: true,
            supports_encryption: true,
            max_recommended_size: Some(10 * 1024 * 1024), // 10MB
        }
    }

    /// Get format by name
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "json" => Some(Self::json()),
            "messagepack" | "msgpack" => Some(Self::messagepack()),
            "binary" | "bin" => Some(Self::binary()),
            _ => None,
        }
    }

    /// Get all supported formats
    pub fn supported_formats() -> Vec<Self> {
        vec![Self::json(), Self::messagepack(), Self::binary()]
    }

    /// Check if format is recommended for given size
    pub fn is_recommended_for_size(&self, size: u64) -> bool {
        if let Some(max_size) = self.max_recommended_size {
            size <= max_size
        } else {
            true // No size limit
        }
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        format!(
            "{} v{} ({}), compression: {}, encryption: {}",
            self.name,
            self.version,
            self.mime_type,
            if self.supports_compression { "yes" } else { "no" },
            if self.supports_encryption { "yes" } else { "no" }
        )
    }
}

impl Default for SerializationFormat {
    fn default() -> Self {
        Self::json()
    }
}
