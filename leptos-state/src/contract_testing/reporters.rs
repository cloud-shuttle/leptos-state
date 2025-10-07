//! Contract test reporting utilities

use super::framework::*;
use std::io::Write;

/// Contract test reporter trait
pub trait ContractTestReporter {
    fn report(&self, results: &ContractTestResults) -> Result<(), std::io::Error>;
}

/// Console reporter for human-readable output
pub struct ConsoleReporter {
    verbose: bool,
}

impl ConsoleReporter {
    pub fn new() -> Self {
        Self { verbose: false }
    }

    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
}

impl ContractTestReporter for ConsoleReporter {
    fn report(&self, results: &ContractTestResults) -> Result<(), std::io::Error> {
        println!("🔍 Contract Test Report");
        println!("======================");
        println!("{}", results.summary());

        if results.has_violations() {
            println!("\n❌ Contract Violations:");
            println!("======================");

            for (i, violation) in results.contract_violations.iter().enumerate() {
                println!("{}. {}", i + 1, violation);
            }
        }

        if self.verbose {
            println!("\n📋 Detailed Results:");
            println!("===================");

            for result in &results.results {
                let status = if result.passed { "✅" } else { "❌" };
                println!("{} {}::{} ({:.2}ms)",
                    status,
                    result.component,
                    result.test_name,
                    result.execution_time.as_millis()
                );

                if !result.passed {
                    if let Some(ref error) = result.error_message {
                        println!("   Error: {}", error);
                    }
                }
            }
        }

        Ok(())
    }
}

/// JSON reporter for machine-readable output
pub struct JsonReporter {
    output_path: Option<std::path::PathBuf>,
}

impl JsonReporter {
    pub fn new() -> Self {
        Self { output_path: None }
    }

    pub fn with_output_path<P: Into<std::path::PathBuf>>(mut self, path: P) -> Self {
        self.output_path = Some(path.into());
        self
    }
}

impl ContractTestReporter for JsonReporter {
    fn report(&self, results: &ContractTestResults) -> Result<(), std::io::Error> {
        use serde_json;

        #[derive(serde::Serialize)]
        struct JsonReport {
            timestamp: String,
            summary: JsonSummary,
            violations: Vec<String>,
            results: Vec<JsonTestResult>,
        }

        #[derive(serde::Serialize)]
        struct JsonSummary {
            total_tests: usize,
            passed_tests: usize,
            failed_tests: usize,
            success_rate: f64,
            total_execution_time_ms: u128,
        }

        #[derive(serde::Serialize)]
        struct JsonTestResult {
            test_name: String,
            component: String,
            passed: bool,
            execution_time_ms: u128,
            error_message: Option<String>,
            contract_violations: Vec<String>,
        }

        let json_report = JsonReport {
            timestamp: chrono::Utc::now().to_rfc3339(),
            summary: JsonSummary {
                total_tests: results.total_tests,
                passed_tests: results.passed_tests,
                failed_tests: results.failed_tests,
                success_rate: results.success_rate(),
                total_execution_time_ms: results.total_execution_time.as_millis(),
            },
            violations: results.contract_violations.clone(),
            results: results.results.iter().map(|r| JsonTestResult {
                test_name: r.test_name.clone(),
                component: r.component.clone(),
                passed: r.passed,
                execution_time_ms: r.execution_time.as_millis(),
                error_message: r.error_message.clone(),
                contract_violations: r.contract_violations.clone(),
            }).collect(),
        };

        let json_string = serde_json::to_string_pretty(&json_report)?;

        match &self.output_path {
            Some(path) => {
                std::fs::write(path, &json_string)?;
                println!("JSON report written to: {}", path.display());
            }
            None => {
                println!("{}", json_string);
            }
        }

        Ok(())
    }
}

/// JUnit XML reporter for CI/CD integration
pub struct JunitReporter {
    output_path: std::path::PathBuf,
}

impl JunitReporter {
    pub fn new<P: Into<std::path::PathBuf>>(output_path: P) -> Self {
        Self {
            output_path: output_path.into(),
        }
    }
}

