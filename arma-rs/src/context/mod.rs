//! Contextual execution information.

use std::{path::Path, sync::Arc};

use crossbeam_queue::SegQueue;

use crate::{IntoArma, State, Value};

/// Contains information about the current execution context
pub struct Context {
    state: Arc<State>,
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
            call: ArmaCallContext::default(),
            queue,
            buffer_size: 0,
        }
    }

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

    #[must_use]
    /// Player that called the extension. Could be [`Caller::Unknown`] when the player's steamID64 is unavailable
    /// # Note
    /// Unlike <https://community.bistudio.com/wiki/getPlayerUID> [`Caller::Steam`] isn't limited to multiplayer.
    pub const fn caller(&self) -> &Caller {
        &self.call.caller
    }

    #[must_use]
    /// Source from where the extension was called.
    pub const fn source(&self) -> &Source {
        &self.call.source
    }

    #[must_use]
    /// Current mission's name.
    /// # Note
    /// Could result in [`Mission::None`] in missions prior to Arma v2.02.
    pub const fn mission(&self) -> &Mission {
        &self.call.mission
    }

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

/// Context automatically provided by Arma on extension call. Supported since Arma version 2.11.
#[derive(Clone, Default)]
pub(crate) struct ArmaCallContext {
    caller: Caller,
    source: Source,
    mission: Mission,
    server: Server,
}

impl ArmaCallContext {
    #[must_use]
    /// Create a new [`ArmaCallContext`]. Mainly for use with [`crate::testing`].
    pub(crate) const fn new(
        caller: Caller,
        source: Source,
        mission: Mission,
        server: Server,
    ) -> Self {
        Self {
            caller,
            source,
            mission,
            server,
        }
    }
}

/// Identification of the player calling your extension.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Caller {
    /// The player's steamID64.
    Steam(u64),
    #[default]
    /// Unable to determine.
    Unknown,
}

impl From<&str> for Caller {
    fn from(s: &str) -> Self {
        if s.is_empty() || s == "0" {
            Self::Unknown
        } else {
            s.parse::<u64>().map_or(Self::Unknown, Self::Steam)
        }
    }
}

/// Source of the extension call.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Source {
    /// Absolute path of the file on the players system.
    /// For example on windows: `C:\Users\user\Documents\Arma 3\missions\test.VR\fn_armaContext.sqf`.
    File(String),
    /// Path inside of a pbo.
    /// For example: `z\test\addons\main\fn_armaContext.sqf`.
    Pbo(String),
    #[default]
    /// Debug console.
    Console,
}

impl From<&str> for Source {
    fn from(s: &str) -> Self {
        if s.is_empty() {
            Self::Console
        } else if Path::new(s).is_absolute() {
            Self::File(s.to_string())
        } else {
            Self::Pbo(s.to_string())
        }
    }
}

/// Current mission.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Mission {
    /// Mission name.
    Mission(String),
    #[default]
    /// Not in a mission.
    None,
}

impl From<&str> for Mission {
    fn from(s: &str) -> Self {
        if s.is_empty() {
            Self::None
        } else {
            Self::Mission(s.to_string())
        }
    }
}

/// Current server.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Server {
    /// Server name
    Multiplayer(String),
    #[default]
    /// Singleplayer or no mission
    Singleplayer,
}

impl From<&str> for Server {
    fn from(s: &str) -> Self {
        if s.is_empty() {
            Self::Singleplayer
        } else {
            Self::Multiplayer(s.to_string())
        }
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

    #[test]
    fn caller_empty() {
        assert_eq!(Caller::from(""), Caller::Unknown);
    }

    #[test]
    fn caller_zero() {
        assert_eq!(Caller::from("0"), Caller::Unknown);
    }

    #[test]
    fn source_empty() {
        assert_eq!(Source::from(""), Source::Console);
    }

    #[test]
    fn source_pbo() {
        let path = "x\\ctx\\addons\\main\\fn_armaContext.sqf";
        assert_eq!(Source::from(path), Source::Pbo(path.to_string()));
    }

    #[test]
    fn source_file() {
        let path = env!("CARGO_MANIFEST_DIR");
        assert_eq!(Source::from(path), Source::File(path.to_string()));
    }

    #[test]
    fn mission_empty() {
        assert_eq!(Mission::from(""), Mission::None);
    }

    #[test]
    fn server_empty() {
        assert_eq!(Server::from(""), Server::Singleplayer);
    }
}
