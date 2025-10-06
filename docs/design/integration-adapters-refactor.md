# Integration Adapters Refactor Design
## Breaking Down machine/integration_adapters.rs (538 lines)

**Version:** 1.0  
**Date:** September 20, 2025  
**Priority:** CRITICAL - File too large and mostly stub code  

---

## Current State Analysis

**File:** `machine/integration_adapters.rs`  
**Lines:** 538  
**Status:** ❌ VIOLATION (>300 line limit)  
**Issues:**
- Multiple adapter implementations in one file
- Extensive stub/placeholder code
- Mixed concerns (HTTP, database, message queue, file system)
- Poor separation of adapter types
- Difficult to test and extend

## Refactor Strategy

### Target Architecture

```
machine/integration/
├── adapters/
│   ├── mod.rs              # Public API and common types
│   ├── http.rs             # HTTP/webhook adapters (120 lines)
│   ├── database.rs         # Database integration (130 lines)
│   ├── message_queue.rs    # Message queue adapters (140 lines)
│   ├── filesystem.rs       # File system operations (110 lines)
│   ├── websocket.rs        # WebSocket connections (120 lines)
│   └── custom.rs           # Custom adapter framework (100 lines)
├── core/
│   ├── mod.rs             # Core integration types
│   ├── adapter_trait.rs   # Adapter trait definitions
│   └── pipeline.rs        # Integration pipeline logic
└── config/
    ├── mod.rs             # Integration configuration
    └── types.rs           # Configuration types
```

### 1. Adapter Trait Definition (core/adapter_trait.rs - 80 lines)

```rust
/// Core trait for all integration adapters
#[async_trait::async_trait]
pub trait IntegrationAdapter: Send + Sync {
    /// Send data through this adapter
    async fn send(&self, data: IntegrationEvent) -> Result<(), IntegrationError>;

    /// Receive data from this adapter
    async fn receive(&self) -> Result<Vec<IntegrationEvent>, IntegrationError>;

    /// Health check for this adapter
    async fn health_check(&self) -> Result<HealthStatus, IntegrationError>;

    /// Get adapter metadata
    fn metadata(&self) -> AdapterMetadata;

    /// Configure the adapter
    async fn configure(&mut self, config: serde_json::Value) -> Result<(), IntegrationError>;
}

/// Adapter metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterMetadata {
    pub name: String,
    pub adapter_type: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub config_schema: serde_json::Value,
}
```

### 2. HTTP/Webhook Adapters (adapters/http.rs - 120 lines)

```rust
/// HTTP-based integration adapter
#[derive(Debug, Clone)]
pub struct HttpAdapter {
    client: reqwest::Client,
    endpoint: String,
    headers: HashMap<String, String>,
    retry_policy: RetryPolicy,
    timeout: Duration,
}

impl HttpAdapter {
    pub fn new(endpoint: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            endpoint,
            headers: HashMap::new(),
            retry_policy: RetryPolicy::default(),
            timeout: Duration::from_secs(30),
        }
    }

    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

#[async_trait::async_trait]
impl IntegrationAdapter for HttpAdapter {
    async fn send(&self, event: IntegrationEvent) -> Result<(), IntegrationError> {
        let mut request = self.client
            .post(&self.endpoint)
            .json(&event)
            .timeout(self.timeout);

        for (key, value) in &self.headers {
            request = request.header(key, value);
        }

        let response = request.send().await?;
        if !response.status().is_success() {
            return Err(IntegrationError::HttpError {
                status: response.status(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        Ok(())
    }

    async fn receive(&self) -> Result<Vec<IntegrationEvent>, IntegrationError> {
        // HTTP adapters typically don't receive unsolicited data
        Ok(Vec::new())
    }

    async fn health_check(&self) -> Result<HealthStatus, IntegrationError> {
        let response = self.client
            .get(&format!("{}/health", self.endpoint.trim_end_matches('/')))
            .timeout(Duration::from_secs(5))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(HealthStatus::Healthy)
        } else {
            Ok(HealthStatus::Unhealthy)
        }
    }

    fn metadata(&self) -> AdapterMetadata {
        AdapterMetadata {
            name: "HTTP Adapter".to_string(),
            adapter_type: "http".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            capabilities: vec!["send".to_string()],
            config_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "endpoint": {"type": "string"},
                    "headers": {"type": "object"},
                    "timeout": {"type": "integer"}
                }
            }),
        }
    }

    async fn configure(&mut self, config: serde_json::Value) -> Result<(), IntegrationError> {
        if let Some(endpoint) = config.get("endpoint").and_then(|v| v.as_str()) {
            self.endpoint = endpoint.to_string();
        }
        // Additional configuration...
        Ok(())
    }
}
```

### 3. Message Queue Adapters (adapters/message_queue.rs - 140 lines)

