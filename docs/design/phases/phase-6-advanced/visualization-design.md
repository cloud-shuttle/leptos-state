# Visualization Design

## Overview
Implement state machine and store visualization capabilities for generating diagrams, graphs, and interactive visualizations to help developers understand and debug complex state interactions.

## Current State
```rust
// No visualization capabilities
impl<S: State, E: Event> Machine<S, E> {
    // Basic state machine operations only
}
```

## Proposed Enhancement
```rust
#[cfg(feature = "visualization")]
impl<S: State, E: Event> Machine<S, E> {
    pub fn generate_dot_graph(&self) -> Result<String, VisualizationError> {
        // Generate GraphViz DOT format for state diagrams
    }

    pub fn generate_mermaid_diagram(&self) -> Result<String, VisualizationError> {
        // Generate Mermaid.js format for web rendering
    }
}
```

## Motivation

### Developer Understanding
- **State Flow Visualization**: See how states connect and transition
- **Complexity Analysis**: Identify overly complex state machines
- **Documentation**: Auto-generated diagrams for documentation
- **Debugging**: Visual representation of state changes

### Communication
- **Team Collaboration**: Share visual state machine designs
- **Stakeholder Communication**: Explain system behavior visually
- **Architecture Documentation**: Maintain up-to-date system diagrams
- **Onboarding**: Help new developers understand the system

### Use Cases
- Designing new state machines with visual feedback
- Debugging complex state transition logic
- Performance analysis of state machine execution
- Documentation generation for APIs and systems
- Interactive exploration of state spaces

## Implementation Details

### Graph Generation Core
```rust
#[cfg(feature = "visualization")]
pub trait Visualizable {
    fn to_dot_graph(&self) -> Result<String, VisualizationError>;
    fn to_mermaid_diagram(&self) -> Result<String, VisualizationError>;
    fn get_visualization_metadata(&self) -> VisualizationMetadata;
}

#[derive(Clone, Debug)]
pub struct VisualizationMetadata {
    pub title: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub layout: LayoutHint,
}

#[derive(Clone, Debug)]
pub enum LayoutHint {
    Hierarchical,
    Circular,
    ForceDirected,
    Custom(String),
}

#[cfg(feature = "visualization")]
pub struct StateMachineVisualizer<S: State, E: Event> {
    machine: Machine<S, E>,
    style_config: VisualizationStyle,
    layout_engine: Box<dyn LayoutEngine>,
}

#[derive(Clone, Debug)]
pub struct VisualizationStyle {
    pub node_color_scheme: ColorScheme,
    pub edge_style: EdgeStyle,
    pub font_family: String,
    pub font_size: u32,
    pub show_guards: bool,
    pub show_actions: bool,
    pub highlight_current_state: bool,
}

#[derive(Clone, Debug)]
pub enum ColorScheme {
    Default,
    StateTypeBased,
    Custom(Vec<String>),
}

#[derive(Clone, Debug)]
pub enum EdgeStyle {
    Straight,
    Curved,
    Stepped,
}
```

