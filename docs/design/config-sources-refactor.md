# Config Sources Refactor Design
## Breaking Down utils/config/sources.rs (635 lines)

**Version:** 1.0  
**Date:** September 20, 2025  
**Priority:** CRITICAL - File too large for maintenance  

---

## Current State Analysis

**File:** `utils/config/sources.rs`  
**Lines:** 635  
**Status:** ❌ VIOLATION (>300 line limit)  
**Issues:**
- Mixed responsibilities (file, HTTP, database, environment)
- Large enum with complex matching logic
- Inconsistent error handling
- Difficult to test and maintain

## Refactor Strategy

### Target Architecture

```
utils/config/
├── sources/
│   ├── mod.rs           # Public API re-exports
│   ├── file.rs          # File-based configuration (150 lines)
│   ├── http.rs          # HTTP-based configuration (120 lines)
│   ├── database.rs      # Database configuration (130 lines)
│   ├── environment.rs   # Environment variables (110 lines)
│   ├── custom.rs        # Custom sources (100 lines)
│   └── traits.rs        # Common traits (80 lines)
├── types.rs             # Core types (unchanged)
└── manager.rs           # Configuration manager (unchanged)
```

### 1. Common Traits (traits.rs - 80 lines)

```rust
/// Common trait for all configuration sources
#[async_trait::async_trait]
pub trait ConfigSource: Send + Sync {
    /// Load configuration from this source
    async fn load(&self) -> Result<serde_json::Value, ConfigError>;

    /// Check if this source is available
    async fn is_available(&self) -> bool;

    /// Get source priority (higher = more preferred)
    fn priority(&self) -> u8;

    /// Validate source configuration
    fn validate(&self) -> Result<(), ConfigError>;
}

/// Configuration source metadata
#[derive(Debug, Clone)]
pub struct SourceMetadata {
    pub name: String,
    pub source_type: String,
    pub priority: u8,
    pub last_modified: Option<SystemTime>,
}
```

### 2. File Sources (file.rs - 150 lines)

```rust
/// File-based configuration source
#[derive(Debug, Clone)]
pub enum FileSource {
    Json { path: PathBuf },
    Toml { path: PathBuf },
    Yaml { path: PathBuf },
    Auto { path: PathBuf }, // Auto-detect format
}

#[async_trait::async_trait]
impl ConfigSource for FileSource {
    async fn load(&self) -> Result<serde_json::Value, ConfigError> {
        match self {
            FileSource::Json { path } => load_json_file(path).await,
            FileSource::Toml { path } => load_toml_file(path).await,
            FileSource::Yaml { path } => load_yaml_file(path).await,
            FileSource::Auto { path } => load_auto_file(path).await,
        }
    }

    async fn is_available(&self) -> bool {
        let path = self.path();
        path.exists() && path.is_file()
    }

    fn priority(&self) -> u8 { 90 }

    fn validate(&self) -> Result<(), ConfigError> {
        let path = self.path();
        if path.to_string_lossy().is_empty() {
            return Err(ConfigError::InvalidPath("Path cannot be empty".into()));
        }
        Ok(())
    }
}
```

### 3. HTTP Sources (http.rs - 120 lines)

```rust
/// HTTP-based configuration source
#[derive(Debug, Clone)]
pub struct HttpSource {
    pub url: String,
    pub headers: HashMap<String, String>,
    pub timeout: Duration,
    pub retry_count: u32,
}

#[async_trait::async_trait]
impl ConfigSource for HttpSource {
    async fn load(&self) -> Result<serde_json::Value, ConfigError> {
        let client = reqwest::Client::new();
        let mut request = client.get(&self.url).timeout(self.timeout);

        for (key, value) in &self.headers {
            request = request.header(key, value);
        }

        let response = request.send().await?;
        let json: serde_json::Value = response.json().await?;

        Ok(json)
    }

    async fn is_available(&self) -> bool {
        // Check if URL is reachable
        reqwest::get(&self.url).await.is_ok()
    }

    fn priority(&self) -> u8 { 70 }

    fn validate(&self) -> Result<(), ConfigError> {
        if self.url.trim().is_empty() {
            return Err(ConfigError::InvalidUrl("URL cannot be empty".into()));
        }
        if !self.url.starts_with("http://") && !self.url.starts_with("https://") {
            return Err(ConfigError::InvalidUrl("URL must start with http:// or https://".into()));
        }
        Ok(())
    }
}
```

### 4. Database Sources (database.rs - 130 lines)

```rust
/// Database configuration source
#[derive(Debug, Clone)]
pub struct DatabaseSource {
    pub connection_string: String,
    pub table: String,
    pub key_column: String,
    pub value_column: String,
    pub query_timeout: Duration,
}

#[async_trait::async_trait]
impl ConfigSource for DatabaseSource {
    async fn load(&self) -> Result<serde_json::Value, ConfigError> {
        // Database connection and query logic
        // Implementation details...
        Ok(serde_json::Value::Null) // Placeholder
    }

    async fn is_available(&self) -> bool {
        // Check database connectivity
        false // Placeholder
    }

    fn priority(&self) -> u8 { 60 }

    fn validate(&self) -> Result<(), ConfigError> {
        if self.connection_string.trim().is_empty() {
            return Err(ConfigError::InvalidConnection("Connection string cannot be empty".into()));
        }
        if self.table.trim().is_empty() {
            return Err(ConfigError::InvalidTable("Table name cannot be empty".into()));
        }
        Ok(())
    }
}
```

