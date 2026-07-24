use crate::entity::LivingEntity;
use crate::entity::metadata::*;
use std::ops::{Deref, DerefMut};

pub struct LivingEntityMetaCast<'entity> {
    living_entity_meta: LivingEntityMeta<'entity>,
}

impl<'entity> LivingEntityMetaCast<'entity> {
    pub(crate) fn new(living_entity: &'entity mut LivingEntity) -> Self {
        Self {
            living_entity_meta: LivingEntityMeta::new(living_entity),
        }
    }
}

macro_rules! define_living_entity_meta_casts {
    ($($method:ident => $metadata:ident),* $(,)?) => {
        impl<'entity> LivingEntityMetaCast<'entity> {
            $(
                pub fn $method(self) -> Option<$metadata<'entity>> {
                    $metadata::from_living_entity_meta(self.living_entity_meta)
                }
            )*
        }
    };
}

define_living_entity_meta_casts!(
    as_avatar => AvatarMeta, as_player => PlayerMeta, as_bat => BatMeta, as_allay => AllayMeta,
    as_sniffer => SnifferMeta, as_dolphin => DolphinMeta, as_axolotl => AxolotlMeta,
    as_pufferfish => PufferfishMeta, as_salmon => SalmonMeta, as_tropical_fish => TropicalFishMeta,
    as_bee => BeeMeta, as_armadillo => ArmadilloMeta, as_fox => FoxMeta, as_ocelot => OcelotMeta,
    as_turtle => TurtleMeta, as_polar_bear => PolarBearMeta, as_hoglin => HoglinMeta,
    as_strider => StriderMeta, as_panda => PandaMeta, as_rabbit => RabbitMeta,
    as_mooshroom => MooshroomMeta, as_parrot => ParrotMeta, as_cat => CatMeta, as_wolf => WolfMeta,
    as_sheep => SheepMeta, as_happy_ghast => HappyGhastMeta, as_nautilus => NautilusMeta,
    as_zombie_nautilus => ZombieNautilusMeta, as_chicken => ChickenMeta, as_cow => CowMeta,
    as_frog => FrogMeta, as_pig => PigMeta, as_blaze => BlazeMeta, as_bogged => BoggedMeta,
    as_piglin => PiglinMeta, as_piglin_brute => PiglinBruteMeta, as_creaking => CreakingMeta,
    as_creeper => CreeperMeta, as_enderman => EndermanMeta, as_phantom => PhantomMeta,
    as_evoker => EvokerMeta, as_illusioner => IllusionerMeta, as_pillager => PillagerMeta,
    as_ravager => RavagerMeta, as_vindicator => VindicatorMeta, as_witch => WitchMeta,
    as_spider => SpiderMeta, as_cave_spider => CaveSpiderMeta, as_warden => WardenMeta,
    as_wither => WitherMeta, as_zoglin => ZoglinMeta, as_zombie => ZombieMeta,
    as_drowned => DrownedMeta, as_husk => HuskMeta, as_zombified_piglin => ZombifiedPiglinMeta,
    as_zombie_villager => ZombieVillagerMeta, as_copper_golem => CopperGolemMeta,
    as_iron_golem => IronGolemMeta, as_snow_golem => SnowGolemMeta, as_shulker => ShulkerMeta,
    as_villager => VillagerMeta, as_ghast => GhastMeta, as_horse => HorseMeta, as_camel => CamelMeta,
    as_camel_husk => CamelHuskMeta, as_donkey => DonkeyMeta, as_mule => MuleMeta,
    as_llama => LlamaMeta, as_skeleton_horse => SkeletonHorseMeta,
    as_zombie_horse => ZombieHorseMeta, as_goat => GoatMeta, as_guardian => GuardianMeta,
    as_vex => VexMeta, as_slime => SlimeMeta, as_magma_cube => MagmaCubeMeta,
    as_ender_dragon => EnderDragonMeta, as_armor_stand => ArmorStandMeta,
);

impl<'entity> Deref for LivingEntityMetaCast<'entity> {
    type Target = LivingEntityMeta<'entity>;
    fn deref(&self) -> &Self::Target {
        &self.living_entity_meta
    }
}
impl<'entity> DerefMut for LivingEntityMetaCast<'entity> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.living_entity_meta
    }
}
