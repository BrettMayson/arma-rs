//! For testing your extension.

use std::time::Duration;

use crate::{CallbackMessage, Context, State, Value};

use crate::{ArmaCallContext, Caller, Mission, Server, Source};

/// Wrapper around [`crate::Extension`] used for testing.
pub struct Extension(crate::Extension);

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

impl Extension {
    /// Create a new testing Extension
    pub fn new(ext: crate::Extension) -> Self {
        Self(ext)
    }

    #[must_use]
    /// Returns a context for simulating interactions with Arma
    pub fn context(&self) -> Context {
        self.0.context().with_buffer_size(BUFFER_SIZE)
    }

    #[must_use]
    /// Get a reference to the extensions state container
    pub fn state(&self) -> &State {
        &self.0.group.state
    }

    #[must_use]
    #[allow(clippy::too_many_arguments)]
    /// Call a function with Arma call context.
    ///
    /// # Safety
    /// This function is unsafe because it interacts with the C API.
    pub fn call_with_context(
        &self,
        function: &str,
        args: Option<Vec<String>>,
        caller: Caller,
        source: Source,
        mission: Mission,
        server: Server,
        remote_exec_owner: i16,
    ) -> (String, libc::c_int) {
        self.0.context_manager.replace(Some(ArmaCallContext::new(
            caller,
            source,
            mission,
            server,
            remote_exec_owner,
        )));
        unsafe { self.handle_call(function, args) }
    }

    #[must_use]
    /// Call a function without Arma call context.
    ///
    /// # Safety
    /// This function is unsafe because it interacts with the C API.
    ///
    /// # Note
    /// If the `call-context` feature is enabled, this function passes default values for each field.
    pub fn call(&self, function: &str, args: Option<Vec<String>>) -> (String, libc::c_int) {
        self.0.context_manager.replace(None);
        unsafe { self.handle_call(function, args) }
    }

    unsafe fn handle_call(
        &self,
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
        let res = self.0.group.handle(
            self.context(),
            &self.0.context_manager,
            function,
            output.as_mut_ptr(),
            BUFFER_SIZE,
            args_pointer.as_mut().map(Vec::as_mut_ptr),
            len,
        );
        if let Some(args) = args_pointer {
            for arg in args {
                let _ = std::ffi::CString::from_raw(arg);
            }
        }
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
    /// or `Result::Timeout` if either no event was received, or the handler
    /// returned `Result::Continue` until the timeout was reached.
    ///
    /// The handler must return a Result indicating the callback was handled to exit
    /// `Result::Continue` will continue to provide events to the handler until another variant is returned
    pub fn callback_handler<F, T, E>(&self, handler: F, timeout: Duration) -> Result<T, E>
    where
        F: Fn(&str, &str, Option<Value>) -> Result<T, E>,
    {
        let (_, rx) = &self.0.callback_channel;
        let deadline = std::time::Instant::now() + timeout;
        loop {
            match rx.recv_deadline(deadline) {
                Ok(CallbackMessage::Call(name, func, data)) => match handler(&name, &func, data) {
                    Result::Ok(value) => return Result::Ok(value),
                    Result::Err(error) => return Result::Err(error),
                    Result::Timeout => return Result::Timeout,
                    Result::Continue => {}
                },
                _ => return Result::Timeout,
            }
        }
    }
}
