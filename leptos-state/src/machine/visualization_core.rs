//! Core visualization functionality

use super::*;
use super::visualization_data::MachineSnapshot;

/// State machine visualizer
pub struct MachineVisualizer<C: Send + Sync, E> {
    /// Visualization configuration
    pub config: VisualizationConfig,
    /// Theme configuration
    pub theme: VisualizationTheme,
    /// Current machine being visualized
    pub machine: Option<Machine<C, E, C>>,
    /// Event history
    pub event_history: Vec<TransitionEvent<C, E>>,
    /// Performance metrics
    pub performance_metrics: Vec<PerformanceEvent>,
    /// Error log
    pub error_log: Vec<ErrorEvent>,
    /// State change history
    pub state_history: Vec<StateChangeEvent<C, E>>,
    /// Current snapshot
    pub current_snapshot: Option<MachineSnapshot<C, E>>,
    /// Whether visualization is enabled
    pub enabled: bool,
}

impl<C: Send + Sync, E> MachineVisualizer<C, E> {
    /// Create a new machine visualizer
    pub fn new() -> Self {
        Self {
            config: VisualizationConfig::default(),
            theme: VisualizationTheme::default(),
            machine: None,
            event_history: Vec::new(),
            performance_metrics: Vec::new(),
            error_log: Vec::new(),
            state_history: Vec::new(),
            current_snapshot: None,
            enabled: true,
        }
    }

    /// Create a visualizer with custom configuration
    pub fn with_config(config: VisualizationConfig) -> Self {
        Self {
            config,
            theme: VisualizationTheme::default(),
            machine: None,
            event_history: Vec::new(),
            performance_metrics: Vec::new(),
            error_log: Vec::new(),
            state_history: Vec::new(),
            current_snapshot: None,
            enabled: true,
        }
    }

    /// Set the machine to visualize
    pub fn with_machine(mut self, machine: Machine<C, E, C>) -> Self {
        self.machine = Some(machine);
        self
    }

