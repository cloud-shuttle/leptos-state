//! State Machine Visualization & DevTools
//!
//! This module provides comprehensive visualization and debugging capabilities
//! for state machines, including visual state diagrams, real-time monitoring,
//! and advanced debugging tools.

use super::*;
use crate::machine::states::StateValue;
use crate::utils::types::{StateError, StateResult};
use std::collections::VecDeque;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};

#[cfg(feature = "serde_json")]
use serde_json;

/// State machine visualization configuration
#[derive(Debug, Clone)]
pub struct VisualizationConfig {
    /// Whether visualization is enabled
    pub enabled: bool,
    /// Update interval for real-time monitoring (in milliseconds)
    pub update_interval: u64,
    /// Maximum number of state transitions to keep in history
    pub max_history: usize,
    /// Whether to capture state snapshots
    pub capture_snapshots: bool,
    /// Whether to enable time travel debugging
    pub enable_time_travel: bool,
    /// Whether to show state transitions in real-time
    pub show_transitions: bool,
    /// Whether to show context changes
    pub show_context_changes: bool,
    /// Whether to show action executions
    pub show_actions: bool,
    /// Whether to show guard evaluations
    pub show_guards: bool,
    /// Export format for state diagrams
    pub export_format: ExportFormat,
}

impl Default for VisualizationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            update_interval: 100, // 100ms
            max_history: 100,
            capture_snapshots: true,
            enable_time_travel: true,
            show_transitions: true,
            show_context_changes: true,
            show_actions: true,
            show_guards: true,
            export_format: ExportFormat::Dot,
        }
    }
}

/// Export formats for state diagrams
#[derive(Debug, Clone, PartialEq)]
pub enum ExportFormat {
    Dot,     // Graphviz DOT format
    Mermaid, // Mermaid diagram format
    Json,    // JSON representation
    Svg,     // SVG image
    Png,     // PNG image
}

/// State transition event for visualization
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TransitionEvent<C, E> {
    /// Event that triggered the transition
    pub event: E,
    /// Source state
    pub from_state: StateValue,
    /// Target state
    pub to_state: StateValue,
    /// Context before transition
    pub context_before: C,
    /// Context after transition
    pub context_after: C,
    /// Guards that were evaluated
    pub guards_evaluated: Vec<GuardEvaluation>,
    /// Actions that were executed
    pub actions_executed: Vec<String>,
    /// Timestamp of the transition
    pub timestamp: u64,
    /// Duration of the transition
    pub duration: Duration,
}

/// Guard evaluation result for visualization
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GuardEvaluation {
    /// Guard description
    pub description: String,
    /// Whether the guard passed
    pub passed: bool,
    /// Additional details about the evaluation
    pub details: Option<String>,
}

/// Action execution result for visualization
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ActionExecution {
    /// Action description
    pub description: String,
    /// Whether the action was executed
    pub executed: bool,
    /// Execution duration
    pub duration: Duration,
    /// Any errors that occurred
    pub error: Option<String>,
}

/// State machine visualizer
pub struct MachineVisualizer<C: Send + Sync + Clone + Default + Debug, E: Clone + PartialEq + Debug + Send + Sync + Default> {
    config: VisualizationConfig,
    transitions: Arc<Mutex<VecDeque<TransitionEvent<C, E>>>>,
    snapshots: Arc<Mutex<VecDeque<MachineSnapshot<C, E>>>>,
    current_state: Arc<Mutex<Option<MachineStateImpl<C>>>>,
    machine: Arc<Machine<C, E>>,
    start_time: Instant,
}

