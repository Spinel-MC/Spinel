use crate::entity::metadata::MetadataHolder;
use crate::entity::{EntityId, EntityView, EquipmentSlot};
use crate::network::client::instance::Client;
use spinel_core::network::clientbound::play::entity_head_look::EntityHeadLookPacket;
use spinel_core::network::clientbound::play::entity_position::EntityPositionPacket;
use spinel_core::network::clientbound::play::entity_position_and_rotation::EntityPositionAndRotationPacket;
use spinel_core::network::clientbound::play::entity_rotation::EntityRotationPacket;
use spinel_core::network::clientbound::play::entity_teleport::EntityTeleportPacket;
use spinel_core::network::clientbound::play::entity_velocity::EntityVelocityPacket;
use spinel_core::network::clientbound::play::player_info_update::PlayerInfoUpdatePacket;
use spinel_core::network::clientbound::play::remove_entities::RemoveEntitiesPacket;
use spinel_core::network::clientbound::play::set_entity_data::SetEntityDataPacket;
use spinel_core::network::clientbound::play::set_equipment::{
    EntityEquipmentEntry, SetEquipmentPacket,
};
use spinel_core::network::clientbound::play::spawn_entity::{EntityAngle, SpawnEntityPacket};
use spinel_network::types::{Slot, Vector3d, Velocity};
use spinel_registry::{EntityBoundingBox, EntityType, ItemStack};
use std::io;
use uuid::Uuid;

const MAX_ENTITY_COORDINATE: f64 = 2_000_000_000.0;

pub struct GenericEntity {
    entity_id: EntityId,
    uuid: Uuid,
    entity_type: EntityType,
    bounding_box: EntityBoundingBox,
    metadata: MetadataHolder,
    view: EntityView,
    position: EntityPosition,
    velocity: Velocity,
    equipment: EntityEquipment,
    world: Option<Uuid>,
    removed: bool,
    ticks: u64,
}

#[derive(Clone, Debug, PartialEq)]
struct EntityEquipment {
    main_hand: ItemStack,
    off_hand: ItemStack,
    boots: ItemStack,
    leggings: ItemStack,
    chestplate: ItemStack,
    helmet: ItemStack,
    body: ItemStack,
    saddle: ItemStack,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EntityPosition {
    x: f64,
    y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
    head_yaw: f32,
}

impl GenericEntity {
    pub fn new(entity_type: EntityType) -> Self {
        Self::with_uuid(entity_type, Uuid::new_v4())
    }