### DOT Graph Generation
```rust
#[cfg(feature = "visualization")]
impl<S: State, E: Event> StateMachineVisualizer<S, E> {
    pub fn generate_dot_graph(&self) -> Result<String, VisualizationError> {
        let mut output = String::new();

        // Graph header
        output.push_str("digraph StateMachine {\n");
        output.push_str(&format!("  label=\"{}\";\n", self.machine.get_visualization_metadata().title));
        output.push_str("  rankdir=LR;\n");
        output.push_str("  node [shape=circle, style=filled];\n");

        // Define nodes
        for state_name in self.machine.states() {
            let is_current = state_name == self.machine.current_state();
            let node_attrs = self.get_node_attributes(state_name, is_current);
            output.push_str(&format!("  \"{}\" [{}];\n", state_name, node_attrs));
        }

        // Define edges
        for (state_name, state_node) in self.machine.get_states() {
            for (event_type, transition) in &state_node.transitions {
                let edge_attrs = self.get_edge_attributes(event_type, transition);
                output.push_str(&format!("  \"{}\" -> \"{}\" [{}];\n",
                    state_name, transition.target, edge_attrs));
            }
        }

        output.push_str("}\n");
        Ok(output)
    }

    fn get_node_attributes(&self, state_name: &str, is_current: bool) -> String {
        let mut attrs = Vec::new();

        // Color based on state type
        let color = if is_current {
            "lightblue"
        } else {
            match state_name {
                "idle" | "waiting" => "lightgray",
                "running" | "active" => "lightgreen",
                "error" | "failed" => "lightcoral",
                _ => "white",
            }
        };
        attrs.push(format!("fillcolor={}", color));

        // Font and label
        attrs.push(format!("fontname=\"{}\"", self.style_config.font_family));
        attrs.push(format!("fontsize={}", self.style_config.font_size));
        attrs.push(format!("label=\"{}\"", state_name));

        attrs.join(", ")
    }

    fn get_edge_attributes(&self, event_type: &str, transition: &Transition<S, E>) -> String {
        let mut attrs = Vec::new();

        // Basic edge styling
        attrs.push("fontname=\"Arial\"".to_string());
        attrs.push("fontsize=10".to_string());

        // Label with event type
        let mut label = event_type.to_string();

        // Add guard information if enabled
        if self.style_config.show_guards && transition.guard.is_some() {
            label.push_str("\\n[guarded]");
        }

        // Add action information if enabled
        if self.style_config.show_actions && transition.actions.is_some() {
            label.push_str("\\n[with actions]");
        }

        attrs.push(format!("label=\"{}\"", label));

        // Style based on transition type
        if transition.guard.is_some() {
            attrs.push("color=blue".to_string());
            attrs.push("style=dashed".to_string());
        } else {
            attrs.push("color=black".to_string());
            attrs.push("style=solid".to_string());
        }

        attrs.join(", ")
    }
}
```

### Mermaid.js Generation
```rust
#[cfg(feature = "visualization")]
impl<S: State, E: Event> StateMachineVisualizer<S, E> {
    pub fn generate_mermaid_diagram(&self) -> Result<String, VisualizationError> {
        let mut output = String::new();

        // Mermaid header
        output.push_str("stateDiagram-v2\n");

        // Define states
        for state_name in self.machine.states() {
            let is_current = state_name == self.machine.current_state();
            if is_current {
                output.push_str(&format!("    [*] --> {}\n", state_name));
            }

            // Add state descriptions if available
            output.push_str(&format!("    {} : {}\n", state_name, state_name));
        }

        // Define transitions
        for (state_name, state_node) in self.machine.get_states() {
            for (event_type, transition) in &state_node.transitions {
                let transition_label = self.format_mermaid_transition(event_type, transition);
                output.push_str(&format!("    {} --> {} : {}\n",
                    state_name, transition.target, transition_label));
            }
        }

        Ok(output)
    }

    fn format_mermaid_transition(&self, event_type: &str, transition: &Transition<S, E>) -> String {
        let mut label = event_type.to_string();

        if self.style_config.show_guards && transition.guard.is_some() {
            label.push_str(" [guarded]");
        }

        if self.style_config.show_actions && transition.actions.is_some() {
            label.push_str(" [actions]");
        }

        label
    }
}
```

