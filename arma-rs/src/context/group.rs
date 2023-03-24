use std::sync::Arc;

use crate::State;

pub struct GroupContext {
    state: Arc<State>,
}

impl GroupContext {
    pub(crate) fn new(state: Arc<State>) -> Self {
        Self { state }
    }

    #[must_use]
    pub fn state(&self) -> &State {
        &self.state
    }
}
