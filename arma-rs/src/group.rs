use std::collections::HashMap;

use crate::{
    command::{fn_handler, CommandFactory, CommandHandler},
    Context,
};

#[derive(Default)]
pub struct Group {
    commands: HashMap<String, Box<CommandHandler>>,
    children: HashMap<String, Self>,
}

impl Group {
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
        F: CommandFactory<I, R> + 'static,
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
        function: String,
        output: *mut libc::c_char,
        size: libc::c_int,
        args: Option<*mut *mut i8>,
        count: Option<libc::c_int>,
    ) -> libc::c_int {
        if let Some((group, function)) = function.split_once(':') {
            if let Some(group) = self.children.get(group) {
                group.handle(context, function.to_string(), output, size, args, count)
            } else {
                1
            }
        } else if let Some(handler) = self.commands.get(&function) {
            (handler.handler)(context, output, size, args, count)
        } else {
            1
        }
    }
}
