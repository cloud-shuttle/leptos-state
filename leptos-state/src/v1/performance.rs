use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Type alias for benchmark operations
type BenchmarkOperation = Box<dyn Fn() -> usize>;

/// Performance benchmarking and optimization utilities
#[derive(Default)]
pub struct PerformanceBenchmark {
    /// Benchmark results
    results: HashMap<String, BenchmarkResult>,
    /// Performance thresholds
    thresholds: PerformanceThresholds,
    /// Optimization suggestions
    suggestions: Vec<OptimizationSuggestion>,
}

/// Individual benchmark result
#[derive(Clone, Debug)]
pub struct BenchmarkResult {
    /// Operation name
    pub operation: String,
    /// Average execution time
    pub avg_time: Duration,
    /// Minimum execution time
    pub min_time: Duration,
    /// Maximum execution time
    pub max_time: Duration,
    /// Number of iterations
    pub iterations: usize,
    /// Memory usage (estimated)
    pub memory_usage: usize,
    /// Performance score (0-100)
    pub performance_score: f64,
}

/// Performance thresholds for optimization
#[derive(Clone, Debug)]
pub struct PerformanceThresholds {
    /// Maximum acceptable transition time
    pub max_transition_time: Duration,
    /// Maximum acceptable memory usage
    pub max_memory_usage: usize,
    /// Minimum acceptable performance score
    pub min_performance_score: f64,
}

/// Optimization suggestion
#[derive(Clone, Debug)]
pub struct OptimizationSuggestion {
    /// Suggestion type
    pub suggestion_type: OptimizationType,
    /// Description
    pub description: String,
    /// Expected improvement
    pub expected_improvement: f64,
    /// Priority (1-5, 5 being highest)
    pub priority: u8,
}

/// Types of optimizations
#[derive(Clone, Debug, PartialEq)]
pub enum OptimizationType {
    /// Memory optimization
    Memory,
    /// Algorithm optimization
    Algorithm,
    /// Data structure optimization
    DataStructure,
    /// Caching optimization
    Caching,
    /// Parallelization
    Parallelization,
}

impl PerformanceBenchmark {
    /// Create new performance benchmark
    pub fn new() -> Self {
        Self::default()
    }

    /// Set performance thresholds
    pub fn with_thresholds(mut self, thresholds: PerformanceThresholds) -> Self {
        self.thresholds = thresholds;
        self
    }

    /// Benchmark a generic operation
    pub fn benchmark_operation<F>(
        &mut self,
        operation_name: &str,
        operation: F,
        iterations: usize,
    ) -> BenchmarkResult
    where
        F: Fn() -> usize,
    {
        let mut times = Vec::with_capacity(iterations);
        let mut memory_usage = 0;

        for _ in 0..iterations {
            let start = Instant::now();
            
            // Execute operation and get memory usage
            memory_usage = operation();
            
            let duration = start.elapsed();
            times.push(duration);
        }

        let result = self.calculate_benchmark_result(operation_name, times, memory_usage, iterations);
        self.results.insert(operation_name.to_string(), result.clone());
        
        result
    }

    /// Benchmark memory usage
    pub fn benchmark_memory_usage(
        &mut self,
        operation: &str,
        f: impl Fn() -> usize,
        iterations: usize,
    ) -> BenchmarkResult {
        let mut memory_usage = Vec::with_capacity(iterations);
        let start = Instant::now();

        for _ in 0..iterations {
            let usage = f();
            memory_usage.push(usage);
        }

        let duration = start.elapsed();
        let avg_memory = memory_usage.iter().sum::<usize>() / memory_usage.len();
        let _min_memory = *memory_usage.iter().min().unwrap_or(&0);
        let _max_memory = *memory_usage.iter().max().unwrap_or(&0);

        let result = BenchmarkResult {
            operation: operation.to_string(),
            avg_time: duration / iterations as u32,
            min_time: Duration::from_nanos(0),
            max_time: Duration::from_nanos(0),
            iterations,
            memory_usage: avg_memory,
            performance_score: self.calculate_memory_score(avg_memory),
        };

        self.results.insert(operation.to_string(), result.clone());
        result
    }

