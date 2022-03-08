use std::sync::Arc;

use crossbeam_queue::SegQueue;

use crate::{IntoArma, Value};

/// Contains information about the current execution context
pub struct Context {
    pub(crate) queue: Arc<SegQueue<(String, String, Option<Value>)>>,
    buffer_len: usize,
}

impl Context {
    pub(crate) fn new(queue: Arc<SegQueue<(String, String, Option<Value>)>>) -> Self {
        Self {
            queue,
            buffer_len: 0,
        }
    }

    pub(crate) const fn with_buffer_size(mut self, buffer_len: usize) -> Self {
        self.buffer_len = buffer_len;
        self
    }

    #[must_use]
    /// Returns the length in bits of the output buffer.
    /// This is the maximum size of the data that can be returned by the extension.
    pub const fn buffer_len(&self) -> usize {
        self.buffer_len
    }

    /// Sends a callback into Arma
    /// <https://community.bistudio.com/wiki/Arma_3:_Mission_Event_Handlers#ExtensionCallback>
    pub fn callback<V>(&self, name: &str, func: &str, data: Option<V>)
    where
        V: IntoArma,
    {
        self.queue
            .push((name.to_string(), func.to_string(), Some(data.into())));
    }
}