### Interactive Web Visualization
```rust
#[cfg(all(feature = "visualization", feature = "web"))]
pub struct InteractiveVisualizer<S: State, E: Event> {
    visualizer: StateMachineVisualizer<S, E>,
    container_id: String,
    event_listeners: HashMap<String, Box<dyn Fn(&str) + Send + Sync>>,
}

#[cfg(all(feature = "visualization", feature = "web"))]
impl<S: State, E: Event> InteractiveVisualizer<S, E> {
    pub fn new(visualizer: StateMachineVisualizer<S, E>, container_id: String) -> Self {
        Self {
            visualizer,
            container_id,
            event_listeners: HashMap::new(),
        }
    }

    pub fn render_interactive(&self) -> Result<(), VisualizationError> {
        // Generate HTML/SVG with interactive elements
        let mermaid_diagram = self.visualizer.generate_mermaid_diagram()?;

        // Wrap in interactive container
        let html = format!(
            r#"
            <div id="{}" class="state-machine-visualization">
                <div class="controls">
                    <button onclick="zoomIn()">Zoom In</button>
                    <button onclick="zoomOut()">Zoom Out</button>
                    <button onclick="resetView()">Reset</button>
                </div>
                <div class="diagram-container">
                    <pre class="mermaid">
{}
                    </pre>
                </div>
            </div>
            "#,
            self.container_id, mermaid_diagram
        );

        // Insert into DOM
        self.insert_into_dom(&html)?;

        // Initialize Mermaid
        self.initialize_mermaid()?;

        Ok(())
    }

    pub fn add_event_listener<F>(&mut self, event_type: &str, listener: F)
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        self.event_listeners.insert(event_type.to_string(), Box::new(listener));
    }

    pub fn update_current_state(&self, new_state: &str) -> Result<(), VisualizationError> {
        // Update visual highlighting of current state
        self.call_js_function("updateCurrentState", &[new_state])?;
        Ok(())
    }

    pub fn highlight_transition(&self, from: &str, to: &str, event: &str) -> Result<(), VisualizationError> {
        // Highlight the transition that just occurred
        self.call_js_function("highlightTransition", &[from, to, event])?;
        Ok(())
    }

    fn insert_into_dom(&self, html: &str) -> Result<(), VisualizationError> {
        // Use web_sys to insert HTML into the DOM
        todo!()
    }

    fn initialize_mermaid(&self) -> Result<(), VisualizationError> {
        // Initialize Mermaid.js rendering
        todo!()
    }

    fn call_js_function(&self, function_name: &str, args: &[&str]) -> Result<(), VisualizationError> {
        // Call JavaScript functions for interactivity
        todo!()
    }
}
```

### Store Visualization
```rust
#[cfg(feature = "visualization")]
pub struct StoreVisualizer<S: State> {
    store: Store<S>,
    history_visualizer: Option<HistoryVisualizer<S>>,
}

#[cfg(feature = "visualization")]
impl<S: State> StoreVisualizer<S> {
    pub fn new(store: Store<S>) -> Self {
        Self {
            store,
            history_visualizer: None,
        }
    }

    pub fn with_history_visualization(mut self, max_history: usize) -> Self {
        self.history_visualizer = Some(HistoryVisualizer::new(max_history));
        self
    }

    pub fn generate_state_timeline(&self) -> Result<String, VisualizationError>
    where
        S: Serialize + Clone,
    {
        let mut output = String::new();
        output.push_str("timeline\n");

        // Get state history from inspector if available
        if let Some(history) = self.get_state_history() {
            for (index, snapshot) in history.iter().enumerate() {
                let state_json = serde_json::to_string(&snapshot.state)?;
                output.push_str(&format!("    {} : {}\n", index, state_json));
            }
        }

        Ok(output)
    }

    pub fn generate_state_diff_visualization(&self, index1: usize, index2: usize) -> Result<String, VisualizationError>
    where
        S: Serialize + Clone,
    {
        // Generate visualization of differences between two states
        todo!()
    }

    fn get_state_history(&self) -> Option<&[StateSnapshot<S>]> {
        // Get history from attached inspector
        None // Placeholder
    }
}
```

