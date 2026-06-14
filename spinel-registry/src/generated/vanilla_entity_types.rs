use crate::Attribute;
use crate::entity::{EntityAttachmentOffset, EntityDefaultAttribute, EntityPacketType, EntityType};
const ACACIA_BOAT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.28125, 0.0),
];
const ACACIA_CHEST_BOAT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.28125, 0.0),
];
const ALLAY_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.6000000238418579, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.6000000238418579, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, -0.03999999910593033, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.30000001192092896, 0.0),
];
const AREA_EFFECT_CLOUD_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.25, 0.0),
];
const ARMADILLO_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.6499999761581421, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.6499999761581421, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.32499998807907104, 0.0),
];
const ARMOR_STAND_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.975000023841858, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.975000023841858, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.987500011920929, 0.0),
];
const ARROW_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.25, 0.0),
];
const AXOLOTL_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.41999998688697815, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.41999998688697815, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.20999999344348907, 0.0),
];
const BAMBOO_CHEST_RAFT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.28125, 0.0),
];
const BAMBOO_RAFT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.28125, 0.0),
];
const BAT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.8999999761581421, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.8999999761581421, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.44999998807907104, 0.0),
];
const BEE_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.6000000238418579, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.6000000238418579, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.30000001192092896, 0.0),
];
const BIRCH_BOAT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.28125, 0.0),
];
const BIRCH_CHEST_BOAT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.28125, 0.0),
];
const BLAZE_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.7999999523162842, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.7999999523162842, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.8999999761581421, 0.0),
];
const BLOCK_DISPLAY_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.0, 0.0),
];
const BOGGED_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.9900000095367432, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.9900000095367432, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.699999988079071, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.9950000047683716, 0.0),
];
const BREEZE_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.7699999809265137, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.7699999809265137, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.8849999904632568, 0.0),
];
const BREEZE_WIND_CHARGE_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.3125, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.3125, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.15625, 0.0),
];
const CAMEL_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 2.375, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 2.375, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 1.1875, 0.0),
];
const CAMEL_HUSK_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 2.375, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 2.375, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 1.1875, 0.0),
];
const CAT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.699999988079071, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.512499988079071, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.3499999940395355, 0.0),
];
const CAVE_SPIDER_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.25, 0.0),
];
const CHERRY_BOAT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.28125, 0.0),
];
const CHERRY_CHEST_BOAT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.28125, 0.0),
];
const CHEST_MINECART_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.699999988079071, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.1875, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.3499999940395355, 0.0),
];
const CHICKEN_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.699999988079071, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0017448145896196369, 0.7, -0.099984610080719),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.3499999940395355, 0.0),
];
const COD_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.30000001192092896, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.30000001192092896, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.15000000596046448, 0.0),
];
const COPPER_GOLEM_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.9800000190734864, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.9800000190734864, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.4900000095367432, 0.0),
];
const COMMAND_BLOCK_MINECART_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.699999988079071, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.1875, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.3499999940395355, 0.0),
];
const COW_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.399999976158142, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.368749976158142, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.699999988079071, 0.0),
];
const CREAKING_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 2.700000047683716, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 2.700000047683716, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 1.350000023841858, 0.0),
];
const CREEPER_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.7000000476837158, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.7000000476837158, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.8500000238418579, 0.0),
];
const DARK_OAK_BOAT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.28125, 0.0),
];
const DARK_OAK_CHEST_BOAT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.28125, 0.0),
];
const DOLPHIN_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.6000000238418579, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.6000000238418579, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.30000001192092896, 0.0),
];
const DONKEY_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.5, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.1124999523162842, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.75, 0.0),
];
const DRAGON_FIREBALL_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.0, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.0, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.5, 0.0),
];
const DROWNED_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.950000047683716, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 2.012500047683716, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.699999988079071, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.975000023841858, 0.0),
];
const EGG_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.25, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.25, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.125, 0.0),
];
const ELDER_GUARDIAN_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.997499942779541, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 2.3506250381469727, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.9987499713897704, 0.0),
];
const ENDERMAN_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 2.9000000953674316, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 2.8062500953674316, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 1.4500000476837158, 0.0),
];
const ENDERMITE_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.30000001192092896, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.23749999701976776, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.15000000596046448, 0.0),
];
const ENDER_DRAGON_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 8.0, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 3.0, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 4.0, 0.0),
];
const ENDER_PEARL_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.25, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.25, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.125, 0.0),
];
const END_CRYSTAL_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 2.0, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 2.0, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 1.0, 0.0),
];
const EVOKER_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.950000047683716, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 2.0, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.6000000238418579, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.975000023841858, 0.0),
];
const EVOKER_FANGS_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.800000011920929, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.800000011920929, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.4000000059604645, 0.0),
];
const EXPERIENCE_BOTTLE_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.25, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.25, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.125, 0.0),
];
const EXPERIENCE_ORB_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.25, 0.0),
];
const EYE_OF_ENDER_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.25, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.25, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.125, 0.0),
];
const FALLING_BLOCK_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.9800000190734864, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.9800000190734864, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.4900000095367432, 0.0),
];
const FIREBALL_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.0, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.0, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.5, 0.0),
];
const FIREWORK_ROCKET_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.25, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.25, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.125, 0.0),
];
const FOX_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.699999988079071, 0.0),
    EntityAttachmentOffset::new("passenger", 0.004362036474049091, 0.6375, -0.24996152520179749),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.3499999940395355, 0.0),
];
const FROG_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5, 0.0),
    EntityAttachmentOffset::new("passenger", 0.004362036474049091, 0.375, -0.24996152520179749),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.25, 0.0),
];
const FURNACE_MINECART_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.699999988079071, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.1875, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.3499999940395355, 0.0),
];
const GHAST_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 4.0, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 4.0625, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, -0.5, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 2.0, 0.0),
];
const HAPPY_GHAST_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 4.0, 0.0),
    EntityAttachmentOffset::new("passenger", -0.029661848023533825, 4.0, 1.699738371372223),
    EntityAttachmentOffset::new("vehicle", 0.0, -0.5, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 2.0, 0.0),
];
const GIANT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 12.0, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 12.0, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 3.75, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 6.0, 0.0),
];
const GLOW_ITEM_FRAME_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.25, 0.0),
];
const GLOW_SQUID_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.800000011920929, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.800000011920929, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.4000000059604645, 0.0),
];
const GOAT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.2999999523162842, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.1124999523162842, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.6499999761581421, 0.0),
];
const GUARDIAN_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.8500000238418579, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.975000023841858, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.42500001192092896, 0.0),
];
const HOGLIN_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.399999976158142, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.493749976158142, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.699999988079071, 0.0),
];
const HOPPER_MINECART_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.699999988079071, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.1875, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.3499999940395355, 0.0),
];
const HORSE_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.600000023841858, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.443750023841858, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.800000011920929, 0.0),
];
const HUSK_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.950000047683716, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 2.075000047683716, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.699999988079071, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.975000023841858, 0.0),
];
const ILLUSIONER_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.950000047683716, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 2.0, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.6000000238418579, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.975000023841858, 0.0),
];
const INTERACTION_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.0, 0.0),
];
const IRON_GOLEM_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 2.700000047683716, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 2.700000047683716, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 1.350000023841858, 0.0),
];
const ITEM_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.25, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.25, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.125, 0.0),
];
const ITEM_DISPLAY_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.0, 0.0),
];
const ITEM_FRAME_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.25, 0.0),
];
const JUNGLE_BOAT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.28125, 0.0),
];
const JUNGLE_CHEST_BOAT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.28125, 0.0),
];
const LEASH_KNOT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.25, 0.0),
];
const LIGHTNING_BOLT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.0, 0.0),
];
const LLAMA_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.8700000047683716, 0.0),
    EntityAttachmentOffset::new("passenger", 0.00523444376885891, 1.37, -0.299953830242157),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.9350000023841858, 0.0),
];
const LLAMA_SPIT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.25, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.25, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.125, 0.0),
];
const MAGMA_CUBE_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5199999809265137, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5199999809265137, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.25999999046325684, 0.0),
];
const MANGROVE_BOAT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.28125, 0.0),
];
const MANGROVE_CHEST_BOAT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.28125, 0.0),
];
const MANNEQUIN_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.7999999523162842, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.7999999523162842, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.6, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.8999999761581421, 0.0),
];
const MARKER_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.0, 0.0),
];
const MINECART_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.699999988079071, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.1875, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.3499999940395355, 0.0),
];
const MOOSHROOM_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.399999976158142, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.368749976158142, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.699999988079071, 0.0),
];
const MULE_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.600000023841858, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.212499976158142, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.800000011920929, 0.0),
];
const NAUTILUS_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.949999988079071, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.1375000476837158, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.4749999940395355, 0.0),
];
const OAK_BOAT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.28125, 0.0),
];
const OAK_CHEST_BOAT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.28125, 0.0),
];
const OCELOT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.699999988079071, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.637499988079071, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.3499999940395355, 0.0),
];
const OMINOUS_ITEM_SPAWNER_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.25, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.25, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.125, 0.0),
];
const PAINTING_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.25, 0.0),
];
const PALE_OAK_BOAT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.28125, 0.0),
];
const PALE_OAK_CHEST_BOAT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.28125, 0.0),
];
const PANDA_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.25, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.25, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.625, 0.0),
];
const PARCHED_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.9900000095367432, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.9900000095367432, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.699999988079071, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.9950000047683716, 0.0),
];
const PARROT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.8999999761581421, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.4625000059604645, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.44999998807907104, 0.0),
];
const PHANTOM_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.3375000059604645, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.125, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.25, 0.0),
];
const PIG_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.8999999761581421, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.8687499761581421, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.44999998807907104, 0.0),
];
const PIGLIN_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.950000047683716, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 2.012500047683716, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.699999988079071, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.975000023841858, 0.0),
];
const PIGLIN_BRUTE_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.950000047683716, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 2.012500047683716, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.699999988079071, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.975000023841858, 0.0),
];
const PILLAGER_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.950000047683716, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 2.0, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.6000000238418579, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.975000023841858, 0.0),
];
const POLAR_BEAR_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.399999976158142, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.399999976158142, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.699999988079071, 0.0),
];
const SPLASH_POTION_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.25, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.25, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.125, 0.0),
];
const LINGERING_POTION_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.25, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.25, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.125, 0.0),
];
const PUFFERFISH_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.699999988079071, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.699999988079071, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.3499999940395355, 0.0),
];
const RABBIT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.25, 0.0),
];
const RAVAGER_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 2.200000047683716, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0010905091185122728, 2.2625, -0.06249038130044937),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 1.100000023841858, 0.0),
];
const SALMON_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.4000000059604645, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.4000000059604645, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.20000000298023224, 0.0),
];
const SHEEP_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.2999999523162842, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.2374999523162842, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.6499999761581421, 0.0),
];
const SHULKER_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.0, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.0, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.5, 0.0),
];
const SHULKER_BULLET_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.3125, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.3125, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.15625, 0.0),
];
const SILVERFISH_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.30000001192092896, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.23749999701976776, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.15000000596046448, 0.0),
];
const SKELETON_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.9900000095367432, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.9900000095367432, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.699999988079071, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.9950000047683716, 0.0),
];
const SKELETON_HORSE_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.600000023841858, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.318750023841858, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.800000011920929, 0.0),
];
const SLIME_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5199999809265137, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5199999809265137, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.25999999046325684, 0.0),
];
const SMALL_FIREBALL_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.3125, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.3125, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.15625, 0.0),
];
const SNIFFER_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 2.049999952316284, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 2.09375, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.875, 0.0),
];
const SNOWBALL_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.25, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.25, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.125, 0.0),
];
const SNOW_GOLEM_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.899999976158142, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.899999976158142, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.949999988079071, 0.0),
];
const SPAWNER_MINECART_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.699999988079071, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.1875, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.3499999940395355, 0.0),
];
const SPECTRAL_ARROW_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.25, 0.0),
];
const SPIDER_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.8999999761581421, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.7649999856948853, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.44999998807907104, 0.0),
];
const SPRUCE_BOAT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.28125, 0.0),
];
const SPRUCE_CHEST_BOAT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5625, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.28125, 0.0),
];
const SQUID_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.800000011920929, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.800000011920929, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.4000000059604645, 0.0),
];
const STRAY_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.9900000095367432, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.9900000095367432, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.699999988079071, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.9950000047683716, 0.0),
];
const STRIDER_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.7000000476837158, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.7000000476837158, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.8500000238418579, 0.0),
];
const TADPOLE_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.30000001192092896, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.30000001192092896, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.15000000596046448, 0.0),
];
const TEXT_DISPLAY_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.0, 0.0),
];
const TNT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.9800000190734864, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.9800000190734864, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.4900000095367432, 0.0),
];
const TNT_MINECART_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.699999988079071, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.1875, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.3499999940395355, 0.0),
];
const TRADER_LLAMA_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.8700000047683716, 0.0),
    EntityAttachmentOffset::new("passenger", 0.00523444376885891, 1.37, -0.299953830242157),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.9350000023841858, 0.0),
];
const TRIDENT_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.5, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.5, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.25, 0.0),
];
const TROPICAL_FISH_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.4000000059604645, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.4000000059604645, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.20000000298023224, 0.0),
];
const TURTLE_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.4000000059604645, 0.0),
    EntityAttachmentOffset::new("passenger", 0.004362036474049091, 0.55625, -0.24996152520179749),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.20000000298023224, 0.0),
];
const VEX_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.800000011920929, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.737500011920929, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, -0.03999999910593033, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.4000000059604645, 0.0),
];
const VILLAGER_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.950000047683716, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.950000047683716, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.975000023841858, 0.0),
];
const VINDICATOR_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.950000047683716, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 2.0, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.6000000238418579, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.975000023841858, 0.0),
];
const WANDERING_TRADER_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.950000047683716, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.950000047683716, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.975000023841858, 0.0),
];
const WARDEN_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 2.9000000953674316, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 3.1500000953674316, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 1.600000023841858, 0.0),
];
const WIND_CHARGE_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.3125, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.3125, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.15625, 0.0),
];
const WITCH_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.950000047683716, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 2.262500047683716, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.975000023841858, 0.0),
];
const WITHER_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 3.5, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 3.5, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 1.75, 0.0),
];
const WITHER_SKELETON_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 2.4000000953674316, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 2.4000000953674316, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.875, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 1.2000000476837158, 0.0),
];
const WITHER_SKULL_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.3125, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.3125, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.15625, 0.0),
];
const WOLF_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.8500000238418579, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0010905091185122728, 0.81875, -0.06249038130044937),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.42500001192092896, 0.0),
];
const ZOGLIN_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.399999976158142, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.493749976158142, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.699999988079071, 0.0),
];
const ZOMBIE_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.950000047683716, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 2.012500047683716, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.699999988079071, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.975000023841858, 0.0),
];
const ZOMBIE_HORSE_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.600000023841858, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.318750023841858, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.800000011920929, 0.0),
];
const ZOMBIE_NAUTILUS_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.949999988079071, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.1375000476837158, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.4749999940395355, 0.0),
];
const ZOMBIE_VILLAGER_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.950000047683716, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 2.125, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.699999988079071, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.975000023841858, 0.0),
];
const ZOMBIFIED_PIGLIN_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.950000047683716, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 2.0, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.699999988079071, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.975000023841858, 0.0),
];
const PLAYER_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 1.7999999523162842, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 1.7999999523162842, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.6, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.8999999761581421, 0.0),
];
const FISHING_BOBBER_ATTACHMENTS: &'static [EntityAttachmentOffset] = &[
    EntityAttachmentOffset::new("name_tag", 0.0, 0.25, 0.0),
    EntityAttachmentOffset::new("passenger", 0.0, 0.25, 0.0),
    EntityAttachmentOffset::new("vehicle", 0.0, 0.0, 0.0),
    EntityAttachmentOffset::new("warden_chest", 0.0, 0.125, 0.0),
];
const ACACIA_BOAT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const ACACIA_CHEST_BOAT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const ALLAY_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 2.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FLYING_SPEED, 0.10000000149011612),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 20.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.10000000149011612),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const AREA_EFFECT_CLOUD_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const ARMADILLO_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 12.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.14),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const ARMOR_STAND_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 20.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.7),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const ARROW_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const AXOLOTL_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 2.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 14.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 1.0),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 1.0),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const BAMBOO_CHEST_RAFT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const BAMBOO_RAFT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const BAT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 6.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.7),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const BEE_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 2.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FLYING_SPEED, 0.6000000238418579),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 10.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.30000001192092896),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const BIRCH_BOAT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const BIRCH_CHEST_BOAT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const BLAZE_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 6.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 48.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 20.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.2300000041723251),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const BLOCK_DISPLAY_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const BOGGED_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 2.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 16.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.25),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const BREEZE_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 3.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 24.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 30.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.6299999952316284),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const BREEZE_WIND_CHARGE_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const CAMEL_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 0.5),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 32.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.09000000357627869),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 6.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 1.5),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const CAMEL_HUSK_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 0.5),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 32.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.09000000357627869),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 6.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 1.5),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const CAT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 3.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 10.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.30000001192092896),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const CAVE_SPIDER_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 2.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 12.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.30000001192092896),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const CHERRY_BOAT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const CHERRY_CHEST_BOAT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const CHEST_MINECART_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const CHICKEN_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 4.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.25),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const COD_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 3.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.7),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const COPPER_GOLEM_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 12.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.20000000298023224),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 1.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const COMMAND_BLOCK_MINECART_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const COW_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 10.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.20000000298023224),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const CREAKING_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 3.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 32.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 1.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.4000000059604645),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 1.0625),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const CREEPER_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 2.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 20.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.25),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const DARK_OAK_BOAT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const DARK_OAK_CHEST_BOAT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const DOLPHIN_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 3.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 10.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 1.2000000476837158),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const DONKEY_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 0.5),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.5),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 53.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.17499999701976776),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 6.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 1.0),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const DRAGON_FIREBALL_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const DROWNED_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 2.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 3.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 35.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 20.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.2300000041723251),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::SPAWN_REINFORCEMENTS, 0.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 1.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const EGG_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const ELDER_GUARDIAN_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 8.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 80.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.30000001192092896),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const ENDERMAN_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 7.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 64.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 40.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.30000001192092896),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 1.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const ENDERMITE_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 2.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 8.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.25),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const ENDER_DRAGON_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 16.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 200.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.7),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const ENDER_PEARL_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const END_CRYSTAL_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const EVOKER_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 2.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 12.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 24.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.5),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const EVOKER_FANGS_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const EXPERIENCE_BOTTLE_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const EXPERIENCE_ORB_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const EYE_OF_ENDER_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const FALLING_BLOCK_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const FIREBALL_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const FIREWORK_ROCKET_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const FOX_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 2.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 32.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 10.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.30000001192092896),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 5.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const FROG_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 10.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 10.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 1.0),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 1.0),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const FURNACE_MINECART_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const GHAST_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 8.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FLYING_SPEED, 0.06),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 100.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 10.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.7),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const HAPPY_GHAST_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 8.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FLYING_SPEED, 0.05),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 20.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.05),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const GIANT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 50.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 16.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 100.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.5),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const GLOW_ITEM_FRAME_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const GLOW_SQUID_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 10.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.7),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const GOAT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 2.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 10.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.20000000298023224),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const GUARDIAN_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 6.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 30.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.5),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const HOGLIN_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 6.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 1.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.6000000238418579),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 40.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.30000001192092896),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const HOPPER_MINECART_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const HORSE_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 0.5),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.7),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 53.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.22499999403953552),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 6.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 1.0),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const HUSK_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 2.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 3.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 35.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 20.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.2300000041723251),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::SPAWN_REINFORCEMENTS, 0.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const ILLUSIONER_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 2.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 18.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 32.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.5),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const INTERACTION_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const IRON_GOLEM_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 15.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 1.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 100.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.25),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 1.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const ITEM_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const ITEM_DISPLAY_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const ITEM_FRAME_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const JUNGLE_BOAT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const JUNGLE_CHEST_BOAT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const LEASH_KNOT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const LIGHTNING_BOLT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const LLAMA_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 0.5),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.5),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 53.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.17499999701976776),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 6.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 1.0),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const LLAMA_SPIT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const MAGMA_CUBE_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 2.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 20.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.20000000298023224),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const MANGROVE_BOAT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const MANGROVE_CHEST_BOAT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const MANNEQUIN_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 20.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.7),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const MARKER_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const MINECART_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const MOOSHROOM_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 10.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.20000000298023224),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const MULE_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 0.5),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.5),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 53.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.17499999701976776),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 6.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 1.0),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const NAUTILUS_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 3.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.30000001192092896),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 15.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 1.0),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const OAK_BOAT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const OAK_CHEST_BOAT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const OCELOT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 3.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 10.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.30000001192092896),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const OMINOUS_ITEM_SPAWNER_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const PAINTING_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const PALE_OAK_BOAT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const PALE_OAK_CHEST_BOAT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const PANDA_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 6.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 20.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.15000000596046448),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const PARCHED_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 2.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 16.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.25),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const PARROT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 3.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FLYING_SPEED, 0.4000000059604645),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 6.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.20000000298023224),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const PHANTOM_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 2.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 20.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.7),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const PIG_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 10.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.25),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const PIGLIN_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 5.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 16.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.3499999940395355),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const PIGLIN_BRUTE_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 7.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 12.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 50.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.3499999940395355),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const PILLAGER_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 5.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 32.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 24.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.3499999940395355),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const POLAR_BEAR_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 6.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 20.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 30.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.25),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const SPLASH_POTION_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const LINGERING_POTION_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const PUFFERFISH_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 3.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.7),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const RABBIT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 3.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 3.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.30000001192092896),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const RAVAGER_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 12.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 1.5),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 32.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.75),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 100.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.3),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 1.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const SALMON_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 3.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.7),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const SHEEP_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 8.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.2300000041723251),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const SHULKER_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 30.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.7),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const SHULKER_BULLET_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const SILVERFISH_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 1.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 8.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.25),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const SKELETON_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 2.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 20.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.25),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const SKELETON_HORSE_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 0.5),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.7),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 15.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.20000000298023224),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 6.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 1.0),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const SLIME_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 2.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 20.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.7),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const SMALL_FIREBALL_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const SNIFFER_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 14.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.10000000149011612),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const SNOWBALL_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const SNOW_GOLEM_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 4.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.20000000298023224),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const SPAWNER_MINECART_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const SPECTRAL_ARROW_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const SPIDER_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 2.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 16.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.30000001192092896),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const SPRUCE_BOAT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const SPRUCE_CHEST_BOAT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const SQUID_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 10.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.7),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const STRAY_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 2.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 20.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.25),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const STRIDER_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 20.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.17499999701976776),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const TADPOLE_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 6.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 1.0),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const TEXT_DISPLAY_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const TNT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const TNT_MINECART_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const TRADER_LLAMA_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 0.5),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.5),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 53.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.17499999701976776),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 6.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 1.0),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const TRIDENT_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const TROPICAL_FISH_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 3.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.7),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const TURTLE_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 30.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.25),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 1.0),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const VEX_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 4.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 14.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.7),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const VILLAGER_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 20.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.5),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const VINDICATOR_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 5.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 12.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 24.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.3499999940395355),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const WANDERING_TRADER_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 20.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.7),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const WARDEN_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 30.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 1.5),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 24.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 1.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 500.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.30000001192092896),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const WIND_CHARGE_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const WITCH_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 2.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 26.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.25),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const WITHER_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 4.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 2.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FLYING_SPEED, 0.6000000238418579),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 40.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 300.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.6000000238418579),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const WITHER_SKELETON_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 2.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 20.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.25),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const WITHER_SKULL_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
const WOLF_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 4.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 8.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.30000001192092896),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const ZOGLIN_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 6.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 1.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.6000000238418579),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 40.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.30000001192092896),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const ZOMBIE_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 2.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 3.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 35.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 20.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.2300000041723251),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::SPAWN_REINFORCEMENTS, 0.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const ZOMBIE_HORSE_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 0.5),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.7),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 25.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.22499999403953552),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 6.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 1.0),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const ZOMBIE_NAUTILUS_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 3.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 16.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.30000001192092896),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 15.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 1.100000023841858),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::TEMPT_RANGE, 10.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const ZOMBIE_VILLAGER_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 2.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 3.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 35.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 20.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.2300000041723251),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::SPAWN_REINFORCEMENTS, 0.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const ZOMBIFIED_PIGLIN_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 2.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 5.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::FOLLOW_RANGE, 35.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 20.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.2300000041723251),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::SPAWN_REINFORCEMENTS, 0.0),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 0.0),
];
const PLAYER_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
    EntityDefaultAttribute::new(Attribute::ARMOR, 0.0),
    EntityDefaultAttribute::new(Attribute::ARMOR_TOUGHNESS, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_DAMAGE, 1.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_KNOCKBACK, 0.0),
    EntityDefaultAttribute::new(Attribute::ATTACK_SPEED, 4.0),
    EntityDefaultAttribute::new(Attribute::BLOCK_BREAK_SPEED, 1.0),
    EntityDefaultAttribute::new(Attribute::BLOCK_INTERACTION_RANGE, 4.5),
    EntityDefaultAttribute::new(Attribute::BURNING_TIME, 1.0),
    EntityDefaultAttribute::new(Attribute::CAMERA_DISTANCE, 4.0),
    EntityDefaultAttribute::new(Attribute::ENTITY_INTERACTION_RANGE, 3.0),
    EntityDefaultAttribute::new(Attribute::EXPLOSION_KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::FALL_DAMAGE_MULTIPLIER, 1.0),
    EntityDefaultAttribute::new(Attribute::GRAVITY, 0.08),
    EntityDefaultAttribute::new(Attribute::JUMP_STRENGTH, 0.41999998688697815),
    EntityDefaultAttribute::new(Attribute::KNOCKBACK_RESISTANCE, 0.0),
    EntityDefaultAttribute::new(Attribute::LUCK, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_ABSORPTION, 0.0),
    EntityDefaultAttribute::new(Attribute::MAX_HEALTH, 20.0),
    EntityDefaultAttribute::new(Attribute::MINING_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::MOVEMENT_SPEED, 0.10000000149011612),
    EntityDefaultAttribute::new(Attribute::OXYGEN_BONUS, 0.0),
    EntityDefaultAttribute::new(Attribute::SAFE_FALL_DISTANCE, 3.0),
    EntityDefaultAttribute::new(Attribute::SCALE, 1.0),
    EntityDefaultAttribute::new(Attribute::SNEAKING_SPEED, 0.3),
    EntityDefaultAttribute::new(Attribute::STEP_HEIGHT, 0.6),
    EntityDefaultAttribute::new(Attribute::SUBMERGED_MINING_SPEED, 0.2),
    EntityDefaultAttribute::new(Attribute::SWEEPING_DAMAGE_RATIO, 0.0),
    EntityDefaultAttribute::new(Attribute::WATER_MOVEMENT_EFFICIENCY, 0.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_RECEIVE_RANGE, 60000000.0),
    EntityDefaultAttribute::new(Attribute::WAYPOINT_TRANSMIT_RANGE, 60000000.0),
];
const FISHING_BOBBER_DEFAULT_ATTRIBUTES: &'static [EntityDefaultAttribute] = &[
];
impl EntityType {
    pub const ACACIA_BOAT: Self = Self::new(0, "acacia_boat", "entity.minecraft.acacia_boat", EntityPacketType::Entity, 1.375, 0.5625, 0.5625, 10, false);
    pub const ACACIA_CHEST_BOAT: Self = Self::new(1, "acacia_chest_boat", "entity.minecraft.acacia_chest_boat", EntityPacketType::Entity, 1.375, 0.5625, 0.5625, 10, false);
    pub const ALLAY: Self = Self::new(2, "allay", "entity.minecraft.allay", EntityPacketType::Living, 0.35, 0.6, 0.36, 8, false);
    pub const AREA_EFFECT_CLOUD: Self = Self::new(3, "area_effect_cloud", "entity.minecraft.area_effect_cloud", EntityPacketType::Entity, 6.0, 0.5, 0.425, 10, true);
    pub const ARMADILLO: Self = Self::new(4, "armadillo", "entity.minecraft.armadillo", EntityPacketType::Living, 0.7, 0.65, 0.26, 10, false);
    pub const ARMOR_STAND: Self = Self::new(5, "armor_stand", "entity.minecraft.armor_stand", EntityPacketType::Living, 0.5, 1.975, 1.7775, 10, false);
    pub const ARROW: Self = Self::new(6, "arrow", "entity.minecraft.arrow", EntityPacketType::Entity, 0.5, 0.5, 0.13, 4, false);
    pub const AXOLOTL: Self = Self::new(7, "axolotl", "entity.minecraft.axolotl", EntityPacketType::Living, 0.75, 0.42, 0.2751, 10, false);
    pub const BAMBOO_CHEST_RAFT: Self = Self::new(8, "bamboo_chest_raft", "entity.minecraft.bamboo_chest_raft", EntityPacketType::Entity, 1.375, 0.5625, 0.5625, 10, false);
    pub const BAMBOO_RAFT: Self = Self::new(9, "bamboo_raft", "entity.minecraft.bamboo_raft", EntityPacketType::Entity, 1.375, 0.5625, 0.5625, 10, false);
    pub const BAT: Self = Self::new(10, "bat", "entity.minecraft.bat", EntityPacketType::Living, 0.5, 0.9, 0.45, 5, false);
    pub const BEE: Self = Self::new(11, "bee", "entity.minecraft.bee", EntityPacketType::Living, 0.7, 0.6, 0.3, 8, false);
    pub const BIRCH_BOAT: Self = Self::new(12, "birch_boat", "entity.minecraft.birch_boat", EntityPacketType::Entity, 1.375, 0.5625, 0.5625, 10, false);
    pub const BIRCH_CHEST_BOAT: Self = Self::new(13, "birch_chest_boat", "entity.minecraft.birch_chest_boat", EntityPacketType::Entity, 1.375, 0.5625, 0.5625, 10, false);
    pub const BLAZE: Self = Self::new(14, "blaze", "entity.minecraft.blaze", EntityPacketType::Living, 0.6, 1.8, 1.53, 8, true);
    pub const BLOCK_DISPLAY: Self = Self::new(15, "block_display", "entity.minecraft.block_display", EntityPacketType::Entity, 0.0, 0.0, 0.0, 10, false);
    pub const BOGGED: Self = Self::new(16, "bogged", "entity.minecraft.bogged", EntityPacketType::Living, 0.6, 1.99, 1.74, 8, false);
    pub const BREEZE: Self = Self::new(17, "breeze", "entity.minecraft.breeze", EntityPacketType::Living, 0.6, 1.77, 1.3452, 10, false);
    pub const BREEZE_WIND_CHARGE: Self = Self::new(18, "breeze_wind_charge", "entity.minecraft.breeze_wind_charge", EntityPacketType::Entity, 0.3125, 0.3125, 0.0, 4, false);
    pub const CAMEL: Self = Self::new(19, "camel", "entity.minecraft.camel", EntityPacketType::Living, 1.7, 2.375, 2.275, 10, false);
    pub const CAMEL_HUSK: Self = Self::new(20, "camel_husk", "entity.minecraft.camel_husk", EntityPacketType::Living, 1.7, 2.375, 2.275, 10, false);
    pub const CAT: Self = Self::new(21, "cat", "entity.minecraft.cat", EntityPacketType::Living, 0.6, 0.7, 0.35, 8, false);
    pub const CAVE_SPIDER: Self = Self::new(22, "cave_spider", "entity.minecraft.cave_spider", EntityPacketType::Living, 0.7, 0.5, 0.45, 8, false);
    pub const CHERRY_BOAT: Self = Self::new(23, "cherry_boat", "entity.minecraft.cherry_boat", EntityPacketType::Entity, 1.375, 0.5625, 0.5625, 10, false);
    pub const CHERRY_CHEST_BOAT: Self = Self::new(24, "cherry_chest_boat", "entity.minecraft.cherry_chest_boat", EntityPacketType::Entity, 1.375, 0.5625, 0.5625, 10, false);
    pub const CHEST_MINECART: Self = Self::new(25, "chest_minecart", "entity.minecraft.chest_minecart", EntityPacketType::Entity, 0.98, 0.7, 0.595, 8, false);
    pub const CHICKEN: Self = Self::new(26, "chicken", "entity.minecraft.chicken", EntityPacketType::Living, 0.4, 0.7, 0.644, 10, false);
    pub const COD: Self = Self::new(27, "cod", "entity.minecraft.cod", EntityPacketType::Living, 0.5, 0.3, 0.195, 4, false);
    pub const COPPER_GOLEM: Self = Self::new(28, "copper_golem", "entity.minecraft.copper_golem", EntityPacketType::Living, 0.49, 0.98, 0.8125, 10, false);
    pub const COMMAND_BLOCK_MINECART: Self = Self::new(29, "command_block_minecart", "entity.minecraft.command_block_minecart", EntityPacketType::Entity, 0.98, 0.7, 0.595, 8, false);
    pub const COW: Self = Self::new(30, "cow", "entity.minecraft.cow", EntityPacketType::Living, 0.9, 1.4, 1.3, 10, false);
    pub const CREAKING: Self = Self::new(31, "creaking", "entity.minecraft.creaking", EntityPacketType::Living, 0.9, 2.7, 2.3, 8, false);
    pub const CREEPER: Self = Self::new(32, "creeper", "entity.minecraft.creeper", EntityPacketType::Living, 0.6, 1.7, 1.445, 8, false);
    pub const DARK_OAK_BOAT: Self = Self::new(33, "dark_oak_boat", "entity.minecraft.dark_oak_boat", EntityPacketType::Entity, 1.375, 0.5625, 0.5625, 10, false);
    pub const DARK_OAK_CHEST_BOAT: Self = Self::new(34, "dark_oak_chest_boat", "entity.minecraft.dark_oak_chest_boat", EntityPacketType::Entity, 1.375, 0.5625, 0.5625, 10, false);
    pub const DOLPHIN: Self = Self::new(35, "dolphin", "entity.minecraft.dolphin", EntityPacketType::Living, 0.9, 0.6, 0.3, 5, false);
    pub const DONKEY: Self = Self::new(36, "donkey", "entity.minecraft.donkey", EntityPacketType::Living, 1.3964844, 1.5, 1.425, 10, false);
    pub const DRAGON_FIREBALL: Self = Self::new(37, "dragon_fireball", "entity.minecraft.dragon_fireball", EntityPacketType::Entity, 1.0, 1.0, 0.85, 4, false);
    pub const DROWNED: Self = Self::new(38, "drowned", "entity.minecraft.drowned", EntityPacketType::Living, 0.6, 1.95, 1.74, 8, false);
    pub const EGG: Self = Self::new(39, "egg", "entity.minecraft.egg", EntityPacketType::Entity, 0.25, 0.25, 0.2125, 4, false);
    pub const ELDER_GUARDIAN: Self = Self::new(40, "elder_guardian", "entity.minecraft.elder_guardian", EntityPacketType::Living, 1.9975, 1.9975, 0.99875, 10, false);
    pub const ENDERMAN: Self = Self::new(41, "enderman", "entity.minecraft.enderman", EntityPacketType::Living, 0.6, 2.9, 2.55, 8, false);
    pub const ENDERMITE: Self = Self::new(42, "endermite", "entity.minecraft.endermite", EntityPacketType::Living, 0.4, 0.3, 0.13, 8, false);
    pub const ENDER_DRAGON: Self = Self::new(43, "ender_dragon", "entity.minecraft.ender_dragon", EntityPacketType::Living, 16.0, 8.0, 6.8, 10, true);
    pub const ENDER_PEARL: Self = Self::new(44, "ender_pearl", "entity.minecraft.ender_pearl", EntityPacketType::Entity, 0.25, 0.25, 0.2125, 4, false);
    pub const END_CRYSTAL: Self = Self::new(45, "end_crystal", "entity.minecraft.end_crystal", EntityPacketType::Entity, 2.0, 2.0, 1.7, 16, true);
    pub const EVOKER: Self = Self::new(46, "evoker", "entity.minecraft.evoker", EntityPacketType::Living, 0.6, 1.95, 1.6575, 8, false);
    pub const EVOKER_FANGS: Self = Self::new(47, "evoker_fangs", "entity.minecraft.evoker_fangs", EntityPacketType::Entity, 0.5, 0.8, 0.68, 6, false);
    pub const EXPERIENCE_BOTTLE: Self = Self::new(48, "experience_bottle", "entity.minecraft.experience_bottle", EntityPacketType::Entity, 0.25, 0.25, 0.2125, 4, false);
    pub const EXPERIENCE_ORB: Self = Self::new(49, "experience_orb", "entity.minecraft.experience_orb", EntityPacketType::Entity, 0.5, 0.5, 0.425, 6, false);
    pub const EYE_OF_ENDER: Self = Self::new(50, "eye_of_ender", "entity.minecraft.eye_of_ender", EntityPacketType::Entity, 0.25, 0.25, 0.2125, 4, false);
    pub const FALLING_BLOCK: Self = Self::new(51, "falling_block", "entity.minecraft.falling_block", EntityPacketType::Entity, 0.98, 0.98, 0.83300006, 10, false);
    pub const FIREBALL: Self = Self::new(52, "fireball", "entity.minecraft.fireball", EntityPacketType::Entity, 1.0, 1.0, 0.85, 4, false);
    pub const FIREWORK_ROCKET: Self = Self::new(53, "firework_rocket", "entity.minecraft.firework_rocket", EntityPacketType::Entity, 0.25, 0.25, 0.2125, 4, false);
    pub const FOX: Self = Self::new(54, "fox", "entity.minecraft.fox", EntityPacketType::Living, 0.6, 0.7, 0.4, 8, false);
    pub const FROG: Self = Self::new(55, "frog", "entity.minecraft.frog", EntityPacketType::Living, 0.5, 0.5, 0.425, 10, false);
    pub const FURNACE_MINECART: Self = Self::new(56, "furnace_minecart", "entity.minecraft.furnace_minecart", EntityPacketType::Entity, 0.98, 0.7, 0.595, 8, false);
    pub const GHAST: Self = Self::new(57, "ghast", "entity.minecraft.ghast", EntityPacketType::Living, 4.0, 4.0, 2.6, 10, true);
    pub const HAPPY_GHAST: Self = Self::new(58, "happy_ghast", "entity.minecraft.happy_ghast", EntityPacketType::Living, 4.0, 4.0, 2.6, 10, false);
    pub const GIANT: Self = Self::new(59, "giant", "entity.minecraft.giant", EntityPacketType::Living, 3.6, 12.0, 10.44, 10, false);
    pub const GLOW_ITEM_FRAME: Self = Self::new(60, "glow_item_frame", "entity.minecraft.glow_item_frame", EntityPacketType::Entity, 0.5, 0.5, 0.0, 10, false);
    pub const GLOW_SQUID: Self = Self::new(61, "glow_squid", "entity.minecraft.glow_squid", EntityPacketType::Living, 0.8, 0.8, 0.4, 10, false);
    pub const GOAT: Self = Self::new(62, "goat", "entity.minecraft.goat", EntityPacketType::Living, 0.9, 1.3, 1.105, 10, false);
    pub const GUARDIAN: Self = Self::new(63, "guardian", "entity.minecraft.guardian", EntityPacketType::Living, 0.85, 0.85, 0.425, 8, false);
    pub const HOGLIN: Self = Self::new(64, "hoglin", "entity.minecraft.hoglin", EntityPacketType::Living, 1.3964844, 1.4, 1.19, 8, false);
    pub const HOPPER_MINECART: Self = Self::new(65, "hopper_minecart", "entity.minecraft.hopper_minecart", EntityPacketType::Entity, 0.98, 0.7, 0.595, 8, false);
    pub const HORSE: Self = Self::new(66, "horse", "entity.minecraft.horse", EntityPacketType::Living, 1.3964844, 1.6, 1.52, 10, false);
    pub const HUSK: Self = Self::new(67, "husk", "entity.minecraft.husk", EntityPacketType::Living, 0.6, 1.95, 1.74, 8, false);
    pub const ILLUSIONER: Self = Self::new(68, "illusioner", "entity.minecraft.illusioner", EntityPacketType::Living, 0.6, 1.95, 1.6575, 8, false);
    pub const INTERACTION: Self = Self::new(69, "interaction", "entity.minecraft.interaction", EntityPacketType::Entity, 0.0, 0.0, 0.0, 10, false);
    pub const IRON_GOLEM: Self = Self::new(70, "iron_golem", "entity.minecraft.iron_golem", EntityPacketType::Living, 1.4, 2.7, 2.295, 10, false);
    pub const ITEM: Self = Self::new(71, "item", "entity.minecraft.item", EntityPacketType::Entity, 0.25, 0.25, 0.2125, 6, false);
    pub const ITEM_DISPLAY: Self = Self::new(72, "item_display", "entity.minecraft.item_display", EntityPacketType::Entity, 0.0, 0.0, 0.0, 10, false);
    pub const ITEM_FRAME: Self = Self::new(73, "item_frame", "entity.minecraft.item_frame", EntityPacketType::Entity, 0.5, 0.5, 0.0, 10, false);
    pub const JUNGLE_BOAT: Self = Self::new(74, "jungle_boat", "entity.minecraft.jungle_boat", EntityPacketType::Entity, 1.375, 0.5625, 0.5625, 10, false);
    pub const JUNGLE_CHEST_BOAT: Self = Self::new(75, "jungle_chest_boat", "entity.minecraft.jungle_chest_boat", EntityPacketType::Entity, 1.375, 0.5625, 0.5625, 10, false);
    pub const LEASH_KNOT: Self = Self::new(76, "leash_knot", "entity.minecraft.leash_knot", EntityPacketType::Entity, 0.375, 0.5, 0.0625, 10, false);
    pub const LIGHTNING_BOLT: Self = Self::new(77, "lightning_bolt", "entity.minecraft.lightning_bolt", EntityPacketType::Entity, 0.0, 0.0, 0.0, 16, false);
    pub const LLAMA: Self = Self::new(78, "llama", "entity.minecraft.llama", EntityPacketType::Living, 0.9, 1.87, 1.7765, 10, false);
    pub const LLAMA_SPIT: Self = Self::new(79, "llama_spit", "entity.minecraft.llama_spit", EntityPacketType::Entity, 0.25, 0.25, 0.2125, 4, false);
    pub const MAGMA_CUBE: Self = Self::new(80, "magma_cube", "entity.minecraft.magma_cube", EntityPacketType::Living, 0.52, 0.52, 0.325, 8, true);
    pub const MANGROVE_BOAT: Self = Self::new(81, "mangrove_boat", "entity.minecraft.mangrove_boat", EntityPacketType::Entity, 1.375, 0.5625, 0.5625, 10, false);
    pub const MANGROVE_CHEST_BOAT: Self = Self::new(82, "mangrove_chest_boat", "entity.minecraft.mangrove_chest_boat", EntityPacketType::Entity, 1.375, 0.5625, 0.5625, 10, false);
    pub const MANNEQUIN: Self = Self::new(83, "mannequin", "entity.minecraft.mannequin", EntityPacketType::Living, 0.6, 1.8, 1.62, 32, false);
    pub const MARKER: Self = Self::new(84, "marker", "entity.minecraft.marker", EntityPacketType::Entity, 0.0, 0.0, 0.0, 0, false);
    pub const MINECART: Self = Self::new(85, "minecart", "entity.minecraft.minecart", EntityPacketType::Entity, 0.98, 0.7, 0.595, 8, false);
    pub const MOOSHROOM: Self = Self::new(86, "mooshroom", "entity.minecraft.mooshroom", EntityPacketType::Living, 0.9, 1.4, 1.3, 10, false);
    pub const MULE: Self = Self::new(87, "mule", "entity.minecraft.mule", EntityPacketType::Living, 1.3964844, 1.6, 1.52, 8, false);
    pub const NAUTILUS: Self = Self::new(88, "nautilus", "entity.minecraft.nautilus", EntityPacketType::Living, 0.875, 0.95, 0.2751, 10, false);
    pub const OAK_BOAT: Self = Self::new(89, "oak_boat", "entity.minecraft.oak_boat", EntityPacketType::Entity, 1.375, 0.5625, 0.5625, 10, false);
    pub const OAK_CHEST_BOAT: Self = Self::new(90, "oak_chest_boat", "entity.minecraft.oak_chest_boat", EntityPacketType::Entity, 1.375, 0.5625, 0.5625, 10, false);
    pub const OCELOT: Self = Self::new(91, "ocelot", "entity.minecraft.ocelot", EntityPacketType::Living, 0.6, 0.7, 0.595, 10, false);
    pub const OMINOUS_ITEM_SPAWNER: Self = Self::new(92, "ominous_item_spawner", "entity.minecraft.ominous_item_spawner", EntityPacketType::Entity, 0.25, 0.25, 0.2125, 8, false);
    pub const PAINTING: Self = Self::new(93, "painting", "entity.minecraft.painting", EntityPacketType::Entity, 0.5, 0.5, 0.425, 10, false);
    pub const PALE_OAK_BOAT: Self = Self::new(94, "pale_oak_boat", "entity.minecraft.pale_oak_boat", EntityPacketType::Entity, 1.375, 0.5625, 0.5625, 10, false);
    pub const PALE_OAK_CHEST_BOAT: Self = Self::new(95, "pale_oak_chest_boat", "entity.minecraft.pale_oak_chest_boat", EntityPacketType::Entity, 1.375, 0.5625, 0.5625, 10, false);
    pub const PANDA: Self = Self::new(96, "panda", "entity.minecraft.panda", EntityPacketType::Living, 1.3, 1.25, 1.0625, 10, false);
    pub const PARCHED: Self = Self::new(97, "parched", "entity.minecraft.parched", EntityPacketType::Living, 0.6, 1.99, 1.74, 8, false);
    pub const PARROT: Self = Self::new(98, "parrot", "entity.minecraft.parrot", EntityPacketType::Living, 0.5, 0.9, 0.54, 8, false);
    pub const PHANTOM: Self = Self::new(99, "phantom", "entity.minecraft.phantom", EntityPacketType::Living, 0.9, 0.5, 0.175, 8, false);
    pub const PIG: Self = Self::new(100, "pig", "entity.minecraft.pig", EntityPacketType::Living, 0.9, 0.9, 0.765, 10, false);
    pub const PIGLIN: Self = Self::new(101, "piglin", "entity.minecraft.piglin", EntityPacketType::Living, 0.6, 1.95, 1.79, 8, false);
    pub const PIGLIN_BRUTE: Self = Self::new(102, "piglin_brute", "entity.minecraft.piglin_brute", EntityPacketType::Living, 0.6, 1.95, 1.79, 8, false);
    pub const PILLAGER: Self = Self::new(103, "pillager", "entity.minecraft.pillager", EntityPacketType::Living, 0.6, 1.95, 1.6575, 8, false);
    pub const POLAR_BEAR: Self = Self::new(104, "polar_bear", "entity.minecraft.polar_bear", EntityPacketType::Living, 1.4, 1.4, 1.19, 10, false);
    pub const SPLASH_POTION: Self = Self::new(105, "splash_potion", "entity.minecraft.splash_potion", EntityPacketType::Entity, 0.25, 0.25, 0.2125, 4, false);
    pub const LINGERING_POTION: Self = Self::new(106, "lingering_potion", "entity.minecraft.lingering_potion", EntityPacketType::Entity, 0.25, 0.25, 0.2125, 4, false);
    pub const PUFFERFISH: Self = Self::new(107, "pufferfish", "entity.minecraft.pufferfish", EntityPacketType::Living, 0.7, 0.7, 0.455, 4, false);
    pub const RABBIT: Self = Self::new(108, "rabbit", "entity.minecraft.rabbit", EntityPacketType::Living, 0.4, 0.5, 0.425, 8, false);
    pub const RAVAGER: Self = Self::new(109, "ravager", "entity.minecraft.ravager", EntityPacketType::Living, 1.95, 2.2, 1.8700001, 10, false);
    pub const SALMON: Self = Self::new(110, "salmon", "entity.minecraft.salmon", EntityPacketType::Living, 0.7, 0.4, 0.26, 4, false);
    pub const SHEEP: Self = Self::new(111, "sheep", "entity.minecraft.sheep", EntityPacketType::Living, 0.9, 1.3, 1.235, 10, false);
    pub const SHULKER: Self = Self::new(112, "shulker", "entity.minecraft.shulker", EntityPacketType::Living, 1.0, 1.0, 0.5, 10, true);
    pub const SHULKER_BULLET: Self = Self::new(113, "shulker_bullet", "entity.minecraft.shulker_bullet", EntityPacketType::Entity, 0.3125, 0.3125, 0.265625, 8, false);
    pub const SILVERFISH: Self = Self::new(114, "silverfish", "entity.minecraft.silverfish", EntityPacketType::Living, 0.4, 0.3, 0.13, 8, false);
    pub const SKELETON: Self = Self::new(115, "skeleton", "entity.minecraft.skeleton", EntityPacketType::Living, 0.6, 1.99, 1.74, 8, false);
    pub const SKELETON_HORSE: Self = Self::new(116, "skeleton_horse", "entity.minecraft.skeleton_horse", EntityPacketType::Living, 1.3964844, 1.6, 1.52, 10, false);
    pub const SLIME: Self = Self::new(117, "slime", "entity.minecraft.slime", EntityPacketType::Living, 0.52, 0.52, 0.325, 10, false);
    pub const SMALL_FIREBALL: Self = Self::new(118, "small_fireball", "entity.minecraft.small_fireball", EntityPacketType::Entity, 0.3125, 0.3125, 0.265625, 4, false);
    pub const SNIFFER: Self = Self::new(119, "sniffer", "entity.minecraft.sniffer", EntityPacketType::Living, 1.9, 1.75, 1.05, 10, false);
    pub const SNOWBALL: Self = Self::new(120, "snowball", "entity.minecraft.snowball", EntityPacketType::Entity, 0.25, 0.25, 0.2125, 4, false);
    pub const SNOW_GOLEM: Self = Self::new(121, "snow_golem", "entity.minecraft.snow_golem", EntityPacketType::Living, 0.7, 1.9, 1.7, 8, false);
    pub const SPAWNER_MINECART: Self = Self::new(122, "spawner_minecart", "entity.minecraft.spawner_minecart", EntityPacketType::Entity, 0.98, 0.7, 0.595, 8, false);
    pub const SPECTRAL_ARROW: Self = Self::new(123, "spectral_arrow", "entity.minecraft.spectral_arrow", EntityPacketType::Entity, 0.5, 0.5, 0.13, 4, false);
    pub const SPIDER: Self = Self::new(124, "spider", "entity.minecraft.spider", EntityPacketType::Living, 1.4, 0.9, 0.65, 8, false);
    pub const SPRUCE_BOAT: Self = Self::new(125, "spruce_boat", "entity.minecraft.spruce_boat", EntityPacketType::Entity, 1.375, 0.5625, 0.5625, 10, false);
    pub const SPRUCE_CHEST_BOAT: Self = Self::new(126, "spruce_chest_boat", "entity.minecraft.spruce_chest_boat", EntityPacketType::Entity, 1.375, 0.5625, 0.5625, 10, false);
    pub const SQUID: Self = Self::new(127, "squid", "entity.minecraft.squid", EntityPacketType::Living, 0.8, 0.8, 0.4, 8, false);
    pub const STRAY: Self = Self::new(128, "stray", "entity.minecraft.stray", EntityPacketType::Living, 0.6, 1.99, 1.74, 8, false);
    pub const STRIDER: Self = Self::new(129, "strider", "entity.minecraft.strider", EntityPacketType::Living, 0.9, 1.7, 1.445, 10, true);
    pub const TADPOLE: Self = Self::new(130, "tadpole", "entity.minecraft.tadpole", EntityPacketType::Living, 0.4, 0.3, 0.19500001, 10, false);
    pub const TEXT_DISPLAY: Self = Self::new(131, "text_display", "entity.minecraft.text_display", EntityPacketType::Entity, 0.0, 0.0, 0.0, 10, false);
    pub const TNT: Self = Self::new(132, "tnt", "entity.minecraft.tnt", EntityPacketType::Entity, 0.98, 0.98, 0.15, 10, true);
    pub const TNT_MINECART: Self = Self::new(133, "tnt_minecart", "entity.minecraft.tnt_minecart", EntityPacketType::Entity, 0.98, 0.7, 0.595, 8, false);
    pub const TRADER_LLAMA: Self = Self::new(134, "trader_llama", "entity.minecraft.trader_llama", EntityPacketType::Living, 0.9, 1.87, 1.7765, 10, false);
    pub const TRIDENT: Self = Self::new(135, "trident", "entity.minecraft.trident", EntityPacketType::Entity, 0.5, 0.5, 0.13, 4, false);
    pub const TROPICAL_FISH: Self = Self::new(136, "tropical_fish", "entity.minecraft.tropical_fish", EntityPacketType::Living, 0.5, 0.4, 0.26, 4, false);
    pub const TURTLE: Self = Self::new(137, "turtle", "entity.minecraft.turtle", EntityPacketType::Living, 1.2, 0.4, 0.34, 10, false);
    pub const VEX: Self = Self::new(138, "vex", "entity.minecraft.vex", EntityPacketType::Living, 0.4, 0.8, 0.51875, 8, true);
    pub const VILLAGER: Self = Self::new(139, "villager", "entity.minecraft.villager", EntityPacketType::Living, 0.6, 1.95, 1.62, 10, false);
    pub const VINDICATOR: Self = Self::new(140, "vindicator", "entity.minecraft.vindicator", EntityPacketType::Living, 0.6, 1.95, 1.6575, 8, false);
    pub const WANDERING_TRADER: Self = Self::new(141, "wandering_trader", "entity.minecraft.wandering_trader", EntityPacketType::Living, 0.6, 1.95, 1.62, 10, false);
    pub const WARDEN: Self = Self::new(142, "warden", "entity.minecraft.warden", EntityPacketType::Living, 0.9, 2.9, 2.4650002, 16, true);
    pub const WIND_CHARGE: Self = Self::new(143, "wind_charge", "entity.minecraft.wind_charge", EntityPacketType::Entity, 0.3125, 0.3125, 0.0, 4, false);
    pub const WITCH: Self = Self::new(144, "witch", "entity.minecraft.witch", EntityPacketType::Living, 0.6, 1.95, 1.62, 8, false);
    pub const WITHER: Self = Self::new(145, "wither", "entity.minecraft.wither", EntityPacketType::Living, 0.9, 3.5, 2.9750001, 10, true);
    pub const WITHER_SKELETON: Self = Self::new(146, "wither_skeleton", "entity.minecraft.wither_skeleton", EntityPacketType::Living, 0.7, 2.4, 2.1, 8, true);
    pub const WITHER_SKULL: Self = Self::new(147, "wither_skull", "entity.minecraft.wither_skull", EntityPacketType::Entity, 0.3125, 0.3125, 0.265625, 4, false);
    pub const WOLF: Self = Self::new(148, "wolf", "entity.minecraft.wolf", EntityPacketType::Living, 0.6, 0.85, 0.68, 10, false);
    pub const ZOGLIN: Self = Self::new(149, "zoglin", "entity.minecraft.zoglin", EntityPacketType::Living, 1.3964844, 1.4, 1.19, 8, true);
    pub const ZOMBIE: Self = Self::new(150, "zombie", "entity.minecraft.zombie", EntityPacketType::Living, 0.6, 1.95, 1.74, 8, false);
    pub const ZOMBIE_HORSE: Self = Self::new(151, "zombie_horse", "entity.minecraft.zombie_horse", EntityPacketType::Living, 1.3964844, 1.6, 1.52, 10, false);
    pub const ZOMBIE_NAUTILUS: Self = Self::new(152, "zombie_nautilus", "entity.minecraft.zombie_nautilus", EntityPacketType::Living, 0.875, 0.95, 0.2751, 10, false);
    pub const ZOMBIE_VILLAGER: Self = Self::new(153, "zombie_villager", "entity.minecraft.zombie_villager", EntityPacketType::Living, 0.6, 1.95, 1.74, 8, false);
    pub const ZOMBIFIED_PIGLIN: Self = Self::new(154, "zombified_piglin", "entity.minecraft.zombified_piglin", EntityPacketType::Living, 0.6, 1.95, 1.79, 8, true);
    pub const PLAYER: Self = Self::new(155, "player", "entity.minecraft.player", EntityPacketType::Player, 0.6, 1.8, 1.62, 32, false);
    pub const FISHING_BOBBER: Self = Self::new(156, "fishing_bobber", "entity.minecraft.fishing_bobber", EntityPacketType::Entity, 0.25, 0.25, 0.2125, 4, false);
    pub const ALL: &'static [Self] = &[
        Self::ACACIA_BOAT,
        Self::ACACIA_CHEST_BOAT,
        Self::ALLAY,
        Self::AREA_EFFECT_CLOUD,
        Self::ARMADILLO,
        Self::ARMOR_STAND,
        Self::ARROW,
        Self::AXOLOTL,
        Self::BAMBOO_CHEST_RAFT,
        Self::BAMBOO_RAFT,
        Self::BAT,
        Self::BEE,
        Self::BIRCH_BOAT,
        Self::BIRCH_CHEST_BOAT,
        Self::BLAZE,
        Self::BLOCK_DISPLAY,
        Self::BOGGED,
        Self::BREEZE,
        Self::BREEZE_WIND_CHARGE,
        Self::CAMEL,
        Self::CAMEL_HUSK,
        Self::CAT,
        Self::CAVE_SPIDER,
        Self::CHERRY_BOAT,
        Self::CHERRY_CHEST_BOAT,
        Self::CHEST_MINECART,
        Self::CHICKEN,
        Self::COD,
        Self::COPPER_GOLEM,
        Self::COMMAND_BLOCK_MINECART,
        Self::COW,
        Self::CREAKING,
        Self::CREEPER,
        Self::DARK_OAK_BOAT,
        Self::DARK_OAK_CHEST_BOAT,
        Self::DOLPHIN,
        Self::DONKEY,
        Self::DRAGON_FIREBALL,
        Self::DROWNED,
        Self::EGG,
        Self::ELDER_GUARDIAN,
        Self::ENDERMAN,
        Self::ENDERMITE,
        Self::ENDER_DRAGON,
        Self::ENDER_PEARL,
        Self::END_CRYSTAL,
        Self::EVOKER,
        Self::EVOKER_FANGS,
        Self::EXPERIENCE_BOTTLE,
        Self::EXPERIENCE_ORB,
        Self::EYE_OF_ENDER,
        Self::FALLING_BLOCK,
        Self::FIREBALL,
        Self::FIREWORK_ROCKET,
        Self::FOX,
        Self::FROG,
        Self::FURNACE_MINECART,
        Self::GHAST,
        Self::HAPPY_GHAST,
        Self::GIANT,
        Self::GLOW_ITEM_FRAME,
        Self::GLOW_SQUID,
        Self::GOAT,
        Self::GUARDIAN,
        Self::HOGLIN,
        Self::HOPPER_MINECART,
        Self::HORSE,
        Self::HUSK,
        Self::ILLUSIONER,
        Self::INTERACTION,
        Self::IRON_GOLEM,
        Self::ITEM,
        Self::ITEM_DISPLAY,
        Self::ITEM_FRAME,
        Self::JUNGLE_BOAT,
        Self::JUNGLE_CHEST_BOAT,
        Self::LEASH_KNOT,
        Self::LIGHTNING_BOLT,
        Self::LLAMA,
        Self::LLAMA_SPIT,
        Self::MAGMA_CUBE,
        Self::MANGROVE_BOAT,
        Self::MANGROVE_CHEST_BOAT,
        Self::MANNEQUIN,
        Self::MARKER,
        Self::MINECART,
        Self::MOOSHROOM,
        Self::MULE,
        Self::NAUTILUS,
        Self::OAK_BOAT,
        Self::OAK_CHEST_BOAT,
        Self::OCELOT,
        Self::OMINOUS_ITEM_SPAWNER,
        Self::PAINTING,
        Self::PALE_OAK_BOAT,
        Self::PALE_OAK_CHEST_BOAT,
        Self::PANDA,
        Self::PARCHED,
        Self::PARROT,
        Self::PHANTOM,
        Self::PIG,
        Self::PIGLIN,
        Self::PIGLIN_BRUTE,
        Self::PILLAGER,
        Self::POLAR_BEAR,
        Self::SPLASH_POTION,
        Self::LINGERING_POTION,
        Self::PUFFERFISH,
        Self::RABBIT,
        Self::RAVAGER,
        Self::SALMON,
        Self::SHEEP,
        Self::SHULKER,
        Self::SHULKER_BULLET,
        Self::SILVERFISH,
        Self::SKELETON,
        Self::SKELETON_HORSE,
        Self::SLIME,
        Self::SMALL_FIREBALL,
        Self::SNIFFER,
        Self::SNOWBALL,
        Self::SNOW_GOLEM,
        Self::SPAWNER_MINECART,
        Self::SPECTRAL_ARROW,
        Self::SPIDER,
        Self::SPRUCE_BOAT,
        Self::SPRUCE_CHEST_BOAT,
        Self::SQUID,
        Self::STRAY,
        Self::STRIDER,
        Self::TADPOLE,
        Self::TEXT_DISPLAY,
        Self::TNT,
        Self::TNT_MINECART,
        Self::TRADER_LLAMA,
        Self::TRIDENT,
        Self::TROPICAL_FISH,
        Self::TURTLE,
        Self::VEX,
        Self::VILLAGER,
        Self::VINDICATOR,
        Self::WANDERING_TRADER,
        Self::WARDEN,
        Self::WIND_CHARGE,
        Self::WITCH,
        Self::WITHER,
        Self::WITHER_SKELETON,
        Self::WITHER_SKULL,
        Self::WOLF,
        Self::ZOGLIN,
        Self::ZOMBIE,
        Self::ZOMBIE_HORSE,
        Self::ZOMBIE_NAUTILUS,
        Self::ZOMBIE_VILLAGER,
        Self::ZOMBIFIED_PIGLIN,
        Self::PLAYER,
        Self::FISHING_BOBBER,
    ];
    pub fn attachments(self) -> &'static [EntityAttachmentOffset] {
        match self {
            Self::ACACIA_BOAT => ACACIA_BOAT_ATTACHMENTS,
            Self::ACACIA_CHEST_BOAT => ACACIA_CHEST_BOAT_ATTACHMENTS,
            Self::ALLAY => ALLAY_ATTACHMENTS,
            Self::AREA_EFFECT_CLOUD => AREA_EFFECT_CLOUD_ATTACHMENTS,
            Self::ARMADILLO => ARMADILLO_ATTACHMENTS,
            Self::ARMOR_STAND => ARMOR_STAND_ATTACHMENTS,
            Self::ARROW => ARROW_ATTACHMENTS,
            Self::AXOLOTL => AXOLOTL_ATTACHMENTS,
            Self::BAMBOO_CHEST_RAFT => BAMBOO_CHEST_RAFT_ATTACHMENTS,
            Self::BAMBOO_RAFT => BAMBOO_RAFT_ATTACHMENTS,
            Self::BAT => BAT_ATTACHMENTS,
            Self::BEE => BEE_ATTACHMENTS,
            Self::BIRCH_BOAT => BIRCH_BOAT_ATTACHMENTS,
            Self::BIRCH_CHEST_BOAT => BIRCH_CHEST_BOAT_ATTACHMENTS,
            Self::BLAZE => BLAZE_ATTACHMENTS,
            Self::BLOCK_DISPLAY => BLOCK_DISPLAY_ATTACHMENTS,
            Self::BOGGED => BOGGED_ATTACHMENTS,
            Self::BREEZE => BREEZE_ATTACHMENTS,
            Self::BREEZE_WIND_CHARGE => BREEZE_WIND_CHARGE_ATTACHMENTS,
            Self::CAMEL => CAMEL_ATTACHMENTS,
            Self::CAMEL_HUSK => CAMEL_HUSK_ATTACHMENTS,
            Self::CAT => CAT_ATTACHMENTS,
            Self::CAVE_SPIDER => CAVE_SPIDER_ATTACHMENTS,
            Self::CHERRY_BOAT => CHERRY_BOAT_ATTACHMENTS,
            Self::CHERRY_CHEST_BOAT => CHERRY_CHEST_BOAT_ATTACHMENTS,
            Self::CHEST_MINECART => CHEST_MINECART_ATTACHMENTS,
            Self::CHICKEN => CHICKEN_ATTACHMENTS,
            Self::COD => COD_ATTACHMENTS,
            Self::COPPER_GOLEM => COPPER_GOLEM_ATTACHMENTS,
            Self::COMMAND_BLOCK_MINECART => COMMAND_BLOCK_MINECART_ATTACHMENTS,
            Self::COW => COW_ATTACHMENTS,
            Self::CREAKING => CREAKING_ATTACHMENTS,
            Self::CREEPER => CREEPER_ATTACHMENTS,
            Self::DARK_OAK_BOAT => DARK_OAK_BOAT_ATTACHMENTS,
            Self::DARK_OAK_CHEST_BOAT => DARK_OAK_CHEST_BOAT_ATTACHMENTS,
            Self::DOLPHIN => DOLPHIN_ATTACHMENTS,
            Self::DONKEY => DONKEY_ATTACHMENTS,
            Self::DRAGON_FIREBALL => DRAGON_FIREBALL_ATTACHMENTS,
            Self::DROWNED => DROWNED_ATTACHMENTS,
            Self::EGG => EGG_ATTACHMENTS,
            Self::ELDER_GUARDIAN => ELDER_GUARDIAN_ATTACHMENTS,
            Self::ENDERMAN => ENDERMAN_ATTACHMENTS,
            Self::ENDERMITE => ENDERMITE_ATTACHMENTS,
            Self::ENDER_DRAGON => ENDER_DRAGON_ATTACHMENTS,
            Self::ENDER_PEARL => ENDER_PEARL_ATTACHMENTS,
            Self::END_CRYSTAL => END_CRYSTAL_ATTACHMENTS,
            Self::EVOKER => EVOKER_ATTACHMENTS,
            Self::EVOKER_FANGS => EVOKER_FANGS_ATTACHMENTS,
            Self::EXPERIENCE_BOTTLE => EXPERIENCE_BOTTLE_ATTACHMENTS,
            Self::EXPERIENCE_ORB => EXPERIENCE_ORB_ATTACHMENTS,
            Self::EYE_OF_ENDER => EYE_OF_ENDER_ATTACHMENTS,
            Self::FALLING_BLOCK => FALLING_BLOCK_ATTACHMENTS,
            Self::FIREBALL => FIREBALL_ATTACHMENTS,
            Self::FIREWORK_ROCKET => FIREWORK_ROCKET_ATTACHMENTS,
            Self::FOX => FOX_ATTACHMENTS,
            Self::FROG => FROG_ATTACHMENTS,
            Self::FURNACE_MINECART => FURNACE_MINECART_ATTACHMENTS,
            Self::GHAST => GHAST_ATTACHMENTS,
            Self::HAPPY_GHAST => HAPPY_GHAST_ATTACHMENTS,
            Self::GIANT => GIANT_ATTACHMENTS,
            Self::GLOW_ITEM_FRAME => GLOW_ITEM_FRAME_ATTACHMENTS,
            Self::GLOW_SQUID => GLOW_SQUID_ATTACHMENTS,
            Self::GOAT => GOAT_ATTACHMENTS,
            Self::GUARDIAN => GUARDIAN_ATTACHMENTS,
            Self::HOGLIN => HOGLIN_ATTACHMENTS,
            Self::HOPPER_MINECART => HOPPER_MINECART_ATTACHMENTS,
            Self::HORSE => HORSE_ATTACHMENTS,
            Self::HUSK => HUSK_ATTACHMENTS,
            Self::ILLUSIONER => ILLUSIONER_ATTACHMENTS,
            Self::INTERACTION => INTERACTION_ATTACHMENTS,
            Self::IRON_GOLEM => IRON_GOLEM_ATTACHMENTS,
            Self::ITEM => ITEM_ATTACHMENTS,
            Self::ITEM_DISPLAY => ITEM_DISPLAY_ATTACHMENTS,
            Self::ITEM_FRAME => ITEM_FRAME_ATTACHMENTS,
            Self::JUNGLE_BOAT => JUNGLE_BOAT_ATTACHMENTS,
            Self::JUNGLE_CHEST_BOAT => JUNGLE_CHEST_BOAT_ATTACHMENTS,
            Self::LEASH_KNOT => LEASH_KNOT_ATTACHMENTS,
            Self::LIGHTNING_BOLT => LIGHTNING_BOLT_ATTACHMENTS,
            Self::LLAMA => LLAMA_ATTACHMENTS,
            Self::LLAMA_SPIT => LLAMA_SPIT_ATTACHMENTS,
            Self::MAGMA_CUBE => MAGMA_CUBE_ATTACHMENTS,
            Self::MANGROVE_BOAT => MANGROVE_BOAT_ATTACHMENTS,
            Self::MANGROVE_CHEST_BOAT => MANGROVE_CHEST_BOAT_ATTACHMENTS,
            Self::MANNEQUIN => MANNEQUIN_ATTACHMENTS,
            Self::MARKER => MARKER_ATTACHMENTS,
            Self::MINECART => MINECART_ATTACHMENTS,
            Self::MOOSHROOM => MOOSHROOM_ATTACHMENTS,
            Self::MULE => MULE_ATTACHMENTS,
            Self::NAUTILUS => NAUTILUS_ATTACHMENTS,
            Self::OAK_BOAT => OAK_BOAT_ATTACHMENTS,
            Self::OAK_CHEST_BOAT => OAK_CHEST_BOAT_ATTACHMENTS,
            Self::OCELOT => OCELOT_ATTACHMENTS,
            Self::OMINOUS_ITEM_SPAWNER => OMINOUS_ITEM_SPAWNER_ATTACHMENTS,
            Self::PAINTING => PAINTING_ATTACHMENTS,
            Self::PALE_OAK_BOAT => PALE_OAK_BOAT_ATTACHMENTS,
            Self::PALE_OAK_CHEST_BOAT => PALE_OAK_CHEST_BOAT_ATTACHMENTS,
            Self::PANDA => PANDA_ATTACHMENTS,
            Self::PARCHED => PARCHED_ATTACHMENTS,
            Self::PARROT => PARROT_ATTACHMENTS,
            Self::PHANTOM => PHANTOM_ATTACHMENTS,
            Self::PIG => PIG_ATTACHMENTS,
            Self::PIGLIN => PIGLIN_ATTACHMENTS,
            Self::PIGLIN_BRUTE => PIGLIN_BRUTE_ATTACHMENTS,
            Self::PILLAGER => PILLAGER_ATTACHMENTS,
            Self::POLAR_BEAR => POLAR_BEAR_ATTACHMENTS,
            Self::SPLASH_POTION => SPLASH_POTION_ATTACHMENTS,
            Self::LINGERING_POTION => LINGERING_POTION_ATTACHMENTS,
            Self::PUFFERFISH => PUFFERFISH_ATTACHMENTS,
            Self::RABBIT => RABBIT_ATTACHMENTS,
            Self::RAVAGER => RAVAGER_ATTACHMENTS,
            Self::SALMON => SALMON_ATTACHMENTS,
            Self::SHEEP => SHEEP_ATTACHMENTS,
            Self::SHULKER => SHULKER_ATTACHMENTS,
            Self::SHULKER_BULLET => SHULKER_BULLET_ATTACHMENTS,
            Self::SILVERFISH => SILVERFISH_ATTACHMENTS,
            Self::SKELETON => SKELETON_ATTACHMENTS,
            Self::SKELETON_HORSE => SKELETON_HORSE_ATTACHMENTS,
            Self::SLIME => SLIME_ATTACHMENTS,
            Self::SMALL_FIREBALL => SMALL_FIREBALL_ATTACHMENTS,
            Self::SNIFFER => SNIFFER_ATTACHMENTS,
            Self::SNOWBALL => SNOWBALL_ATTACHMENTS,
            Self::SNOW_GOLEM => SNOW_GOLEM_ATTACHMENTS,
            Self::SPAWNER_MINECART => SPAWNER_MINECART_ATTACHMENTS,
            Self::SPECTRAL_ARROW => SPECTRAL_ARROW_ATTACHMENTS,
            Self::SPIDER => SPIDER_ATTACHMENTS,
            Self::SPRUCE_BOAT => SPRUCE_BOAT_ATTACHMENTS,
            Self::SPRUCE_CHEST_BOAT => SPRUCE_CHEST_BOAT_ATTACHMENTS,
            Self::SQUID => SQUID_ATTACHMENTS,
            Self::STRAY => STRAY_ATTACHMENTS,
            Self::STRIDER => STRIDER_ATTACHMENTS,
            Self::TADPOLE => TADPOLE_ATTACHMENTS,
            Self::TEXT_DISPLAY => TEXT_DISPLAY_ATTACHMENTS,
            Self::TNT => TNT_ATTACHMENTS,
            Self::TNT_MINECART => TNT_MINECART_ATTACHMENTS,
            Self::TRADER_LLAMA => TRADER_LLAMA_ATTACHMENTS,
            Self::TRIDENT => TRIDENT_ATTACHMENTS,
            Self::TROPICAL_FISH => TROPICAL_FISH_ATTACHMENTS,
            Self::TURTLE => TURTLE_ATTACHMENTS,
            Self::VEX => VEX_ATTACHMENTS,
            Self::VILLAGER => VILLAGER_ATTACHMENTS,
            Self::VINDICATOR => VINDICATOR_ATTACHMENTS,
            Self::WANDERING_TRADER => WANDERING_TRADER_ATTACHMENTS,
            Self::WARDEN => WARDEN_ATTACHMENTS,
            Self::WIND_CHARGE => WIND_CHARGE_ATTACHMENTS,
            Self::WITCH => WITCH_ATTACHMENTS,
            Self::WITHER => WITHER_ATTACHMENTS,
            Self::WITHER_SKELETON => WITHER_SKELETON_ATTACHMENTS,
            Self::WITHER_SKULL => WITHER_SKULL_ATTACHMENTS,
            Self::WOLF => WOLF_ATTACHMENTS,
            Self::ZOGLIN => ZOGLIN_ATTACHMENTS,
            Self::ZOMBIE => ZOMBIE_ATTACHMENTS,
            Self::ZOMBIE_HORSE => ZOMBIE_HORSE_ATTACHMENTS,
            Self::ZOMBIE_NAUTILUS => ZOMBIE_NAUTILUS_ATTACHMENTS,
            Self::ZOMBIE_VILLAGER => ZOMBIE_VILLAGER_ATTACHMENTS,
            Self::ZOMBIFIED_PIGLIN => ZOMBIFIED_PIGLIN_ATTACHMENTS,
            Self::PLAYER => PLAYER_ATTACHMENTS,
            Self::FISHING_BOBBER => FISHING_BOBBER_ATTACHMENTS,
            _ => &[],
        }
    }
    pub fn default_attributes(self) -> &'static [EntityDefaultAttribute] {
        match self {
            Self::ACACIA_BOAT => ACACIA_BOAT_DEFAULT_ATTRIBUTES,
            Self::ACACIA_CHEST_BOAT => ACACIA_CHEST_BOAT_DEFAULT_ATTRIBUTES,
            Self::ALLAY => ALLAY_DEFAULT_ATTRIBUTES,
            Self::AREA_EFFECT_CLOUD => AREA_EFFECT_CLOUD_DEFAULT_ATTRIBUTES,
            Self::ARMADILLO => ARMADILLO_DEFAULT_ATTRIBUTES,
            Self::ARMOR_STAND => ARMOR_STAND_DEFAULT_ATTRIBUTES,
            Self::ARROW => ARROW_DEFAULT_ATTRIBUTES,
            Self::AXOLOTL => AXOLOTL_DEFAULT_ATTRIBUTES,
            Self::BAMBOO_CHEST_RAFT => BAMBOO_CHEST_RAFT_DEFAULT_ATTRIBUTES,
            Self::BAMBOO_RAFT => BAMBOO_RAFT_DEFAULT_ATTRIBUTES,
            Self::BAT => BAT_DEFAULT_ATTRIBUTES,
            Self::BEE => BEE_DEFAULT_ATTRIBUTES,
            Self::BIRCH_BOAT => BIRCH_BOAT_DEFAULT_ATTRIBUTES,
            Self::BIRCH_CHEST_BOAT => BIRCH_CHEST_BOAT_DEFAULT_ATTRIBUTES,
            Self::BLAZE => BLAZE_DEFAULT_ATTRIBUTES,
            Self::BLOCK_DISPLAY => BLOCK_DISPLAY_DEFAULT_ATTRIBUTES,
            Self::BOGGED => BOGGED_DEFAULT_ATTRIBUTES,
            Self::BREEZE => BREEZE_DEFAULT_ATTRIBUTES,
            Self::BREEZE_WIND_CHARGE => BREEZE_WIND_CHARGE_DEFAULT_ATTRIBUTES,
            Self::CAMEL => CAMEL_DEFAULT_ATTRIBUTES,
            Self::CAMEL_HUSK => CAMEL_HUSK_DEFAULT_ATTRIBUTES,
            Self::CAT => CAT_DEFAULT_ATTRIBUTES,
            Self::CAVE_SPIDER => CAVE_SPIDER_DEFAULT_ATTRIBUTES,
            Self::CHERRY_BOAT => CHERRY_BOAT_DEFAULT_ATTRIBUTES,
            Self::CHERRY_CHEST_BOAT => CHERRY_CHEST_BOAT_DEFAULT_ATTRIBUTES,
            Self::CHEST_MINECART => CHEST_MINECART_DEFAULT_ATTRIBUTES,
            Self::CHICKEN => CHICKEN_DEFAULT_ATTRIBUTES,
            Self::COD => COD_DEFAULT_ATTRIBUTES,
            Self::COPPER_GOLEM => COPPER_GOLEM_DEFAULT_ATTRIBUTES,
            Self::COMMAND_BLOCK_MINECART => COMMAND_BLOCK_MINECART_DEFAULT_ATTRIBUTES,
            Self::COW => COW_DEFAULT_ATTRIBUTES,
            Self::CREAKING => CREAKING_DEFAULT_ATTRIBUTES,
            Self::CREEPER => CREEPER_DEFAULT_ATTRIBUTES,
            Self::DARK_OAK_BOAT => DARK_OAK_BOAT_DEFAULT_ATTRIBUTES,
            Self::DARK_OAK_CHEST_BOAT => DARK_OAK_CHEST_BOAT_DEFAULT_ATTRIBUTES,
            Self::DOLPHIN => DOLPHIN_DEFAULT_ATTRIBUTES,
            Self::DONKEY => DONKEY_DEFAULT_ATTRIBUTES,
            Self::DRAGON_FIREBALL => DRAGON_FIREBALL_DEFAULT_ATTRIBUTES,
            Self::DROWNED => DROWNED_DEFAULT_ATTRIBUTES,
            Self::EGG => EGG_DEFAULT_ATTRIBUTES,
            Self::ELDER_GUARDIAN => ELDER_GUARDIAN_DEFAULT_ATTRIBUTES,
            Self::ENDERMAN => ENDERMAN_DEFAULT_ATTRIBUTES,
            Self::ENDERMITE => ENDERMITE_DEFAULT_ATTRIBUTES,
            Self::ENDER_DRAGON => ENDER_DRAGON_DEFAULT_ATTRIBUTES,
            Self::ENDER_PEARL => ENDER_PEARL_DEFAULT_ATTRIBUTES,
            Self::END_CRYSTAL => END_CRYSTAL_DEFAULT_ATTRIBUTES,
            Self::EVOKER => EVOKER_DEFAULT_ATTRIBUTES,
            Self::EVOKER_FANGS => EVOKER_FANGS_DEFAULT_ATTRIBUTES,
            Self::EXPERIENCE_BOTTLE => EXPERIENCE_BOTTLE_DEFAULT_ATTRIBUTES,
            Self::EXPERIENCE_ORB => EXPERIENCE_ORB_DEFAULT_ATTRIBUTES,
            Self::EYE_OF_ENDER => EYE_OF_ENDER_DEFAULT_ATTRIBUTES,
            Self::FALLING_BLOCK => FALLING_BLOCK_DEFAULT_ATTRIBUTES,
            Self::FIREBALL => FIREBALL_DEFAULT_ATTRIBUTES,
            Self::FIREWORK_ROCKET => FIREWORK_ROCKET_DEFAULT_ATTRIBUTES,
            Self::FOX => FOX_DEFAULT_ATTRIBUTES,
            Self::FROG => FROG_DEFAULT_ATTRIBUTES,
            Self::FURNACE_MINECART => FURNACE_MINECART_DEFAULT_ATTRIBUTES,
            Self::GHAST => GHAST_DEFAULT_ATTRIBUTES,
            Self::HAPPY_GHAST => HAPPY_GHAST_DEFAULT_ATTRIBUTES,
            Self::GIANT => GIANT_DEFAULT_ATTRIBUTES,
            Self::GLOW_ITEM_FRAME => GLOW_ITEM_FRAME_DEFAULT_ATTRIBUTES,
            Self::GLOW_SQUID => GLOW_SQUID_DEFAULT_ATTRIBUTES,
            Self::GOAT => GOAT_DEFAULT_ATTRIBUTES,
            Self::GUARDIAN => GUARDIAN_DEFAULT_ATTRIBUTES,
            Self::HOGLIN => HOGLIN_DEFAULT_ATTRIBUTES,
            Self::HOPPER_MINECART => HOPPER_MINECART_DEFAULT_ATTRIBUTES,
            Self::HORSE => HORSE_DEFAULT_ATTRIBUTES,
            Self::HUSK => HUSK_DEFAULT_ATTRIBUTES,
            Self::ILLUSIONER => ILLUSIONER_DEFAULT_ATTRIBUTES,
            Self::INTERACTION => INTERACTION_DEFAULT_ATTRIBUTES,
            Self::IRON_GOLEM => IRON_GOLEM_DEFAULT_ATTRIBUTES,
            Self::ITEM => ITEM_DEFAULT_ATTRIBUTES,
            Self::ITEM_DISPLAY => ITEM_DISPLAY_DEFAULT_ATTRIBUTES,
            Self::ITEM_FRAME => ITEM_FRAME_DEFAULT_ATTRIBUTES,
            Self::JUNGLE_BOAT => JUNGLE_BOAT_DEFAULT_ATTRIBUTES,
            Self::JUNGLE_CHEST_BOAT => JUNGLE_CHEST_BOAT_DEFAULT_ATTRIBUTES,
            Self::LEASH_KNOT => LEASH_KNOT_DEFAULT_ATTRIBUTES,
            Self::LIGHTNING_BOLT => LIGHTNING_BOLT_DEFAULT_ATTRIBUTES,
            Self::LLAMA => LLAMA_DEFAULT_ATTRIBUTES,
            Self::LLAMA_SPIT => LLAMA_SPIT_DEFAULT_ATTRIBUTES,
            Self::MAGMA_CUBE => MAGMA_CUBE_DEFAULT_ATTRIBUTES,
            Self::MANGROVE_BOAT => MANGROVE_BOAT_DEFAULT_ATTRIBUTES,
            Self::MANGROVE_CHEST_BOAT => MANGROVE_CHEST_BOAT_DEFAULT_ATTRIBUTES,
            Self::MANNEQUIN => MANNEQUIN_DEFAULT_ATTRIBUTES,
            Self::MARKER => MARKER_DEFAULT_ATTRIBUTES,
            Self::MINECART => MINECART_DEFAULT_ATTRIBUTES,
            Self::MOOSHROOM => MOOSHROOM_DEFAULT_ATTRIBUTES,
            Self::MULE => MULE_DEFAULT_ATTRIBUTES,
            Self::NAUTILUS => NAUTILUS_DEFAULT_ATTRIBUTES,
            Self::OAK_BOAT => OAK_BOAT_DEFAULT_ATTRIBUTES,
            Self::OAK_CHEST_BOAT => OAK_CHEST_BOAT_DEFAULT_ATTRIBUTES,
            Self::OCELOT => OCELOT_DEFAULT_ATTRIBUTES,
            Self::OMINOUS_ITEM_SPAWNER => OMINOUS_ITEM_SPAWNER_DEFAULT_ATTRIBUTES,
            Self::PAINTING => PAINTING_DEFAULT_ATTRIBUTES,
            Self::PALE_OAK_BOAT => PALE_OAK_BOAT_DEFAULT_ATTRIBUTES,
            Self::PALE_OAK_CHEST_BOAT => PALE_OAK_CHEST_BOAT_DEFAULT_ATTRIBUTES,
            Self::PANDA => PANDA_DEFAULT_ATTRIBUTES,
            Self::PARCHED => PARCHED_DEFAULT_ATTRIBUTES,
            Self::PARROT => PARROT_DEFAULT_ATTRIBUTES,
            Self::PHANTOM => PHANTOM_DEFAULT_ATTRIBUTES,
            Self::PIG => PIG_DEFAULT_ATTRIBUTES,
            Self::PIGLIN => PIGLIN_DEFAULT_ATTRIBUTES,
            Self::PIGLIN_BRUTE => PIGLIN_BRUTE_DEFAULT_ATTRIBUTES,
            Self::PILLAGER => PILLAGER_DEFAULT_ATTRIBUTES,
            Self::POLAR_BEAR => POLAR_BEAR_DEFAULT_ATTRIBUTES,
            Self::SPLASH_POTION => SPLASH_POTION_DEFAULT_ATTRIBUTES,
            Self::LINGERING_POTION => LINGERING_POTION_DEFAULT_ATTRIBUTES,
            Self::PUFFERFISH => PUFFERFISH_DEFAULT_ATTRIBUTES,
            Self::RABBIT => RABBIT_DEFAULT_ATTRIBUTES,
            Self::RAVAGER => RAVAGER_DEFAULT_ATTRIBUTES,
            Self::SALMON => SALMON_DEFAULT_ATTRIBUTES,
            Self::SHEEP => SHEEP_DEFAULT_ATTRIBUTES,
            Self::SHULKER => SHULKER_DEFAULT_ATTRIBUTES,
            Self::SHULKER_BULLET => SHULKER_BULLET_DEFAULT_ATTRIBUTES,
            Self::SILVERFISH => SILVERFISH_DEFAULT_ATTRIBUTES,
            Self::SKELETON => SKELETON_DEFAULT_ATTRIBUTES,
            Self::SKELETON_HORSE => SKELETON_HORSE_DEFAULT_ATTRIBUTES,
            Self::SLIME => SLIME_DEFAULT_ATTRIBUTES,
            Self::SMALL_FIREBALL => SMALL_FIREBALL_DEFAULT_ATTRIBUTES,
            Self::SNIFFER => SNIFFER_DEFAULT_ATTRIBUTES,
            Self::SNOWBALL => SNOWBALL_DEFAULT_ATTRIBUTES,
            Self::SNOW_GOLEM => SNOW_GOLEM_DEFAULT_ATTRIBUTES,
            Self::SPAWNER_MINECART => SPAWNER_MINECART_DEFAULT_ATTRIBUTES,
            Self::SPECTRAL_ARROW => SPECTRAL_ARROW_DEFAULT_ATTRIBUTES,
            Self::SPIDER => SPIDER_DEFAULT_ATTRIBUTES,
            Self::SPRUCE_BOAT => SPRUCE_BOAT_DEFAULT_ATTRIBUTES,
            Self::SPRUCE_CHEST_BOAT => SPRUCE_CHEST_BOAT_DEFAULT_ATTRIBUTES,
            Self::SQUID => SQUID_DEFAULT_ATTRIBUTES,
            Self::STRAY => STRAY_DEFAULT_ATTRIBUTES,
            Self::STRIDER => STRIDER_DEFAULT_ATTRIBUTES,
            Self::TADPOLE => TADPOLE_DEFAULT_ATTRIBUTES,
            Self::TEXT_DISPLAY => TEXT_DISPLAY_DEFAULT_ATTRIBUTES,
            Self::TNT => TNT_DEFAULT_ATTRIBUTES,
            Self::TNT_MINECART => TNT_MINECART_DEFAULT_ATTRIBUTES,
            Self::TRADER_LLAMA => TRADER_LLAMA_DEFAULT_ATTRIBUTES,
            Self::TRIDENT => TRIDENT_DEFAULT_ATTRIBUTES,
            Self::TROPICAL_FISH => TROPICAL_FISH_DEFAULT_ATTRIBUTES,
            Self::TURTLE => TURTLE_DEFAULT_ATTRIBUTES,
            Self::VEX => VEX_DEFAULT_ATTRIBUTES,
            Self::VILLAGER => VILLAGER_DEFAULT_ATTRIBUTES,
            Self::VINDICATOR => VINDICATOR_DEFAULT_ATTRIBUTES,
            Self::WANDERING_TRADER => WANDERING_TRADER_DEFAULT_ATTRIBUTES,
            Self::WARDEN => WARDEN_DEFAULT_ATTRIBUTES,
            Self::WIND_CHARGE => WIND_CHARGE_DEFAULT_ATTRIBUTES,
            Self::WITCH => WITCH_DEFAULT_ATTRIBUTES,
            Self::WITHER => WITHER_DEFAULT_ATTRIBUTES,
            Self::WITHER_SKELETON => WITHER_SKELETON_DEFAULT_ATTRIBUTES,
            Self::WITHER_SKULL => WITHER_SKULL_DEFAULT_ATTRIBUTES,
            Self::WOLF => WOLF_DEFAULT_ATTRIBUTES,
            Self::ZOGLIN => ZOGLIN_DEFAULT_ATTRIBUTES,
            Self::ZOMBIE => ZOMBIE_DEFAULT_ATTRIBUTES,
            Self::ZOMBIE_HORSE => ZOMBIE_HORSE_DEFAULT_ATTRIBUTES,
            Self::ZOMBIE_NAUTILUS => ZOMBIE_NAUTILUS_DEFAULT_ATTRIBUTES,
            Self::ZOMBIE_VILLAGER => ZOMBIE_VILLAGER_DEFAULT_ATTRIBUTES,
            Self::ZOMBIFIED_PIGLIN => ZOMBIFIED_PIGLIN_DEFAULT_ATTRIBUTES,
            Self::PLAYER => PLAYER_DEFAULT_ATTRIBUTES,
            Self::FISHING_BOBBER => FISHING_BOBBER_DEFAULT_ATTRIBUTES,
            _ => &[],
        }
    }
}
