use spinel_registry::Identifier;
use std::collections::BTreeMap;
use std::sync::LazyLock;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct GameRule<T> {
    key: &'static str,
    id: i32,
    default_value: T,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct AnyGameRule {
    key: &'static str,
    id: i32,
    default_value: GameRuleValue,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum GameRuleValue {
    Boolean(bool),
    Integer(i32),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameRuleRequestEntry {
    game_rule: AnyGameRule,
    value: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorldGameRules {
    values: BTreeMap<&'static str, GameRuleValue>,
}

pub trait GameRuleValueType: Copy {
    fn into_game_rule_value(value: Self) -> GameRuleValue;
    fn from_game_rule_value(value: GameRuleValue) -> Option<Self>;
}

impl<T: Copy> GameRule<T> {
    pub const fn new(key: &'static str, id: i32, default_value: T) -> Self {
        Self {
            key,
            id,
            default_value,
        }
    }

    pub const fn get_key(self) -> &'static str {
        self.key
    }

    pub const fn get_id(self) -> i32 {
        self.id
    }

    pub const fn get_default_value(self) -> T {
        self.default_value
    }

    pub fn erase(self) -> AnyGameRule
    where
        T: GameRuleValueType,
    {
        AnyGameRule::new(
            self.key,
            self.id,
            T::into_game_rule_value(self.default_value),
        )
    }
}

impl GameRule<bool> {
    pub const ADVANCE_TIME: Self = Self::new("advance_time", 0, true);
    pub const ADVANCE_WEATHER: Self = Self::new("advance_weather", 1, true);
    pub const ALLOW_ENTERING_NETHER_USING_PORTALS: Self =
        Self::new("allow_entering_nether_using_portals", 2, true);
    pub const BLOCK_DROPS: Self = Self::new("block_drops", 3, true);
    pub const BLOCK_EXPLOSION_DROP_DECAY: Self = Self::new("block_explosion_drop_decay", 4, true);
    pub const COMMAND_BLOCKS_WORK: Self = Self::new("command_blocks_work", 5, true);
    pub const COMMAND_BLOCK_OUTPUT: Self = Self::new("command_block_output", 6, true);
    pub const DROWNING_DAMAGE: Self = Self::new("drowning_damage", 7, true);
    pub const ELYTRA_MOVEMENT_CHECK: Self = Self::new("elytra_movement_check", 8, true);
    pub const ENDER_PEARLS_VANISH_ON_DEATH: Self =
        Self::new("ender_pearls_vanish_on_death", 9, true);
    pub const ENTITY_DROPS: Self = Self::new("entity_drops", 10, true);
    pub const FALL_DAMAGE: Self = Self::new("fall_damage", 11, true);
    pub const FIRE_DAMAGE: Self = Self::new("fire_damage", 12, true);
    pub const FORGIVE_DEAD_PLAYERS: Self = Self::new("forgive_dead_players", 14, true);
    pub const FREEZE_DAMAGE: Self = Self::new("freeze_damage", 15, true);
    pub const GLOBAL_SOUND_EVENTS: Self = Self::new("global_sound_events", 16, true);
    pub const IMMEDIATE_RESPAWN: Self = Self::new("immediate_respawn", 17, false);
    pub const KEEP_INVENTORY: Self = Self::new("keep_inventory", 18, false);
    pub const LAVA_SOURCE_CONVERSION: Self = Self::new("lava_source_conversion", 19, false);
    pub const LIMITED_CRAFTING: Self = Self::new("limited_crafting", 20, false);
    pub const LOCATOR_BAR: Self = Self::new("locator_bar", 21, true);
    pub const LOG_ADMIN_COMMANDS: Self = Self::new("log_admin_commands", 22, true);
    pub const MOB_DROPS: Self = Self::new("mob_drops", 29, true);
    pub const MOB_EXPLOSION_DROP_DECAY: Self = Self::new("mob_explosion_drop_decay", 30, true);
    pub const MOB_GRIEFING: Self = Self::new("mob_griefing", 31, true);
    pub const NATURAL_HEALTH_REGENERATION: Self =
        Self::new("natural_health_regeneration", 32, true);
    pub const PLAYER_MOVEMENT_CHECK: Self = Self::new("player_movement_check", 33, true);
    pub const PROJECTILES_CAN_BREAK_BLOCKS: Self =
        Self::new("projectiles_can_break_blocks", 38, true);
    pub const PVP: Self = Self::new("pvp", 39, true);
    pub const RAIDS: Self = Self::new("raids", 40, true);
    pub const REDUCED_DEBUG_INFO: Self = Self::new("reduced_debug_info", 42, false);
    pub const SEND_COMMAND_FEEDBACK: Self = Self::new("send_command_feedback", 44, true);
    pub const SHOW_ADVANCEMENT_MESSAGES: Self = Self::new("show_advancement_messages", 45, true);
    pub const SHOW_DEATH_MESSAGES: Self = Self::new("show_death_messages", 46, true);
    pub const SPAWNER_BLOCKS_WORK: Self = Self::new("spawner_blocks_work", 47, true);
    pub const SPAWN_MOBS: Self = Self::new("spawn_mobs", 48, true);
    pub const SPAWN_MONSTERS: Self = Self::new("spawn_monsters", 49, true);
    pub const SPAWN_PATROLS: Self = Self::new("spawn_patrols", 50, true);
    pub const SPAWN_PHANTOMS: Self = Self::new("spawn_phantoms", 51, true);
    pub const SPAWN_WANDERING_TRADERS: Self = Self::new("spawn_wandering_traders", 52, true);
    pub const SPAWN_WARDENS: Self = Self::new("spawn_wardens", 53, true);
    pub const SPECTATORS_GENERATE_CHUNKS: Self = Self::new("spectators_generate_chunks", 54, true);
    pub const SPREAD_VINES: Self = Self::new("spread_vines", 55, true);
    pub const TNT_EXPLODES: Self = Self::new("tnt_explodes", 56, true);
    pub const TNT_EXPLOSION_DROP_DECAY: Self = Self::new("tnt_explosion_drop_decay", 57, true);
    pub const UNIVERSAL_ANGER: Self = Self::new("universal_anger", 58, false);
    pub const WATER_SOURCE_CONVERSION: Self = Self::new("water_source_conversion", 59, true);
}

impl GameRule<i32> {
    pub const FIRE_SPREAD_RADIUS_AROUND_PLAYER: Self =
        Self::new("fire_spread_radius_around_player", 13, 128);
    pub const MAX_BLOCK_MODIFICATIONS: Self = Self::new("max_block_modifications", 23, 1_000_000);
    pub const MAX_COMMAND_FORKS: Self = Self::new("max_command_forks", 24, 65_536);
    pub const MAX_COMMAND_SEQUENCE_LENGTH: Self =
        Self::new("max_command_sequence_length", 25, 65_536);
    pub const MAX_ENTITY_CRAMMING: Self = Self::new("max_entity_cramming", 26, 24);
    pub const MAX_MINECART_SPEED: Self = Self::new("max_minecart_speed", 27, 8);
    pub const MAX_SNOW_ACCUMULATION_HEIGHT: Self = Self::new("max_snow_accumulation_height", 28, 1);
    pub const PLAYERS_NETHER_PORTAL_CREATIVE_DELAY: Self =
        Self::new("players_nether_portal_creative_delay", 34, 1);
    pub const PLAYERS_NETHER_PORTAL_DEFAULT_DELAY: Self =
        Self::new("players_nether_portal_default_delay", 35, 80);
    pub const PLAYERS_SLEEPING_PERCENTAGE: Self = Self::new("players_sleeping_percentage", 36, 100);
    pub const RANDOM_TICK_SPEED: Self = Self::new("random_tick_speed", 41, 3);
    pub const RESPAWN_RADIUS: Self = Self::new("respawn_radius", 43, 10);
}

impl AnyGameRule {
    pub const fn new(key: &'static str, id: i32, default_value: GameRuleValue) -> Self {
        Self {
            key,
            id,
            default_value,
        }
    }

    pub const fn get_key(self) -> &'static str {
        self.key
    }

    pub const fn get_id(self) -> i32 {
        self.id
    }

    pub const fn get_default_value(self) -> GameRuleValue {
        self.default_value
    }

    pub fn get_identifier(self) -> Identifier {
        Identifier::minecraft(self.key)
    }

    pub fn parse_boolean_value(self, value: &str) -> Option<bool> {
        match self.default_value {
            GameRuleValue::Boolean(_) => value.parse().ok(),
            GameRuleValue::Integer(_) => None,
        }
    }

    pub fn parse_integer_value(self, value: &str) -> Option<i32> {
        match self.default_value {
            GameRuleValue::Boolean(_) => None,
            GameRuleValue::Integer(_) => value.parse().ok(),
        }
    }
}

impl GameRuleRequestEntry {
    pub fn new(game_rule: AnyGameRule, value: impl Into<String>) -> Self {
        Self {
            game_rule,
            value: value.into(),
        }
    }

    pub const fn get_game_rule(&self) -> AnyGameRule {
        self.game_rule
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}

impl Default for WorldGameRules {
    fn default() -> Self {
        let values = static_game_rules()
            .iter()
            .map(|game_rule| (game_rule.get_key(), game_rule.get_default_value()))
            .collect();
        Self { values }
    }
}

impl WorldGameRules {
    pub fn get<T: GameRuleValueType>(&self, game_rule: GameRule<T>) -> T {
        self.values
            .get(game_rule.get_key())
            .copied()
            .and_then(T::from_game_rule_value)
            .unwrap_or_else(|| game_rule.get_default_value())
    }

    pub fn set<T: GameRuleValueType>(&mut self, game_rule: GameRule<T>, value: T) {
        self.values
            .insert(game_rule.get_key(), T::into_game_rule_value(value));
    }

    pub fn value_strings(&self) -> BTreeMap<Identifier, String> {
        self.values
            .iter()
            .map(|(game_rule_key, value)| {
                (Identifier::minecraft(*game_rule_key), value.to_string())
            })
            .collect()
    }
}

impl GameRuleValueType for bool {
    fn into_game_rule_value(value: Self) -> GameRuleValue {
        GameRuleValue::Boolean(value)
    }

    fn from_game_rule_value(value: GameRuleValue) -> Option<Self> {
        match value {
            GameRuleValue::Boolean(value) => Some(value),
            GameRuleValue::Integer(_) => None,
        }
    }
}

impl GameRuleValueType for i32 {
    fn into_game_rule_value(value: Self) -> GameRuleValue {
        GameRuleValue::Integer(value)
    }

    fn from_game_rule_value(value: GameRuleValue) -> Option<Self> {
        match value {
            GameRuleValue::Boolean(_) => None,
            GameRuleValue::Integer(value) => Some(value),
        }
    }
}

impl std::fmt::Display for GameRuleValue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean(value) => value.fmt(formatter),
            Self::Integer(value) => value.fmt(formatter),
        }
    }
}