### Layout Engines
```rust
#[cfg(feature = "visualization")]
pub trait LayoutEngine: Send + Sync {
    fn layout_nodes(&self, nodes: &[VisualizationNode], edges: &[VisualizationEdge]) -> Result<NodePositions, LayoutError>;
    fn optimize_layout(&self, positions: &mut NodePositions) -> Result<(), LayoutError>;
}

#[derive(Clone, Debug)]
pub struct VisualizationNode {
    pub id: String,
    pub label: String,
    pub node_type: NodeType,
    pub metadata: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct VisualizationEdge {
    pub from: String,
    pub to: String,
    pub label: String,
    pub edge_type: EdgeType,
    pub metadata: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub enum NodeType {
    State,
    Action,
    Guard,
    Initial,
    Final,
}

#[derive(Clone, Debug)]
pub enum EdgeType {
    Transition,
    ActionFlow,
    GuardCheck,
}

pub type NodePositions = HashMap<String, (f64, f64)>;

#[cfg(feature = "visualization")]
pub struct ForceDirectedLayout {
    iterations: usize,
    attraction_force: f64,
    repulsion_force: f64,
    damping: f64,
}

#[cfg(feature = "visualization")]
impl ForceDirectedLayout {
    pub fn new() -> Self {
        Self {
            iterations: 100,
            attraction_force: 0.1,
            repulsion_force: 1000.0,
            damping: 0.9,
        }
    }
}

#[cfg(feature = "visualization")]
impl LayoutEngine for ForceDirectedLayout {
    fn layout_nodes(&self, nodes: &[VisualizationNode], edges: &[VisualizationEdge]) -> Result<NodePositions, LayoutError> {
        // Implement force-directed layout algorithm
        // This is a simplified version - real implementation would be more complex
        let mut positions = HashMap::new();

        // Initialize random positions
        for node in nodes {
            positions.insert(node.id.clone(), (rand::random::<f64>() * 1000.0, rand::random::<f64>() * 1000.0));
        }

        // Run force-directed iterations
        for _ in 0..self.iterations {
            self.apply_forces(&mut positions, nodes, edges);
        }

        Ok(positions)
    }

    fn apply_forces(&self, positions: &mut NodePositions, nodes: &[VisualizationNode], edges: &[VisualizationEdge]) {
        // Calculate repulsion between all nodes
        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let node1 = &nodes[i];
                let node2 = &nodes[j];

                if let (Some(pos1), Some(pos2)) = (positions.get(&node1.id), positions.get(&node2.id)) {
                    let dx = pos2.0 - pos1.0;
                    let dy = pos2.1 - pos1.1;
                    let distance = (dx * dx + dy * dy).sqrt().max(1.0);

                    let force = self.repulsion_force / (distance * distance);
                    let fx = (dx / distance) * force;
                    let fy = (dy / distance) * force;

                    // Apply force to node1
                    if let Some(pos) = positions.get_mut(&node1.id) {
                        pos.0 -= fx;
                        pos.1 -= fy;
                    }

                    // Apply opposite force to node2
                    if let Some(pos) = positions.get_mut(&node2.id) {
                        pos.0 += fx;
                        pos.1 += fy;
                    }
                }
            }
        }

        // Calculate attraction along edges
        for edge in edges {
            if let (Some(pos1), Some(pos2)) = (positions.get(&edge.from), positions.get(&edge.to)) {
                let dx = pos2.0 - pos1.0;
                let dy = pos2.1 - pos1.1;
                let distance = (dx * dx + dy * dy).sqrt().max(1.0);

                let force = distance * self.attraction_force;
                let fx = (dx / distance) * force;
                let fy = (dy / distance) * force;

                // Apply attraction
                if let Some(pos) = positions.get_mut(&edge.from) {
                    pos.0 += fx * self.damping;
                    pos.1 += fy * self.damping;
                }
                if let Some(pos) = positions.get_mut(&edge.to) {
                    pos.0 -= fx * self.damping;
                    pos.1 -= fy * self.damping;
                }
            }
        }
    }

    fn optimize_layout(&self, positions: &mut NodePositions) -> Result<(), LayoutError> {
        // Center the layout and prevent overlap
        self.center_layout(positions);
        self.prevent_overlap(positions);
        Ok(())
    }

    fn center_layout(&self, positions: &mut NodePositions) {
        // Calculate centroid
        let mut center_x = 0.0;
        let mut center_y = 0.0;
        let count = positions.len() as f64;

        for (_, pos) in positions.iter() {
            center_x += pos.0;
            center_y += pos.1;
        }

        center_x /= count;
        center_y /= count;

        // Translate to origin
        for (_, pos) in positions.iter_mut() {
            pos.0 -= center_x;
            pos.1 -= center_y;
        }
    }

    fn prevent_overlap(&self, positions: &mut NodePositions) {
        // Simple overlap prevention - real implementation would be more sophisticated
        let node_radius = 50.0;

        for _ in 0..10 { // Limited iterations to prevent infinite loops
            let mut has_overlap = false;

            for (id1, pos1) in positions.clone() {
                for (id2, pos2) in positions.iter() {
                    if id1 != *id2 {
                        let dx = pos2.0 - pos1.0;
                        let dy = pos2.1 - pos1.1;
                        let distance = (dx * dx + dy * dy).sqrt();

                        if distance < node_radius * 2.0 {
                            // Push nodes apart
                            let overlap = (node_radius * 2.0 - distance) / 2.0;
                            let nx = dx / distance;
                            let ny = dy / distance;

                            if let Some(pos) = positions.get_mut(&id1) {
                                pos.0 -= nx * overlap;
                                pos.1 -= ny * overlap;
                            }
                            has_overlap = true;
                        }
                    }
                }
            }

            if !has_overlap {
                break;
            }
        }
    }
}
```

