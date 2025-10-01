# Serialization Design

## Overview
Add optional serde-based serialization support to enable state persistence and data interchange while maintaining minimal compilation requirements.

## Current State
```rust
// Current minimal bounds (no serialization)
pub trait State: Send + Sync + Clone + 'static {}
pub trait Event: Send + Sync + Clone + 'static {}
```

## Proposed Enhancement
```rust
// Optional serialization features
#[cfg(feature = "serde")]
pub trait SerializableState: State + Serialize + DeserializeOwned {}
#[cfg(feature = "serde")]
pub trait SerializableEvent: Event + Serialize + DeserializeOwned {}
```

## Motivation

### Data Persistence
- **State Saving**: Persist application state across sessions
- **Data Export**: Export/import state for backup or migration
- **API Integration**: Serialize state for network transmission
- **Development**: Debug state inspection and logging

### Use Cases
- Browser localStorage/sessionStorage persistence
- Server-side state synchronization
- Configuration file storage
- Development debugging and state inspection
- API responses and requests

## Implementation Details

### Optional Dependencies
```toml
[features]
default = []
serde = ["dep:serde", "dep:serde_json"]

[dependencies]
serde = { version = "1.0", features = ["derive"], optional = true }
serde_json = { version = "1.0", optional = true }
```

### Conditional Trait Bounds
```rust
#[cfg(feature = "serde")]
pub trait SerializableState: State + Serialize + DeserializeOwned {}
#[cfg(feature = "serde")]
pub trait SerializableEvent: Event + Serialize + DeserializeOwned {}

// Auto-implement for types with required bounds
#[cfg(feature = "serde")]
impl<T> SerializableState for T
where
    T: State + Serialize + DeserializeOwned {}

#[cfg(feature = "serde")]
impl<T> SerializableEvent for T
where
    T: Event + Serialize + DeserializeOwned {}
```

### Store Integration
```rust
impl<S: State> Store<S> {
    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> Result<String, StoreError>
    where
        S: SerializableState,
    {
        serde_json::to_string(&self.signal.get_untracked())
            .map_err(|e| StoreError::SerializationError(e.to_string()))
    }

    #[cfg(feature = "serde")]
    pub fn from_json(&self, json: &str) -> Result<(), StoreError>
    where
        S: SerializableState,
    {
        let state: S = serde_json::from_str(json)
            .map_err(|e| StoreError::DeserializationError(e.to_string()))?;
        self.set(state)
    }

    #[cfg(feature = "serde")]
    pub fn export_state(&self) -> Result<StateSnapshot<S>, StoreError>
    where
        S: SerializableState,
    {
        Ok(StateSnapshot {
            data: self.signal.get_untracked(),
            timestamp: Utc::now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        })
    }

    #[cfg(feature = "serde")]
    pub fn import_state(&self, snapshot: StateSnapshot<S>) -> Result<(), StoreError>
    where
        S: SerializableState,
    {
        // Version compatibility check
        if snapshot.version != env!("CARGO_PKG_VERSION") {
            return Err(StoreError::VersionMismatch {
                expected: env!("CARGO_PKG_VERSION").to_string(),
                found: snapshot.version,
            });
        }

        self.set(snapshot.data)
    }
}

#[cfg(feature = "serde")]
#[derive(Clone, Serialize, Deserialize)]
pub struct StateSnapshot<S> {
    pub data: S,
    pub timestamp: DateTime<Utc>,
    pub version: String,
}
```

