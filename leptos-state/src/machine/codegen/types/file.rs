//! Generated file management and utilities

use crate::machine::codegen::config::ProgrammingLanguage;

/// Generated file information
#[derive(Debug, Clone)]
pub struct GeneratedFile {
    /// File name
    pub file_name: String,
    /// Generated content
    pub content: String,
    /// Target programming language
    pub language: ProgrammingLanguage,
    /// Time taken to generate
    pub generation_time: std::time::Duration,
    /// Number of lines in generated file
    pub line_count: usize,
}

impl GeneratedFile {
    /// Create a new generated file
    pub fn new(file_name: String, content: String, language: ProgrammingLanguage) -> Self {
        Self {
            file_name,
            line_count: content.lines().count(),
            content,
            language,
            generation_time: std::time::Duration::from_nanos(0),
        }
    }

    /// Save the file to disk
    pub fn save_to_file(&self, base_path: &std::path::Path) -> Result<std::path::PathBuf, String> {
        let full_path = base_path.join(&self.file_name);

        // Create directory if it doesn't exist
        if let Some(parent) = full_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        std::fs::write(&full_path, &self.content)
            .map_err(|e| format!("Failed to write file: {}", e))?;

        Ok(full_path)
    }

    /// Get file extension
    pub fn extension(&self) -> &str {
        self.language.extension()
    }

    /// Get file size in bytes
    pub fn size(&self) -> usize {
        self.content.len()
    }

    /// Check if file is empty
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Get file path for a given base directory
    pub fn path(&self, base_path: &std::path::Path) -> std::path::PathBuf {
        base_path.join(&self.file_name)
    }

    /// Set generation time
    pub fn with_generation_time(mut self, duration: std::time::Duration) -> Self {
        self.generation_time = duration;
        self
    }

    /// Update line count (useful if content was modified)
    pub fn update_line_count(&mut self) {
        self.line_count = self.content.lines().count();
    }

    /// Append content to the file
    pub fn append(&mut self, content: &str) {
        self.content.push_str(content);
        self.update_line_count();
    }

    /// Prepend content to the file
    pub fn prepend(&mut self, content: &str) {
        self.content = format!("{}{}", content, self.content);
        self.update_line_count();
    }

    /// Replace content in the file
    pub fn replace(&mut self, content: String) {
        self.content = content;
        self.update_line_count();
    }

    /// Check if the file contains a specific string
    pub fn contains(&self, pattern: &str) -> bool {
        self.content.contains(pattern)
    }

    /// Get lines from the file
    pub fn lines(&self) -> std::str::Lines {
        self.content.lines()
    }

    /// Get a specific line (0-indexed)
    pub fn line(&self, index: usize) -> Option<&str> {
        self.lines().nth(index)
    }
}

impl std::fmt::Display for GeneratedFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GeneratedFile {{ name: {}, language: {}, lines: {}, size: {} bytes, time: {:?} }}",
            self.file_name,
            self.language.as_str(),
            self.line_count,
            self.size(),
            self.generation_time
        )
    }
}
