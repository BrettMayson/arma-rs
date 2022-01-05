use std::collections::HashMap;

use crate::{
    command::{fn_handler, Factory, Handler},
    Context,
};

#[derive(Default)]
/// A group of commands.
/// Called from Arma using `[group]:[command]`.
pub struct Group {
    commands: HashMap<String, Box<Handler>>,
    children: HashMap<String, Self>,
}

impl Group {
    #[must_use]
    /// Creates a new group
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            children: HashMap::new(),
        }
    }

    #[inline]
    /// Add a command to the group
    pub fn command<S, F, I, R>(mut self, name: S, handler: F) -> Self
    where
        S: Into<String>,
        F: Factory<I, R> + 'static,
    {
        self.commands
            .insert(name.into(), Box::new(fn_handler(handler)));
        self
    }

    #[inline]
    /// Add a group to the group
    pub fn group<S>(mut self, name: S, child: Self) -> Self
    where
        S: Into<String>,
    {
        self.children.insert(name.into(), child);
        self
    }

    pub(crate) fn handle(
        &self,
        context: Context,
        function: &str,
        output: *mut libc::c_char,
        size: libc::size_t,
        args: Option<*mut *mut i8>,
        count: Option<libc::c_int>,
    ) -> libc::c_int {
        if let Some((group, function)) = function.split_once(':') {
            self.children.get(group).map_or(1, |group| {
                group.handle(context, function, output, size, args, count)
            })
        } else if let Some(handler) = self.commands.get(function) {
            (handler.handler)(context, output, size, args, count)
        } else {
            1
        }
    }
}