### Machine Integration
```rust
impl<S: State, E: Event> Machine<S, E> {
    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> Result<String, MachineError>
    where
        S: SerializableState,
    {
        let snapshot = MachineSnapshot {
            current_state: self.current_state.clone(),
            context: self.context.clone(),
            timestamp: Utc::now(),
        };

        serde_json::to_string(&snapshot)
            .map_err(|e| MachineError::SerializationError(e.to_string()))
    }

    #[cfg(feature = "serde")]
    pub fn from_json(&self, json: &str) -> Result<(), MachineError>
    where
        S: SerializableState,
    {
        let snapshot: MachineSnapshot<S> = serde_json::from_str(json)
            .map_err(|e| MachineError::DeserializationError(e.to_string()))?;

        self.current_state = snapshot.current_state;
        self.context = snapshot.context;

        Ok(())
    }
}

#[cfg(feature = "serde")]
#[derive(Clone, Serialize, Deserialize)]
pub struct MachineSnapshot<S> {
    pub current_state: String,
    pub context: S,
    pub timestamp: DateTime<Utc>,
}
```

## Serialization Formats

### JSON (Primary)
```rust
#[cfg(feature = "serde")]
pub mod json {
    use super::*;

    pub fn serialize_state<S: SerializableState>(state: &S) -> Result<String, SerializationError> {
        serde_json::to_string_pretty(state)
            .map_err(|e| SerializationError::Json(e.to_string()))
    }

    pub fn deserialize_state<S: SerializableState>(json: &str) -> Result<S, SerializationError> {
        serde_json::from_str(json)
            .map_err(|e| SerializationError::Json(e.to_string()))
    }
}
```

### Binary (Performance)
```rust
#[cfg(feature = "serde")]
pub mod bincode {
    use super::*;

    pub fn serialize_state<S: SerializableState>(state: &S) -> Result<Vec<u8>, SerializationError> {
        bincode::serialize(state)
            .map_err(|e| SerializationError::Bincode(e.to_string()))
    }

    pub fn deserialize_state<S: SerializableState>(data: &[u8]) -> Result<S, SerializationError> {
        bincode::deserialize(data)
            .map_err(|e| SerializationError::Bincode(e.to_string()))
    }
}
```

### MessagePack (Compact)
```rust
#[cfg(feature = "serde")]
pub mod msgpack {
    use super::*;

    pub fn serialize_state<S: SerializableState>(state: &S) -> Result<Vec<u8>, SerializationError> {
        rmp_serde::to_vec(state)
            .map_err(|e| SerializationError::MsgPack(e.to_string()))
    }

    pub fn deserialize_state<S: SerializableState>(data: &[u8]) -> Result<S, SerializationError> {
        rmp_serde::from_slice(data)
            .map_err(|e| SerializationError::MsgPack(e.to_string()))
    }
}
```

## Error Handling

### Serialization Errors
```rust
#[derive(Debug, Clone, thiserror::Error)]
pub enum SerializationError {
    #[error("JSON serialization error: {0}")]
    Json(String),

    #[error("Bincode serialization error: {0}")]
    Bincode(String),

    #[error("MessagePack serialization error: {0}")]
    MsgPack(String),

    #[error("Version mismatch: expected {expected}, found {found}")]
    VersionMismatch { expected: String, found: String },

    #[error("Data corruption detected")]
    DataCorruption,

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
}
```

### Safe Deserialization
```rust
impl<S: SerializableState> Store<S> {
    #[cfg(feature = "serde")]
    pub fn safe_import_state(&self, snapshot: StateSnapshot<S>) -> Result<(), StoreError> {
        // Validate data integrity
        self.validate_snapshot(&snapshot)?;

        // Attempt import with rollback
        let original_state = self.signal.get_untracked();
        match self.import_state(snapshot) {
            Ok(()) => Ok(()),
            Err(e) => {
                // Rollback on failure
                self.set(original_state)?;
                Err(e)
            }
        }
    }

    #[cfg(feature = "serde")]
    fn validate_snapshot(&self, snapshot: &StateSnapshot<S>) -> Result<(), StoreError> {
        // Check version compatibility
        // Validate data structure
        // Check for data corruption
        todo!()
    }
}
```

## Testing Strategy

