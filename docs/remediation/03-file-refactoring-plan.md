# File Refactoring Plan - Large File Breakdown

**Priority**: P2 (Technical Debt)  
**Timeline**: 3 weeks  
**Target**: All files under 300 lines

## Current File Size Violations

| File | Lines | Priority | Complexity |
|------|-------|----------|------------|
| `machine/core.rs` | 1,411 | P0 | Very High |
| `machine/testing.rs` | 1,180 | P1 | High |
| `machine/performance.rs` | 1,060 | P2 | Medium |
| `machine/persistence.rs` | 1,053 | P1 | High |  
| `machine/documentation.rs` | 1,032 | P2 | Low |
| `version.rs` | 976 | P2 | Low |
| `machine/actions.rs` | 939 | P1 | Medium |
| `machine/visualization.rs` | 934 | P2 | Low |
| `machine/guards.rs` | 924 | P1 | Medium |

## Refactoring Strategy

### Phase 1: Critical Core (Week 1)

#### `machine/core.rs` (1,411 → 4 files @ ~300 each)

**Split Plan**:
```
machine/
├── core/
│   ├── mod.rs           # Public API, re-exports (50 lines)
│   ├── builder.rs       # Machine builder pattern (280 lines)  
│   ├── runtime.rs       # State transitions, execution (290 lines)
│   ├── hierarchical.rs  # Nested/parallel states (285 lines)
│   └── algorithms.rs    # State resolution, optimization (280 lines)
```

**Migration Steps**:
1. Create new module structure
2. Move builder-related code to `builder.rs` 
3. Move runtime execution to `runtime.rs`
4. Move hierarchical state logic to `hierarchical.rs`
5. Move algorithms and utilities to `algorithms.rs`
6. Update imports across codebase

### Phase 2: High-Impact Files (Week 2)

#### `machine/testing.rs` (1,180 → 4 files @ ~295 each)
```
machine/testing/
├── mod.rs           # Main testing traits and utilities
├── fixtures.rs      # Test data generation and fixtures
├── assertions.rs    # Custom test assertions and matchers  
├── integration.rs   # Integration test helpers
```

#### `machine/persistence.rs` (1,053 → 4 files @ ~263 each)  
```
machine/persistence/
├── mod.rs           # Persistence traits and API
├── backends/        # Storage backends
│   ├── memory.rs    # In-memory storage
│   ├── local.rs     # LocalStorage (web)
│   └── indexeddb.rs # IndexedDB (web)
└── serialization.rs # Serde integration
```

### Phase 3: Remaining Files (Week 3)

#### `machine/actions.rs` + `machine/guards.rs` → Combined module
```
machine/conditions/
├── mod.rs          # Public API
├── actions.rs      # Action implementations (~280 lines)
├── guards.rs       # Guard implementations (~280 lines)  
└── combinators.rs  # Action/guard combinators (~200 lines)
```

#### Other large files:
- `machine/performance.rs` → `performance/` module with benchmarks split
- `version.rs` → Most content moved to build script, keep only public API
- Documentation/visualization files → Feature-gated modules

## Implementation Guidelines

### Module Structure Template
```rust
// mod.rs - Public API only
pub use self::builder::*;
pub use self::runtime::*;

mod builder;
mod runtime;  
// ... other modules

// Individual files - single responsibility
```

### Code Movement Rules

1. **Public API**: Keep in `mod.rs`, never in sub-modules
2. **Private utilities**: Move to most appropriate sub-module  
3. **Tests**: Keep with implementation (`#[cfg(test)]` in each file)
4. **Dependencies**: Minimize cross-module imports
5. **Documentation**: Move with code, update links

### Refactoring Checklist per File

- [ ] Identify logical groupings (single responsibility)
- [ ] Create new module structure  
- [ ] Move code maintaining functionality
- [ ] Update all imports and re-exports
- [ ] Run test suite to verify no breakage
- [ ] Update documentation and examples
- [ ] Verify file sizes under 300 lines

## Testing Strategy

### Before Each Refactor:
```bash
cargo test --all-features  # Baseline
cargo clippy --all-features -- -D warnings
```

### After Each Refactor:
```bash
cargo test --all-features  # Verify no regression
cargo build --all-features # Verify compilation
cargo doc --all-features   # Verify documentation
```

### Integration Verification:
- [ ] All examples still compile and run
- [ ] Public API remains unchanged
- [ ] Documentation builds without warnings
- [ ] Performance tests show no regression

## Success Metrics

- [ ] No files over 300 lines
- [ ] Module cohesion improved (single responsibility)  
- [ ] Import complexity reduced
- [ ] Test coverage maintained or improved
- [ ] Build time improved due to better parallelization
- [ ] IDE navigation and analysis improved

## Risk Mitigation

1. **Breaking Changes**: Use `pub use` to maintain API compatibility
2. **Import Hell**: Careful re-export planning in `mod.rs` files
3. **Test Breakage**: Move tests with code, run frequently
4. **Performance**: Measure build times before/after
5. **Review**: Each file refactor needs peer review

## Dependencies

- Requires completion of feature flag fixes (affects conditional compilation)
- May uncover additional stub implementations
- Documentation updates needed after file moves

This refactoring will significantly improve maintainability, testability, and LLM comprehension of the codebase.
