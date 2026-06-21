use crate::entity::ai::goal::melee_attack::{cooldown_is_ready, duration_to_ticks};
use crate::entity::ai::{AiCooldown, GoalSelector, TargetSelector};
use crate::entity::pathfinding::PathRequest;
use crate::entity::{EntityCreature, EntityId, ProjectileEntity};
use crate::world::WorldSnapshot;
use spinel_registry::EntityType;
use std::io::{Error, ErrorKind, Result};
use std::time::Duration;

type ProjectileGenerator = Box<dyn Fn(&EntityCreature) -> ProjectileEntity + Send>;

pub struct RangedAttackGoal {
    delay_ticks: u64,
    attack_range_squared: f64,
    desirable_range_squared: f64,
    should_come_close: bool,
    power: f64,
    spread: f64,
    projectile_generator: ProjectileGenerator,
    cooldown: AiCooldown,
    last_shot_tick: Option<u64>,
    should_stop: bool,
    cached_target: Option<EntityId>,
}

impl RangedAttackGoal {
    pub fn new(
        delay: Duration,
        attack_range: i32,
        desirable_range: i32,
        should_come_close: bool,
        power: f64,
        spread: f64,
    ) -> Result<Self> {
        if desirable_range > attack_range {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Desirable range cannot exceed attack range.",
            ));
        }
        Ok(Self {
            delay_ticks: duration_to_ticks(delay),
            attack_range_squared: f64::from(attack_range * attack_range),
            desirable_range_squared: f64::from(desirable_range * desirable_range),
            should_come_close,
            power,
            spread,
            projectile_generator: Box::new(|creature| {
                ProjectileEntity::new(Some(creature.get_entity_id()), EntityType::ARROW)
            }),
            cooldown: AiCooldown::new(Duration::from_millis(250)),
            last_shot_tick: None,
            should_stop: false,
            cached_target: None,
        })
    }

    pub const fn get_cooldown(&self) -> &AiCooldown {
        &self.cooldown
    }

    pub fn get_cooldown_mut(&mut self) -> &mut AiCooldown {
        &mut self.cooldown
    }

    pub fn set_projectile_generator(
        &mut self,
        projectile_generator: impl Fn(&EntityCreature) -> ProjectileEntity + Send + 'static,
    ) {
        self.projectile_generator = Box::new(projectile_generator);
    }
}

impl GoalSelector for RangedAttackGoal {
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
            .and_then(|target| world.get_entity(target))
            .map(|target| target.get_position())
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
            .and_then(|target| world.get_entity(target));
        let Some(target) = target else {
            self.should_stop = true;
            return;
        };
        let distance_squared = creature.get_position().get_distance_squared(target.get_position());
        let mut should_come_close = false;
        if distance_squared <= self.attack_range_squared
            && cooldown_is_ready(time, self.last_shot_tick, self.delay_ticks)
        {
            if world.has_line_of_sight(creature.get_entity_id(), target.get_entity_id()) {
                let projectile = (self.projectile_generator)(creature);
                let target_position =
                    target
                        .get_position()
                        .get_offset(0.0, target.get_entity_type().get_eye_height(), 0.0);
                creature.queue_projectile(projectile, target_position, self.power, self.spread);
                self.last_shot_tick = Some(time);
            } else {
                should_come_close = self.should_come_close;
            }
        }
        if !should_come_close && distance_squared <= self.desirable_range_squared {
            creature.get_navigator_mut().reset();
            creature.look_at_position(target.get_position());
            return;
        }
        if creature.get_navigator().get_path_position() == Some(target.get_position())
            || !self.cooldown.is_ready(time)
        {
            return;
        }
        self.cooldown.refresh_last_update(time);
        if creature
            .set_path_to_in_world(world, PathRequest::from(target.get_position()))
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
        creature.get_navigator_mut().reset();
    }
}
