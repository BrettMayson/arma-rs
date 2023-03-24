use std::sync::Arc;

use crate::State;

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

    #[must_use]
    /// Global state container
    pub fn state(&self) -> &State {
        &self.state
    }
}
