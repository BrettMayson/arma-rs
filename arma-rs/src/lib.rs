pub use arma_rs_proc::arma;
pub use libc;

mod arma;
pub use arma::{ArmaValue, FromArma, IntoArma};
mod command;
mod group;

pub use command::*;
pub use group::Group;

pub struct Extension {
    version: String,
    group: Group,
    allow_no_args: bool,
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
        self.group
            .handle(function.to_string(), output, size, args, count)
    }

    /// Call a function, intended for tests
    /// # Safety
    /// This function is unsafe because it interacts with the C API.
    pub unsafe fn call(&self, function: &str, args: Option<Vec<String>>) -> (String, usize) {
        let output = std::ffi::CString::new("").unwrap().into_raw();
        let len = args.as_ref().map(|a| a.len());
        let mut args_pointer = args.map(|v| {
            v.into_iter()
                .map(|s| std::ffi::CString::new(s).unwrap().into_raw())
                .collect::<Vec<*mut i8>>()
        });
        let res = self.group.handle(
            function.to_string(),
            output,
            10240,
            args_pointer.as_mut().map(|a| a.as_mut_ptr()),
            len,
        );
        (
            std::ffi::CStr::from_ptr(output)
                .to_str()
                .unwrap()
                .to_string(),
            res,
        )
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
