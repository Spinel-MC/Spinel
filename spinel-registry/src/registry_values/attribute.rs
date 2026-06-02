use crate::Identifier;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Attribute {
    protocol_id: i32,
    key: &'static str,
    default_value: f64,
    min_value: f64,
    max_value: f64,
    syncable: bool,
}

impl Attribute {
    pub const ARMOR: Self = Self::new(0, "armor", 0.0, 0.0, 30.0, true);
    pub const ARMOR_TOUGHNESS: Self = Self::new(1, "armor_toughness", 0.0, 0.0, 20.0, true);
    pub const ATTACK_DAMAGE: Self = Self::new(2, "attack_damage", 2.0, 0.0, 2048.0, false);
    pub const ATTACK_KNOCKBACK: Self = Self::new(3, "attack_knockback", 0.0, 0.0, 5.0, false);
    pub const ATTACK_SPEED: Self = Self::new(4, "attack_speed", 4.0, 0.0, 1024.0, true);
    pub const BLOCK_BREAK_SPEED: Self = Self::new(5, "block_break_speed", 1.0, 0.0, 1024.0, true);
    pub const BLOCK_INTERACTION_RANGE: Self =
        Self::new(6, "block_interaction_range", 4.5, 0.0, 64.0, true);
    pub const BURNING_TIME: Self = Self::new(7, "burning_time", 1.0, 0.0, 1024.0, true);
    pub const CAMERA_DISTANCE: Self = Self::new(8, "camera_distance", 4.0, 0.0, 32.0, true);
    pub const EXPLOSION_KNOCKBACK_RESISTANCE: Self =
        Self::new(9, "explosion_knockback_resistance", 0.0, 0.0, 1.0, true);
    pub const ENTITY_INTERACTION_RANGE: Self =
        Self::new(10, "entity_interaction_range", 3.0, 0.0, 64.0, true);
    pub const FALL_DAMAGE_MULTIPLIER: Self =
        Self::new(11, "fall_damage_multiplier", 1.0, 0.0, 100.0, true);
    pub const FLYING_SPEED: Self = Self::new(12, "flying_speed", 0.4, 0.0, 1024.0, true);
    pub const FOLLOW_RANGE: Self = Self::new(13, "follow_range", 32.0, 0.0, 2048.0, false);
    pub const GRAVITY: Self = Self::new(14, "gravity", 0.08, -1.0, 1.0, true);
    pub const JUMP_STRENGTH: Self =
        Self::new(15, "jump_strength", 0.41999998688697815, 0.0, 32.0, true);
    pub const KNOCKBACK_RESISTANCE: Self =
        Self::new(16, "knockback_resistance", 0.0, 0.0, 1.0, false);
    pub const LUCK: Self = Self::new(17, "luck", 0.0, -1024.0, 1024.0, true);
    pub const MAX_ABSORPTION: Self = Self::new(18, "max_absorption", 0.0, 0.0, 2048.0, true);
    pub const MAX_HEALTH: Self = Self::new(19, "max_health", 20.0, 1.0, 1024.0, true);
    pub const MINING_EFFICIENCY: Self = Self::new(20, "mining_efficiency", 0.0, 0.0, 1024.0, true);
    pub const MOVEMENT_EFFICIENCY: Self = Self::new(21, "movement_efficiency", 0.0, 0.0, 1.0, true);
    pub const MOVEMENT_SPEED: Self = Self::new(22, "movement_speed", 0.7, 0.0, 1024.0, true);
    pub const OXYGEN_BONUS: Self = Self::new(23, "oxygen_bonus", 0.0, 0.0, 1024.0, true);
    pub const SAFE_FALL_DISTANCE: Self =
        Self::new(24, "safe_fall_distance", 3.0, -1024.0, 1024.0, true);
    pub const SCALE: Self = Self::new(25, "scale", 1.0, 0.0625, 16.0, true);
    pub const SNEAKING_SPEED: Self = Self::new(26, "sneaking_speed", 0.3, 0.0, 1.0, true);
    pub const SPAWN_REINFORCEMENTS: Self =
        Self::new(27, "spawn_reinforcements", 0.0, 0.0, 1.0, false);
    pub const STEP_HEIGHT: Self = Self::new(28, "step_height", 0.6, 0.0, 10.0, true);
    pub const SUBMERGED_MINING_SPEED: Self =
        Self::new(29, "submerged_mining_speed", 0.2, 0.0, 20.0, true);
    pub const SWEEPING_DAMAGE_RATIO: Self =
        Self::new(30, "sweeping_damage_ratio", 0.0, 0.0, 1.0, true);
    pub const TEMPT_RANGE: Self = Self::new(31, "tempt_range", 10.0, 0.0, 2048.0, false);
    pub const WATER_MOVEMENT_EFFICIENCY: Self =
        Self::new(32, "water_movement_efficiency", 0.0, 0.0, 1.0, true);
    pub const WAYPOINT_TRANSMIT_RANGE: Self =
        Self::new(33, "waypoint_transmit_range", 0.0, 0.0, 60_000_000.0, false);
    pub const WAYPOINT_RECEIVE_RANGE: Self =
        Self::new(34, "waypoint_receive_range", 0.0, 0.0, 60_000_000.0, false);

