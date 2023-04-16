use std::{collections::HashMap, sync::Arc};

use crate::{
    command::{fn_handler, Factory, Handler},
    context::{Context, GroupContext},
    State,
};

#[derive(Default)]
/// A group of commands.
/// Called from Arma using `[group]:[command]`.
pub struct Group {
    commands: HashMap<String, Box<Handler>>,
    children: HashMap<String, Self>,
    state: State,
}

impl Group {
    #[must_use]
    /// Creates a new group
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            children: HashMap::new(),
            state: State::default(),
        }
    }

    #[inline]
    #[must_use]
    /// Add a new state value to the group if it has not be added already
    pub fn state<T>(self, state: T) -> Self
    where
        T: Send + Sync + 'static,
    {
        self.state.set(state);
        self
    }

    #[inline]
    #[must_use]
    /// Freeze the group's state, preventing the state from changing, allowing for faster reads
    pub fn freeze_state(mut self) -> Self {
        self.state.freeze();
        self
    }

    #[inline]
    #[must_use]
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
    #[must_use]
    /// Add a group to the group
    pub fn group<S>(mut self, name: S, child: Self) -> Self
    where
        S: Into<String>,
    {
        self.children.insert(name.into(), child);
        self
    }
}

pub(crate) struct InternalGroup {
    commands: HashMap<String, Box<Handler>>,
    children: HashMap<String, Self>,
    pub(crate) state: Arc<State>,
}

impl InternalGroup {
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
            (handler.handler)(
                context.with_group_ctx(GroupContext::new(self.state.clone())),
                output,
                size,
                args,
                count,
            )
        } else {
            1
        }
    }
}

impl From<Group> for InternalGroup {
    fn from(group: Group) -> Self {
        let children = group
            .children
            .into_iter()
            .map(|(name, group)| (name, Self::from(group)))
            .collect();
        Self {
            commands: group.commands,
            children,
            state: Arc::new(group.state),
        }
    }
}
