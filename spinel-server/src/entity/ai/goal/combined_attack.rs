use crate::entity::ai::goal::melee_attack::{cooldown_is_ready, duration_to_ticks};
use crate::entity::ai::{AiCooldown, GoalSelector, TargetSelector};
use crate::entity::{CreatureEntity, EntityId, ProjectileEntity};
use crate::world::WorldSnapshot;
use spinel_registry::EntityType;
use std::io::{Error, ErrorKind, Result};
use std::time::Duration;

type ProjectileGenerator = Box<dyn Fn(&CreatureEntity) -> ProjectileEntity + Send>;

pub struct CombinedAttackGoal {
    melee_range_squared: f64,
    melee_delay_ticks: u64,
    ranged_range_squared: f64,
    ranged_power: f64,
    ranged_spread: f64,
    ranged_delay_ticks: u64,
    desirable_range_squared: f64,
    should_come_close: bool,
    projectile_generator: ProjectileGenerator,
    cooldown: AiCooldown,
    last_attack_tick: Option<u64>,
    should_stop: bool,
    cached_target: Option<EntityId>,
}

impl CombinedAttackGoal {
    pub fn new(
        melee_range: i32,
        melee_delay: Duration,
        ranged_range: i32,
        ranged_power: f64,
        ranged_spread: f64,
        ranged_delay: Duration,
        desirable_range: i32,
        should_come_close: bool,
    ) -> Result<Self> {
        if desirable_range > ranged_range {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Desirable range cannot exceed ranged range.",
            ));
        }
        Ok(Self {
            melee_range_squared: f64::from(melee_range * melee_range),
            melee_delay_ticks: duration_to_ticks(melee_delay),
            ranged_range_squared: f64::from(ranged_range * ranged_range),
            ranged_power,
            ranged_spread,
            ranged_delay_ticks: duration_to_ticks(ranged_delay),
            desirable_range_squared: f64::from(desirable_range * desirable_range),
            should_come_close,
            projectile_generator: Box::new(|creature| {
                ProjectileEntity::new(Some(creature.entity_id()), EntityType::ARROW)
            }),
            cooldown: AiCooldown::new(Duration::from_millis(250)),
            last_attack_tick: None,
            should_stop: false,
            cached_target: None,
        })
    }

    pub const fn cooldown(&self) -> &AiCooldown {
        &self.cooldown
    }

    pub fn cooldown_mut(&mut self) -> &mut AiCooldown {
        &mut self.cooldown
    }

    pub fn set_projectile_generator(
        &mut self,
        projectile_generator: impl Fn(&CreatureEntity) -> ProjectileEntity + Send + 'static,
    ) {
        self.projectile_generator = Box::new(projectile_generator);
    }
}

impl GoalSelector for CombinedAttackGoal {
    fn should_start(
        &mut self,
        creature: &CreatureEntity,
        world: &WorldSnapshot,
        target_selectors: &mut [Box<dyn TargetSelector>],
    ) -> bool {
        self.cached_target = self.find_target(creature, world, target_selectors);
        self.cached_target.is_some()
    }

    fn start(
        &mut self,
        creature: &mut CreatureEntity,
        world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) {
        self.should_stop = false;
        if let Some(target_position) = self
            .cached_target
            .and_then(|target| world.entity(target))
            .map(|target| target.position())
        {
            creature.set_path_to_default(world, Some(target_position));
        }
    }

    fn tick(
        &mut self,
        creature: &mut CreatureEntity,
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
        let mut should_come_close = false;
        if distance_squared <= self.melee_range_squared
            && cooldown_is_ready(time, self.last_attack_tick, self.melee_delay_ticks)
        {
            creature.queue_attack(target.entity_id(), true);
            self.last_attack_tick = Some(time);
        } else if distance_squared <= self.ranged_range_squared
            && cooldown_is_ready(time, self.last_attack_tick, self.ranged_delay_ticks)
        {
            if world.has_line_of_sight(creature.entity_id(), target.entity_id()) {
                let projectile = (self.projectile_generator)(creature);
                let target_position =
                    target
                        .position()
                        .offset(0.0, target.entity_type().eye_height(), 0.0);
                creature.queue_projectile(
                    projectile,
                    target_position,
                    self.ranged_power,
                    self.ranged_spread,
                );
                self.last_attack_tick = Some(time);
            } else {
                should_come_close = self.should_come_close;
            }
        }
        if !should_come_close && distance_squared <= self.desirable_range_squared {
            creature.navigator_mut().reset();
            creature.look_at_position(target.position());
            return;
        }
        if creature.navigator().path_position() == Some(target.position())
            || !self.cooldown.is_ready(time)
        {
            return;
        }
        self.cooldown.refresh_last_update(time);
        creature.set_path_to_default(world, Some(target.position()));
    }

    fn should_end(
        &mut self,
        _creature: &CreatureEntity,
        _world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) -> bool {
        self.should_stop
    }

    fn end(
        &mut self,
        creature: &mut CreatureEntity,
        _world: &WorldSnapshot,
        _target_selectors: &mut [Box<dyn TargetSelector>],
    ) {
        creature.navigator_mut().reset();
    }
}