    pub const fn new(
        protocol_id: i32,
        key: &'static str,
        default_value: f64,
        min_value: f64,
        max_value: f64,
        syncable: bool,
    ) -> Self {
        Self {
            protocol_id,
            key,
            default_value,
            min_value,
            max_value,
            syncable,
        }
    }

    pub const fn protocol_id(self) -> i32 {
        self.protocol_id
    }

    pub fn identifier(self) -> Identifier {
        Identifier::minecraft(self.key)
    }

    pub const fn key(self) -> &'static str {
        self.key
    }

    pub const fn default_value(self) -> f64 {
        self.default_value
    }

    pub const fn min_value(self) -> f64 {
        self.min_value
    }

    pub const fn max_value(self) -> f64 {
        self.max_value
    }

    pub const fn is_syncable(self) -> bool {
        self.syncable
    }

    pub fn from_identifier(identifier: &Identifier) -> Option<Self> {
        Self::from_key(&identifier.path)
    }

    pub fn from_protocol_id(protocol_id: i32) -> Option<Self> {
        ALL_ATTRIBUTES
            .iter()
            .copied()
            .find(|attribute| attribute.protocol_id == protocol_id)
    }

    pub fn from_key(key: &str) -> Option<Self> {
        ALL_ATTRIBUTES
            .iter()
            .copied()
            .find(|attribute| attribute.key == key)
    }
}

pub const ALL_ATTRIBUTES: &[Attribute] = &[
    Attribute::ARMOR,
    Attribute::ARMOR_TOUGHNESS,
    Attribute::ATTACK_DAMAGE,
    Attribute::ATTACK_KNOCKBACK,
    Attribute::ATTACK_SPEED,
    Attribute::BLOCK_BREAK_SPEED,
    Attribute::BLOCK_INTERACTION_RANGE,
    Attribute::BURNING_TIME,
    Attribute::CAMERA_DISTANCE,
    Attribute::EXPLOSION_KNOCKBACK_RESISTANCE,
    Attribute::ENTITY_INTERACTION_RANGE,
    Attribute::FALL_DAMAGE_MULTIPLIER,
    Attribute::FLYING_SPEED,
    Attribute::FOLLOW_RANGE,
    Attribute::GRAVITY,
    Attribute::JUMP_STRENGTH,
    Attribute::KNOCKBACK_RESISTANCE,
    Attribute::LUCK,
    Attribute::MAX_ABSORPTION,
    Attribute::MAX_HEALTH,
    Attribute::MINING_EFFICIENCY,
    Attribute::MOVEMENT_EFFICIENCY,
    Attribute::MOVEMENT_SPEED,
    Attribute::OXYGEN_BONUS,
    Attribute::SAFE_FALL_DISTANCE,
    Attribute::SCALE,
    Attribute::SNEAKING_SPEED,
    Attribute::SPAWN_REINFORCEMENTS,
    Attribute::STEP_HEIGHT,
    Attribute::SUBMERGED_MINING_SPEED,
    Attribute::SWEEPING_DAMAGE_RATIO,
    Attribute::TEMPT_RANGE,
    Attribute::WATER_MOVEMENT_EFFICIENCY,
    Attribute::WAYPOINT_TRANSMIT_RANGE,
    Attribute::WAYPOINT_RECEIVE_RANGE,
];
