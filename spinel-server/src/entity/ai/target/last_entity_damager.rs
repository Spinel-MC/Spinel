use crate::entity::ai::TargetSelector;
use crate::entity::{EntityCreature, EntityId};
use crate::world::WorldSnapshot;

pub struct LastEntityDamagerTarget {
    range: f64,
}

impl LastEntityDamagerTarget {
    pub const fn new(range: f64) -> Self {
        Self { range }
    }
}

impl TargetSelector for LastEntityDamagerTarget {
    fn find_target(
        &mut self,
        creature: &EntityCreature,
        world: &WorldSnapshot,
    ) -> Option<EntityId> {
        let source = creature.last_damage()?.source()?;
        let source = world.entity(source)?;
        let is_valid_target = !source.is_removed()
            && source.position().distance_squared(creature.position()) < self.range * self.range;
        is_valid_target.then_some(source.entity_id())
    }
}
