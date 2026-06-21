use crate::entity::metadata::definitions;
use crate::entity::{EntityId, EntityPosition, GenericEntity, ProjectileEntityMeta};
use rand::Rng;
use spinel_core::network::clientbound::play::spawn_entity::SpawnEntityPacket;
use spinel_network::types::{Slot, entity_metadata::MetadataValue};
use spinel_network::types::{Vector3d, Velocity};
use spinel_registry::EntityType;
use spinel_registry::ItemStack;
use std::f64::consts::TAU;
use std::ops::{Deref, DerefMut};

const PROJECTILE_GRAVITY_AIM_COMPENSATION: f64 = 0.20000000298023224;
const PROJECTILE_SPREAD_SCALE: f64 = 0.007499999832361937;
const SERVER_TICKS_PER_SECOND: f64 = 20.0;

pub struct ProjectileEntity {
    entity: GenericEntity,
    shooter: Option<EntityId>,
    was_stuck: bool,
}

impl ProjectileEntity {
    pub fn new(shooter: Option<EntityId>, entity_type: EntityType) -> Self {
        let mut entity = GenericEntity::new(entity_type);
        entity.set_has_physics(false);
        let mut projectile = Self {
            entity,
            shooter: None,
            was_stuck: false,
        };
        projectile.set_shooter(shooter);
        projectile
    }
    pub fn get_entity_meta_mut(&mut self) -> ProjectileEntityMeta<'_> {
        ProjectileEntityMeta::new(self)
    }


    pub const fn get_shooter(&self) -> Option<EntityId> {
        self.shooter
    }

    pub fn set_shooter(&mut self, shooter: Option<EntityId>) {
        self.shooter = shooter;
        if self.get_entity_type() == EntityType::FIREWORK_ROCKET {
            self.set_firework_shooter_entity_id(shooter.map(EntityId::get_value));
        }
    }

    pub const fn get_was_stuck(&self) -> bool {
        self.was_stuck
    }

    pub(crate) fn set_was_stuck(&mut self, was_stuck: bool) {
        self.was_stuck = was_stuck;
    }

    pub fn spawn_packet(&self) -> SpawnEntityPacket {
        let mut packet = self.entity.spawn_packet();
        if matches!(
            self.get_entity_type(),
            EntityType::ARROW
                | EntityType::BREEZE_WIND_CHARGE
                | EntityType::DRAGON_FIREBALL
                | EntityType::FIREBALL
                | EntityType::SMALL_FIREBALL
                | EntityType::SPECTRAL_ARROW
                | EntityType::WIND_CHARGE
                | EntityType::WITHER_SKULL
        ) {
            packet.data = self.shooter.map_or(0, EntityId::get_value);
            packet.velocity = self.entity.get_protocol_velocity();
        }
        packet
    }

    pub fn get_projectile_item(&self) -> ItemStack {
        metadata_item_stack(self, &definitions::projectile_item())
    }

    pub fn set_projectile_item(&mut self, item_stack: ItemStack) {
        self.get_metadata_mut().set(
            &definitions::projectile_item(),
            MetadataValue::Slot(Slot::from_item_stack(&item_stack)),
        );
    }

    pub fn is_critical(&self) -> bool {
        self.get_metadata().flag(&definitions::is_critical_arrow())
    }

    pub fn set_critical(&mut self, is_critical: bool) {
        self.get_metadata_mut()
            .set_flag(&definitions::is_critical_arrow(), is_critical);
    }

    pub fn has_no_clip(&self) -> bool {
        self.get_metadata().flag(&definitions::has_no_clip_arrow())
    }

    pub fn set_no_clip(&mut self, has_no_clip: bool) {
        self.get_metadata_mut()
            .set_flag(&definitions::has_no_clip_arrow(), has_no_clip);
    }

    pub fn get_piercing_level(&self) -> i8 {
        metadata_byte(self, &definitions::get_piercing_level())
    }

    pub fn set_piercing_level(&mut self, piercing_level: i8) {
        self.get_metadata_mut().set(
            &definitions::get_piercing_level(),
            MetadataValue::Byte(piercing_level),
        );
    }

    pub fn is_in_ground(&self) -> bool {
        metadata_boolean(self, &definitions::is_arrow_in_ground())
    }

    pub fn set_in_ground(&mut self, is_in_ground: bool) {
        self.get_metadata_mut().set(
            &definitions::is_arrow_in_ground(),
            MetadataValue::Boolean(is_in_ground),
        );
    }

    pub fn get_color(&self) -> i32 {
        metadata_var_int(self, &definitions::arrow_color())
    }

    pub fn set_color(&mut self, color: i32) {
        self.get_metadata_mut()
            .set(&definitions::arrow_color(), MetadataValue::VarInt(color));
    }

    pub fn get_loyalty_level(&self) -> i8 {
        metadata_byte(self, &definitions::trident_loyalty_level())
    }

    pub fn set_loyalty_level(&mut self, loyalty_level: i8) {
        self.get_metadata_mut().set(
            &definitions::trident_loyalty_level(),
            MetadataValue::Byte(loyalty_level),
        );
    }

    pub fn has_enchantment_glint(&self) -> bool {
        metadata_boolean(self, &definitions::trident_has_enchantment_glint())
    }

    pub fn set_enchantment_glint(&mut self, has_enchantment_glint: bool) {
        self.get_metadata_mut().set(
            &definitions::trident_has_enchantment_glint(),
            MetadataValue::Boolean(has_enchantment_glint),
        );
    }

    pub fn is_invulnerable_wither_skull(&self) -> bool {
        metadata_boolean(self, &definitions::wither_skull_is_invulnerable())
    }

    pub fn set_invulnerable_wither_skull(&mut self, is_invulnerable: bool) {
        self.get_metadata_mut().set(
            &definitions::wither_skull_is_invulnerable(),
            MetadataValue::Boolean(is_invulnerable),
        );
    }

    pub fn get_firework_info(&self) -> ItemStack {
        metadata_item_stack(self, &definitions::get_firework_info())
    }

    pub fn set_firework_info(&mut self, item_stack: ItemStack) {
        self.get_metadata_mut().set(
            &definitions::get_firework_info(),
            MetadataValue::Slot(Slot::from_item_stack(&item_stack)),
        );
    }

    pub fn get_firework_shooter_entity_id(&self) -> Option<i32> {
        match self
            .get_metadata()
            .get_value(&definitions::firework_shooter_entity_id())
        {
            MetadataValue::OptionalVarInt(shooter_entity_id) => shooter_entity_id,
            _ => None,
        }
    }

    pub fn set_firework_shooter_entity_id(&mut self, shooter_entity_id: Option<i32>) {
        self.get_metadata_mut().set(
            &definitions::firework_shooter_entity_id(),
            MetadataValue::OptionalVarInt(shooter_entity_id),
        );
    }

    pub fn is_shot_at_angle(&self) -> bool {
        metadata_boolean(self, &definitions::firework_is_shot_at_angle())
    }

    pub fn set_shot_at_angle(&mut self, is_shot_at_angle: bool) {
        self.get_metadata_mut().set(
            &definitions::firework_is_shot_at_angle(),
            MetadataValue::Boolean(is_shot_at_angle),
        );
    }

    pub fn shoot_from(
        &mut self,
        from: EntityPosition,
        to: EntityPosition,
        power: f64,
        spread: f64,
    ) {
        let mut random = rand::rng();
        self.shoot_from_with_rng(from, to, power, spread, &mut random);
    }

    pub(crate) fn shoot_from_with_rng(
        &mut self,
        from: EntityPosition,
        to: EntityPosition,
        power: f64,
        spread: f64,
        random: &mut impl Rng,
    ) {
        let delta_x = to.get_x() - from.get_x();
        let mut delta_y = to.get_y() - from.get_y();
        let delta_z = to.get_z() - from.get_z();
        if !self.has_no_gravity() {
            delta_y += delta_x.hypot(delta_z) * PROJECTILE_GRAVITY_AIM_COMPENSATION;
        }
        let length = delta_x
            .mul_add(delta_x, delta_y.mul_add(delta_y, delta_z * delta_z))
            .sqrt();
        if length == 0.0 {
            self.set_velocity(Velocity(Vector3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }));
            return;
        }
        let spread_scale = spread * PROJECTILE_SPREAD_SCALE;
        let direction_x = delta_x / length + gaussian(random) * spread_scale;
        let direction_y = delta_y / length + gaussian(random) * spread_scale;
        let direction_z = delta_z / length + gaussian(random) * spread_scale;
        let velocity_multiplier = SERVER_TICKS_PER_SECOND * power;
        self.set_velocity(Velocity(Vector3d {
            x: direction_x * velocity_multiplier,
            y: direction_y * velocity_multiplier,
            z: direction_z * velocity_multiplier,
        }));
        let head_yaw = self.get_position().get_head_yaw();
        self.set_view(
            direction_x.atan2(direction_z).to_degrees() as f32,
            direction_y
                .atan2(direction_x.hypot(direction_z))
                .to_degrees() as f32,
            head_yaw,
        );
    }
}

