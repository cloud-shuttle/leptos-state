# 🤝 Contributing to leptos-state

Thank you for your interest in contributing to `leptos-state`! This guide will help you get started with contributing to the project.

## 🎯 **How to Contribute**

### **Types of Contributions**
- 🐛 **Bug Reports** - Help us identify and fix issues
- 💡 **Feature Requests** - Suggest new functionality
- 📝 **Documentation** - Improve guides and examples
- 🧪 **Testing** - Add test coverage and benchmarks
- 🚀 **Code** - Implement features and fix bugs
- 🔍 **Code Review** - Review pull requests from others

## 🚀 **Getting Started**

### **Prerequisites**
- **Rust 1.70+** - Latest stable Rust toolchain
- **Git** - Version control system
- **GitHub account** - For issues and pull requests

### **Development Setup**
```bash
# Clone the repository
git clone https://github.com/cloud-shuttle/leptos-state.git
cd leptos-state

# Install dependencies
cargo build

# Run tests to ensure everything works
cargo test --features "testing,persist,devtools"

# Run benchmarks
cargo bench --features "testing,persist"
```

### **IDE Setup**
We recommend using **VS Code** with the **rust-analyzer** extension for the best development experience.

## 🔧 **Development Workflow**

### **1. Create an Issue**
Before starting work, create an issue to discuss your contribution:
- **Bug reports**: Include steps to reproduce, expected vs actual behavior
- **Feature requests**: Describe the use case and proposed solution
- **Documentation**: Specify what needs improvement

### **2. Fork and Clone**
```bash
# Fork the repository on GitHub
# Then clone your fork
git clone https://github.com/YOUR_USERNAME/leptos-state.git
cd leptos-state

# Add the upstream remote
git remote add upstream https://github.com/cloud-shuttle/leptos-state.git
```

### **3. Create a Branch**
```bash
# Create a feature branch
git checkout -b feature/your-feature-name

# Or for bug fixes
git checkout -b fix/issue-description
```

### **4. Make Changes**
- Write your code following our [coding standards](#coding-standards)
- Add tests for new functionality
- Update documentation as needed
- Ensure all tests pass

### **5. Test Your Changes**
```bash
# Run all tests
cargo test --features "testing,persist,devtools"

# Run specific test modules
cargo test --package leptos-state --lib

# Run benchmarks
cargo bench --features "testing,persist"

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings
```

### **6. Commit Your Changes**
```bash
# Stage your changes
git add .

# Commit with a descriptive message
git commit -m "feat: add new state machine validation

- Add validation for state transitions
- Include comprehensive test coverage
- Update documentation with examples

Closes #123"
```

### **7. Push and Create PR**
```bash
# Push your branch
git push origin feature/your-feature-name

# Create a pull request on GitHub
# Include a description of your changes and reference related issues
```

## 📝 **Coding Standards**

### **Rust Code Style**
- Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)
- Use `cargo fmt` to format code automatically
- Run `cargo clippy` to catch common issues

### **Code Organization**
```rust
// Module structure
pub mod module_name {
    // Re-exports first
    pub use super::other_module::*;
    
    // Types and traits
    pub struct MyStruct {
        // Fields
    }
    
    impl MyStruct {
        // Methods
    }
    
    // Tests at the bottom
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn test_my_struct() {
            // Test implementation
        }
    }
}
```

### **Documentation**
- Document all public APIs with doc comments
- Include examples in documentation
- Use proper markdown formatting

```rust
/// A state machine that manages application state.
///
/// # Examples
///
/// ```rust
/// use leptos_state::v1::*;
///
/// let machine = Machine::new(InitialState, Context::default());
/// machine.send(Event::Start)?;
/// ```
pub struct Machine<C, E, S> {
    // Implementation
}
```

### **Error Handling**
- Use custom error types for domain-specific errors
- Provide meaningful error messages
- Include context information when possible

```rust
#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("Invalid state transition from {from:?} to {to:?}")]
    InvalidTransition { from: State, to: State },
    
    #[error("Context validation failed: {reason}")]
    ContextError { reason: String },
}
```

### **Testing**
- Write unit tests for all public functions
- Use property-based testing for complex logic
- Include integration tests for major features
- Aim for high test coverage

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    #[test]
    fn test_basic_functionality() {
        // Basic test
    }
    
    proptest! {
        #[test]
        fn test_property_based(input: u32) {
            // Property-based test
        }
    }
}
```

## 🧪 **Testing Guidelines**

