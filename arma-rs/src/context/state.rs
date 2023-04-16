use crate::ContextError;

/// A trait for accessing state values
pub trait ContextState {
    /// Get a reference to a state value
    fn get<T>(&self) -> Result<&T, ContextError>
    where
        T: Send + Sync + 'static;

    /// Set a state value
    fn set<T>(&self, value: T) -> bool
    where
        T: Send + Sync + 'static;
}
