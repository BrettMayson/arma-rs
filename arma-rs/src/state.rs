use std::{
    marker::{Send, Sync},
    sync::{Arc, RwLock},
};

struct Entry<T>(RwLock<T>);

#[derive(Default, Clone)]
pub struct State(Arc<state::Container![Send + Sync]>);

impl State {
    pub fn get<T>(&self) -> T
    where
        T: Clone + Send + Sync + 'static,
    {
        self.try_get().unwrap()
    }

    pub fn try_get<T>(&self) -> Option<T>
    where
        T: Clone + Send + Sync + 'static,
    {
        let entry = self.0.try_get::<Entry<T>>();
        entry.map(|x| x.0.read().unwrap().clone())
    }

    pub fn set<T>(&self, state: T)
    where
        T: Clone + Send + Sync + 'static,
    {
        if let Some(entry) = self.0.try_get::<Entry<T>>() {
            let mut field = entry.0.write().unwrap();
            *field = state
        } else {
            self.0.set(Entry(RwLock::new(state)));
        }
    }
}
