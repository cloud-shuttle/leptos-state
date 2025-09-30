# 📁 File Refactoring Plan - September 20, 2025

## Executive Summary

**Current Status**: 53 files exceed 300-line limit (target: ≤300 lines)
**Goal**: Break down oversized files into maintainable, focused modules
**Timeline**: 2-3 days for complete refactoring
**Success Criteria**: All files ≤300 lines, improved maintainability

## Current File Size Analysis

### Critical Violations (>400 lines)
| File | Lines | Priority | Refactor Strategy |
|------|-------|----------|-------------------|
| `machine.rs` | 1,323 | 🔴 CRITICAL | Split into 6 modules |
| `machine/testing.rs` | 1,182 | 🔴 CRITICAL | Split into 5 modules |
| `machine/persistence.rs` | 1,100 | 🔴 CRITICAL | Split into 6 modules |

### High Priority (>350 lines)
| File | Lines | Refactor Strategy |
|------|-------|-------------------|
| `utils/collections.rs` | 610 | 3 focused modules |
| `machine/persistence_storage.rs` | 608 | 2 modules |
| `machine/codegen_types.rs` | 587 | 2 modules |
| `machine/integration_events.rs` | 579 | 3 modules |

### Medium Priority (300-350 lines)
| File | Lines | Refactor Strategy |
|------|-------|-------------------|
| 41 additional files | 300-350 | 2 modules each |

## Refactoring Strategy

### Phase 1: Critical Files (Days 1-2)

#### 1. `machine.rs` → 6 Modules (1,323 lines → ~220 lines each)

**Current Structure**: Monolithic state machine implementation
**Target Structure**:

```
machine/
├── core/           # Core state machine logic (220 lines)
├── transitions/    # Transition handling (220 lines)
├── states/         # State management (220 lines)
├── execution/      # Action/guard execution (220 lines)
├── validation/     # State machine validation (220 lines)
├── mod.rs          # Public API exports (223 lines)
```

**Migration Plan**:
1. Extract `transition_*` methods → `transitions.rs`
2. Extract state management → `states.rs`
3. Extract action/guard logic → `execution.rs`
4. Extract validation logic → `validation.rs`
5. Keep core types in `core.rs`
6. Update all imports incrementally

#### 2. `machine/testing.rs` → 5 Modules (1,182 lines → ~236 lines each)

**Target Structure**:
```
machine/testing/
├── test_cases.rs       # Test case definitions
├── test_runner.rs      # Test execution logic
├── property_tests.rs   # Property-based testing
├── mock_machines.rs    # Test machine factories
├── test_utils.rs       # Testing utilities
```

#### 3. `machine/persistence.rs` → 6 Modules (1,100 lines → ~183 lines each)

**Target Structure**:
```
machine/persistence/
├── core.rs             # Main persistence logic
├── storage.rs          # Storage backends
├── serialization.rs    # Data serialization
├── metadata.rs         # Persistence metadata
├── versioning.rs       # Version management
├── recovery.rs         # Error recovery
```

### Phase 2: High Priority Files (Day 3)

#### Collections Utils (610 lines → 3 modules)
```
utils/
├── collections/
│   ├── core.rs         # Core collection types
│   ├── algorithms.rs   # Collection algorithms
│   └── mod.rs          # Exports
```

#### Integration Events (579 lines → 3 modules)
```
machine/integration/
├── events.rs           # Event definitions
├── handlers.rs         # Event handlers
├── mod.rs              # Integration API
```

### Phase 3: Medium Priority Files (Days 4-5)

**Strategy**: Extract cohesive functionality into focused modules
- Group related functions together
- Maintain clear module boundaries
- Preserve existing APIs through re-exports

## Implementation Guidelines

### 1. Module Extraction Rules

#### ✅ DO Extract When:
- **Single Responsibility**: Module has one clear purpose
- **High Cohesion**: Functions work together closely
- **Size Threshold**: >200 lines of focused code
- **Dependency Clarity**: Clear import/export boundaries

