use std::{path::Path, fmt::{Display, Formatter}};

#[derive(Clone, Default)]
pub(crate) struct ArmaCallContext {
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

impl Display for Caller {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Steam(s) => write!(f, "{}", s),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Caller {
    /// Returns the steamID64 of the caller, if available.
    pub fn as_u64(&self) -> Option<&u64> {
        match self {
            Caller::Steam(inner) => {
                Some(inner)
            },
            _ => None,
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

impl Display for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(s) => write!(f, "{}", s),
            Self::Pbo(s) => write!(f, "{}", s),
            Self::Console => write!(f, "Console"),
        }
    }
}

impl Source {
    /// Returns the string representation of the source, if available.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Source::File(inner) | Source::Pbo(inner) => {
                Some(inner)
            },
            _ => None,
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

impl Display for Mission {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mission(s) => write!(f, "{}", s),
            Self::None => write!(f, "None"),
        }
    }
}

impl Mission {
    /// Returns the string representation of the mission, if available.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Mission::Mission(inner) => {
                Some(inner)
            },
            _ => None,
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

impl Display for Server {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Multiplayer(s) => write!(f, "{}", s),
            Self::Singleplayer => write!(f, "Singleplayer"),
        }
    }
}

impl Server {
    /// Returns the string representation of the server, if available.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Server::Multiplayer(inner) => {
                Some(inner)
            },
            _ => None,
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
