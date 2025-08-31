# Todo App Example

A comprehensive Todo application built with Leptos and `leptos-state`, demonstrating advanced state management patterns, state machines, and modern UI design.

## Features

### 🎯 Core Functionality
- **CRUD Operations**: Create, read, update, and delete todos
- **Priority Levels**: Low, Medium, High, and Urgent priorities with color coding
- **Tags System**: Organize todos with custom tags
- **Due Dates**: Set and track due dates with overdue detection
- **Completion Tracking**: Mark todos as complete with visual feedback

### 🔍 Advanced Filtering & Search
- **Multiple Filters**: All, Active, Completed, Overdue, Priority-based, Tag-based
- **Real-time Search**: Search across titles, descriptions, and tags
- **Sorting Options**: Sort by creation date, update date, due date, priority, or title
- **Bulk Operations**: Select multiple todos for batch actions

### 🎨 Modern UI/UX
- **Responsive Design**: Works seamlessly on desktop, tablet, and mobile
- **Dark/Light Theme**: Toggle between themes with system preference detection
- **Smooth Animations**: CSS transitions and hover effects
- **Accessible**: Keyboard navigation and screen reader support
- **Progressive Enhancement**: Works without JavaScript for basic functionality

### 💾 Data Persistence
- **Local Storage**: Automatic saving to browser's local storage
- **Auto-save**: Real-time persistence with configurable settings
- **Export/Import**: JSON export and import functionality
- **Data Management**: Clear all data with confirmation

### 🚀 Performance Features
- **Reactive Updates**: Efficient re-rendering with Leptos signals
- **Memoization**: Computed values for filtered and sorted lists
- **Lazy Loading**: Components load only when needed
- **Optimized Rendering**: Minimal DOM updates

### 🔧 State Management
- **Centralized Store**: Single source of truth for application state
- **State Machines**: Complex workflows for editing, bulk operations, and sync
- **Reactive Signals**: Real-time UI updates based on state changes
- **Context Providers**: Clean dependency injection

## Architecture

### State Management Pattern
The app uses a centralized store pattern with reactive signals:

```rust
pub struct TodoStore {
    state: ReadSignal<AppState>,
    dispatch: WriteSignal<AppState>,
}
```

### State Machines
Complex workflows are managed with state machines:

- **Todo Edit Machine**: Handles editing workflow with validation
- **Bulk Operations Machine**: Manages batch operations with progress tracking
- **Sync Machine**: Handles data synchronization states
- **Search Machine**: Manages search states and debouncing

### Component Structure
```
TodoApp
├── Header (Search, Settings)
├── Sidebar (Filters, Tags, Actions)
└── Main Content
    ├── Stats Display
    ├── Filter Bar
    ├── Todo List
    └── Add Todo Form
```

## Getting Started

### Prerequisites
- Rust 1.70+
- Cargo
- Web browser with WebAssembly support

### Installation

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd leptos-state/examples/todo-app
   ```

2. **Install dependencies**:
   ```bash
   cargo build
   ```

3. **Run the development server**:
   ```bash
   cargo run
   ```

4. **Open your browser** and navigate to `http://localhost:3000`

### Building for Production

```bash
cargo build --release
```

## Usage

### Creating Todos
1. Use the "Add New Todo" form at the bottom of the page
2. Enter a title (required)
3. Optionally add description, priority, and tags
4. Click "Add Todo" to create

### Managing Todos
- **Complete**: Check the checkbox to mark as complete
- **Edit**: Click the edit button (✏️) to modify details
- **Delete**: Click the delete button (🗑️) to remove
- **Expand**: Click on a todo to view additional details

### Filtering and Search
- **Quick Filters**: Use sidebar buttons for common filters
- **Search**: Use the search bar to find specific todos
- **Advanced Filters**: Use the filter bar for precise filtering
- **Sorting**: Change sort order and direction

### Bulk Operations
1. Select multiple todos using checkboxes
2. Use bulk action buttons to:
   - Complete selected todos
   - Delete selected todos
   - Clear all completed todos

### Settings
Access settings via the gear icon (⚙️) in the header:
- Toggle completed todos visibility
- Change theme (Light/Dark/Auto)
- Toggle auto-save
- Clear all data

## Technical Implementation

### State Management
The app demonstrates several state management patterns:

1. **Centralized Store**: All application state in one place
2. **Reactive Signals**: Real-time UI updates
3. **Computed Values**: Derived state for filtered/sorted lists
4. **Context Providers**: Clean dependency injection

### State Machines
Complex workflows use state machines for predictable behavior:

```rust
// Example: Todo Edit Machine
MachineBuilder::new()
    .initial("idle")
    .state("idle")
        .on(TodoEditEvent::StartEdit(Uuid::nil()), "editing")
    .state("editing")
        .on(TodoEditEvent::Save, "saving")
        .on(TodoEditEvent::Cancel, "idle")
    .build()
```

### Performance Optimizations
- **Memoization**: Expensive computations are memoized
- **Efficient Updates**: Only changed components re-render
- **Lazy Loading**: Components load on demand
- **Debounced Search**: Search input is debounced for performance

### Data Persistence
- **Local Storage**: Automatic saving to browser storage
- **Serialization**: Full state serialization with serde
- **Error Handling**: Graceful handling of storage errors
- **Migration**: Future-proof data format

## Customization

### Adding New Features
1. **New Todo Fields**: Add fields to the `Todo` struct
2. **New Filters**: Extend the `TodoFilter` enum
3. **New Actions**: Add to the `TodoEvent` enum
4. **New Components**: Create new UI components

### Styling
The app uses CSS custom properties for easy theming:
- Modify `:root` variables in `styles.css`
- Add new theme variants
- Customize component styles

### State Machines
Add new workflows by creating state machines:
1. Define context and events
2. Build the machine with `MachineBuilder`
3. Integrate with the main app

## Best Practices Demonstrated

### Code Organization
- **Separation of Concerns**: UI, state, and logic are separated
- **Modular Components**: Reusable, focused components
- **Type Safety**: Strong typing throughout the application
- **Error Handling**: Comprehensive error handling

### Performance
- **Efficient Rendering**: Minimal re-renders
- **Memory Management**: Proper cleanup and resource management
- **Bundle Size**: Optimized for production

### User Experience
- **Responsive Design**: Works on all screen sizes
- **Accessibility**: Keyboard navigation and screen readers
- **Progressive Enhancement**: Works without JavaScript
- **Error Recovery**: Graceful error handling

### Testing
- **Unit Tests**: Comprehensive test coverage
- **Integration Tests**: End-to-end functionality
- **Performance Tests**: Benchmarking and optimization

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [Leptos](https://leptos.dev/) - A full-stack, isomorphic Rust web framework
- State management powered by `leptos-state`
- Icons from Unicode emoji
- Design inspired by modern web applications