    pub fn with_uuid(entity_type: EntityType, uuid: Uuid) -> Self {
        let entity_id = EntityId::next();
        Self {
            entity_id,
            uuid,
            entity_type,
            bounding_box: entity_type.bounding_box(),
            metadata: MetadataHolder::default(),
            view: EntityView::new(entity_id),
            position: EntityPosition::default(),
            velocity: Velocity(Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            equipment: EntityEquipment::default(),
            world: None,
            removed: false,
            ticks: 0,
        }
    }

    pub const fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub const fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub const fn entity_type(&self) -> EntityType {
        self.entity_type
    }

    pub const fn world(&self) -> Option<Uuid> {
        self.world
    }

    pub(crate) fn set_world(&mut self, world: Uuid) {
        self.world = Some(world);
    }

    pub const fn bounding_box(&self) -> EntityBoundingBox {
        self.bounding_box
    }

    pub const fn metadata(&self) -> &MetadataHolder {
        &self.metadata
    }

    pub const fn metadata_mut(&mut self) -> &mut MetadataHolder {
        &mut self.metadata
    }

    pub const fn view(&self) -> &EntityView {
        &self.view
    }

    pub const fn view_mut(&mut self) -> &mut EntityView {
        &mut self.view
    }

    pub fn viewers(&self) -> std::collections::BTreeSet<EntityId> {
        self.view.viewers()
    }

    pub fn add_viewer(&mut self, viewer_id: EntityId) -> bool {
        self.view.manual_add(viewer_id)
    }

    pub fn remove_viewer(&mut self, viewer_id: EntityId) -> bool {
        self.view.manual_remove(viewer_id)
    }

    pub fn is_viewer(&self, viewer_id: EntityId) -> bool {
        self.view.is_viewer(viewer_id)
    }

    pub const fn position(&self) -> EntityPosition {
        self.position
    }

    pub fn set_position(&mut self, position: EntityPosition) {
        self.position = position.clamped_to_minestom_entity_bounds();
    }

    pub const fn velocity(&self) -> Velocity {
        self.velocity
    }

    pub fn set_velocity(&mut self, velocity: Velocity) {
        self.velocity = velocity;
    }

    pub fn has_velocity(&self) -> bool {
        self.velocity.0.x != 0.0 || self.velocity.0.y != 0.0 || self.velocity.0.z != 0.0
    }

    pub fn equipment(&self, equipment_slot: EquipmentSlot) -> &ItemStack {
        self.equipment.item_stack(equipment_slot)
    }

    pub fn set_equipment(&mut self, equipment_slot: EquipmentSlot, item_stack: ItemStack) {
        self.equipment.set_item_stack(equipment_slot, item_stack);
    }

    pub const fn is_removed(&self) -> bool {
        self.removed
    }

    pub const fn ticks(&self) -> u64 {
        self.ticks
    }

    pub fn switch_entity_type(&mut self, entity_type: EntityType) {
        self.entity_type = entity_type;
        self.metadata = MetadataHolder::default();
    }

    pub fn remove(&mut self) {
        self.removed = true;
    }

    pub fn spawn_packet(&self) -> SpawnEntityPacket {
        SpawnEntityPacket {
            entity_id: self.entity_id.value(),
            uuid: self.uuid,
            entity_type: self.entity_type.id(),
            x: self.position.x,
            y: self.position.y,
            z: self.position.z,
            velocity: self.velocity,
            pitch: EntityAngle(self.position.pitch),
            yaw: EntityAngle(self.position.yaw),
            head_yaw: EntityAngle(self.position.head_yaw),
            data: 0,
        }
    }

    pub fn metadata_packet(&self) -> SetEntityDataPacket {
        SetEntityDataPacket::new(self.entity_id.value(), self.metadata.entries())
    }

    pub fn equipment_packet(&self) -> SetEquipmentPacket {
        SetEquipmentPacket::new(self.entity_id.value(), self.equipment.entries())
    }

    pub fn velocity_packet(&self) -> EntityVelocityPacket {
        EntityVelocityPacket {
            entity_id: self.entity_id.value(),
            velocity: self.velocity,
        }
    }

    pub fn position_delta_packet(
        &self,
        previous_position: EntityPosition,
        on_ground: bool,
    ) -> EntityPositionPacket {
        EntityPositionPacket {
            entity_id: self.entity_id.value(),
            delta_x: EntityPositionPacket::delta(self.position.x, previous_position.x),
            delta_y: EntityPositionPacket::delta(self.position.y, previous_position.y),
            delta_z: EntityPositionPacket::delta(self.position.z, previous_position.z),
            on_ground,
        }
    }

    pub fn position_and_rotation_delta_packet(
        &self,
        previous_position: EntityPosition,
        on_ground: bool,
    ) -> EntityPositionAndRotationPacket {
        EntityPositionAndRotationPacket {
            entity_id: self.entity_id.value(),
            delta_x: EntityPositionPacket::delta(self.position.x, previous_position.x),
            delta_y: EntityPositionPacket::delta(self.position.y, previous_position.y),
            delta_z: EntityPositionPacket::delta(self.position.z, previous_position.z),
            yaw: EntityAngle(self.position.yaw),
            pitch: EntityAngle(self.position.pitch),
            on_ground,
        }
    }

    pub fn rotation_packet(&self, on_ground: bool) -> EntityRotationPacket {
        EntityRotationPacket {
            entity_id: self.entity_id.value(),
            yaw: EntityAngle(self.position.yaw),
            pitch: EntityAngle(self.position.pitch),
            on_ground,
        }
    }

    pub fn head_look_packet(&self) -> EntityHeadLookPacket {
        EntityHeadLookPacket {
            entity_id: self.entity_id.value(),
            head_yaw: EntityAngle(self.position.head_yaw),
        }
    }

    pub fn teleport_packet(&self) -> EntityTeleportPacket {
        EntityTeleportPacket {
            entity_id: self.entity_id.value(),
            position: self.position.as_vector(),
            delta: Vector3d {
                x: self.velocity.0.x,
                y: self.velocity.0.y,
                z: self.velocity.0.z,
            },
            yaw: self.position.yaw,
            pitch: self.position.pitch,
            flags: 0,
            on_ground: false,
        }
    }

    pub fn dirty_metadata_packet(&mut self) -> Option<SetEntityDataPacket> {
        let dirty_entries = self.metadata.drain_dirty_entries();
        if dirty_entries.is_empty() {
            return None;
        }
        Some(SetEntityDataPacket::new(
            self.entity_id.value(),
            dirty_entries,
        ))
    }

    pub fn remove_packet(&self) -> RemoveEntitiesPacket {
        RemoveEntitiesPacket::new(vec![self.entity_id.value()])
    }

    pub(crate) fn show_to_viewer(
        &mut self,
        viewer_id: EntityId,
        client: &mut Client,
    ) -> io::Result<bool> {
        if !self.add_viewer(viewer_id) {
            return Ok(false);
        }
        if self.entity_type == EntityType::PLAYER {
            PlayerInfoUpdatePacket::add_listed_player(
                self.uuid,
                format!("test_player_{}", self.entity_id.value()),
            )
            .dispatch(client)?;
        }
        self.spawn_packet().dispatch(client)?;
        if self.has_velocity() {
            self.velocity_packet().dispatch(client)?;
        }
        self.metadata_packet().dispatch(client)?;
        self.equipment_packet().dispatch(client)?;
        self.head_look_packet().dispatch(client)?;
        Ok(true)
    }

    pub(crate) fn tick(&mut self) {
        if self.removed {
            return;
        }
        self.ticks += 1;
    }
}

impl EntityEquipment {
    fn item_stack(&self, equipment_slot: EquipmentSlot) -> &ItemStack {
        match equipment_slot {
            EquipmentSlot::MainHand => &self.main_hand,
            EquipmentSlot::OffHand => &self.off_hand,
            EquipmentSlot::Boots => &self.boots,
            EquipmentSlot::Leggings => &self.leggings,
            EquipmentSlot::Chestplate => &self.chestplate,
            EquipmentSlot::Helmet => &self.helmet,
            EquipmentSlot::Body => &self.body,
            EquipmentSlot::Saddle => &self.saddle,
        }
    }

