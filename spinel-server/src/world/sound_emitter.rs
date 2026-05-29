use crate::entity::EntityId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldSoundEmitter {
    Entity(EntityId),
    SelfPlayer,
}