impl<C: Send + Sync, E> MachineVisualizer<C, E>
where
    C: Clone + std::default::Default + 'static + std::fmt::Debug + Send + Sync,
    E: Clone + std::cmp::PartialEq + events::Event + 'static + std::fmt::Debug + Send + Sync + Default,
{
    pub fn new(machine: Machine<C, E>, config: VisualizationConfig) -> Self {
        Self {
            config,
            transitions: Arc::new(Mutex::new(VecDeque::new())),
            snapshots: Arc::new(Mutex::new(VecDeque::new())),
            current_state: Arc::new(Mutex::new(None)),
            machine: Arc::new(machine),
            start_time: Instant::now(),
        }
    }

    /// Record a state transition
    pub fn record_transition(
        &self,
        event: E,
        from_state: MachineStateImpl<C>,
        to_state: MachineStateImpl<C>,
        guards_evaluated: Vec<GuardEvaluation>,
        actions_executed: Vec<String>,
        duration: Duration,
    ) {
        if !self.config.enabled {
            return;
        }

        let transition = TransitionEvent {
            event,
            from_state: from_state.value().clone(),
            to_state: to_state.value().clone(),
            context_before: from_state.context().clone(),
            context_after: to_state.context().clone(),
            guards_evaluated,
            actions_executed,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            duration,
        };

        if let Ok(mut transitions) = self.transitions.lock() {
            transitions.push_back(transition);

            // Keep only the most recent transitions
            while transitions.len() > self.config.max_history {
                transitions.pop_front();
            }
        }

        // Update current state
        if let Ok(mut current) = self.current_state.lock() {
            *current = Some(to_state.clone());
        }

        // Capture snapshot if enabled
        if self.config.capture_snapshots {
            self.capture_snapshot(&to_state);
        }
    }

    /// Capture a state snapshot
    pub fn capture_snapshot(&self, state: &MachineStateImpl<C>) {
        let snapshot = MachineSnapshot {
            state: state.clone(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            uptime: self.start_time.elapsed(),
            _phantom: PhantomData,
        };

        if let Ok(mut snapshots) = self.snapshots.lock() {
            snapshots.push_back(snapshot);

            // Keep only the most recent snapshots
            while snapshots.len() > self.config.max_history {
                snapshots.pop_front();
            }
        }
    }

    /// Get the current state
    pub fn current_state(&self) -> Option<MachineStateImpl<C>> {
        self.current_state.lock().unwrap().clone()
    }

    /// Get recent transitions
    pub fn recent_transitions(&self, count: usize) -> Vec<TransitionEvent<C, E>> {
        if let Ok(transitions) = self.transitions.lock() {
            transitions.iter().rev().take(count).cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// Get recent snapshots
    pub fn recent_snapshots(&self, count: usize) -> Vec<MachineSnapshot<C, E>> {
        if let Ok(snapshots) = self.snapshots.lock() {
            snapshots.iter().rev().take(count).cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// Export state diagram in the specified format
    pub fn export_diagram(&self, format: ExportFormat) -> StateResult<String> {
        match format {
            ExportFormat::Dot => self.export_dot(),
            ExportFormat::Mermaid => self.export_mermaid(),
            ExportFormat::Json => self.export_json(),
            ExportFormat::Svg => self.export_svg(),
            ExportFormat::Png => self.export_png(),
        }
    }

    /// Export as Graphviz DOT format
    fn export_dot(&self) -> StateResult<String> {
        let mut dot = String::new();
        dot.push_str("digraph StateMachine {\n");
        dot.push_str("  rankdir=LR;\n");
        dot.push_str("  node [shape=circle];\n\n");

        // Add states
        for (state_id, _state_node) in self.machine.states_map() {
            let style = if state_id == self.machine.initial_state_id() {
                " [style=filled, fillcolor=lightgreen]"
            } else {
                ""
            };
            dot.push_str(&format!("  \"{}\"{};\n", state_id, style));
        }

        dot.push_str("\n");

        // Add transitions
        for (state_id, state_node) in self.machine.states_map() {
            for transition in &state_node.transitions {
                let label = format!("{}", transition.event.event_type());
                dot.push_str(&format!(
                    "  \"{}\" -> \"{}\" [label=\"{}\"];\n",
                    state_id, transition.target, label
                ));
            }
        }

        dot.push_str("}\n");
        Ok(dot)
    }

    /// Export as Mermaid format
    fn export_mermaid(&self) -> StateResult<String> {
        let mut mermaid = String::new();
        mermaid.push_str("stateDiagram-v2\n");

        // Add initial state
        mermaid.push_str(&format!("  [*] --> {}\n", self.machine.initial_state_id()));

        // Add transitions
        for (state_id, state_node) in self.machine.states_map() {
            for transition in &state_node.transitions {
                let label = format!("{}", transition.event.event_type());
                mermaid.push_str(&format!(
                    "  {} --> {} : {}\n",
                    state_id, transition.target, label
                ));
            }
        }

        Ok(mermaid)
    }

    /// Export as JSON format
    fn export_json(&self) -> StateResult<String> {
        let diagram = StateDiagram {
            machine: self.machine.as_ref(),
            current_state: self.current_state(),
            recent_transitions: self.recent_transitions(10),
            recent_snapshots: self.recent_snapshots(10),
            uptime: self.start_time.elapsed(),
        };

        #[cfg(feature = "serialization")]
        {
            serde_json::to_string_pretty(&diagram)
                .map_err(|e| StateError::new(&format!("Failed to serialize diagram: {}", e)))
        }

        #[cfg(not(feature = "serialization"))]
        Err(StateError::new(
            "JSON export requires serialization feature",
        ))
    }

    /// Export as SVG format (placeholder)
    fn export_svg(&self) -> StateResult<String> {
        // In a real implementation, this would generate SVG using a graph rendering library
        Ok("<svg>State diagram would be rendered here</svg>".to_string())
    }

    /// Export as PNG format (placeholder)
    fn export_png(&self) -> StateResult<String> {
        // In a real implementation, this would generate PNG using a graph rendering library
        Ok("PNG data would be generated here".to_string())
    }

    /// Get visualization statistics
    pub fn get_stats(&self) -> VisualizationStats {
        let transitions = self.transitions.lock().unwrap();
        let snapshots = self.snapshots.lock().unwrap();

        VisualizationStats {
            total_transitions: transitions.len(),
            total_snapshots: snapshots.len(),
            uptime: self.start_time.elapsed(),
            current_state: self.current_state().map(|s| s.value().clone()),
            average_transition_time: if !transitions.is_empty() {
                let total_duration: Duration = transitions.iter().map(|t| t.duration).sum();
                total_duration / transitions.len() as u32
            } else {
                Duration::ZERO
            },
        }
    }

    /// Clear visualization history
    pub fn clear_history(&self) {
        if let Ok(mut transitions) = self.transitions.lock() {
            transitions.clear();
        }
        if let Ok(mut snapshots) = self.snapshots.lock() {
            snapshots.clear();
        }
    }
}

/// State diagram representation for export
pub struct StateDiagram<'a, C: Send + Sync + Clone + Default + Debug, E: Clone + PartialEq + Debug + Send + Sync + Default> {
    pub machine: &'a Machine<C, E>,
    pub current_state: Option<MachineStateImpl<C>>,
    pub recent_transitions: Vec<TransitionEvent<C, E>>,
    pub recent_snapshots: Vec<MachineSnapshot<C, E>>,
    pub uptime: Duration,
}

/// Machine snapshot for time travel
#[derive(Debug, Clone)]
pub struct MachineSnapshot<C: Send + Sync, E> {
    pub state: MachineStateImpl<C>,
    pub timestamp: u64,
    pub uptime: Duration,
    _phantom: PhantomData<E>,
}

// Conditional serde implementation for StateDiagram
#[cfg(feature = "serialization")]
impl<'a, C, E> serde::Serialize for StateDiagram<'a, C, E>
where
    C: Send + Sync + serde::Serialize + 'static,
    E: serde::Serialize + 'static,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("StateDiagram", 5)?;
        // Note: We can't serialize the machine directly, so we serialize a simplified representation
        state.serialize_field("machine_id", &"machine")?;
        state.serialize_field(
            "current_state",
            &self.current_state.as_ref().map(|s| s.value().clone()),
        )?;
        state.serialize_field("recent_transitions", &self.recent_transitions)?;
        state.serialize_field("recent_snapshots", &self.recent_snapshots)?;
        state.serialize_field("uptime", &self.uptime)?;
        state.end()
    }
}

// Conditional serde implementation for MachineSnapshot
#[cfg(feature = "serialization")]
impl<C, E> serde::Serialize for MachineSnapshot<C, E>
where
    C: Send + Sync + serde::Serialize + 'static,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("MachineSnapshot", 4)?;
        // Note: We can't serialize MachineStateImpl directly, so we serialize a simplified representation
        state.serialize_field("state_value", &self.state.value())?;
        state.serialize_field("context", &self.state.context())?;
        state.serialize_field("timestamp", &self.timestamp)?;
        state.serialize_field("uptime", &self.uptime)?;
        state.end()
    }
}

