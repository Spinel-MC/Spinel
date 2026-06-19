use crate::entity::ai::{CreatureAiAction, EntityAiGroup};
use crate::entity::metadata::EntityMeta;
use crate::entity::pathfinding::{
    Navigator, NodeFollowerPhysicsTiming, PathRequest, SetPathToError,
};
use crate::entity::{Entity, EntityId, EntityPosition, GenericEntity};
use crate::world::{World, WorldSnapshot};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};
use uuid::Uuid;

pub struct EntityCreature {
    entity: GenericEntity,
    ai_groups: Vec<EntityAiGroup>,
    navigator: Navigator,
    target: Option<EntityId>,
    pending_path_request: Option<PathRequest>,
    last_path_error: Option<SetPathToError>,
    pending_ai_actions: Vec<CreatureAiAction>,
    removal_animation_delay_millis: i32,
}

impl EntityCreature {
    pub fn new(entity_type: EntityType) -> Self {
        Self {
            entity: GenericEntity::new(entity_type),
            ai_groups: Vec::new(),
            navigator: Navigator::default(),
            target: None,
            pending_path_request: None,
            last_path_error: None,
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

    pub fn entity_meta_mut(&mut self) -> EntityMeta<'_> {
        self.entity.entity_meta_mut()
    }

    pub const fn navigator(&self) -> &Navigator {
        &self.navigator
    }

    pub fn navigator_mut(&mut self) -> &mut Navigator {
        &mut self.navigator
    }

    pub const fn last_path_error(&self) -> Option<SetPathToError> {
        self.last_path_error
    }

    pub fn take_last_path_error(&mut self) -> Option<SetPathToError> {
        self.last_path_error.take()
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

    pub fn attack(&mut self, target: &Entity) {
        self.queue_attack(target.entity_id(), false);
    }

    pub fn attack_with_swing(&mut self, target: &Entity) {
        self.queue_attack(target.entity_id(), true);
    }

    pub(crate) fn attack_entity_with_swing(&mut self, target: EntityId) {
        self.queue_attack(target, true);
    }

    fn queue_attack(&mut self, target: EntityId, should_swing_main_hand: bool) {
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

    pub fn set_path_to(&mut self, request: PathRequest) -> Result<bool, SetPathToError> {
        self.last_path_error = None;
        if request.destination().is_none() {
            self.pending_path_request = None;
            self.navigator.reset();
            return Ok(false);
        }
        if self.entity.world().is_none() {
            return Err(SetPathToError::EntityHasNoWorld);
        }
        self.pending_path_request = Some(request);
        Ok(true)
    }

    pub(crate) fn set_path_to_in_world(
        &mut self,
        world: &WorldSnapshot,
        request: PathRequest,
    ) -> Result<bool, SetPathToError> {
        self.last_path_error = None;
        if request.destination().is_none() {
            self.navigator.reset();
            return Ok(false);
        }
        if self.entity.world().is_none() {
            return Err(SetPathToError::EntityHasNoWorld);
        }
        self.resolve_path_request(world, request)
    }

    fn resolve_path_request(
        &mut self,
        world: &WorldSnapshot,
        request: PathRequest,
    ) -> Result<bool, SetPathToError> {
        let start = self.entity.position();
        let bounding_box = self.entity.bounding_box();
        let is_on_ground = self.entity.is_on_ground();
        self.navigator
            .set_path_to(world, start, bounding_box, is_on_ground, request)
    }

    pub fn set_instance(self, world: &mut World) -> bool {
        world.add_entity(crate::entity::Entity::Creature(self))
    }

    pub fn set_instance_at(mut self, world: &mut World, position: EntityPosition) -> bool {
        self.set_position(position);
        self.set_instance(world)
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
        self.resolve_pending_path_request(world);
        let entity_is_dead = self.entity.is_dead();
        self.navigator.tick(&mut self.entity, world, entity_is_dead);
    }

    pub(crate) fn resolve_pending_path_request(&mut self, world: &WorldSnapshot) {
        let Some(request) = self.pending_path_request.take() else {
            return;
        };
        if let Err(path_error) = self.resolve_path_request(world, request) {
            self.last_path_error = Some(path_error);
        }
    }
}

impl Deref for EntityCreature {
    type Target = GenericEntity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}

impl DerefMut for EntityCreature {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}

impl AsRef<GenericEntity> for EntityCreature {
    fn as_ref(&self) -> &GenericEntity {
        &self.entity
    }
}

impl AsMut<GenericEntity> for EntityCreature {
    fn as_mut(&mut self) -> &mut GenericEntity {
        &mut self.entity
    }
}
