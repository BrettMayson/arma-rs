use std::path::Path;

#[derive(Clone, Default)]
pub struct ArmaCallContext {
    pub(super) caller: Caller,
    pub(super) source: Source,
    pub(super) mission: Mission,
    pub(super) server: Server,
}

impl ArmaCallContext {
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

    pub fn caller(&self) -> &Caller {
        &self.caller
    }

    pub fn source(&self) -> &Source {
        &self.source
    }

    pub fn mission(&self) -> &Mission {
        &self.mission
    }

    pub fn server(&self) -> &Server {
        &self.server
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
    /// Debug console or an other form of on the fly execution, such as mission triggers.
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
