use std::sync::Arc;

use crate::State;

pub struct GlobalContext {
    version: String,
    state: Arc<State>,
}

impl GlobalContext {
    pub(crate) fn new(version: String, state: Arc<State>) -> Self {
        Self { version, state }
    }

    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    #[must_use]
    /// Get a reference to the extensions state container.
    pub fn state(&self) -> &State {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
