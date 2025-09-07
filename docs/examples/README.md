# 🚀 Leptos State Examples

This directory contains comprehensive examples demonstrating the **leptos-state v1.0.0** library's capabilities, from basic usage to complex real-world applications with modern Rust and Leptos 0.8+.

## 📚 Example Categories

### 🎯 **Basic Examples**
Simple examples to get you started with state management.

- **[Counter](./counter/)**: Basic store usage with reactive updates
- **[Traffic Light](./traffic-light/)**: Simple state machine demonstration
- **[History](./history/)**: State history and undo/redo functionality

### 🏗️ **Intermediate Examples**
More complex patterns and real-world use cases.

- **[Todo App](./todo-app/)**: Full CRUD application with state management
- **[Compatibility Example](./compatibility-example/)**: Compatibility testing and validation

### 🚀 **Advanced Examples**
Complex applications showcasing advanced features.

- **[Analytics Dashboard](./analytics-dashboard/)**: Real-time data visualization
- **[Code Generator](./codegen/)**: Multi-language code generation with state machines
- **[Ecosystem Integration](./ecosystem-integration/)**: Integration with companion crates

### 🧪 **Feature Examples**
Examples demonstrating specific features and capabilities.

- **Actions Example**: State machine actions and side effects
- **Guards Example**: State machine guards and conditions
- **Persistence Example**: State persistence and serialization
- **Testing Example**: Testing utilities and patterns
- **Visualization Example**: State machine visualization
- **Performance Example**: Performance monitoring and optimization
- **Integration Example**: Integration testing patterns
- **Documentation Example**: Code generation and documentation

## 🚀 Getting Started

### **Prerequisites**
```bash
# Install Rust and Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Trunk for WASM examples
cargo install trunk

# Install Node.js and pnpm
curl -fsSL https://get.pnpm.io/install.sh | sh -
```

### **Running Examples**

#### **Rust Examples**
```bash
# Navigate to example directory
cd examples/counter

# Build and run
cargo run
```

#### **WASM Examples**
```bash
# Navigate to example directory
cd examples/counter

# Build with Trunk
trunk build

# Serve locally
trunk serve
```

#### **All Examples**
```bash
# Build all examples
cargo build --workspace

# Test all examples
cargo test --workspace

# Build WASM examples
./scripts/build-wasm-examples.sh
```

## 📖 Example Structure

Each example follows a consistent structure:

```
example-name/
├── Cargo.toml          # Dependencies and configuration
├── src/
│   ├── main.rs         # Main application logic
│   ├── components.rs    # UI components
│   └── state.rs        # State management logic
├── Trunk.toml          # WASM build configuration (if applicable)
├── index.html          # HTML entry point (if applicable)
├── styles.css          # Styling (if applicable)
└── README.md           # Example-specific documentation
```

## 🎯 Learning Path

### **Beginner** 🟢
1. **Counter**: Learn basic store creation and usage
2. **Traffic Light**: Understand state machines
3. **History**: Practice state history and undo/redo

### **Intermediate** 🟡
1. **Todo App**: Build a complete CRUD application
2. **Compatibility Example**: Test compatibility and validation
3. **Actions Example**: Learn state machine actions

### **Advanced** 🔴
1. **Analytics Dashboard**: Real-time data and complex state
2. **Code Generator**: Advanced state machine patterns
3. **Guards Example**: Complex state machine conditions
4. **Persistence Example**: State persistence and serialization
5. **Testing Example**: Advanced testing patterns
6. **Visualization Example**: State machine visualization
7. **Performance Example**: Performance monitoring and optimization

## 🔧 Example Features

### **State Management**
- **Stores**: Zustand-inspired state containers
- **State Machines**: XState-style finite state machines
- **Hooks**: React-style hooks for Leptos
- **Middleware**: Extensible middleware system

### **Reactivity**
- **Signals**: Leptos reactive primitives
- **Effects**: Side effects and subscriptions
- **Computed**: Derived state and selectors
- **Batching**: Optimized state updates

### **Persistence**
- **Local Storage**: Browser persistence
- **Session Storage**: Session-based persistence
- **Custom Storage**: Custom persistence adapters
- **Serialization**: State serialization and deserialization

### **DevTools**
- **Time Travel**: Undo/redo functionality
- **State Inspection**: Visual state debugging
- **Performance Profiling**: Performance monitoring
- **Action Logging**: Action and transition logging

## 🧪 Testing Examples

### **Unit Tests**
```bash
# Test specific example
cargo test -p counter-example

# Test all examples
cargo test --workspace
```

### **Integration Tests**
```bash
# Run integration tests
cargo test --test integration

# Test with specific features
cargo test --features persist,visualization
```

### **WASM Tests**
```bash
# Test WASM compilation
wasm-pack test --headless

# Browser testing
wasm-pack test --headless --chrome
```

## 📊 Example Metrics

### **Performance**
- **Bundle Size**: <100KB for basic examples
- **Load Time**: <1 second for simple examples
- **Memory Usage**: <10MB for typical applications
- **CPU Usage**: <5% for idle applications

### **Compatibility**
- **Browsers**: Chrome, Firefox, Safari, Edge
- **Mobile**: iOS Safari, Chrome Mobile
- **WASM**: Full WebAssembly support
- **SSR**: Server-side rendering support

## 🔗 Related Documentation

- **[User Guide](../user-guide/)**: Comprehensive usage tutorials
- **[API Reference](../api-reference/)**: Complete API documentation
- **[Migration Guide](../migration/)**: Upgrade from other solutions
- **[Development Guide](../development/)**: Advanced development patterns

## 🆘 Getting Help

### **Example Issues**
- **Build Problems**: Check Rust and WASM toolchain
- **Runtime Errors**: Verify browser compatibility
- **Performance Issues**: Review state management patterns
- **Styling Problems**: Check CSS and HTML structure

### **Resources**
- **GitHub Issues**: [Report example problems](https://github.com/cloud-shuttle/leptos-state/issues)
- **Discussions**: [Ask questions](https://github.com/cloud-shuttle/leptos-state/discussions)
- **Documentation**: [Comprehensive guides](../user-guide/)

---

*Explore these examples to master leptos-state and build amazing applications! 🚀✨*
