use crate::Identifier;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VillagerProfession {
    protocol_id: i32,
    key: Identifier,
}

impl VillagerProfession {
    pub const fn new(protocol_id: i32, key: Identifier) -> Self {
        Self { protocol_id, key }
    }

    pub const fn protocol_id(&self) -> i32 {
        self.protocol_id
    }

    pub const fn key(&self) -> &Identifier {
        &self.key
    }
}
