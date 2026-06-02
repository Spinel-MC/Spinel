use std::sync::atomic::{AtomicI32, Ordering};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityId(i32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EntityIdentity {
    uuid: uuid::Uuid,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EntityPointers {
    uuid: uuid::Uuid,
    entity_id: EntityId,
}

impl EntityId {
    pub const fn from_raw(entity_id: i32) -> Self {
        Self(entity_id)
    }

    pub fn next() -> Self {
        static LAST_ENTITY_ID: AtomicI32 = AtomicI32::new(0);
        Self(LAST_ENTITY_ID.fetch_add(1, Ordering::SeqCst) + 1)
    }

    pub const fn value(self) -> i32 {
        self.0
    }
}

impl EntityIdentity {
    pub const fn new(uuid: uuid::Uuid) -> Self {
        Self { uuid }
    }

    pub const fn uuid(self) -> uuid::Uuid {
        self.uuid
    }
}

impl EntityPointers {
    pub const fn new(uuid: uuid::Uuid, entity_id: EntityId) -> Self {
        Self { uuid, entity_id }
    }

    pub const fn uuid(self) -> uuid::Uuid {
        self.uuid
    }

    pub const fn entity_id(self) -> EntityId {
        self.entity_id
    }

    pub const fn identity(self) -> EntityIdentity {
        EntityIdentity::new(self.uuid)
    }
}
