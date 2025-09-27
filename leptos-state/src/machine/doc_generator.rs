//! Documentation generator implementation

use super::*;
use std::collections::HashMap;

/// Documentation generator for state machines
pub struct DocumentationGenerator<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> {
    /// Machine being documented
    pub machine: Machine<C, E, C>,
    /// Configuration
    pub config: DocumentationConfig,
    /// Documentation data
    pub data: DocumentationData,
}

impl<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + Hash + Eq + 'static> DocumentationGenerator<C, E> {
    /// Create a new documentation generator
    pub fn new(machine: Machine<C, E, C>, config: DocumentationConfig) -> Self {
        Self {
            machine: machine.clone(),
            config,
            data: DocumentationData::new(machine),
        }
    }

    /// Generate documentation in the configured format
    pub fn generate(&self) -> StateResult<GeneratedDocument> {
        match self.config.format {
            DocumentationFormat::Markdown => self.generate_markdown(),
            DocumentationFormat::Html => self.generate_html(),
            DocumentationFormat::Json => self.generate_json(),
            DocumentationFormat::Text => self.generate_text(),
            DocumentationFormat::Dot => self.generate_dot(),
            DocumentationFormat::PlantUml => self.generate_plantuml(),
        }
    }

    /// Generate markdown documentation
    pub fn generate_markdown(&self) -> StateResult<GeneratedDocument> {
        let mut content = String::new();

        // Header
        content.push_str(&format!("# {} State Machine Documentation\n\n", self.data.machine_name));
        content.push_str(&format!("**Generated:** {}\n\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));

        // Overview
        content.push_str("## Overview\n\n");
        content.push_str(&format!("This documentation describes a state machine with {} states and {} transitions.\n\n",
            self.data.states.len(), self.data.transitions.len()));

        // States section
        content.push_str("## States\n\n");
        content.push_str("| State | Description |\n");
        content.push_str("|-------|-------------|\n");
        for state in &self.data.states {
            content.push_str(&format!("| {} | {} |\n", state.name, state.description.as_deref().unwrap_or("")));
        }
        content.push_str("\n");

        // Transitions section
        content.push_str("## Transitions\n\n");
        content.push_str("| From | To | Event | Guards | Actions |\n");
        content.push_str("|------|----|-------|--------|---------|\n");
        for transition in &self.data.transitions {
            content.push_str(&format!("| {} | {} | {} | {} | {} |\n",
                transition.from_state,
                transition.to_state,
                transition.event,
                transition.guards.join(", "),
                transition.actions.join(", ")
            ));
        }
        content.push_str("\n");

        // Actions section
        if self.config.include_details && !self.data.actions.is_empty() {
            content.push_str("## Actions\n\n");
            for action in &self.data.actions {
                content.push_str(&format!("### {}\n\n{}\n\n", action.name, action.description));
            }
        }

        // Guards section
        if self.config.include_details && !self.data.guards.is_empty() {
            content.push_str("## Guards\n\n");
            for guard in &self.data.guards {
                content.push_str(&format!("### {}\n\n{}\n\n", guard.name, guard.description));
            }
        }

        // Diagram section
        if self.config.include_diagrams {
            content.push_str("## State Diagram\n\n");
            content.push_str("```dot\n");
            content.push_str(&self.generate_dot_content());
            content.push_str("```\n\n");
        }

        Ok(GeneratedDocument {
            format: DocumentationFormat::Markdown,
            content,
            filename: format!("{}.md", self.config.file_prefix),
            metadata: HashMap::new(),
        })
    }

    /// Generate HTML documentation
    pub fn generate_html(&self) -> StateResult<GeneratedDocument> {
        let mut content = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} State Machine Documentation</title>
    <style>
{}
    </style>
</head>
<body>
    <h1>{} State Machine Documentation</h1>
    <p><strong>Generated:</strong> {}</p>
"#,
            self.data.machine_name,
            if self.config.styling.dark_theme { HtmlStyling::dark_theme_css() } else { HtmlStyling::default_css() },
            self.data.machine_name,
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        );

        // States section
        content.push_str("<h2>States</h2>\n<table>\n<thead>\n<tr><th>State</th><th>Description</th></tr>\n</thead>\n<tbody>\n");
        for state in &self.data.states {
            content.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>\n",
                state.name,
                state.description.as_deref().unwrap_or("")
            ));
        }
        content.push_str("</tbody>\n</table>\n");

        // Transitions section
        content.push_str("<h2>Transitions</h2>\n<table>\n<thead>\n<tr><th>From</th><th>To</th><th>Event</th><th>Guards</th><th>Actions</th></tr>\n</thead>\n<tbody>\n");
        for transition in &self.data.transitions {
            content.push_str(&format!("<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
                transition.from_state,
                transition.to_state,
                transition.event,
                transition.guards.join(", "),
                transition.actions.join(", ")
            ));
        }
        content.push_str("</tbody>\n</table>\n");

        content.push_str("</body>\n</html>\n");

        Ok(GeneratedDocument {
            format: DocumentationFormat::Html,
            content,
            filename: format!("{}.html", self.config.file_prefix),
            metadata: HashMap::new(),
        })
    }

    /// Generate JSON documentation
    pub fn generate_json(&self) -> StateResult<GeneratedDocument> {
        let json_data = serde_json::to_string_pretty(&self.data)
            .map_err(|e| StateError::DocumentationError(format!("JSON serialization failed: {}", e)))?;

        Ok(GeneratedDocument {
            format: DocumentationFormat::Json,
            content: json_data,
            filename: format!("{}.json", self.config.file_prefix),
            metadata: HashMap::new(),
        })
    }

    /// Generate plain text documentation
    pub fn generate_text(&self) -> StateResult<GeneratedDocument> {
        let mut content = format!("{} State Machine Documentation\n", self.data.machine_name);
        content.push_str(&format!("Generated: {}\n\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));

        content.push_str("STATES:\n");
        for state in &self.data.states {
            content.push_str(&format!("  {}: {}\n", state.name, state.description.as_deref().unwrap_or("")));
        }

        content.push_str("\nTRANSITIONS:\n");
        for transition in &self.data.transitions {
            content.push_str(&format!("  {} -> {} on {}\n", transition.from_state, transition.to_state, transition.event));
            if !transition.guards.is_empty() {
                content.push_str(&format!("    Guards: {}\n", transition.guards.join(", ")));
            }
            if !transition.actions.is_empty() {
                content.push_str(&format!("    Actions: {}\n", transition.actions.join(", ")));
            }
        }

        Ok(GeneratedDocument {
            format: DocumentationFormat::Text,
            content,
            filename: format!("{}.txt", self.config.file_prefix),
            metadata: HashMap::new(),
        })
    }

    /// Generate DOT format for graphviz
    pub fn generate_dot(&self) -> StateResult<GeneratedDocument> {
        let content = format!("digraph \"{}\" {{\n{}\n}}\n", self.data.machine_name, self.generate_dot_content());

        Ok(GeneratedDocument {
            format: DocumentationFormat::Dot,
            content,
            filename: format!("{}.dot", self.config.file_prefix),
            metadata: HashMap::new(),
        })
    }

    /// Generate PlantUML format
    pub fn generate_plantuml(&self) -> StateResult<GeneratedDocument> {
        let mut content = format!("@startuml {}\n", self.data.machine_name);

        // Initial state
        content.push_str(&format!("[*] --> {}\n", self.machine.initial));

        // States
        for state in &self.data.states {
            content.push_str(&format!("state \"{}\" as {}\n", state.name, state.name));
        }

        // Transitions
        for transition in &self.data.transitions {
            content.push_str(&format!("{} --> {} : {}\n",
                transition.from_state,
                transition.to_state,
                transition.event
            ));
        }

        content.push_str("@enduml\n");

        Ok(GeneratedDocument {
            format: DocumentationFormat::PlantUml,
            content,
            filename: format!("{}.puml", self.config.file_prefix),
            metadata: HashMap::new(),
        })
    }

    /// Generate DOT content for diagrams
    fn generate_dot_content(&self) -> String {
        let mut content = String::new();

        // Graph attributes
        content.push_str("  rankdir=LR;\n");
        content.push_str("  node [shape=box];\n");

        // Initial state
        content.push_str(&format!("  \"\" [shape=point];\n"));
        content.push_str(&format!("  \"\" -> {};\n", self.machine.initial));

        // States
        for state in &self.data.states {
            content.push_str(&format!("  {} [label=\"{}\"];\n", state.name, state.name));
        }

        // Transitions
        for transition in &self.data.transitions {
            content.push_str(&format!("  {} -> {} [label=\"{}\"];\n",
                transition.from_state,
                transition.to_state,
                transition.event
            ));
        }

        content
    }

    /// Render a template with data
    pub fn render_template(&self, template: &TemplateData) -> StateResult<String> {
        let mut rendered = template.content.clone();

        // Replace template variables
        for (key, value) in &template.variables {
            rendered = rendered.replace(&format!("{{{{{}}}}}", key), value);
        }

        // Replace data placeholders
        rendered = rendered.replace("{{machine_name}}", &self.data.machine_name);
        rendered = rendered.replace("{{states_table}}", &self.generate_states_table());
        rendered = rendered.replace("{{transitions_table}}", &self.generate_transitions_table());
        rendered = rendered.replace("{{diagram}}", &self.generate_dot_content());

        Ok(rendered)
    }

    /// Generate states table for templates
    fn generate_states_table(&self) -> String {
        let mut table = "| State | Description |\n|-------|-------------|\n".to_string();
        for state in &self.data.states {
            table.push_str(&format!("| {} | {} |\n",
                state.name,
                state.description.as_deref().unwrap_or("")
            ));
        }
        table
    }

    /// Generate transitions table for templates
    fn generate_transitions_table(&self) -> String {
        let mut table = "| From | To | Event | Guards | Actions |\n|------|----|-------|--------|---------|\n".to_string();
        for transition in &self.data.transitions {
            table.push_str(&format!("| {} | {} | {} | {} | {} |\n",
                transition.from_state,
                transition.to_state,
                transition.event,
                transition.guards.join(", "),
                transition.actions.join(", ")
            ));
        }
        table
    }
}
