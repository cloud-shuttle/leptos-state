# Contributing to Leptos State

Thank you for your interest in contributing to the Leptos State Management Library! This document provides guidelines and information for contributors.

## 🤝 How to Contribute

### Reporting Issues

Before creating a new issue, please:

1. **Search existing issues** to see if your problem has already been reported
2. **Check the documentation** to ensure you're using the library correctly
3. **Provide a minimal reproduction** when reporting bugs

When creating an issue, please include:

- **Description**: Clear explanation of the problem
- **Reproduction**: Steps to reproduce the issue
- **Expected behavior**: What you expected to happen
- **Actual behavior**: What actually happened
- **Environment**: Rust version, Leptos version, OS, etc.
- **Code example**: Minimal code that demonstrates the issue

### Suggesting Features

We welcome feature suggestions! When proposing a new feature:

1. **Describe the use case** and why it's needed
2. **Provide examples** of how it would be used
3. **Consider the API design** and how it fits with existing patterns
4. **Discuss alternatives** you've considered

### Pull Requests

Before submitting a pull request:

1. **Fork the repository** and create a feature branch
2. **Write tests** for new functionality
3. **Update documentation** for any API changes
4. **Follow the coding style** (see below)
5. **Ensure all tests pass** locally

## 🛠️ Development Setup

### Prerequisites

- Rust 1.70+ 
- `wasm-pack` for WASM examples
- `cargo-watch` (optional, for development)

### Getting Started

```bash
# Clone the repository
git clone git@github.com:cloud-shuttle/leptos-state.git
cd leptos-state

# Install dependencies
cargo build

# Run tests
cargo test

# Run examples
cd examples/todo-app
wasm-pack build --target web
python3 -m http.server 8080
```

### Development Workflow

```bash
# Run tests in watch mode
cargo watch -x test

# Check formatting
cargo fmt

# Run lints
cargo clippy

# Run all checks
cargo check --all-targets --all-features
```

## 📝 Coding Standards

### Rust Code Style

- Follow [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)
- Use `cargo fmt` to format code
- Use `cargo clippy` to catch common issues
- Write meaningful commit messages

### Documentation

- Document all public APIs
- Include examples in doc comments
- Update README.md for user-facing changes
- Add tests for documentation examples

### Testing

- Write unit tests for all new functionality
- Include integration tests for complex features
- Test both native and WASM targets
- Aim for high test coverage

## 🏗️ Project Structure

```
leptos-state/
├── src/                    # Core library source
│   ├── machine/           # State machine implementation
│   ├── store/             # Store management
│   ├── utils/             # Utilities and helpers
│   └── lib.rs             # Library entry point
├── examples/              # Example applications
│   ├── todo-app/          # Todo application
│   ├── analytics-dashboard/ # Analytics dashboard
│   └── traffic-light/     # Traffic light state machine
├── tests/                 # Integration tests
├── docs/                  # Documentation
└── benches/               # Benchmarks
```

## 🚀 Release Process

### Versioning

We follow [Semantic Versioning](https://semver.org/):

- **Major**: Breaking changes
- **Minor**: New features (backward compatible)
- **Patch**: Bug fixes (backward compatible)

### Release Checklist

Before releasing:

- [ ] All tests pass
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated
- [ ] Version is bumped in Cargo.toml
- [ ] Examples are tested
- [ ] WASM builds work
- [ ] Performance benchmarks are run

## 🐛 Bug Reports

When reporting bugs, please include:

```rust
// Minimal reproduction example
use leptos::*;
use leptos_state::*;

#[component]
fn BugExample() -> impl IntoView {
    // Your code here
    view! { <div>"Hello"</div> }
}
```

## 💡 Feature Requests

When requesting features, please describe:

1. **Problem**: What problem does this solve?
2. **Solution**: How would you like it to work?
3. **Alternatives**: What have you tried?
4. **Use Cases**: Who would benefit from this?

## 📞 Getting Help

- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Documentation**: Check the README and API docs first

## 🙏 Recognition

Contributors will be recognized in:

- The README.md file
- Release notes
- The project's GitHub contributors page

Thank you for contributing to Leptos State! 🦀