    fn set_item_stack(&mut self, equipment_slot: EquipmentSlot, item_stack: ItemStack) {
        match equipment_slot {
            EquipmentSlot::MainHand => self.main_hand = item_stack,
            EquipmentSlot::OffHand => self.off_hand = item_stack,
            EquipmentSlot::Boots => self.boots = item_stack,
            EquipmentSlot::Leggings => self.leggings = item_stack,
            EquipmentSlot::Chestplate => self.chestplate = item_stack,
            EquipmentSlot::Helmet => self.helmet = item_stack,
            EquipmentSlot::Body => self.body = item_stack,
            EquipmentSlot::Saddle => self.saddle = item_stack,
        }
    }

    fn entries(&self) -> Vec<EntityEquipmentEntry> {
        [
            EquipmentSlot::MainHand,
            EquipmentSlot::OffHand,
            EquipmentSlot::Boots,
            EquipmentSlot::Leggings,
            EquipmentSlot::Chestplate,
            EquipmentSlot::Helmet,
            EquipmentSlot::Body,
        ]
        .into_iter()
        .map(|equipment_slot| EntityEquipmentEntry {
            slot: equipment_slot.entity_equipment_slot(),
            item: Slot::from_item_stack(self.item_stack(equipment_slot)),
        })
        .collect()
    }
}

impl Default for EntityEquipment {
    fn default() -> Self {
        Self {
            main_hand: ItemStack::air(),
            off_hand: ItemStack::air(),
            boots: ItemStack::air(),
            leggings: ItemStack::air(),
            chestplate: ItemStack::air(),
            helmet: ItemStack::air(),
            body: ItemStack::air(),
            saddle: ItemStack::air(),
        }
    }
}

impl EntityPosition {
    pub const fn new(x: f64, y: f64, z: f64, yaw: f32, pitch: f32) -> Self {
        Self {
            x,
            y,
            z,
            yaw,
            pitch,
            head_yaw: yaw,
        }
    }

