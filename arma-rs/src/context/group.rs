use std::sync::Arc;

use crate::{ContextError, ContextState, State};

/// Contains information about the current group
pub struct GroupContext {
    state: Arc<State>,
}

impl GroupContext {
    pub(crate) fn new(state: Arc<State>) -> Self {
        Self { state }
    }
}

impl ContextState for GroupContext {
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