    /// Run comprehensive benchmark suite
    pub fn run_benchmark_suite(
        &mut self,
        operations: Vec<(&str, BenchmarkOperation)>,
        iterations: usize,
    ) -> BenchmarkSuite {
        let mut suite = BenchmarkSuite::new();

        // Benchmark each operation
        for (name, operation) in operations {
            let result = self.benchmark_operation(name, &*operation, iterations);
            suite.add_result(name, result);
        }

        // Generate optimization suggestions
        self.generate_suggestions(&suite);

        suite
    }

    /// Get all benchmark results
    pub fn get_results(&self) -> &HashMap<String, BenchmarkResult> {
        &self.results
    }

    /// Get optimization suggestions
    pub fn get_suggestions(&self) -> &[OptimizationSuggestion] {
        &self.suggestions
    }

    /// Check if performance meets thresholds
    pub fn meets_thresholds(&self, result: &BenchmarkResult) -> bool {
        result.avg_time <= self.thresholds.max_transition_time
            && result.memory_usage <= self.thresholds.max_memory_usage
            && result.performance_score >= self.thresholds.min_performance_score
    }

    // Private helper methods
    fn calculate_benchmark_result(
        &self,
        operation: &str,
        times: Vec<Duration>,
        memory_usage: usize,
        iterations: usize,
    ) -> BenchmarkResult {
        let avg_time = times.iter().sum::<Duration>() / times.len() as u32;
        let min_time = *times.iter().min().unwrap_or(&Duration::from_nanos(0));
        let max_time = *times.iter().max().unwrap_or(&Duration::from_nanos(0));
        let performance_score = self.calculate_performance_score(avg_time, memory_usage);

        BenchmarkResult {
            operation: operation.to_string(),
            avg_time,
            min_time,
            max_time,
            iterations,
            memory_usage,
            performance_score,
        }
    }

    fn calculate_performance_score(&self, avg_time: Duration, memory_usage: usize) -> f64 {
        // Simple scoring algorithm (0-100)
        let time_score = if avg_time.as_micros() < 100 {
            100.0
        } else if avg_time.as_micros() < 1000 {
            80.0
        } else if avg_time.as_micros() < 10000 {
            60.0
        } else {
            40.0
        };

        let memory_score = if memory_usage < 1024 {
            100.0
        } else if memory_usage < 10240 {
            80.0
        } else if memory_usage < 102400 {
            60.0
        } else {
            40.0
        };

        (time_score + memory_score) / 2.0
    }

    fn calculate_memory_score(&self, memory_usage: usize) -> f64 {
        if memory_usage < 1024 {
            100.0
        } else if memory_usage < 10240 {
            80.0
        } else if memory_usage < 102400 {
            60.0
        } else {
            40.0
        }
    }

    fn generate_suggestions(&mut self, suite: &BenchmarkSuite) {
        self.suggestions.clear();

        for (name, result) in suite.get_results() {
            if !self.meets_thresholds(result) {
                // Generate suggestions based on performance issues
                if result.avg_time > self.thresholds.max_transition_time {
                    self.suggestions.push(OptimizationSuggestion {
                        suggestion_type: OptimizationType::Algorithm,
                        description: format!("Optimize {} transitions - current: {:?}, target: {:?}", 
                            name, result.avg_time, self.thresholds.max_transition_time),
                        expected_improvement: 0.3,
                        priority: 4,
                    });
                }

                if result.memory_usage > self.thresholds.max_memory_usage {
                    self.suggestions.push(OptimizationSuggestion {
                        suggestion_type: OptimizationType::Memory,
                        description: format!("Reduce {} memory usage - current: {} bytes, target: {} bytes", 
                            name, result.memory_usage, self.thresholds.max_memory_usage),
                        expected_improvement: 0.4,
                        priority: 3,
                    });
                }

                if result.performance_score < self.thresholds.min_performance_score {
                    self.suggestions.push(OptimizationSuggestion {
                        suggestion_type: OptimizationType::DataStructure,
                        description: format!("Improve {} overall performance - current: {:.1}, target: {:.1}", 
                            name, result.performance_score, self.thresholds.min_performance_score),
                        expected_improvement: 0.5,
                        priority: 5,
                    });
                }
            }
        }
    }
}

