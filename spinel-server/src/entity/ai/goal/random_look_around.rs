use crate::entity::ai::{GoalSelector, TargetSelector};
use crate::entity::{EntityCreature, EntityPosition};
use crate::world::WorldSnapshot;
use rand::Rng;
use std::f64::consts::TAU;

pub struct RandomLookAroundGoal {
    chance_per_tick: u32,
    minimum_look_time: Box<dyn FnMut() -> i32 + Send>,
    random_direction: Box<dyn FnMut(&EntityCreature) -> (f64, f64, f64) + Send>,
    look_direction: Option<(f64, f64, f64)>,
    remaining_look_ticks: i32,
}

impl RandomLookAroundGoal {
    pub fn new(chance_per_tick: u32) -> Self {
        Self::with_generators(
            chance_per_tick,
            || rand::rng().random_range(20..40),
            |_| {
                let angle = rand::rng().random_range(0.0..TAU);
                (angle.cos(), 0.0, angle.sin())
            },
        )
    }

    pub fn with_generators(
        chance_per_tick: u32,
        minimum_look_time: impl FnMut() -> i32 + Send + 'static,
        random_direction: impl FnMut(&EntityCreature) -> (f64, f64, f64) + Send + 'static,
    ) -> Self {
        Self {
            chance_per_tick,
            minimum_look_time: Box::new(minimum_look_time),
            random_direction: Box::new(random_direction),
            look_direction: None,
            remaining_look_ticks: 0,
        }
    }
}

impl GoalSelector for RandomLookAroundGoal {
    fn should_start(
        &mut self,
        creature: &EntityCreature,
        _world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) -> bool {
        let should_look =
            self.chance_per_tick > 0 && rand::rng().random_range(0..self.chance_per_tick) == 0;
        should_look && creature.get_navigator().get_path_position().is_none()
    }

    fn start(
        &mut self,
        creature: &mut EntityCreature,
        _world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) {
        self.remaining_look_ticks = (self.minimum_look_time)();
        self.look_direction = Some((self.random_direction)(creature));
    }

    fn tick(
        &mut self,
        creature: &mut EntityCreature,
        _world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
        _time: u64,
    ) {
        self.remaining_look_ticks -= 1;
        let Some((direction_x, direction_y, direction_z)) = self.look_direction else {
            return;
        };
        let horizontal_distance = direction_x.hypot(direction_z);
        let yaw = (-direction_x.atan2(direction_z).to_degrees()) as f32;
        let pitch = (-direction_y.atan2(horizontal_distance).to_degrees()) as f32;
        let position = creature.get_position();
        creature.set_position(EntityPosition::new(
            position.get_x(),
            position.get_y(),
            position.get_z(),
            yaw,
            pitch,
        ));
    }

    fn should_end(
        &mut self,
        _creature: &EntityCreature,
        _world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) -> bool {
        self.remaining_look_ticks < 0
    }

    fn end(
        &mut self,
        _creature: &mut EntityCreature,
        _world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) {
    }
}
