use std::sync::Arc;

use crate::{ContextError, ContextState, State};

/// Contains information about the extension
pub struct GlobalContext {
    version: String,
    state: Arc<State>,
}

impl GlobalContext {
    pub(crate) fn new(version: String, state: Arc<State>) -> Self {
        Self { version, state }
    }

    #[must_use]
    /// Version of the Arma extension
    pub fn version(&self) -> &str {
        &self.version
    }
}

impl ContextState for GlobalContext {
    fn get<T>(&self) -> Result<&T, ContextError>
    where
        T: Send + Sync + 'static,
    {
        self.state.try_get().ok_or(ContextError::NoGroupState)
    }

    fn set<T>(&self, value: T) -> bool
    where
        T: Send + Sync + 'static,
    {
        self.state.set(value)
    }
}
