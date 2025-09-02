# 🤝 Contributing to leptos-state

## 🎯 **Quick Start for Contributors**

### **Prerequisites**
- Rust 1.70+ installed
- Git for version control
- Basic understanding of Rust and Leptos

### **Setup Development Environment**
```bash
# Clone the repository
git clone https://github.com/cloud-shuttle/leptos-state.git
cd leptos-state

# Install dependencies
cargo build

# Run tests to ensure everything works
cargo test -p leptos-state
```

## 🚧 **Current Priority: Fix CI Issues**

The library is **functionally complete** but has CI pipeline issues that need fixing. These are **architectural improvements** that will make the codebase more robust.

### **What We're Working On**
1. **Variable Naming Issues** - Fix compilation conflicts between local and CI environments
2. **Serde Trait Bounds** - Resolve generic type serialization constraints
3. **Extension Trait Bounds** - Fix trait bound mismatches for optional features
4. **Async Store Type Inference** - Resolve generic type inference problems

### **How to Help**

#### **Option 1: Pick an Issue**
1. Check the [CI Issues Analysis](CI_ISSUES_ANALYSIS.md) document
2. Choose an issue that matches your expertise
3. Create a branch: `git checkout -b fix/issue-description`
4. Implement the fix following the solution strategy
5. Test locally: `cargo check -p leptos-state`
6. Submit a PR with detailed description

#### **Option 2: Test and Report**
1. Try different feature combinations
2. Test edge cases and unusual configurations
3. Report any compilation errors or warnings
4. Help identify patterns in the issues

#### **Option 3: Documentation**
1. Improve error messages and debugging info
2. Update examples and guides
3. Add helpful comments to complex code sections
4. Create migration guides for breaking changes

## 🔧 **Development Workflow**

### **1. Local Development**
```bash
# Check compilation
cargo check -p leptos-state

# Run tests
cargo test -p leptos-state

# Check formatting
cargo fmt

# Run clippy
cargo clippy -p leptos-state
```

### **2. Testing Different Features**
```bash
# Test with persistence features
cargo check -p leptos-state --features persist,visualization

# Test without default features
cargo check -p leptos-state --no-default-features

# Test specific feature combinations
cargo check -p leptos-state --features persist,testing,devtools
```

### **3. WASM Testing**
```bash
# Install trunk if not already installed
cargo install trunk

# Test WASM compilation
cd examples/todo-app
trunk build
```

## 📋 **Issue Categories**

### **Low Hanging Fruit (Good for Beginners)**
- [ ] Fix variable naming in `persistence.rs`
- [ ] Fix variable naming in `visualization.rs`
- [ ] Add missing imports in various files
- [ ] Update documentation comments

### **Medium Complexity (Intermediate Rust)**
- [ ] Fix serde trait bounds for generic types
- [ ] Resolve extension trait bound mismatches
- [ ] Implement proper conditional compilation
- [ ] Fix async store type inference

### **High Complexity (Advanced Rust)**
- [ ] Refactor type system architecture
- [ ] Implement comprehensive feature flag system
- [ ] Create type-safe builder patterns
- [ ] Optimize compilation performance

## 🎨 **Code Style Guidelines**

### **Rust Conventions**
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use meaningful variable and function names
- Add comprehensive documentation comments
- Write tests for new functionality

### **Error Handling**
- Use `Result<T, E>` for fallible operations
- Provide helpful error messages
- Implement proper error recovery where possible
- Add context to error chains

### **Performance**
- Avoid unnecessary allocations
- Use appropriate data structures
- Profile performance-critical code
- Document performance characteristics

## 🧪 **Testing Guidelines**

### **Unit Tests**
- Test all public functions and methods
- Test edge cases and error conditions
- Use property-based testing where appropriate
- Mock external dependencies

### **Integration Tests**
- Test feature combinations
- Test different compilation targets
- Test WASM compilation and execution
- Test documentation generation

### **Performance Tests**
- Benchmark critical operations
- Test memory usage patterns
- Monitor compilation times
- Track performance regressions

## 📚 **Learning Resources**

### **Rust Fundamentals**
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Reference](https://doc.rust-lang.org/reference/)

### **Advanced Topics**
- [Rust Nomicon](https://doc.rust-lang.org/nomicon/)
- [Asynchronous Programming](https://rust-lang.github.io/async-book/)
- [WebAssembly with Rust](https://rustwasm.github.io/book/)

### **Leptos Framework**
- [Leptos Book](https://leptos-rs.github.io/leptos/)
- [Leptos Examples](https://github.com/leptos-rs/leptos/tree/main/examples)
- [Leptos API Reference](https://docs.rs/leptos/)

## 🚀 **Getting Help**

### **Community Channels**
- **GitHub Issues**: For bug reports and feature requests
- **GitHub Discussions**: For questions and community help
- **Discord**: For real-time chat and support

### **Code Reviews**
- All PRs require code review
- Be open to feedback and suggestions
- Ask questions if something isn't clear
- Help review other contributors' code

### **Mentorship**
- Experienced contributors are happy to help
- Don't hesitate to ask for guidance
- We value all contributions, big and small
- Learning together makes us all better

## 🎉 **Recognition**

### **Contributor Benefits**
- Your name in the project's contributors list
- Experience with modern Rust development
- Understanding of state management systems
- Portfolio piece for future opportunities

### **Types of Contributions**
- **Code**: Bug fixes, features, improvements
- **Documentation**: Guides, examples, API docs
- **Testing**: Test cases, edge case discovery
- **Community**: Helping others, answering questions

## 📝 **Pull Request Process**

### **Before Submitting**
1. Ensure all tests pass locally
2. Check that CI checks would pass
3. Update documentation if needed
4. Add tests for new functionality

### **PR Description Template**
```markdown
## Description
Brief description of what this PR accomplishes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Local tests pass
- [ ] Feature combinations tested
- [ ] WASM compilation tested
- [ ] Documentation builds

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] Tests added/updated
```

## 🔄 **Release Process**

### **Version Bumping**
- Patch version (0.2.1 → 0.2.2): Bug fixes
- Minor version (0.2.1 → 0.3.0): New features, CI fixes
- Major version (0.2.1 → 1.0.0): Breaking changes

### **Release Checklist**
- [ ] All tests pass
- [ ] CI pipeline green
- [ ] Documentation updated
- [ ] Changelog updated
- [ ] Version bumped in Cargo.toml
- [ ] GitHub release created
- [ ] Crate published to crates.io

---

**Thank you for contributing to leptos-state!** 🎉

Your contributions help make this library better for the entire Rust and Leptos community.