```rust
/// Message queue integration adapter
#[derive(Debug)]
pub struct MessageQueueAdapter {
    connection: Arc<Mutex<Option<QueueConnection>>>,
    queue_name: String,
    exchange_name: Option<String>,
    routing_key: Option<String>,
    prefetch_count: u16,
}

impl MessageQueueAdapter {
    pub fn new(queue_name: String) -> Self {
        Self {
            connection: Arc::new(Mutex::new(None)),
            queue_name,
            exchange_name: None,
            routing_key: None,
            prefetch_count: 10,
        }
    }

    pub async fn connect(&self, connection_string: &str) -> Result<(), IntegrationError> {
        // Establish connection to message queue
        // Implementation depends on specific MQ system (RabbitMQ, Kafka, etc.)
        Ok(())
    }
}

#[async_trait::async_trait]
impl IntegrationAdapter for MessageQueueAdapter {
    async fn send(&self, event: IntegrationEvent) -> Result<(), IntegrationError> {
        let conn = self.connection.lock().await;
        if let Some(connection) = conn.as_ref() {
            // Send message to queue
            Ok(())
        } else {
            Err(IntegrationError::NotConnected)
        }
    }

    async fn receive(&self) -> Result<Vec<IntegrationEvent>, IntegrationError> {
        let conn = self.connection.lock().await;
        if let Some(connection) = conn.as_ref() {
            // Receive messages from queue
            Ok(Vec::new())
        } else {
            Err(IntegrationError::NotConnected)
        }
    }

    async fn health_check(&self) -> Result<HealthStatus, IntegrationError> {
        let conn = self.connection.lock().await;
        if conn.is_some() {
            Ok(HealthStatus::Healthy)
        } else {
            Ok(HealthStatus::Unhealthy)
        }
    }

    fn metadata(&self) -> AdapterMetadata {
        AdapterMetadata {
            name: "Message Queue Adapter".to_string(),
            adapter_type: "message_queue".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            capabilities: vec!["send".to_string(), "receive".to_string()],
            config_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "queue_name": {"type": "string"},
                    "exchange_name": {"type": "string"},
                    "routing_key": {"type": "string"}
                }
            }),
        }
    }

    async fn configure(&mut self, config: serde_json::Value) -> Result<(), IntegrationError> {
        if let Some(queue) = config.get("queue_name").and_then(|v| v.as_str()) {
            self.queue_name = queue.to_string();
        }
        // Additional configuration...
        Ok(())
    }
}
```

### 4. File System Adapters (adapters/filesystem.rs - 110 lines)

```rust
/// File system integration adapter
#[derive(Debug, Clone)]
pub struct FileSystemAdapter {
    base_path: PathBuf,
    file_format: FileFormat,
    compression: bool,
    buffer_size: usize,
}

#[derive(Debug, Clone)]
pub enum FileFormat {
    Json,
    Csv,
    Tsv,
    Custom(String),
}

impl FileSystemAdapter {
    pub fn new(base_path: PathBuf) -> Self {
        Self {
            base_path,
            file_format: FileFormat::Json,
            compression: false,
            buffer_size: 8192,
        }
    }
}

#[async_trait::async_trait]
impl IntegrationAdapter for FileSystemAdapter {
    async fn send(&self, event: IntegrationEvent) -> Result<(), IntegrationError> {
        let filename = format!("{}.{}", event.id, self.file_extension());
        let filepath = self.base_path.join(filename);

        let data = match self.file_format {
            FileFormat::Json => serde_json::to_string(&event)?,
            FileFormat::Csv => self.format_csv(&event)?,
            FileFormat::Tsv => self.format_tsv(&event)?,
            FileFormat::Custom(ref format) => self.format_custom(&event, format)?,
        };

        tokio::fs::write(&filepath, data).await?;
        Ok(())
    }

    async fn receive(&self) -> Result<Vec<IntegrationEvent>, IntegrationError> {
        let mut events = Vec::new();
        let mut dir = tokio::fs::read_dir(&self.base_path).await?;

        while let Some(entry) = dir.next_entry().await? {
            if entry.file_type().await?.is_file() {
                let content = tokio::fs::read_to_string(entry.path()).await?;
                let event: IntegrationEvent = match self.file_format {
                    FileFormat::Json => serde_json::from_str(&content)?,
                    _ => return Err(IntegrationError::UnsupportedFormat),
                };
                events.push(event);
            }
        }

        Ok(events)
    }

    async fn health_check(&self) -> Result<HealthStatus, IntegrationError> {
        if self.base_path.exists() {
            Ok(HealthStatus::Healthy)
        } else {
            Ok(HealthStatus::Unhealthy)
        }
    }

    fn metadata(&self) -> AdapterMetadata {
        AdapterMetadata {
            name: "File System Adapter".to_string(),
            adapter_type: "filesystem".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            capabilities: vec!["send".to_string(), "receive".to_string()],
            config_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "base_path": {"type": "string"},
                    "format": {"type": "string", "enum": ["json", "csv", "tsv"]},
                    "compression": {"type": "boolean"}
                }
            }),
        }
    }

    async fn configure(&mut self, config: serde_json::Value) -> Result<(), IntegrationError> {
        if let Some(path) = config.get("base_path").and_then(|v| v.as_str()) {
            self.base_path = PathBuf::from(path);
        }
        // Additional configuration...
        Ok(())
    }
}
```

