use crate::entity::metadata::{MetadataDefinition, definitions};
use crate::entity::{EntityId, ProjectileEntity};
use spinel_network::types::Slot;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::{EntityType, ItemStack};
use std::ops::{Deref, DerefMut};

pub struct ProjectileEntityMeta<'entity> {
    projectile: &'entity mut ProjectileEntity,
}

impl<'entity> ProjectileEntityMeta<'entity> {
    pub(crate) fn new(projectile: &'entity mut ProjectileEntity) -> Self {
        Self { projectile }
    }

    pub fn as_arrow(self) -> Option<ArrowMeta<'entity>> {
        (self.projectile.get_entity_type() == EntityType::ARROW).then(|| ArrowMeta {
            abstract_arrow_meta: AbstractArrowMeta::new(self),
        })
    }

    pub fn as_spectral_arrow(self) -> Option<SpectralArrowMeta<'entity>> {
        (self.projectile.get_entity_type() == EntityType::SPECTRAL_ARROW).then(|| SpectralArrowMeta {
            abstract_arrow_meta: AbstractArrowMeta::new(self),
        })
    }

    pub fn as_thrown_trident(self) -> Option<ThrownTridentMeta<'entity>> {
        (self.projectile.get_entity_type() == EntityType::TRIDENT).then(|| ThrownTridentMeta {
            abstract_arrow_meta: AbstractArrowMeta::new(self),
        })
    }

    pub fn as_wither_skull(self) -> Option<WitherSkullMeta<'entity>> {
        (self.projectile.get_entity_type() == EntityType::WITHER_SKULL).then_some(WitherSkullMeta {
            projectile_meta: self,
        })
    }

    pub fn as_firework_rocket(self) -> Option<FireworkRocketMeta<'entity>> {
        (self.projectile.get_entity_type() == EntityType::FIREWORK_ROCKET).then_some(
            FireworkRocketMeta {
                projectile_meta: self,
            },
        )
    }

    pub fn as_fireball(self) -> Option<FireballMeta<'entity>> {
        (self.projectile.get_entity_type() == EntityType::FIREBALL).then_some(FireballMeta {
            projectile_meta: self,
        })
    }

    pub fn as_small_fireball(self) -> Option<SmallFireballMeta<'entity>> {
        (self.projectile.get_entity_type() == EntityType::SMALL_FIREBALL).then_some(SmallFireballMeta {
            projectile_meta: self,
        })
    }

    pub fn as_dragon_fireball(self) -> Option<DragonFireballMeta<'entity>> {
        (self.projectile.get_entity_type() == EntityType::DRAGON_FIREBALL).then_some(
            DragonFireballMeta {
                projectile_meta: self,
            },
        )
    }

    pub fn as_wind_charge(self) -> Option<WindChargeMeta<'entity>> {
        (self.projectile.get_entity_type() == EntityType::WIND_CHARGE).then(|| WindChargeMeta {
            abstract_wind_charge_meta: AbstractWindChargeMeta::new(self),
        })
    }

    pub fn as_breeze_wind_charge(self) -> Option<BreezeWindChargeMeta<'entity>> {
        (self.projectile.get_entity_type() == EntityType::BREEZE_WIND_CHARGE).then(|| {
            BreezeWindChargeMeta {
                abstract_wind_charge_meta: AbstractWindChargeMeta::new(self),
            }
        })
    }

    pub fn as_snowball(self) -> Option<SnowballMeta<'entity>> {
        (self.projectile.get_entity_type() == EntityType::SNOWBALL).then(|| SnowballMeta {
            thrown_item_projectile_meta: ThrownItemProjectileMeta::new(self),
        })
    }

    pub fn as_thrown_egg(self) -> Option<ThrownEggMeta<'entity>> {
        (self.projectile.get_entity_type() == EntityType::EGG).then(|| ThrownEggMeta {
            thrown_item_projectile_meta: ThrownItemProjectileMeta::new(self),
        })
    }

    pub fn as_thrown_ender_pearl(self) -> Option<ThrownEnderPearlMeta<'entity>> {
        (self.projectile.get_entity_type() == EntityType::ENDER_PEARL).then(|| ThrownEnderPearlMeta {
            thrown_item_projectile_meta: ThrownItemProjectileMeta::new(self),
        })
    }

    pub fn as_thrown_experience_bottle(self) -> Option<ThrownExperienceBottleMeta<'entity>> {
        (self.projectile.get_entity_type() == EntityType::EXPERIENCE_BOTTLE).then(|| {
            ThrownExperienceBottleMeta {
                thrown_item_projectile_meta: ThrownItemProjectileMeta::new(self),
            }
        })
    }

    pub fn as_splash_potion(self) -> Option<SplashPotionMeta<'entity>> {
        (self.projectile.get_entity_type() == EntityType::SPLASH_POTION).then(|| SplashPotionMeta {
            thrown_item_projectile_meta: ThrownItemProjectileMeta::new(self),
        })
    }

    pub fn as_lingering_potion(self) -> Option<LingeringPotionMeta<'entity>> {
        (self.projectile.get_entity_type() == EntityType::LINGERING_POTION).then(|| {
            LingeringPotionMeta {
                thrown_item_projectile_meta: ThrownItemProjectileMeta::new(self),
            }
        })
    }

    pub fn as_eye_of_ender(self) -> Option<EyeOfEnderMeta<'entity>> {
        (self.projectile.get_entity_type() == EntityType::EYE_OF_ENDER).then_some(EyeOfEnderMeta {
            projectile_meta: self,
        })
    }

    fn get_projectile(&self) -> &ProjectileEntity {
        self.projectile
    }

    fn projectile_mut(&mut self) -> &mut ProjectileEntity {
        self.projectile
    }
}