    pub const fn x(self) -> f64 {
        self.x
    }

    pub const fn y(self) -> f64 {
        self.y
    }

    pub const fn z(self) -> f64 {
        self.z
    }

    pub const fn yaw(self) -> f32 {
        self.yaw
    }

    pub const fn pitch(self) -> f32 {
        self.pitch
    }

    pub const fn head_yaw(self) -> f32 {
        self.head_yaw
    }

    pub const fn with_head_yaw(self, head_yaw: f32) -> Self {
        Self { head_yaw, ..self }
    }

    pub const fn as_vector(self) -> Vector3d {
        Vector3d {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub const fn offset(self, x: f64, y: f64, z: f64) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
            yaw: self.yaw,
            pitch: self.pitch,
            head_yaw: self.head_yaw,
        }
    }

    pub fn clamped_to_minestom_entity_bounds(self) -> Self {
        Self {
            x: clamp_entity_coordinate(self.x),
            y: clamp_entity_coordinate(self.y),
            z: clamp_entity_coordinate(self.z),
            yaw: self.yaw,
            pitch: self.pitch,
            head_yaw: self.head_yaw,
        }
    }
}

fn clamp_entity_coordinate(coordinate: f64) -> f64 {
    coordinate.clamp(-MAX_ENTITY_COORDINATE, MAX_ENTITY_COORDINATE)
}

#[cfg(test)]
mod tests {
    use super::{EntityPosition, GenericEntity};
    use crate::entity::metadata::definitions;
    use crate::entity::{EntityId, EquipmentSlot};
    use spinel_network::types::entity_metadata::MetadataValue;
    use spinel_registry::{EntityType, ItemStack, Material};

    #[test]
    fn generic_entity_owns_minestom_entity_identity_and_type() {
        let entity = GenericEntity::new(EntityType::ZOMBIE);

        assert!(entity.entity_id().value() > 0);
        assert_eq!(entity.entity_type(), EntityType::ZOMBIE);
        assert_eq!(entity.bounding_box(), EntityType::ZOMBIE.bounding_box());
    }

    #[test]
    fn generic_entity_switch_type_preserves_bounding_box_like_minestom() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);
        let original_bounding_box = entity.bounding_box();

        entity.switch_entity_type(EntityType::ARROW);

        assert_eq!(entity.entity_type(), EntityType::ARROW);
        assert_eq!(entity.bounding_box(), original_bounding_box);
    }

