use std::time::Duration;

use crate::{Context, Value};

pub struct Extension<S>(crate::Extension<S>);

const BUFFER_SIZE: libc::size_t = 10240; // The sized used by Arma 3 as of 2021-12-30

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

impl<T, E> Result<T, E> {
    /// Returns true if the result is an ok result
    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Ok(_))
    }

    /// Returns true if the result is an error
    pub fn is_err(&self) -> bool {
        matches!(self, Self::Err(_))
    }

    /// Returns true if the result is a continue result
    pub fn is_continue(&self) -> bool {
        matches!(self, Self::Continue)
    }

    /// Returns true if the result is a timeout result
    pub fn is_timeout(&self) -> bool {
        matches!(self, Self::Timeout)
    }
}

impl<S> Extension<S> {
    pub fn new(ext: crate::Extension<S>) -> Self {
        Self(ext)
    }

    #[must_use]
    /// Returns a context for simulating interactions with Arma
    pub fn context(&self) -> Context {
        Context::new(self.0.callback_queue.clone()).with_buffer_size(BUFFER_SIZE)
    }

    #[must_use]
    /// Call a function, intended for tests
    ///
    /// # Safety
    /// This function is unsafe because it interacts with the C API.
    pub unsafe fn call(
        &mut self,
        function: &str,
        args: Option<Vec<String>>,
    ) -> (String, libc::c_int) {
        let mut output = [0; BUFFER_SIZE];
        let len = args.as_ref().map(|a| a.len().try_into().unwrap());
        let mut args_pointer = args.map(|v| {
            v.into_iter()
                .map(|s| std::ffi::CString::new(s).unwrap().into_raw())
                .collect::<Vec<*mut i8>>()
        });
        let context = self.context();
        let res = self.0.group.handle(
            &mut self.0.state,
            context,
            function,
            output.as_mut_ptr(),
            BUFFER_SIZE,
            args_pointer.as_mut().map(Vec::as_mut_ptr),
            len,
        );
        (
            std::ffi::CStr::from_ptr(output.as_ptr())
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
        let queue = self.0.callback_queue.clone();
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