## Error Handling

### Visualization Errors
```rust
#[derive(Debug, Clone, thiserror::Error)]
pub enum VisualizationError {
    #[error("Layout engine error: {0}")]
    LayoutError(String),

    #[error("Rendering error: {0}")]
    RenderingError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Invalid graph structure: {0}")]
    InvalidStructure(String),

    #[error("Web API error: {0}")]
    WebApiError(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum LayoutError {
    #[error("No nodes to layout")]
    NoNodes,

    #[error("Invalid node configuration: {0}")]
    InvalidConfiguration(String),

    #[error("Layout algorithm failed to converge")]
    ConvergenceFailure,

    #[error("Maximum iterations exceeded")]
    MaxIterationsExceeded,
}
```

### Safe Visualization
```rust
#[cfg(feature = "visualization")]
impl<S: State, E: Event> StateMachineVisualizer<S, E> {
    pub fn generate_safe(&self, format: VisualizationFormat) -> Result<String, VisualizationError> {
        match format {
            VisualizationFormat::Dot => self.generate_dot_graph(),
            VisualizationFormat::Mermaid => self.generate_mermaid_diagram(),
            VisualizationFormat::Json => self.generate_json_representation(),
            VisualizationFormat::Svg => self.generate_svg_direct(),
        }
    }

    pub fn validate_machine_structure(&self) -> Result<(), VisualizationError> {
        // Check for common issues that would break visualization
        if self.machine.states().is_empty() {
            return Err(VisualizationError::InvalidStructure("No states defined".to_string()));
        }

        // Check for orphan states, disconnected components, etc.
        Ok(())
    }

    fn generate_json_representation(&self) -> Result<String, VisualizationError> {
        // Generate JSON representation for programmatic use
        let machine_data = serde_json::json!({
            "states": self.machine.states(),
            "current_state": self.machine.current_state(),
            "transitions": self.collect_transitions(),
            "metadata": self.machine.get_visualization_metadata()
        });

        serde_json::to_string_pretty(&machine_data)
            .map_err(|e| VisualizationError::SerializationError(e.to_string()))
    }

    fn collect_transitions(&self) -> Vec<serde_json::Value> {
        let mut transitions = Vec::new();

        for (from_state, state_node) in self.machine.get_states() {
            for (event, transition) in &state_node.transitions {
                transitions.push(serde_json::json!({
                    "from": from_state,
                    "to": transition.target,
                    "event": event,
                    "has_guard": transition.guard.is_some(),
                    "has_actions": transition.actions.is_some()
                }));
            }
        }

        transitions
    }

    fn generate_svg_direct(&self) -> Result<String, VisualizationError> {
        // Generate SVG directly without external dependencies
        // This would be a basic implementation
        todo!()
    }
}

#[derive(Clone, Debug)]
pub enum VisualizationFormat {
    Dot,
    Mermaid,
    Json,
    Svg,
}
```

