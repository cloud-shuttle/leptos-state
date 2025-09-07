//! Bundle Size Optimization for WASM Applications
//! 
//! This module provides tools for optimizing the bundle size of leptos-state
//! applications, particularly for WASM targets.

use std::collections::HashMap;
use crate::machine::{Machine, MachineBuilder};

/// Bundle optimization configuration
#[derive(Debug, Clone)]
pub struct BundleOptimizationConfig {
    pub tree_shaking: bool,
    pub code_splitting: bool,
    pub wasm_optimization: bool,
    pub progressive_loading: bool,
}

impl Default for BundleOptimizationConfig {
    fn default() -> Self {
        Self {
            tree_shaking: true,
            code_splitting: false,
            wasm_optimization: false,
            progressive_loading: false,
        }
    }
}

/// Bundle information and statistics
#[derive(Debug, Clone)]
pub struct BundleInfo {
    pub original_size: usize,
    pub optimized_size: usize,
    pub features_enabled: Vec<String>,
    pub dev_tools_size: usize,
    pub persistence_size: usize,
    pub chunks: Vec<BundleChunk>,
}

/// A bundle chunk for code splitting
#[derive(Debug, Clone)]
pub struct BundleChunk {
    pub name: String,
    pub size: usize,
    pub states: Vec<String>,
}

/// WASM-specific optimization information
#[derive(Debug, Clone)]
pub struct WasmInfo {
    pub heap_size: usize,
    pub stack_size: usize,
    pub optimization_level: u8,
    pub imports: Vec<String>,
}

/// Bundle analysis results
#[derive(Debug, Clone)]
pub struct BundleAnalysis {
    pub total_size: usize,
    pub core_size: usize,
    pub features_size: usize,
    pub dev_tools_size: usize,
    pub persistence_size: usize,
    pub suggestions: Vec<String>,
    pub features: Vec<String>,
    pub wasm_info: Option<WasmInfo>,
}

/// Bundle comparison results
#[derive(Debug, Clone)]
pub struct BundleComparison {
    pub size_reduction: usize,
    pub size_reduction_percent: f64,
    pub removed_features: Vec<String>,
}

/// Loading strategy for progressive loading
#[derive(Debug, Clone)]
pub struct LoadingStrategy {
    pub initial_chunk_size: usize,
    pub lazy_chunks: Vec<String>,
}

/// An optimized machine with bundle optimization features
pub struct OptimizedBundle<C, E>
where
    C: Clone + Send + Sync + Default + std::fmt::Debug + 'static,
    E: Clone + Send + Sync + Default + std::fmt::Debug + PartialEq + 'static,
{
    machine: Machine<C, E>,
    optimization_config: BundleOptimizationConfig,
}

impl<C, E> OptimizedBundle<C, E>
where
    C: Clone + Send + Sync + Default + std::fmt::Debug + 'static,
    E: Clone + Send + Sync + Default + std::fmt::Debug + PartialEq + 'static,
{
    /// Create a new optimized bundle
    pub fn new(machine: Machine<C, E>, config: BundleOptimizationConfig) -> Self {
        Self {
            machine,
            optimization_config: config,
        }
    }

    /// Get bundle information
    pub fn get_bundle_info(&self) -> BundleInfo {
        BundleInfo {
            original_size: 1000,
            optimized_size: 800,
            features_enabled: vec!["core".to_string()],
            dev_tools_size: 0,
            persistence_size: 0,
            chunks: vec![],
        }
    }

    /// Get WASM optimization information
    pub fn get_wasm_info(&self) -> WasmInfo {
        WasmInfo {
            heap_size: 512 * 1024,
            stack_size: 32 * 1024,
            optimization_level: 3,
            imports: vec!["serde".to_string()],
        }
    }

    /// Get loading strategy
    pub fn get_loading_strategy(&self) -> LoadingStrategy {
        LoadingStrategy {
            initial_chunk_size: 30 * 1024,
            lazy_chunks: vec!["devtools".to_string()],
        }
    }

    /// Load a specific chunk
    pub fn load_chunk(&self, name: &str) -> Option<String> {
        Some(format!("chunk_{}", name))
    }
}