#### ❌ DON'T Extract When:
- **Tight Coupling**: Functions share complex state
- **Performance Critical**: Extraction would hurt performance
- **API Stability**: Would break public interfaces significantly

### 2. Import Management

#### Before Extraction:
```rust
// Single large file with all imports
use std::collections::HashMap;
use crate::machine::types::*;
use crate::utils::*;
// ... 50+ imports
```

#### After Extraction:
```rust
// Focused imports per module
mod transitions;
mod states;
mod execution;

pub use transitions::*;
pub use states::*;
pub use execution::*;
```

### 3. Testing Strategy

#### Pre-Refactoring:
```bash
cargo test --workspace  # Test everything
```

#### During Refactoring:
```bash
# Test after each module extraction
cargo test machine::transitions
cargo test machine::states
cargo test machine::execution
```

#### Post-Refactoring:
```bash
cargo test --workspace  # Ensure no regressions
```

## Risk Mitigation

### High-Risk Areas

#### 1. Import Chain Breaks
- **Risk**: Refactoring breaks module visibility
- **Mitigation**: Use `pub use` for backward compatibility
- **Validation**: `cargo check` after each change

#### 2. Circular Dependencies
- **Risk**: Poor module boundaries create cycles
- **Mitigation**: Plan dependencies upfront, use dependency graphs
- **Validation**: Compiler will catch cycles

#### 3. Performance Impact
- **Risk**: Module boundaries affect inlining/function call overhead
- **Mitigation**: Keep hot paths together, measure performance
- **Validation**: Benchmark before/after refactoring

### Rollback Plan

#### If Refactoring Fails:
1. **Immediate**: Stop current extraction
2. **Assessment**: Identify what broke and why
3. **Decision**: Fix forward vs rollback
4. **Communication**: Document lessons learned

## Success Metrics

### Quantitative Goals
- [ ] **0 files >300 lines** (currently 53)
- [ ] **<10 files >250 lines** (maintainable threshold)
- [ ] **Average file size <200 lines**
- [ ] **All tests pass** after refactoring

### Qualitative Goals
- [ ] **Clear module boundaries** - single responsibility principle
- [ ] **Reduced cognitive load** - easier to understand individual files
- [ ] **Improved maintainability** - focused, cohesive modules
- [ ] **Preserved functionality** - no behavioral changes

## Tools and Automation

### File Size Monitoring
```bash
# Check current status
find leptos-state/src -name "*.rs" -exec wc -l {} \; | sort -nr | head -10

# Automated size checking
#!/bin/bash
MAX_LINES=300
for file in $(find src -name "*.rs"); do
    lines=$(wc -l < "$file")
    if [ "$lines" -gt "$MAX_LINES" ]; then
        echo "ERROR: $file has $lines lines (max $MAX_LINES)"
        exit 1
    fi
done
```

### Module Dependency Analysis
```bash
# Check for circular dependencies
cargo check --workspace

# Generate dependency graph
cargo modules generate graph --package leptos-state
```

## Timeline and Milestones

### Day 1: Critical Infrastructure
- [ ] Extract `machine.rs` into 6 modules
- [ ] Validate compilation after each extraction
- [ ] Update imports and re-exports

### Day 2: Testing and Persistence
- [ ] Extract `testing.rs` into 5 modules
- [ ] Extract `persistence.rs` into 6 modules
- [ ] Full test suite validation

### Day 3: High Priority Utils
- [ ] Collections utilities (3 modules)
- [ ] Integration events (3 modules)
- [ ] Code generation cleanup

### Days 4-5: Medium Priority Cleanup
- [ ] Extract remaining oversized files
- [ ] Final size validation
- [ ] Performance regression testing

## Communication Plan

### Internal Coordination
- **Daily Standups**: Progress updates, blocker resolution
- **Code Reviews**: All refactoring changes reviewed
- **Documentation**: Update module documentation

### External Communication
- **Status Updates**: Weekly progress reports
- **API Changes**: Document any breaking changes
- **Migration Guide**: Help users adapt if needed

---

*File refactoring plan created September 20, 2025 - Targeting zero oversized files*
