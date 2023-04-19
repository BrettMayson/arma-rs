//! Contextual execution information.

use std::sync::Arc;

use crossbeam_queue::SegQueue;

use crate::{IntoArma, State, Value};

#[cfg(feature = "call-context")]
mod call;

#[cfg(feature = "call-context")]
pub use call::*;

/// Contains information about the current execution context
pub struct Context {
    state: Arc<State>,
    #[cfg(feature = "call-context")]
    call: ArmaCallContext,
    queue: Arc<SegQueue<(String, String, Option<Value>)>>,
    buffer_size: usize,
}

impl Context {
    pub(crate) fn new(
        state: Arc<State>,
        queue: Arc<SegQueue<(String, String, Option<Value>)>>,
    ) -> Self {
        Self {
            state,
            #[cfg(feature = "call-context")]
            call: ArmaCallContext::default(),
            queue,
            buffer_size: 0,
        }
    }

    #[cfg(feature = "call-context")]
    pub(crate) fn with_call(mut self, call: ArmaCallContext) -> Self {
        self.call = call;
        self
    }

    pub(crate) const fn with_buffer_size(mut self, buffer_size: usize) -> Self {
        self.buffer_size = buffer_size;
        self
    }

    /// Get a reference to the extensions state container.
    pub fn state(&self) -> &State {
        &self.state
    }

    #[cfg(feature = "call-context")]
    #[must_use]
    /// Player that called the extension. Could be [`Caller::Unknown`] when the player's steamID64 is unavailable
    /// # Note
    /// Unlike <https://community.bistudio.com/wiki/getPlayerUID> [`Caller::Steam`] isn't limited to multiplayer.
    pub const fn caller(&self) -> &Caller {
        &self.call.caller
    }

    #[cfg(feature = "call-context")]
    #[must_use]
    /// Source from where the extension was called.
    pub const fn source(&self) -> &Source {
        &self.call.source
    }

    #[cfg(feature = "call-context")]
    #[must_use]
    /// Current mission's name.
    /// # Note
    /// Could result in [`Mission::None`] in missions prior to Arma v2.02.
    pub const fn mission(&self) -> &Mission {
        &self.call.mission
    }

    #[cfg(feature = "call-context")]
    #[must_use]
    /// Current server's name
    pub const fn server(&self) -> &Server {
        &self.call.server
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_buffer_len_zero() {
        let ctx = Context::new(Arc::new(State::default()), Arc::new(SegQueue::new()));
        assert_eq!(ctx.buffer_len(), 0);
    }

    #[test]
    fn context_buffer_len() {
        let ctx = Context::new(Arc::new(State::default()), Arc::new(SegQueue::new()))
            .with_buffer_size(100);
        assert_eq!(ctx.buffer_len(), 99);
    }
}
