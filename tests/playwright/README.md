# Playwright Testing for Leptos State Examples

This directory contains comprehensive Playwright tests for all the WASM examples in the `leptos-state` library.

## Test Structure

### Individual Example Tests
- **`counter-wasm.spec.ts`** - Tests the counter example functionality
- **`traffic-light-wasm.spec.ts`** - Tests the traffic light state machine
- **`todo-app-wasm.spec.ts`** - Tests the todo application with full CRUD operations
- **`analytics-dashboard-wasm.spec.ts`** - Tests the analytics dashboard with metrics and interactions
- **`compatibility-example-wasm.spec.ts`** - Tests the compatibility layer functionality
- **`codegen-wasm.spec.ts`** - Tests the code generation example with state machines
- **`history-wasm.spec.ts`** - Tests the history tracking and restoration functionality

### Integration Tests
- **`run-all-examples.spec.ts`** - Comprehensive test that runs all examples to verify they work together

### Static Test Pages
- **`test-pages/counter.html`** - Static HTML version for testing without WASM
- **`test-pages/traffic-light.html`** - Static HTML version for testing without WASM

## Running Tests

### Prerequisites
1. Install dependencies: `pnpm install`
2. Install browsers: `pnpm install:browsers`
3. Build WASM examples: `cargo build`
4. Serve examples: `pnpm serve:examples`

### Test Commands

#### Run All Tests
```bash
# Run all Playwright tests
pnpm test:web

# Run with UI
pnpm test:web:ui

# Run in headed mode (visible browser)
pnpm test:web:headed
```

#### Run WASM Example Tests Only
```bash
# Run all WASM example tests
pnpm test:examples

# Run with UI
pnpm test:examples:ui

# Run in headed mode
pnpm test:examples:headed
```

#### Run Integration Tests
```bash
# Run the comprehensive integration test
pnpm test:all-examples

# Run with UI
pnpm test:all-examples:ui

# Run in headed mode
pnpm test:all-examples:headed
```

#### Run Specific Test Files
```bash
# Run only counter tests
pnpm playwright test tests/playwright/wasm-examples/counter-wasm.spec.ts

# Run only traffic light tests
pnpm playwright test tests/playwright/wasm-examples/traffic-light-wasm.spec.ts
```

## Test Coverage

### Counter Example
- ✅ Initial state display
- ✅ Increment functionality
- ✅ Decrement functionality
- ✅ Reset functionality
- ✅ User name input handling
- ✅ State persistence across interactions

### Traffic Light Example
- ✅ Initial state display
- ✅ State transitions via timer
- ✅ Pedestrian request handling
- ✅ Emergency stop functionality
- ✅ Reset to initial state
- ✅ State machine behavior

### Todo App Example
- ✅ Adding new todos
- ✅ Completing/uncompleting todos
- ✅ Deleting todos
- ✅ Input validation (empty/whitespace)
- ✅ Multiple todo management
- ✅ State persistence

### Analytics Dashboard Example
- ✅ Dashboard display
- ✅ Metrics rendering
- ✅ Timeframe selection
- ✅ Data refresh functionality
- ✅ Responsive layout
- ✅ Loading states

### Compatibility Example
- ✅ Counter functionality
- ✅ State management
- ✅ Computed values
- ✅ Effects handling
- ✅ State persistence
- ✅ Edge case handling

### Codegen Example
- ✅ Game state management
- ✅ State transitions (start, pause, resume, stop)
- ✅ Score and level management
- ✅ Code generation display
- ✅ Generated files listing
- ✅ State machine behavior

### History Example
- ✅ History tracking
- ✅ Deep history restoration
- ✅ Shallow history restoration
- ✅ History clearing
- ✅ Metadata display
- ✅ Edge case handling

## Test Features

### WASM Integration
- All tests wait for WASM to load before proceeding
- Verify `window.wasmBindings` availability
- Test actual compiled WASM functionality

### Cross-Browser Testing
- **Chromium** - Chrome/Edge compatibility
- **Firefox** - Firefox compatibility  
- **WebKit** - Safari compatibility

### Responsive Testing
- Tests work across different viewport sizes
- Verify responsive layout behavior

### Error Handling
- Tests handle edge cases gracefully
- Verify graceful degradation
- Test invalid input handling

## Configuration

### Playwright Config (`playwright.config.ts`)
- Base URL: `http://localhost:8000`
- Web server: Python HTTP server on port 8000
- Test directory: `./tests/playwright`
- Parallel execution enabled
- HTML reporter with screenshots on failure

### Test Timeouts
- Default timeout: 30 seconds
- WASM loading timeout: 10 seconds
- State transition delays: 100ms

## Development Workflow

### Adding New Tests
1. Create test file in appropriate directory
2. Follow naming convention: `{example-name}-wasm.spec.ts`
3. Include WASM loading wait in `beforeEach`
4. Test core functionality and edge cases
5. Add to integration test if applicable

### Debugging Tests
```bash
# Run with debug logging
DEBUG=pw:api pnpm test:web

# Run specific test with UI
pnpm test:web:ui --grep "test name"

# Run with headed browser for visual debugging
pnpm test:web:headed
```

### Test Data
- Tests use mock data where appropriate
- State changes are verified after actions
- Tests are isolated and don't depend on each other

## Continuous Integration

### GitHub Actions
- Tests run on push to main branch
- Tests run on pull requests
- Multiple browser testing
- Screenshot artifacts on failure

### Local Development
- Run tests before committing
- Verify all examples work locally
- Check cross-browser compatibility

## Troubleshooting

### Common Issues

#### WASM Not Loading
- Ensure examples are built: `cargo build`
- Check browser console for errors
- Verify Trunk configuration

#### Tests Failing
- Check if examples are served on port 8000
- Verify `data-testid` attributes exist
- Check browser compatibility

#### Browser Issues
- Reinstall browsers: `pnpm install:browsers`
- Clear browser cache
- Check browser version compatibility

### Debug Commands
```bash
# Check if examples are built
ls examples/*/dist/

# Check if server is running
curl http://localhost:8000/examples/counter/dist/

# Run tests with verbose output
pnpm playwright test --debug
```

## Contributing

When adding new examples or modifying existing ones:

1. **Add Playwright tests** for new functionality
2. **Update integration tests** to include new examples
3. **Maintain test coverage** for all user-facing features
4. **Follow naming conventions** for consistency
5. **Add appropriate assertions** for state changes
6. **Test edge cases** and error conditions

## Performance

- Tests run in parallel for efficiency
- WASM loading is optimized with appropriate timeouts
- Screenshots only captured on failure
- Minimal test data to reduce execution time
