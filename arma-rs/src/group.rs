use std::collections::HashMap;

use crate::{
    command::{fn_handler, Factory, Handler},
    Context,
};

#[derive(Default)]
/// A group of commands.
/// Called from Arma using `[group]:[command]`.
pub struct Group<S> {
    commands: HashMap<String, Box<Handler<S>>>,
    children: HashMap<String, Self>,
}

impl<S> Group<S> {
    #[must_use]
    /// Creates a new group
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            children: HashMap::new(),
        }
    }

    #[inline]
    #[must_use]
    /// Add a command to the group
    pub fn command<F, I, R>(mut self, name: impl Into<String>, handler: F) -> Self
    where
        F: Factory<I, S, R> + 'static,
    {
        self.commands
            .insert(name.into(), Box::new(fn_handler(handler)));
        self
    }

    #[inline]
    #[must_use]
    /// Add a group to the group
    pub fn group(mut self, name: impl Into<String>, child: Self) -> Self {
        self.children.insert(name.into(), child);
        self
    }

    pub(crate) fn handle(
        &self,
        context: Context<S>,
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
