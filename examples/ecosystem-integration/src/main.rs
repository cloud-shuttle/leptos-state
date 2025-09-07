//! Ecosystem Integration Example
//! 
//! This example demonstrates how leptos-state can work with other
//! companion crates in the Leptos ecosystem. While the actual
//! companion crates are not yet available, this shows the intended
//! integration patterns.

use leptos::*;
use leptos_state::*;

// Mock types for demonstration - these would come from the actual crates
mod mock_crates {
    // leptos-ws-pro integration
    pub struct WebSocketSync;
    impl WebSocketSync {
        pub fn new(_url: &str) -> Self { Self }
        pub fn sync_state<T>(&self, _state: &T) { /* sync logic */ }
    }

    // leptos-forms integration
    pub trait Form {
        fn validate(&self) -> bool;
    }

    // radix-leptos integration
    pub struct Dialog;
    pub struct DialogContent;
    pub struct DialogTrigger;

    // leptos-query integration
    pub struct Query<T> {
        pub data: Option<T>,
        pub loading: bool,
        pub error: Option<String>,
    }

    impl<T> Query<T> {
        pub fn new() -> Self {
            Self {
                data: None,
                loading: false,
                error: None,
            }
        }
    }
}

use mock_crates::*;

// Application state
#[derive(Clone, Debug, PartialEq)]
struct AppState {
    user: Option<User>,
    settings: Settings,
    notifications: Vec<Notification>,
}

#[derive(Clone, Debug, PartialEq)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Clone, Debug, PartialEq)]
struct Settings {
    theme: String,
    notifications_enabled: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct Notification {
    id: u32,
    message: String,
    read: bool,
}

// Form state for user profile
#[derive(Clone, Debug, PartialEq, Form)]
struct UserForm {
    name: String,
    email: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            user: None,
            settings: Settings {
                theme: "light".to_string(),
                notifications_enabled: true,
            },
            notifications: vec![],
        }
    }
}

// Main application component
#[component]
fn App() -> impl IntoView {
    // State management with leptos-state
    let (state, set_state) = use_store::<AppState>();
    
    // WebSocket synchronization (leptos-ws-pro)
    let ws_sync = WebSocketSync::new("ws://localhost:8080/ws");
    
    // Query for user data (leptos-query)
    let user_query = Query::<User>::new();
    
    // Form state (leptos-forms)
    let (form_state, set_form) = use_store::<UserForm>();
    
    // Modal state for settings
    let (modal_open, set_modal_open) = create_signal(false);

    // Sync state with WebSocket
    create_effect(move |_| {
        if let Some(user) = state.get().user.as_ref() {
            ws_sync.sync_state(user);
        }
    });

    // Handle form submission
    let handle_form_submit = move |_| {
        let form = form_state.get();
        if form.validate() {
            set_state.update(|state| {
                state.user = Some(User {
                    id: 1,
                    name: form.name.clone(),
                    email: form.email.clone(),
                });
            });
        }
    };

    view! {
        <div class="app">
            <header>
                <h1>"Leptos State Ecosystem Demo"</h1>
                <nav>
                    <button on:click=move |_| set_modal_open.set(true)>
                        "Settings"
                    </button>
                </nav>
            </header>

            <main>
                // User profile section
                <section class="profile">
                    <h2>"User Profile"</h2>
                    {move || {
                        if let Some(user) = state.get().user.as_ref() {
                            view! {
                                <div class="user-info">
                                    <p>"Name: " {user.name.clone()}</p>
                                    <p>"Email: " {user.email.clone()}</p>
                                </div>
                            }.into_view()
                        } else {
                            view! {
                                <UserForm 
                                    form_state=form_state
                                    set_form=set_form
                                    on_submit=handle_form_submit
                                />
                            }.into_view()
                        }
                    }}
                </section>

                // Notifications section
                <section class="notifications">
                    <h2>"Notifications"</h2>
                    <div class="notification-list">
                        {move || {
                            state.get().notifications.iter().map(|notification| {
                                view! {
                                    <div class="notification" class:read=notification.read>
                                        <p>{notification.message.clone()}</p>
                                        <button on:click=move |_| {
                                            set_state.update(|state| {
                                                if let Some(n) = state.notifications.iter_mut()
                                                    .find(|n| n.id == notification.id) {
                                                    n.read = true;
                                                }
                                            });
                                        }>
                                            "Mark as Read"
                                        </button>
                                    </div>
                                }
                            }).collect_view()
                        }}
                    </div>
                </section>

                // Query status (leptos-query)
                <section class="query-status">
                    <h2>"Data Status"</h2>
                    {move || {
                        if user_query.loading {
                            view! { <p>"Loading user data..."</p> }.into_view()
                        } else if let Some(error) = &user_query.error {
                            view! { <p class="error">"Error: " {error.clone()}</p> }.into_view()
                        } else if let Some(data) = &user_query.data {
                            view! { <p>"Data loaded: " {format!("{:?}", data)}</p> }.into_view()
                        } else {
                            view! { <p>"No data available"</p> }.into_view()
                        }
                    }}
                </section>
            </main>

            // Settings modal (radix-leptos style)
            {move || {
                if modal_open.get() {
                    view! {
                        <div class="modal-overlay" on:click=move |_| set_modal_open.set(false)>
                            <div class="modal-content" on:click=|ev| ev.stop_propagation()>
                                <h3>"Settings"</h3>
                                <div class="setting">
                                    <label>
                                        <input 
                                            type="checkbox"
                                            checked=move || state.get().settings.notifications_enabled
                                            on:change=move |ev| {
                                                set_state.update(|state| {
                                                    state.settings.notifications_enabled = 
                                                        event_target_checked(&ev);
                                                });
                                            }
                                        />
                                        "Enable Notifications"
                                    </label>
                                </div>
                                <div class="setting">
                                    <label>
                                        "Theme: "
                                        <select 
                                            value=move || state.get().settings.theme.clone()
                                            on:change=move |ev| {
                                                set_state.update(|state| {
                                                    state.settings.theme = 
                                                        event_target_value(&ev);
                                                });
                                            }
                                        >
                                            <option value="light">"Light"</option>
                                            <option value="dark">"Dark"</option>
                                            <option value="auto">"Auto"</option>
                                        </select>
                                    </label>
                                </div>
                                <button on:click=move |_| set_modal_open.set(false)>
                                    "Close"
                                </button>
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }
            }}
        </div>
    }
}

// User form component (leptos-forms integration)
#[component]
fn UserForm(
    form_state: ReadSignal<UserForm>,
    set_form: WriteSignal<UserForm>,
    on_submit: impl Fn() + 'static,
) -> impl IntoView {
    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            on_submit();
        }>
            <div class="form-group">
                <label for="name">"Name:"</label>
                <input
                    id="name"
                    type="text"
                    value=move || form_state.get().name
                    on:input=move |ev| {
                        set_form.update(|form| {
                            form.name = event_target_value(&ev);
                        });
                    }
                    required
                />
            </div>
            <div class="form-group">
                <label for="email">"Email:"</label>
                <input
                    id="email"
                    type="email"
                    value=move || form_state.get().email
                    on:input=move |ev| {
                        set_form.update(|form| {
                            form.email = event_target_value(&ev);
                        });
                    }
                    required
                />
            </div>
            <button type="submit">"Save Profile"</button>
        </form>
    }
}

fn main() {
    mount_to_body(|| {
        view! {
            <App />
        }
    })
}
