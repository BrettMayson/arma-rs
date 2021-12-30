use std::sync::Arc;

use crossbeam_queue::SegQueue;

use crate::{ArmaValue, IntoArma};

pub struct Context {
    pub(crate) queue: Arc<SegQueue<(String, String, Option<ArmaValue>)>>,
}

impl Context {
    pub fn new(queue: Arc<SegQueue<(String, String, Option<ArmaValue>)>>) -> Self {
        Self { queue }
    }

    /// Sends a callback into Arma
    /// https://community.bistudio.com/wiki/Arma_3:_Mission_Event_Handlers#ExtensionCallback
    pub fn callback<V>(&self, name: &str, func: &str, data: Option<V>)
    where
        V: IntoArma,
    {
        self.queue
            .push((name.to_string(), func.to_string(), Some(data.into())));
    }
}