impl Deref for ProjectileEntity {
    type Target = GenericEntity;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}

impl DerefMut for ProjectileEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}

fn gaussian(random: &mut impl Rng) -> f64 {
    let radius = (-2.0 * random.random_range(f64::MIN_POSITIVE..1.0).ln()).sqrt();
    let angle = random.random_range(0.0..TAU);
    radius * angle.cos()
}

fn metadata_boolean(
    projectile: &ProjectileEntity,
    definition: &crate::entity::metadata::MetadataDefinition,
) -> bool {
    match projectile.get_metadata().get_value(definition) {
        MetadataValue::Boolean(is_enabled) => is_enabled,
        _ => false,
    }
}

fn metadata_byte(
    projectile: &ProjectileEntity,
    definition: &crate::entity::metadata::MetadataDefinition,
) -> i8 {
    match projectile.get_metadata().get_value(definition) {
        MetadataValue::Byte(byte) => byte,
        _ => 0,
    }
}

fn metadata_var_int(
    projectile: &ProjectileEntity,
    definition: &crate::entity::metadata::MetadataDefinition,
) -> i32 {
    match projectile.get_metadata().get_value(definition) {
        MetadataValue::VarInt(var_int) => var_int,
        _ => 0,
    }
}

fn metadata_item_stack(
    projectile: &ProjectileEntity,
    definition: &crate::entity::metadata::MetadataDefinition,
) -> ItemStack {
    match projectile.get_metadata().get_value(definition) {
        MetadataValue::Slot(slot) => slot.to_item_stack(),
        _ => ItemStack::air(),
    }
}
