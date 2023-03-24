use std::sync::Arc;

use crate::State;

/// Contains information about the current group
pub struct GroupContext {
    state: Arc<State>,
}

impl GroupContext {
    pub(crate) fn new(state: Arc<State>) -> Self {
        Self { state }
    }

    #[must_use]
    /// Group state container
    pub fn state(&self) -> &State {
        &self.state
    }
}