pub struct AbstractArrowMeta<'entity> {
    projectile_meta: ProjectileEntityMeta<'entity>,
}

impl<'entity> AbstractArrowMeta<'entity> {
    fn new(projectile_meta: ProjectileEntityMeta<'entity>) -> Self {
        Self { projectile_meta }
    }

    pub fn is_critical(&self) -> bool {
        self.get_projectile()
            .get_metadata()
            .flag(&definitions::is_critical_arrow())
    }

    pub fn set_critical(&mut self, is_critical: bool) {
        self.projectile_mut()
            .get_metadata_mut()
            .set_flag(&definitions::is_critical_arrow(), is_critical);
    }

    pub fn is_no_clip(&self) -> bool {
        self.get_projectile()
            .get_metadata()
            .flag(&definitions::has_no_clip_arrow())
    }

    pub fn set_no_clip(&mut self, is_no_clip: bool) {
        self.projectile_mut()
            .get_metadata_mut()
            .set_flag(&definitions::has_no_clip_arrow(), is_no_clip);
    }

    pub fn get_piercing_level(&self) -> i8 {
        metadata_byte(self.get_projectile(), &definitions::get_piercing_level())
    }

    pub fn set_piercing_level(&mut self, piercing_level: i8) {
        self.projectile_mut().get_metadata_mut().set(
            &definitions::get_piercing_level(),
            MetadataValue::Byte(piercing_level),
        );
    }

    pub fn is_in_ground(&self) -> bool {
        metadata_boolean(self.get_projectile(), &definitions::is_arrow_in_ground())
    }

    pub fn set_in_ground(&mut self, is_in_ground: bool) {
        self.projectile_mut().get_metadata_mut().set(
            &definitions::is_arrow_in_ground(),
            MetadataValue::Boolean(is_in_ground),
        );
    }
}

impl<'entity> Deref for AbstractArrowMeta<'entity> {
    type Target = ProjectileEntityMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.projectile_meta
    }
}

impl<'entity> DerefMut for AbstractArrowMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.projectile_meta
    }
}

pub struct ArrowMeta<'entity> {
    abstract_arrow_meta: AbstractArrowMeta<'entity>,
}

impl<'entity> ArrowMeta<'entity> {
    pub fn get_color(&self) -> i32 {
        metadata_var_int(self.get_projectile(), &definitions::arrow_color())
    }

    pub fn set_color(&mut self, color: i32) {
        self.projectile_mut()
            .get_metadata_mut()
            .set(&definitions::arrow_color(), MetadataValue::VarInt(color));
    }

    pub fn get_shooter(&self) -> Option<EntityId> {
        self.get_projectile().get_shooter()
    }

    pub fn set_shooter(&mut self, shooter: Option<EntityId>) {
        self.projectile_mut().set_shooter(shooter);
    }
}

impl<'entity> Deref for ArrowMeta<'entity> {
    type Target = AbstractArrowMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_arrow_meta
    }
}

