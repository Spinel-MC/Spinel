use crate::world::World;
use std::collections::HashMap;

type WorldEventCallback = Box<dyn FnMut(&mut World) + Send>;

#[derive(Default)]
pub struct WorldEventNode {
    listeners: HashMap<&'static str, Vec<WorldEventCallback>>,
}

impl WorldEventNode {
    pub fn listen(
        &mut self,
        event_name: &'static str,
        callback: impl FnMut(&mut World) + Send + 'static,
    ) {
        self.listeners
            .entry(event_name)
            .or_default()
            .push(Box::new(callback));
    }

    pub fn listener_count(&self, event_name: &'static str) -> usize {
        self.listeners
            .get(event_name)
            .map(Vec::len)
            .unwrap_or_default()
    }

    pub(crate) fn dispatch(&mut self, event_name: &'static str, world: &mut World) {
        let Some(mut listeners) = self.listeners.remove(event_name) else {
            return;
        };
        listeners.iter_mut().for_each(|listener| listener(world));
        self.listeners.insert(event_name, listeners);
    }
}
