use std::path::Path;

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

/// Context automatically provided by Arma on extension call. Supported since Arma version 2.11.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ArmaCallContext {
    caller: Caller,
    source: Source,
    mission: Mission,
    server: Server,
}

impl ArmaCallContext {
    #[must_use]
    /// Create a new [`ArmaCallContext`]. Mainly for use with [`crate::testing`].
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

#[cfg(test)]
mod tests {
    use super::*;

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
