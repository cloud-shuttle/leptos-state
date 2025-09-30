//! Code structure generation (imports, structs, constants)

use crate::machine::codegen::config::CodeGenConfig;
use crate::machine::{Machine, MachineStateImpl};

impl<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + std::hash::Hash + Eq + 'static>
    super::generator::CodeGenerator<C, E>
{
    /// Generate imports for the target language
    pub fn generate_imports(&self, machine_name: &str) -> Result<String, String> {
        match self.config.language.as_str() {
            "rust" => self.generate_rust_imports(machine_name),
            "typescript" => self.generate_typescript_imports(machine_name),
            "javascript" => self.generate_javascript_imports(machine_name),
            "python" => self.generate_python_imports(machine_name),
            _ => Ok("// Imports for unknown language\n".to_string()),
        }
    }

    /// Generate machine structure
    pub fn generate_machine_structure(&self, machine: &Machine<C, E, C>, machine_name: &str) -> Result<String, String> {
        let mut code = String::new();

        match self.config.language.as_str() {
            "rust" => {
                code.push_str(&format!("/// Auto-generated state machine: {}\n", machine_name));
                code.push_str("#[derive(Debug, Clone)]\n");
                code.push_str(&format!("pub struct {} {{\n", machine_name));
                code.push_str("    /// Current state\n");
                code.push_str("    current_state: MachineStateImpl<C>,\n");
                code.push_str("    /// Machine configuration\n");
                code.push_str("    machine: Machine<C, E, C>,\n");
                code.push_str("}\n\n");

                code.push_str(&format!("impl<C, E> {}<C, E>\n", machine_name));
                code.push_str("where\n");
                code.push_str("    C: Clone + Send + Sync + std::fmt::Debug + 'static,\n");
                code.push_str("    E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static,\n");
                code.push_str("{\n");
                code.push_str(&format!("    /// Create a new {}\n", machine_name));
                code.push_str("    pub fn new(machine: Machine<C, E, C>) -> Self {\n");
                code.push_str("        let initial_state = machine.initial_state();\n");
                code.push_str("        Self {\n");
                code.push_str("            current_state: initial_state,\n");
                code.push_str("            machine,\n");
                code.push_str("        }\n");
                code.push_str("    }\n");
                code.push_str("}\n");
            }
            "typescript" => {
                code.push_str(&format!("// Auto-generated state machine: {}\n", machine_name));
                code.push_str(&format!("export class {} {{\n", machine_name));
                code.push_str("    private currentState: MachineStateImpl<C>;\n");
                code.push_str("    private machine: Machine<C, E, C>;\n\n");
                code.push_str("    constructor(machine: Machine<C, E, C>) {\n");
                code.push_str("        this.currentState = machine.initialState();\n");
                code.push_str("        this.machine = machine;\n");
                code.push_str("    }\n");
                code.push_str("}\n");
            }
            _ => {
                code.push_str(&format!("// Machine structure for {} in {}\n", machine_name, self.config.language.as_str()));
            }
        }

        Ok(code)
    }

    /// Generate state constants
    pub fn generate_state_constants(&self, machine: &Machine<C, E, C>, machine_name: &str) -> Result<String, String> {
        let mut code = String::new();

        match self.config.language.as_str() {
            "rust" => {
                code.push_str("// State constants\n");
                // This would generate actual state constants based on machine states
                code.push_str(&format!("pub const {}_INITIAL_STATE: &str = \"initial\";\n", machine_name.to_uppercase()));
            }
            "typescript" => {
                code.push_str("// State constants\n");
                code.push_str(&format!("export const {}_INITIAL_STATE = \"initial\";\n", machine_name.to_uppercase()));
            }
            "python" => {
                code.push_str("# State constants\n");
                code.push_str(&format!("{}_INITIAL_STATE = \"initial\"\n", machine_name.to_uppercase()));
            }
            _ => {
                code.push_str(&format!("// State constants for {}\n", machine_name));
            }
        }

        Ok(code)
    }

    /// Generate Rust imports
    fn generate_rust_imports(&self, _machine_name: &str) -> Result<String, String> {
        Ok("use leptos_state::{Machine, MachineStateImpl};\n".to_string())
    }

    /// Generate TypeScript imports
    fn generate_typescript_imports(&self, _machine_name: &str) -> Result<String, String> {
        Ok("import { Machine, MachineStateImpl } from 'leptos-state';\n".to_string())
    }

    /// Generate JavaScript imports
    fn generate_javascript_imports(&self, _machine_name: &str) -> Result<String, String> {
        Ok("const { Machine, MachineStateImpl } = require('leptos-state');\n".to_string())
    }

    /// Generate Python imports
    fn generate_python_imports(&self, _machine_name: &str) -> Result<String, String> {
        Ok("from leptos_state import Machine, MachineStateImpl\n".to_string())
    }
}
