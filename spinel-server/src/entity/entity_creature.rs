use crate::entity::ai::{CreatureAiAction, EntityAiGroup};
use crate::entity::metadata::EntityMetaCast;
use crate::entity::pathfinding::{
    Navigator, NodeFollowerPhysicsTiming, PathRequest, SetPathToError,
};
use crate::entity::{Entity, EntityId, EntityPosition, GenericEntity};
use crate::events::entity_attack::EntityAttackEvent;
use crate::server::MinecraftServer;
use crate::world::{World, WorldSnapshot};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EntityCreatureAttackError {
    #[error("entity creature is not assigned to a world")]
    EntityHasNoWorld,

    #[error("entity creature world does not have an event dispatcher")]
    EventDispatcherUnavailable,

    #[error("entity creature world is no longer available")]
    WorldUnavailable,

    #[error("entity creature main-hand animation could not be sent: {message}")]
    MainHandAnimationDispatchFailed { message: String },
}

pub struct EntityCreature {
    entity: GenericEntity,
    ai_groups: Vec<EntityAiGroup>,
    navigator: Navigator,
    target: Option<EntityId>,
    pending_ai_actions: Vec<CreatureAiAction>,
    event_dispatcher: Option<usize>,
    pathfinding_world: Option<Arc<WorldSnapshot>>,
    removal_animation_delay_millis: i32,
}

impl EntityCreature {
    pub fn new(entity_type: EntityType) -> Self {
        let mut entity = GenericEntity::new(entity_type);
        entity.heal();
        Self {
            entity,
            ai_groups: Vec::new(),
            navigator: Navigator::default(),
            target: None,
            pending_ai_actions: Vec::new(),
            event_dispatcher: None,
            pathfinding_world: None,
            removal_animation_delay_millis: 1000,
        }
    }

    pub const fn get_entity(&self) -> &GenericEntity {
        &self.entity
    }

    pub fn get_entity_mut(&mut self) -> &mut GenericEntity {
        &mut self.entity
    }

    pub fn get_entity_meta_mut(&mut self) -> EntityMetaCast<'_> {
        self.entity.get_entity_meta_mut()
    }

    pub const fn get_navigator(&self) -> &Navigator {
        &self.navigator
    }

    pub fn get_navigator_mut(&mut self) -> &mut Navigator {
        &mut self.navigator
    }

    pub fn get_ai_groups(&self) -> &[EntityAiGroup] {
        &self.ai_groups
    }

    pub fn get_ai_groups_mut(&mut self) -> &mut Vec<EntityAiGroup> {
        &mut self.ai_groups
    }

    pub fn add_ai_group(&mut self, group: EntityAiGroup) {
        self.ai_groups.push(group);
    }

    pub const fn get_target(&self) -> Option<EntityId> {
        self.target
    }

    pub fn set_target(&mut self, target: Option<EntityId>) {
        self.target = target;
    }

    pub const fn get_removal_animation_delay_millis(&self) -> i32 {
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

    pub fn attack(&mut self, target: &Entity) -> Result<(), EntityCreatureAttackError> {
        self.dispatch_attack(target)
    }

    pub fn attack_with_swing(&mut self, target: &Entity) -> Result<(), EntityCreatureAttackError> {
        self.dispatch_main_hand_animation()?;
        self.dispatch_attack(target)
    }

    fn dispatch_attack(&self, target: &Entity) -> Result<(), EntityCreatureAttackError> {
        if self.entity.get_world().is_none() {
            return Err(EntityCreatureAttackError::EntityHasNoWorld);
        }
        let Some(event_dispatcher) = self.event_dispatcher else {
            return Err(EntityCreatureAttackError::EventDispatcherUnavailable);
        };
        let server = unsafe { &mut *(event_dispatcher as *mut MinecraftServer) };
        EntityAttackEvent::new(self.get_entity_id(), target.get_entity_id()).dispatch(server);
        Ok(())
    }

    fn dispatch_main_hand_animation(&self) -> Result<(), EntityCreatureAttackError> {
        let Some(world) = self.entity.get_world() else {
            return Err(EntityCreatureAttackError::EntityHasNoWorld);
        };
        let Some(event_dispatcher) = self.event_dispatcher else {
            return Err(EntityCreatureAttackError::EventDispatcherUnavailable);
        };
        let server = unsafe { &mut *(event_dispatcher as *mut MinecraftServer) };
        let Some(world) = server.world_manager.world_mut(world) else {
            return Err(EntityCreatureAttackError::WorldUnavailable);
        };
        world
            .swing_generic_entity_main_hand(self.get_entity_id())
            .map_err(
                |error| EntityCreatureAttackError::MainHandAnimationDispatchFailed {
                    message: error.to_string(),
                },
            )?;
        Ok(())
    }

    pub(crate) fn attack_entity_with_swing(&mut self, target: EntityId) {
        self.queue_attack(target, true);
    }

    fn queue_attack(&mut self, target: EntityId, should_swing_main_hand: bool) {
        self.pending_ai_actions.push(CreatureAiAction::Attack {
            source: self.get_entity_id(),
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
            shooter: self.get_entity_id(),
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
        if request.get_destination().is_none() {
            self.navigator.reset();
            return Ok(false);
        }
        if self.entity.get_world().is_none() {
            return Err(SetPathToError::EntityHasNoWorld);
        }
        let Some(world) = self.pathfinding_world.clone() else {
            return Err(SetPathToError::WorldSnapshotUnavailable);
        };
        self.resolve_path_request(&world, request)
    }
    pub(crate) fn set_path_to_in_world(
        &mut self,
        world: &WorldSnapshot,
        request: PathRequest,
    ) -> Result<bool, SetPathToError> {
        if request.get_destination().is_none() {
            self.navigator.reset();
            return Ok(false);
        }
        if self.entity.get_world().is_none() {
            return Err(SetPathToError::EntityHasNoWorld);
        }
        self.resolve_path_request(world, request)
    }

    fn resolve_path_request(
        &mut self,
        world: &WorldSnapshot,
        request: PathRequest,
    ) -> Result<bool, SetPathToError> {
        let start = self.entity.get_position();
        let bounding_box = self.entity.get_bounding_box();
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

    pub(crate) fn set_event_dispatcher(&mut self, event_dispatcher: Option<usize>) {
        self.event_dispatcher = event_dispatcher;
    }

    pub(crate) fn set_pathfinding_world(&mut self, world: Arc<WorldSnapshot>) {
        self.pathfinding_world = Some(world);
    }

    pub(crate) fn tick_before_movement(&mut self, world: &WorldSnapshot, time: u64) {
        if self.navigator.get_physics_timing() != NodeFollowerPhysicsTiming::BeforePhysics {
            return;
        }
        self.tick_navigation(world, time);
    }

    pub(crate) fn tick_after_movement(&mut self, world: &WorldSnapshot, time: u64) {
        if self.navigator.get_physics_timing() == NodeFollowerPhysicsTiming::AfterPhysics {
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