### **Test Structure**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Test data setup
    fn create_test_data() -> TestData {
        TestData::default()
    }
    
    // Individual test cases
    #[test]
    fn test_specific_behavior() {
        let data = create_test_data();
        let result = function_under_test(data);
        assert!(result.is_ok());
    }
    
    // Edge cases
    #[test]
    fn test_edge_cases() {
        // Test boundary conditions
    }
    
    // Error conditions
    #[test]
    fn test_error_conditions() {
        let result = function_under_test(InvalidData);
        assert!(result.is_err());
    }
}
```

### **Property-Based Testing**
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_state_machine_properties(
        events in prop::collection::vec(any::<Event>(), 0..100)
    ) {
        let machine = create_test_machine();
        let mut state = machine.initial_state();
        
        for event in events {
            if machine.can_transition(&state, event.clone()) {
                state = machine.transition(&state, event);
                assert!(machine.is_valid_state(&state));
            }
        }
    }
}
```

### **Benchmarking**
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_function(c: &mut Criterion) {
    c.bench_function("function_name", |b| {
        b.iter(|| {
            let input = black_box(create_test_input());
            function_under_test(input)
        });
    });
}

criterion_group!(benches, benchmark_function);
criterion_main!(benches);
```

## 📚 **Documentation Guidelines**

### **User Documentation**
- Write for developers using the library
- Include practical examples
- Explain concepts clearly
- Use consistent formatting

### **API Documentation**
- Document all public functions, types, and traits
- Include parameter descriptions
- Provide return value information
- Add usage examples

### **Code Comments**
- Explain complex logic
- Document design decisions
- Include references to specifications
- Keep comments up to date

## 🔍 **Code Review Process**

### **Before Submitting**
- [ ] All tests pass
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation is updated
- [ ] Commit messages are clear

### **Review Checklist**
- [ ] Code follows project standards
- [ ] Tests cover new functionality
- [ ] Documentation is accurate
- [ ] Performance considerations addressed
- [ ] Security implications considered

### **Review Comments**
- Be constructive and specific
- Suggest improvements clearly
- Reference relevant documentation
- Ask questions when unclear

## 🚀 **Release Process**

### **Version Bumping**
- **Patch** (0.0.X): Bug fixes and minor improvements
- **Minor** (0.X.0): New features, backward compatible
- **Major** (X.0.0): Breaking changes

### **Release Checklist**
- [ ] All tests pass
- [ ] Documentation is updated
- [ ] Changelog is current
- [ ] Version numbers are updated
- [ ] Release notes are prepared

## 🐛 **Bug Reports**

### **Bug Report Template**
```markdown
**Describe the bug**
A clear description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Go to '...'
2. Click on '....'
3. Scroll down to '....'
4. See error

**Expected behavior**
A clear description of what you expected to happen.

**Actual behavior**
A clear description of what actually happened.

**Environment:**
- OS: [e.g. macOS, Windows, Linux]
- Rust version: [e.g. 1.70.0]
- leptos-state version: [e.g. 1.0.0-beta.1]
- Browser: [e.g. Chrome, Firefox, Safari]

**Additional context**
Add any other context about the problem here.
```

## 💡 **Feature Requests**

### **Feature Request Template**
```markdown
**Is your feature request related to a problem?**
A clear description of what the problem is.

**Describe the solution you'd like**
A clear description of what you want to happen.

**Describe alternatives you've considered**
A clear description of any alternative solutions.

**Additional context**
Add any other context or screenshots about the feature request.
```

## 🤝 **Community Guidelines**

### **Be Respectful**
- Treat all contributors with respect
- Use inclusive language
- Be patient with newcomers
- Provide constructive feedback

### **Communication**
- Use clear, concise language
- Ask questions when unsure
- Share knowledge and help others
- Be open to different approaches

### **Collaboration**
- Work together on solutions
- Share ideas and suggestions
- Help review others' work
- Celebrate contributions

## 📞 **Getting Help**

### **Resources**
- **[Documentation](docs/)** - Comprehensive guides and references
- **[Examples](examples/)** - Working code samples
- **[Issues](https://github.com/cloud-shuttle/leptos-state/issues)** - Search existing issues
- **[Discussions](https://github.com/cloud-shuttle/leptos-state/discussions)** - Ask questions

### **Contact**
- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and ideas
- **Discord**: For real-time chat (link in README)

## 🎉 **Recognition**

### **Contributor Recognition**
- All contributors are listed in the README
- Significant contributions are highlighted
- Contributors are mentioned in release notes
- Special recognition for major features

### **Contributor Types**
- **Code Contributors**: Write and review code
- **Documentation Contributors**: Improve guides and examples
- **Test Contributors**: Add test coverage and benchmarks
- **Community Contributors**: Help others and provide feedback

---

**Thank you for contributing to leptos-state! Your contributions help make this library better for everyone.** 🚀

*Questions? Feel free to open an issue or start a discussion on GitHub.*
