use std::sync::Arc;

pub use arma_rs_proc::arma;
use crossbeam_queue::SegQueue;
pub use libc;

mod arma;
pub use arma::{ArmaValue, FromArma, IntoArma};
mod command;
mod context;
mod group;
mod testing;

pub use command::*;
pub use context::Context;
pub use group::Group;
pub use testing::TestingExtension;

#[cfg(windows)]
pub type Callback = extern "stdcall" fn(
    *const libc::c_char,
    *const libc::c_char,
    *const libc::c_char,
) -> libc::c_int;
#[cfg(not(windows))]
pub type Callback =
    extern "C" fn(*const libc::c_char, *const libc::c_char, *const libc::c_char) -> libc::c_int;

pub struct Extension {
    version: String,
    group: Group,
    allow_no_args: bool,
    callback: Option<Callback>,
    callback_queue: Arc<SegQueue<(String, String, Option<ArmaValue>)>>,
}

impl Extension {
    /// Creates a new extension.
    pub fn build() -> ExtensionBuilder {
        ExtensionBuilder {
            version: env!("CARGO_PKG_VERSION").to_string(),
            group: Group::new(),
            allow_no_args: false,
        }
    }

    pub fn version(&self) -> &str {
        &self.version
    }

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

    /// Get a context for interacting with Arma
    pub fn context(&self) -> Context {
        Context::new(self.callback_queue.clone())
    }

    /// Called by generated code, do not call directly.
    /// # Safety
    /// This function is unsafe because it interacts with the C API.
    pub unsafe fn handle(
        &self,
        function: *mut libc::c_char,
        output: *mut libc::c_char,
        size: libc::c_int,
        args: Option<*mut *mut i8>,
        count: Option<libc::c_int>,
    ) -> libc::c_int {
        let function = std::ffi::CStr::from_ptr(function).to_str().unwrap();
        self.group.handle(
            self.context().with_buffer_size(size.try_into().unwrap()),
            function.to_string(),
            output,
            size,
            args,
            count,
        )
    }

    /// Create a version of the extension that can be used in tests.
    pub fn testing(self) -> TestingExtension {
        TestingExtension::new(self)
    }

    /// Called by generated code, do not call directly.
    pub fn run_callbacks(&self) {
        let queue = self.callback_queue.clone();
        let callback = self.callback;
        std::thread::spawn(move || loop {
            if let Some((name, func, data)) = queue.pop() {
                if let Some(c) = callback {
                    let name = std::ffi::CString::new(name).unwrap().into_raw();
                    let func = std::ffi::CString::new(func).unwrap().into_raw();
                    let data = std::ffi::CString::new(match data {
                        Some(value) => match value {
                            ArmaValue::String(s) => s,
                            v => v.to_string(),
                        },
                        None => String::new(),
                    })
                    .unwrap()
                    .into_raw();
                    loop {
                        if c(name, func, data) >= 0 {
                            break;
                        }
                        std::thread::sleep(std::time::Duration::from_millis(1));
                    }
                }
            }
        });
    }
}

pub struct ExtensionBuilder {
    version: String,
    group: Group,
    allow_no_args: bool,
}

impl ExtensionBuilder {
    #[inline]
    /// Sets the version of the extension.
    pub fn version(mut self, version: String) -> Self {
        self.version = version;
        self
    }

    #[inline]
    /// Add a group to the extension.
    pub fn group<S>(mut self, name: S, group: Group) -> Self
    where
        S: Into<String>,
    {
        self.group = self.group.group(name.into(), group);
        self
    }

    #[inline]
    /// Allows the extension to be called without any arguments.
    /// Example:
    /// ```sqf
    /// "my_ext" callExtension "my_func"
    /// ``
    pub const fn allow_no_args(mut self) -> Self {
        self.allow_no_args = true;
        self
    }

    #[inline]
    /// Add a command to the extension.
    pub fn command<S, F, I, R>(mut self, name: S, handler: F) -> Self
    where
        S: Into<String>,
        F: CommandFactory<I, R> + 'static,
    {
        self.group = self.group.command(name, handler);
        self
    }

    #[inline]
    /// Builds the extension.
    pub fn finish(self) -> Extension {
        Extension {
            version: self.version,
            group: self.group,
            allow_no_args: self.allow_no_args,
            callback: None,
            callback_queue: Arc::new(SegQueue::new()),
        }
    }
}

/// Called by generated code, do not call directly.
/// # Safety
/// This function is unsafe because it interacts with the C API.
pub unsafe fn write_cstr(
    string: String,
    ptr: *mut libc::c_char,
    buf_size: libc::c_int,
) -> Option<libc::c_int> {
    if !string.is_ascii() {
        return None;
    };
    let cstr = std::ffi::CString::new(string).ok()?;
    let cstr_bytes = cstr.as_bytes();
    let len_to_copy = cstr_bytes.len();
    if len_to_copy * 8 >= (buf_size - 8).try_into().unwrap() {
        return None;
    }
    ptr.copy_from(cstr.as_ptr(), len_to_copy);
    ptr.add(len_to_copy).write(0x00);
    Some(len_to_copy.try_into().unwrap())
}
