use crate::entity::ai::{AiCooldown, GoalSelector, TargetSelector};
use crate::entity::pathfinding::PathRequest;
use crate::entity::{EntityCreature, EntityId};
use crate::world::WorldSnapshot;
use std::time::Duration;

pub struct MeleeAttackGoal {
    range_squared: f64,
    delay_ticks: u64,
    cooldown: AiCooldown,
    last_hit_tick: Option<u64>,
    should_stop: bool,
    cached_target: Option<EntityId>,
}

impl MeleeAttackGoal {
    pub fn new(range: f64, delay: Duration) -> Self {
        Self {
            range_squared: range * range,
            delay_ticks: duration_to_ticks(delay),
            cooldown: AiCooldown::new(Duration::from_millis(250)),
            last_hit_tick: None,
            should_stop: false,
            cached_target: None,
        }
    }

    pub const fn cooldown(&self) -> &AiCooldown {
        &self.cooldown
    }

    pub fn cooldown_mut(&mut self) -> &mut AiCooldown {
        &mut self.cooldown
    }
}

impl GoalSelector for MeleeAttackGoal {
    fn should_start(
        &mut self,
        creature: &EntityCreature,
        world: &WorldSnapshot,
        target_selectors: &mut [Box<dyn TargetSelector>],
    ) -> bool {
        self.cached_target = self.find_target(creature, world, target_selectors);
        self.cached_target.is_some()
    }

    fn start(
        &mut self,
        creature: &mut EntityCreature,
        world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) {
        self.should_stop = false;
        if let Some(target_position) = self
            .cached_target
            .and_then(|target| world.entity(target))
            .map(|target| target.position())
        {
            if creature
                .set_path_to_in_world(world, PathRequest::from(target_position))
                .is_err()
            {
                self.should_stop = true;
            }
        }
    }

    fn tick(
        &mut self,
        creature: &mut EntityCreature,
        world: &WorldSnapshot,
        target_selectors: &mut [Box<dyn TargetSelector>],
        time: u64,
    ) {
        let target = self
            .cached_target
            .take()
            .or_else(|| self.find_target(creature, world, target_selectors))
            .and_then(|target| world.entity(target));
        let Some(target) = target else {
            self.should_stop = true;
            return;
        };
        let distance_squared = creature.position().distance_squared(target.position());
        if distance_squared <= self.range_squared {
            creature.look_at_position(target.position());
            if cooldown_is_ready(time, self.last_hit_tick, self.delay_ticks) {
                creature.attack_entity_with_swing(target.entity_id());
                self.last_hit_tick = Some(time);
            }
            return;
        }
        if creature.navigator().path_position() == Some(target.position())
            || !self.cooldown.is_ready(time)
        {
            return;
        }
        self.cooldown.refresh_last_update(time);
        if creature
            .set_path_to_in_world(world, PathRequest::from(target.position()))
            .is_err()
        {
            self.should_stop = true;
        }
    }

    fn should_end(
        &mut self,
        _creature: &EntityCreature,
        _world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) -> bool {
        self.should_stop
    }

    fn end(
        &mut self,
        creature: &mut EntityCreature,
        _world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) {
        creature.navigator_mut().reset();
    }
}

pub(super) fn duration_to_ticks(duration: Duration) -> u64 {
    duration.as_millis().div_ceil(50) as u64
}

pub(super) fn cooldown_is_ready(time: u64, last_time: Option<u64>, delay: u64) -> bool {
    last_time.is_none_or(|last_time| time.saturating_sub(last_time) >= delay)
}
