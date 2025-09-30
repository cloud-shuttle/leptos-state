//! Transition handling and execution logic

use super::*;
use crate::machine::states::StateValue;

/// Main transition method - handles all transition types
pub fn transition<C, E>(
    machine: &Machine<C, E, C>,
    state: &MachineStateImpl<C>,
    event: E,
) -> MachineStateImpl<C>
where
    C: Send + Sync + Clone + std::fmt::Debug + Default + 'static,
    E: Send + Clone + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static,
{
    match &state.value() {
        StateValue::Simple(id) => transition_simple(machine, state, id, event),
        StateValue::Compound { parent, child } => {
            transition_hierarchical(machine, state, parent, child, event)
        }
        StateValue::Parallel(states) => {
            // Handle parallel states by transitioning each active region
            let mut new_states = Vec::new();
            let mut context = state.context().clone();

            for parallel_state in states {
                let temp_state = MachineStateImpl {
                    value: parallel_state.clone(),
                    context: context.clone(),
                };
                let transitioned = transition(machine, &temp_state, event.clone());
                new_states.push(transitioned.value().clone());
                context = transitioned.context().clone();
            }

            MachineStateImpl {
                value: StateValue::Parallel(new_states),
                context,
            }
        }
    }
}

/// Handle simple state transitions
pub fn transition_simple<C, E>(
    machine: &Machine<C, E, C>,
    state: &MachineStateImpl<C>,
    state_id: &str,
    event: E,
) -> MachineStateImpl<C>
where
    C: Send + Sync + Clone + std::fmt::Debug + Default + 'static,
    E: Send + Clone + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static,
{
    if let Some(state_node) = machine.states.get(state_id) {
        // Look for a matching transition
        for transition in &state_node.transitions {
            if transition.event == event {
                // Check all guards
                let guards_pass = transition
                    .guards
                    .iter()
                    .all(|guard| guard.check(state.context(), &event));

                if guards_pass {
                    let mut new_context = state.context().clone();

                    // Execute transition actions
                    for action in &transition.actions {
                        action.execute(&mut new_context, &event);
                    }

                    // Execute exit actions for current state
                    for action in &state_node.exit_actions {
                        action.execute(&mut new_context, &event);
                    }

                    // Determine target state value (simple or compound)
                    let new_value = resolve_target_state(machine, &transition.target);

                    let new_state = MachineStateImpl {
                        value: new_value,
                        context: new_context,
                    };

                    // Execute entry actions for target state
                    return execute_entry_actions(machine, new_state, &transition.target, &event);
                }
            }
        }
    }

    // No valid transition found, return current state
    state.clone()
}

/// Handle hierarchical (compound) state transitions
pub fn transition_hierarchical<C, E>(
    machine: &Machine<C, E, C>,
    state: &MachineStateImpl<C>,
    parent_id: &str,
    child: &StateValue,
    event: E,
) -> MachineStateImpl<C>
where
    C: Send + Sync + Clone + std::fmt::Debug + Default + 'static,
    E: Send + Clone + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static,
{
    // First try child state transitions
    let child_state = MachineStateImpl {
        value: (*child).clone(),
        context: state.context().clone(),
    };

    let child_transitioned = transition(machine, &child_state, event.clone());

    // If child transitioned, update the compound state
    if child_transitioned.value() != child {
        return MachineStateImpl {
            value: StateValue::Compound {
                parent: parent_id.to_string(),
                child: Box::new(child_transitioned.value().clone()),
            },
            context: child_transitioned.context().clone(),
        };
    }

    // If child didn't transition, try parent transitions
    transition_simple(machine, state, parent_id, event)
}

/// Resolve a target state identifier to a StateValue
pub fn resolve_target_state<C, E>(machine: &Machine<C, E, C>, target: &str) -> StateValue
where
    C: Send + Sync + Clone + std::fmt::Debug + Default + 'static,
    E: Send + Clone + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static,
{
    if let Some(state_node) = machine.states.get(target) {
        if !state_node.child_states.is_empty() {
            // This is a compound state, resolve initial child
            if let Some(initial_child) = &state_node.initial_child {
                return StateValue::Compound {
                    parent: target.to_string(),
                    child: Box::new(resolve_target_state(machine, initial_child)),
                };
            }
        }
    }

    StateValue::Simple(target.to_string())
}

/// Execute entry actions for a target state
pub fn execute_entry_actions<C, E>(
    machine: &Machine<C, E, C>,
    mut state: MachineStateImpl<C>,
    target_id: &str,
    event: &E,
) -> MachineStateImpl<C>
where
    C: Send + Sync + Clone + std::fmt::Debug + Default + 'static,
    E: Send + Clone + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static,
{
    if let Some(target_node) = machine.states.get(target_id) {
        for action in &target_node.entry_actions {
            action.execute(&mut state.context, event);
        }
    }

    state
}

/// Get initial state for a machine
pub fn initial_state<C, E>(machine: &Machine<C, E, C>) -> MachineStateImpl<C>
where
    C: Send + Sync + Clone + std::fmt::Debug + Default + 'static,
    E: Send + Clone + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static,
{
    MachineStateImpl {
        value: StateValue::Simple(machine.initial.clone()),
        context: Default::default(),
    }
}

/// Create initial state with custom context
pub fn initial_with_context<C, E>(
    machine: &Machine<C, E, C>,
    context: C,
) -> MachineStateImpl<C>
where
    C: Send + Sync + Clone + std::fmt::Debug + Default + 'static,
    E: Send + Clone + std::fmt::Debug + PartialEq + Eq + std::hash::Hash + 'static,
{
    MachineStateImpl {
        value: StateValue::Simple(machine.initial.clone()),
        context,
    }
}
