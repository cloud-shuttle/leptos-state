//! Guards and actions generation

use crate::machine::codegen::config::CodeGenConfig;
use crate::machine::{Machine, MachineStateImpl};

impl<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + std::hash::Hash + Eq + 'static>
    super::generator::CodeGenerator<C, E>
{
    /// Generate guards
    pub fn generate_guards(&self, machine: &Machine<C, E, C>, machine_name: &str) -> Result<String, String> {
        let mut code = String::new();

        match self.config.language.as_str() {
            "rust" => {
                code.push_str("// Guard functions\n");
                code.push_str("/// Check if transition is allowed\n");
                code.push_str(&format!("pub fn {}_can_transition(from: &str, to: &str, context: &C) -> bool {{\n", machine_name.to_lowercase()));
                code.push_str("    // Guard logic here\n");
                code.push_str("    true\n");
                code.push_str("}\n\n");

                code.push_str("/// Validate state transition\n");
                code.push_str(&format!("pub fn {}_validate_transition(event: &E, context: &C) -> Result<(), String> {{\n", machine_name.to_lowercase()));
                code.push_str("    // Validation logic here\n");
                code.push_str("    Ok(())\n");
                code.push_str("}\n");
            }
            "typescript" => {
                code.push_str("// Guard functions\n");
                code.push_str("export function canTransition(from: string, to: string, context: C): boolean {\n");
                code.push_str("    // Guard logic here\n");
                code.push_str("    return true;\n");
                code.push_str("}\n\n");

                code.push_str("export function validateTransition(event: E, context: C): void {\n");
                code.push_str("    // Validation logic here\n");
                code.push_str("}\n");
            }
            "python" => {
                code.push_str("# Guard functions\n");
                code.push_str("def can_transition(from_state, to_state, context):\n");
                code.push_str("    \"\"\"Check if transition is allowed\"\"\"\n");
                code.push_str("    # Guard logic here\n");
                code.push_str("    return True\n\n");

                code.push_str("def validate_transition(event, context):\n");
                code.push_str("    \"\"\"Validate state transition\"\"\"\n");
                code.push_str("    # Validation logic here\n");
                code.push_str("    pass\n");
            }
            _ => {
                code.push_str(&format!("// Guard functions for {}\n", machine_name));
            }
        }

        Ok(code)
    }

    /// Generate actions
    pub fn generate_actions(&self, machine: &Machine<C, E, C>, machine_name: &str) -> Result<String, String> {
        let mut code = String::new();

        match self.config.language.as_str() {
            "rust" => {
                code.push_str("// Action functions\n");
                code.push_str("/// Execute entry action for a state\n");
                code.push_str(&format!("pub fn {}_on_enter_state(state: &str, context: &mut C) {{\n", machine_name.to_lowercase()));
                code.push_str("    // Entry action logic here\n");
                code.push_str("}\n\n");

                code.push_str("/// Execute exit action for a state\n");
                code.push_str(&format!("pub fn {}_on_exit_state(state: &str, context: &mut C) {{\n", machine_name.to_lowercase()));
                code.push_str("    // Exit action logic here\n");
                code.push_str("}\n\n");

                code.push_str("/// Execute transition action\n");
                code.push_str(&format!("pub fn {}_on_transition(from: &str, to: &str, event: &E, context: &mut C) {{\n", machine_name.to_lowercase()));
                code.push_str("    // Transition action logic here\n");
                code.push_str("}\n");
            }
            "typescript" => {
                code.push_str("// Action functions\n");
                code.push_str("export function onEnterState(state: string, context: C): void {\n");
                code.push_str("    // Entry action logic here\n");
                code.push_str("}\n\n");

                code.push_str("export function onExitState(state: string, context: C): void {\n");
                code.push_str("    // Exit action logic here\n");
                code.push_str("}\n\n");

                code.push_str("export function onTransition(from: string, to: string, event: E, context: C): void {\n");
                code.push_str("    // Transition action logic here\n");
                code.push_str("}\n");
            }
            "python" => {
                code.push_str("# Action functions\n");
                code.push_str("def on_enter_state(state, context):\n");
                code.push_str("    \"\"\"Execute entry action for a state\"\"\"\n");
                code.push_str("    # Entry action logic here\n");
                code.push_str("    pass\n\n");

                code.push_str("def on_exit_state(state, context):\n");
                code.push_str("    \"\"\"Execute exit action for a state\"\"\"\n");
                code.push_str("    # Exit action logic here\n");
                code.push_str("    pass\n\n");

                code.push_str("def on_transition(from_state, to_state, event, context):\n");
                code.push_str("    \"\"\"Execute transition action\"\"\"\n");
                code.push_str("    # Transition action logic here\n");
                code.push_str("    pass\n");
            }
            _ => {
                code.push_str(&format!("// Action functions for {}\n", machine_name));
            }
        }

        Ok(code)
    }
}
