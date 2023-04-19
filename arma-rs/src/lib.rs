#![warn(missing_docs, nonstandard_style)]

//! Library for building powerful Extensions for Arma 3 easily in Rust

#[cfg(feature = "extension")]
use std::sync::Arc;
#[cfg(feature = "call-context")]
use std::{cell::RefCell, cmp::Ordering};

pub use arma_rs_proc::arma;

#[cfg(feature = "extension")]
use crossbeam_queue::SegQueue;
#[cfg(feature = "extension")]
pub use libc;

#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub use link_args;

#[cfg(feature = "extension")]
#[macro_use]
extern crate log;

mod value;
pub use value::{loadout, FromArma, IntoArma, Value};

#[cfg(feature = "extension")]
mod ext_result;
#[cfg(feature = "extension")]
pub use ext_result::IntoExtResult;
#[cfg(feature = "extension")]
mod command;
#[cfg(feature = "extension")]
pub use command::*;
#[cfg(feature = "extension")]
pub mod context;
#[cfg(feature = "extension")]
pub use context::*;
#[cfg(feature = "extension")]
mod group;
#[cfg(feature = "extension")]
pub use group::Group;
#[cfg(feature = "extension")]
pub mod testing;
#[cfg(feature = "extension")]
pub use testing::Result;

#[cfg(all(windows, feature = "extension"))]
#[doc(hidden)]
/// Used by generated code to call back into Arma
pub type Callback = extern "stdcall" fn(
    *const libc::c_char,
    *const libc::c_char,
    *const libc::c_char,
) -> libc::c_int;
#[cfg(all(not(windows), feature = "extension"))]
#[doc(hidden)]
/// Used by generated code to call back into Arma
pub type Callback =
    extern "C" fn(*const libc::c_char, *const libc::c_char, *const libc::c_char) -> libc::c_int;

#[cfg(feature = "extension")]
/// State container that can hold at most one value per type key.
pub type State = state::Container![Send + Sync];

/// Contains all the information about your extension
/// This is used by the generated code to interface with Arma
#[cfg(feature = "extension")]
pub struct Extension {
    version: String,
    group: group::InternalGroup,
    allow_no_args: bool,
    callback: Option<Callback>,
    callback_queue: Arc<SegQueue<(String, String, Option<Value>)>>,
    #[cfg(feature = "call-context")]
    call_ctx: RefCell<ArmaCallContext>,
}

#[cfg(feature = "extension")]
impl Extension {
    #[must_use]
    /// Creates a new extension.
    pub fn build() -> ExtensionBuilder {
        ExtensionBuilder {
            version: String::from("0.0.0"),
            group: Group::new(),
            allow_no_args: false,
        }
    }
}

#[cfg(feature = "extension")]
impl Extension {
    #[must_use]
    /// Returns the version of the extension.
    pub fn version(&self) -> &str {
        &self.version
    }

    #[must_use]
    /// Returns if the extension can be called without any arguments.
    /// Example:
    /// ```sqf
    /// "my_ext" callExtension "my_func"
    /// ```
    pub const fn allow_no_args(&self) -> bool {
        self.allow_no_args
    }

    #[doc(hidden)]
    /// Called by generated code, do not call directly.
    pub fn register_callback(&mut self, callback: Callback) {
        self.callback = Some(callback);
    }

    #[cfg(feature = "call-context")]
    #[doc(hidden)]
    /// Called by generated code, do not call directly.
    /// # Safety
    /// This function is unsafe because it interacts with the C API.
    pub unsafe fn handle_call_context(&mut self, args: *mut *mut i8, count: libc::c_int) {
        const CONTEXT_COUNT: usize = 4; // As of Arma 2.11 four args get passed (https://community.bistudio.com/wiki/callExtension)
        let ctx = match count.cmp(&(CONTEXT_COUNT as i32)) {
            Ordering::Less => {
                error!("invalid amount of args passed to `handle_call_context`");
                ArmaCallContext::default()
            }
            ordering => {
                if ordering == Ordering::Greater {
                    warn!("unexpected amount of args passed to `handle_call_context`");
                }

                let argv: Vec<_> = std::slice::from_raw_parts(args, CONTEXT_COUNT)
                    .iter()
                    .map(|&s| std::ffi::CStr::from_ptr(s).to_string_lossy())
                    .collect();
                ArmaCallContext::new(
                    Caller::from(argv[0].as_ref()),
                    Source::from(argv[1].as_ref()),
                    Mission::from(argv[2].as_ref()),
                    Server::from(argv[3].as_ref()),
                )
            }
        };
        self.call_ctx.replace(ctx);
    }

