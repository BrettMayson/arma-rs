//! Contextual execution information.

use crossbeam_channel::Sender;

use crate::{CallbackMessage, IntoArma};

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
    callback_tx: Sender<CallbackMessage>,
    global: GlobalContext,
    group: GroupContext,
    arma: Option<ArmaContext>,
    buffer_size: usize,
}

impl Context {
    pub(crate) fn new(
        callback_tx: Sender<CallbackMessage>,
        global: GlobalContext,
        group: GroupContext,
        arma: Option<ArmaContext>,
    ) -> Self {
        Self {
            callback_tx,
            global,
            group,
            arma,
            buffer_size: 0,
        }
    }

    pub(crate) fn with_group(mut self, ctx: GroupContext) -> Self {
        self.group = ctx;
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

    /// Group context, is equal to `GlobalContext` if the call is from the global scope.
    pub fn group(&self) -> &GroupContext {
        &self.group
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
        let _ = self.callback_tx.try_send(CallbackMessage::Call(
            name.to_string(),
            func.to_string(),
            Some(data.to_arma()),
        ));
    }

    /// Sends a callback with data into Arma
    /// <https://community.bistudio.com/wiki/Arma_3:_Mission_Event_Handlers#ExtensionCallback>
    pub fn callback_data<V>(&self, name: &str, func: &str, data: V)
    where
        V: IntoArma,
    {
        let _ = self.callback_tx.try_send(CallbackMessage::Call(
            name.to_string(),
            func.to_string(),
            Some(data.to_arma()),
        ));
    }

    /// Sends a callback without data into Arma
    /// <https://community.bistudio.com/wiki/Arma_3:_Mission_Event_Handlers#ExtensionCallback>
    pub fn callback_null(&self, name: &str, func: &str) {
        let _ = self.callback_tx.try_send(CallbackMessage::Call(
            name.to_string(),
            func.to_string(),
            None,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::State;
    use crossbeam_channel::unbounded;
    use std::sync::Arc;

    fn context() -> Context {
        let (tx, _) = unbounded();
        Context::new(
            tx,
            GlobalContext::new(String::new(), Arc::new(State::default())),
            GroupContext::new(Arc::new(State::default())),
            None,
        )
    }

    #[test]
    fn context_buffer_len_zero() {
        assert_eq!(context().buffer_len(), 0);
    }

    #[test]
    fn context_buffer_len() {
        assert_eq!(context().with_buffer_size(100).buffer_len(), 99);
    }
}
