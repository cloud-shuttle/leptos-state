//! Extension traits for visualization

use super::*;
use crate::machine::visualization::monitor::{StateMonitor, HealthChecker, HealthCheckResult};

/// Extension trait for adding visualization to machines
pub trait MachineVisualizationExt<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static> {
    /// Create a visualizer for this machine
    fn visualizer(&self) -> MachineVisualizer<C, E>;

    /// Create a visualizer with custom configuration
    fn visualizer_with_config(&self, config: VisualizationConfig) -> MachineVisualizer<C, E>;

    /// Export state diagram in the specified format
    fn export_diagram(&self, format: ExportFormat) -> Result<String, String>;

    /// Create a state diagram representation
    fn to_state_diagram(&self) -> StateDiagram<C, E>;

    /// Get a monitor for this machine
    fn monitor(&self) -> StateMonitor<C, E>;
}

impl<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static> MachineVisualizationExt<C, E> for Machine<C, E, C> {
    fn visualizer(&self) -> MachineVisualizer<C, E> {
        MachineVisualizer::new().with_machine(self.clone())
    }

    fn visualizer_with_config(&self, config: VisualizationConfig) -> MachineVisualizer<C, E> {
        MachineVisualizer::with_config(config).with_machine(self.clone())
    }

    fn export_diagram(&self, format: ExportFormat) -> Result<String, String> {
        let visualizer = self.visualizer();
        visualizer.export_diagram(format)
    }

    fn to_state_diagram(&self) -> StateDiagram<C, E> {
        let config = VisualizationConfig::default();
        StateDiagram::new(self, &config)
    }

    fn monitor(&self) -> StateMonitor<C, E> {
        StateMonitor::new().with_machine(self.clone())
    }
}

/// A state machine with visualization capabilities
#[derive(Debug)]
pub struct VisualizedMachine<C: Clone + Send + Sync + std::fmt::Debug + Default + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static> {
    /// The underlying machine
    pub machine: Machine<C, E, C>,
    /// The current state
    pub current_state: MachineStateImpl<C>,
    /// The visualizer
    pub visualizer: MachineVisualizer<C, E>,
    /// The monitor
    pub monitor: StateMonitor<C, E>,
    /// Time travel debugger
    pub debugger: TimeTravelDebugger<C, E>,
}

impl<C: Clone + Send + Sync + std::fmt::Debug + Default + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static> VisualizedMachine<C, E> {
    /// Create a new visualized machine
    pub fn new(machine: Machine<C, E, C>) -> Self {
        let current_state = machine.initial_state();
        let visualizer = machine.visualizer();
        let monitor = machine.monitor();
        let debugger = TimeTravelDebugger::new();

        Self {
            machine,
            current_state,
            visualizer,
            monitor,
            debugger,
        }
    }

    /// Create with custom configuration
    pub fn with_config(machine: Machine<C, E, C>, config: VisualizationConfig) -> Self {
        let current_state = machine.initial_state();
        let visualizer = machine.visualizer_with_config(config);
        let monitor = machine.monitor();
        let debugger = TimeTravelDebugger::new();

        Self {
            machine,
            current_state,
            visualizer,
            monitor,
            debugger,
        }
    }

    /// Transition with full visualization tracking
    pub fn transition(&mut self, event: &E) -> Result<(), String> {
        let start_time = std::time::Instant::now();
        let from_state = self.current_state.clone();

        // Create transition event
        let transition_event = TransitionEvent::success(
            from_state.clone(),
            String::new(), // Will be filled after transition
            Some(event.clone()),
            None, // Context not easily accessible
        );

        // Perform the transition
        let new_state = self.machine.transition(&self.current_state, event.clone());
        self.current_state = new_state.clone();

        // Complete the transition event
        let mut completed_event = transition_event;
        completed_event.to_state = new_state.value().to_string();
        completed_event.success = true;

        // Record the event
        self.visualizer.record_transition(completed_event.clone())?;

        // Notify monitor
        let state_change = StateChangeEvent::new(
            from_state.value().to_string(),
            new_state.value().to_string(),
            StateChangeType::Transition,
        );
        self.monitor.notify_state_change(&state_change);

        // Record performance
        let duration = start_time.elapsed();
        let perf_event = PerformanceEvent::new(PerformanceEventType::Transition, duration);
        self.visualizer.record_performance(perf_event)?;

        Ok(())
    }

    /// Get current state
    pub fn current_state(&self) -> &MachineStateImpl<C> {
        &self.current_state
    }

    /// Get current context
    pub fn current_context(&self) -> &C {
        self.current_state.context()
    }

    /// Export current diagram
    pub fn export_diagram(&self, format: ExportFormat) -> Result<String, String> {
        self.visualizer.export_diagram(format)
    }

