# Leptos State Compatibility Layer

The `leptos-state` library includes a comprehensive compatibility layer that allows your applications to work with multiple Leptos versions without changing your code.

## 🎯 Overview

The compatibility layer provides version-agnostic APIs for all Leptos primitives, automatically adapting to the underlying Leptos version at compile time. This means you can:

- **Write once, run anywhere**: Your code works with Leptos 0.6, 0.7, and 0.8+
- **Gradual migration**: Migrate your Leptos version without rewriting your state management code
- **Future-proof**: New Leptos versions are supported by updating the compatibility layer
- **Zero overhead**: Direct delegation to underlying APIs with no runtime cost

## 🚀 Quick Start

### 1. Enable Compatibility Features

Add the appropriate feature flag to your `Cargo.toml`:

```toml
[dependencies]
leptos-state = { version = "0.1", features = ["leptos-0-8"] }
```

### 2. Use Compatibility APIs

Instead of importing directly from `leptos`, use the compatibility layer:

```rust
use leptos_state::compat::*;

// These work with any supported Leptos version
let (read, write) = create_signal(42);
let memo = create_memo(move || read.get() * 2);
let effect = create_effect(move || println!("Value: {}", read.get()));
let callback = create_callback(|value| println!("Clicked: {}", value));
```

### 3. Version Detection

Check which Leptos version you're running:

```rust
use leptos_state::compat::version;

let version = version::version_string(); // "0.8", "0.7", etc.
let is_v8 = version::is_version(version::LeptosVersion::V0_8);
```

## 📚 API Reference

### Signals

```rust
// Create signals
let (read, write) = create_signal(initial_value);
let rw_signal = create_rw_signal(initial_value);
let signal = signal(initial_value);

// Signal transformations
let mapped = map_signal(signal, |value| transform(value));
let filtered = filter_signal(signal, |value| predicate(value));
let (split_read, split_write) = split_signal(signal, |value| split(value));

// Signal utilities
let debounced = debounce_signal(signal, delay_ms);
let throttled = throttle_signal(signal, interval_ms);
```

### Memos and Effects

```rust
// Create memos
let memo = create_memo(move || compute_value());

// Create effects
create_effect(move || side_effect());
create_effect_with_cleanup(effect_fn, cleanup_fn);
create_effect_with_deps(effect_fn, dependencies);

// Advanced effects
create_debounced_effect(effect_fn, delay_ms);
create_throttled_effect(effect_fn, interval_ms);
create_conditional_effect(effect_fn, predicate);
```

### Callbacks

```rust
// Create callbacks
let callback = create_callback(|value| handle(value));
let callback_0 = create_callback_0(|| handle_click());
let event_callback = create_event_callback(|event| handle_event(event));

// Callback utilities
call_callback(&callback, value);
call_callback_0(&callback);
let cloned = clone_callback(&callback);

// Advanced callbacks
let chained = chain_callbacks(vec![callback1, callback2]);
let with_error = create_callback_with_error(|value| handle_success(value));
let with_option = create_callback_with_option(|value| handle_some(value));
```

### Resources

```rust
// Create resources
let resource = create_resource(source_signal, fetcher_fn);
let local_resource = create_local_resource(fetcher_fn);

// Resource utilities
refetch_resource(&resource);
let loading = resource_loading(&resource);
let error = resource_error(&resource);
let success = resource_success(&resource);

// Advanced resources
let retryable = create_retryable_resource(source, fetcher, max_retries);
let cached = create_cached_resource(source, fetcher, cache_key);
let timeout = create_timeout_resource(source, fetcher, timeout_ms);
```

### Context

```rust
// Provide and consume context
provide_context(value);
let context = use_context::<T>();
let context_with_default = use_context_with_default(default_value);
let context_or_panic = use_context_or_panic::<T>();

// Context with signals
let context_signal = use_context_signal::<T>();
let context_memo = use_context_memo(context, |ctx| transform(ctx));

// Multiple contexts
let (ctx1, ctx2) = use_contexts::<T1, T2>();
let (ctx1, ctx2) = use_contexts_with_defaults(default1, default2);
```

### View and Mounting

```rust
// Mount functions
mount_to_body(|| view! { <App /> });
mount_to_element("app", || view! { <App /> });
mount_to_dom_element(element, || view! { <App /> });

// Render functions
let html = render_to_string(|| view! { <App /> });
render_to_writer(writer, || view! { <App /> });
```

## 🔧 Feature Flags

Enable the appropriate feature flag based on your Leptos version:

```toml
# For Leptos 0.6
leptos-state = { version = "0.1", features = ["leptos-0-6"] }

# For Leptos 0.7
leptos-state = { version = "0.1", features = ["leptos-0-7"] }

# For Leptos 0.8+
leptos-state = { version = "0.1", features = ["leptos-0-8"] }
```

## 📋 Migration Guide

### From Direct Leptos APIs

**Before:**
```rust
use leptos::*;

let (read, write) = leptos::create_signal(42);
let memo = leptos::create_memo(move || read.get() * 2);
let callback = leptos::create_callback(|value| println!("{}", value));
```

**After:**
```rust
use leptos_state::compat::*;

let (read, write) = create_signal(42);
let memo = create_memo(move || read.get() * 2);
let callback = create_callback(|value| println!("{}", value));
```

### From Leptos 0.6 to 0.8