impl<'entity> DerefMut for ArrowMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_arrow_meta
    }
}

pub struct SpectralArrowMeta<'entity> {
    abstract_arrow_meta: AbstractArrowMeta<'entity>,
}

impl<'entity> SpectralArrowMeta<'entity> {
    pub fn get_shooter(&self) -> Option<EntityId> {
        self.get_projectile().get_shooter()
    }

    pub fn set_shooter(&mut self, shooter: Option<EntityId>) {
        self.projectile_mut().set_shooter(shooter);
    }
}

impl<'entity> Deref for SpectralArrowMeta<'entity> {
    type Target = AbstractArrowMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_arrow_meta
    }
}

impl<'entity> DerefMut for SpectralArrowMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_arrow_meta
    }
}

pub struct ThrownTridentMeta<'entity> {
    abstract_arrow_meta: AbstractArrowMeta<'entity>,
}

impl<'entity> ThrownTridentMeta<'entity> {
    pub fn get_loyalty_level(&self) -> i8 {
        metadata_byte(self.get_projectile(), &definitions::trident_loyalty_level())
    }

    pub fn set_loyalty_level(&mut self, loyalty_level: i8) {
        self.projectile_mut().get_metadata_mut().set(
            &definitions::trident_loyalty_level(),
            MetadataValue::Byte(loyalty_level),
        );
    }

    pub fn has_enchantment_glint(&self) -> bool {
        metadata_boolean(
            self.get_projectile(),
            &definitions::trident_has_enchantment_glint(),
        )
    }

    pub fn set_has_enchantment_glint(&mut self, has_enchantment_glint: bool) {
        self.projectile_mut().get_metadata_mut().set(
            &definitions::trident_has_enchantment_glint(),
            MetadataValue::Boolean(has_enchantment_glint),
        );
    }
}

impl<'entity> Deref for ThrownTridentMeta<'entity> {
    type Target = AbstractArrowMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_arrow_meta
    }
}

impl<'entity> DerefMut for ThrownTridentMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_arrow_meta
    }
}

pub struct WitherSkullMeta<'entity> {
    projectile_meta: ProjectileEntityMeta<'entity>,
}

impl<'entity> WitherSkullMeta<'entity> {
    pub fn is_invulnerable(&self) -> bool {
        metadata_boolean(
            self.get_projectile(),
            &definitions::wither_skull_is_invulnerable(),
        )
    }

    pub fn set_invulnerable(&mut self, is_invulnerable: bool) {
        self.projectile_mut().get_metadata_mut().set(
            &definitions::wither_skull_is_invulnerable(),
            MetadataValue::Boolean(is_invulnerable),
        );
    }

    pub fn get_shooter(&self) -> Option<EntityId> {
        self.get_projectile().get_shooter()
    }

    pub fn set_shooter(&mut self, shooter: Option<EntityId>) {
        self.projectile_mut().set_shooter(shooter);
    }
}

impl<'entity> Deref for WitherSkullMeta<'entity> {
    type Target = ProjectileEntityMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.projectile_meta
    }
}

impl<'entity> DerefMut for WitherSkullMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.projectile_meta
    }
}

pub struct FireworkRocketMeta<'entity> {
    projectile_meta: ProjectileEntityMeta<'entity>,
}

impl<'entity> FireworkRocketMeta<'entity> {
    pub fn get_firework_info(&self) -> ItemStack {
        metadata_item_stack(self.get_projectile(), &definitions::get_firework_info())
    }

    pub fn set_firework_info(&mut self, firework_info: ItemStack) {
        set_metadata_item_stack(
            self.projectile_mut(),
            &definitions::get_firework_info(),
            firework_info,
        );
    }

    pub fn get_shooter_entity_id(&self) -> Option<i32> {
        match self
            .get_projectile()
            .get_metadata()
            .get_value(&definitions::firework_shooter_entity_id())
        {
            MetadataValue::OptionalVarInt(shooter_entity_id) => shooter_entity_id,
            _ => None,
        }
    }

    pub fn get_shooter(&self) -> Option<EntityId> {
        self.get_projectile().get_shooter()
    }

    pub fn set_shooter(&mut self, shooter: Option<EntityId>) {
        self.projectile_mut().set_shooter(shooter);
    }

    pub fn is_shot_at_angle(&self) -> bool {
        metadata_boolean(self.get_projectile(), &definitions::firework_is_shot_at_angle())
    }

