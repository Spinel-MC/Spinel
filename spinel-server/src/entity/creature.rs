use crate::entity::ai::{CreatureAiAction, EntityAiGroup};
use crate::entity::pathfinding::{Navigator, NodeFollowerPhysicsTiming};
use crate::entity::{EntityId, EntityPosition, GenericEntity};
use crate::world::WorldSnapshot;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};
use uuid::Uuid;

pub struct CreatureEntity {
    entity: GenericEntity,
    ai_groups: Vec<EntityAiGroup>,
    navigator: Navigator,
    target: Option<EntityId>,
    pending_ai_actions: Vec<CreatureAiAction>,
    removal_animation_delay_millis: i32,
}

impl CreatureEntity {
    pub fn new(entity_type: EntityType) -> Self {
        Self {
            entity: GenericEntity::new(entity_type),
            ai_groups: Vec::new(),
            navigator: Navigator::default(),
            target: None,
            pending_ai_actions: Vec::new(),
            removal_animation_delay_millis: 1000,
        }
    }

    pub const fn entity(&self) -> &GenericEntity {
        &self.entity
    }

    pub fn entity_mut(&mut self) -> &mut GenericEntity {
        &mut self.entity
    }

    pub const fn navigator(&self) -> &Navigator {
        &self.navigator
    }

    pub fn navigator_mut(&mut self) -> &mut Navigator {
        &mut self.navigator
    }

    pub fn ai_groups(&self) -> &[EntityAiGroup] {
        &self.ai_groups
    }

    pub fn ai_groups_mut(&mut self) -> &mut Vec<EntityAiGroup> {
        &mut self.ai_groups
    }

    pub fn add_ai_group(&mut self, group: EntityAiGroup) {
        self.ai_groups.push(group);
    }

    pub const fn target(&self) -> Option<EntityId> {
        self.target
    }

    pub fn set_target(&mut self, target: Option<EntityId>) {
        self.target = target;
    }

    pub const fn removal_animation_delay_millis(&self) -> i32 {
        self.removal_animation_delay_millis
    }

    pub fn set_removal_animation_delay_millis(&mut self, removal_animation_delay_millis: i32) {
        self.removal_animation_delay_millis = removal_animation_delay_millis;
    }

    pub fn kill(&mut self) -> bool {
        if !self.entity.kill() {
            return false;
        }
        if self.removal_animation_delay_millis <= 0 {
            self.entity.remove();
            return true;
        }
        self.entity
            .schedule_remove_after_duration(std::time::Duration::from_millis(
                self.removal_animation_delay_millis as u64,
            ));
        true
    }

    pub(crate) fn queue_attack(&mut self, target: EntityId, should_swing_main_hand: bool) {
        self.pending_ai_actions.push(CreatureAiAction::Attack {
            source: self.entity_id(),
            target,
            should_swing_main_hand,
        });
    }

    pub(crate) fn queue_projectile(
        &mut self,
        projectile: crate::entity::ProjectileEntity,
        target: EntityPosition,
        power: f64,
        spread: f64,
    ) {
        self.pending_ai_actions.push(CreatureAiAction::Shoot {
            shooter: self.entity_id(),
            projectile,
            target,
            power,
            spread,
        });
    }

    pub(crate) fn take_ai_actions(&mut self) -> Vec<CreatureAiAction> {
        std::mem::take(&mut self.pending_ai_actions)
    }

    pub fn set_path_to_default(
        &mut self,
        world: &WorldSnapshot,
        goal: Option<EntityPosition>,
    ) -> bool {
        if self.entity.world().is_none() {
            return false;
        }
        let start = self.entity.position();
        let bounding_box = self.entity.bounding_box();
        let is_on_ground = self.entity.is_on_ground();
        self.navigator
            .set_path_to_default(world, start, goal, bounding_box, is_on_ground)
    }

    pub fn set_path_to_with_completion(
        &mut self,
        world: &WorldSnapshot,
        goal: Option<EntityPosition>,
        minimum_distance: f64,
        on_complete: Option<Box<dyn FnOnce() + Send>>,
    ) -> bool {
        if self.entity.world().is_none() {
            return false;
        }
        let start = self.entity.position();
        let bounding_box = self.entity.bounding_box();
        let is_on_ground = self.entity.is_on_ground();
        self.navigator.set_path_to_with_completion(
            world,
            start,
            goal,
            bounding_box,
            is_on_ground,
            minimum_distance,
            on_complete,
        )
    }

    pub fn set_path_to(
        &mut self,
        world: &WorldSnapshot,
        goal: Option<EntityPosition>,
        minimum_distance: f64,
        maximum_distance: f64,
        variance: f64,
        on_complete: Option<Box<dyn FnOnce() + Send>>,
    ) -> bool {
        if self.entity.world().is_none() {
            return false;
        }
        let start = self.entity.position();
        let bounding_box = self.entity.bounding_box();
        let is_on_ground = self.entity.is_on_ground();
        self.navigator.set_path_to(
            world,
            start,
            goal,
            bounding_box,
            is_on_ground,
            minimum_distance,
            maximum_distance,
            variance,
            on_complete,
        )
    }

    pub fn ai_tick(&mut self, world: &WorldSnapshot, time: u64) {
        let mut ai_groups = std::mem::take(&mut self.ai_groups);
        ai_groups
            .iter_mut()
            .for_each(|group| group.tick(self, world, time));
        self.ai_groups = ai_groups;
    }

    pub(crate) fn set_world(&mut self, world: Uuid) {
        self.navigator.reset();
        self.entity.set_world(world);
    }

    pub(crate) fn tick_before_movement(&mut self, world: &WorldSnapshot, time: u64) {
        if self.navigator.physics_timing() != NodeFollowerPhysicsTiming::BeforePhysics {
            return;
        }
        self.tick_navigation(world, time);
    }

    pub(crate) fn tick_after_movement(&mut self, world: &WorldSnapshot, time: u64) {
        if self.navigator.physics_timing() == NodeFollowerPhysicsTiming::AfterPhysics {
            self.tick_navigation(world, time);
        }
        self.entity.tick();
    }

    pub fn tick(&mut self, world: &WorldSnapshot, time: u64) {
        self.tick_navigation(world, time);
        self.entity.tick();
    }

    fn tick_navigation(&mut self, world: &WorldSnapshot, time: u64) {
        self.ai_tick(world, time);
        let entity_is_dead = self.entity.is_dead();
        self.navigator.tick(&mut self.entity, world, entity_is_dead);
    }
}

impl Deref for CreatureEntity {
    type Target = GenericEntity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}

impl DerefMut for CreatureEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