impl ContractTestReporter for JunitReporter {
    fn report(&self, results: &ContractTestResults) -> Result<(), std::io::Error> {
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<testsuites>\n");

        // Group results by component
        let mut component_groups: std::collections::HashMap<String, Vec<&ContractTestResult>> = std::collections::HashMap::new();

        for result in &results.results {
            component_groups.entry(result.component.clone())
                .or_insert_with(Vec::new)
                .push(result);
        }

        for (component, component_results) in component_groups {
            let passed = component_results.iter().filter(|r| r.passed).count();
            let failed = component_results.len() - passed;
            let total_time: u128 = component_results.iter().map(|r| r.execution_time.as_millis()).sum();

            xml.push_str(&format!(
                "  <testsuite name=\"{}\" tests=\"{}\" failures=\"{}\" time=\"{}\">\n",
                component,
                component_results.len(),
                failed,
                total_time as f64 / 1000.0
            ));

            for result in component_results {
                let status = if result.passed { "passed" } else { "failed" };
                xml.push_str(&format!(
                    "    <testcase name=\"{}\" status=\"{}\" time=\"{}\">\n",
                    result.test_name,
                    status,
                    result.execution_time.as_millis() as f64 / 1000.0
                ));

                if !result.passed {
                    xml.push_str("      <failure>\n");
                    if let Some(ref error) = result.error_message {
                        xml.push_str(&format!("        <![CDATA[{}]]>\n", escape_xml(error)));
                    }
                    for violation in &result.contract_violations {
                        xml.push_str(&format!("        <![CDATA[Contract Violation: {}]]>\n", escape_xml(violation)));
                    }
                    xml.push_str("      </failure>\n");
                }

                xml.push_str("    </testcase>\n");
            }

            xml.push_str("  </testsuite>\n");
        }

        xml.push_str("</testsuites>\n");

        std::fs::write(&self.output_path, &xml)?;
        println!("JUnit XML report written to: {}", self.output_path.display());

        Ok(())
    }
}

/// HTML reporter for web-based reports
pub struct HtmlReporter {
    output_path: std::path::PathBuf,
    title: String,
}

impl HtmlReporter {
    pub fn new<P: Into<std::path::PathBuf>>(output_path: P) -> Self {
        Self {
            output_path: output_path.into(),
            title: "Contract Test Report".to_string(),
        }
    }

    pub fn with_title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = title.into();
        self
    }
}

impl ContractTestReporter for HtmlReporter {
    fn report(&self, results: &ContractTestResults) -> Result<(), std::io::Error> {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n<head>\n");
        html.push_str(&format!("<title>{}</title>\n", self.title));
        html.push_str("<style>\n");
        html.push_str(include_str!("html_report.css"));
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");

        html.push_str(&format!("<h1>{}</h1>\n", self.title));
        html.push_str(&format!("<p><strong>Generated:</strong> {}</p>\n",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));

        // Summary section
        html.push_str("<div class=\"summary\">\n");
        html.push_str(&format!("<h2>Summary</h2>\n"));
        html.push_str(&format!("<p>Total Tests: <strong>{}</strong></p>\n", results.total_tests));
        html.push_str(&format!("<p>Passed: <strong class=\"passed\">{}</strong></p>\n", results.passed_tests));
        html.push_str(&format!("<p>Failed: <strong class=\"failed\">{}</strong></p>\n", results.failed_tests));
        html.push_str(&format!("<p>Success Rate: <strong>{:.1}%</strong></p>\n", results.success_rate()));
        html.push_str(&format!("<p>Total Time: <strong>{:.2}s</strong></p>\n", results.total_execution_time.as_secs_f64()));
        html.push_str("</div>\n");

        // Violations section
        if results.has_violations() {
            html.push_str("<div class=\"violations\">\n");
            html.push_str("<h2>Contract Violations</h2>\n");
            html.push_str("<ul>\n");
            for violation in &results.contract_violations {
                html.push_str(&format!("<li>{}</li>\n", escape_html(violation)));
            }
            html.push_str("</ul>\n");
            html.push_str("</div>\n");
        }

        // Detailed results
        html.push_str("<div class=\"results\">\n");
        html.push_str("<h2>Detailed Results</h2>\n");
        html.push_str("<table>\n");
        html.push_str("<thead>\n");
        html.push_str("<tr><th>Test</th><th>Component</th><th>Status</th><th>Time</th><th>Error</th></tr>\n");
        html.push_str("</thead>\n");
        html.push_str("<tbody>\n");

        for result in &results.results {
            let status_class = if result.passed { "passed" } else { "failed" };
            let status_text = if result.passed { "✅ PASS" } else { "❌ FAIL" };

            html.push_str("<tr>\n");
            html.push_str(&format!("<td>{}</td>\n", escape_html(&result.test_name)));
            html.push_str(&format!("<td>{}</td>\n", escape_html(&result.component)));
            html.push_str(&format!("<td class=\"{}\">{}</td>\n", status_class, status_text));
            html.push_str(&format!("<td>{:.2}ms</td>\n", result.execution_time.as_millis()));
            html.push_str("<td>");
            if let Some(ref error) = result.error_message {
                html.push_str(&escape_html(error));
            }
            html.push_str("</td>\n");
            html.push_str("</tr>\n");
        }

        html.push_str("</tbody>\n");
        html.push_str("</table>\n");
        html.push_str("</div>\n");

        html.push_str("</body>\n</html>\n");

        std::fs::write(&self.output_path, &html)?;
        println!("HTML report written to: {}", self.output_path.display());

        Ok(())
    }
}