    pub fn set_shot_at_angle(&mut self, is_shot_at_angle: bool) {
        self.projectile_mut().get_metadata_mut().set(
            &definitions::firework_is_shot_at_angle(),
            MetadataValue::Boolean(is_shot_at_angle),
        );
    }
}

impl<'entity> Deref for FireworkRocketMeta<'entity> {
    type Target = ProjectileEntityMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.projectile_meta
    }
}

impl<'entity> DerefMut for FireworkRocketMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.projectile_meta
    }
}

pub struct FireballMeta<'entity> {
    projectile_meta: ProjectileEntityMeta<'entity>,
}

impl<'entity> FireballMeta<'entity> {
    pub fn get_item(&self) -> ItemStack {
        metadata_item_stack(self.get_projectile(), &definitions::fireball::get_item())
    }

    pub fn set_item(&mut self, item: ItemStack) {
        set_metadata_item_stack(self.projectile_mut(), &definitions::fireball::get_item(), item);
    }

    pub fn get_shooter(&self) -> Option<EntityId> {
        self.get_projectile().get_shooter()
    }

    pub fn set_shooter(&mut self, shooter: Option<EntityId>) {
        self.projectile_mut().set_shooter(shooter);
    }
}

impl<'entity> Deref for FireballMeta<'entity> {
    type Target = ProjectileEntityMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.projectile_meta
    }
}

impl<'entity> DerefMut for FireballMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.projectile_meta
    }
}

pub struct SmallFireballMeta<'entity> {
    projectile_meta: ProjectileEntityMeta<'entity>,
}

impl<'entity> SmallFireballMeta<'entity> {
    pub fn get_item(&self) -> ItemStack {
        metadata_item_stack(self.get_projectile(), &definitions::smart_fireball::get_item())
    }

    pub fn set_item(&mut self, item: ItemStack) {
        set_metadata_item_stack(
            self.projectile_mut(),
            &definitions::smart_fireball::get_item(),
            item,
        );
    }

    pub fn get_shooter(&self) -> Option<EntityId> {
        self.get_projectile().get_shooter()
    }

    pub fn set_shooter(&mut self, shooter: Option<EntityId>) {
        self.projectile_mut().set_shooter(shooter);
    }
}

impl<'entity> Deref for SmallFireballMeta<'entity> {
    type Target = ProjectileEntityMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.projectile_meta
    }
}

impl<'entity> DerefMut for SmallFireballMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.projectile_meta
    }
}

pub struct DragonFireballMeta<'entity> {
    projectile_meta: ProjectileEntityMeta<'entity>,
}

impl<'entity> DragonFireballMeta<'entity> {
    pub fn get_shooter(&self) -> Option<EntityId> {
        self.get_projectile().get_shooter()
    }

    pub fn set_shooter(&mut self, shooter: Option<EntityId>) {
        self.projectile_mut().set_shooter(shooter);
    }
}

impl<'entity> Deref for DragonFireballMeta<'entity> {
    type Target = ProjectileEntityMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.projectile_meta
    }
}

impl<'entity> DerefMut for DragonFireballMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.projectile_meta
    }
}

pub struct AbstractWindChargeMeta<'entity> {
    projectile_meta: ProjectileEntityMeta<'entity>,
}

impl<'entity> AbstractWindChargeMeta<'entity> {
    fn new(projectile_meta: ProjectileEntityMeta<'entity>) -> Self {
        Self { projectile_meta }
    }

    pub fn get_shooter(&self) -> Option<EntityId> {
        self.get_projectile().get_shooter()
    }

    pub fn set_shooter(&mut self, shooter: Option<EntityId>) {
        self.projectile_mut().set_shooter(shooter);
    }
}

impl<'entity> Deref for AbstractWindChargeMeta<'entity> {
    type Target = ProjectileEntityMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.projectile_meta
    }
}

impl<'entity> DerefMut for AbstractWindChargeMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.projectile_meta
    }
}

pub struct WindChargeMeta<'entity> {
    abstract_wind_charge_meta: AbstractWindChargeMeta<'entity>,
}

impl<'entity> Deref for WindChargeMeta<'entity> {
    type Target = AbstractWindChargeMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_wind_charge_meta
    }
}

impl<'entity> DerefMut for WindChargeMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_wind_charge_meta
    }
}