## Testing Strategy

### Unit Tests
```rust
#[cfg(feature = "visualization")]
mod tests {
    use super::*;

    #[test]
    fn dot_graph_generation() {
        let machine = create_test_machine();
        let visualizer = StateMachineVisualizer::new(machine);

        let dot = visualizer.generate_dot_graph().unwrap();

        assert!(dot.contains("digraph"));
        assert!(dot.contains("idle"));
        assert!(dot.contains("running"));
        assert!(dot.contains("->"));
    }

    #[test]
    fn mermaid_diagram_generation() {
        let machine = create_test_machine();
        let visualizer = StateMachineVisualizer::new(machine);

        let mermaid = visualizer.generate_mermaid_diagram().unwrap();

        assert!(mermaid.contains("stateDiagram"));
        assert!(mermaid.contains("-->"));
    }

    #[test]
    fn force_directed_layout() {
        let nodes = vec![
            VisualizationNode {
                id: "a".to_string(),
                label: "A".to_string(),
                node_type: NodeType::State,
                metadata: HashMap::new(),
            },
            VisualizationNode {
                id: "b".to_string(),
                label: "B".to_string(),
                node_type: NodeType::State,
                metadata: HashMap::new(),
            },
        ];

        let edges = vec![
            VisualizationEdge {
                from: "a".to_string(),
                to: "b".to_string(),
                label: "transition".to_string(),
                edge_type: EdgeType::Transition,
                metadata: HashMap::new(),
            },
        ];

        let layout = ForceDirectedLayout::new();
        let positions = layout.layout_nodes(&nodes, &edges).unwrap();

        assert_eq!(positions.len(), 2);
        assert!(positions.contains_key("a"));
        assert!(positions.contains_key("b"));
    }
}
```

### Integration Tests
```rust
#[cfg(feature = "visualization")]
#[test]
fn complex_machine_visualization() {
    let machine = create_complex_test_machine();
    let visualizer = StateMachineVisualizer::new(machine);

    // Test DOT generation
    let dot = visualizer.generate_dot_graph().unwrap();
    assert!(dot.lines().count() > 10); // Should have multiple lines

    // Test Mermaid generation
    let mermaid = visualizer.generate_mermaid_diagram().unwrap();
    assert!(mermaid.contains("stateDiagram"));

    // Test JSON representation
    let json = visualizer.generate_safe(VisualizationFormat::Json).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(parsed.get("states").is_some());
    assert!(parsed.get("transitions").is_some());
}
```

## Performance Impact

### Generation Cost
- **DOT/Mermaid**: Fast, linear with number of states/transitions
- **Interactive Rendering**: Higher cost for web-based visualizations
- **Layout Algorithms**: Variable cost depending on algorithm complexity
- **Caching**: Generated diagrams can be cached