### Unit Tests
```rust
#[cfg(feature = "serde")]
mod tests {
    use super::*;

    #[derive(Clone, Serialize, Deserialize)]
    struct TestState {
        count: i32,
        name: String,
    }

    impl State for TestState {}

    #[test]
    fn serialize_deserialize_roundtrip() {
        let state = TestState {
            count: 42,
            name: "test".to_string(),
        };

        let json = serde_json::to_string(&state).unwrap();
        let deserialized: TestState = serde_json::from_str(&json).unwrap();

        assert_eq!(state.count, deserialized.count);
        assert_eq!(state.name, deserialized.name);
    }

    #[test]
    fn store_json_export_import() {
        let store = Store::new(TestState {
            count: 100,
            name: "store_test".to_string(),
        });

        let json = store.to_json().unwrap();
        let new_store = Store::new(TestState {
            count: 0,
            name: "".to_string(),
        });

        new_store.from_json(&json).unwrap();

        let imported = new_store.get().get_untracked();
        assert_eq!(imported.count, 100);
        assert_eq!(imported.name, "store_test");
    }
}
```

### Property-Based Testing
```rust
#[cfg(feature = "serde")]
proptest! {
    #[test]
    fn serialization_roundtrip(state: TestState) {
        let json = serde_json::to_string(&state).unwrap();
        let deserialized: TestState = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(state, deserialized);
    }

    #[test]
    fn store_serialization_consistency(state: TestState) {
        let store1 = Store::new(state.clone());
        let store2 = Store::new(TestState::default());

        let json = store1.to_json().unwrap();
        store2.from_json(&json).unwrap();

        prop_assert_eq!(store1.get().get_untracked(), store2.get().get_untracked());
    }
}
```

### Integration Tests
```rust
#[cfg(feature = "serde")]
#[test]
fn machine_state_persistence() {
    let mut machine = Machine::new("idle", TestContext::default());

    // Modify machine state
    machine.context_mut().value = 42;
    machine.send(TestEvent::Start).unwrap();

    // Serialize machine state
    let json = machine.to_json().unwrap();

    // Create new machine and deserialize
    let mut new_machine = Machine::new("idle", TestContext::default());
    new_machine.from_json(&json).unwrap();

    // Verify state transfer
    assert_eq!(machine.current_state(), new_machine.current_state());
    assert_eq!(machine.context().value, new_machine.context().value);
}
```

## Performance Impact

### Compile Time
- **Minimal**: Serde is widely optimized
- **Conditional**: Only compiled when feature is enabled
- **Impact**: ~5-10% increase when serde feature is used

### Runtime
- **JSON**: Human-readable but slower
- **Binary**: Fast but not human-readable
- **Memory**: Temporary allocations during serialization

### Optimization Opportunities
```rust
#[cfg(feature = "serde")]
pub struct SerializationCache<S> {
    last_serialized: Option<String>,
    last_state: Option<S>,
}

#[cfg(feature = "serde")]
impl<S: SerializableState + PartialEq> SerializationCache<S> {
    pub fn get_cached_json(&mut self, state: &S) -> Option<&str> {
        if Some(state) == self.last_state.as_ref() {
            self.last_serialized.as_deref()
        } else {
            None
        }
    }

    pub fn update_cache(&mut self, state: S, json: String) {
        self.last_state = Some(state);
        self.last_serialized = Some(json);
    }
}
```

## Security Considerations

### Safe Deserialization
```rust
#[cfg(feature = "serde")]
pub fn safe_deserialize_state<S: SerializableState>(
    json: &str,
    max_size: usize
) -> Result<S, SerializationError> {
    // Check size limits
    if json.len() > max_size {
        return Err(SerializationError::DataCorruption);
    }

    // Use serde with limits
    let mut deserializer = serde_json::Deserializer::from_str(json);
    let state = S::deserialize(&mut deserializer)?;

    // Additional validation
    validate_deserialized_state(&state)?;

    Ok(state)
}
```

### Data Validation
```rust
#[cfg(feature = "serde")]
fn validate_deserialized_state<S>(state: &S) -> Result<(), SerializationError> {
    // Implement custom validation logic
    // Check for reasonable values
    // Validate internal consistency
    todo!()
}
```

