use std::{sync::Arc, time::Duration};

use crossbeam_queue::SegQueue;

use crate::{ArmaValue, Context, Extension};

pub struct TestingExtension {
    pub ext: Extension,
    callback_queue: Arc<SegQueue<(String, String, Option<ArmaValue>)>>,
}

impl TestingExtension {
    pub fn new(ext: Extension) -> Self {
        Self {
            ext,
            callback_queue: Arc::new(SegQueue::new()),
        }
    }

    /// Returns a context for simulating interactions with Arma
    pub fn context(&self) -> Context {
        Context::new(self.callback_queue.clone())
    }

    /// Call a function, intended for tests
    /// # Safety
    /// This function is unsafe because it interacts with the C API.
    pub unsafe fn call(&self, function: &str, args: Option<Vec<String>>) -> (String, libc::c_int) {
        let output = std::ffi::CString::new("").unwrap().into_raw();
        let len = args.as_ref().map(|a| a.len().try_into().unwrap());
        let mut args_pointer = args.map(|v| {
            v.into_iter()
                .map(|s| std::ffi::CString::new(s).unwrap().into_raw())
                .collect::<Vec<*mut i8>>()
        });
        let res = self.ext.group.handle(
            self.context(),
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

    /// Create a callback handler
    ///
    /// Returns true if the callback was handled
    /// returns false if the timeout is reached
    ///
    /// The handler must return a boolean indicating whether the callback was handled
    /// Return true to end the callback loop
    /// Return false to continue the callback loop
    pub fn callback_handler<F>(&self, handler: F, timeout: Duration) -> bool
    where
        F: Fn(&str, &str, Option<ArmaValue>) -> bool,
    {
        let queue = self.callback_queue.clone();
        let start = std::time::Instant::now();
        loop {
            if let Some((name, func, data)) = queue.pop() {
                if handler(&name, &func, data) {
                    return true;
                }
            }
            if start.elapsed() > timeout {
                return false;
            }
        }
    }
}
