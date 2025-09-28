/// Action trait for state changes
pub trait Action<C, E>: Send + Sync {
    fn execute(&self, context: &mut C, event: &E);
    fn name(&self) -> &str;
    fn description(&self) -> String {
        self.name().to_string()
    }
    fn has_side_effects(&self) -> bool {
        true
    }
    fn clone_action(&self) -> Box<dyn Action<C, E>>;
}