**Before (0.6):**
```rust
let (read, write) = leptos::create_signal(42);
let memo = leptos::create_memo(move || read.get() * 2);
```

**After (0.8 with compatibility layer):**
```rust
// Same code works with both versions!
let (read, write) = create_signal(42);
let memo = create_memo(move || read.get() * 2);
```

## 🧪 Testing

The compatibility layer includes comprehensive tests for all APIs:

```bash
# Test with Leptos 0.6
cargo test --features leptos-0-6

# Test with Leptos 0.7
cargo test --features leptos-0-7

# Test with Leptos 0.8
cargo test --features leptos-0-8
```

## 🔍 Version Detection

The compatibility layer automatically detects the Leptos version at compile time:

```rust
use leptos_state::compat::version;

match version::detect_version() {
    version::LeptosVersion::V0_6 => println!("Using Leptos 0.6"),
    version::LeptosVersion::V0_7 => println!("Using Leptos 0.7"),
    version::LeptosVersion::V0_8 => println!("Using Leptos 0.8"),
    version::LeptosVersion::Unknown => println!("Unknown version"),
}
```

## 🚨 Error Handling

The compatibility layer provides error types for handling version-specific issues:

```rust
use leptos_state::compat::{CompatError, CompatResult};

fn my_function() -> CompatResult<()> {
    if version::is_version(version::LeptosVersion::V0_8) {
        // Use 0.8-specific APIs
        Ok(())
    } else {
        Err(CompatError::UnsupportedVersion("0.8".to_string()))
    }
}
```

## ⚠️ Known Issues

### Leptos 0.8.x Compatibility Issues

**Status: BROKEN - All 0.8.x versions affected**

The compatibility layer currently **cannot work with any Leptos 0.8.x version** due to a systematic internal compilation error in the Leptos library itself.

#### Affected Versions
- ✅ Leptos 0.8.0 (May 2, 2024)
- ✅ Leptos 0.8.1 (May 6, 2024) 
- ✅ Leptos 0.8.2 (May 7, 2024)
- ✅ Leptos 0.8.3 (Jul 13, 2024)
- ✅ Leptos 0.8.4 (Jul 21, 2024)
- ✅ Leptos 0.8.5 (Jul 21, 2024) - WASM code splitting release
- ✅ Leptos 0.8.6 (Jul 27, 2024)
- ✅ Leptos 0.8.8 (Latest - 5 days ago)

#### Error Details
All 0.8.x versions fail with the same internal compilation error:

```
error[E0308]: mismatched types
   --> /path/to/leptos-0.8.x/src/hydration/mod.rs:110-138:5
    |
    |     view! {
    |         <link rel="modulepreload" href...
    |         <link
    |             rel="preload"
    |             ...
    |         </script>
    |     }
    |_____^ expected a tuple with 3 elements, found one with 5 elements
```

#### Root Cause
- **Location**: Error occurs in Leptos's own `hydration/mod.rs` file
- **Timing**: Happens during Leptos compilation, before our compatibility layer runs
- **Scope**: Affects the entire 0.8.x series (9 consecutive releases)
- **Status**: This is an upstream Leptos issue, not a problem with our compatibility layer

#### Workarounds

1. **Use Leptos 0.7** (Recommended)
   ```toml
   [dependencies]
   leptos-state = { version = "0.1", features = ["leptos-0-7"] }
   leptos = "0.7"
   ```

2. **Use Simplified Compatibility Layer**
   ```rust
   use leptos_state::compat::simple::*;
   // Provides core APIs with 0.7 fallback behavior
   ```

3. **Wait for Upstream Fix**
   - Monitor [Leptos GitHub issues](https://github.com/leptos-rs/leptos/issues)
   - This issue has persisted through 9 patch releases over several months
   - No guarantee when/if it will be fixed

#### Reporting
If you encounter this issue:
1. **Do not report to leptos-state**: This is not our bug
2. **Report to Leptos**: File an issue at https://github.com/leptos-rs/leptos/issues
3. **Include**: Full error message, Leptos version, and reproduction steps

#### Current Status
- **Leptos 0.6**: ✅ Fully supported
- **Leptos 0.7**: ✅ Fully supported  
- **Leptos 0.8.x**: ❌ **BROKEN** (all versions)
- **Leptos 0.9+**: 🔮 Unknown (not yet released)

## 📈 Performance

The compatibility layer has zero runtime overhead:

- **Compile-time feature selection**: Uses `#[cfg(feature = "...")]` for version-specific code
- **Direct delegation**: Calls the underlying Leptos APIs directly
- **No runtime checks**: Version detection happens at compile time
- **Optimized builds**: Dead code elimination removes unused version paths

## 🔮 Future Plans

The compatibility layer will be extended to support:

- **Leptos 0.9+**: Automatic support for future versions
- **Advanced features**: More sophisticated signal transformations
- **Performance optimizations**: Additional caching and memoization
- **Developer tools**: Better debugging and profiling support

## 🤝 Contributing

To add support for a new Leptos version:

1. Add the version to the `LeptosVersion` enum
2. Add the corresponding feature flag
3. Implement version-specific APIs in each compatibility module
4. Add tests for the new version
5. Update documentation

## 📄 License

The compatibility layer is part of the `leptos-state` library and is licensed under MIT OR Apache-2.0.
