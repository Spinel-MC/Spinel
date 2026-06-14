use crate::entity::CreatureEntity;
use crate::entity::ai::{GoalSelector, TargetSelector};
use crate::world::WorldSnapshot;
use rand::Rng;
use std::time::{Duration, Instant};

pub struct DoNothingGoal {
    duration: Duration,
    chance: f32,
    started_at: Option<Instant>,
}

impl DoNothingGoal {
    pub fn new(duration_millis: u64, chance: f32) -> Self {
        Self {
            duration: Duration::from_millis(duration_millis),
            chance: chance.clamp(0.0, 1.0),
            started_at: None,
        }
    }
}

impl GoalSelector for DoNothingGoal {
    fn should_start(
        &mut self,
        _creature: &CreatureEntity,
        _world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) -> bool {
        rand::rng().random::<f32>() <= self.chance
    }

    fn start(
        &mut self,
        _creature: &mut CreatureEntity,
        _world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) {
        self.started_at = Some(Instant::now());
    }

    fn tick(
        &mut self,
        _creature: &mut CreatureEntity,
        _world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
        _time: u64,
    ) {
    }

    fn should_end(
        &mut self,
        _creature: &CreatureEntity,
        _world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) -> bool {
        self.started_at
            .is_some_and(|started_at| started_at.elapsed() >= self.duration)
    }

    fn end(
        &mut self,
        _creature: &mut CreatureEntity,
        _world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) {
        self.started_at = None;
    }
}