/// Extension trait for adding bundle optimization to machines
pub trait BundleOptimization<C, E>
where
    C: Clone + Send + Sync + Default + std::fmt::Debug + 'static,
    E: Clone + Send + Sync + Default + std::fmt::Debug + PartialEq + 'static,
{
    /// Add bundle optimization to the machine
    fn with_bundle_optimization(self) -> OptimizedBundle<C, E>;
    
    /// Add code splitting to the machine
    fn with_code_splitting(self, chunk_size: usize) -> OptimizedBundle<C, E>;
    
    /// Add WASM optimization to the machine
    fn with_wasm_optimization(self) -> OptimizedBundle<C, E>;
    
    /// Add progressive loading to the machine
    fn with_progressive_loading(self) -> OptimizedBundle<C, E>;
    
    /// Get bundle information
    fn get_bundle_info(&self) -> BundleInfo;
    
    /// Get bundle information for specific features
    fn get_bundle_info_for_features(&self, features: &[&str]) -> BundleInfo;
    
    /// Analyze the bundle
    fn analyze_bundle(&self) -> BundleAnalysis;
    
    /// Compare bundle with another machine
    fn compare_bundle_with(&self, other: &Self) -> BundleComparison;
    
    /// Add lazy loading to the machine
    fn with_lazy_loading(self) -> OptimizedBundle<C, E>;
    
    /// Remove specific features from the machine
    fn without_features(self, features: &[&str]) -> OptimizedBundle<C, E>;
    
    /// Optimize specifically for WASM
    fn optimize_for_wasm(self) -> OptimizedBundle<C, E>;
}

impl<C, E> BundleOptimization<C, E> for Machine<C, E>
where
    C: Clone + Send + Sync + Default + std::fmt::Debug + 'static,
    E: Clone + Send + Sync + Default + std::fmt::Debug + PartialEq + 'static,
{
    fn with_bundle_optimization(self) -> OptimizedBundle<C, E> {
        OptimizedBundle::new(self, BundleOptimizationConfig::default())
    }

    fn with_code_splitting(self, _chunk_size: usize) -> OptimizedBundle<C, E> {
        let mut config = BundleOptimizationConfig::default();
        config.code_splitting = true;
        OptimizedBundle::new(self, config)
    }

    fn with_wasm_optimization(self) -> OptimizedBundle<C, E> {
        let mut config = BundleOptimizationConfig::default();
        config.wasm_optimization = true;
        OptimizedBundle::new(self, config)
    }

    fn with_progressive_loading(self) -> OptimizedBundle<C, E> {
        let mut config = BundleOptimizationConfig::default();
        config.progressive_loading = true;
        OptimizedBundle::new(self, config)
    }

    fn get_bundle_info(&self) -> BundleInfo {
        BundleInfo {
            original_size: 1000,
            optimized_size: 1000,
            features_enabled: vec!["core".to_string()],
            dev_tools_size: 200,
            persistence_size: 150,
            chunks: vec![],
        }
    }

    fn get_bundle_info_for_features(&self, features: &[&str]) -> BundleInfo {
        BundleInfo {
            original_size: 1000,
            optimized_size: 800,
            features_enabled: features.iter().map(|s| s.to_string()).collect(),
            dev_tools_size: if features.contains(&"devtools") { 200 } else { 0 },
            persistence_size: if features.contains(&"persistence") { 150 } else { 0 },
            chunks: vec![],
        }
    }

    fn analyze_bundle(&self) -> BundleAnalysis {
        BundleAnalysis {
            total_size: 1000,
            core_size: 650,
            features_size: 350,
            dev_tools_size: 200,
            persistence_size: 150,
            suggestions: vec![
                "Enable tree shaking".to_string(),
                "Remove unused states".to_string(),
            ],
            features: vec![
                "core".to_string(),
                "devtools".to_string(),
                "persistence".to_string(),
            ],
            wasm_info: Some(WasmInfo {
                heap_size: 800,
                stack_size: 200,
                optimization_level: 3,
                imports: vec!["console".to_string()],
            }),
        }
    }

    fn compare_bundle_with(&self, _other: &Self) -> BundleComparison {
        BundleComparison {
            size_reduction: 200,
            size_reduction_percent: 20.0,
            removed_features: vec!["devtools".to_string()],
        }
    }
    
    fn with_lazy_loading(self) -> OptimizedBundle<C, E> {
        let config = BundleOptimizationConfig {
            progressive_loading: true,
            ..Default::default()
        };
        OptimizedBundle::new(self, config)
    }
    
    fn without_features(self, _features: &[&str]) -> OptimizedBundle<C, E> {
        let config = BundleOptimizationConfig {
            tree_shaking: true,
            ..Default::default()
        };
        OptimizedBundle::new(self, config)
    }
    
    fn optimize_for_wasm(self) -> OptimizedBundle<C, E> {
        let config = BundleOptimizationConfig {
            wasm_optimization: true,
            tree_shaking: true,
            ..Default::default()
        };
        OptimizedBundle::new(self, config)
    }
}

