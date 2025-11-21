use crate::damage_type::{
    DamageEffects, DamageScaling, DamageType, DamageTypeRegistry, DeathMessageType,
};
use crate::types::Identifier;
pub const ARROW: &DamageType = &DamageType {
    key: Identifier::vanilla_static("arrow"),
    message_id: "arrow",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const BAD_RESPAWN_POINT: &DamageType = &DamageType {
    key: Identifier::vanilla_static("bad_respawn_point"),
    message_id: "badRespawnPoint",
    scaling: DamageScaling::Always,
    exhaustion: 0.1f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::IntentionalGameDesign,
};
pub const CACTUS: &DamageType = &DamageType {
    key: Identifier::vanilla_static("cactus"),
    message_id: "cactus",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const CAMPFIRE: &DamageType = &DamageType {
    key: Identifier::vanilla_static("campfire"),
    message_id: "inFire",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Burning,
    death_message_type: DeathMessageType::Default,
};
pub const CRAMMING: &DamageType = &DamageType {
    key: Identifier::vanilla_static("cramming"),
    message_id: "cramming",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const DRAGON_BREATH: &DamageType = &DamageType {
    key: Identifier::vanilla_static("dragon_breath"),
    message_id: "dragonBreath",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const DROWN: &DamageType = &DamageType {
    key: Identifier::vanilla_static("drown"),
    message_id: "drown",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0f32,
    effects: DamageEffects::Drowning,
    death_message_type: DeathMessageType::Default,
};
pub const DRY_OUT: &DamageType = &DamageType {
    key: Identifier::vanilla_static("dry_out"),
    message_id: "dryout",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const ENDER_PEARL: &DamageType = &DamageType {
    key: Identifier::vanilla_static("ender_pearl"),
    message_id: "fall",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::FallVariants,
};
pub const EXPLOSION: &DamageType = &DamageType {
    key: Identifier::vanilla_static("explosion"),
    message_id: "explosion",
    scaling: DamageScaling::Always,
    exhaustion: 0.1f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const FALL: &DamageType = &DamageType {
    key: Identifier::vanilla_static("fall"),
    message_id: "fall",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::FallVariants,
};
pub const FALLING_ANVIL: &DamageType = &DamageType {
    key: Identifier::vanilla_static("falling_anvil"),
    message_id: "anvil",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const FALLING_BLOCK: &DamageType = &DamageType {
    key: Identifier::vanilla_static("falling_block"),
    message_id: "fallingBlock",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const FALLING_STALACTITE: &DamageType = &DamageType {
    key: Identifier::vanilla_static("falling_stalactite"),
    message_id: "fallingStalactite",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const FIREBALL: &DamageType = &DamageType {
    key: Identifier::vanilla_static("fireball"),
    message_id: "fireball",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Burning,
    death_message_type: DeathMessageType::Default,
};
pub const FIREWORKS: &DamageType = &DamageType {
    key: Identifier::vanilla_static("fireworks"),
    message_id: "fireworks",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const FLY_INTO_WALL: &DamageType = &DamageType {
    key: Identifier::vanilla_static("fly_into_wall"),
    message_id: "flyIntoWall",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const FREEZE: &DamageType = &DamageType {
    key: Identifier::vanilla_static("freeze"),
    message_id: "freeze",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0f32,
    effects: DamageEffects::Freezing,
    death_message_type: DeathMessageType::Default,
};
pub const GENERIC: &DamageType = &DamageType {
    key: Identifier::vanilla_static("generic"),
    message_id: "generic",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const GENERIC_KILL: &DamageType = &DamageType {
    key: Identifier::vanilla_static("generic_kill"),
    message_id: "genericKill",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const HOT_FLOOR: &DamageType = &DamageType {
    key: Identifier::vanilla_static("hot_floor"),
    message_id: "hotFloor",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Burning,
    death_message_type: DeathMessageType::Default,
};
pub const IN_FIRE: &DamageType = &DamageType {
    key: Identifier::vanilla_static("in_fire"),
    message_id: "inFire",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Burning,
    death_message_type: DeathMessageType::Default,
};
pub const IN_WALL: &DamageType = &DamageType {
    key: Identifier::vanilla_static("in_wall"),
    message_id: "inWall",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const INDIRECT_MAGIC: &DamageType = &DamageType {
    key: Identifier::vanilla_static("indirect_magic"),
    message_id: "indirectMagic",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const LAVA: &DamageType = &DamageType {
    key: Identifier::vanilla_static("lava"),
    message_id: "lava",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Burning,
    death_message_type: DeathMessageType::Default,
};
pub const LIGHTNING_BOLT: &DamageType = &DamageType {
    key: Identifier::vanilla_static("lightning_bolt"),
    message_id: "lightningBolt",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const MACE_SMASH: &DamageType = &DamageType {
    key: Identifier::vanilla_static("mace_smash"),
    message_id: "mace_smash",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const MAGIC: &DamageType = &DamageType {
    key: Identifier::vanilla_static("magic"),
    message_id: "magic",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const MOB_ATTACK: &DamageType = &DamageType {
    key: Identifier::vanilla_static("mob_attack"),
    message_id: "mob",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const MOB_ATTACK_NO_AGGRO: &DamageType = &DamageType {
    key: Identifier::vanilla_static("mob_attack_no_aggro"),
    message_id: "mob",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const MOB_PROJECTILE: &DamageType = &DamageType {
    key: Identifier::vanilla_static("mob_projectile"),
    message_id: "mob",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const ON_FIRE: &DamageType = &DamageType {
    key: Identifier::vanilla_static("on_fire"),
    message_id: "onFire",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0f32,
    effects: DamageEffects::Burning,
    death_message_type: DeathMessageType::Default,
};
pub const OUT_OF_WORLD: &DamageType = &DamageType {
    key: Identifier::vanilla_static("out_of_world"),
    message_id: "outOfWorld",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const OUTSIDE_BORDER: &DamageType = &DamageType {
    key: Identifier::vanilla_static("outside_border"),
    message_id: "outsideBorder",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const PLAYER_ATTACK: &DamageType = &DamageType {
    key: Identifier::vanilla_static("player_attack"),
    message_id: "player",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const PLAYER_EXPLOSION: &DamageType = &DamageType {
    key: Identifier::vanilla_static("player_explosion"),
    message_id: "explosion.player",
    scaling: DamageScaling::Always,
    exhaustion: 0.1f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const SONIC_BOOM: &DamageType = &DamageType {
    key: Identifier::vanilla_static("sonic_boom"),
    message_id: "sonic_boom",
    scaling: DamageScaling::Always,
    exhaustion: 0f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const SPIT: &DamageType = &DamageType {
    key: Identifier::vanilla_static("spit"),
    message_id: "mob",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const STALAGMITE: &DamageType = &DamageType {
    key: Identifier::vanilla_static("stalagmite"),
    message_id: "stalagmite",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const STARVE: &DamageType = &DamageType {
    key: Identifier::vanilla_static("starve"),
    message_id: "starve",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const STING: &DamageType = &DamageType {
    key: Identifier::vanilla_static("sting"),
    message_id: "sting",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const SWEET_BERRY_BUSH: &DamageType = &DamageType {
    key: Identifier::vanilla_static("sweet_berry_bush"),
    message_id: "sweetBerryBush",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Poking,
    death_message_type: DeathMessageType::Default,
};
pub const THORNS: &DamageType = &DamageType {
    key: Identifier::vanilla_static("thorns"),
    message_id: "thorns",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Thorns,
    death_message_type: DeathMessageType::Default,
};
pub const THROWN: &DamageType = &DamageType {
    key: Identifier::vanilla_static("thrown"),
    message_id: "thrown",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const TRIDENT: &DamageType = &DamageType {
    key: Identifier::vanilla_static("trident"),
    message_id: "trident",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const UNATTRIBUTED_FIREBALL: &DamageType = &DamageType {
    key: Identifier::vanilla_static("unattributed_fireball"),
    message_id: "onFire",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Burning,
    death_message_type: DeathMessageType::Default,
};
pub const WIND_CHARGE: &DamageType = &DamageType {
    key: Identifier::vanilla_static("wind_charge"),
    message_id: "mob",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const WITHER: &DamageType = &DamageType {
    key: Identifier::vanilla_static("wither"),
    message_id: "wither",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub const WITHER_SKULL: &DamageType = &DamageType {
    key: Identifier::vanilla_static("wither_skull"),
    message_id: "witherSkull",
    scaling: DamageScaling::WhenCausedByLivingNonPlayer,
    exhaustion: 0.1f32,
    effects: DamageEffects::Hurt,
    death_message_type: DeathMessageType::Default,
};
pub fn register_damage_types(registry: &mut DamageTypeRegistry) {
    registry.register(ARROW);
    registry.register(BAD_RESPAWN_POINT);
    registry.register(CACTUS);
    registry.register(CAMPFIRE);
    registry.register(CRAMMING);
    registry.register(DRAGON_BREATH);
    registry.register(DROWN);
    registry.register(DRY_OUT);
    registry.register(ENDER_PEARL);
    registry.register(EXPLOSION);
    registry.register(FALL);
    registry.register(FALLING_ANVIL);
    registry.register(FALLING_BLOCK);
    registry.register(FALLING_STALACTITE);
    registry.register(FIREBALL);
    registry.register(FIREWORKS);
    registry.register(FLY_INTO_WALL);
    registry.register(FREEZE);
    registry.register(GENERIC);
    registry.register(GENERIC_KILL);
    registry.register(HOT_FLOOR);
    registry.register(IN_FIRE);
    registry.register(IN_WALL);
    registry.register(INDIRECT_MAGIC);
    registry.register(LAVA);
    registry.register(LIGHTNING_BOLT);
    registry.register(MACE_SMASH);
    registry.register(MAGIC);
    registry.register(MOB_ATTACK);
    registry.register(MOB_ATTACK_NO_AGGRO);
    registry.register(MOB_PROJECTILE);
    registry.register(ON_FIRE);
    registry.register(OUT_OF_WORLD);
    registry.register(OUTSIDE_BORDER);
    registry.register(PLAYER_ATTACK);
    registry.register(PLAYER_EXPLOSION);
    registry.register(SONIC_BOOM);
    registry.register(SPIT);
    registry.register(STALAGMITE);
    registry.register(STARVE);
    registry.register(STING);
    registry.register(SWEET_BERRY_BUSH);
    registry.register(THORNS);
    registry.register(THROWN);
    registry.register(TRIDENT);
    registry.register(UNATTRIBUTED_FIREBALL);
    registry.register(WIND_CHARGE);
    registry.register(WITHER);
    registry.register(WITHER_SKULL);
}
