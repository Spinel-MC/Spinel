use crate::entity::metadata::definitions;
use crate::entity::{EntityId, EntityPosition, GenericEntity};
use spinel_core::network::clientbound::play::spawn_entity::SpawnEntityPacket;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_network::types::{Vector3d, Velocity};
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct ExperienceOrb {
    entity: GenericEntity,
    experience_count: i16,
    target: Option<EntityId>,
    last_target_update_tick: i64,
}

impl ExperienceOrb {
    pub fn new(experience_count: i16) -> Self {
        let mut entity = GenericEntity::new(EntityType::EXPERIENCE_ORB);
        entity.set_bounding_box_dimensions(0.5, 0.5, 0.5);
        entity.metadata_mut().set(
            &definitions::experience_orb::value(),
            MetadataValue::VarInt(i32::from(experience_count)),
        );
        Self {
            entity,
            experience_count,
            target: None,
            last_target_update_tick: 0,
        }
    }

    pub const fn experience_count(&self) -> i16 {
        self.experience_count
    }

    pub fn set_experience_count(&mut self, experience_count: i16) {
        self.experience_count = experience_count;
        self.entity.metadata_mut().set(
            &definitions::experience_orb::value(),
            MetadataValue::VarInt(i32::from(experience_count)),
        );
    }

    pub(crate) const fn target(&self) -> Option<EntityId> {
        self.target
    }

    pub(crate) fn set_target(&mut self, target: Option<EntityId>) {
        self.target = target;
    }

    pub(crate) const fn last_target_update_tick(&self) -> i64 {
        self.last_target_update_tick
    }

    pub(crate) fn set_last_target_update_tick(&mut self, tick: i64) {
        self.last_target_update_tick = tick;
    }

    pub(crate) fn apply_attraction(&mut self, target_position: EntityPosition, eye_height: f64) {
        let position = self.position();
        let delta = Vector3d {
            x: target_position.x() - position.x(),
            y: target_position.y() + eye_height / 2.0 - position.y(),
            z: target_position.z() - position.z(),
        };
        let distance = delta
            .x
            .mul_add(delta.x, delta.y.mul_add(delta.y, delta.z * delta.z))
            .sqrt();
        if distance >= 8.0 || distance == 0.0 {
            return;
        }
        let acceleration = (1.0 - distance / 8.0).powi(2) * 0.1;
        let velocity = self.velocity().0;
        self.set_velocity(Velocity(Vector3d {
            x: velocity.x + delta.x / distance * acceleration,
            y: velocity.y + delta.y / distance * acceleration,
            z: velocity.z + delta.z / distance * acceleration,
        }));
    }

    pub(crate) fn apply_gravity(&mut self) {
        let velocity = self.velocity().0;
        if !self.has_no_gravity() {
            return;
        }
        self.set_velocity(Velocity(Vector3d {
            x: velocity.x,
            y: velocity.y - 0.3,
            z: velocity.z,
        }));
    }

    pub(crate) fn apply_drag(&mut self) {
        let velocity = self.velocity().0;
        let horizontal_drag = if self.is_on_ground() {
            0.6 * 0.98
        } else {
            0.98
        };
        let vertical_velocity = if self.is_on_ground() {
            velocity.y * 0.98 * -0.9
        } else {
            velocity.y * 0.98
        };
        self.set_velocity(Velocity(Vector3d {
            x: velocity.x * horizontal_drag,
            y: vertical_velocity,
            z: velocity.z * horizontal_drag,
        }));
    }

    pub(crate) fn spawn_packet(&self) -> SpawnEntityPacket {
        let mut packet = self.entity.spawn_packet();
        packet.data = i32::from(self.experience_count);
        packet
    }
}

impl Deref for ExperienceOrb {
    type Target = GenericEntity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}

impl DerefMut for ExperienceOrb {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
