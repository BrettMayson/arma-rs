//! Contextual execution information.

use std::{fmt::Debug, sync::Arc};

use crossbeam_queue::SegQueue;

use crate::{IntoArma, Value};

mod arma;
mod global;
mod group;
mod state;

pub use self::state::ContextState;
pub use arma::*;
pub use global::GlobalContext;
pub use group::GroupContext;

/// Contains information about the current execution context
pub struct Context {
    global: GlobalContext,
    group: Option<GroupContext>,
    arma: Option<ArmaContext>,
    queue: Arc<SegQueue<(String, String, Option<Value>)>>,
    buffer_size: usize,
}

impl Context {
    pub(crate) fn new(
        global: GlobalContext,
        arma: Option<ArmaContext>,
        queue: Arc<SegQueue<(String, String, Option<Value>)>>,
    ) -> Self {
        Self {
            global,
            group: None,
            arma,
            queue,
            buffer_size: 0,
        }
    }

    pub(crate) fn with_group_ctx(mut self, ctx: GroupContext) -> Self {
        self.group = Some(ctx);
        self
    }

    pub(crate) const fn with_buffer_size(mut self, buffer_size: usize) -> Self {
        self.buffer_size = buffer_size;
        self
    }

    #[must_use]
    /// Global context
    pub const fn global(&self) -> &GlobalContext {
        &self.global
    }

    /// Group context, only provided in called commands
    pub fn group(&self) -> Result<&GroupContext, ContextError> {
        self.group.as_ref().ok_or(ContextError::NoGroupContext)
    }

    #[must_use]
    /// Context automatically provided by Arma. Supported since Arma version 2.11.
    pub const fn arma(&self) -> Option<&ArmaContext> {
        self.arma.as_ref()
    }

    #[must_use]
    /// Returns the length in bytes of the output buffer.
    /// This is the maximum size of the data that can be returned by the extension.
    pub const fn buffer_len(&self) -> usize {
        if self.buffer_size == 0 {
            0
        } else {
            self.buffer_size - 1
        }
    }

    /// Sends a callback with data into Arma
    /// <https://community.bistudio.com/wiki/Arma_3:_Mission_Event_Handlers#ExtensionCallback>
    #[deprecated(
        since = "1.8.0",
        note = "Use `callback_data` instead. This function may be removed in future versions."
    )]
    pub fn callback<V>(&self, name: &str, func: &str, data: Option<V>)
    where
        V: IntoArma,
    {
        self.queue
            .push((name.to_string(), func.to_string(), Some(data.to_arma())));
    }

    /// Sends a callback with data into Arma
    /// <https://community.bistudio.com/wiki/Arma_3:_Mission_Event_Handlers#ExtensionCallback>
    pub fn callback_data<V>(&self, name: &str, func: &str, data: V)
    where
        V: IntoArma,
    {
        self.queue
            .push((name.to_string(), func.to_string(), Some(data.to_arma())));
    }

    /// Sends a callback without data into Arma
    /// <https://community.bistudio.com/wiki/Arma_3:_Mission_Event_Handlers#ExtensionCallback>
    pub fn callback_null(&self, name: &str, func: &str) {
        self.queue.push((name.to_string(), func.to_string(), None));
    }
}

/// Errors that can occur when trying to access context information
pub enum ContextError {
    /// The group context is not available
    NoGroupContext,
    /// The type is not in the state
    NotInState,
}

impl ToString for ContextError {
    fn to_string(&self) -> String {
        match self {
            Self::NoGroupContext => "No group context available".to_string(),
            Self::NotInState => "Type not in state".to_string(),
        }
    }
}

impl IntoArma for ContextError {
    fn to_arma(&self) -> Value {
        Value::String(self.to_string())
    }
}

impl Debug for ContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::State;

    #[test]
    fn context_buffer_len_zero() {
        let ctx = Context::new(
            GlobalContext::new(String::new(), Arc::new(State::default())),
            None,
            Arc::new(SegQueue::new()),
        );
        assert_eq!(ctx.buffer_len(), 0);
    }

    #[test]
    fn context_buffer_len() {
        let ctx = Context::new(
            GlobalContext::new(String::new(), Arc::new(State::default())),
            None,
            Arc::new(SegQueue::new()),
        )
        .with_buffer_size(100);
        assert_eq!(ctx.buffer_len(), 99);
    }
}
