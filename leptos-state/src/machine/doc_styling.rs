//! Documentation styling and templates

use super::*;

/// Template data for documentation generation
#[derive(Debug, Clone)]
pub struct TemplateData {
    /// Template name
    pub name: String,
    /// Template content
    pub content: String,
    /// Template variables
    pub variables: std::collections::HashMap<String, String>,
    /// Template metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl TemplateData {
    /// Create a new template data
    pub fn new(name: String, content: String) -> Self {
        Self {
            name,
            content,
            variables: std::collections::HashMap::new(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Set a template variable
    pub fn set_variable(&mut self, key: String, value: String) {
        self.variables.insert(key, value);
    }

    /// Get a template variable
    pub fn get_variable(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }

    /// Render the template with variables
    pub fn render(&self) -> String {
        let mut result = self.content.clone();

        for (key, value) in &self.variables {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }

        result
    }
}

/// Built-in documentation templates
pub struct BuiltInTemplates;

impl BuiltInTemplates {
    /// Get the default template
    pub fn default() -> TemplateData {
        let content = r#"# {{title}}

{{description}}

## Overview

This document provides comprehensive documentation for the state machine.

## States

{{states_table}}

## Transitions

{{transitions_table}}

{{#if include_diagrams}}
## State Diagram

```dot
{{diagram}}
```
{{/if}}

{{#if include_details}}
## Implementation Details

### Actions
{{actions_list}}

### Guards
{{guards_list}}
{{/if}}

{{#if include_performance}}
## Performance Characteristics

{{performance_metrics}}
{{/if}}
"#
        .to_string();

        let mut template = TemplateData::new("default".to_string(), content);
        template.set_variable(
            "title".to_string(),
            "{{machine_name}} State Machine".to_string(),
        );
        template.set_variable(
            "description".to_string(),
            "Generated documentation for the state machine.".to_string(),
        );
        template
    }

    /// Get the minimal template
    pub fn minimal() -> TemplateData {
        let content = r#"# {{title}}

## States
{{states_list}}

## Transitions
{{transitions_list}}
"#
        .to_string();

        let mut template = TemplateData::new("minimal".to_string(), content);
        template.set_variable("title".to_string(), "{{machine_name}}".to_string());
        template
    }

    /// Get the technical template
    pub fn technical() -> TemplateData {
        let content = r#"# {{title}} - Technical Documentation

## Machine Configuration

- **Type**: {{machine_type}}
- **States**: {{state_count}}
- **Transitions**: {{transition_count}}
- **Events**: {{event_count}}

## State Definitions

{{states_technical}}

## Transition Specifications

{{transitions_technical}}

## Action Implementations

{{actions_technical}}

## Guard Conditions

{{guards_technical}}

## Type Information

{{type_information}}
"#
        .to_string();

        let mut template = TemplateData::new("technical".to_string(), content);
        template.set_variable("title".to_string(), "{{machine_name}}".to_string());
        template
    }

    /// Get the user template
    pub fn user() -> TemplateData {
        let content = r#"# {{title}}

Welcome to the {{machine_name}} state machine documentation.

## What This Machine Does

{{description}}

## Available States

{{states_user_friendly}}

## Common Transitions

{{transitions_user_friendly}}

## Getting Help

{{help_information}}
"#
        .to_string();

        let mut template = TemplateData::new("user".to_string(), content);
        template.set_variable("title".to_string(), "{{machine_name}} Guide".to_string());
        template.set_variable(
            "description".to_string(),
            "This state machine manages {{purpose}}.".to_string(),
        );
        template
    }

    /// Get the API template
    pub fn api() -> TemplateData {
        let content = r#"# {{title}} API Reference

## Overview

{{description}}

## Types

{{api_types}}

## Methods

{{api_methods}}

## Events

{{api_events}}

## Examples

{{api_examples}}
"#
        .to_string();

        let mut template = TemplateData::new("api".to_string(), content);
        template.set_variable("title".to_string(), "{{machine_name}} API".to_string());
        template
    }
}

/// CSS styling for HTML documentation
pub struct HtmlStyling;

impl HtmlStyling {
    /// Get default CSS for HTML documentation
    pub fn default_css() -> String {
        r#"
body {
    font-family: Arial, sans-serif;
    line-height: 1.6;
    margin: 40px;
    color: #333;
}

h1, h2, h3, h4, h5, h6 {
    color: #2c3e50;
    margin-top: 24px;
    margin-bottom: 16px;
}

h1 { font-size: 2.5em; border-bottom: 2px solid #3498db; padding-bottom: 10px; }
h2 { font-size: 2em; border-bottom: 1px solid #bdc3c7; padding-bottom: 5px; }
h3 { font-size: 1.5em; }

code {
    background-color: #f8f8f8;
    padding: 2px 4px;
    border-radius: 3px;
    font-family: 'Consolas', 'Monaco', monospace;
}

pre {
    background-color: #f8f8f8;
    padding: 16px;
    border-radius: 6px;
    overflow-x: auto;
}

table {
    border-collapse: collapse;
    width: 100%;
    margin: 16px 0;
}

th, td {
    border: 1px solid #ddd;
    padding: 12px;
    text-align: left;
}

th {
    background-color: #f2f2f2;
    font-weight: bold;
}

tr:nth-child(even) {
    background-color: #f9f9f9;
}

.state-node {
    background-color: #e8f4f8;
    border: 2px solid #3498db;
    border-radius: 8px;
    padding: 12px;
    margin: 8px 0;
}

.transition-arrow {
    color: #27ae60;
    font-weight: bold;
}

.guard-condition {
    background-color: #fff3cd;
    border-left: 4px solid #f39c12;
    padding: 8px 12px;
    margin: 8px 0;
}

.action-executed {
    background-color: #d5f4e6;
    border-left: 4px solid #27ae60;
    padding: 8px 12px;
    margin: 8px 0;
}
"#
        .to_string()
    }

    /// Get dark theme CSS
    pub fn dark_theme_css() -> String {
        r#"
body {
    background-color: #2c3e50;
    color: #ecf0f1;
    font-family: Arial, sans-serif;
    line-height: 1.6;
    margin: 40px;
}

h1, h2, h3, h4, h5, h6 {
    color: #ecf0f1;
}

h1 { border-bottom-color: #3498db; }
h2 { border-bottom-color: #7f8c8d; }

code {
    background-color: #34495e;
    color: #ecf0f1;
}

pre {
    background-color: #34495e;
    color: #ecf0f1;
}

table {
    border-color: #7f8c8d;
}

th {
    background-color: #34495e;
    color: #ecf0f1;
}

tr:nth-child(even) {
    background-color: #34495e;
}

.state-node {
    background-color: #34495e;
    border-color: #3498db;
    color: #ecf0f1;
}

.guard-condition {
    background-color: #8b4513;
    border-left-color: #f39c12;
}

.action-executed {
    background-color: #0b6623;
    border-left-color: #27ae60;
}
"#
        .to_string()
    }
}

/// Markdown styling and formatting utilities
pub struct MarkdownStyling;

impl MarkdownStyling {
    /// Format a state as a markdown section
    pub fn format_state(state_name: &str, description: Option<&str>) -> String {
        let desc = description.unwrap_or("No description available");
        format!("### {}\n\n{}\n", state_name, desc)
    }

    /// Format a transition as markdown
    pub fn format_transition(
        from: &str,
        to: &str,
        event: &str,
        guards: &[String],
        actions: &[String],
    ) -> String {
        let mut result = format!("- **{}** → **{}** (on `{}`)\n", from, to, event);

        if !guards.is_empty() {
            result.push_str(&format!("  - Guards: {}\n", guards.join(", ")));
        }

        if !actions.is_empty() {
            result.push_str(&format!("  - Actions: {}\n", actions.join(", ")));
        }

        result
    }

    /// Format a table row
    pub fn format_table_row(columns: &[&str]) -> String {
        format!("| {} |\n", columns.join(" | "))
    }

    /// Format a table header
    pub fn format_table_header(columns: &[&str]) -> String {
        let mut result = Self::format_table_row(columns);
        result.push_str(&format!("|{}|\n", vec!["---"; columns.len()].join("|")));
        result
    }

    /// Create a code block
    pub fn code_block(language: &str, code: &str) -> String {
        format!("```{}\n{}\n```\n", language, code)
    }
}
