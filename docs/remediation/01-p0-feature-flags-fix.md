# P0: Feature Flags System Fix

**Priority**: P0 (Production Blocker)  
**Timeline**: 2 days  
**Assignee**: TBD

## Problem Statement

Multiple features are referenced in code but not declared in `Cargo.toml`, causing:
- Conditional compilation failures
- CI pipeline false positives  
- Developer confusion about available functionality

## Referenced but Missing Features
- `visualization`
- `integration` 
- `documentation`
- `codegen`
- `performance`
- `testing`
- `persist`
- `wasm`

## Solution Design

### Step 1: Declare Features in Cargo.toml
```toml
[features]
default = []
full = [
    "persist", "devtools", "testing", "codegen", 
    "wasm", "visualization", "integration", 
    "documentation", "performance"
]

# Core features
persist = ["serde", "parking_lot"]
devtools = ["serde_json"]
testing = ["quickcheck"]

# Advanced features  
codegen = ["quote", "syn"]
wasm = ["wasm-bindgen", "js-sys", "web-sys"]
visualization = ["plotters"]
integration = ["tokio"]
documentation = ["pulldown-cmark"]
performance = ["criterion"]
```

### Step 2: Update CI Matrix
```yaml
strategy:
  matrix:
    features:
      - ""  # default
      - "persist"
      - "devtools"  
      - "full"
```

### Step 3: Feature Gate Validation
- Add `#[cfg(feature = "...")]` guards around unimplemented modules
- Use `#[cfg(not(feature = "..."))]` with compile_error! for clarity

## Acceptance Criteria

- [ ] `cargo check --all-features` passes
- [ ] `cargo build --no-default-features` passes  
- [ ] CI tests each feature combination
- [ ] All feature-gated code compiles or shows clear errors

## Implementation Notes

- Start with minimal feature declarations
- Gradually enable more complex features as implementations complete
- Use feature flags to hide incomplete functionality rather than stub it

## Dependencies

None - this is foundational work

## Risks

- May reveal additional compilation issues in feature-gated code
- Could expose more stub implementations that need fixing
