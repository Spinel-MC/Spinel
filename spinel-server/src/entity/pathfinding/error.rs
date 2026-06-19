use crate::entity::EntityPosition;

#[derive(Debug, Clone, Copy, PartialEq, thiserror::Error)]
pub enum SetPathToError {
    #[error("entity is not assigned to a world")]
    EntityHasNoWorld,

    #[error("target position {target:?} is outside the world border")]
    TargetOutsideWorldBorder { target: EntityPosition },

    #[error("target position {target:?} is in an unloaded chunk")]
    TargetChunkUnloaded { target: EntityPosition },
}