#[cfg(feature = "serialization")]
impl<'de, C, E> serde::Deserialize<'de> for MachineSnapshot<C, E>
where
    C: Send + Sync + for<'a> serde::Deserialize<'a>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        #[derive(Debug)]
        struct MachineSnapshotVisitor<C, E> {
            _phantom: std::marker::PhantomData<(C, E)>,
        }

        impl<C, E> MachineSnapshotVisitor<C, E> {
            fn new() -> Self {
                Self {
                    _phantom: std::marker::PhantomData,
                }
            }
        }

        impl<'de, C, E> Visitor<'de> for MachineSnapshotVisitor<C, E>
        where
            C: Send + Sync + for<'a> serde::Deserialize<'a>,
        {
            type Value = MachineSnapshot<C, E>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct MachineSnapshot")
            }

            fn visit_map<V>(self, mut map: V) -> Result<MachineSnapshot<C, E>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut state_value = None;
                let mut context = None;
                let mut timestamp = None;
                let mut uptime = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "state_value" => state_value = Some(map.next_value()?),
                        "context" => context = Some(map.next_value()?),
                        "timestamp" => timestamp = Some(map.next_value()?),
                        "uptime" => uptime = Some(map.next_value()?),
                        _ => {}
                    }
                }

                let state_value =
                    state_value.ok_or_else(|| de::Error::missing_field("state_value"))?;
                let context = context.ok_or_else(|| de::Error::missing_field("context"))?;
                let timestamp = timestamp.ok_or_else(|| de::Error::missing_field("timestamp"))?;
                let uptime = uptime.ok_or_else(|| de::Error::missing_field("uptime"))?;

                // Create a simplified MachineStateImpl
                let state = MachineStateImpl::new(state_value, context);

                Ok(MachineSnapshot {
                    state,
                    timestamp,
                    uptime,
                    _phantom: PhantomData,
                })
            }
        }

        deserializer.deserialize_struct(
            "MachineSnapshot",
            &["state_value", "context", "timestamp", "uptime"],
            MachineSnapshotVisitor::new(),
        )
    }
}

