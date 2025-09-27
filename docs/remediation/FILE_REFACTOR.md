# 📁 File Refactoring - Breaking Down Large Files

## Overview
8 files exceed 300 lines (largest: 1258 lines). Break them into focused, maintainable modules.

## Large Files Requiring Immediate Refactoring

### 1. `machine/machine.rs` (1000 lines) - CRITICAL
**Current Responsibilities:** Machine struct, builders, transitions, hierarchical states
**Complexity Score:** 🔴 CRITICAL

**Breakdown Plan:**
```
machine/
├── core.rs              # Core Machine struct (150 lines)
├── builder/
│   ├── mod.rs          # Builder trait and exports (50 lines)
│   ├── machine.rs      # MachineBuilder (200 lines)
│   ├── state.rs        # StateBuilder (150 lines)
│   └── transition.rs   # TransitionBuilder (100 lines)
├── hierarchical.rs     # Child states and nested logic (200 lines)
└── types.rs           # Common types and enums (100 lines)
```

**Migration Steps:**
1. Extract core Machine struct and basic methods
2. Move builders to separate module hierarchy
3. Extract hierarchical state logic
4. Update imports across codebase

### 2. `machine/documentation.rs` (1000 lines) - HIGH
**Current Responsibilities:** Multiple documentation formats, templates, file I/O
**Complexity Score:** 🟡 HIGH

**Breakdown Plan:**
```
machine/documentation/
├── mod.rs              # Main DocumentationGenerator (100 lines)
├── config.rs           # DocumentationConfig types (100 lines)
├── generators/
│   ├── mod.rs         # Generator trait (50 lines)
│   ├── markdown.rs    # Markdown generation (200 lines)
│   ├── html.rs        # HTML generation (200 lines)
│   ├── pdf.rs         # PDF generation (150 lines)
│   └── asciidoc.rs    # AsciiDoc generation (100 lines)
├── templates/
│   ├── mod.rs         # Template management (50 lines)
│   └── engine.rs      # Template rendering (100 lines)
└── export.rs          # File I/O and export logic (100 lines)
```

### 3. `machine/persistence.rs` (1000 lines) - HIGH
**Current Responsibilities:** Serialization, storage backends, backup, encryption
**Complexity Score:** 🟡 HIGH

**Breakdown Plan:**
```
machine/persistence/
├── mod.rs              # Main persistence API (100 lines)
├── config.rs           # Persistence/Backup configs (150 lines)
├── serialization.rs    # Serialization logic (200 lines)
├── storage/
│   ├── mod.rs         # Storage trait (50 lines)
│   ├── memory.rs      # MemoryStorage (100 lines)
│   ├── local.rs       # LocalStorage (150 lines)
│   └── indexeddb.rs   # IndexedDB storage (100 lines)
├── backup.rs          # Backup management (150 lines)
└── encryption.rs      # Encryption utilities (100 lines)
```

### 4. `machine/testing.rs` (1000 lines) - MEDIUM
**Current Responsibilities:** Test framework, property testing, coverage, performance
**Complexity Score:** 🟡 MEDIUM

**Breakdown Plan:**
```
machine/testing/
├── mod.rs              # Main testing API (100 lines)
├── config.rs           # TestConfig and options (100 lines)
├── runner.rs           # Core test runner (200 lines)
├── coverage.rs         # Coverage tracking (150 lines)
├── performance.rs      # Performance monitoring (150 lines)
├── property/
│   ├── mod.rs         # Property-based testing (100 lines)
│   └── generators.rs  # Test data generators (150 lines)
└── assertions.rs      # Custom test assertions (100 lines)
```

### 5. `machine/performance.rs` (1000 lines) - MEDIUM
**Current Responsibilities:** Profiling, caching, lazy evaluation, optimization
**Complexity Score:** 🟡 MEDIUM

**Breakdown Plan:**
```
machine/performance/
├── mod.rs              # Performance API (100 lines)
├── config.rs           # PerformanceConfig (100 lines)
├── profiler.rs         # Performance profiling (200 lines)
├── cache/
│   ├── mod.rs         # Cache trait and types (50 lines)
│   ├── transition.rs  # Transition caching (150 lines)
│   └── state.rs       # State caching (100 lines)
├── lazy.rs            # Lazy evaluation (150 lines)
├── optimization.rs    # Optimization strategies (150 lines)
└── metrics.rs         # Performance metrics (100 lines)
```

### 6. `machine/visualization.rs` (933 lines) - MEDIUM
**Current Responsibilities:** Visualization, debugging, time travel, export formats
**Complexity Score:** 🟡 MEDIUM

