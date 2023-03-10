use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Caller {
    Steam(u64),
    Unknown, // Wiki states arma could provide a `0`, its unknown when this happens
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    File(String),
    Pbo(String),
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

// Note: is unknown when not in a mission and could be unknown in missions prior to arma v2.02
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mission {
    Mission(String),
    Unknown,
}

impl From<&str> for Mission {
    fn from(s: &str) -> Self {
        if s.is_empty() {
            Self::Unknown
        } else {
            Self::Mission(s.to_string())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Server {
    Multiplayer(String),
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

#[derive(Clone)]
pub struct ArmaContext {
    caller: Caller,
    source: Source,
    mission: Mission,
    server: Server,
}

impl ArmaContext {
    #[must_use]
    pub const fn new(caller: Caller, source: Source, mission: Mission, server: Server) -> Self {
        Self {
            caller,
            source,
            mission,
            server,
        }
    }

    #[must_use]
    pub const fn caller(&self) -> &Caller {
        &self.caller
    }

    #[must_use]
    pub const fn source(&self) -> &Source {
        &self.source
    }

    #[must_use]
    pub const fn mission(&self) -> &Mission {
        &self.mission
    }

    #[must_use]
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
        assert_eq!(Mission::from(""), Mission::Unknown);
    }

    #[test]
    fn server_empty() {
        assert_eq!(Server::from(""), Server::Singleplayer);
    }
}