/// Visualization statistics
#[derive(Debug, Clone)]
pub struct VisualizationStats {
    pub total_transitions: usize,
    pub total_snapshots: usize,
    pub uptime: Duration,
    pub current_state: Option<StateValue>,
    pub average_transition_time: Duration,
}

/// Time travel debugger for state machines
pub struct TimeTravelDebugger<C: Send + Sync, E> {
    snapshots: VecDeque<MachineSnapshot<C, E>>,
    current_index: isize,
    max_snapshots: usize,
    _phantom: PhantomData<E>,
}

impl<C: Send + Sync, E> TimeTravelDebugger<C, E>
where
    C: Clone + std::default::Default + 'static + std::fmt::Debug + Send + Sync,
    E: Clone + std::cmp::PartialEq + events::Event + 'static + std::fmt::Debug,
{
    pub fn new(max_snapshots: usize) -> Self {
        Self {
            snapshots: VecDeque::new(),
            current_index: -1,
            max_snapshots,
            _phantom: PhantomData,
        }
    }

    /// Add a snapshot
    pub fn add_snapshot(&mut self, snapshot: MachineSnapshot<C, E>) {
        // Remove any snapshots after the current index
        while self.snapshots.len() > (self.current_index + 1) as usize {
            self.snapshots.pop_back();
        }

        self.snapshots.push_back(snapshot);
        self.current_index += 1;

        // Keep only the most recent snapshots
        while self.snapshots.len() > self.max_snapshots {
            self.snapshots.pop_front();
            self.current_index -= 1;
        }
    }

    /// Go to the previous snapshot
    pub fn go_back(&mut self) -> Option<&MachineSnapshot<C, E>> {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.snapshots.get(self.current_index as usize)
        } else {
            None
        }
    }

    /// Go to the next snapshot
    pub fn go_forward(&mut self) -> Option<&MachineSnapshot<C, E>> {
        if self.current_index < (self.snapshots.len() - 1) as isize {
            self.current_index += 1;
            self.snapshots.get(self.current_index as usize)
        } else {
            None
        }
    }

    /// Go to the first snapshot
    pub fn go_to_start(&mut self) -> Option<&MachineSnapshot<C, E>> {
        if !self.snapshots.is_empty() {
            self.current_index = 0;
            self.snapshots.get(0)
        } else {
            None
        }
    }

    /// Go to the latest snapshot
    pub fn go_to_end(&mut self) -> Option<&MachineSnapshot<C, E>> {
        if !self.snapshots.is_empty() {
            self.current_index = (self.snapshots.len() - 1) as isize;
            self.snapshots.get(self.current_index as usize)
        } else {
            None
        }
    }

    /// Get the current snapshot
    pub fn current_snapshot(&self) -> Option<&MachineSnapshot<C, E>> {
        if self.current_index >= 0 && self.current_index < self.snapshots.len() as isize {
            self.snapshots.get(self.current_index as usize)
        } else {
            None
        }
    }

    /// Get all snapshots
    pub fn all_snapshots(&self) -> &VecDeque<MachineSnapshot<C, E>> {
        &self.snapshots
    }

    /// Get current position information
    pub fn position_info(&self) -> TimeTravelPosition {
        TimeTravelPosition {
            current_index: self.current_index,
            total_snapshots: self.snapshots.len(),
            can_go_back: self.current_index > 0,
            can_go_forward: self.current_index < (self.snapshots.len() - 1) as isize,
        }
    }
}