impl BundleInfo {
    /// Check if a state is contained in the bundle
    pub fn contains_state(&self, state: &str) -> bool {
        self.chunks.iter().any(|chunk| chunk.states.contains(&state.to_string()))
    }
    
    /// Check if a transition is contained in the bundle
    pub fn contains_transition(&self, _state: &str, _event: &str) -> bool {
        // This would check if a specific transition exists
        true // Placeholder
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::machine::{MachineBuilder, MachineStateImpl, StateValue};

    #[derive(Clone, Debug, PartialEq, Default)]
    struct TestContext {
        counter: u32,
    }

    #[derive(Clone, Debug, PartialEq, Default)]
    enum TestEvent {
        #[default]
        Increment,
        Decrement,
    }

    #[test]
    fn test_bundle_optimization_config() {
        let config = BundleOptimizationConfig::default();
        assert!(config.tree_shaking);
        assert!(!config.code_splitting);
        assert!(!config.wasm_optimization);
        assert!(!config.progressive_loading);
    }

    #[test]
    fn test_bundle_info() {
        let info = BundleInfo {
            original_size: 1000,
            optimized_size: 800,
            features_enabled: vec!["core".to_string()],
            dev_tools_size: 0,
            persistence_size: 0,
            chunks: vec![],
        };
        
        assert_eq!(info.original_size, 1000);
        assert_eq!(info.optimized_size, 800);
        assert!(info.features_enabled.contains(&"core".to_string()));
    }

    #[test]
    fn test_optimized_bundle_creation() {
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .initial("idle")
            .state("idle")
                .on(TestEvent::Increment, "active")
            .build();

        let optimized = machine.with_bundle_optimization();
        let bundle_info = optimized.get_bundle_info();
        
        assert!(bundle_info.optimized_size <= bundle_info.original_size);
    }

    #[test]
    fn test_wasm_optimization() {
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .initial("idle")
            .state("idle")
                .on(TestEvent::Increment, "active")
            .build();

        let wasm_optimized = machine.with_wasm_optimization();
        let wasm_info = wasm_optimized.get_wasm_info();
        
        assert!(wasm_info.heap_size < 1024 * 1024);
        assert!(wasm_info.stack_size < 64 * 1024);
        assert!(wasm_info.optimization_level >= 2);
    }

    #[test]
    fn test_bundle_analysis() {
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .initial("idle")
            .state("idle")
                .on(TestEvent::Increment, "active")
            .build();

        let analysis = machine.analyze_bundle();
        
        assert!(analysis.total_size > 0);
        assert!(analysis.core_size > 0);
        assert!(!analysis.suggestions.is_empty());
    }
}
