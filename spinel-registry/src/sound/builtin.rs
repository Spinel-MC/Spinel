use crate::Identifier;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BuiltinSoundEvent {
    id: i32,
    key: Identifier,
}

impl BuiltinSoundEvent {
    pub const fn new(id: i32, key: Identifier) -> Self {
        Self { id, key }
    }

    pub const fn id(&self) -> i32 {
        self.id
    }

    pub const fn key(&self) -> &Identifier {
        &self.key
    }
}
