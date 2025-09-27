// Generated State Machine Code
// This file was automatically generated

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct StateContext {
    pub id: String,
    pub data: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StateEvent {
    Start,
    Stop,
    Pause,
    Resume,
}

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    idle,
    playing,
}

pub struct StateMachine {
    current_state: State,
    context: StateContext,
    transitions: HashMap<(State, StateEvent), State>,
}

impl StateMachine {
    pub fn new() -> Self {
        let mut transitions = HashMap::new();
        transitions.insert((State::idle, StateEvent::Start), State::running);
        transitions.insert((State::running, StateEvent::Pause), State::paused);
        transitions.insert((State::paused, StateEvent::Resume), State::running);
        transitions.insert((State::running, StateEvent::Stop), State::idle);
        Self {
            current_state: State::idle,
            context: StateContext {
                id: uuid::Uuid::new_v4().to_string(),
                data: HashMap::new(),
            },
            transitions,
        }
    }

    pub fn transition(&mut self, event: StateEvent) -> Result<State, String> {
        let key = (self.current_state.clone(), event);
        if let Some(new_state) = self.transitions.get(&key) {
            self.current_state = new_state.clone();
            Ok(new_state.clone())
        } else {
            Err("Invalid transition".to_string())
        }
    }

    pub fn current_state(&self) -> &State {
        &self.current_state
    }

    pub fn context(&self) -> &StateContext {
        &self.context
    }
}
