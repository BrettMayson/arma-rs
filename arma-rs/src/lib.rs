#![warn(missing_docs, nonstandard_style)]

//! Library for building powerful Extensions for Arma 3 easily in Rust

#[cfg(feature = "extension")]
use std::sync::{Arc, RwLock};

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
mod context;
#[cfg(feature = "extension")]
pub use context::Context;
#[cfg(feature = "extension")]
mod group;
#[cfg(feature = "extension")]
pub use group::Group;
#[cfg(feature = "extension")]
mod testing;
#[cfg(feature = "extension")]
pub use testing::Result;

#[cfg(all(windows, feature = "extension"))]
/// Used by generated code to call back into Arma
pub type Callback = extern "stdcall" fn(
    *const libc::c_char,
    *const libc::c_char,
    *const libc::c_char,
) -> libc::c_int;
#[cfg(all(not(windows), feature = "extension"))]
/// Used by generated code to call back into Arma
pub type Callback =
    extern "C" fn(*const libc::c_char, *const libc::c_char, *const libc::c_char) -> libc::c_int;

/// Contains all the information about your extension
/// This is used by the generated code to interface with Arma
#[cfg(feature = "extension")]
pub struct Extension<S> {
    version: String,
    group: Group<S>,
    state: Arc<RwLock<S>>,
    allow_no_args: bool,
    callback: Option<Callback>,
    callback_queue: Arc<SegQueue<(String, String, Option<Value>)>>,
}

#[cfg(feature = "extension")]
impl Extension<()> {
    #[must_use]
    /// Creates a new extension with no state.
    pub fn build() -> ExtensionBuilder<()> {
        Self::build_with_state(())
    }
}

#[cfg(feature = "extension")]
impl<S> Extension<S> {
    #[must_use]
    /// Creates a new extension with a persistent state.
    pub fn build_with_state(state: S) -> ExtensionBuilder<S> {
        ExtensionBuilder {
            version: env!("CARGO_PKG_VERSION").to_string(),
            group: Group::new(),
            allow_no_args: false,
            state,
        }
    }

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

    /// Called by generated code, do not call directly.
    pub fn register_callback(&mut self, callback: Callback) {
        self.callback = Some(callback);
    }

    #[must_use]
    /// Get a RwLock for editing the extensions persistent state field
    pub fn state(&self) -> &RwLock<S> {
        self.state.as_ref()
    }

    #[must_use]
    /// Get a context for interacting with Arma
    pub fn context(&self) -> Context<S> {
        Context::new(self.state.clone(), self.callback_queue.clone())
    }

    /// Called by generated code, do not call directly.
    /// # Safety
    /// This function is unsafe because it interacts with the C API.
    pub unsafe fn handle(
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
        let context = self.context().with_buffer_size(size);
        self.group
            .handle(context, &function, output, size, args, count)
    }

    #[must_use]
    /// Create a version of the extension that can be used in tests.
    pub fn testing(self) -> testing::Extension<S> {
        testing::Extension::new(self)
    }

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
pub struct ExtensionBuilder<S> {
    version: String,
    group: Group<S>,
    state: S,
    allow_no_args: bool,
}

#[cfg(feature = "extension")]
impl<S> ExtensionBuilder<S> {
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
    pub fn group(mut self, name: impl Into<String>, group: Group<S>) -> Self {
        self.group = self.group.group(name.into(), group);
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
    pub fn command<F, I, R>(mut self, name: impl Into<String>, handler: F) -> Self
    where
        F: Factory<I, S, R> + 'static,
    {
        self.group = self.group.command(name, handler);
        self
    }

    #[inline]
    #[must_use]
    /// Builds the extension.
    pub fn finish(self) -> Extension<S> {
        Extension {
            version: self.version,
            group: self.group,
            state: Arc::new(RwLock::new(self.state)),
            allow_no_args: self.allow_no_args,
            callback: None,
            callback_queue: Arc::new(SegQueue::new()),
        }
    }
}

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
