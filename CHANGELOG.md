# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Leptos 0.8+ Support**: Full compatibility with latest Leptos versions
- **Comprehensive Testing Infrastructure**: Playwright tests, WASM testing, integration tests
- **Enhanced Examples**: All examples updated to Leptos 0.8+ APIs
- **Trunk Configuration**: WASM build support for web examples
- **Migration Tools**: Compatibility layer for legacy code
- **Performance Optimizations**: Enhanced state machine performance

### Changed
- **API Compatibility**: Updated to use latest Leptos 0.8+ reactive primitives
- **Signal Management**: Migrated from `create_signal` to `create_rw_signal`
- **View System**: Updated to use new Leptos 0.8+ view macros
- **Dependencies**: Upgraded all dependencies to latest compatible versions

## [0.2.0] - 2025-09-01

### Added
- **Leptos 0.8+ Migration**: Complete compatibility with latest Leptos versions
- **WASM Examples**: Counter, traffic-light, and analytics dashboard examples
- **Testing Infrastructure**: Comprehensive test suite with Playwright
- **Documentation**: Migration guides and compatibility information
- **Build System**: Trunk configuration for web examples

## [0.1.0] - 2024-12-31

### Added
- Initial release of leptos-state library
- State machine implementation with history states
- Store management with reactive updates
- Guards and conditions system
- Actions and effects framework
- Persistence and serialization
- Visualization and DevTools support
- Testing framework with property-based testing
- Performance optimization features
- Integration patterns for complex applications
- Documentation generator
- Code generation capabilities
- Comprehensive example applications:
  - Todo App with CRUD operations
  - Analytics Dashboard with real-time metrics
  - Traffic Light state machine
- WASM compatibility and optimization
- Type-safe APIs with strong compile-time guarantees
- Middleware system for extensibility
- Time-travel debugging support
- Computed state and selectors
- Hierarchical and parallel state machines

### Changed
- N/A (Initial release)

### Deprecated
- N/A (Initial release)

### Removed
- N/A (Initial release)

### Fixed
- N/A (Initial release)

### Security
- N/A (Initial release)

## [0.1.0] - 2024-12-31

### Added
- Initial release
- Core state management functionality
- Basic examples and documentation

---

## Version History

- **0.2.0**: Leptos 0.8+ compatibility and enhanced testing infrastructure
- **0.1.0**: Initial release with core functionality
- **Unreleased**: Development version with latest features

## Migration Guide

### From 0.1.0 to 0.2.0

**Leptos 0.8+ Migration**: This release includes full compatibility with Leptos 0.8+.

#### Breaking Changes
- **Signal APIs**: Updated to use `create_rw_signal` instead of `create_signal`
- **View Macros**: Updated to use latest Leptos 0.8+ view syntax
- **Dependencies**: Requires Leptos 0.8+ (no longer compatible with 0.6/0.7)

#### Migration Steps
1. Update your `Cargo.toml` to use `leptos = "0.8"`
2. Update signal creation from `create_signal` to `create_rw_signal`
3. Update view macros to use latest Leptos 0.8+ syntax
4. Test your application thoroughly

#### Compatibility Layer
The library includes a compatibility layer to help with migration. See [COMPATIBILITY.md](docs/COMPATIBILITY.md) for detailed information.

### From 0.0.x to 0.1.0

This is the initial release, so no migration is needed.

---

For detailed information about each release, see the [GitHub releases page](https://github.com/cloud-shuttle/leptos-state/releases).
