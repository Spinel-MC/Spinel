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
        let source = creature.get_last_damage()?.get_source()?;
        let source = world.get_entity(source)?;
        let is_valid_target = !source.is_removed()
            && source.get_position().get_distance_squared(creature.get_position()) < self.range * self.range;
        is_valid_target.then_some(source.get_entity_id())
    }
}
