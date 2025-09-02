# Testing Guide

This project now includes a comprehensive testing setup with Nix, Playwright, and Make for testing both Rust code and web examples.

## 🚀 Quick Start

### 1. Enter the Nix Development Environment
```bash
nix develop
```

### 2. Install Dependencies
```bash
make setup
```

### 3. Run Tests
```bash
make test          # Run all tests
make test-rust     # Run only Rust tests
make test-web      # Run only web tests
```

## 🧪 Testing Types

### Rust Tests
- **Unit Tests**: `cargo test --workspace`
- **Integration Tests**: `cargo test --package integration-tests`
- **Performance Tests**: `cargo test --package performance-tests`

### Web Tests (Playwright)
- **Counter Example**: Tests the basic counter functionality
- **Traffic Light Example**: Tests state machine transitions
- **Todo App**: Tests CRUD operations
- **Analytics Dashboard**: Tests complex state management

## 🌐 Web Testing Workflow

### 1. Build Web Examples
```bash
make build-web
```

### 2. Serve Examples
```bash
make serve          # Static file server
make dev-counter    # Trunk dev server for counter
make dev-todo       # Trunk dev server for todo app
```

### 3. Run Playwright Tests
```bash
# Run all web tests
pnpm test:web

# Run with UI (interactive)
pnpm test:web:ui

# Run in headed mode (see browser)
pnpm test:web:headed
```

## 🛠️ Development Commands

### Building
```bash
make build          # Build all Rust targets
make build-wasm     # Build WASM examples
make build-web      # Build web examples with Trunk
```

### Development Servers
```bash
make serve          # Static file server on :8000
make dev-counter    # Counter example dev server
make dev-todo       # Todo app dev server
make dev-analytics  # Analytics dashboard dev server
```

### Cleanup
```bash
make clean          # Clean Rust build artifacts
make clean-all      # Clean everything including node_modules
```

## 📁 Test Structure

```
tests/
├── common/                 # Common test utilities
├── integration/           # Rust integration tests
├── performance/           # Performance tests
├── unit/                 # Rust unit tests
└── playwright/           # Playwright web tests
    ├── counter.spec.ts   # Counter example tests
    ├── traffic-light.spec.ts # Traffic light tests
    └── todo.spec.ts      # Todo app tests
```

## 🔧 Configuration

### Playwright Config
- **Base URL**: `http://localhost:8000`
- **Browsers**: Chromium, Firefox, WebKit
- **Web Server**: Auto-starts Python HTTP server
- **Screenshots**: On failure
- **Traces**: On first retry

### Nix Environment
- **Rust**: Latest stable with rust-analyzer
- **Node.js**: v20 with pnpm
- **Playwright**: All system dependencies included
- **Build Tools**: wasm-pack, trunk, cargo-watch

## 🐛 Troubleshooting

### Common Issues

1. **Playwright browsers not found**
   ```bash
   pnpm exec playwright install
   ```

2. **WASM build fails**
   ```bash
   rustup target add wasm32-unknown-unknown
   cargo install wasm-pack
   ```

3. **Trunk not found**
   ```bash
   cargo install trunk
   ```

4. **Port 8000 already in use**
   ```bash
   # Kill existing process or change port in playwright.config.ts
   lsof -ti:8000 | xargs kill -9
   ```

### Debug Mode
```bash
# Run tests with debug output
RUST_LOG=debug cargo test
RUST_BACKTRACE=1 cargo test

# Playwright debug
pnpm test:web --debug
```

## 📊 Test Reports

After running Playwright tests, view the HTML report:
```bash
open playwright-report/index.html
```

## 🔄 CI/CD Integration

The testing setup is designed to work in CI environments:
- **Nix**: Reproducible builds
- **Playwright**: Headless mode for CI
- **Make**: Standardized commands
- **Exit Codes**: Proper failure reporting
