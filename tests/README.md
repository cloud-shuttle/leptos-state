# 🧪 Leptos State Test Suite

This directory contains the comprehensive test suite for the **leptos-state** library, organized by type and purpose.

## 🗂️ Test Organization

### 🦀 [Rust Tests](./rust/)
Core Rust-based tests for the library functionality.

#### **Unit Tests** (`./rust/unit/`)
- **Core Tests**: Basic functionality and edge cases
- **Store Tests**: State management and reactivity
- **Machine Tests**: State machine logic and transitions
- **Hook Tests**: React-style hooks functionality
- **Utility Tests**: Helper functions and utilities

#### **Integration Tests** (`./rust/integration/`)
- **End-to-End**: Complete workflow testing
- **Cross-Module**: Integration between different components
- **Performance**: Performance benchmarks and profiling
- **Fixtures**: Test data and setup utilities

#### **Performance Tests** (`./rust/performance/`)
- **Benchmarks**: Performance measurement tools
- **Profiling**: Memory and CPU usage analysis
- **Stress Tests**: High-load scenario testing

#### **Examples Tests** (`./rust/examples/`)
- **Code Generation**: Generated code validation
- **Documentation**: Documentation accuracy tests
- **Template Tests**: Template rendering validation

### 🌐 [Web Tests](./web/)
Web-based tests using Playwright for WASM and browser testing.

#### **Playwright Tests** (`./web/playwright/`)
- **WASM Examples**: End-to-end testing of compiled examples
- **Browser Integration**: Cross-browser compatibility
- **User Interactions**: Click, type, and navigation testing
- **Visual Regression**: UI consistency validation

#### **Test Pages** (`./web/playwright/test-pages/`)
- **Counter Example**: Interactive counter testing
- **Traffic Light**: State machine demonstration testing
- **Custom Test Pages**: Specialized testing scenarios

### 📊 [Test Results](./test-results/)
Generated test results and reports.

## 🚀 Running Tests

### Rust Tests
```bash
# Run all Rust tests
cargo test

# Run specific test categories
cargo test --package leptos-state
cargo test --test integration
cargo test --test unit

# Run with output
cargo test -- --nocapture
```

### Web Tests
```bash
# Install Playwright
pnpm install

# Run Playwright tests
pnpm test:playwright

# Run specific test suites
pnpm test:wasm
pnpm test:browser
```

### Performance Tests
```bash
# Run benchmarks
cargo bench

# Run specific benchmarks
cargo bench --package leptos-state
```

## 🧪 Test Categories

### **Unit Tests**
- **Fast execution** (< 1 second per test)
- **Isolated testing** of individual functions
- **Mock dependencies** for controlled testing
- **Edge case coverage** for robustness

### **Integration Tests**
- **Real dependencies** and actual workflows
- **Cross-module testing** of interactions
- **Performance validation** of complete systems
- **Error handling** and recovery testing

### **Web Tests**
- **WASM compilation** and execution
- **Browser compatibility** across platforms
- **User interaction** simulation
- **Visual consistency** validation

### **Performance Tests**
- **Benchmarking** of critical paths
- **Memory profiling** and leak detection
- **CPU usage** optimization validation
- **Scalability** testing for large datasets

## 📋 Test Coverage

### **Core Library**
- ✅ **Stores**: State management and reactivity
- ✅ **State Machines**: Transitions, guards, and actions
- ✅ **Hooks**: React-style hook implementations
- ✅ **Middleware**: Extensibility and plugins
- ✅ **Persistence**: State serialization and storage

### **Examples**
- ✅ **Counter**: Basic state management
- ✅ **Traffic Light**: State machine demonstration
- ✅ **Todo App**: CRUD operations
- ✅ **Analytics Dashboard**: Complex state patterns
- ✅ **Code Generation**: Multi-language output

### **Integration**
- ✅ **Leptos 0.8+ Compatibility**: Full API validation
- ✅ **WASM Support**: WebAssembly compilation
- ✅ **Performance**: Optimization validation
- ✅ **Documentation**: Accuracy and completeness

## 🔧 Test Configuration

### **Environment Variables**
```bash
# Enable verbose test output
RUST_LOG=debug

# Run specific test suites
TEST_SUITE=unit,integration

# Performance test configuration
PERF_ITERATIONS=1000
PERF_TIMEOUT=30s
```

### **Test Features**
```toml
[features]
test-utils = []           # Testing utilities
performance-tests = []     # Performance benchmarks
integration-tests = []     # Integration test suite
web-tests = []            # Web and WASM tests
```

## 📊 Test Metrics

### **Coverage Goals**
- **Unit Tests**: >95% line coverage
- **Integration Tests**: >90% workflow coverage
- **Web Tests**: >85% user interaction coverage
- **Performance Tests**: <10% performance regression

### **Quality Metrics**
- **Test Execution**: <30 seconds for full suite
- **Flakiness**: <1% test failure rate
- **Performance**: <5% variance in benchmarks
- **Documentation**: 100% public API coverage

## 🆘 Troubleshooting

### **Common Issues**
1. **Test Timeouts**: Increase timeout values for slow tests
2. **Memory Issues**: Reduce test data size for memory-constrained environments
3. **Browser Issues**: Update Playwright and browser drivers
4. **WASM Issues**: Ensure proper WASM target setup

### **Getting Help**
- **Test Failures**: Check test output and logs
- **Performance Issues**: Review benchmark results
- **Setup Problems**: Verify development environment
- **Documentation**: Review test strategy and implementation guides

---

*Comprehensive testing ensures the reliability and quality of the leptos-state library! 🧪✨*