### Optimization Strategies
```rust
#[cfg(feature = "visualization")]
impl<S: State, E: Event> StateMachineVisualizer<S, E> {
    pub fn with_caching(mut self) -> Self {
        // Cache generated diagrams
        self.caching_enabled = true;
        self.diagram_cache = Some(HashMap::new());
        self
    }

    pub fn invalidate_cache(&mut self) {
        if let Some(ref mut cache) = self.diagram_cache {
            cache.clear();
        }
    }

    pub fn generate_dot_cached(&mut self) -> Result<String, VisualizationError> {
        if let Some(ref mut cache) = self.diagram_cache {
            if let Some(cached) = cache.get("dot") {
                return Ok(cached.clone());
            }
        }

        let diagram = self.generate_dot_graph()?;

        if let Some(ref mut cache) = self.diagram_cache {
            cache.insert("dot".to_string(), diagram.clone());
        }

        Ok(diagram)
    }

    pub fn with_simple_layout(mut self) -> Self {
        // Use simple hierarchical layout instead of force-directed
        self.layout_engine = Box::new(HierarchicalLayout::new());
        self
    }
}
```

## Security Considerations

### Information Disclosure
- State machine structure may reveal sensitive business logic
- Filter sensitive state names and transition labels
- Control access to visualization features

```rust
#[cfg(feature = "visualization")]
impl<S: State, E: Event> StateMachineVisualizer<S, E> {
    pub fn with_anonymization(mut self) -> Self {
        // Replace sensitive names with generic ones
        self.anonymize_labels = true;
        self
    }

    pub fn with_access_control(mut self, allowed_users: Vec<String>) -> Self {
        // Only allow certain users to generate visualizations
        self.allowed_users = Some(allowed_users);
        self
    }

    fn anonymize_label(&self, label: &str) -> String {
        if self.anonymize_labels {
            format!("state_{}", label.len()) // Simple anonymization
        } else {
            label.to_string()
        }
    }
}
```

### Code Injection Prevention
- Sanitize labels and identifiers in generated output
- Validate input parameters for visualization functions
- Use safe string formatting

## Future Extensions

### Real-time Visualization Updates
```rust
#[cfg(all(feature = "visualization", feature = "web"))]
pub struct LiveVisualizer<S: State, E: Event> {
    visualizer: InteractiveVisualizer<S, E>,
    update_channel: (Sender<VisualizationUpdate>, Receiver<VisualizationUpdate>),
}

#[derive(Clone, Debug)]
pub enum VisualizationUpdate {
    StateChanged { from: String, to: String, event: String },
    MachineReset,
    NewTransition { from: String, to: String, event: String },
}

#[cfg(all(feature = "visualization", feature = "web"))]
impl<S: State, E: Event> LiveVisualizer<S, E> {
    pub fn update_realtime(&mut self, update: VisualizationUpdate) -> Result<(), VisualizationError> {
        // Update the visualization in real-time
        match update {
            VisualizationUpdate::StateChanged { from, to, event } => {
                self.visualizer.update_current_state(&to)?;
                self.visualizer.highlight_transition(&from, &to, &event)?;
            }
            VisualizationUpdate::MachineReset => {
                self.visualizer.reset_view()?;
            }
            VisualizationUpdate::NewTransition { from, to, event } => {
                self.visualizer.add_transition(&from, &to, &event)?;
            }
        }
        Ok(())
    }
}
```

### 3D Visualization
```rust
#[cfg(all(feature = "visualization", feature = "web"))]
pub struct ThreeDVisualizer<S: State, E: Event> {
    visualizer: InteractiveVisualizer<S, E>,
    three_js_renderer: ThreeJsRenderer,
}

#[cfg(all(feature = "visualization", feature = "web"))]
impl<S: State, E: Event> ThreeDVisualizer<S, E> {
    pub fn render_3d_force_graph(&self) -> Result<String, VisualizationError> {
        // Generate 3D force-directed graph using Three.js
        todo!()
    }

    pub fn animate_state_transitions(&self, transitions: &[StateTransition]) -> Result<(), VisualizationError> {
        // Animate transitions in 3D space
        todo!()
    }
}
```