    #[test]
    fn generic_entity_viewer_add_and_remove_match_minestom_no_op_return_values() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);
        let viewer_id = EntityId::next();

        assert!(entity.add_viewer(viewer_id));
        assert!(!entity.add_viewer(viewer_id));
        assert!(entity.is_viewer(viewer_id));
        assert!(entity.remove_viewer(viewer_id));
        assert!(!entity.remove_viewer(viewer_id));
        assert!(!entity.is_viewer(viewer_id));
    }

    #[test]
    fn removed_generic_entity_does_not_tick() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);

        entity.remove();
        entity.tick();

        assert_eq!(entity.ticks(), 0);
    }

    #[test]
    fn generic_entity_builds_spawn_and_metadata_packets_from_owned_state() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);
        entity.set_position(EntityPosition::new(1.0, 2.0, 3.0, 45.0, 90.0));
        entity
            .metadata_mut()
            .set(&definitions::air_ticks(), MetadataValue::VarInt(10));

        let spawn_packet = entity.spawn_packet();
        let metadata_packet = entity.dirty_metadata_packet().unwrap();

        assert_eq!(spawn_packet.entity_id, entity.entity_id().value());
        assert_eq!(spawn_packet.entity_type, EntityType::ZOMBIE.id());
        assert_eq!(spawn_packet.x, 1.0);
        assert_eq!(metadata_packet.entity_id, entity.entity_id().value());
        assert_eq!(metadata_packet.entries.0.len(), 1);
        assert!(entity.dirty_metadata_packet().is_none());
    }

    #[test]
    fn generic_entity_builds_minestom_equipment_packet_from_owned_equipment() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);
        entity.set_equipment(
            EquipmentSlot::MainHand,
            ItemStack::of(Material::DIAMOND_SWORD),
        );
        entity.set_equipment(
            EquipmentSlot::Helmet,
            ItemStack::of(Material::DIAMOND_HELMET),
        );

        let equipment_packet = entity.equipment_packet();

        assert_eq!(equipment_packet.entity_id, entity.entity_id().value());
        assert_eq!(equipment_packet.equipment.0.len(), 7);
        assert_eq!(
            equipment_packet.equipment.0[0]
                .item
                .to_item_stack()
                .material(),
            &Material::DIAMOND_SWORD
        );
        assert_eq!(
            equipment_packet.equipment.0[5]
                .item
                .to_item_stack()
                .material(),
            &Material::DIAMOND_HELMET
        );
    }

    #[test]
    fn generic_entity_builds_velocity_head_look_and_teleport_packets_from_owned_state() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);
        entity.set_position(EntityPosition::new(1.0, 2.0, 3.0, 45.0, 90.0).with_head_yaw(30.0));
        entity.set_velocity(spinel_network::types::Velocity(
            spinel_network::types::Vector3d {
                x: 0.25,
                y: 0.0,
                z: -0.25,
            },
        ));

        let velocity_packet = entity.velocity_packet();
        let head_look_packet = entity.head_look_packet();
        let teleport_packet = entity.teleport_packet();

        assert_eq!(velocity_packet.entity_id, entity.entity_id().value());
        assert_eq!(head_look_packet.entity_id, entity.entity_id().value());
        assert_eq!(head_look_packet.head_yaw.0, 30.0);
        assert_eq!(teleport_packet.entity_id, entity.entity_id().value());
        assert_eq!(teleport_packet.position, entity.position().as_vector());
    }

    #[test]
    fn generic_entity_builds_minestom_relative_movement_packets_from_previous_position() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);
        let previous_position = EntityPosition::new(1.0, 2.0, 3.0, 0.0, 0.0);
        entity.set_position(EntityPosition::new(1.5, 2.0, 2.5, 90.0, 45.0));

        let position_packet = entity.position_delta_packet(previous_position, true);
        let position_and_rotation_packet =
            entity.position_and_rotation_delta_packet(previous_position, false);
        let rotation_packet = entity.rotation_packet(true);

        assert_eq!(position_packet.delta_x, 2048);
        assert_eq!(position_packet.delta_z, -2048);
        assert!(position_packet.on_ground);
        assert_eq!(position_and_rotation_packet.delta_x, 2048);
        assert_eq!(position_and_rotation_packet.yaw.0, 90.0);
        assert!(!position_and_rotation_packet.on_ground);
        assert_eq!(rotation_packet.pitch.0, 45.0);
        assert!(rotation_packet.on_ground);
    }

    #[test]
    fn generic_entity_position_clamps_to_minestom_coordinate_limit() {
        let mut entity = GenericEntity::new(EntityType::ZOMBIE);

        entity.set_position(EntityPosition::new(
            3_000_000_000.0,
            -3_000_000_000.0,
            12.0,
            0.0,
            0.0,
        ));

        assert_eq!(entity.position().x(), 2_000_000_000.0);
        assert_eq!(entity.position().y(), -2_000_000_000.0);
        assert_eq!(entity.position().z(), 12.0);
    }
}