    #[must_use]
    /// Get a context for interacting with Arma
    pub fn context(&self) -> Context {
        #[allow(unused_mut, clippy::let_and_return)]
        Context::new(
            self.callback_queue.clone(),
            GlobalContext::new(self.version.clone(), self.group.state.clone()),
            GroupContext::new(self.group.state.clone()),
            #[cfg(feature = "call-context")]
            self.call_ctx.borrow().clone(),
        )
    }

    #[doc(hidden)]
    /// Called by generated code, do not call directly.
    /// # Safety
    /// This function is unsafe because it interacts with the C API.
    pub unsafe fn handle_call(
        &self,
        function: *mut libc::c_char,
        output: *mut libc::c_char,
        size: libc::size_t,
        args: Option<*mut *mut i8>,
        count: Option<libc::c_int>,
    ) -> libc::c_int {
        let function = if let Ok(cstring) = std::ffi::CStr::from_ptr(function).to_str() {
            cstring.to_string()
        } else {
            return 1;
        };
        self.group.handle(
            self.context().with_buffer_size(size),
            &function,
            output,
            size,
            args,
            count,
        )
    }

    #[must_use]
    /// Create a version of the extension that can be used in tests.
    pub fn testing(self) -> testing::Extension {
        testing::Extension::new(self)
    }

    #[doc(hidden)]
    /// Called by generated code, do not call directly.
    pub fn run_callbacks(&self) {
        let queue = self.callback_queue.clone();
        let callback = self.callback;
        std::thread::spawn(move || loop {
            if let Some((name, func, data)) = queue.pop() {
                if let Some(c) = callback {
                    let name = if let Ok(cstring) = std::ffi::CString::new(name) {
                        cstring
                    } else {
                        error!("callback name was not valid");
                        continue;
                    };
                    let func = if let Ok(cstring) = std::ffi::CString::new(func) {
                        cstring
                    } else {
                        error!("callback func was not valid");
                        continue;
                    };
                    let data = if let Ok(cstring) = std::ffi::CString::new(match data {
                        Some(value) => match value {
                            Value::String(s) => s,
                            v => v.to_string(),
                        },
                        None => String::new(),
                    }) {
                        cstring
                    } else {
                        error!("callback data was not valid");
                        continue;
                    };

                    let (name, func, data) = (name.into_raw(), func.into_raw(), data.into_raw());
                    loop {
                        if c(name, func, data) >= 0 {
                            break;
                        }
                        std::thread::sleep(std::time::Duration::from_millis(1));
                    }
                    unsafe {
                        drop(std::ffi::CString::from_raw(name));
                        drop(std::ffi::CString::from_raw(func));
                        drop(std::ffi::CString::from_raw(data));
                    }
                }
            }
        });
    }
}

/// Used to build an extension.
#[cfg(feature = "extension")]
pub struct ExtensionBuilder {
    version: String,
    group: Group,
    allow_no_args: bool,
}

#[cfg(feature = "extension")]
impl ExtensionBuilder {
    #[inline]
    #[must_use]
    /// Sets the version of the extension.
    pub fn version(mut self, version: String) -> Self {
        self.version = version;
        self
    }

    #[inline]
    #[must_use]
    /// Add a group to the extension.
    pub fn group<S>(mut self, name: S, group: Group) -> Self
    where
        S: Into<String>,
    {
        self.group = self.group.group(name.into(), group);
        self
    }

    #[inline]
    #[must_use]
    /// Add a new state value to the extension if it has not be added already
    pub fn state<T>(mut self, state: T) -> Self
    where
        T: Send + Sync + 'static,
    {
        self.group = self.group.state(state);
        self
    }

    #[inline]
    #[must_use]
    /// Freeze the extension's state, preventing the state from changing, allowing for faster reads
    pub fn freeze_state(mut self) -> Self {
        self.group = self.group.freeze_state();
        self
    }

    #[inline]
    #[must_use]
    /// Allows the extension to be called without any arguments.
    /// Example:
    /// ```sqf
    /// "my_ext" callExtension "my_func"
    /// ```
    pub const fn allow_no_args(mut self) -> Self {
        self.allow_no_args = true;
        self
    }

    #[inline]
    #[must_use]
    /// Add a command to the extension.
    pub fn command<S, F, I, R>(mut self, name: S, handler: F) -> Self
    where
        S: Into<String>,
        F: Factory<I, R> + 'static,
    {
        self.group = self.group.command(name, handler);
        self
    }

