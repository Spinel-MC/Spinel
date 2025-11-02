use crate as spinel;
use spinel_macros::packet_dispatcher;
use spinel_nbt::NbtCompound;
use spinel_network::types::alias::{Array, Identifier, Optional};

#[packet_dispatcher(id: 0x07)]
pub struct RegistryDataPacket {
    pub registry_id: Identifier,
    pub entries: Array<(Identifier, Optional<NbtCompound>)>,
}


//TODO: This entire thing is not functional, and I spent days trying to figure out why, but I can't.
//Attempted Fixes: 1 (increment this integer if you tried to fix it)

impl RegistryDataPacket {
    pub fn new(registry_id: Identifier, entries: Array<(Identifier, Optional<NbtCompound>)>) -> Self {
        Self { registry_id, entries }
    }

    fn from_identifiers(registry_id: &str, ids: &[&str]) -> Self {
        let entries = ids
            .iter()
            .map(|id| (id.to_string(), None))
            .collect();
        Self::new(registry_id.to_string(), Array(entries))
    }

    pub fn vanilla_registries() -> Vec<Self> {
        vec![
            Self::from_identifiers("minecraft:worldgen/biome", &[
                "minecraft:badlands", "minecraft:bamboo_jungle", "minecraft:basalt_deltas", "minecraft:beach", "minecraft:birch_forest", "minecraft:cherry_grove",
                "minecraft:cold_ocean", "minecraft:crimson_forest", "minecraft:dark_forest", "minecraft:deep_cold_ocean", "minecraft:deep_dark", "minecraft:deep_frozen_ocean",
                "minecraft:deep_lukewarm_ocean", "minecraft:deep_ocean", "minecraft:desert", "minecraft:dripstone_caves", "minecraft:end_barrens", "minecraft:end_highlands",
                "minecraft:end_midlands", "minecraft:eroded_badlands", "minecraft:flower_forest", "minecraft:forest", "minecraft:frozen_ocean", "minecraft:frozen_peaks",
                "minecraft:frozen_river", "minecraft:grove", "minecraft:ice_spikes", "minecraft:jagged_peaks", "minecraft:jungle", "minecraft:lukewarm_ocean",
                "minecraft:lush_caves", "minecraft:mangrove_swamp", "minecraft:meadow", "minecraft:mushroom_fields", "minecraft:nether_wastes", "minecraft:ocean",
                "minecraft:old_growth_birch_forest", "minecraft:old_growth_pine_taiga", "minecraft:old_growth_spruce_taiga", "minecraft:pale_garden", "minecraft:plains", "minecraft:river",
                "minecraft:savanna", "minecraft:savanna_plateau", "minecraft:small_end_islands", "minecraft:snowy_beach", "minecraft:snowy_plains", "minecraft:snowy_slopes",
                "minecraft:snowy_taiga", "minecraft:soul_sand_valley", "minecraft:sparse_jungle", "minecraft:stony_peaks", "minecraft:stony_shore",
                "minecraft:sunflower_plains", "minecraft:swamp", "minecraft:taiga", "minecraft:the_end", "minecraft:the_void", "minecraft:warm_ocean",
                "minecraft:warped_forest", "minecraft:windswept_forest", "minecraft:windswept_gravelly_hills", "minecraft:windswept_hills", "minecraft:windswept_savanna",
                "minecraft:wooded_badlands"
            ]),
            Self::from_identifiers("minecraft:chat_type", &[
                "minecraft:chat", "minecraft:emote_command", "minecraft:msg_command_incoming", "minecraft:msg_command_outgoing",
                "minecraft:say_command", "minecraft:team_msg_command_incoming", "minecraft:team_msg_command_outgoing"
            ]),
            Self::from_identifiers("minecraft:trim_pattern", &[
                "minecraft:bolt", "minecraft:coast", "minecraft:dune", "minecraft:eye", "minecraft:flow", "minecraft:host", "minecraft:raiser",
                "minecraft:rib", "minecraft:sentry", "minecraft:shaper", "minecraft:silence", "minecraft:snout", "minecraft:spire",
                "minecraft:tide", "minecraft:vex", "minecraft:ward", "minecraft:wayfinder", "minecraft:wild"
            ]),
            Self::from_identifiers("minecraft:trim_material", &[
                "minecraft:amethyst", "minecraft:copper", "minecraft:diamond", "minecraft:emerald", "minecraft:gold", "minecraft:iron",
                "minecraft:lapis", "minecraft:netherite", "minecraft:quartz", "minecraft:redstone", "minecraft:resin"
            ]),
            Self::from_identifiers("minecraft:wolf_variant", &[
                "minecraft:ashen", "minecraft:black", "minecraft:chestnut", "minecraft:pale", "minecraft:rusty",
                "minecraft:snowy", "minecraft:spotted", "minecraft:striped", "minecraft:woods"
            ]),
             Self::from_identifiers("minecraft:wolf_sound_variant", &["minecraft:angry", "minecraft:big", "minecraft:classic", "minecraft:cute", "minecraft:grumpy", "minecraft:puglin", "minecraft:sad"]),
            Self::from_identifiers("minecraft:pig_variant", &["minecraft:cold", "minecraft:temperate", "minecraft:warm"]),
            Self::from_identifiers("minecraft:frog_variant", &["minecraft:cold", "minecraft:temperate", "minecraft:warm"]),
            Self::from_identifiers("minecraft:cat_variant", &["minecraft:all_black", "minecraft:black", "minecraft:british_shorthair", "minecraft:calico", "minecraft:jellie", "minecraft:persian", "minecraft:ragdoll", "minecraft:red", "minecraft:siamese", "minecraft:tabby", "minecraft:white"]),
            Self::from_identifiers("minecraft:cow_variant", &["minecraft:cold", "minecraft:temperate", "minecraft:warm"]),
            Self::from_identifiers("minecraft:chicken_variant", &["minecraft:cold", "minecraft:temperate", "minecraft:warm"]),
            Self::from_identifiers("minecraft:painting_variant", &["minecraft:alban", "minecraft:aztec", "minecraft:aztec2", "minecraft:backyard", "minecraft:baroque", "minecraft:bomb", "minecraft:bouquet", "minecraft:burning_skull", "minecraft:bust", "minecraft:cavebird", "minecraft:changing", "minecraft:cotan", "minecraft:courbet", "minecraft:creebet", "minecraft:donkey_kong", "minecraft:earth", "minecraft:endboss", "minecraft:fern", "minecraft:fighters", "minecraft:finding", "minecraft:fire", "minecraft:graham", "minecraft:humble", "minecraft:kebab", "minecraft:lowmist", "minecraft:match", "minecraft:meditative", "minecraft:orb", "minecraft:owlemons", "minecraft:passage", "minecraft:pigscene", "minecraft:plant", "minecraft:pointer", "minecraft:pond", "minecraft:pool", "minecraft:prairie_ride", "minecraft:sea", "minecraft:skeleton", "minecraft:skull_and_roses", "minecraft:stage", "minecraft:sunflowers", "minecraft:sunset", "minecraft:tides", "minecraft:unpacked", "minecraft:void", "minecraft:wanderer", "minecraft:wasteland", "minecraft:water", "minecraft:wind", "minecraft:wither"]),
            Self::from_identifiers("minecraft:dimension_type", &[
                "minecraft:overworld", "minecraft:overworld_caves", "minecraft:the_end", "minecraft:the_nether"
            ]),
            Self::from_identifiers("minecraft:damage_type", &[
                "minecraft:arrow", "minecraft:bad_respawn_point", "minecraft:cactus", "minecraft:campfire", "minecraft:cramming", "minecraft:dragon_breath",
                "minecraft:drown", "minecraft:dry_out", "minecraft:ender_pearl", "minecraft:explosion", "minecraft:fall", "minecraft:falling_anvil",
                "minecraft:falling_block", "minecraft:falling_stalactite", "minecraft:fireball", "minecraft:fireworks", "minecraft:fly_into_wall", "minecraft:freeze",
                "minecraft:generic", "minecraft:generic_kill", "minecraft:hot_floor", "minecraft:in_fire", "minecraft:in_wall", "minecraft:indirect_magic",
                "minecraft:lava", "minecraft:lightning_bolt", "minecraft:mace_smash", "minecraft:magic", "minecraft:mob_attack", "minecraft:mob_attack_no_aggro",
                "minecraft:mob_projectile", "minecraft:on_fire", "minecraft:out_of_world", "minecraft:outside_border", "minecraft:player_attack",
                "minecraft:player_explosion", "minecraft:sonic_boom", "minecraft:spit", "minecraft:stalagmite", "minecraft:starve", "minecraft:sting",
                "minecraft:sweet_berry_bush", "minecraft:thorns", "minecraft:thrown", "minecraft:trident", "minecraft:unattributed_fireball",
                "minecraft:wind_charge", "minecraft:wither", "minecraft:wither_skull"
            ]),
            Self::from_identifiers("minecraft:banner_pattern", &["minecraft:base", "minecraft:border", "minecraft:bricks", "minecraft:circle", "minecraft:creeper", "minecraft:cross", "minecraft:curly_border", "minecraft:diagonal_left", "minecraft:diagonal_right", "minecraft:diagonal_up_left", "minecraft:diagonal_up_right", "minecraft:flow", "minecraft:flower", "minecraft:globe", "minecraft:gradient", "minecraft:gradient_up", "minecraft:guster", "minecraft:half_horizontal", "minecraft:half_horizontal_bottom", "minecraft:half_vertical", "minecraft:half_vertical_right", "minecraft:mojang", "minecraft:piglin", "minecraft:rhombus", "minecraft:skull", "minecraft:small_stripes", "minecraft:square_bottom_left", "minecraft:square_bottom_right", "minecraft:square_top_left", "minecraft:square_top_right", "minecraft:straight_cross", "minecraft:stripe_bottom", "minecraft:stripe_center", "minecraft:stripe_downleft", "minecraft:stripe_downright", "minecraft:stripe_left", "minecraft:stripe_middle", "minecraft:stripe_right", "minecraft:stripe_top", "minecraft:triangle_bottom", "minecraft:triangle_top", "minecraft:triangles_bottom", "minecraft:triangles_top"]),
            Self::from_identifiers("minecraft:enchantment", &["minecraft:aqua_affinity", "minecraft:bane_of_arthropods", "minecraft:binding_curse", "minecraft:blast_protection", "minecraft:breach", "minecraft:channeling", "minecraft:density", "minecraft:depth_strider", "minecraft:efficiency", "minecraft:feather_falling", "minecraft:fire_aspect", "minecraft:fire_protection", "minecraft:flame", "minecraft:fortune", "minecraft:frost_walker", "minecraft:impaling", "minecraft:infinity", "minecraft:knockback", "minecraft:looting", "minecraft:loyalty", "minecraft:luck_of_the_sea", "minecraft:lure", "minecraft:mending", "minecraft:multishot", "minecraft:piercing", "minecraft:power", "minecraft:projectile_protection", "minecraft:protection", "minecraft:punch", "minecraft:quick_charge", "minecraft:respiration", "minecraft:riptide", "minecraft:sharpness", "minecraft:silk_touch", "minecraft:smite", "minecraft:soul_speed", "minecraft:sweeping_edge", "minecraft:swift_sneak", "minecraft:thorns", "minecraft:unbreaking", "minecraft:vanishing_curse", "minecraft:wind_burst"]),
            Self::from_identifiers("minecraft:jukebox_song", &["minecraft:11", "minecraft:13", "minecraft:5", "minecraft:blocks", "minecraft:cat", "minecraft:chirp", "minecraft:creator", "minecraft:creator_music_box", "minecraft:far", "minecraft:mall", "minecraft:mellohi", "minecraft:otherside", "minecraft:pigstep", "minecraft:precipice", "minecraft:relic", "minecraft:stal", "minecraft:strad", "minecraft:wait", "minecraft:ward"]),
            Self::from_identifiers("minecraft:instrument", &["minecraft:admire_goat_horn", "minecraft:call_goat_horn", "minecraft:dream_goat_horn", "minecraft:feel_goat_horn", "minecraft:ponder_goat_horn", "minecraft:seek_goat_horn", "minecraft:sing_goat_horn", "minecraft:yearn_goat_horn"]),
            Self::from_identifiers("minecraft:dialog", &["minecraft:custom_options", "minecraft:quick_actions", "minecraft:server_links"]),
            Self::from_identifiers("minecraft:test_environment", &["minecraft:default"]),
            Self::from_identifiers("minecraft:test_instance", &["minecraft:always_pass"]),
        ]
    }
}