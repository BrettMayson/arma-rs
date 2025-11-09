use std::{cell::RefCell, rc::Rc};

use crate::ContextRequest;

use super::CallContextStackTrace;

/// Manages requesting and replacing the ArmaCallContext
pub struct ArmaContextManager {
    pub(crate) request: RefCell<ContextRequest>,
    state: Rc<RefCell<Option<CallContextStackTrace>>>,
}

impl ArmaContextManager {
    /// Create a new ArmaContextManager
    pub fn new(request: ContextRequest) -> Self {
        Self {
            request: RefCell::new(request),
            state: Rc::new(RefCell::new(None)),
        }
    }

    /// Request a new ArmaCallContext from Arma
    pub fn request(&self) -> CallContextStackTrace {
        // When the request is called, Arma will send the request to the extension
        // The extension will set the state to the request it just received
        unsafe {
            (self.request.borrow())();
        }
        // When the request function returns, the state has been set by Arma
        // It can now be taken and sent to the Context
        self.state
            .replace(None)
            .unwrap_or_default()
    }

    /// Replace the current ArmaCallContext with a new one
    pub fn replace(&self, value: Option<CallContextStackTrace>) {
        *self.state.borrow_mut() = value;
    }
}
