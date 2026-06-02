use crate::entity::EntityId;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct EntityEventNode {
    listeners: BTreeMap<&'static str, Vec<Arc<dyn Fn(EntityId) + Send + Sync>>>,
}

impl EntityEventNode {
    pub fn listen(
        &mut self,
        event_name: &'static str,
        listener: impl Fn(EntityId) + Send + Sync + 'static,
    ) {
        self.listeners
            .entry(event_name)
            .or_default()
            .push(Arc::new(listener));
    }

    pub fn listener_count(&self, event_name: &'static str) -> usize {
        self.listeners.get(event_name).map_or(0, Vec::len)
    }

    pub fn dispatch(&self, event_name: &'static str, entity_id: EntityId) {
        let Some(listeners) = self.listeners.get(event_name) else {
            return;
        };
        listeners.iter().for_each(|listener| listener(entity_id));
    }
}

impl std::fmt::Debug for EntityEventNode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("EntityEventNode")
            .field("event_count", &self.listeners.len())
            .finish()
    }
}
