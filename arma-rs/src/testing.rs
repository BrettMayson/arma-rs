use std::{sync::Arc, time::Duration};

use crossbeam_queue::SegQueue;

use crate::{Context, Value};

pub struct Extension {
    pub ext: crate::Extension,
    callback_queue: Arc<SegQueue<(String, String, Option<Value>)>>,
}

const BUFFER_SIZE: libc::size_t = 10240;

impl Extension {
    #[must_use]
    pub fn new(ext: crate::Extension) -> Self {
        Self {
            ext,
            callback_queue: Arc::new(SegQueue::new()),
        }
    }

    #[must_use]
    /// Returns a context for simulating interactions with Arma
    pub fn context(&self) -> Context {
        Context::new(self.callback_queue.clone()).with_buffer_size(
            10240, // The sized used by Arma 3 as of 2021-12-30
        )
    }

    #[must_use]
    /// Call a function, intended for tests
    /// # Safety
    /// This function is unsafe because it interacts with the C API.
    pub unsafe fn call(&self, function: &str, args: Option<Vec<String>>) -> (String, libc::c_int) {
        let output = [0; BUFFER_SIZE].as_mut_ptr();
        let len = args.as_ref().map(|a| a.len().try_into().unwrap());
        let mut args_pointer = args.map(|v| {
            v.into_iter()
                .map(|s| std::ffi::CString::new(s).unwrap().into_raw())
                .collect::<Vec<*mut i8>>()
        });
        let res = self.ext.group.handle(
            self.context(),
            function,
            output,
            BUFFER_SIZE,
            args_pointer.as_mut().map(Vec::as_mut_ptr),
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
        F: Fn(&str, &str, Option<Value>) -> bool,
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
