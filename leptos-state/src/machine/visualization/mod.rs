//! Visualization functionality for state machines
//!
//! This module provides visualization capabilities for generating diagrams
//! and graphs to help developers understand complex state interactions.

pub mod monitor;

// Re-export monitor functionality
pub use monitor::*;

use crate::{State, Event, Machine};
use serde::{Deserialize, Serialize};

/// Errors that can occur during visualization
#[derive(Debug, Clone, thiserror::Error)]
pub enum VisualizationError {
    #[error("Visualization not supported for this type")]
    NotSupported,
    #[error("Serialization failed: {message}")]
    SerializationFailed { message: String },
    #[error("Invalid state machine structure: {reason}")]
    InvalidStructure { reason: String },
}

/// Layout hint for diagram generation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LayoutHint {
    /// Hierarchical layout (top-down)
    Hierarchical,
    /// Circular layout
    Circular,
    /// Force-directed layout
    ForceDirected,
    /// Custom layout specification
    Custom(String),
}

/// Metadata for visualization
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VisualizationMetadata {
    pub title: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub layout: LayoutHint,
}

impl Default for VisualizationMetadata {
    fn default() -> Self {
        Self {
            title: "State Machine".to_string(),
            description: None,
            tags: Vec::new(),
            layout: LayoutHint::Hierarchical,
        }
    }
}

/// Trait for types that can be visualized
pub trait Visualizable {
    /// Generate GraphViz DOT format
    fn to_dot_graph(&self) -> Result<String, VisualizationError>;

    /// Generate Mermaid.js diagram format
    fn to_mermaid_diagram(&self) -> Result<String, VisualizationError>;

    /// Get visualization metadata
    fn get_visualization_metadata(&self) -> VisualizationMetadata {
        VisualizationMetadata::default()
    }
}

/// State machine visualizer
pub struct StateMachineVisualizer<'a, C: State, E: Event> {
    machine: &'a Machine<C, E, C>,
    metadata: VisualizationMetadata,
    include_entry_actions: bool,
    include_exit_actions: bool,
    include_guards: bool,
}

impl<'a, C: State, E: Event> StateMachineVisualizer<'a, C, E> {
    /// Create a new visualizer for a state machine
    pub fn new(machine: &'a Machine<C, E, C>) -> Self {
        Self {
            machine,
            metadata: VisualizationMetadata::default(),
            include_entry_actions: true,
            include_exit_actions: true,
            include_guards: true,
        }
    }

    /// Set visualization metadata
    pub fn with_metadata(mut self, metadata: VisualizationMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Include entry actions in visualization
    pub fn with_entry_actions(mut self, include: bool) -> Self {
        self.include_entry_actions = include;
        self
    }

    /// Include exit actions in visualization
    pub fn with_exit_actions(mut self, include: bool) -> Self {
        self.include_exit_actions = include;
        self
    }

    /// Include guards in visualization
    pub fn with_guards(mut self, include: bool) -> Self {
        self.include_guards = include;
        self
    }

    /// Generate DOT format for GraphViz
    pub fn to_dot(&self) -> Result<String, VisualizationError> {
        let mut dot = String::new();

        // Start the graph
        dot.push_str(&format!("digraph \"{}\" {{\n", self.metadata.title));
        dot.push_str("  rankdir=TB;\n");
        dot.push_str("  node [shape=rectangle, style=filled, fillcolor=lightblue, fontsize=10];\n");
        dot.push_str("  edge [fontsize=9, fontcolor=blue];\n\n");

        // Add nodes for states
        for (state_name, state_node) in &self.machine.states {
            let is_current = *state_name == self.machine.initial;
            let fillcolor = if is_current { "lightgreen" } else { "lightblue" };
            let shape = if is_current { "rectangle" } else { "rectangle" };

            dot.push_str(&format!("  \"{}\" [shape={}, fillcolor={}, label=\"{}\\n", state_name, shape, fillcolor, state_name));

            // Add actions if enabled
            if self.include_entry_actions && !state_node.entry_actions.is_empty() {
                dot.push_str(&format!("Entry: {} actions\\n", state_node.entry_actions.len()));
            }
            if self.include_exit_actions && !state_node.exit_actions.is_empty() {
                dot.push_str(&format!("Exit: {} actions\\n", state_node.exit_actions.len()));
            }

            dot.push_str("\"];\n");
        }

        dot.push_str("\n");

        // Add initial state marker
        dot.push_str(&format!("  \"{}_init\" [shape=point, fillcolor=black, label=\"\"];\n", self.machine.initial));
        dot.push_str(&format!("  \"{}_init\" -> \"{}\" [label=\"initial\"];\n\n", self.machine.initial, self.machine.initial));

        // Add transitions
        for (from_state, state_node) in &self.machine.states {
            for transition in &state_node.transitions {
                let event_display = format!("{:?}", transition.event);
                let label = if self.include_guards && !transition.guards.is_empty() {
                    format!("{}\\n[{} guards]", event_display, transition.guards.len())
                } else {
                    event_display
                };

                let action_count = transition.actions.len();
                let action_label = if action_count > 0 {
                    format!("\\n{} actions", action_count)
                } else {
                    String::new()
                };

                dot.push_str(&format!("  \"{}\" -> \"{}\" [label=\"{}{}\"];\n",
                    from_state, transition.target, label, action_label));
            }
        }

        // Close the graph
        dot.push_str("}\n");

        Ok(dot)
    }

    /// Generate Mermaid.js state diagram
    pub fn to_mermaid(&self) -> Result<String, VisualizationError> {
        let mut mermaid = String::new();

        // Start the state diagram
        mermaid.push_str("stateDiagram-v2\n");

        // Add title if provided
        if let Some(ref desc) = self.metadata.description {
            mermaid.push_str(&format!("    note right of {}\n        {}\n    end note\n", self.machine.initial, desc));
        }

        // Add initial state
        mermaid.push_str(&format!("    [*] --> {}\n", self.machine.initial));

        // Add states
        for (state_name, _state_node) in &self.machine.states {
            if *state_name == self.machine.initial {
                mermaid.push_str(&format!("    {} : Initial State\\n{}\n", state_name, state_name));
            } else {
                mermaid.push_str(&format!("    {} : {}\n", state_name, state_name));
            }
        }

        mermaid.push_str("\n");

        // Add transitions
        for (from_state, state_node) in &self.machine.states {
            for transition in &state_node.transitions {
                let event_display = format!("{:?}", transition.event);
                let guard_note = if self.include_guards && !transition.guards.is_empty() {
                    format!(" [{} guards]", transition.guards.len())
                } else {
                    String::new()
                };

                let action_note = if transition.actions.len() > 0 {
                    format!(" / {} actions", transition.actions.len())
                } else {
                    String::new()
                };

                mermaid.push_str(&format!("    {} --> {} : {}{}{}\n",
                    from_state, transition.target, event_display, guard_note, action_note));
            }
        }

        Ok(mermaid)
    }
}

impl<'a, C: State, E: Event> Visualizable for StateMachineVisualizer<'a, C, E> {
    fn to_dot_graph(&self) -> Result<String, VisualizationError> {
        self.to_dot()
    }