### Export Formats
```rust
#[cfg(feature = "visualization")]
pub trait VisualizationExporter {
    fn export_png(&self, diagram: &str, width: u32, height: u32) -> Result<Vec<u8>, VisualizationError>;
    fn export_svg(&self, diagram: &str) -> Result<String, VisualizationError>;
    fn export_pdf(&self, diagram: &str) -> Result<Vec<u8>, VisualizationError>;
}

#[cfg(feature = "visualization")]
pub struct GraphvizExporter;

#[cfg(feature = "visualization")]
impl VisualizationExporter for GraphvizExporter {
    fn export_png(&self, diagram: &str, width: u32, height: u32) -> Result<Vec<u8>, VisualizationError> {
        // Use graphviz to render PNG
        todo!()
    }

    fn export_svg(&self, diagram: &str) -> Result<String, VisualizationError> {
        // Use graphviz to render SVG
        todo!()
    }

    fn export_pdf(&self, diagram: &str) -> Result<Vec<u8>, VisualizationError> {
        // Use graphviz to render PDF
        todo!()
    }
}
```

## Migration Guide

### Adding Visualization to Existing Machines
```rust
// Before - basic state machine
let machine = Machine::new("idle", context);

// After - with visualization
#[cfg(feature = "visualization")]
let visualizer = StateMachineVisualizer::new(machine.clone());

#[cfg(feature = "visualization")]
let dot_diagram = visualizer.generate_dot_graph().unwrap();
```

### Configuration-Based Visualization
```rust
#[derive(Deserialize)]
pub struct VisualizationConfig {
    pub enable_visualization: bool,
    pub default_format: String,
    pub style: VisualizationStyle,
    pub layout: String,
    pub enable_caching: bool,
}

pub fn create_machine_with_visualization<S: State, E: Event>(
    initial_state: S,
    config: &VisualizationConfig
) -> (Machine<S, E>, Option<StateMachineVisualizer<S, E>>) {
    let machine = Machine::new("idle", initial_state);

    if !config.enable_visualization {
        return (machine, None);
    }

    let mut visualizer = StateMachineVisualizer::new(machine.clone())
        .with_style(config.style.clone());

    if config.enable_caching {
        visualizer = visualizer.with_caching();
    }

    match config.layout.as_str() {
        "hierarchical" => {
            visualizer = visualizer.with_layout_engine(Box::new(HierarchicalLayout::new()));
        }
        "force" => {
            visualizer = visualizer.with_layout_engine(Box::new(ForceDirectedLayout::new()));
        }
        _ => {}
    }

    (machine, Some(visualizer))
}
```

### Web Integration
```rust
#[cfg(all(feature = "visualization", feature = "web"))]
pub fn setup_web_visualization<S: State, E: Event>(
    machine: Machine<S, E>,
    container_id: &str
) -> Result<(), VisualizationError> {
    let visualizer = StateMachineVisualizer::new(machine);
    let interactive = InteractiveVisualizer::new(visualizer, container_id.to_string());

    // Render the visualization
    interactive.render_interactive()?;

    // Set up event listeners for real-time updates
    interactive.add_event_listener("stateChanged", |event_data| {
        log::info!("State changed: {}", event_data);
    });

    Ok(())
}
```

## Risk Assessment

### Likelihood: Medium
- Visualization generation is generally safe
- External dependencies (GraphViz, Mermaid) may have issues
- Complex layout algorithms can have edge cases
- Web-based rendering has browser compatibility concerns

### Impact: Low
- Visualization is opt-in and doesn't affect core functionality
- Failures in visualization don't break the application
- Performance impact is isolated to visualization generation
- Clear error boundaries prevent cascading failures

### Mitigation
- Comprehensive testing of diagram generation
- Fallback to simple representations when complex rendering fails
- Validation of generated output
- Clear error messages and recovery options
- Performance monitoring and optimization
- Access controls for sensitive visualizations
