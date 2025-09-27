# AGENTS.md - Leptos State Development Guide

## Build/Test/Lint Commands
- **Build all**: `cargo build --workspace` or `make build`
- **Test all**: `make test` (runs Rust + web tests)
- **Test Rust only**: `cargo test --workspace` or `make test-rust`
- **Test single file**: `cargo test --package leptos-state test_store`
- **Web tests**: `pnpm test:web` or `make test-web`
- **WASM tests**: `pnpm test:wasm` or `make test-wasm`
- **Check/typecheck**: `cargo check --workspace`

## Architecture
- **Core library**: `leptos-state/` - Main state management library with stores, state machines, hooks
- **Examples**: `examples/` - Counter, todo-app, analytics-dashboard, traffic-light demos
- **Tests**: `tests/rust/` (unit/integration) and `tests/web/` (Playwright)
- **Generated code**: `builder_generated/` and `custom_generated/` - Auto-generated state machines
- **Key modules**: store/, machine/, hooks/, utils/, compat/

## Code Style & Conventions
- **Naming**: PascalCase for types/structs, snake_case for functions/fields, SCREAMING_SNAKE_CASE for constants
- **Error handling**: Use `StateResult<T>` and `StateError` types, propagate with `?`
- **Types**: Explicit trait bounds (Clone + PartialEq + Send + Sync + 'static), derive Debug for debugging
- **Imports**: Use absolute imports, group std/external/local, prefer prelude imports (leptos::prelude::*)
- **Testing**: Use rstest for parameterized tests, proptest for property testing, wasm-bindgen-test for WASM
