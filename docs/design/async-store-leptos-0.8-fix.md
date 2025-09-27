# 🔧 Async Store Leptos 0.8.9 API Fix Design

## Problem
The async store implementation uses `create_resource` API that doesn't exist in Leptos 0.8.9, causing compilation failures.

## Current Issue
```rust
// This fails in Leptos 0.8.9
let resource = create_resource(
    move || input_signal.get(),
    move |input| async move {
        // async loading logic
    }
);
```

## Solution Design

### Option 1: Use `create_resource_with_initial_value` (Recommended)
```rust
// Leptos 0.8.9 correct API
let resource = create_resource_with_initial_value(
    move || input_signal.get(),
    move |input| async move {
        if let Some(next_input) = I::next_page_input(&state.get()) {
            set_input_signal.set(next_input);
            I::load_page(input).await
        } else {
            Ok(Default::default())
        }
    },
    || I::Page::default() // Initial value
);
```

### Option 2: Use `create_local_resource` for component-local async
```rust
// For component-scoped async operations
let resource = create_local_resource(
    move || input_signal.get(),
    move |input| async move {
        I::load_page(input).await
    }
);
```

### Option 3: Use `create_resource` with correct signature
```rust
// If create_resource exists, use correct pattern
let resource = create_resource(
    || input_signal.get(), // No move closure needed
    |input| async move {
        I::load_page(input).await
    }
);
```

## Implementation Strategy

### Phase 1: API Discovery
1. **Research Leptos 0.8.9 resource APIs** - Determine exact function signatures
2. **Test compilation** - Verify which API works
3. **Update imports** - Ensure correct imports are in place

### Phase 2: Code Migration
1. **Update function calls** - Replace `create_resource` calls
2. **Fix trait bounds** - Add `Send` bounds for async closure parameters
3. **Update error handling** - Handle new resource error types

### Phase 3: Testing
1. **Unit tests** - Test resource creation and loading
2. **Integration tests** - Test with actual async operations
3. **Error tests** - Test error handling paths

## Risk Assessment

**High Risk:**
- API may have changed significantly between versions
- Async trait bounds may need adjustment

**Mitigation:**
- Use feature flags to maintain compatibility
- Add comprehensive error handling
- Test thoroughly before committing

## Success Criteria

- [ ] Code compiles without `create_resource` errors
- [ ] Async store functionality works correctly
- [ ] No breaking changes to existing API
- [ ] All tests pass with new implementation

## Files to Modify

- `leptos-state/src/store/async_store.rs` - Main implementation
- `leptos-state/src/hooks/use_store.rs` - Hook integration
- `tests/rust/integration/async_tests.rs` - Test updates
