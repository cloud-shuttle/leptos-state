use std::fmt;

/// Represents the hierarchical value of a state
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StateValue {
    /// Simple state (e.g., "idle")
    Simple(String),
    /// Compound state with child (e.g., "power.on")
    Compound {
        parent: String,
        child: Box<StateValue>,
    },
    /// Multiple parallel states
    Parallel(Vec<StateValue>),
}

impl StateValue {
    /// Create a simple state value
    pub fn simple(name: impl Into<String>) -> Self {
        Self::Simple(name.into())
    }

    /// Create a compound state value
    pub fn compound(parent: impl Into<String>, child: StateValue) -> Self {
        Self::Compound {
            parent: parent.into(),
            child: Box::new(child),
        }
    }

    /// Create parallel state values
    pub fn parallel(states: Vec<StateValue>) -> Self {
        Self::Parallel(states)
    }

    /// Check if this state matches a pattern
    pub fn matches(&self, pattern: &str) -> bool {
        match self {
            StateValue::Simple(name) => name == pattern || pattern == "*",
            StateValue::Compound { parent, child } => {
                if pattern == "*" {
                    return true;
                }

                if pattern == parent {
                    return true;
                }

                // Check for exact compound match (e.g., "power.on")
                if pattern.contains('.') {
                    let parts: Vec<&str> = pattern.split('.').collect();
                    if parts.len() == 2 && parts[0] == parent {
                        return child.matches(parts[1]);
                    }
                }

                // Check child recursively
                child.matches(pattern)
            }
            StateValue::Parallel(states) => {
                if pattern == "*" {
                    return true;
                }

                // Match if any parallel state matches
                states.iter().any(|state| state.matches(pattern))
            }
        }
    }

    /// Get the top-level state name
    pub fn top_level(&self) -> &str {
        match self {
            StateValue::Simple(name) => name,
            StateValue::Compound { parent, .. } => parent,
            StateValue::Parallel(states) => {
                if let Some(first) = states.first() {
                    first.top_level()
                } else {
                    "unknown"
                }
            }
        }
    }

    /// Check if this is a compound state
    pub fn is_compound(&self) -> bool {
        matches!(self, StateValue::Compound { .. })
    }

    /// Check if this is a parallel state
    pub fn is_parallel(&self) -> bool {
        matches!(self, StateValue::Parallel(_))
    }

    /// Get all leaf states (final nested states)
    pub fn leaf_states(&self) -> Vec<String> {
        match self {
            StateValue::Simple(name) => vec![name.clone()],
            StateValue::Compound { parent, child } => {
                let child_leaves = child.leaf_states();
                child_leaves
                    .into_iter()
                    .map(|leaf| format!("{}.{}", parent, leaf))
                    .collect()
            }
            StateValue::Parallel(states) => states.iter().flat_map(|s| s.leaf_states()).collect(),
        }
    }

    /// Convert to a dot-notation string
    pub fn to_string(&self) -> String {
        match self {
            StateValue::Simple(name) => name.clone(),
            StateValue::Compound { parent, child } => {
                format!("{}.{}", parent, child.to_string())
            }
            StateValue::Parallel(states) => {
                let state_strings: Vec<String> = states.iter().map(|s| s.to_string()).collect();
                format!("[{}]", state_strings.join(", "))
            }
        }
    }
}

impl fmt::Display for StateValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<&str> for StateValue {
    fn from(s: &str) -> Self {
        if s.contains('.') {
            let parts: Vec<&str> = s.split('.').collect();
            if parts.len() == 2 {
                StateValue::Compound {
                    parent: parts[0].to_string(),
                    child: Box::new(StateValue::Simple(parts[1].to_string())),
                }
            } else {
                // Handle more complex nesting
                let parent = parts[0].to_string();
                let child_path = parts[1..].join(".");
                StateValue::Compound {
                    parent,
                    child: Box::new(StateValue::from(child_path.as_str())),
                }
            }
        } else {
            StateValue::Simple(s.to_string())
        }
    }
}

impl From<String> for StateValue {
    fn from(s: String) -> Self {
        StateValue::from(s.as_str())
    }
}

/// History type for state machines
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HistoryType {
    /// Shallow history - remember only direct child state
    Shallow,
    /// Deep history - remember the entire state configuration
    Deep,
}

/// History state configuration
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HistoryState {
    pub history_type: HistoryType,
    pub target: Option<StateValue>,
}

impl HistoryState {
    pub fn shallow(target: Option<StateValue>) -> Self {
        Self {
            history_type: HistoryType::Shallow,
            target,
        }
    }

    pub fn deep(target: Option<StateValue>) -> Self {
        Self {
            history_type: HistoryType::Deep,
            target,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_state_matches_exact() {
        let state = StateValue::simple("idle");
        assert!(state.matches("idle"));
        assert!(!state.matches("running"));
        assert!(state.matches("*"));
    }

    #[test]
    fn compound_state_matches_patterns() {
        let state = StateValue::compound("power", StateValue::simple("on"));

        assert!(state.matches("power"));
        assert!(state.matches("power.on"));
        assert!(state.matches("on"));
        assert!(state.matches("*"));
        assert!(!state.matches("power.off"));
    }

    #[test]
    fn parallel_states_match_any_child() {
        let state = StateValue::parallel(vec![
            StateValue::simple("heating"),
            StateValue::simple("cooling"),
        ]);

        assert!(state.matches("heating"));
        assert!(state.matches("cooling"));
        assert!(state.matches("*"));
        assert!(!state.matches("idle"));
    }

    #[test]
    fn state_conversion_from_string() {
        let simple: StateValue = "idle".into();
        assert_eq!(simple, StateValue::simple("idle"));

        let compound: StateValue = "power.on".into();
        assert_eq!(
            compound,
            StateValue::compound("power", StateValue::simple("on"))
        );
    }

    #[test]
    fn leaf_states_collection() {
        let simple = StateValue::simple("idle");
        assert_eq!(simple.leaf_states(), vec!["idle"]);

        let compound = StateValue::compound("power", StateValue::simple("on"));
        assert_eq!(compound.leaf_states(), vec!["power.on"]);

        let parallel = StateValue::parallel(vec![
            StateValue::simple("heating"),
            StateValue::compound("cooling", StateValue::simple("active")),
        ]);
        assert_eq!(parallel.leaf_states(), vec!["heating", "cooling.active"]);
    }

    #[test]
    fn state_display_formatting() {
        let simple = StateValue::simple("idle");
        assert_eq!(simple.to_string(), "idle");

        let compound = StateValue::compound("power", StateValue::simple("on"));
        assert_eq!(compound.to_string(), "power.on");

        let parallel = StateValue::parallel(vec![
            StateValue::simple("heating"),
            StateValue::simple("cooling"),
        ]);
        assert_eq!(parallel.to_string(), "[heating, cooling]");
    }
}