### 5. Custom Adapter Framework (adapters/custom.rs - 100 lines)

```rust
/// Framework for creating custom integration adapters
pub struct CustomAdapter<F, G>
where
    F: Fn(IntegrationEvent) -> Result<(), IntegrationError> + Send + Sync,
    G: Fn() -> Result<Vec<IntegrationEvent>, IntegrationError> + Send + Sync,
{
    sender: F,
    receiver: G,
    metadata: AdapterMetadata,
}

impl<F, G> CustomAdapter<F, G>
where
    F: Fn(IntegrationEvent) -> Result<(), IntegrationError> + Send + Sync,
    G: Fn() -> Result<Vec<IntegrationEvent>, IntegrationError> + Send + Sync,
{
    pub fn new(sender: F, receiver: G, metadata: AdapterMetadata) -> Self {
        Self {
            sender,
            receiver,
            metadata,
        }
    }
}

#[async_trait::async_trait]
impl<F, G> IntegrationAdapter for CustomAdapter<F, G>
where
    F: Fn(IntegrationEvent) -> Result<(), IntegrationError> + Send + Sync,
    G: Fn() -> Result<Vec<IntegrationEvent>, IntegrationError> + Send + Sync,
{
    async fn send(&self, event: IntegrationEvent) -> Result<(), IntegrationError> {
        (self.sender)(event)
    }

    async fn receive(&self) -> Result<Vec<IntegrationEvent>, IntegrationError> {
        (self.receiver)()
    }

    async fn health_check(&self) -> Result<HealthStatus, IntegrationError> {
        // Custom adapters report as healthy by default
        Ok(HealthStatus::Healthy)
    }

    fn metadata(&self) -> AdapterMetadata {
        self.metadata.clone()
    }

    async fn configure(&mut self, _config: serde_json::Value) -> Result<(), IntegrationError> {
        // Custom adapters handle their own configuration
        Ok(())
    }
}
```

## Implementation Status

### Currently Implemented
- [ ] HTTP/Webhook adapters (basic stub)
- [ ] Database integration (minimal stub)
- [ ] Message queue adapters (placeholder)
- [ ] File system operations (partial)
- [ ] WebSocket connections (not started)

### Implementation Plan

### Phase 1: Core Framework (Week 1-2)
- [ ] Extract IntegrationAdapter trait
- [ ] Create adapter metadata system
- [ ] Implement configuration framework
- [ ] Set up proper error handling

### Phase 2: HTTP Adapters (Week 3-4)
- [ ] Complete HTTP/Webhook adapter
- [ ] Add retry logic and error handling
- [ ] Implement health checks
- [ ] Add comprehensive tests

### Phase 3: Message Queue (Week 5-6)
- [ ] Implement RabbitMQ adapter
- [ ] Add Kafka support
- [ ] Implement connection pooling
- [ ] Add message serialization

### Phase 4: File System (Week 7-8)
- [ ] Complete file operations
- [ ] Add compression support
- [ ] Implement batch processing
- [ ] Add file watching capabilities

### Phase 5: Advanced Features (Week 9-10)
- [ ] WebSocket connections
- [ ] Database integrations
- [ ] Custom adapter framework
- [ ] Performance monitoring

## Testing Strategy

### Unit Tests
- [ ] Adapter creation and configuration
- [ ] Send/receive operations
- [ ] Error handling scenarios
- [ ] Health check functionality

### Integration Tests
- [ ] End-to-end data flow
- [ ] Connection reliability
- [ ] Performance under load
- [ ] Error recovery

### Contract Tests
- [ ] Adapter trait compliance
- [ ] Configuration schema validation
- [ ] Metadata accuracy

## Benefits

### Maintainability
- **Focused modules:** Each adapter type in separate file
- **Clear interfaces:** Well-defined trait boundaries
- **Easier testing:** Isolated adapter functionality
- **Better documentation:** Specific adapter guides

### Extensibility
- **Plugin architecture:** Easy to add new adapters
- **Configuration framework:** Standardized setup
- **Health monitoring:** Built-in operational visibility
- **Error handling:** Consistent error patterns

### Performance
- **Async operations:** Non-blocking I/O
- **Connection pooling:** Efficient resource usage
- **Batch processing:** Optimized data transfer
- **Monitoring:** Performance tracking

---

**Design Author:** Senior Rust Engineer  
**File Size Reduction:** 538 → ~120 lines each (77% reduction)  
**Stub Code Reduction:** 70% → 10% (significant improvement)  
**Timeline:** 10 weeks  
**Success Criteria:** All adapters functional, 85% test coverage