pub fn static_game_rules() -> &'static [AnyGameRule] {
    static GAME_RULES: LazyLock<Vec<AnyGameRule>> = LazyLock::new(|| {
        vec![
            bool_rule(GameRule::<bool>::ADVANCE_TIME),
            bool_rule(GameRule::<bool>::ADVANCE_WEATHER),
            bool_rule(GameRule::<bool>::ALLOW_ENTERING_NETHER_USING_PORTALS),
            bool_rule(GameRule::<bool>::BLOCK_DROPS),
            bool_rule(GameRule::<bool>::BLOCK_EXPLOSION_DROP_DECAY),
            bool_rule(GameRule::<bool>::COMMAND_BLOCKS_WORK),
            bool_rule(GameRule::<bool>::COMMAND_BLOCK_OUTPUT),
            bool_rule(GameRule::<bool>::DROWNING_DAMAGE),
            bool_rule(GameRule::<bool>::ELYTRA_MOVEMENT_CHECK),
            bool_rule(GameRule::<bool>::ENDER_PEARLS_VANISH_ON_DEATH),
            bool_rule(GameRule::<bool>::ENTITY_DROPS),
            bool_rule(GameRule::<bool>::FALL_DAMAGE),
            bool_rule(GameRule::<bool>::FIRE_DAMAGE),
            int_rule(GameRule::<i32>::FIRE_SPREAD_RADIUS_AROUND_PLAYER),
            bool_rule(GameRule::<bool>::FORGIVE_DEAD_PLAYERS),
            bool_rule(GameRule::<bool>::FREEZE_DAMAGE),
            bool_rule(GameRule::<bool>::GLOBAL_SOUND_EVENTS),
            bool_rule(GameRule::<bool>::IMMEDIATE_RESPAWN),
            bool_rule(GameRule::<bool>::KEEP_INVENTORY),
            bool_rule(GameRule::<bool>::LAVA_SOURCE_CONVERSION),
            bool_rule(GameRule::<bool>::LIMITED_CRAFTING),
            bool_rule(GameRule::<bool>::LOCATOR_BAR),
            bool_rule(GameRule::<bool>::LOG_ADMIN_COMMANDS),
            int_rule(GameRule::<i32>::MAX_BLOCK_MODIFICATIONS),
            int_rule(GameRule::<i32>::MAX_COMMAND_FORKS),
            int_rule(GameRule::<i32>::MAX_COMMAND_SEQUENCE_LENGTH),
            int_rule(GameRule::<i32>::MAX_ENTITY_CRAMMING),
            int_rule(GameRule::<i32>::MAX_MINECART_SPEED),
            int_rule(GameRule::<i32>::MAX_SNOW_ACCUMULATION_HEIGHT),
            bool_rule(GameRule::<bool>::MOB_DROPS),
            bool_rule(GameRule::<bool>::MOB_EXPLOSION_DROP_DECAY),
            bool_rule(GameRule::<bool>::MOB_GRIEFING),
            bool_rule(GameRule::<bool>::NATURAL_HEALTH_REGENERATION),
            bool_rule(GameRule::<bool>::PLAYER_MOVEMENT_CHECK),
            int_rule(GameRule::<i32>::PLAYERS_NETHER_PORTAL_CREATIVE_DELAY),
            int_rule(GameRule::<i32>::PLAYERS_NETHER_PORTAL_DEFAULT_DELAY),
            int_rule(GameRule::<i32>::PLAYERS_SLEEPING_PERCENTAGE),
            bool_rule(GameRule::<bool>::PROJECTILES_CAN_BREAK_BLOCKS),
            bool_rule(GameRule::<bool>::PVP),
            bool_rule(GameRule::<bool>::RAIDS),
            int_rule(GameRule::<i32>::RANDOM_TICK_SPEED),
            bool_rule(GameRule::<bool>::REDUCED_DEBUG_INFO),
            int_rule(GameRule::<i32>::RESPAWN_RADIUS),
            bool_rule(GameRule::<bool>::SEND_COMMAND_FEEDBACK),
            bool_rule(GameRule::<bool>::SHOW_ADVANCEMENT_MESSAGES),
            bool_rule(GameRule::<bool>::SHOW_DEATH_MESSAGES),
            bool_rule(GameRule::<bool>::SPAWNER_BLOCKS_WORK),
            bool_rule(GameRule::<bool>::SPAWN_MOBS),
            bool_rule(GameRule::<bool>::SPAWN_MONSTERS),
            bool_rule(GameRule::<bool>::SPAWN_PATROLS),
            bool_rule(GameRule::<bool>::SPAWN_PHANTOMS),
            bool_rule(GameRule::<bool>::SPAWN_WANDERING_TRADERS),
            bool_rule(GameRule::<bool>::SPAWN_WARDENS),
            bool_rule(GameRule::<bool>::SPECTATORS_GENERATE_CHUNKS),
            bool_rule(GameRule::<bool>::SPREAD_VINES),
            bool_rule(GameRule::<bool>::TNT_EXPLODES),
            bool_rule(GameRule::<bool>::TNT_EXPLOSION_DROP_DECAY),
            bool_rule(GameRule::<bool>::UNIVERSAL_ANGER),
            bool_rule(GameRule::<bool>::WATER_SOURCE_CONVERSION),
        ]
    });
    &GAME_RULES
}

pub fn get_static_game_rule(key: &str) -> Option<AnyGameRule> {
    static_game_rules()
        .iter()
        .copied()
        .find(|game_rule| game_rule.get_key() == key)
}

const fn bool_rule(game_rule: GameRule<bool>) -> AnyGameRule {
    AnyGameRule::new(
        game_rule.get_key(),
        game_rule.get_id(),
        GameRuleValue::Boolean(game_rule.get_default_value()),
    )
}

const fn int_rule(game_rule: GameRule<i32>) -> AnyGameRule {
    AnyGameRule::new(
        game_rule.get_key(),
        game_rule.get_id(),
        GameRuleValue::Integer(game_rule.get_default_value()),
    )
}

impl crate::world::World {
    pub fn get_game_rule<T: GameRuleValueType>(&self, game_rule: GameRule<T>) -> T {
        self.game_rules.get(game_rule)
    }

    pub fn set_game_rule<T: GameRuleValueType>(&mut self, game_rule: GameRule<T>, value: T) {
        self.game_rules.set(game_rule, value);
    }

    pub fn game_rule_value_strings(&self) -> BTreeMap<Identifier, String> {
        self.game_rules.value_strings()
    }
}