    /// Set the theme
    pub fn with_theme(mut self, theme: VisualizationTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Enable or disable visualization
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Update configuration
    pub fn update_config(&mut self, config: VisualizationConfig) {
        self.config = config;
    }

    /// Update theme
    pub fn update_theme(&mut self, theme: VisualizationTheme) {
        self.theme = theme;
    }

    /// Record a transition event
    pub fn record_transition(&mut self, event: TransitionEvent<C, E>) {
        if !self.enabled {
            return;
        }

        self.event_history.push(event.clone());

        // Also record as state change
        let state_change = StateChangeEvent::new(
            event.from_state,
            event.to_state,
            if event.success {
                StateChangeType::Transition
            } else {
                StateChangeType::ErrorRecovery
            },
        );

        self.state_history.push(state_change);

        // Keep history size manageable
        if self.event_history.len() > 1000 {
            self.event_history.remove(0);
        }

        if self.state_history.len() > 1000 {
            self.state_history.remove(0);
        }
    }

    /// Record a performance event
    pub fn record_performance(&mut self, event: PerformanceEvent) {
        if !self.enabled {
            return;
        }

        self.performance_metrics.push(event);

        // Keep metrics size manageable
        if self.performance_metrics.len() > 5000 {
            self.performance_metrics.remove(0);
        }
    }

    /// Record an error event
    pub fn record_error(&mut self, error: ErrorEvent) {
        if !self.enabled {
            return;
        }

        self.error_log.push(error);

        // Keep error log size manageable
        if self.error_log.len() > 1000 {
            self.error_log.remove(0);
        }
    }

    /// Take a snapshot of the current machine state
    pub fn take_snapshot(&mut self) -> Result<(), String> {
        if let Some(ref machine) = self.machine {
            let snapshot = MachineSnapshot::new(machine.clone());
            self.current_snapshot = Some(snapshot);
            Ok(())
        } else {
            Err("No machine set for visualization".to_string())
        }
    }

    /// Export state diagram in the specified format
    pub fn export_diagram(&self, format: ExportFormat) -> Result<String, String> {
        if let Some(ref machine) = self.machine {
            match format {
                ExportFormat::Dot => self.export_dot(machine),
                ExportFormat::Mermaid => self.export_mermaid(machine),
                ExportFormat::PlantUml => self.export_plantuml(machine),
                ExportFormat::Json => self.export_json(machine),
                _ => Err(format!("Format {:?} not supported for diagram export", format)),
            }
        } else {
            Err("No machine set for visualization".to_string())
        }
    }

    /// Export as GraphViz DOT format
    fn export_dot(&self, machine: &Machine<C, E, C>) -> Result<String, String> {
        let mut output = String::new();

        output.push_str("digraph StateMachine {\n");
        output.push_str("  rankdir=LR;\n");
        output.push_str(&format!("  bgcolor=\"{}\";\n", self.theme.background_color));
        output.push_str("  node [shape=rectangle, style=filled];\n");

        // Add initial state
        let initial_state = machine.initial_state();
        output.push_str(&format!("  \"{}\" [fillcolor=\"{}\", shape=circle, label=\"\"];\n",
            "start", self.theme.initial_state_color));

        // Add states
        for state_name in machine.get_states() {
            let color = if state_name == initial_state {
                &self.theme.initial_state_color
            } else {
                &self.theme.state_color
            };

            let label = if self.config.show_descriptions {
                state_name.clone()
            } else {
                state_name.clone()
            };

            output.push_str(&format!("  \"{}\" [fillcolor=\"{}\", label=\"{}\"];\n",
                state_name, color, label));
        }

        // Add initial transition
        output.push_str(&format!("  \"start\" -> \"{}\";\n", initial_state));

        // Add transitions
        for state_name in machine.get_states() {
            if let Some(state_node) = machine.states_map().get(&state_name) {
                for transition in &state_node.transitions {
                    let target = &transition.target;
                    let label = if self.config.show_guards && !transition.guards.is_empty() {
                        format!("guards: {}", transition.guards.len())
                    } else {
                        String::new()
                    };

                    let color = &self.theme.transition_color;
                    output.push_str(&format!("  \"{}\" -> \"{}\" [color=\"{}\", label=\"{}\"];\n",
                        state_name, target, color, label));
                }
            }
        }

        output.push_str("}\n");
        Ok(output)
    }

    /// Export as Mermaid format
    fn export_mermaid(&self, machine: &Machine<C, E, C>) -> Result<String, String> {
        let mut output = String::new();

        output.push_str("stateDiagram-v2\n");

        // Add initial state
        let initial_state = machine.initial_state();
        output.push_str(&format!("  [*] --> {}\n", initial_state));

        // Add states
        for state_name in machine.get_states() {
            if state_name != initial_state {
                output.push_str(&format!("  {}\n", state_name));
            }
        }

        // Add transitions
        for state_name in machine.get_states() {
            if let Some(state_node) = machine.states_map().get(&state_name) {
                for transition in &state_node.transitions {
                    let target = &transition.target;
                    let label = if self.config.show_guards && !transition.guards.is_empty() {
                        format!(" : guards({})", transition.guards.len())
                    } else {
                        String::new()
                    };

                    output.push_str(&format!("  {} --> {}{}\n",
                        state_name, target, label));
                }
            }
        }

        Ok(output)
    }

    /// Export as PlantUML format
    fn export_plantuml(&self, machine: &Machine<C, E, C>) -> Result<String, String> {
        let mut output = String::new();

        output.push_str("@startuml\n");
        output.push_str("skinparam backgroundColor #FEFEFE\n");
        output.push_str("skinparam state {\n");
        output.push_str(&format!("  BackgroundColor<<initial>> {}\n", self.theme.initial_state_color));
        output.push_str(&format!("  BackgroundColor<<normal>> {}\n", self.theme.state_color));
        output.push_str("}\n");

        // Add initial state
        let initial_state = machine.initial_state();
        output.push_str(&format!("[*] --> {}\n", initial_state));

        // Add states
        for state_name in machine.get_states() {
            if state_name != initial_state {
                output.push_str(&format!("state {}\n", state_name));
            }
        }

        // Add transitions
        for state_name in machine.get_states() {
            if let Some(state_node) = machine.states_map().get(&state_name) {
                for transition in &state_node.transitions {
                    let target = &transition.target;
                    let label = if self.config.show_guards && !transition.guards.is_empty() {
                        format!(" : [guards: {}]", transition.guards.len())
                    } else {
                        String::new()
                    };

                    output.push_str(&format!("{} --> {}{}\n",
                        state_name, target, label));
                }
            }
        }

        output.push_str("@enduml\n");
        Ok(output)
    }

    /// Export as JSON format
    fn export_json(&self, machine: &Machine<C, E, C>) -> Result<String, String> {
        let diagram = StateDiagram::new(machine, &self.config);
        serde_json::to_string_pretty(&diagram)
            .map_err(|e| format!("JSON serialization failed: {}", e))
    }

    /// Generate performance report
    pub fn generate_performance_report(&self) -> PerformanceReport {
        let total_events = self.performance_metrics.len();
        let total_duration: std::time::Duration = self.performance_metrics.iter()
            .map(|e| e.duration)
            .sum();

        let avg_duration = if total_events > 0 {
            total_duration / total_events as u32
        } else {
            std::time::Duration::from_nanos(0)
        };

        let max_duration = self.performance_metrics.iter()
            .map(|e| e.duration)
            .max()
            .unwrap_or(std::time::Duration::from_nanos(0));

        let event_counts = self.performance_metrics.iter()
            .fold(std::collections::HashMap::new(), |mut acc, event| {
                *acc.entry(event.event_type.clone()).or_insert(0) += 1;
                acc
            });

        PerformanceReport {
            total_events,
            total_duration,
            avg_duration,
            max_duration,
            event_counts,
            generated_at: std::time::Instant::now(),
        }
    }

    /// Generate error summary
    pub fn generate_error_summary(&self) -> ErrorSummary {
        let total_errors = self.error_log.len();
        let error_counts = self.error_log.iter()
            .fold(std::collections::HashMap::new(), |mut acc, error| {
                *acc.entry(error.error_type.clone()).or_insert(0) += 1;
                acc
            });

        let recent_errors: Vec<_> = self.error_log.iter()
            .rev()
            .take(10)
            .cloned()
            .collect();

        ErrorSummary {
            total_errors,
            error_counts,
            recent_errors,
            generated_at: std::time::Instant::now(),
        }
    }

    /// Clear all recorded data
    pub fn clear(&mut self) {
        self.event_history.clear();
        self.performance_metrics.clear();
        self.error_log.clear();
        self.state_history.clear();
        self.current_snapshot = None;
    }
}

/// Performance report
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    /// Total number of performance events
    pub total_events: usize,
    /// Total duration of all events
    pub total_duration: std::time::Duration,
    /// Average duration per event
    pub avg_duration: std::time::Duration,
    /// Maximum duration of any event
    pub max_duration: std::time::Duration,
    /// Count of events by type
    pub event_counts: std::collections::HashMap<PerformanceEventType, usize>,
    /// When the report was generated
    pub generated_at: std::time::Instant,
}

