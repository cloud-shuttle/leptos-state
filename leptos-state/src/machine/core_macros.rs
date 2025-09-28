/// Helper macros for machine creation
#[macro_export]
macro_rules! machine_state {
    ($name:expr, $state_type:expr) => {
        StateNode::new($name.to_string(), $state_type)
    };
}

#[macro_export]
macro_rules! machine_transition {
    ($state:expr, $event:expr, $target:expr) => {
        $state.add_transition($event, $target.to_string())
    };
}
