//! Code generation templates for different languages

use std::collections::HashMap;

/// Code generation templates for different languages
#[derive(Debug, Clone)]
pub struct CodeTemplates {
    /// Templates by language and template type
    templates: HashMap<String, HashMap<String, String>>,
}

impl CodeTemplates {
    /// Create new templates
    pub fn new() -> Self {
        let mut templates = HashMap::new();

        // Initialize with built-in templates
        Self::add_builtin_templates(&mut templates);

        Self { templates }
    }

    /// Get template for language and type
    pub fn get_template(&self, language: &str, template_type: &str) -> Option<&String> {
        self.templates
            .get(language)?
            .get(template_type)
    }

    /// Set template for language and type
    pub fn set_template(&mut self, language: &str, template_type: &str, template: String) {
        self.templates
            .entry(language.to_string())
            .or_insert_with(HashMap::new)
            .insert(template_type.to_string(), template);
    }

    /// Check if template exists
    pub fn has_template(&self, language: &str, template_type: &str) -> bool {
        self.get_template(language, template_type).is_some()
    }

    /// Get all languages with templates
    pub fn languages(&self) -> Vec<&str> {
        self.templates.keys().map(|s| s.as_str()).collect()
    }

    /// Get all template types for a language
    pub fn template_types(&self, language: &str) -> Vec<&str> {
        self.templates
            .get(language)
            .map(|types| types.keys().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Remove template
    pub fn remove_template(&mut self, language: &str, template_type: &str) {
        if let Some(lang_templates) = self.templates.get_mut(language) {
            lang_templates.remove(template_type);
        }
    }

    /// Clear all templates for a language
    pub fn clear_language(&mut self, language: &str) {
        self.templates.remove(language);
    }

    /// Clear all templates
    pub fn clear_all(&mut self) {
        self.templates.clear();
    }

    /// Add built-in templates
    fn add_builtin_templates(templates: &mut HashMap<String, HashMap<String, String>>) {
        // Rust templates
        let mut rust_templates = HashMap::new();

        rust_templates.insert(
            "struct_header".to_string(),
            r#"/// Auto-generated state machine: {machine_name}
#[derive(Debug, Clone)]
pub struct {machine_name}<C, E> {{
    context: C,
    current_state: MachineStateImpl<C>,
    machine: Machine<C, E, C>,
}}"#.to_string(),
        );

        rust_templates.insert(
            "impl_header".to_string(),
            r#"impl<C, E> {machine_name}<C, E>
where
    C: Clone + Send + Sync + std::fmt::Debug + 'static,
    E: Clone + Send + Sync + std::fmt::Debug + PartialEq + 'static,
{{
    /// Create a new {machine_name}
    pub fn new(machine: Machine<C, E, C>) -> Self {{
        let initial_state = machine.initial_state();
        Self {{
            context: initial_state.context(),
            current_state: initial_state,
            machine,
        }}
    }}"#.to_string(),
        );

        rust_templates.insert(
            "transition_method".to_string(),
            r#"    /// Transition to a new state
    pub fn transition(&mut self, event: E) -> Result<(), String> {{
        let new_state = self.machine.transition(&self.current_state, event);
        self.current_state = new_state;
        Ok(())
    }}"#.to_string(),
        );

        templates.insert("rust".to_string(), rust_templates);

        // TypeScript templates
        let mut ts_templates = HashMap::new();

        ts_templates.insert(
            "class_header".to_string(),
            r#"// Auto-generated state machine: {machine_name}
export class {machine_name}<C, E> {{
    private context: C;
    private currentState: MachineStateImpl<C>;
    private machine: Machine<C, E, C>;

    constructor(machine: Machine<C, E, C>) {{
        const initialState = machine.initialState();
        this.context = initialState.context();
        this.currentState = initialState;
        this.machine = machine;
    }}"#.to_string(),
        );

        ts_templates.insert(
            "transition_method".to_string(),
            r#"    // Transition to a new state
    public transition(event: E): void {{
        const newState = this.machine.transition(this.currentState, event);
        this.currentState = newState;
    }}"#.to_string(),
        );

        templates.insert("typescript".to_string(), ts_templates);

        // Python templates
        let mut py_templates = HashMap::new();

        py_templates.insert(
            "class_header".to_string(),
            r#"# Auto-generated state machine: {machine_name}
class {machine_name}:
    def __init__(self, machine):
        self.context = None
        self.current_state = None
        self.machine = machine
        self._initialize()

    def _initialize(self):
        initial_state = self.machine.initial_state()
        self.context = initial_state.context()
        self.current_state = initial_state"#.to_string(),
        );

        py_templates.insert(
            "transition_method".to_string(),
            r#"    def transition(self, event):
        """Transition to a new state"""
        new_state = self.machine.transition(self.current_state, event)
        self.current_state = new_state"#.to_string(),
        );

        templates.insert("python".to_string(), py_templates);
    }

    /// Render template with variables
    pub fn render_template(&self, language: &str, template_type: &str, variables: &HashMap<&str, &str>) -> Option<String> {
        self.get_template(language, template_type).map(|template| {
            let mut rendered = template.clone();
            for (key, value) in variables {
                let placeholder = format!("{{{}}}", key);
                rendered = rendered.replace(&placeholder, value);
            }
            rendered
        })
    }

    /// Validate template syntax
    pub fn validate_template(&self, template: &str) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Check for unmatched braces
        let mut brace_count = 0;
        for (i, ch) in template.chars().enumerate() {
            match ch {
                '{' => brace_count += 1,
                '}' => {
                    brace_count -= 1;
                    if brace_count < 0 {
                        errors.push(format!("Unmatched closing brace at position {}", i));
                    }
                }
                _ => {}
            }
        }

        if brace_count > 0 {
            errors.push(format!("{} unmatched opening braces", brace_count));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Get template statistics
    pub fn statistics(&self) -> TemplateStatistics {
        let mut stats = TemplateStatistics::default();

        for (language, lang_templates) in &self.templates {
            stats.total_languages += 1;
            stats.total_templates += lang_templates.len();

            for template in lang_templates.values() {
                stats.total_characters += template.len();

                // Count placeholders
                let placeholders = template.matches('{').count();
                stats.total_placeholders += placeholders;
            }
        }

        stats
    }

    /// Export templates to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.templates)
    }

    /// Import templates from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let templates: HashMap<String, HashMap<String, String>> = serde_json::from_str(json)?;
        Ok(Self { templates })
    }

    /// Merge with another template collection
    pub fn merge(&mut self, other: &CodeTemplates) {
        for (language, lang_templates) in &other.templates {
            let self_lang_templates = self.templates.entry(language.clone()).or_insert_with(HashMap::new);

            for (template_type, template) in lang_templates {
                self_lang_templates.insert(template_type.clone(), template.clone());
            }
        }
    }

    /// Clone template from one language to another
    pub fn clone_templates(&mut self, from_language: &str, to_language: &str) {
        if let Some(from_templates) = self.templates.get(from_language) {
            let cloned_templates = from_templates.clone();
            self.templates.insert(to_language.to_string(), cloned_templates);
        }
    }
}

impl Default for CodeTemplates {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for CodeTemplates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stats = self.statistics();
        write!(f, "CodeTemplates({} languages, {} templates)", stats.total_languages, stats.total_templates)
    }
}

/// Template statistics
#[derive(Debug, Clone, Default)]
pub struct TemplateStatistics {
    /// Total number of languages
    pub total_languages: usize,
    /// Total number of templates
    pub total_templates: usize,
    /// Total characters across all templates
    pub total_characters: usize,
    /// Total placeholders across all templates
    pub total_placeholders: usize,
}

impl TemplateStatistics {
    /// Get average templates per language
    pub fn avg_templates_per_language(&self) -> f64 {
        if self.total_languages == 0 {
            0.0
        } else {
            self.total_templates as f64 / self.total_languages as f64
        }
    }

    /// Get average characters per template
    pub fn avg_characters_per_template(&self) -> f64 {
        if self.total_templates == 0 {
            0.0
        } else {
            self.total_characters as f64 / self.total_templates as f64
        }
    }

    /// Get average placeholders per template
    pub fn avg_placeholders_per_template(&self) -> f64 {
        if self.total_templates == 0 {
            0.0
        } else {
            self.total_placeholders as f64 / self.total_templates as f64
        }
    }
}

/// Template renderer for advanced templating
pub struct TemplateRenderer {
    templates: CodeTemplates,
}

impl TemplateRenderer {
    /// Create a new renderer
    pub fn new(templates: CodeTemplates) -> Self {
        Self { templates }
    }

    /// Render a complete file template
    pub fn render_file(
        &self,
        language: &str,
        machine_name: &str,
        config: &super::core::CodeGenConfig,
    ) -> Result<String, String> {
        let mut output = String::new();

        // Add file header
        if config.include_comments {
            let header = self.render_file_header(language, machine_name)?;
            output.push_str(&header);
            output.push('\n');
        }

        // Add struct/class definition
        let struct_def = self.render_struct(language, machine_name)?;
        output.push_str(&struct_def);
        output.push('\n');

        // Add implementation
        let impl_def = self.render_implementation(language, machine_name)?;
        output.push_str(&impl_def);

        Ok(output)
    }

    /// Render file header
    pub fn render_file_header(&self, language: &str, machine_name: &str) -> Result<String, String> {
        let variables = HashMap::from([
            ("machine_name", machine_name),
        ]);

        self.templates
            .render_template(language, "file_header", &variables)
            .or_else(|| {
                // Default header if no template
                Some(format!("// Auto-generated state machine: {}\n", machine_name))
            })
            .ok_or_else(|| format!("No file_header template for language: {}", language))
    }

    /// Render struct/class definition
    pub fn render_struct(&self, language: &str, machine_name: &str) -> Result<String, String> {
        let variables = HashMap::from([
            ("machine_name", machine_name),
        ]);

        self.templates
            .render_template(language, "struct_header", &variables)
            .or_else(|| self.templates.render_template(language, "class_header", &variables))
            .ok_or_else(|| format!("No struct/class template for language: {}", language))
    }

    /// Render implementation
    pub fn render_implementation(&self, language: &str, machine_name: &str) -> Result<String, String> {
        let mut output = String::new();

        // Add impl header
        let variables = HashMap::from([
            ("machine_name", machine_name),
        ]);

        if let Some(impl_header) = self.templates.render_template(language, "impl_header", &variables) {
            output.push_str(&impl_header);
            output.push('\n');
        }

        // Add methods
        if let Some(transition_method) = self.templates.render_template(language, "transition_method", &variables) {
            output.push_str(&transition_method);
            output.push('\n');
        }

        // Close implementation
        if let Some(impl_footer) = self.templates.render_template(language, "impl_footer", &variables) {
            output.push_str(&impl_footer);
        }

        Ok(output)
    }

    /// Validate all templates for a language
    pub fn validate_language_templates(&self, language: &str) -> Vec<String> {
        let mut errors = Vec::new();

        for template_type in self.templates.template_types(language) {
            if let Some(template) = self.templates.get_template(language, template_type) {
                if let Err(template_errors) = self.templates.validate_template(template) {
                    for error in template_errors {
                        errors.push(format!("{} template '{}': {}", language, template_type, error));
                    }
                }
            }
        }

        errors
    }
}

impl Default for TemplateRenderer {
    fn default() -> Self {
        Self::new(CodeTemplates::new())
    }
}
