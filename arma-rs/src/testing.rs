use std::{sync::Arc, time::Duration};

use crossbeam_queue::SegQueue;

use crate::{Context, Value};

pub struct Extension {
    pub ext: crate::Extension,
    callback_queue: Arc<SegQueue<(String, String, Option<Value>)>>,
}

const BUFFER_SIZE: libc::size_t = 10240;

#[derive(Debug, PartialEq, Eq)]
/// Result of an event handler
pub enum Result<T, E> {
    /// an event has been handled and the handler is done, the value of T is the return value of the event handler
    Ok(T),
    /// the handler has encountered an error, the value of T is the return value of the event handler
    Err(E),
    /// an event is handled but the handler is not done and should receive another event
    Continue,
    /// the handler reached the specified timeout
    Timeout,
}

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
            10240 - 8, // The sized used by Arma 3 as of 2021-12-30
        )
    }

    #[must_use]
    /// Call a function, intended for tests
    ///
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
    /// Returns a Result from the handler if the callback was handled,
    /// or `Result::Timeout` if either no event was recieved,or the handler
    /// returned `Result::Continue` until the timeout was reached.
    ///
    /// The handler must return a Result indicating the callback was handled to exit
    /// `Result::Continue` will continue to provide events to the handler until another variant is returned
    pub fn callback_handler<F, T, E>(&self, handler: F, timeout: Duration) -> Result<T, E>
    where
        F: Fn(&str, &str, Option<Value>) -> Result<T, E>,
    {
        let queue = self.callback_queue.clone();
        let start = std::time::Instant::now();
        loop {
            if let Some((name, func, data)) = queue.pop() {
                match handler(&name, &func, data) {
                    Result::Ok(value) => return Result::Ok(value),
                    Result::Err(error) => return Result::Err(error),
                    Result::Timeout => return Result::Timeout,
                    Result::Continue => {}
                }
            }
            if start.elapsed() > timeout {
                return Result::Timeout;
            }
        }
    }
}