### 5. Environment Variables (environment.rs - 110 lines)

```rust
/// Environment variable configuration source
#[derive(Debug, Clone)]
pub struct EnvironmentSource {
    pub prefix: Option<String>,
    pub separator: String,
    pub case_sensitive: bool,
}

#[async_trait::async_trait]
impl ConfigSource for EnvironmentSource {
    async fn load(&self) -> Result<serde_json::Value, ConfigError> {
        let mut config = serde_json::Map::new();

        for (key, value) in std::env::vars() {
            if self.matches_prefix(&key) {
                let config_key = self.transform_key(&key);
                config.insert(config_key, serde_json::Value::String(value));
            }
        }

        Ok(serde_json::Value::Object(config))
    }

    async fn is_available(&self) -> bool {
        // Environment variables are always available
        true
    }

    fn priority(&self) -> u8 { 50 }

    fn validate(&self) -> Result<(), ConfigError> {
        // Environment sources are always valid
        Ok(())
    }
}
```

### 6. Custom Sources (custom.rs - 100 lines)

```rust
/// Custom configuration source
pub struct CustomSource<F> {
    pub loader: F,
    pub name: String,
    pub priority: u8,
}

#[async_trait::async_trait]
impl<F> ConfigSource for CustomSource<F>
where
    F: Fn() -> Result<serde_json::Value, ConfigError> + Send + Sync,
{
    async fn load(&self) -> Result<serde_json::Value, ConfigError> {
        (self.loader)()
    }

    async fn is_available(&self) -> bool {
        // Custom sources are available by definition
        true
    }

    fn priority(&self) -> u8 { self.priority }

    fn validate(&self) -> Result<(), ConfigError> {
        if self.name.trim().is_empty() {
            return Err(ConfigError::InvalidName("Source name cannot be empty".into()));
        }
        Ok(())
    }
}
```

### 7. Module Re-exports (mod.rs - 50 lines)

```rust
//! Configuration sources for leptos-state

pub mod traits;
pub mod file;
pub mod http;
pub mod database;
pub mod environment;
pub mod custom;

// Re-export commonly used types
pub use traits::{ConfigSource, SourceMetadata};
pub use file::FileSource;
pub use http::HttpSource;
pub use database::DatabaseSource;
pub use environment::EnvironmentSource;
pub use custom::CustomSource;

// Legacy compatibility - will be removed in future version
pub use file::FileSource as JsonFile;
pub use file::FileSource as TomlFile;
pub use file::FileSource as YamlFile;
pub use http::HttpSource as RemoteUrl;
pub use database::DatabaseSource as Database;
pub use environment::EnvironmentSource as Environment;
```

## Migration Strategy

### Phase 1: Extract Files (Week 1)
1. Create new module structure
2. Move code from sources.rs to individual files
3. Update imports throughout codebase
4. Ensure compilation passes

### Phase 2: API Compatibility (Week 2)
1. Add re-export compatibility layer
2. Update documentation
3. Add deprecation warnings for old API
4. Update examples and tests

### Phase 3: Enhancement (Week 3)
1. Add new features to individual modules
2. Improve error handling
3. Add comprehensive tests
4. Update performance optimizations

## Benefits

### Maintainability
- **Smaller files:** Each <150 lines vs 635 lines
- **Single responsibility:** Each file handles one concern
- **Easier testing:** Isolated functionality
- **Better code review:** Focused changes

### Performance
- **Reduced compilation time:** Smaller files compile faster
- **Better optimization:** Compiler can optimize focused code
- **Memory usage:** Less memory overhead

### Developer Experience
- **Faster navigation:** Find relevant code quickly
- **Easier debugging:** Isolated concerns
- **Better testing:** Focused test coverage
- **Clearer APIs:** Well-defined boundaries

## Implementation Checklist

### File Creation
- [ ] `sources/traits.rs` - Common traits and types
- [ ] `sources/file.rs` - File-based sources
- [ ] `sources/http.rs` - HTTP-based sources
- [ ] `sources/database.rs` - Database sources
- [ ] `sources/environment.rs` - Environment variables
- [ ] `sources/custom.rs` - Custom sources
- [ ] `sources/mod.rs` - Public API

### Code Migration
- [ ] Move existing implementations to new files
- [ ] Update all import statements
- [ ] Add backward compatibility re-exports
- [ ] Update documentation

### Testing
- [ ] Unit tests for each source type
- [ ] Integration tests for source loading
- [ ] Error handling tests
- [ ] Performance tests

### Documentation
- [ ] Update API documentation
- [ ] Add usage examples
- [ ] Migration guide
- [ ] Performance benchmarks

## Risk Assessment

### Low Risk
- API compatibility through re-exports
- Gradual migration possible
- No breaking changes for end users

### Mitigation
- Comprehensive testing before deployment
- Feature flags for new implementations
- Rollback plan if issues discovered

---

**Design Author:** Senior Rust Engineer  
**File Size Reduction:** 635 → ~150 lines each (75% reduction)  
**Maintainability Improvement:** High  
**Breaking Changes:** None (compatibility layer)  
**Timeline:** 3 weeks  
**Success Criteria:** All files <200 lines, 100% test coverage
