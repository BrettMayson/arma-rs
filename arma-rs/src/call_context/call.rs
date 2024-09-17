use std::path::Path;

use super::stack::ArmaContextStackTrace;

#[repr(C)]
struct RawArmaCallContext {
    pub steam_id: u64,
    pub source: *const libc::c_char,
    pub mission: *const libc::c_char,
    pub server: *const libc::c_char,
    pub remote_exec_owner: i16,
    pub call_stack: Option<*const super::stack::RawContextStackTrace>,
}

impl RawArmaCallContext {
    fn from_arma(args: *mut *mut i8, count: libc::c_int) -> Self {
        let steam_id = unsafe { *args.offset(0) as u64 };
        let source = unsafe { *args.offset(1) as *const libc::c_char };
        let mission = unsafe { *args.offset(2) as *const libc::c_char };
        let server = unsafe { *args.offset(3) as *const libc::c_char };
        let remote_exec_owner = unsafe { *args.offset(4) as i16 };

        let call_stack = if count > 5 {
            let stack = unsafe { *args.offset(5) as *const super::stack::RawContextStackTrace };
            Some(stack)
        } else {
            None
        };

        Self {
            steam_id,
            source,
            mission,
            server,
            remote_exec_owner,
            call_stack,
        }
    }
}

pub trait StackRequest {}

pub struct WithStackTrace;
impl StackRequest for WithStackTrace {}

pub struct WithoutStackTrace;
impl StackRequest for WithoutStackTrace {}

/// Context of the callExtension, provided by Arma.
pub type CallContext = ArmaCallContext<WithoutStackTrace>;
/// Context of the callExtension, provided by Arma, with a stack trace.
pub type CallContextStackTrace = ArmaCallContext<WithStackTrace>;

#[derive(Clone, Default)]
/// Context of the Arma call.
pub struct ArmaCallContext<T: StackRequest> {
    pub(super) caller: Caller,
    pub(super) source: Source,
    pub(super) mission: Mission,
    pub(super) server: Server,
    pub(super) remote_exec_owner: i16,

    _stack_marker: std::marker::PhantomData<T>,
    stack: Option<ArmaContextStackTrace>,
}

impl<T: StackRequest> ArmaCallContext<T> {
    pub(crate) const fn new(
        caller: Caller,
        source: Source,
        mission: Mission,
        server: Server,
        remote_exec_owner: i16,
    ) -> Self {
        Self {
            caller,
            source,
            mission,
            server,
            remote_exec_owner,

            _stack_marker: std::marker::PhantomData,
            stack: None,
        }
    }

    /// Create a new ArmaCallContext from pointers provided by Arma.
    pub fn from_arma(args: *mut *mut i8, count: libc::c_int) -> Self {
        let raw = RawArmaCallContext::from_arma(args, count);
        Self {
            caller: Caller::Steam(raw.steam_id),
            source: Source::from(unsafe { std::ffi::CStr::from_ptr(raw.source).to_str().unwrap() }),
            mission: Mission::from(unsafe {
                std::ffi::CStr::from_ptr(raw.mission).to_str().unwrap()
            }),
            server: Server::from(unsafe { std::ffi::CStr::from_ptr(raw.server).to_str().unwrap() }),
            remote_exec_owner: raw.remote_exec_owner,

            _stack_marker: std::marker::PhantomData,
            stack: raw.call_stack.map(ArmaContextStackTrace::from),
        }
    }

    #[must_use]
    /// Player that called the extension. Can be [`Caller::Unknown`] when the player's steamID64 is unavailable
    /// # Note
    /// Unlike <https://community.bistudio.com/wiki/getPlayerUID> [`Caller::Steam`] isn't limited to multiplayer.
    pub fn caller(&self) -> &Caller {
        &self.caller
    }

    #[must_use]
    /// Source from where the extension was called.
    pub fn source(&self) -> &Source {
        &self.source
    }

    #[must_use]
    /// Current mission's name.
    /// # Note
    /// Can result in [`Mission::None`] in missions made prior to Arma v2.02.
    pub fn mission(&self) -> &Mission {
        &self.mission
    }

    #[must_use]
    /// Current server's name
    pub fn server(&self) -> &Server {
        &self.server
    }

    #[must_use]
    /// Remote execution owner.
    pub fn remote_exec_owner(&self) -> i16 {
        self.remote_exec_owner
    }
}

impl ArmaCallContext<WithStackTrace> {
    #[must_use]
    /// Call stack of the extension call.
    pub fn stack_trace(&self) -> &ArmaContextStackTrace {
        // By the time this gets to consumer code, to_without_stack would've been called if the stack was not requested
        self.stack.as_ref().expect("Stack is missing")
    }

    /// Convert the context to one without a stack trace.
    pub(crate) fn into_without_stack(self) -> ArmaCallContext<WithoutStackTrace> {
        ArmaCallContext::new(
            self.caller,
            self.source,
            self.mission,
            self.server,
            self.remote_exec_owner,
        )
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

impl Caller {
    /// Convert the caller to a string.
    pub fn as_str(&self) -> String {
        match self {
            Self::Steam(id) => id.to_string(),
            Self::Unknown => "0".to_string(),
        }
    }

    /// Convert the caller to a u64.
    pub fn as_u64(&self) -> u64 {
        match self {
            Self::Steam(id) => *id,
            Self::Unknown => 0,
        }
    }
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

impl From<u64> for Caller {
    fn from(id: u64) -> Self {
        if id == 0 {
            Self::Unknown
        } else {
            Self::Steam(id)
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

impl Source {
    /// Convert the source to a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::File(s) | Self::Pbo(s) => s,
            Self::Console => "",
        }
    }
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

impl From<*const libc::c_char> for Source {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from(s: *const libc::c_char) -> Self {
        Self::from(unsafe { std::ffi::CStr::from_ptr(s).to_str().unwrap() })
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

impl Mission {
    /// Convert the mission to a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Mission(s) => s,
            Self::None => "",
        }
    }
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

impl From<*const libc::c_char> for Mission {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from(s: *const libc::c_char) -> Self {
        Self::from(unsafe { std::ffi::CStr::from_ptr(s).to_str().unwrap() })
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

impl Server {
    /// Convert the server to a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Multiplayer(s) => s,
            Self::Singleplayer => "",
        }
    }
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

impl From<*const libc::c_char> for Server {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from(s: *const libc::c_char) -> Self {
        Self::from(unsafe { std::ffi::CStr::from_ptr(s).to_str().unwrap() })
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
