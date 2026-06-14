use crate::entity::EntityId;
use spinel_core::network::clientbound::play::attach_entity::AttachEntityPacket;
use std::collections::BTreeSet;

pub(crate) struct EntityLeash {
    holder: Option<EntityId>,
    leashed_entities: BTreeSet<EntityId>,
}

impl EntityLeash {
    pub(crate) const fn new() -> Self {
        Self {
            holder: None,
            leashed_entities: BTreeSet::new(),
        }
    }

    pub(crate) const fn holder(&self) -> Option<EntityId> {
        self.holder
    }

    pub(crate) fn leashed_entities(&self) -> &BTreeSet<EntityId> {
        &self.leashed_entities
    }

    pub(crate) fn set_holder(&mut self, holder: Option<EntityId>) {
        self.holder = holder;
    }

    pub(crate) fn add_leashed_entity(&mut self, entity_id: EntityId) -> bool {
        self.leashed_entities.insert(entity_id)
    }

    pub(crate) fn remove_leashed_entity(&mut self, entity_id: EntityId) -> bool {
        self.leashed_entities.remove(&entity_id)
    }

    pub(crate) fn packet(&self, entity_id: EntityId) -> AttachEntityPacket {
        AttachEntityPacket {
            attached_entity_id: entity_id.value(),
            holding_entity_id: self.holder.map(EntityId::value).unwrap_or(-1),
        }
    }
}
