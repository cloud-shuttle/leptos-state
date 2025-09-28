/// Guard trait for conditional transitions
pub trait Guard<C, E>: Send + Sync {
    fn check(&self, context: &C, event: &E) -> bool;
    fn name(&self) -> &str;
}
