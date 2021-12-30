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
pub type Callback =
    extern "stdcall" fn(*const libc::c_char, *const libc::c_char, *const libc::c_char) -> i32;
#[cfg(not(windows))]
pub type Callback =
    extern "C" fn(*const libc::c_char, *const libc::c_char, *const libc::c_char) -> i32;

pub struct Extension {
    version: String,
    group: Group,
    allow_no_args: bool,
    callback: Option<Callback>,
    callback_queue: Arc<SegQueue<(String, String, Option<ArmaValue>)>>,
}

impl Extension {
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

    pub const fn allow_no_args(&self) -> bool {
        self.allow_no_args
    }

    pub fn register_callback(&mut self, callback: Callback) {
        self.callback = Some(callback);
    }

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
        size: usize,
        args: Option<*mut *mut i8>,
        count: Option<usize>,
    ) -> usize {
        let function = std::ffi::CStr::from_ptr(function).to_str().unwrap();
        self.group.handle(
            self.context(),
            function.to_string(),
            output,
            size,
            args,
            count,
        )
    }

    pub fn testing(self) -> TestingExtension {
        TestingExtension::new(self)
    }

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
    pub fn version(mut self, version: String) -> Self {
        self.version = version;
        self
    }

    #[inline]
    pub fn group<S>(mut self, name: S, group: Group) -> Self
    where
        S: Into<String>,
    {
        self.group = self.group.group(name.into(), group);
        self
    }

    #[inline]
    pub const fn allow_no_args(mut self) -> Self {
        self.allow_no_args = true;
        self
    }

    #[inline]
    pub fn command<S, F, I, R>(mut self, name: S, handler: F) -> Self
    where
        S: Into<String>,
        F: CommandFactory<I, R> + 'static,
    {
        self.group = self.group.command(name, handler);
        self
    }

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
    buf_size: libc::size_t,
) -> Option<usize> {
    if !string.is_ascii() {
        return None;
    };
    let cstr = std::ffi::CString::new(string).ok()?;
    let cstr_bytes = cstr.as_bytes();
    let amount_to_copy = std::cmp::min(cstr_bytes.len(), buf_size - 1);
    if amount_to_copy > isize::MAX as usize {
        return None;
    }
    ptr.copy_from(cstr.as_ptr(), amount_to_copy);
    ptr.add(amount_to_copy).write(0x00);
    Some(amount_to_copy)
}