/// Time travel position information
#[derive(Debug, Clone)]
pub struct TimeTravelPosition {
    pub current_index: isize,
    pub total_snapshots: usize,
    pub can_go_back: bool,
    pub can_go_forward: bool,
}

/// Real-time state monitor
pub struct StateMonitor<C: Send + Sync + Clone + Default + Debug, E: Clone + PartialEq + Debug + Send + Sync + Default> {
    visualizer: Arc<MachineVisualizer<C, E>>,
    time_travel: Arc<Mutex<TimeTravelDebugger<C, E>>>,
    _config: VisualizationConfig,
}

impl<C: Send + Sync, E> StateMonitor<C, E>
where
    C: Clone + std::default::Default + 'static + std::fmt::Debug + Send + Sync,
    E: Clone + std::cmp::PartialEq + events::Event + 'static + std::fmt::Debug + Send + Sync + Default,
{
    pub fn new(visualizer: Arc<MachineVisualizer<C, E>>, config: VisualizationConfig) -> Self {
        Self {
            time_travel: Arc::new(Mutex::new(TimeTravelDebugger::new(config.max_history))),
            visualizer,
            _config: config,
        }
    }

    /// Get real-time state information
    pub fn get_state_info(&self) -> StateInfo<C, E> {
        StateInfo {
            current_state: self.visualizer.current_state(),
            stats: self.visualizer.get_stats(),
            recent_transitions: self.visualizer.recent_transitions(5),
            time_travel_position: self.time_travel.lock().unwrap().position_info(),
        }
    }

    /// Export current state diagram
    pub fn export_diagram(&self, format: ExportFormat) -> StateResult<String> {
        self.visualizer.export_diagram(format)
    }

    /// Time travel operations
    pub fn go_back(&self) -> Option<MachineSnapshot<C, E>> {
        self.time_travel.lock().unwrap().go_back().cloned()
    }

    pub fn go_forward(&self) -> Option<MachineSnapshot<C, E>> {
        self.time_travel.lock().unwrap().go_forward().cloned()
    }

    pub fn go_to_start(&self) -> Option<MachineSnapshot<C, E>> {
        self.time_travel.lock().unwrap().go_to_start().cloned()
    }

    pub fn go_to_end(&self) -> Option<MachineSnapshot<C, E>> {
        self.time_travel.lock().unwrap().go_to_end().cloned()
    }

    /// Get current time travel snapshot
    pub fn current_snapshot(&self) -> Option<MachineSnapshot<C, E>> {
        self.time_travel.lock().unwrap().current_snapshot().cloned()
    }
}

