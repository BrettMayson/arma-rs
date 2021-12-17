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
    pub fn new() -> ExtensionBuilder {
        ExtensionBuilder {
            version: env!("CARGO_PKG_VERSION").to_string(),
            group: Group::new(""),
            allow_no_args: false,
        }
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn allow_no_args(&self) -> bool {
        self.allow_no_args
    }

    pub fn handle(
        &self,
        function: *mut libc::c_char,
        output: *mut libc::c_char,
        size: usize,
        args: Option<*mut *mut i8>,
        count: Option<usize>,
    ) -> usize {
        let function = unsafe { std::ffi::CStr::from_ptr(function).to_str().unwrap() };
        self.group
            .handle(function.to_string(), output, size, args, count)
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
    pub fn group(mut self, group: Group) -> Self {
        self.group = self.group.child(group.name().to_string(), group);
        self
    }

    #[inline]
    pub fn allow_no_args(mut self) -> Self {
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