    /// Get visualization statistics
    pub fn get_stats(&self) -> VisualizationStats {
        let mut stats = VisualizationStats::default();
        stats.update(&self.visualizer);
        stats
    }

    /// Enable or disable visualization
    pub fn set_visualization_enabled(&mut self, enabled: bool) {
        self.visualizer.set_enabled(enabled);
        self.monitor.set_enabled(enabled);
    }

    /// Clear all recorded data
    pub fn clear_data(&mut self) {
        self.visualizer.clear();
        self.monitor.reset_stats();
        self.debugger.clear();
    }
}

/// Automatic visualization integration
pub struct AutoVisualizer<C: Clone + Send + Sync + 'static, E: Clone + Send + Sync + 'static> {
    /// The visualized machine
    pub machine: VisualizedMachine<C, E>,
    /// Auto-export settings
    pub auto_export: Option<AutoExportSettings>,
    /// Health checker
    pub health_checker: Option<HealthChecker<C, E>>,
}

#[derive(Debug, Clone)]
pub struct AutoExportSettings {
    /// Export format
    pub format: ExportFormat,
    /// Export interval
    pub interval: std::time::Duration,
    /// Export path
    pub path: std::path::PathBuf,
    /// Last export time
    pub last_export: Option<std::time::Instant>,
}

impl<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + 'static> AutoVisualizer<C, E> {
    /// Create a new auto visualizer
    pub fn new(machine: Machine<C, E, C>) -> Self {
        Self {
            machine: VisualizedMachine::new(machine),
            auto_export: None,
            health_checker: None,
        }
    }

    /// Enable auto-export
    pub fn with_auto_export(mut self, settings: AutoExportSettings) -> Self {
        self.auto_export = Some(settings);
        self
    }

    /// Enable health checking
    pub fn with_health_checking(mut self, checker: HealthChecker<C, E>) -> Self {
        self.health_checker = Some(checker);
        self
    }

    /// Process auto-export if needed
    pub fn process_auto_export(&mut self) -> Result<(), String> {
        if let Some(ref mut settings) = self.auto_export {
            let now = std::time::Instant::now();

            let should_export = if let Some(last) = settings.last_export {
                now.duration_since(last) >= settings.interval
            } else {
                true
            };

            if should_export {
                let diagram = self.machine.export_diagram(settings.format.clone())?;
                std::fs::write(&settings.path, diagram)
                    .map_err(|e| format!("Failed to write diagram: {}", e))?;

                settings.last_export = Some(now);
            }
        }

        Ok(())
    }

    /// Perform health checks
    pub fn perform_health_checks(&mut self) -> Option<Vec<HealthCheckResult>> {
        if let Some(ref mut checker) = self.health_checker {
            Some(checker.perform_checks(&self.machine.machine, &self.machine.monitor))
        } else {
            None
        }
    }

    /// Get overall health status
    pub fn health_status(&self) -> Option<HealthStatus> {
        self.health_checker.as_ref().map(|c| c.overall_status())
    }

    /// Transition with full auto-visualization
    pub fn transition(&mut self, event: &E) -> Result<(), String> {
        let result = self.machine.transition(event);

        // Process auto-export
        if let Err(e) = self.process_auto_export() {
            // Log but don't fail the transition
            eprintln!("Auto-export failed: {}", e);
        }

        result
    }
}

/// Fluent API for creating visualizations
pub mod visualization {
    use super::*;

    /// Create a basic visualizer
    pub fn visualizer<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static>(machine: &Machine<C, E, C>) -> MachineVisualizer<C, E> {
        machine.visualizer()
    }

    /// Create a visualizer with custom config
    pub fn visualizer_with_config<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static>(
        machine: &Machine<C, E, C>,
        config: VisualizationConfig,
    ) -> MachineVisualizer<C, E> {
        machine.visualizer_with_config(config)
    }

    /// Create a visualized machine
    pub fn visualized_machine<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static>(
        machine: Machine<C, E, C>,
    ) -> VisualizedMachine<C, E> {
        VisualizedMachine::new(machine)
    }

    /// Create an auto-visualizer
    pub fn auto_visualizer<C: Clone + Send + Sync + std::fmt::Debug + 'static, E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static>(machine: Machine<C, E, C>) -> AutoVisualizer<C, E> {
        AutoVisualizer::new(machine)
    }

    /// Create a dark theme
    pub fn dark_theme() -> VisualizationTheme {
        VisualizationTheme::dark()
    }

    /// Create a high contrast theme
    pub fn high_contrast_theme() -> VisualizationTheme {
        VisualizationTheme::high_contrast()
    }

    /// Create default rendering options
    pub fn rendering_options() -> RenderingOptions {
        RenderingOptions::default()
    }
}
