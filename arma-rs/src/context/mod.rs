//! Contextual execution information.

use crossbeam_channel::Sender;

use crate::{CallbackMessage, IntoArma, Value};

mod call;
mod global;
mod group;
mod manager;
mod state;

pub use self::state::ContextState;
pub use call::*;
pub use global::GlobalContext;
pub use group::GroupContext;
pub use manager::ArmaContextManager;

/// Contains information about the current execution context
pub struct Context {
    callback_tx: Sender<CallbackMessage>,
    global: GlobalContext,
    group: GroupContext,
    buffer_size: usize,
}

impl Context {
    pub(crate) fn new(
        callback_tx: Sender<CallbackMessage>,
        global: GlobalContext,
        group: GroupContext,
    ) -> Self {
        Self {
            callback_tx,
            global,
            group,
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
    pub const fn group(&self) -> &GroupContext {
        &self.group
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

    fn callback(&self, name: &str, func: &str, data: Option<Value>) -> Result<(), CallbackError> {
        self.callback_tx
            .send(CallbackMessage::Call(
                name.to_string(),
                func.to_string(),
                data,
            ))
            .map_err(|_| CallbackError::ChannelClosed)
    }

    /// Sends a callback with data into Arma
    /// <https://community.bistudio.com/wiki/Arma_3:_Mission_Event_Handlers#ExtensionCallback>
    pub fn callback_data<V>(&self, name: &str, func: &str, data: V) -> Result<(), CallbackError>
    where
        V: IntoArma,
    {
        self.callback(name, func, Some(data.to_arma()))
    }

    /// Sends a callback without data into Arma
    /// <https://community.bistudio.com/wiki/Arma_3:_Mission_Event_Handlers#ExtensionCallback>
    pub fn callback_null(&self, name: &str, func: &str) -> Result<(), CallbackError> {
        self.callback(name, func, None)
    }
}

/// Error that can occur when sending a callback
#[derive(Debug)]
pub enum CallbackError {
    /// The callback channel has been closed
    ChannelClosed,
}

impl std::fmt::Display for CallbackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ChannelClosed => write!(f, "Callback channel closed"),
        }
    }
}

impl IntoArma for CallbackError {
    fn to_arma(&self) -> Value {
        Value::String(self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::State;
    use crossbeam_channel::{bounded, Sender};
    use std::sync::Arc;

    fn context(tx: Sender<CallbackMessage>) -> Context {
        Context::new(
            tx,
            GlobalContext::new(String::new(), Arc::new(State::default())),
            GroupContext::new(Arc::new(State::default())),
        )
    }

    #[test]
    fn context_buffer_len_zero() {
        let (tx, _) = bounded(0);
        assert_eq!(context(tx).buffer_len(), 0);
    }

    #[test]
    fn context_buffer_len() {
        let (tx, _) = bounded(0);
        assert_eq!(context(tx).with_buffer_size(100).buffer_len(), 99);
    }

    #[test]
    fn context_callback_block() {
        let (tx, rx) = bounded(0);
        let callback_tx = tx.clone();
        std::thread::spawn(|| {
            context(callback_tx).callback_null("", "").unwrap();
        });
        let callback_tx = tx;
        std::thread::spawn(|| {
            context(callback_tx).callback_data("", "", "").unwrap();
        });

        std::thread::sleep(std::time::Duration::from_millis(50));
        assert_eq!(rx.iter().count(), 2);
    }

    #[test]
    fn context_callback_closed() {
        let (tx, _) = bounded(0);
        assert!(matches!(
            context(tx.clone()).callback_null("", ""),
            Err(CallbackError::ChannelClosed)
        ));
        assert!(matches!(
            context(tx).callback_data("", "", ""),
            Err(CallbackError::ChannelClosed)
        ));
    }
}
