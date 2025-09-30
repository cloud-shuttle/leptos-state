//! Coverage tracking for state machine tests

use super::*;
use std::collections::{HashMap, HashSet};

/// Coverage tracker for tests
pub struct CoverageTracker {
    /// States that have been covered
    pub covered_states: HashSet<String>,
    /// Transitions that have been covered
    pub covered_transitions: HashSet<String>,
    /// Guards that have been covered
    pub covered_guards: HashSet<String>,
    /// Actions that have been covered
    pub covered_actions: HashSet<String>,
    /// Coverage statistics
    pub stats: CoverageStats,
}

/// Coverage statistics
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageStats {
    /// Total number of states
    pub total_states: usize,
    /// Total number of transitions
    pub total_transitions: usize,
    /// Total number of guards
    pub total_guards: usize,
    /// Total number of actions
    pub total_actions: usize,
    /// States covered
    pub states_covered: usize,
    /// Transitions covered
    pub transitions_covered: usize,
    /// Guards covered
    pub guards_covered: usize,
    /// Actions covered
    pub actions_covered: usize,
}

impl CoverageTracker {
    /// Create a new coverage tracker
    pub fn new() -> Self {
        Self {
            covered_states: HashSet::new(),
            covered_transitions: HashSet::new(),
            covered_guards: HashSet::new(),
            covered_actions: HashSet::new(),
            stats: CoverageStats {
                total_states: 0,
                total_transitions: 0,
                total_guards: 0,
                total_actions: 0,
                states_covered: 0,
                transitions_covered: 0,
                guards_covered: 0,
                actions_covered: 0,
            },
        }
    }

    /// Record that a state was covered
    pub fn record_state(&mut self, state: &str) {
        self.covered_states.insert(state.to_string());
        self.stats.states_covered = self.covered_states.len();
    }

    /// Record that a transition was covered
    pub fn record_transition(&mut self, from: &str, to: &str, event: &str) {
        let transition_key = format!("{} -> {} ({})", from, to, event);
        self.covered_transitions.insert(transition_key);
        self.stats.transitions_covered = self.covered_transitions.len();
    }

    /// Record that a guard was covered
    pub fn record_guard(&mut self, guard: &str) {
        self.covered_guards.insert(guard.to_string());
        self.stats.guards_covered = self.covered_guards.len();
    }

    /// Record that an action was covered
    pub fn record_action(&mut self, action: &str) {
        self.covered_actions.insert(action.to_string());
        self.stats.actions_covered = self.covered_actions.len();
    }

    /// Get coverage percentage for states
    pub fn state_coverage(&self) -> f64 {
        if self.stats.total_states == 0 {
            0.0
        } else {
            self.stats.states_covered as f64 / self.stats.total_states as f64
        }
    }

    /// Get coverage percentage for transitions
    pub fn transition_coverage(&self) -> f64 {
        if self.stats.total_transitions == 0 {
            0.0
        } else {
            self.stats.transitions_covered as f64 / self.stats.total_transitions as f64
        }
    }

    /// Get coverage percentage for guards
    pub fn guard_coverage(&self) -> f64 {
        if self.stats.total_guards == 0 {
            0.0
        } else {
            self.stats.guards_covered as f64 / self.stats.total_guards as f64
        }
    }

    /// Get coverage percentage for actions
    pub fn action_coverage(&self) -> f64 {
        if self.stats.total_actions == 0 {
            0.0
        } else {
            self.stats.actions_covered as f64 / self.stats.total_actions as f64
        }
    }

    /// Get overall coverage percentage
    pub fn overall_coverage(&self) -> f64 {
        let state_cov = self.state_coverage();
        let transition_cov = self.transition_coverage();
        let guard_cov = self.guard_coverage();
        let action_cov = self.action_coverage();

        (state_cov + transition_cov + guard_cov + action_cov) / 4.0
    }

    /// Set total counts for coverage calculation
    pub fn set_totals(&mut self, states: usize, transitions: usize, guards: usize, actions: usize) {
        self.stats.total_states = states;
        self.stats.total_transitions = transitions;
        self.stats.total_guards = guards;
        self.stats.total_actions = actions;
    }

    /// Get uncovered states
    pub fn uncovered_states(&self, all_states: &[String]) -> Vec<String> {
        all_states
            .iter()
            .filter(|state| !self.covered_states.contains(*state))
            .cloned()
            .collect()
    }

    /// Get uncovered transitions
    pub fn uncovered_transitions(&self, all_transitions: &[String]) -> Vec<String> {
        all_transitions
            .iter()
            .filter(|transition| !self.covered_transitions.contains(*transition))
            .cloned()
            .collect()
    }

    /// Get uncovered guards
    pub fn uncovered_guards(&self, all_guards: &[String]) -> Vec<String> {
        all_guards
            .iter()
            .filter(|guard| !self.covered_guards.contains(*guard))
            .cloned()
            .collect()
    }

    /// Get uncovered actions
    pub fn uncovered_actions(&self, all_actions: &[String]) -> Vec<String> {
        all_actions
            .iter()
            .filter(|action| !self.covered_actions.contains(*action))
            .cloned()
            .collect()
    }

    /// Generate a coverage report
    pub fn generate_report(&self) -> CoverageReport {
        CoverageReport {
            stats: self.stats.clone(),
            state_coverage: self.state_coverage(),
            transition_coverage: self.transition_coverage(),
            guard_coverage: self.guard_coverage(),
            action_coverage: self.action_coverage(),
            overall_coverage: self.overall_coverage(),
            covered_states: self.covered_states.iter().cloned().collect(),
            covered_transitions: self.covered_transitions.iter().cloned().collect(),
            covered_guards: self.covered_guards.iter().cloned().collect(),
            covered_actions: self.covered_actions.iter().cloned().collect(),
        }
    }
}

/// Coverage report
#[derive(Debug, Clone, PartialEq)]
pub struct CoverageReport {
    /// Coverage statistics
    pub stats: CoverageStats,
    /// State coverage percentage
    pub state_coverage: f64,
    /// Transition coverage percentage
    pub transition_coverage: f64,
    /// Guard coverage percentage
    pub guard_coverage: f64,
    /// Action coverage percentage
    pub action_coverage: f64,
    /// Overall coverage percentage
    pub overall_coverage: f64,
    /// Covered states
    pub covered_states: Vec<String>,
    /// Covered transitions
    pub covered_transitions: Vec<String>,
    /// Covered guards
    pub covered_guards: Vec<String>,
    /// Covered actions
    pub covered_actions: Vec<String>,
}

impl CoverageReport {
    /// Check if coverage meets threshold
    pub fn meets_threshold(&self, threshold: f64) -> bool {
        self.overall_coverage >= threshold
    }

    /// Get coverage summary
    pub fn summary(&self) -> String {
        format!(
            "Coverage Summary:\n\
            States: {:.1}% ({}/{})\n\
            Transitions: {:.1}% ({}/{})\n\
            Guards: {:.1}% ({}/{})\n\
            Actions: {:.1}% ({}/{})\n\
            Overall: {:.1}%",
            self.state_coverage * 100.0,
            self.stats.states_covered,
            self.stats.total_states,
            self.transition_coverage * 100.0,
            self.stats.transitions_covered,
            self.stats.total_transitions,
            self.guard_coverage * 100.0,
            self.stats.guards_covered,
            self.stats.total_guards,
            self.action_coverage * 100.0,
            self.stats.actions_covered,
            self.stats.total_actions,
            self.overall_coverage * 100.0
        )
    }
}