/// Complete benchmark suite results
#[derive(Clone, Debug)]
pub struct BenchmarkSuite {
    /// All benchmark results
    results: HashMap<String, BenchmarkResult>,
    /// Overall performance score
    overall_score: f64,
    /// Suite execution time
    execution_time: Duration,
}

impl Default for BenchmarkSuite {
    fn default() -> Self {
        Self {
            results: HashMap::new(),
            overall_score: 0.0,
            execution_time: Duration::from_nanos(0),
        }
    }
}

impl BenchmarkSuite {
    /// Create new benchmark suite
    pub fn new() -> Self {
        Self::default()
    }

    /// Add benchmark result
    pub fn add_result(&mut self, name: &str, result: BenchmarkResult) {
        self.results.insert(name.to_string(), result);
        self.calculate_overall_score();
    }

    /// Get all results
    pub fn get_results(&self) -> &HashMap<String, BenchmarkResult> {
        &self.results
    }

    /// Get overall performance score
    pub fn get_overall_score(&self) -> f64 {
        self.overall_score
    }

    /// Get performance summary
    pub fn get_summary(&self) -> String {
        let mut summary = String::new();
        summary.push_str("Benchmark Suite Results\n");
        summary.push_str(&format!("Overall Score: {:.1}/100\n", self.overall_score));
        summary.push_str(&format!("Execution Time: {:?}\n", self.execution_time));
        summary.push_str(&format!("Tests Run: {}\n\n", self.results.len()));

        for (name, result) in &self.results {
            summary.push_str(&format!("{}:\n", name));
            summary.push_str(&format!("  Score: {:.1}/100\n", result.performance_score));
            summary.push_str(&format!("  Avg Time: {:?}\n", result.avg_time));
            summary.push_str(&format!("  Memory: {} bytes\n", result.memory_usage));
            summary.push_str(&format!("  Iterations: {}\n\n", result.iterations));
        }

        summary
    }

    /// Check if suite meets performance requirements
    pub fn meets_requirements(&self, min_score: f64) -> bool {
        self.overall_score >= min_score
    }

    // Private helper methods
    fn calculate_overall_score(&mut self) {
        if self.results.is_empty() {
            self.overall_score = 0.0;
            return;
        }

        let total_score: f64 = self.results.values().map(|r| r.performance_score).sum();
        self.overall_score = total_score / self.results.len() as f64;
    }
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_transition_time: Duration::from_micros(100),
            max_memory_usage: 10240, // 10KB
            min_performance_score: 70.0,
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_benchmark_creation() {
        let benchmark = PerformanceBenchmark::new();
        
        assert_eq!(benchmark.get_results().len(), 0);
        assert_eq!(benchmark.get_suggestions().len(), 0);
    }

    #[test]
    fn test_performance_thresholds_default() {
        let thresholds = PerformanceThresholds::default();
        
        assert_eq!(thresholds.max_transition_time, Duration::from_micros(100));
        assert_eq!(thresholds.max_memory_usage, 10240);
        assert_eq!(thresholds.min_performance_score, 70.0);
    }

    #[test]
    fn test_benchmark_result_creation() {
        let result = BenchmarkResult {
            operation: "test".to_string(),
            avg_time: Duration::from_micros(50),
            min_time: Duration::from_micros(40),
            max_time: Duration::from_micros(60),
            iterations: 100,
            memory_usage: 1024,
            performance_score: 85.0,
        };
        
        assert_eq!(result.operation, "test");
        assert_eq!(result.avg_time, Duration::from_micros(50));
        assert_eq!(result.performance_score, 85.0);
    }

    #[test]
    fn test_optimization_suggestion() {
        let suggestion = OptimizationSuggestion {
            suggestion_type: OptimizationType::Memory,
            description: "Reduce memory usage".to_string(),
            expected_improvement: 0.3,
            priority: 4,
        };
        
        assert_eq!(suggestion.suggestion_type, OptimizationType::Memory);
        assert_eq!(suggestion.priority, 4);
        assert_eq!(suggestion.expected_improvement, 0.3);
    }

    #[test]
    fn test_benchmark_suite_creation() {
        let suite = BenchmarkSuite::new();
        
        assert_eq!(suite.get_results().len(), 0);
        assert_eq!(suite.get_overall_score(), 0.0);
    }