    #[inline]
    #[must_use]
    /// Builds the extension.
    pub fn finish(self) -> Extension {
        Extension {
            version: self.version,
            group: self.group.into(),
            allow_no_args: self.allow_no_args,
            callback: None,
            callback_queue: Arc::new(SegQueue::new()),
            #[cfg(feature = "call-context")]
            call_ctx: RefCell::new(ArmaCallContext::default()),
        }
    }
}

#[doc(hidden)]
/// Called by generated code, do not call directly.
///
/// # Safety
/// This function is unsafe because it interacts with the C API.
///
/// # Note
/// This function assumes `buf_size` includes space for a single terminating zero byte at the end.
#[cfg(feature = "extension")]
pub unsafe fn write_cstr(
    string: String,
    ptr: *mut libc::c_char,
    buf_size: libc::size_t,
) -> Option<libc::size_t> {
    if string.is_empty() {
        return Some(0);
    }

    let cstr = std::ffi::CString::new(string).ok()?;
    let len_to_copy = cstr.as_bytes().len();
    if len_to_copy >= buf_size {
        return None;
    }

    ptr.copy_from(cstr.as_ptr(), len_to_copy);
    ptr.add(len_to_copy).write(0x00);
    Some(len_to_copy)
}

#[cfg(all(test, feature = "extension"))]
mod tests {
    use super::*;

    #[test]
    fn write_size_zero() {
        const BUF_SIZE: libc::size_t = 0;
        let mut buf = [0; BUF_SIZE];
        let result = unsafe { write_cstr("a".to_string(), buf.as_mut_ptr(), BUF_SIZE) };

        assert_eq!(result, None);
        assert_eq!(buf, [0; BUF_SIZE]);
    }

    #[test]
    fn write_size_zero_empty() {
        const BUF_SIZE: libc::size_t = 0;
        let mut buf = [0; BUF_SIZE];
        let result = unsafe { write_cstr("".to_string(), buf.as_mut_ptr(), BUF_SIZE) };

        assert_eq!(result, Some(0));
        assert_eq!(buf, [0; BUF_SIZE]);
    }

    #[test]
    fn write_size_one() {
        const BUF_SIZE: libc::size_t = 1;
        let mut buf = [0; BUF_SIZE];
        let result = unsafe { write_cstr("a".to_string(), buf.as_mut_ptr(), BUF_SIZE) };

        assert_eq!(result, None);
        assert_eq!(buf, [0; BUF_SIZE]);
    }

    #[test]
    fn write_size_one_empty() {
        const BUF_SIZE: libc::size_t = 1;
        let mut buf = [0; BUF_SIZE];
        let result = unsafe { write_cstr("".to_string(), buf.as_mut_ptr(), BUF_SIZE) };

        assert_eq!(result, Some(0));
        assert_eq!(buf, [0; BUF_SIZE]);
    }

    #[test]
    fn write_empty() {
        const BUF_SIZE: libc::size_t = 7;
        let mut buf = [0; BUF_SIZE];
        let result = unsafe { write_cstr("".to_string(), buf.as_mut_ptr(), BUF_SIZE) };

        assert_eq!(result, Some(0));
        assert_eq!(buf, [0; BUF_SIZE]);
    }

    #[test]
    fn write_half() {
        const BUF_SIZE: libc::size_t = 7;
        let mut buf = [0; BUF_SIZE];
        let result = unsafe { write_cstr("foo".to_string(), buf.as_mut_ptr(), BUF_SIZE) };

        assert_eq!(result, Some(3));
        assert_eq!(buf, (b"foo\0\0\0\0").map(|c| c as i8));
    }

    #[test]
    fn write_full() {
        const BUF_SIZE: libc::size_t = 7;
        let mut buf = [0; BUF_SIZE];
        let result = unsafe { write_cstr("foobar".to_string(), buf.as_mut_ptr(), BUF_SIZE) };

        assert_eq!(result, Some(6));
        assert_eq!(buf, (b"foobar\0").map(|c| c as i8));
    }

    #[test]
    fn write_overflow() {
        const BUF_SIZE: libc::size_t = 7;
        let mut buf = [0; BUF_SIZE];
        let result = unsafe { write_cstr("foo bar".to_string(), buf.as_mut_ptr(), BUF_SIZE) };

        assert_eq!(result, None);
        assert_eq!(buf, [0; BUF_SIZE]);
    }

    #[test]
    fn write_overwrite() {
        const BUF_SIZE: libc::size_t = 7;
        let mut buf = (b"zzzzzz\0").map(|c| c as i8);
        let result = unsafe { write_cstr("a".to_string(), buf.as_mut_ptr(), BUF_SIZE) };

        assert_eq!(result, Some(1));
        assert_eq!(buf, (b"a\0zzzz\0").map(|c| c as i8));
    }
}
