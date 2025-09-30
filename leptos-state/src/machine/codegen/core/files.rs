//! Multi-file generation and output handling

use crate::machine::codegen::config::CodeGenConfig;
use crate::machine::codegen::types::GeneratedFile;
use crate::machine::{Machine, MachineStateImpl};

impl<C: Send + Sync + Clone + PartialEq + 'static, E: Clone + Send + Sync + std::hash::Hash + Eq + 'static>
    super::generator::CodeGenerator<C, E>
{
    /// Generate multiple files if configured
    pub fn generate_separate_files(
        &mut self,
        machine: &Machine<C, E, C>,
    ) -> Result<Vec<GeneratedFile>, String> {
        let mut files = Vec::new();

        // Generate main machine file
        let main_file = self.generate(machine)?;
        files.push(main_file);

        // Generate separate test file if configured
        if self.config.generate_tests && self.config.separate_test_file {
            if let Some(test_file) = self.generate_separate_test_file(machine)? {
                files.push(test_file);
            }
        }

        Ok(files)
    }

    /// Generate separate test file
    pub fn generate_separate_test_file(
        &self,
        machine: &Machine<C, E, C>,
    ) -> Result<Option<GeneratedFile>, String> {
        if !self.config.generate_tests {
            return Ok(None);
        }

        let machine_name = "GeneratedMachine".to_string();
        let test_code = self.generate_tests(machine, &machine_name)?;

        let filename = match self.config.language.as_str() {
            "rust" => format!("{}_test.rs", machine_name.to_lowercase()),
            "typescript" | "javascript" => format!("{}.test.{}", machine_name.to_lowercase(), self.config.language.file_extension()),
            "python" => format!("test_{}.py", machine_name.to_lowercase()),
            _ => format!("{}_test.{}", machine_name.to_lowercase(), self.config.language.file_extension()),
        };

        let test_file = GeneratedFile {
            filename,
            content: test_code,
            language: self.config.language.clone(),
            machine_name: format!("{}_tests", machine_name),
            generation_time: std::time::Duration::from_secs(0), // Simplified
            line_count: test_code.lines().count(),
        };

        Ok(Some(test_file))
    }

    /// Write generated files to disk
    pub fn write_files(&self, files: &[GeneratedFile], output_dir: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output_dir)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;

        for file in files {
            let file_path = output_dir.join(&file.filename);
            std::fs::write(&file_path, &file.content)
                .map_err(|e| format!("Failed to write file {}: {}", file_path.display(), e))?;
        }

        Ok(())
    }

    /// Generate and write files
    pub fn generate_and_write(&mut self, machine: &Machine<C, E, C>, output_dir: &std::path::Path) -> Result<Vec<GeneratedFile>, String> {
        let files = if self.config.separate_files {
            self.generate_separate_files(machine)?
        } else {
            vec![self.generate(machine)?]
        };

        self.write_files(&files, output_dir)?;
        Ok(files)
    }

    /// Validate output directory
    pub fn validate_output_dir(&self, output_dir: &std::path::Path) -> Result<(), String> {
        if !output_dir.exists() {
            return Err(format!("Output directory does not exist: {}", output_dir.display()));
        }

        if !output_dir.is_dir() {
            return Err(format!("Output path is not a directory: {}", output_dir.display()));
        }

        // Check if directory is writable
        let test_file = output_dir.join(".test_write");
        match std::fs::File::create(&test_file) {
            Ok(_) => {
                let _ = std::fs::remove_file(test_file);
                Ok(())
            }
            Err(e) => Err(format!("Output directory is not writable: {}", e)),
        }
    }

    /// Get file extension for current language
    pub fn file_extension(&self) -> &'static str {
        self.config.language.file_extension()
    }

    /// Format filename according to configuration
    pub fn format_filename(&self, machine_name: &str, suffix: Option<&str>) -> String {
        let base_name = machine_name.to_lowercase();
        let extension = self.file_extension();

        match suffix {
            Some(s) => format!("{}_{}.{}", base_name, s, extension),
            None => format!("{}.{}", base_name, extension),
        }
    }

    /// Create output directory if it doesn't exist
    pub fn ensure_output_dir(&self, output_dir: &std::path::Path) -> Result<(), String> {
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir)
                .map_err(|e| format!("Failed to create output directory: {}", e))?;
        }
        Ok(())
    }

    /// Get relative path for generated file
    pub fn get_relative_path(&self, filename: &str) -> std::path::PathBuf {
        if let Some(ref output_dir) = self.config.output_dir {
            std::path::PathBuf::from(output_dir).join(filename)
        } else {
            std::path::PathBuf::from(filename)
        }
    }

    /// List generated files
    pub fn list_generated_files(&self) -> Vec<String> {
        self.generated_code.keys().cloned().collect()
    }

    /// Get file info for generated files
    pub fn get_file_info(&self) -> Vec<FileInfo> {
        self.generated_code.iter()
            .map(|(name, content)| FileInfo {
                name: name.clone(),
                size_bytes: content.len(),
                line_count: content.lines().count(),
                language: self.config.language.clone(),
            })
            .collect()
    }
}

/// File information
#[derive(Debug, Clone)]
pub struct FileInfo {
    /// File name
    pub name: String,
    /// File size in bytes
    pub size_bytes: usize,
    /// Number of lines
    pub line_count: usize,
    /// Programming language
    pub language: crate::machine::codegen::config::ProgrammingLanguage,
}

impl std::fmt::Display for FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({} lines, {} bytes, {})",
            self.name,
            self.line_count,
            self.size_bytes,
            self.language.as_str()
        )
    }
}