impl PerformanceReport {
    /// Get events per second rate
    pub fn events_per_second(&self) -> f64 {
        if self.total_duration.as_nanos() == 0 {
            0.0
        } else {
            self.total_events as f64 / self.total_duration.as_secs_f64()
        }
    }

    /// Get average duration in milliseconds
    pub fn avg_duration_ms(&self) -> f64 {
        self.avg_duration.as_secs_f64() * 1000.0
    }

    /// Get max duration in milliseconds
    pub fn max_duration_ms(&self) -> f64 {
        self.max_duration.as_secs_f64() * 1000.0
    }
}

/// Error summary
#[derive(Debug, Clone)]
pub struct ErrorSummary {
    /// Total number of errors
    pub total_errors: usize,
    /// Count of errors by type
    pub error_counts: std::collections::HashMap<ErrorEventType, usize>,
    /// Recent errors (last 10)
    pub recent_errors: Vec<ErrorEvent>,
    /// When the summary was generated
    pub generated_at: std::time::Instant,
}

impl ErrorSummary {
    /// Get most common error type
    pub fn most_common_error_type(&self) -> Option<ErrorEventType> {
        self.error_counts.iter()
            .max_by_key(|(_, count)| *count)
            .map(|(error_type, _)| error_type.clone())
    }

    /// Check if there are critical errors
    pub fn has_critical_errors(&self) -> bool {
        self.error_counts.get(&ErrorEventType::InternalError).unwrap_or(&0) > &0
    }
}
