use crate::entity::{EntityId, EntityPosition, ProjectileEntity};

pub(crate) enum CreatureAiAction {
    Attack {
        source: EntityId,
        target: EntityId,
        should_swing_main_hand: bool,
    },
    Shoot {
        shooter: EntityId,
        projectile: ProjectileEntity,
        target: EntityPosition,
        power: f64,
        spread: f64,
    },
}
