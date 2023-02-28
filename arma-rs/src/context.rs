use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use crossbeam_queue::SegQueue;

use crate::{IntoArma, State, Value};

#[derive(Clone)]
pub(crate) struct ArmaContext {
    steam_id: Option<String>,
    file_source: Option<PathBuf>,
    mission_name: Option<String>,
    server_name: Option<String>,
}

impl ArmaContext {
    pub(crate) fn new() -> Self {
        Self {
            steam_id: None,
            file_source: None,
            mission_name: None,
            server_name: None,
        }
    }

    pub(crate) fn with_steam_id(mut self, steam_id: &str) -> Self {
        self.steam_id = if !steam_id.is_empty() && steam_id != "0" {
            Some(steam_id.to_string())
        } else {
            None
        };
        self
    }

    pub(crate) fn with_file_source(mut self, file_source: &str) -> Self {
        self.file_source = if !file_source.is_empty() {
            Some(PathBuf::from(file_source))
        } else {
            None
        };
        self
    }

    pub(crate) fn with_mission_name(mut self, mission_name: &str) -> Self {
        self.mission_name = if !mission_name.is_empty() {
            Some(mission_name.to_string())
        } else {
            None
        };
        self
    }

    pub(crate) fn with_server_name(mut self, server_name: &str) -> Self {
        self.server_name = if !server_name.is_empty() {
            Some(server_name.to_string())
        } else {
            None
        };
        self
    }
}

/// Contains information about the current execution context
pub struct Context {
    arma_ctx: ArmaContext,
    state: Arc<State>,
    queue: Arc<SegQueue<(String, String, Option<Value>)>>,
    buffer_size: usize,
}

impl Context {
    pub(crate) fn new(
        arma_ctx: ArmaContext,
        state: Arc<State>,
        queue: Arc<SegQueue<(String, String, Option<Value>)>>,
    ) -> Self {
        Self {
            arma_ctx,
            state,
            queue,
            buffer_size: 0,
        }
    }

    pub(crate) const fn with_buffer_size(mut self, buffer_size: usize) -> Self {
        self.buffer_size = buffer_size;
        self
    }

    /// Get a reference to the extensions state container.
    pub fn state(&self) -> &State {
        &self.state
    }

    #[must_use]
    pub fn steam_id(&self) -> Option<&str> {
        self.arma_ctx.steam_id.as_deref()
    }

    #[must_use]
    pub fn file_source(&self) -> Option<&Path> {
        self.arma_ctx.file_source.as_deref()
    }

    #[must_use]
    pub fn mission_name(&self) -> Option<&str> {
        self.arma_ctx.mission_name.as_deref()
    }

    #[must_use]
    pub fn server_name(&self) -> Option<&str> {
        self.arma_ctx.server_name.as_deref()
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
        let ctx = Context::new(
            ArmaContext::new(),
            Arc::new(State::default()),
            Arc::new(SegQueue::new()),
        );
        assert_eq!(ctx.buffer_len(), 0);
    }

    #[test]
    fn context_buffer_len() {
        let ctx = Context::new(
            ArmaContext::new(),
            Arc::new(State::default()),
            Arc::new(SegQueue::new()),
        )
        .with_buffer_size(100);
        assert_eq!(ctx.buffer_len(), 99);
    }
}
