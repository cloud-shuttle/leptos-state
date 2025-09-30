//! Transition and state management generation

use crate::machine::codegen::config::CodeGenConfig;
use crate::machine::{Machine, MachineStateImpl};

impl<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + std::hash::Hash + Eq + 'static>
    super::generator::CodeGenerator<C, E>
{
    /// Generate transitions
    pub fn generate_transitions(&self, machine: &Machine<C, E, C>, machine_name: &str) -> Result<String, String> {
        let mut code = String::new();

        match self.config.language.as_str() {
            "rust" => {
                code.push_str(&format!("    /// Transition to a new state\n"));
                code.push_str(&format!("    pub fn transition(&mut self, event: E) -> Result<(), String> {{\n"));
                code.push_str(&format!("        self.current_state = self.machine.transition(&self.current_state, event)?;\n"));
                code.push_str(&format!("        Ok(())\n"));
                code.push_str(&format!("    }}\n\n"));

                code.push_str(&format!("    /// Get current state\n"));
                code.push_str(&format!("    pub fn current_state(&self) -> &MachineStateImpl<C> {{\n"));
                code.push_str(&format!("        &self.current_state\n"));
                code.push_str(&format!("    }}\n\n"));

                code.push_str(&format!("    /// Check if machine can transition\n"));
                code.push_str(&format!("    pub fn can_transition(&self, event: &E) -> bool {{\n"));
                code.push_str(&format!("        self.machine.can_transition(&self.current_state, event)\n"));
                code.push_str(&format!("    }}\n"));
            }
            "typescript" => {
                code.push_str(&format!("    // Transition to a new state\n"));
                code.push_str(&format!("    public transition(event: E): void {{\n"));
                code.push_str(&format!("        this.currentState = this.machine.transition(this.currentState, event);\n"));
                code.push_str(&format!("    }}\n\n"));

                code.push_str(&format!("    // Get current state\n"));
                code.push_str(&format!("    public get currentState(): MachineStateImpl<C> {{\n"));
                code.push_str(&format!("        return this.currentState;\n"));
                code.push_str(&format!("    }}\n\n"));

                code.push_str(&format!("    // Check if machine can transition\n"));
                code.push_str(&format!("    public canTransition(event: E): boolean {{\n"));
                code.push_str(&format!("        return this.machine.canTransition(this.currentState, event);\n"));
                code.push_str(&format!("    }}\n"));
            }
            "python" => {
                code.push_str(&format!("    def transition(self, event):\n"));
                code.push_str(&format!("        \"\"\"Transition to a new state\"\"\"\n"));
                code.push_str(&format!("        self.current_state = self.machine.transition(self.current_state, event)\n\n"));

                code.push_str(&format!("    @property\n"));
                code.push_str(&format!("    def current_state(self):\n"));
                code.push_str(&format!("        \"\"\"Get current state\"\"\"\n"));
                code.push_str(&format!("        return self.current_state\n\n"));

                code.push_str(&format!("    def can_transition(self, event):\n"));
                code.push_str(&format!("        \"\"\"Check if machine can transition\"\"\"\n"));
                code.push_str(&format!("        return self.machine.can_transition(self.current_state, event)\n"));
            }
            _ => {
                code.push_str(&format!("    // Transition methods for {}\n", machine_name));
            }
        }

        Ok(code)
    }

    /// Generate events
    pub fn generate_events(&self, machine: &Machine<C, E, C>, machine_name: &str) -> Result<String, String> {
        let mut code = String::new();

        match self.config.language.as_str() {
            "rust" => {
                code.push_str("// Event definitions\n");
                code.push_str("#[derive(Debug, Clone, PartialEq, Eq, Hash)]\n");
                code.push_str(&format!("pub enum {}Event {{\n", machine_name));
                // This would generate actual event variants based on machine events
                code.push_str("    Start,\n");
                code.push_str("    Stop,\n");
                code.push_str("}\n");
            }
            "typescript" => {
                code.push_str("// Event definitions\n");
                code.push_str(&format!("export enum {}Event {{\n", machine_name));
                code.push_str("    Start = 'start',\n");
                code.push_str("    Stop = 'stop',\n");
                code.push_str("}\n");
            }
            "python" => {
                code.push_str("# Event definitions\n");
                code.push_str(&format!("class {}Event:\n", machine_name));
                code.push_str("    START = 'start'\n");
                code.push_str("    STOP = 'stop'\n");
            }
            _ => {
                code.push_str(&format!("// Event definitions for {}\n", machine_name));
            }
        }

        Ok(code)
    }
}
