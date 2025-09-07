# Leptos State Ecosystem

This document outlines the companion crates available for enhancing `leptos-state` applications and their potential integration opportunities.

## Available Companion Crates

### 🌐 **leptos-ws-pro**
**WebSocket Professional** - Advanced WebSocket management for real-time applications.

**Potential Integration:**
- Real-time state synchronization across clients
- Live collaboration features
- Real-time notifications and updates
- Multi-user state management

**Use Cases:**
- Collaborative editing applications
- Live dashboards and monitoring
- Real-time chat applications
- Multi-player games

### 🔄 **leptos-sync**
**State Synchronization** - Advanced state synchronization utilities.

**Potential Integration:**
- Cross-component state synchronization
- Server-client state consistency
- Optimistic updates with conflict resolution
- State persistence and recovery

**Use Cases:**
- Offline-first applications
- Multi-tab synchronization
- Server state management
- Conflict resolution in collaborative apps

### 🎨 **radix-leptos**
**UI Component Library** - Accessible, customizable UI components inspired by Radix UI.

**Potential Integration:**
- State-aware UI components
- Form components with built-in state management
- Modal and dialog state management
- Navigation state handling

**Use Cases:**
- Professional web applications
- Accessible user interfaces
- Design system implementation
- Complex form handling

### 📝 **leptos-forms**
**Form Management** - Derive forms directly from Rust structs.

**Potential Integration:**
- Form state management with `leptos-state`
- Validation state handling
- Form persistence and recovery
- Multi-step form workflows

**Use Cases:**
- Complex form applications
- Data entry systems
- User registration and profiles
- Configuration interfaces

### 🔍 **leptos-query**
**Async State Management** - Data fetching, caching, and synchronization.

**Potential Integration:**
- Server state management
- Cache invalidation with local state
- Optimistic updates
- Background data synchronization

**Use Cases:**
- Data-heavy applications
- API-driven interfaces
- Real-time data updates
- Offline data management

## Integration Patterns

### 1. **State Machine + WebSocket**
```rust
use leptos_state::*;
use leptos_ws_pro::*;

// Real-time state machine updates
let machine = create_machine::<GameState, GameEvent>()
    .with_websocket_sync("ws://game-server")
    .build();
```

### 2. **Form State + Validation**
```rust
use leptos_state::*;
use leptos_forms::*;

#[derive(Form, Clone)]
struct UserForm {
    name: String,
    email: String,
}

// Form state with validation
let (form_state, set_form) = use_store_with_validation::<UserForm>();
```

### 3. **Query Cache + Local State**
```rust
use leptos_state::*;
use leptos_query::*;

// Combine server state with local state
let (local_state, set_local) = use_store::<AppState>();
let query = use_query(|| fetch_user_data(), |data| {
    set_local.update(|state| state.user = data);
});
```

### 4. **UI Components + State**
```rust
use leptos_state::*;
use radix_leptos::*;

// State-aware UI components
let (modal_state, set_modal) = use_store::<ModalState>();

view! {
    <Dialog open=move || modal_state.get().is_open>
        <DialogContent>
            // Modal content with state management
        </DialogContent>
    </Dialog>
}
```

## Future Integration Opportunities

### 🚀 **Planned Integrations**

1. **State Machine + WebSocket Sync**
   - Real-time state synchronization
   - Conflict resolution strategies
   - Offline state management

2. **Form State Management**
   - Integrated form validation
   - Multi-step form workflows
   - Form persistence

3. **Query Integration**
   - Server state synchronization
   - Cache invalidation strategies
   - Optimistic updates

4. **UI Component Library**
   - State-aware components
   - Built-in state management
   - Accessibility features

### 📋 **Integration Checklist**

- [ ] **leptos-ws-pro**: Real-time state synchronization
- [ ] **leptos-sync**: Cross-component state management
- [ ] **radix-leptos**: State-aware UI components
- [ ] **leptos-forms**: Form state management
- [ ] **leptos-query**: Server state integration

## Getting Started

### Installation
```toml
[dependencies]
leptos-state = "1.0.0"
leptos-ws-pro = "0.1"      # When available
leptos-sync = "0.1"        # When available
radix-leptos = "0.1"       # When available
leptos-forms = "0.1"       # When available
leptos-query = "0.1"       # When available
```

### Basic Setup
```rust
use leptos_state::*;
use leptos::*;

fn main() {
    mount_to_body(|| {
        view! {
            <App />
        }
    })
}
```

## Contributing

We welcome contributions to enhance the integration between `leptos-state` and these companion crates. Please see our [Contributing Guide](CONTRIBUTING.md) for more information.

## Resources

- [leptos-state Documentation](https://docs.rs/leptos-state)
- [Leptos Framework](https://leptos.dev)
- [Radix UI](https://www.radix-ui.com)
- [Tanstack Query](https://tanstack.com/query)

---

*Last updated: September 2025*