**Breakdown Plan:**
```
machine/visualization/
├── mod.rs              # Main visualization API (100 lines)
├── config.rs           # VisualizationConfig (100 lines)
├── exporters/
│   ├── mod.rs         # Exporter trait (50 lines)
│   ├── dot.rs         # Graphviz DOT export (150 lines)
│   ├── mermaid.rs     # Mermaid export (150 lines)
│   ├── svg.rs         # SVG export (100 lines)
│   └── json.rs        # JSON export (100 lines)
├── debug/
│   ├── mod.rs         # Debug utilities (50 lines)
│   ├── time_travel.rs # Time travel debugging (150 lines)
│   └── monitor.rs     # Real-time monitoring (100 lines)
└── render.rs          # Rendering utilities (100 lines)
```

### 7. `machine/codegen.rs` (829 lines) - MEDIUM
**Current Responsibilities:** Multi-language code generation, templates, file management
**Complexity Score:** 🟡 MEDIUM

**Breakdown Plan:**
```
machine/codegen/
├── mod.rs              # Main codegen API (100 lines)
├── config.rs           # CodeGenConfig (100 lines)
├── generators/
│   ├── mod.rs         # Generator trait (50 lines)
│   ├── rust.rs        # Rust code generation (200 lines)
│   ├── typescript.rs  # TypeScript generation (150 lines)
│   ├── python.rs      # Python generation (100 lines)
│   └── javascript.rs  # JavaScript generation (100 lines)
├── templates/
│   ├── mod.rs         # Template management (50 lines)
│   └── registry.rs    # Template registry (100 lines)
└── output.rs          # File output management (100 lines)
```

### 8. `machine/integration.rs` (685 lines) - LOW
**Current Responsibilities:** External integrations, adapters, event routing
**Complexity Score:** 🟢 LOW

**Breakdown Plan:**
```
machine/integration/
├── mod.rs              # Integration API (100 lines)
├── config.rs           # IntegrationConfig (100 lines)
├── adapters/
│   ├── mod.rs         # Adapter trait (50 lines)
│   ├── http.rs        # HTTP API adapter (150 lines)
│   ├── database.rs    # Database adapter (100 lines)
│   ├── queue.rs       # Message queue adapter (100 lines)
│   └── websocket.rs   # WebSocket adapter (100 lines)
└── manager.rs         # Integration manager (100 lines)
```

## Refactoring Implementation Plan

### Phase 1: Critical Files (Week 1)
**Priority:** machine.rs refactoring
1. Create new module structure
2. Extract core Machine struct
3. Move builders to separate modules
4. Update all imports and tests

### Phase 2: High-Impact Files (Week 2)
**Priority:** documentation.rs and persistence.rs
1. Break down documentation generators
2. Separate storage backends
3. Isolate template rendering

### Phase 3: Supporting Files (Week 3)
**Priority:** testing.rs, performance.rs, visualization.rs
1. Separate test framework components
2. Break down performance modules
3. Split visualization exporters

### Phase 4: Final Files (Week 4)
**Priority:** codegen.rs, integration.rs
1. Separate code generators by language
2. Split integration adapters
3. Final cleanup and testing

## Refactoring Guidelines

### Module Organization Rules
1. **Single Responsibility:** Each file handles one concept
2. **Size Limit:** Max 300 lines per file
3. **Clear Interfaces:** Public API in mod.rs files
4. **Logical Grouping:** Related functionality together

### Code Migration Strategy
```rust
// 1. Create new module structure
// 2. Move code with git mv to preserve history
// 3. Update imports gradually
// 4. Run tests after each move
// 5. Update documentation
```

### Import Update Pattern
```rust
// Before
use crate::machine::machine::{Machine, MachineBuilder};

// After
use crate::machine::{Machine};
use crate::machine::builder::MachineBuilder;
```

## Validation Steps

### For Each Refactored Module
1. **Compilation:** `cargo check --package leptos-state`
2. **Tests:** `cargo test --package leptos-state`
3. **Documentation:** `cargo doc --package leptos-state`
4. **Examples:** Verify examples still work

### File Size Validation
```bash
# Check file sizes after refactoring
find leptos-state/src -name "*.rs" -exec wc -l {} + | sort -n | tail -20
# No file should exceed 300 lines
```

## Benefits Expected

### Immediate Benefits
- ✅ Faster compilation (smaller compilation units)
- ✅ Easier code navigation
- ✅ Better IDE support (goto definition, etc.)
- ✅ Reduced merge conflicts

### Long-term Benefits
- ✅ Easier feature development
- ✅ Better testability
- ✅ Simpler maintenance
- ✅ Team collaboration improvements

## Risk Mitigation

### Potential Issues
1. **Import Hell:** Too many nested modules
2. **API Breakage:** Public interface changes
3. **Test Failures:** Broken test references

### Mitigation Strategies
1. Use `pub use` to maintain public API
2. Gradual migration with feature flags
3. Comprehensive test verification at each step

## Success Metrics

- [ ] No file exceeds 300 lines
- [ ] All tests pass after refactoring
- [ ] Public API remains unchanged
- [ ] Documentation builds successfully
- [ ] Examples continue to work

**Next Steps:** After file refactoring, proceed to STUB_IMPLEMENTATION.md
