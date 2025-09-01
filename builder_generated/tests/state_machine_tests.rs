#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_machine_creation() {
        let machine = StateMachine::new();
        assert_eq!(*machine.current_state(), State::idle);
    }

    #[test]
    fn test_valid_transitions() {
        let mut machine = StateMachine::new();
        let result = machine.transition(StateEvent::Start);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), State::running);
        let result = machine.transition(StateEvent::Pause);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), State::paused);
        let result = machine.transition(StateEvent::Resume);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), State::running);
        let result = machine.transition(StateEvent::Stop);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), State::idle);
    }
}
