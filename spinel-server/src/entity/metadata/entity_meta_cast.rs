use crate::entity::EntityState;
use crate::entity::metadata::*;
use std::ops::{Deref, DerefMut};

pub struct EntityMetaCast<'entity> {
    entity_meta: EntityMeta<'entity>,
}

impl<'entity> EntityMetaCast<'entity> {
    pub(crate) fn new(state: &'entity mut EntityState) -> Self {
        Self {
            entity_meta: EntityMeta::new(state),
        }
    }
}

macro_rules! define_entity_meta_casts {
    ($($method:ident => $metadata:ident),* $(,)?) => {
        impl<'entity> EntityMetaCast<'entity> {
            $(
                pub fn $method(self) -> Option<$metadata<'entity>> {
                    $metadata::from_entity_meta(self.entity_meta)
                }
            )*
        }
    };
}

define_entity_meta_casts!(
    as_interaction => InteractionMeta,
    as_block_display => BlockDisplayMeta,
    as_item_display => ItemDisplayMeta,
    as_text_display => TextDisplayMeta,
    as_boat => BoatMeta,
    as_minecart => MinecartMeta,
    as_chest_minecart => ChestMinecartMeta,
    as_hopper_minecart => HopperMinecartMeta,
    as_spawner_minecart => SpawnerMinecartMeta,
    as_tnt_minecart => TntMinecartMeta,
    as_furnace_minecart => FurnaceMinecartMeta,
    as_command_block_minecart => CommandBlockMinecartMeta,
    as_area_effect_cloud => AreaEffectCloudMeta,
    as_fishing_hook => FishingHookMeta,
    as_end_crystal => EndCrystalMeta,
    as_item_frame => ItemFrameMeta,
    as_painting => PaintingMeta,
    as_primed_tnt => PrimedTntMeta,
    as_ominous_item_spawner => OminousItemSpawnerMeta,
    as_falling_block => FallingBlockMeta,
);

impl<'entity> Deref for EntityMetaCast<'entity> {
    type Target = EntityMeta<'entity>;

    fn deref(&self) -> &Self::Target {
        &self.entity_meta
    }
}

impl<'entity> DerefMut for EntityMetaCast<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_meta
    }
}
