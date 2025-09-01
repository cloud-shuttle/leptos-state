.PHONY: help install setup test build clean serve dev

# Default target
help:
	@echo "🚀 Leptos State Development Commands"
	@echo ""
	@echo "📦 Setup & Installation:"
	@echo "  make setup          - Install dependencies and setup environment"
	@echo "  make install        - Install Playwright browsers"
	@echo ""
	@echo "🔨 Building:"
	@echo "  make build          - Build all Rust targets"
	@echo "  make build-wasm     - Build WASM examples"
	@echo "  make build-web      - Build web examples"
	@echo ""
	@echo "🧪 Testing:"
	@echo "  make test           - Run all tests"
	@echo "  make test-rust      - Run Rust unit tests"
	@echo "  make test-web       - Run Playwright web tests"
	@echo "  make test-web-ui    - Run Playwright tests with UI"
	@echo "  make test-wasm      - Run Playwright WASM example tests"
	@echo "  make test-wasm-ui   - Run Playwright WASM tests with UI"
	@echo ""
	@echo "🌐 Development:"
	@echo "  make serve          - Serve examples on http://localhost:8000"
	@echo "  make dev-counter    - Serve counter example with Trunk"
	@echo "  make dev-todo       - Serve todo app with Trunk"
	@echo "  make dev-analytics  - Serve analytics dashboard with Trunk"
	@echo ""
	@echo "🧹 Maintenance:"
	@echo "  make clean          - Clean build artifacts"
	@echo "  make clean-all      - Clean all artifacts including node_modules"

# Setup environment
setup: install
	@echo "🔧 Setting up development environment..."
	@pnpm install
	@echo "✅ Setup complete!"

# Install Playwright browsers
install:
	@echo "📦 Installing Playwright browsers..."
	@pnpm exec playwright install
	@echo "✅ Playwright browsers installed!"

# Build targets
build:
	@echo "🔨 Building all targets..."
	@cargo build --workspace
	@echo "✅ Build complete!"

build-wasm:
	@echo "🔨 Building WASM examples..."
	@cargo build --target wasm32-unknown-unknown --workspace
	@echo "✅ WASM build complete!"

build-web:
	@echo "🔨 Building web examples..."
	@cd examples/counter && trunk build
	@cd examples/todo-app && trunk build
	@cd examples/analytics-dashboard && trunk build
	@echo "✅ Web build complete!"

# Testing targets
test: test-rust test-web
	@echo "✅ All tests complete!"

test-rust:
	@echo "🧪 Running Rust tests..."
	@cargo test --workspace
	@echo "✅ Rust tests complete!"

test-web: build-web
	@echo "🧪 Running Playwright web tests..."
	@pnpm test:web
	@echo "✅ Web tests complete!"

test-wasm: build-web
	@echo "🧪 Running Playwright WASM example tests..."
	@pnpm test:wasm
	@echo "✅ WASM tests complete!"

test-web-ui: build-web
	@echo "🧪 Running Playwright tests with UI..."
	@pnpm test:web:ui

test-wasm-ui: build-web
	@echo "🧪 Running Playwright WASM tests with UI..."
	@pnpm test:wasm:ui

# Development servers
serve:
	@echo "🌐 Serving examples on http://localhost:8000..."
	@python3 -m http.server 8000 --directory examples

dev-counter:
	@echo "🌐 Serving counter example..."
	@cd examples/counter && trunk serve

dev-todo:
	@echo "🌐 Serving todo app..."
	@cd examples/todo-app && trunk serve

dev-analytics:
	@echo "🌐 Serving analytics dashboard..."
	@cd examples/analytics-dashboard && trunk serve

# Cleanup targets
clean:
	@echo "🧹 Cleaning build artifacts..."
	@cargo clean
	@rm -rf target/
	@echo "✅ Clean complete!"

clean-all: clean
	@echo "🧹 Cleaning all artifacts..."
	@rm -rf node_modules/
	@rm -rf pnpm-lock.yaml
	@echo "✅ Full clean complete!"

# Quick development workflow
dev: build test-web
	@echo "🚀 Development workflow complete!"
