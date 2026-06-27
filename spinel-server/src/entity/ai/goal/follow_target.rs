use crate::entity::ai::{GoalSelector, TargetSelector};
use crate::entity::pathfinding::PathRequest;
use crate::entity::{EntityCreature, EntityId, EntityPosition};
use crate::world::WorldSnapshot;
use std::time::Duration;

pub struct FollowTargetGoal {
    path_update_ticks: u64,
    last_update_tick: u64,
    should_force_end: bool,
    last_target_position: Option<EntityPosition>,
    target: Option<EntityId>,
}

impl FollowTargetGoal {
    pub fn new(path_duration: Duration) -> Self {
        Self {
            path_update_ticks: duration_to_ticks(path_duration),
            last_update_tick: 0,
            should_force_end: false,
            last_target_position: None,
            target: None,
        }
    }
}

impl GoalSelector for FollowTargetGoal {
    fn should_start(
        &mut self,
        creature: &EntityCreature,
        world: &WorldSnapshot,
        target_selectors: &mut [Box<dyn TargetSelector>],
    ) -> bool {
        let target = creature
            .get_target()
            .or_else(|| self.find_target(creature, world, target_selectors));
        let Some(target) = target.and_then(|target| world.get_entity(target)) else {
            return false;
        };
        let should_follow = target
            .get_position()
            .get_distance_squared(creature.get_position())
            >= 4.0;
        if should_follow {
            self.target = Some(target.get_entity_id());
        }
        should_follow
    }

    fn start(
        &mut self,
        creature: &mut EntityCreature,
        world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) {
        self.last_update_tick = 0;
        self.should_force_end = false;
        self.last_target_position = None;
        let Some(target) = self.target.and_then(|target| world.get_entity(target)) else {
            self.should_force_end = true;
            return;
        };
        creature.set_target(Some(target.get_entity_id()));
        self.last_target_position = Some(target.get_position());
        if target
            .get_position()
            .get_distance_squared(creature.get_position())
            < 4.0
        {
            self.should_force_end = true;
            creature.get_navigator_mut().reset();
            return;
        }
        if creature.get_navigator().get_path_position() == Some(target.get_position()) {
            self.should_force_end = true;
            return;
        }
        if creature
            .set_path_to_in_world(world, PathRequest::from(target.get_position()))
            .is_err()
        {
            self.should_force_end = true;
        }
    }

    fn tick(
        &mut self,
        creature: &mut EntityCreature,
        world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
        time: u64,
    ) {
        if self.should_force_end
            || self.path_update_ticks == 0
            || self.last_update_tick + self.path_update_ticks > time
        {
            return;
        }
        let Some(target_position) = creature
            .get_target()
            .and_then(|target| world.get_entity(target))
            .map(|target| target.get_position())
        else {
            return;
        };
        if self
            .last_target_position
            .is_some_and(|position| same_block(position, target_position))
        {
            return;
        }
        self.last_update_tick = time;
        self.last_target_position = Some(target_position);
        if creature
            .set_path_to_in_world(world, PathRequest::from(target_position))
            .is_err()
        {
            self.should_force_end = true;
        }
    }

    fn should_end(
        &mut self,
        creature: &EntityCreature,
        world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) -> bool {
        self.should_force_end
            || creature.get_target().is_none_or(|target| {
                world.get_entity(target).is_none_or(|target| {
                    target.is_removed()
                        || target
                            .get_position()
                            .get_distance_squared(creature.get_position())
                            < 4.0
                })
            })
    }

    fn end(
        &mut self,
        creature: &mut EntityCreature,
        _world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) {
        creature.get_navigator_mut().reset();
    }
}

fn duration_to_ticks(duration: Duration) -> u64 {
    duration.as_millis().div_ceil(50) as u64
}

fn same_block(first: EntityPosition, second: EntityPosition) -> bool {
    first.get_x().floor() == second.get_x().floor()
        && first.get_y().floor() == second.get_y().floor()
        && first.get_z().floor() == second.get_z().floor()
}
