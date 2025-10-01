//! State machine and store visualization
//!
//! This module provides visualization capabilities for generating diagrams
//! and graphs to help developers understand complex state interactions.

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
pub struct StateMachineVisualizer<'a, S: State, E: Event> {
    machine: &'a Machine<S, E>,
    metadata: VisualizationMetadata,
    include_entry_actions: bool,
    include_exit_actions: bool,
    include_guards: bool,
}

impl<'a, S: State, E: Event> StateMachineVisualizer<'a, S, E> {
    /// Create a new visualizer for a state machine
    pub fn new(machine: &'a Machine<S, E>) -> Self {
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
        dot.push_str("  node [shape=circle, style=filled, fillcolor=lightblue];\n");
        dot.push_str("  edge [fontsize=10];\n\n");

        // Add nodes for states (simplified - we don't have direct access to states map)
        // In a real implementation, we would need to expose the states from Machine
        dot.push_str(&format!("  \"{}\" [fillcolor=green];\n", self.machine.current_state()));

        // Add transitions (simplified - we don't have direct access to transitions)
        // In a real implementation, we would iterate through all states and their transitions
        dot.push_str("  // Transitions would be added here\n");

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
            mermaid.push_str(&format!("    note right of {}\n        {}\n    end note\n", self.machine.current_state(), desc));
        }

        // Add current state
        mermaid.push_str(&format!("    [*] --> {}\n", self.machine.current_state()));

        // Add state styling
        mermaid.push_str(&format!("    {} : {}\n", self.machine.current_state(), self.machine.current_state()));

        // Add transitions (simplified - we don't have direct access to transitions)
        mermaid.push_str("    note right of State\n        Transitions would be shown here\n    end note\n");

        Ok(mermaid)
    }
}

impl<'a, S: State, E: Event> Visualizable for StateMachineVisualizer<'a, S, E> {
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
impl<S: State, E: Event> Machine<S, E> {
    /// Create a visualizer for this machine
    #[cfg(feature = "visualization")]
    pub fn visualizer(&self) -> StateMachineVisualizer<'_, S, E> {
        StateMachineVisualizer::new(self)
    }

    /// Generate DOT graph for this machine
    #[cfg(feature = "visualization")]
    pub fn to_dot_graph(&self) -> Result<String, VisualizationError> {
        self.visualizer().to_dot_graph()
    }

    /// Generate Mermaid diagram for this machine
    #[cfg(feature = "visualization")]
    pub fn to_mermaid_diagram(&self) -> Result<String, VisualizationError> {
        self.visualizer().to_mermaid_diagram()
    }
}

/// Simple DOT graph generator for basic state machines
pub fn generate_simple_dot_graph<S: State>(
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
pub fn generate_simple_mermaid_diagram<S: State>(
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
pub fn export_visualization<S: State + Serialize, E: Event>(
    machine: &Machine<S, E>,
    format: VisualizationFormat,
) -> Result<String, VisualizationError> {
    match format {
        VisualizationFormat::Dot => machine.to_dot_graph(),
        VisualizationFormat::Mermaid => machine.to_mermaid_diagram(),
        VisualizationFormat::Json => {
            // Simple JSON representation
            let data = serde_json::json!({
                "current_state": machine.current_state(),
                "states": "states_not_exposed", // Would need Machine to expose states
                "transitions": "transitions_not_exposed", // Would need Machine to expose transitions
            });
            serde_json::to_string_pretty(&data)
                .map_err(|e| VisualizationError::SerializationFailed {
                    message: e.to_string(),
                })
        }
    }
}
