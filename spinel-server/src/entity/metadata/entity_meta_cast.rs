use crate::entity::GenericEntity;
use crate::entity::metadata::*;
use std::ops::{Deref, DerefMut};

pub struct EntityMetaCast<'entity> {
    entity_meta: EntityMeta<'entity>,
}

impl<'entity> EntityMetaCast<'entity> {
    pub(crate) fn new(entity: &'entity mut GenericEntity) -> Self {
        Self {
            entity_meta: EntityMeta::new(entity),
        }
    }

    pub fn as_living(self) -> Option<LivingEntityMeta<'entity>> {
        self.entity_meta
            .get_entity()
            .get_entity_type()
            .is_living()
            .then(|| LivingEntityMeta::new(self.entity_meta))
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
    as_avatar => AvatarMeta,
    as_player => PlayerMeta,
    as_bat => BatMeta,
    as_allay => AllayMeta,
    as_sniffer => SnifferMeta,
    as_dolphin => DolphinMeta,
    as_axolotl => AxolotlMeta,
    as_pufferfish => PufferfishMeta,
    as_salmon => SalmonMeta,
    as_tropical_fish => TropicalFishMeta,
    as_bee => BeeMeta,
    as_armadillo => ArmadilloMeta,
    as_fox => FoxMeta,
    as_ocelot => OcelotMeta,
    as_turtle => TurtleMeta,
    as_polar_bear => PolarBearMeta,
    as_hoglin => HoglinMeta,
    as_strider => StriderMeta,
    as_panda => PandaMeta,
    as_rabbit => RabbitMeta,
    as_mooshroom => MooshroomMeta,
    as_parrot => ParrotMeta,
    as_cat => CatMeta,
    as_wolf => WolfMeta,
    as_sheep => SheepMeta,
    as_happy_ghast => HappyGhastMeta,
    as_nautilus => NautilusMeta,
    as_zombie_nautilus => ZombieNautilusMeta,
    as_chicken => ChickenMeta,
    as_cow => CowMeta,
    as_frog => FrogMeta,
    as_pig => PigMeta,
    as_blaze => BlazeMeta,
    as_bogged => BoggedMeta,
    as_piglin => PiglinMeta,
    as_piglin_brute => PiglinBruteMeta,
    as_creaking => CreakingMeta,
    as_creeper => CreeperMeta,
    as_enderman => EndermanMeta,
    as_phantom => PhantomMeta,
    as_evoker => EvokerMeta,
    as_illusioner => IllusionerMeta,
    as_pillager => PillagerMeta,
    as_ravager => RavagerMeta,
    as_vindicator => VindicatorMeta,
    as_witch => WitchMeta,
    as_spider => SpiderMeta,
    as_cave_spider => CaveSpiderMeta,
    as_warden => WardenMeta,
    as_wither => WitherMeta,
    as_zoglin => ZoglinMeta,
    as_zombie => ZombieMeta,
    as_drowned => DrownedMeta,
    as_husk => HuskMeta,
    as_zombified_piglin => ZombifiedPiglinMeta,
    as_zombie_villager => ZombieVillagerMeta,
    as_copper_golem => CopperGolemMeta,
    as_iron_golem => IronGolemMeta,
    as_snow_golem => SnowGolemMeta,
    as_shulker => ShulkerMeta,
    as_villager => VillagerMeta,
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
    as_ghast => GhastMeta,
    as_horse => HorseMeta,
    as_camel => CamelMeta,
    as_camel_husk => CamelHuskMeta,
    as_donkey => DonkeyMeta,
    as_mule => MuleMeta,
    as_llama => LlamaMeta,
    as_skeleton_horse => SkeletonHorseMeta,
    as_zombie_horse => ZombieHorseMeta,
    as_goat => GoatMeta,
    as_guardian => GuardianMeta,
    as_vex => VexMeta,
    as_area_effect_cloud => AreaEffectCloudMeta,
    as_fishing_hook => FishingHookMeta,
    as_end_crystal => EndCrystalMeta,
    as_item_frame => ItemFrameMeta,
    as_painting => PaintingMeta,
    as_primed_tnt => PrimedTntMeta,
    as_ominous_item_spawner => OminousItemSpawnerMeta,
    as_falling_block => FallingBlockMeta,
    as_slime => SlimeMeta,
    as_magma_cube => MagmaCubeMeta,
    as_ender_dragon => EnderDragonMeta,
    as_armor_stand => ArmorStandMeta,
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
