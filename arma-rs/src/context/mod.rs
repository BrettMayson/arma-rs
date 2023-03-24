//! Contextual execution information.

use std::{path::Path, sync::Arc};

use crossbeam_queue::SegQueue;

use crate::{IntoArma, State, Value};

mod global;

pub use global::GlobalContext;

/// Contains information about the current execution context
pub struct Context {
    global: GlobalContext,
    state: Arc<State>,
    arma_info: Option<ArmaInfo>,
    queue: Arc<SegQueue<(String, String, Option<Value>)>>,
    buffer_size: usize,
}

impl Context {
    pub(crate) fn new(
        global: GlobalContext,
        state: Arc<State>,
        queue: Arc<SegQueue<(String, String, Option<Value>)>>,
    ) -> Self {
        Self {
            global,
            state,
            arma_info: None,
            queue,
            buffer_size: 0,
        }
    }

    pub(crate) fn with_state(mut self, state: Arc<State>) -> Self {
        self.state = state;
        self
    }

    pub(crate) const fn with_buffer_size(mut self, buffer_size: usize) -> Self {
        self.buffer_size = buffer_size;
        self
    }

    pub(crate) fn with_arma_info(mut self, arma_info: Option<ArmaInfo>) -> Self {
        self.arma_info = arma_info;
        self
    }

    #[must_use]
    pub const fn global(&self) -> &GlobalContext {
        &self.global
    }

    #[must_use]
    pub fn state(&self) -> &State {
        &self.state
    }

    #[must_use]
    /// Info automatically provided by Arma. Supported since Arma version 2.11.
    pub const fn arma_info(&self) -> Option<&ArmaInfo> {
        self.arma_info.as_ref()
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

/// Info automatically provided by Arma on extension call. Supported since Arma version 2.11.
#[derive(Clone)]
pub struct ArmaInfo {
    caller: Caller,
    source: Source,
    mission: Mission,
    server: Server,
}

impl ArmaInfo {
    #[must_use]
    /// Create a new [`ArmaInfo`]. Mainly for use with [`crate::testing`].
    pub const fn new(caller: Caller, source: Source, mission: Mission, server: Server) -> Self {
        Self {
            caller,
            source,
            mission,
            server,
        }
    }

    #[must_use]
    /// Player that called the extension. Could be [`Caller::Unknown`] when the player's steamID64 is unavailable
    /// # Note
    /// Unlike <https://community.bistudio.com/wiki/getPlayerUID> [`Caller::Steam`] isn't limited to multiplayer.
    pub const fn caller(&self) -> &Caller {
        &self.caller
    }

    #[must_use]
    /// Source from where the extension was called.
    pub const fn source(&self) -> &Source {
        &self.source
    }

    #[must_use]
    /// Current mission's name.
    /// # Note
    /// Could result in [`Mission::None`] in missions prior to Arma v2.02.
    pub const fn mission(&self) -> &Mission {
        &self.mission
    }

    #[must_use]
    /// Current server's name
    pub const fn server(&self) -> &Server {
        &self.server
    }
}

/// Identification of the player calling your extension.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Caller {
    /// The player's steamID64.
    Steam(u64),
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    /// Absolute path of the file on the players system.
    /// For example on windows: `C:\Users\user\Documents\Arma 3\missions\test.VR\fn_armaInfo.sqf`.
    File(String),
    /// Path inside of a pbo.
    /// For example: `z\test\addons\main\fn_armaInfo.sqf`.
    Pbo(String),
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mission {
    /// Mission name.
    Mission(String),
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Server {
    /// Server name
    Multiplayer(String),
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

    use crate::State;

    #[test]
    fn context_buffer_len_zero() {
        let ctx = Context::new(
            GlobalContext::new(String::new(), Arc::new(State::default())),
            Arc::new(State::default()),
            Arc::new(SegQueue::new()),
        );
        assert_eq!(ctx.buffer_len(), 0);
    }

    #[test]
    fn context_buffer_len() {
        let ctx = Context::new(
            GlobalContext::new(String::new(), Arc::new(State::default())),
            Arc::new(State::default()),
            Arc::new(SegQueue::new()),
        )
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
        let path = "x\\ctx\\addons\\main\\fn_armaInfo.sqf";
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