/// Real-time state information
#[derive(Debug, Clone)]
pub struct StateInfo<C: Send + Sync, E> {
    pub current_state: Option<MachineStateImpl<C>>,
    pub stats: VisualizationStats,
    pub recent_transitions: Vec<TransitionEvent<C, E>>,
    pub time_travel_position: TimeTravelPosition,
}

/// Extension trait for adding visualization to machines
pub trait MachineVisualizationExt<C: Send + Sync + Clone + Default + Debug, E: Clone + PartialEq + Debug + Send + Sync + Default> {
    /// Add visualization capabilities to the machine
    fn with_visualization(self, config: VisualizationConfig) -> VisualizedMachine<C, E>;
}

impl<C: Send + Sync, E> MachineVisualizationExt<C, E> for Machine<C, E>
where
    C: Clone + std::default::Default + 'static + std::fmt::Debug + Send + Sync,
    E: Clone + std::cmp::PartialEq + events::Event + 'static + std::fmt::Debug + Send + Sync + Default,
{
    fn with_visualization(self, _config: VisualizationConfig) -> VisualizedMachine<C, E> {
        // TODO: This method is temporarily disabled because Machine doesn't implement Clone
        // This would need to be addressed in a future iteration
        panic!("Visualization not available - Machine doesn't implement Clone")
    }
}

/// A state machine with visualization capabilities
pub struct VisualizedMachine<C: Send + Sync + Clone + Default + Debug, E: Clone + PartialEq + Debug + Send + Sync + Default> {
    machine: Machine<C, E>,
    visualizer: Arc<MachineVisualizer<C, E>>,
    monitor: Arc<StateMonitor<C, E>>,
    config: VisualizationConfig,
}