### Information Disclosure
- Avoid serializing sensitive data
- Use selective serialization with `#[serde(skip)]`
- Implement redacted serialization for logs

```rust
#[derive(Clone, Serialize, Deserialize)]
struct UserState {
    user_id: String,
    #[serde(skip_serializing)]
    password_hash: String,  // Never serialize
    #[serde(skip_deserializing)]
    session_token: Option<String>,  // Only serialize, not deserialize
}
```

## Migration Guide

### Adding Serialization Support
```rust
// Before - no serialization
#[derive(Clone)]
struct MyState {
    count: i32,
    name: String,
}

// After - add serialization
#[derive(Clone, Serialize, Deserialize)]
struct MyState {
    count: i32,
    name: String,
}
```

### Feature Gates
```rust
// In Cargo.toml
[features]
serde = ["dep:serde", "dep:serde_json"]

// In code
#[cfg(feature = "serde")]
fn save_state(store: &Store<MyState>) -> Result<(), Box<dyn Error>> {
    let json = store.to_json()?;
    std::fs::write("state.json", json)?;
    Ok(())
}

#[cfg(not(feature = "serde"))]
fn save_state(_store: &Store<MyState>) -> Result<(), Box<dyn Error>> {
    Err("Serialization not enabled".into())
}
```

### Gradual Adoption
```rust
// Phase 1: Add serde derives (no functional change)
#[derive(Clone, Serialize, Deserialize)]
struct MyState { /* ... */ }

// Phase 2: Add serialization methods
impl MyState {
    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> Result<String, SerializationError> {
        serde_json::to_string(self)
            .map_err(|e| SerializationError::Json(e.to_string()))
    }
}

// Phase 3: Add persistence layer integration
#[cfg(feature = "serde")]
pub struct PersistentStore<S: SerializableState> {
    store: Store<S>,
    storage: Box<dyn StorageBackend>,
}
```

## Future Extensions

### Custom Serialization
```rust
#[cfg(feature = "serde")]
pub trait CustomSerializable {
    fn serialize_custom(&self, serializer: &mut dyn Serializer) -> Result<(), SerializationError>;
    fn deserialize_custom(deserializer: &mut dyn Deserializer) -> Result<Self, SerializationError>
    where
        Self: Sized;
}
```

### Compression
```rust
#[cfg(feature = "serde")]
pub mod compressed {
    use flate2::{Compression, GzBuilder};
    use std::io::Read;

    pub fn compress_json(json: &str) -> Result<Vec<u8>, SerializationError> {
        let mut encoder = GzBuilder::new()
            .compression(Compression::default())
            .finish(Vec::new());

        encoder.write_all(json.as_bytes())?;
        encoder.finish()
    }

    pub fn decompress_json(data: &[u8]) -> Result<String, SerializationError> {
        let mut decoder = flate2::read::GzDecoder::new(data);
        let mut json = String::new();
        decoder.read_to_string(&mut json)?;
        Ok(json)
    }
}
```

### Streaming Serialization
```rust
#[cfg(feature = "serde")]
pub struct StreamingSerializer<W: Write> {
    writer: W,
    serializer: serde_json::Serializer<W>,
}

#[cfg(feature = "serde")]
impl<W: Write> StreamingSerializer<W> {
    pub fn serialize_state<S: SerializableState>(&mut self, state: &S) -> Result<(), SerializationError> {
        state.serialize(&mut self.serializer)?;
        Ok(())
    }
}
```

## Risk Assessment

### Likelihood: Low
- Serde is mature and widely used
- Optional feature prevents mandatory dependencies
- Compile-time feature gates prevent runtime issues

### Impact: Low
- Backward compatible (feature-gated)
- Clear error messages for missing features
- No performance impact when disabled

### Mitigation
- Comprehensive testing with all supported formats
- Security-focused deserialization
- Clear documentation on feature usage
- Migration examples and helpers