/// Helper function to escape XML content
fn escape_xml(input: &str) -> String {
    input
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
}

/// Helper function to escape HTML content
fn escape_html(input: &str) -> String {
    input
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#x27;")
}

/// CSS styles for HTML reports (embedded)
const HTML_REPORT_CSS: &str = r#"
body {
    font-family: Arial, sans-serif;
    margin: 20px;
    background-color: #f5f5f5;
}

h1, h2 {
    color: #333;
}

.summary {
    background-color: white;
    padding: 20px;
    border-radius: 5px;
    margin-bottom: 20px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.violations {
    background-color: #ffe6e6;
    padding: 20px;
    border-radius: 5px;
    margin-bottom: 20px;
    border-left: 4px solid #ff4444;
}

.results {
    background-color: white;
    padding: 20px;
    border-radius: 5px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

table {
    width: 100%;
    border-collapse: collapse;
    margin-top: 10px;
}

th, td {
    padding: 8px 12px;
    text-align: left;
    border-bottom: 1px solid #ddd;
}

th {
    background-color: #f8f9fa;
    font-weight: bold;
}

.passed {
    color: #28a745;
    font-weight: bold;
}

.failed {
    color: #dc3545;
    font-weight: bold;
}

ul {
    margin: 0;
    padding-left: 20px;
}

li {
    margin: 5px 0;
}
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_console_reporter() {
        let mut results = ContractTestResults::new();
        results.add_result(ContractTestResult {
            test_name: "test_1".to_string(),
            component: "test_component".to_string(),
            passed: true,
            execution_time: std::time::Duration::from_millis(100),
            error_message: None,
            contract_violations: Vec::new(),
        });

        let reporter = ConsoleReporter::new();
        let result = reporter.report(&results);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_reporter() {
        let results = ContractTestResults::new();

        let temp_file = NamedTempFile::new().unwrap();
        let reporter = JsonReporter::new().with_output_path(temp_file.path());

        let result = reporter.report(&results);
        assert!(result.is_ok());

        // Check that file was created
        assert!(temp_file.path().exists());
    }

    #[test]
    fn test_junit_reporter() {
        let mut results = ContractTestResults::new();
        results.add_result(ContractTestResult {
            test_name: "test_1".to_string(),
            component: "test_component".to_string(),
            passed: false,
            execution_time: std::time::Duration::from_millis(100),
            error_message: Some("Test failure".to_string()),
            contract_violations: vec!["Violation".to_string()],
        });

        let temp_file = NamedTempFile::new().unwrap();
        let reporter = JunitReporter::new(temp_file.path());

        let result = reporter.report(&results);
        assert!(result.is_ok());

        // Check that file was created and contains expected content
        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        assert!(content.contains("testsuite"));
        assert!(content.contains("testcase"));
        assert!(content.contains("failure"));
    }

    #[test]
    fn test_xml_escaping() {
        assert_eq!(escape_xml("<>&\"'"), "&lt;&gt;&amp;&quot;&apos;");
    }

    #[test]
    fn test_html_escaping() {
        assert_eq!(escape_html("<>&\"'"), "&lt;&gt;&amp;&quot;&#x27;");
    }
}
