//! Test generation for different languages

use crate::machine::codegen::config::CodeGenConfig;
use crate::machine::{Machine, MachineStateImpl};

impl<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + std::hash::Hash + Eq + 'static>
    super::generator::CodeGenerator<C, E>
{
    /// Generate tests
    pub fn generate_tests(&self, machine: &Machine<C, E, C>, machine_name: &str) -> Result<String, String> {
        match self.config.language.as_str() {
            "rust" => self.generate_rust_tests(machine, machine_name),
            "typescript" => self.generate_typescript_tests(machine, machine_name),
            "javascript" => self.generate_javascript_tests(machine, machine_name),
            "python" => self.generate_python_tests(machine, machine_name),
            _ => Ok(format!("// Tests for {} in unsupported language\n", machine_name)),
        }
    }

    /// Generate Rust tests
    fn generate_rust_tests(&self, machine: &Machine<C, E, C>, machine_name: &str) -> Result<String, String> {
        let mut code = String::new();

        code.push_str("#[cfg(test)]\n");
        code.push_str("mod tests {\n");
        code.push_str("    use super::*;\n\n");

        code.push_str("    #[test]\n");
        code.push_str(&format!("    fn test_{}_creation() {{\n", machine_name.to_lowercase()));
        code.push_str(&format!("        let machine = Machine::new(/* ... */);\n"));
        code.push_str(&format!("        let generated_machine = {}::new(machine);\n", machine_name));
        code.push_str(&format!("        assert_eq!(generated_machine.current_state().value(), \"initial\");\n"));
        code.push_str("    }\n\n");

        code.push_str("    #[test]\n");
        code.push_str(&format!("    fn test_{}_transition() {{\n", machine_name.to_lowercase()));
        code.push_str(&format!("        let machine = Machine::new(/* ... */);\n"));
        code.push_str(&format!("        let mut generated_machine = {}::new(machine);\n", machine_name));
        code.push_str(&format!("        let event = {}Event::Start;\n", machine_name));
        code.push_str("        generated_machine.transition(event).unwrap();\n");
        code.push_str("        // Add assertions\n");
        code.push_str("    }\n\n");

        code.push_str("    #[test]\n");
        code.push_str(&format!("    fn test_{}_guards() {{\n", machine_name.to_lowercase()));
        code.push_str("        // Test guard functions\n");
        code.push_str(&format!("        assert!({}_can_transition(\"from\", \"to\", &context));\n", machine_name.to_lowercase()));
        code.push_str("    }\n");

        code.push_str("}\n");

        Ok(code)
    }

    /// Generate TypeScript tests
    fn generate_typescript_tests(&self, machine: &Machine<C, E, C>, machine_name: &str) -> Result<String, String> {
        let mut code = String::new();

        code.push_str("import { expect, test, describe } from '@jest/globals';\n\n");

        code.push_str(&format!("describe('{}', () => {{\n", machine_name));

        code.push_str("    test('creation', () => {\n");
        code.push_str("        const machine = new Machine(/* ... */);\n");
        code.push_str(&format!("        const generatedMachine = new {}(machine);\n", machine_name));
        code.push_str("        expect(generatedMachine.currentState.value()).toBe('initial');\n");
        code.push_str("    });\n\n");

        code.push_str("    test('transition', () => {\n");
        code.push_str("        const machine = new Machine(/* ... */);\n");
        code.push_str(&format!("        const generatedMachine = new {}(machine);\n", machine_name));
        code.push_str(&format!("        const event = {}Event.Start;\n", machine_name));
        code.push_str("        generatedMachine.transition(event);\n");
        code.push_str("        // Add expectations\n");
        code.push_str("    });\n\n");

        code.push_str("    test('guards', () => {\n");
        code.push_str("        // Test guard functions\n");
        code.push_str("        expect(canTransition('from', 'to', context)).toBe(true);\n");
        code.push_str("    });\n");

        code.push_str("});\n");

        Ok(code)
    }

    /// Generate JavaScript tests
    fn generate_javascript_tests(&self, machine: &Machine<C, E, C>, machine_name: &str) -> Result<String, String> {
        let mut code = String::new();

        code.push_str("const { expect } = require('chai');\n\n");

        code.push_str(&format!("describe('{}', () => {{\n", machine_name));

        code.push_str("    it('should create machine', () => {\n");
        code.push_str("        const machine = new Machine(/* ... */);\n");
        code.push_str(&format!("        const generatedMachine = new {}(machine);\n", machine_name));
        code.push_str("        expect(generatedMachine.currentState.value()).to.equal('initial');\n");
        code.push_str("    });\n\n");

        code.push_str("    it('should transition', () => {\n");
        code.push_str("        const machine = new Machine(/* ... */);\n");
        code.push_str(&format!("        const generatedMachine = new {}(machine);\n", machine_name));
        code.push_str(&format!("        const event = {}Event.Start;\n", machine_name));
        code.push_str("        generatedMachine.transition(event);\n");
        code.push_str("        // Add assertions\n");
        code.push_str("    });\n");

        code.push_str("});\n");

        Ok(code)
    }

    /// Generate Python tests
    fn generate_python_tests(&self, machine: &Machine<C, E, C>, machine_name: &str) -> Result<String, String> {
        let mut code = String::new();

        code.push_str("import unittest\n");
        code.push_str("from . import *\n\n");

        code.push_str(&format!("class Test{}(unittest.TestCase):\n", machine_name));

        code.push_str("    def test_creation(self):\n");
        code.push_str("        machine = Machine()  # Initialize machine\n");
        code.push_str(&format!("        generated_machine = {}Machine(machine)\n", machine_name));
        code.push_str("        self.assertEqual(generated_machine.current_state.value(), 'initial')\n\n");

        code.push_str("    def test_transition(self):\n");
        code.push_str("        machine = Machine()  # Initialize machine\n");
        code.push_str(&format!("        generated_machine = {}Machine(machine)\n", machine_name));
        code.push_str(&format!("        event = {}Event.START\n", machine_name));
        code.push_str("        generated_machine.transition(event)\n");
        code.push_str("        # Add assertions\n\n");

        code.push_str("    def test_guards(self):\n");
        code.push_str("        # Test guard functions\n");
        code.push_str("        self.assertTrue(can_transition('from', 'to', context))\n\n");

        code.push_str("if __name__ == '__main__':\n");
        code.push_str("    unittest.main()\n");

        Ok(code)
    }
}