pub struct BreezeWindChargeMeta<'entity> {
    abstract_wind_charge_meta: AbstractWindChargeMeta<'entity>,
}

impl<'entity> Deref for BreezeWindChargeMeta<'entity> {
    type Target = AbstractWindChargeMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.abstract_wind_charge_meta
    }
}

impl<'entity> DerefMut for BreezeWindChargeMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.abstract_wind_charge_meta
    }
}

pub struct ThrownItemProjectileMeta<'entity> {
    projectile_meta: ProjectileEntityMeta<'entity>,
}

impl<'entity> ThrownItemProjectileMeta<'entity> {
    fn new(projectile_meta: ProjectileEntityMeta<'entity>) -> Self {
        Self { projectile_meta }
    }

    pub fn get_item(&self) -> ItemStack {
        metadata_item_stack(self.get_projectile(), &definitions::projectile_item())
    }

    pub fn set_item(&mut self, item: ItemStack) {
        set_metadata_item_stack(self.projectile_mut(), &definitions::projectile_item(), item);
    }
}

impl<'entity> Deref for ThrownItemProjectileMeta<'entity> {
    type Target = ProjectileEntityMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.projectile_meta
    }
}

impl<'entity> DerefMut for ThrownItemProjectileMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.projectile_meta
    }
}

macro_rules! thrown_item_projectile_owner {
    ($name:ident) => {
        pub struct $name<'entity> {
            thrown_item_projectile_meta: ThrownItemProjectileMeta<'entity>,
        }

        impl<'entity> Deref for $name<'entity> {
            type Target = ThrownItemProjectileMeta<'entity>;

            fn deref(&self) -> &Self::Target {
                &self.thrown_item_projectile_meta
            }
        }

        impl<'entity> DerefMut for $name<'entity> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.thrown_item_projectile_meta
            }
        }
    };
}

thrown_item_projectile_owner!(SnowballMeta);
thrown_item_projectile_owner!(ThrownEggMeta);
thrown_item_projectile_owner!(ThrownEnderPearlMeta);
thrown_item_projectile_owner!(ThrownExperienceBottleMeta);
thrown_item_projectile_owner!(SplashPotionMeta);
thrown_item_projectile_owner!(LingeringPotionMeta);

pub struct EyeOfEnderMeta<'entity> {
    projectile_meta: ProjectileEntityMeta<'entity>,
}

impl<'entity> EyeOfEnderMeta<'entity> {
    pub fn get_item(&self) -> ItemStack {
        metadata_item_stack(self.get_projectile(), &definitions::eye_of_ender::get_item())
    }

    pub fn set_item(&mut self, item: ItemStack) {
        set_metadata_item_stack(
            self.projectile_mut(),
            &definitions::eye_of_ender::get_item(),
            item,
        );
    }
}

impl<'entity> Deref for EyeOfEnderMeta<'entity> {
    type Target = ProjectileEntityMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.projectile_meta
    }
}

impl<'entity> DerefMut for EyeOfEnderMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.projectile_meta
    }
}

fn metadata_boolean(projectile: &ProjectileEntity, definition: &MetadataDefinition) -> bool {
    match projectile.get_metadata().get_value(definition) {
        MetadataValue::Boolean(is_enabled) => is_enabled,
        _ => false,
    }
}

fn metadata_byte(projectile: &ProjectileEntity, definition: &MetadataDefinition) -> i8 {
    match projectile.get_metadata().get_value(definition) {
        MetadataValue::Byte(byte) => byte,
        _ => 0,
    }
}

fn metadata_var_int(projectile: &ProjectileEntity, definition: &MetadataDefinition) -> i32 {
    match projectile.get_metadata().get_value(definition) {
        MetadataValue::VarInt(var_int) => var_int,
        _ => 0,
    }
}

fn metadata_item_stack(
    projectile: &ProjectileEntity,
    definition: &MetadataDefinition,
) -> ItemStack {
    match projectile.get_metadata().get_value(definition) {
        MetadataValue::Slot(slot) => slot.to_item_stack(),
        _ => ItemStack::air(),
    }
}

fn set_metadata_item_stack(
    projectile: &mut ProjectileEntity,
    definition: &MetadataDefinition,
    item_stack: ItemStack,
) {
    projectile.get_metadata_mut().set(
        definition,
        MetadataValue::Slot(Slot::from_item_stack(&item_stack)),
    );
}