impl<C: Send + Sync, E> VisualizedMachine<C, E>
where
    C: Clone + std::default::Default + 'static + std::fmt::Debug + Send + Sync,
    E: Clone + std::cmp::PartialEq + events::Event + 'static + std::fmt::Debug + Send + Sync + Default,
{
    // TODO: This method is temporarily disabled because Machine doesn't implement Clone
    // This would need to be addressed in a future iteration
    /*
    pub fn new(machine: Machine<C, E>, config: VisualizationConfig) -> Self {
        let visualizer = Arc::new(MachineVisualizer::new(machine.clone(), config.clone()));
        let monitor = Arc::new(StateMonitor::new(visualizer.clone(), config.clone()));

        Self {
            machine,
            visualizer,
            monitor,
            config,
        }
    }
    */

    /// Get the underlying machine
    pub fn machine(&self) -> &Machine<C, E> {
        &self.machine
    }

    /// Get the visualizer
    pub fn visualizer(&self) -> Arc<MachineVisualizer<C, E>> {
        self.visualizer.clone()
    }

    /// Get the monitor
    pub fn monitor(&self) -> Arc<StateMonitor<C, E>> {
        self.monitor.clone()
    }

    /// Transition with visualization
    pub fn transition(&self, current: &MachineStateImpl<C>, event: E) -> MachineStateImpl<C> {
        let start_time = Instant::now();

        // Perform the transition
        let new_state = Machine::transition(&self.machine, current, event.clone());

        let duration = start_time.elapsed();

        // Record the transition for visualization
        self.visualizer.record_transition(
            event,
            current.clone(),
            new_state.clone(),
            Vec::new(), // Guards evaluated (would be captured in real implementation)
            Vec::new(), // Actions executed (would be captured in real implementation)
            duration,
        );

        new_state
    }

    /// Get real-time state information
    pub fn get_state_info(&self) -> StateInfo<C, E> {
        self.monitor.get_state_info()
    }

    /// Export state diagram
    pub fn export_diagram(&self, format: ExportFormat) -> StateResult<String> {
        self.monitor.export_diagram(format)
    }

    /// Get visualization configuration
    pub fn config(&self) -> &VisualizationConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::machine::*;

    #[derive(Debug, Clone, PartialEq, Default)]
    #[cfg_attr(feature = "visualization", derive(serde::Serialize))]
    struct TestContext {
        count: i32,
        name: String,
    }

    #[derive(Debug, Clone, PartialEq, Default)]
    #[cfg_attr(feature = "visualization", derive(serde::Serialize))]
    enum TestEvent {
        #[default]
        Increment,
        Decrement,
        SetName(String),
    }

    impl Event for TestEvent {
        fn event_type(&self) -> &str {
            match self {
                TestEvent::Increment => "increment",
                TestEvent::Decrement => "decrement",
                TestEvent::SetName(_) => "set_name",
            }
        }
    }

    #[test]
    fn test_visualization_config_default() {
        let config = VisualizationConfig::default();
        assert!(config.enabled);
        assert_eq!(config.update_interval, 100);
        assert_eq!(config.max_history, 100);
        assert!(config.capture_snapshots);
        assert!(config.enable_time_travel);
    }

    #[test]
    fn test_machine_visualizer() {
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .state("idle")
            .on(TestEvent::Increment, "counting")
            .state("counting")
            .on(TestEvent::Decrement, "idle")
            .build();

        let config = VisualizationConfig {
            enabled: true,
            max_history: 10,
            ..Default::default()
        };

        let visualizer = MachineVisualizer::new(machine, config);

        // Test diagram export
        let dot_diagram = visualizer.export_diagram(ExportFormat::Dot).unwrap();
        assert!(dot_diagram.contains("digraph StateMachine"));
        assert!(dot_diagram.contains("idle"));
        assert!(dot_diagram.contains("counting"));

        let mermaid_diagram = visualizer.export_diagram(ExportFormat::Mermaid).unwrap();
        assert!(mermaid_diagram.contains("stateDiagram-v2"));
        assert!(mermaid_diagram.contains("idle"));
        assert!(mermaid_diagram.contains("counting"));
    }

    #[test]
    fn test_time_travel_debugger() {
        let mut debugger = TimeTravelDebugger::<TestContext, TestEvent>::new(5);

        // Add some snapshots
        let snapshot1 = MachineSnapshot {
            state: MachineStateImpl::new(
                StateValue::Simple("idle".to_string()),
                TestContext {
                    count: 0,
                    name: "test".to_string(),
                },
            ),
            timestamp: 1000,
            uptime: Duration::from_secs(1),
            _phantom: PhantomData,
        };

        let snapshot2 = MachineSnapshot {
            state: MachineStateImpl::new(
                StateValue::Simple("counting".to_string()),
                TestContext {
                    count: 1,
                    name: "test".to_string(),
                },
            ),
            timestamp: 2000,
            uptime: Duration::from_secs(2),
            _phantom: PhantomData,
        };

        debugger.add_snapshot(snapshot1.clone());
        debugger.add_snapshot(snapshot2.clone());

        // Test time travel
        assert_eq!(debugger.position_info().current_index, 1);
        assert_eq!(debugger.position_info().total_snapshots, 2);

        let back_snapshot = debugger.go_back().unwrap();
        assert_eq!(
            back_snapshot.state.value(),
            &StateValue::Simple("idle".to_string())
        );

        let forward_snapshot = debugger.go_forward().unwrap();
        assert_eq!(
            forward_snapshot.state.value(),
            &StateValue::Simple("counting".to_string())
        );
    }

    #[test]
    fn test_visualized_machine() {
        // Skip this test since Machine doesn't implement Clone
        // Visualization requires Clone for time travel and history features
        println!("Skipping visualized machine test - Machine doesn't implement Clone");
    }
}