    fn to_mermaid_diagram(&self) -> Result<String, VisualizationError> {
        self.to_mermaid()
    }

    fn get_visualization_metadata(&self) -> VisualizationMetadata {
        self.metadata.clone()
    }
}

/// Add visualization methods to Machine
impl<C: State, E: Event> Machine<C, E, C> {
    /// Create a visualizer for this machine
    pub fn visualizer(&self) -> StateMachineVisualizer<'_, C, E> {
        StateMachineVisualizer::new(self)
    }

    /// Generate DOT graph for this machine
    pub fn to_dot_graph(&self) -> Result<String, VisualizationError> {
        self.visualizer().to_dot_graph()
    }

    /// Generate Mermaid diagram for this machine
    pub fn to_mermaid_diagram(&self) -> Result<String, VisualizationError> {
        self.visualizer().to_mermaid_diagram()
    }
}

/// Simple DOT graph generator for basic state machines
pub fn generate_simple_dot_graph(
    states: &[&str],
    transitions: &[(&str, &str, &str)], // (from, event, to)
    current_state: &str,
) -> String {
    let mut dot = String::new();

    dot.push_str("digraph StateMachine {\n");
    dot.push_str("  rankdir=TB;\n");
    dot.push_str("  node [shape=circle, style=filled];\n\n");

    // Add states
    for state in states {
        let color = if *state == current_state { "green" } else { "lightblue" };
        dot.push_str(&format!("  \"{}\" [fillcolor={}];\n", state, color));
    }

    dot.push_str("\n");

    // Add transitions
    for (from, event, to) in transitions {
        dot.push_str(&format!("  \"{}\" -> \"{}\" [label=\"{}\"];\n", from, to, event));
    }

    dot.push_str("}\n");
    dot
}

/// Simple Mermaid diagram generator
pub fn generate_simple_mermaid_diagram(
    states: &[&str],
    transitions: &[(&str, &str, &str)], // (from, event, to)
    current_state: &str,
) -> String {
    let mut mermaid = String::new();

    mermaid.push_str("stateDiagram-v2\n");

    // Add initial state
    mermaid.push_str(&format!("    [*] --> {}\n", current_state));

    // Add states
    for state in states {
        if *state == current_state {
            mermaid.push_str(&format!("    {} : Current State\n", state));
        } else {
            mermaid.push_str(&format!("    {} : {}\n", state, state));
        }
    }

    mermaid.push_str("\n");

    // Add transitions
    for (from, event, to) in transitions {
        mermaid.push_str(&format!("    {} --> {} : {}\n", from, to, event));
    }

    mermaid
}

/// Export visualization to different formats
pub enum VisualizationFormat {
    /// GraphViz DOT format
    Dot,
    /// Mermaid.js format
    Mermaid,
    /// JSON representation
    Json,
}

/// Export visualization data
pub fn export_visualization<C: State + Serialize, E: Event>(
    machine: &Machine<C, E, C>,
    format: VisualizationFormat,
) -> Result<String, VisualizationError> {
    match format {
        VisualizationFormat::Dot => machine.to_dot_graph(),
        VisualizationFormat::Mermaid => machine.to_mermaid_diagram(),
        VisualizationFormat::Json => {
            // JSON representation with available data
            let states: Vec<String> = machine.states.keys().cloned().collect();
            let transitions: Vec<serde_json::Value> = machine.states.iter()
                .flat_map(|(from, node)| {
                    node.transitions.iter().map(move |trans| {
                        serde_json::json!({
                            "from": from,
                            "event": format!("{:?}", trans.event),
                            "to": trans.target,
                            "guards": trans.guards.len(),
                            "actions": trans.actions.len()
                        })
                    })
                })
                .collect();

            let data = serde_json::json!({
                "initial_state": machine.initial,
                "states": states,
                "transitions": transitions,
                "metadata": VisualizationMetadata::default()
            });

            serde_json::to_string_pretty(&data)
                .map_err(|e| VisualizationError::SerializationFailed {
                    message: e.to_string(),
                })
        }
    }
}
