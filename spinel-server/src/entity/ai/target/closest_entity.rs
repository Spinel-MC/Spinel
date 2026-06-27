use crate::entity::ai::TargetSelector;
use crate::entity::{EntityCreature, EntityId, EntityObservation};
use crate::world::WorldSnapshot;

pub struct ClosestEntityTarget {
    range: f64,
    target_predicate: Box<dyn Fn(EntityObservation) -> bool + Send + Sync>,
}

impl ClosestEntityTarget {
    pub fn new(
        range: f64,
        target_predicate: impl Fn(EntityObservation) -> bool + Send + Sync + 'static,
    ) -> Self {
        Self {
            range,
            target_predicate: Box::new(target_predicate),
        }
    }
}

impl TargetSelector for ClosestEntityTarget {
    fn find_target(
        &mut self,
        creature: &EntityCreature,
        world: &WorldSnapshot,
    ) -> Option<EntityId> {
        let maximum_distance_squared = self.range * self.range;
        world
            .entities()
            .iter()
            .copied()
            .filter(|entity| entity.get_entity_id() != creature.get_entity_id())
            .filter(|entity| !entity.is_removed())
            .filter(|entity| (self.target_predicate)(*entity))
            .filter_map(|entity| {
                let distance_squared = entity
                    .get_position()
                    .get_distance_squared(creature.get_position());
                (distance_squared <= maximum_distance_squared)
                    .then_some((entity.get_entity_id(), distance_squared))
            })
            .min_by(|first, second| first.1.total_cmp(&second.1))
            .map(|(entity_id, _)| entity_id)
    }
}
