use crate::entity::metadata::{EntityMeta, LivingEntityMeta, MobMeta, definitions};
use spinel_network::types::entity_metadata::MetadataValue;
use spinel_registry::EntityType;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnderDragonPhase {
    Circling,
    Strafing,
    FlyingToThePortal,
    LandingOnThePortal,
    TakingOffFromThePortal,
    BreathAttack,
    LookingForBreathAttackPlayer,
    Roar,
    ChargingPlayer,
    FlyingToThePortalToDie,
    HoveringWithoutAi,
}

impl EnderDragonPhase {
    const fn protocol_id(self) -> i32 {
        self as i32
    }

    const fn from_protocol_id(protocol_id: i32) -> Self {
        match protocol_id {
            0 => Self::Circling,
            1 => Self::Strafing,
            2 => Self::FlyingToThePortal,
            3 => Self::LandingOnThePortal,
            4 => Self::TakingOffFromThePortal,
            5 => Self::BreathAttack,
            6 => Self::LookingForBreathAttackPlayer,
            7 => Self::Roar,
            8 => Self::ChargingPlayer,
            9 => Self::FlyingToThePortalToDie,
            _ => Self::HoveringWithoutAi,
        }
    }
}

pub struct EnderDragonMeta<'entity> {
    mob_meta: MobMeta<'entity>,
}

impl<'entity> EnderDragonMeta<'entity> {
    pub(crate) fn from_entity_meta(entity_meta: EntityMeta<'entity>) -> Option<Self> {
        (entity_meta.get_entity().get_entity_type() == EntityType::ENDER_DRAGON).then(|| Self {
            mob_meta: MobMeta::new(LivingEntityMeta::new(entity_meta)),
        })
    }

    pub fn get_phase(&self) -> EnderDragonPhase {
        match self
            .get_entity()
            .get_metadata()
            .get_value(&definitions::ender_dragon::get_phase())
        {
            MetadataValue::VarInt(phase) => EnderDragonPhase::from_protocol_id(phase),
            _ => EnderDragonPhase::HoveringWithoutAi,
        }
    }

    pub fn set_phase(&mut self, phase: EnderDragonPhase) {
        self.get_entity_mut().get_metadata_mut().set(
            &definitions::ender_dragon::get_phase(),
            MetadataValue::VarInt(phase.protocol_id()),
        );
    }
}

impl<'entity> Deref for EnderDragonMeta<'entity> {
    type Target = MobMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.mob_meta
    }
}

impl<'entity> DerefMut for EnderDragonMeta<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mob_meta
    }
}