    #[test]
    fn test_benchmark_suite_add_result() {
        let mut suite = BenchmarkSuite::new();
        
        let result = BenchmarkResult {
            operation: "test".to_string(),
            avg_time: Duration::from_micros(50),
            min_time: Duration::from_micros(40),
            max_time: Duration::from_micros(60),
            iterations: 100,
            memory_usage: 1024,
            performance_score: 85.0,
        };
        
        suite.add_result("test", result);
        
        assert_eq!(suite.get_results().len(), 1);
        assert_eq!(suite.get_overall_score(), 85.0);
    }

    #[test]
    fn test_benchmark_suite_requirements() {
        let mut suite = BenchmarkSuite::new();
        
        let result = BenchmarkResult {
            operation: "test".to_string(),
            avg_time: Duration::from_micros(50),
            min_time: Duration::from_micros(40),
            max_time: Duration::from_micros(60),
            iterations: 100,
            memory_usage: 1024,
            performance_score: 85.0,
        };
        
        suite.add_result("test", result);
        
        assert!(suite.meets_requirements(80.0));
        assert!(!suite.meets_requirements(90.0));
    }

    #[test]
    fn test_performance_benchmark_with_thresholds() {
        let mut thresholds = PerformanceThresholds::default();
        thresholds.max_transition_time = Duration::from_micros(50);
        thresholds.min_performance_score = 80.0;
        
        let benchmark = PerformanceBenchmark::new()
            .with_thresholds(thresholds);
        
        let result = BenchmarkResult {
            operation: "test".to_string(),
            avg_time: Duration::from_micros(30),
            min_time: Duration::from_micros(25),
            max_time: Duration::from_micros(35),
            iterations: 100,
            memory_usage: 1024,
            performance_score: 85.0,
        };
        
        assert!(benchmark.meets_thresholds(&result));
    }

    #[test]
    fn test_optimization_type_comparison() {
        let memory = OptimizationType::Memory;
        let algorithm = OptimizationType::Algorithm;
        
        assert_ne!(memory, algorithm);
        assert_eq!(memory, OptimizationType::Memory);
    }

    #[test]
    fn test_benchmark_suite_summary() {
        let mut suite = BenchmarkSuite::new();
        
        let result = BenchmarkResult {
            operation: "test".to_string(),
            avg_time: Duration::from_micros(50),
            min_time: Duration::from_micros(40),
            max_time: Duration::from_micros(60),
            iterations: 100,
            memory_usage: 1024,
            performance_score: 85.0,
        };
        
        suite.add_result("test", result);
        let summary = suite.get_summary();
        
        assert!(summary.contains("Overall Score: 85.0/100"));
        assert!(summary.contains("test:"));
        assert!(summary.contains("Score: 85.0/100"));
        assert!(summary.contains("Memory: 1024 bytes"));
    }

    #[test]
    fn test_benchmark_operation() {
        let mut benchmark = PerformanceBenchmark::new();
        
        let result = benchmark.benchmark_operation(
            "test_op",
            || 1024, // Return memory usage
            10,
        );
        
        assert_eq!(result.operation, "test_op");
        assert_eq!(result.iterations, 10);
        assert_eq!(result.memory_usage, 1024);
        assert!(result.avg_time > Duration::from_nanos(0));
    }

    #[test]
    fn test_benchmark_memory_usage() {
        let mut benchmark = PerformanceBenchmark::new();
        
        let result = benchmark.benchmark_memory_usage(
            "memory_test",
            || 2048,
            5,
        );
        
        assert_eq!(result.operation, "memory_test");
        assert_eq!(result.iterations, 5);
        assert_eq!(result.memory_usage, 2048);
    }

    #[test]
    fn test_benchmark_suite() {
        let mut benchmark = PerformanceBenchmark::new();
        
        let operations = vec![
            ("op1", Box::new(|| 1024usize) as Box<dyn Fn() -> usize>),
            ("op2", Box::new(|| 2048usize) as Box<dyn Fn() -> usize>),
        ];
        
        let suite = benchmark.run_benchmark_suite(operations, 5);
        
        assert_eq!(suite.get_results().len(), 2);
        assert!(suite.get_overall_score() > 0.0);
    }
}
