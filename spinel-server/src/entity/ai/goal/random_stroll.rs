use crate::entity::ai::{GoalSelector, TargetSelector};
use crate::entity::pathfinding::PathRequest;
use crate::entity::{EntityCreature, EntityPosition};
use crate::world::WorldSnapshot;
use rand::seq::IndexedRandom;
use std::time::{Duration, Instant};

pub struct RandomStrollGoal {
    radius: i32,
    offsets: Vec<(i32, i32, i32)>,
    last_stroll: Option<Instant>,
}

impl RandomStrollGoal {
    pub fn new(radius: i32) -> Self {
        let offsets = (-radius..=radius)
            .flat_map(|offset_x| {
                (-radius..=radius).flat_map(move |offset_y| {
                    (-radius..=radius).map(move |offset_z| (offset_x, offset_y, offset_z))
                })
            })
            .collect();
        Self {
            radius,
            offsets,
            last_stroll: None,
        }
    }

    pub const fn radius(&self) -> i32 {
        self.radius
    }
}

impl GoalSelector for RandomStrollGoal {
    fn should_start(
        &mut self,
        _creature: &EntityCreature,
        _world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) -> bool {
        self.last_stroll
            .is_none_or(|last_stroll| last_stroll.elapsed() >= Duration::from_millis(2500))
    }

    fn start(
        &mut self,
        creature: &mut EntityCreature,
        world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) {
        let mut remaining_offsets = self.offsets.clone();
        let mut random = rand::rng();
        while let Some(offset) = remaining_offsets.choose(&mut random).copied() {
            let offset_index = remaining_offsets
                .iter()
                .position(|candidate| *candidate == offset)
                .unwrap_or_default();
            remaining_offsets.swap_remove(offset_index);
            let position = creature.position();
            let target = EntityPosition::new(
                position.x() + f64::from(offset.0),
                position.y() + f64::from(offset.1),
                position.z() + f64::from(offset.2),
                position.yaw(),
                position.pitch(),
            );
            if creature
                .set_path_to_in_world(world, PathRequest::from(target))
                .is_ok_and(|path_was_accepted| path_was_accepted)
            {
                break;
            }
        }
    }

    fn tick(
        &mut self,
        _creature: &mut EntityCreature,
        _world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
        _time: u64,
    ) {
    }

    fn should_end(
        &mut self,
        _creature: &EntityCreature,
        _world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) -> bool {
        true
    }

    fn end(
        &mut self,
        _creature: &mut EntityCreature,
        _world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) {
        self.last_stroll = Some(Instant::now());
    }
}
