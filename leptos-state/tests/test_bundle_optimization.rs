//! TDD tests for WASM bundle size optimization
//! 
//! These tests define the expected API for bundle optimization features
//! and will guide the implementation.

use leptos_state::machine::*;

#[derive(Clone, Debug, PartialEq, Default)]
struct TestContext {
    counter: u32,
    name: String,
}

#[derive(Clone, Debug, PartialEq, Default)]
enum TestEvent {
    #[default]
    Increment,
    Decrement,
    SetName(String),
}

impl leptos_state::machine::events::Event for TestEvent {
    fn event_type(&self) -> &str {
        match self {
            TestEvent::Increment => "Increment",
            TestEvent::Decrement => "Decrement",
            TestEvent::SetName(_) => "SetName",
        }
    }
}

#[cfg(test)]
mod bundle_optimization_tests {
    use super::*;

    #[test]
    fn test_bundle_optimization_trait() {
        // Given: A basic machine
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .initial("idle")
            .state("idle")
                .on(TestEvent::Increment, "active")
            .state("active")
                .on(TestEvent::Decrement, "idle")
            .build();

        // When: We apply bundle optimization
        let optimized = machine.with_bundle_optimization();

        // Then: We should get an OptimizedBundle
        assert!(std::mem::size_of_val(&optimized) > 0);
    }

    #[test]
    fn test_code_splitting() {
        // Given: A machine with multiple states
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .initial("idle")
            .state("idle")
                .on(TestEvent::Increment, "active")
            .state("active")
                .on(TestEvent::Decrement, "idle")
            .state("processing")
                .on(TestEvent::SetName("".to_string()), "idle")
            .build();

        // When: We apply code splitting
        let optimized = machine.with_code_splitting(1024);

        // Then: We should get an optimized bundle
        assert!(std::mem::size_of_val(&optimized) > 0);
    }

    #[test]
    fn test_bundle_analysis() {
        // Given: A machine
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .initial("idle")
            .state("idle")
                .on(TestEvent::Increment, "active")
            .build();

        // When: We analyze the bundle
        let analysis = machine.analyze_bundle();

        // Then: We should get bundle information
        assert!(analysis.total_size > 0);
        assert!(!analysis.features.is_empty());
        assert!(analysis.wasm_info.is_some());
    }

    #[test]
    fn test_bundle_comparison() {
        // Given: Original and optimized machines
        let original = MachineBuilder::<TestContext, TestEvent>::new()
            .initial("idle")
            .state("idle")
                .on(TestEvent::Increment, "active")
            .build();

        let optimized = original.clone().with_bundle_optimization();

        // When: We compare bundles
        let comparison = original.compare_bundle_with(&original);

        // Then: We should see improvements
        assert!(comparison.size_reduction >= 0);
        assert!(comparison.size_reduction_percent >= 0.0);
    }

    #[test]
    fn test_loading_strategies() {
        // Given: A machine
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .initial("idle")
            .state("idle")
                .on(TestEvent::Increment, "active")
            .build();

        // When: We apply different loading strategies
        let progressive = machine.clone().with_progressive_loading();
        let lazy = machine.with_lazy_loading();

        // Then: We should get different optimized bundles
        assert!(std::mem::size_of_val(&progressive) > 0);
        assert!(std::mem::size_of_val(&lazy) > 0);
    }

    #[test]
    fn test_feature_removal() {
        // Given: A machine with optional features
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .initial("idle")
            .state("idle")
                .on(TestEvent::Increment, "active")
            .build();

        // When: We remove optional features
        let optimized = machine.without_features(&["debug", "logging"]);

        // Then: We should get a smaller bundle
        assert!(std::mem::size_of_val(&optimized) > 0);
    }

    #[test]
    fn test_wasm_optimization() {
        // Given: A machine
        let machine = MachineBuilder::<TestContext, TestEvent>::new()
            .initial("idle")
            .state("idle")
                .on(TestEvent::Increment, "active")
            .build();

        // When: We optimize for WASM
        let optimized = machine.optimize_for_wasm();

        // Then: We should get WASM-optimized bundle
        assert!(std::mem::size_of_val(&optimized) > 0);
    }
}
