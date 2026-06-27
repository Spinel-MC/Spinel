use crate::entity::metadata::{EntityMeta, LivingEntityMeta, definitions};
use spinel_network::types::MainHand;
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

pub struct AvatarMeta<'entity> {
    living_entity_meta: LivingEntityMeta<'entity>,
}

impl<'entity> AvatarMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::PLAYER).then(|| Self {
            living_entity_meta: LivingEntityMeta::new(entity_meta),
        })
    }

    pub(crate) fn from_player_entity_meta(entity_meta: EntityMeta<'entity>) -> Self {
        Self {
            living_entity_meta: LivingEntityMeta::new(entity_meta),
        }
    }

    pub fn get_main_hand(&self) -> MainHand {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::avatar::get_main_hand())
        {
            MetadataValue::MainHand(main_hand) => main_hand,
            _ => MainHand::Right,
        }
    }

    pub fn set_main_hand(&mut self, main_hand: MainHand) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::avatar::get_main_hand(),
            MetadataValue::MainHand(main_hand),
        );
    }

    pub fn is_cape_enabled(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .get_flag(&definitions::avatar::is_cape_enabled())
    }

    pub fn set_cape_enabled(&mut self, is_cape_enabled: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::avatar::is_cape_enabled(), is_cape_enabled);
    }

    pub fn is_jacket_enabled(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .get_flag(&definitions::avatar::is_jacket_enabled())
    }

    pub fn set_jacket_enabled(&mut self, is_jacket_enabled: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::avatar::is_jacket_enabled(), is_jacket_enabled);
    }

    pub fn is_left_sleeve_enabled(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .get_flag(&definitions::avatar::is_left_sleeve_enabled())
    }

    pub fn set_left_sleeve_enabled(&mut self, is_left_sleeve_enabled: bool) {
        self.get_entity_mut().get_metadata_mut().set_flag(
            &definitions::avatar::is_left_sleeve_enabled(),
            is_left_sleeve_enabled,
        );
    }

    pub fn is_right_sleeve_enabled(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .get_flag(&definitions::avatar::is_right_sleeve_enabled())
    }

    pub fn set_right_sleeve_enabled(&mut self, is_right_sleeve_enabled: bool) {
        self.get_entity_mut().get_metadata_mut().set_flag(
            &definitions::avatar::is_right_sleeve_enabled(),
            is_right_sleeve_enabled,
        );
    }

    pub fn is_left_leg_enabled(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .get_flag(&definitions::avatar::is_left_pants_leg_enabled())
    }

    pub fn set_left_leg_enabled(&mut self, is_left_leg_enabled: bool) {
        self.get_entity_mut().get_metadata_mut().set_flag(
            &definitions::avatar::is_left_pants_leg_enabled(),
            is_left_leg_enabled,
        );
    }

    pub fn is_right_leg_enabled(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .get_flag(&definitions::avatar::is_right_pants_leg_enabled())
    }

    pub fn set_right_leg_enabled(&mut self, is_right_leg_enabled: bool) {
        self.get_entity_mut().get_metadata_mut().set_flag(
            &definitions::avatar::is_right_pants_leg_enabled(),
            is_right_leg_enabled,
        );
    }

    pub fn is_hat_enabled(&self) -> bool {
        self.get_entity()
            .get_metadata()
            .get_flag(&definitions::avatar::is_hat_enabled())
    }

    pub fn set_hat_enabled(&mut self, is_hat_enabled: bool) {
        self.get_entity_mut()
            .get_metadata_mut()
            .set_flag(&definitions::avatar::is_hat_enabled(), is_hat_enabled);
    }

    pub fn get_displayed_skin_parts(&self) -> i8 {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::avatar::displayed_model_parts_flags())
        {
            MetadataValue::Byte(displayed_skin_parts) => displayed_skin_parts,
            _ => 0,
        }
    }

    pub fn set_displayed_skin_parts(&mut self, displayed_skin_parts: i8) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::avatar::displayed_model_parts_flags(),
            MetadataValue::Byte(displayed_skin_parts),
        );
    }
}

impl<'entity> Deref for AvatarMeta<'entity> {
    type Target = LivingEntityMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.living_entity_meta
    }
}

impl<'entity> DerefMut for AvatarMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.living_entity_meta
    }
}
