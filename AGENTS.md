# AGENTS.md - AI Coding Agent Guidelines

## Build/Test Commands
- **Build**: `cargo build --workspace` (or `make build`)
- **Test all**: `cargo test --workspace` (or `make test`)  
- **Test single**: `cargo test [test_name] --package leptos-state`
- **Rust unit tests**: `pnpm run test:unit` or `cargo test`
- **Integration tests**: `pnpm run test:integration` or `cargo test --test integration`
- **Playwright web tests**: `pnpm test:web` (requires `make build-web` first)
- **Lint**: `cargo clippy --workspace -- -D warnings` (or `make lint`)
- **Format**: `cargo fmt --workspace` (or `make format`)

## Architecture
- **Workspace**: Root has leptos-state library + examples + integration tests
- **Core library**: `leptos-state/src/` with modules: machine, store, hooks, utils, v1
- **State machines**: XState-inspired with guards, actions, nested states
- **Reactive stores**: Zustand-inspired with Leptos signal integration
- **Persistence**: Multiple backends (LocalStorage, Memory, IndexedDB)
- **Features**: `persist`, `devtools`, `testing`, `codegen`, `wasm` (use sparingly)

## Code Style
- **Rust edition**: 2024, MSRV 1.89+
- **Format**: rustfmt with 100 char width, 4 spaces, imports_granularity = "Crate"
- **Clippy**: Strict (deny pedantic/nursery/cargo), allow too_many_arguments/needless_pass_by_value
- **Traits**: Implement MachineState/MachineEvent for state machines, StoreState for stores
- **Naming**: snake_case for functions/vars, PascalCase for types, SCREAMING_SNAKE for constants
- **Error handling**: Use thiserror, return Result<T, E>, avoid unwrap() in production code
- **Imports**: Group by std/external/crate, prefer explicit imports over glob imports

## Notes
- Use workspace dependencies from Cargo.toml root
- Test with both `cargo test` and `pnpm test:web` for full coverage
- Examples in `examples/` directory demonstrate usage patterns
