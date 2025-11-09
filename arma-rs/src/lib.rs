#![warn(missing_docs, nonstandard_style)]
#![doc = include_str!(concat!(env!("OUT_DIR"), "/README.md"))]

use std::rc::Rc;

pub use arma_rs_proc::{arma, FromArma, IntoArma};

#[cfg(feature = "extension")]
use crossbeam_channel::{unbounded, Receiver, Sender};
#[cfg(feature = "extension")]
pub use libc;

#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub use link_args;

#[cfg(feature = "extension")]
#[macro_use]
extern crate log;

mod flags;

mod value;
pub use value::{loadout, DirectReturn, FromArma, FromArmaError, IntoArma, Value};

#[cfg(feature = "extension")]
mod call_context;
#[cfg(feature = "extension")]
use call_context::{ArmaCallContext, ArmaContextManager};
#[cfg(feature = "extension")]
pub use call_context::{CallContext, CallContextStackTrace, Caller, Mission, Server, Source};
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

#[cfg(all(feature = "extension"))]
#[doc(hidden)]
/// Used by generated code to call back into Arma
pub type Callback = extern "system" fn(
    *const libc::c_char,
    *const libc::c_char,
    *const libc::c_char,
) -> libc::c_int;
/// Requests a call context from Arma
pub type ContextRequest = unsafe extern "system" fn();

#[cfg(feature = "extension")]
enum CallbackMessage {
    Call(String, String, Option<Value>),
    Terminate,
}

#[cfg(feature = "extension")]
/// State TypeMap that can hold at most one value per type key.
pub type State = state::TypeMap![Send + Sync];

#[cfg(windows)]
/// Allows a console to be allocated for the extension.
static CONSOLE_ALLOCATED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[no_mangle]
#[allow(non_upper_case_globals, reason = "This is a C API")]
/// Feature flags read on each callExtension call.
pub static mut RVExtensionFeatureFlags: u64 = flags::RV_CONTEXT_NO_DEFAULT_CALL;

/// Contains all the information about your extension
/// This is used by the generated code to interface with Arma
#[cfg(feature = "extension")]
pub struct Extension {
    version: String,
    group: group::InternalGroup,
    allow_no_args: bool,
    callback: Option<Callback>,
    callback_channel: (Sender<CallbackMessage>, Receiver<CallbackMessage>),
    callback_thread: Option<std::thread::JoinHandle<()>>,
    context_manager: Rc<ArmaContextManager>,
    pre218_clear_context_override: bool,
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

    #[doc(hidden)]
    /// Called by generated code, do not call directly.
    /// # Safety
    /// This function is unsafe because it interacts with the C API.
    pub unsafe fn handle_call_context(&mut self, args: *mut *mut i8, count: libc::c_int) {
        self.context_manager
            .replace(Some(ArmaCallContext::from_arma(args, count)));
    }

    #[must_use]
    /// Get a context for interacting with Arma
    pub fn context(&self) -> Context {
        Context::new(
            self.callback_channel.0.clone(),
            GlobalContext::new(self.version.clone(), self.group.state.clone()),
            GroupContext::new(self.group.state.clone()),
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
        clear_call_context: bool,
    ) -> libc::c_int {
        if clear_call_context && !self.pre218_clear_context_override {
            self.context_manager.replace(None);
        }
        let function = if let Ok(cstring) = std::ffi::CStr::from_ptr(function).to_str() {
            cstring.to_string()
        } else {
            return 1;
        };
        match function.as_str() {
            #[cfg(windows)]
            "::console" => {
                if !CONSOLE_ALLOCATED.swap(true, std::sync::atomic::Ordering::SeqCst) {
                    let _ = windows::Win32::System::Console::AllocConsole();
                }
                0
            }
            _ => self.group.handle(
                self.context().with_buffer_size(size),
                self.context_manager.as_ref(),
                &function,
                output,
                size,
                args,
                count,
            ),
        }
    }

    #[must_use]
    /// Create a version of the extension that can be used in tests.
    pub fn testing(self) -> testing::Extension {
        testing::Extension::new(self)
    }

    #[doc(hidden)]
    /// Called by generated code, do not call directly.
    pub fn run_callbacks(&mut self) {
        let callback = self.callback;
        let (_, rx) = self.callback_channel.clone();
        self.callback_thread = Some(std::thread::spawn(move || {
            while let Ok(CallbackMessage::Call(name, func, data)) = rx.recv() {
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
        }));
    }
}

#[cfg(feature = "extension")]
impl Drop for Extension {
    // Never called when loaded by arma, instead this is purely required for rust testing.
    fn drop(&mut self) {
        if let Some(thread) = self.callback_thread.take() {
            let (tx, _) = &self.callback_channel;
            tx.send(CallbackMessage::Terminate).unwrap();
            thread.join().unwrap();
        }
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
        #[expect(unused_mut, reason = "Only used on Windows release")]
        let mut pre218 = false;
        #[allow(unused_variables)]
        let function_name =
            std::ffi::CString::new("RVExtensionRequestContext").expect("CString::new failed");
        #[cfg(all(windows, not(debug_assertions)))]
        let request_context: ContextRequest = {
            let handle = unsafe { winapi::um::libloaderapi::GetModuleHandleW(std::ptr::null()) };
            if handle.is_null() {
                panic!("GetModuleHandleW failed");
            }
            let func_address =
                unsafe { winapi::um::libloaderapi::GetProcAddress(handle, function_name.as_ptr()) };
            if func_address.is_null() {
                pre218 = true;
                empty_request_context
            } else {
                unsafe { std::mem::transmute(func_address) }
            }
        };
        #[cfg(all(not(windows), not(debug_assertions)))]
        let request_context: ContextRequest = {
            let handle = unsafe { libc::dlopen(std::ptr::null(), libc::RTLD_LAZY) };
            if handle.is_null() {
                panic!("Failed to open handle to current process");
            }
            let func_address = unsafe { libc::dlsym(handle, function_name.as_ptr()) };
            if func_address.is_null() {
                pre218 = true;
                empty_request_context
            } else {
                let func = unsafe { std::mem::transmute(func_address) };
                unsafe { libc::dlclose(handle) };
                func
            }
        };

        #[cfg(debug_assertions)]
        let request_context = empty_request_context;

        Extension {
            version: self.version,
            group: self.group.into(),
            allow_no_args: self.allow_no_args,
            callback: None,
            callback_channel: unbounded(),
            callback_thread: None,
            context_manager: Rc::new(ArmaContextManager::new(request_context)),
            pre218_clear_context_override: pre218,
        }
    }
}

unsafe extern "system" fn empty_request_context() {}

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
