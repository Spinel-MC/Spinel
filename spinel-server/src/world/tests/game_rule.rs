use super::super::instance::World;
use crate::world::{
    GameRule, GameRuleRequestEntry, GameRuleValue, get_static_game_rule, static_game_rules,
};
use spinel_registry::dimension_type::DimensionType;
use uuid::Uuid;

#[test]
fn world_game_rules_use_vanilla_combat_defaults() {
    let world = World::new(Uuid::new_v4(), DimensionType::OVERWORLD);

    assert!(world.get_game_rule(GameRule::<bool>::PVP));
    assert!(world.get_game_rule(GameRule::<bool>::FALL_DAMAGE));
}

#[test]
fn world_game_rules_store_world_local_values() {
    let mut first_world = World::new(Uuid::new_v4(), DimensionType::OVERWORLD);
    let second_world = World::new(Uuid::new_v4(), DimensionType::OVERWORLD);

    first_world.set_game_rule(GameRule::<bool>::PVP, false);
    first_world.set_game_rule(GameRule::<bool>::FALL_DAMAGE, false);
    first_world.set_game_rule(GameRule::<i32>::RANDOM_TICK_SPEED, 7);

    assert!(!first_world.get_game_rule(GameRule::<bool>::PVP));
    assert!(!first_world.get_game_rule(GameRule::<bool>::FALL_DAMAGE));
    assert_eq!(
        first_world.get_game_rule(GameRule::<i32>::RANDOM_TICK_SPEED),
        7
    );
    assert!(second_world.get_game_rule(GameRule::<bool>::PVP));
    assert!(second_world.get_game_rule(GameRule::<bool>::FALL_DAMAGE));
    assert_eq!(
        second_world.get_game_rule(GameRule::<i32>::RANDOM_TICK_SPEED),
        3
    );
}

#[test]
fn static_game_rule_registry_exposes_vanilla_names_and_defaults() {
    let pvp_rule = get_static_game_rule("pvp").expect("pvp game rule should exist");
    let random_tick_speed_rule = get_static_game_rule("random_tick_speed")
        .expect("random tick speed game rule should exist");

    assert_eq!(static_game_rules().len(), 59);
    assert_eq!(pvp_rule.get_key(), "pvp");
    assert_eq!(pvp_rule.get_id(), 39);
    assert_eq!(pvp_rule.get_default_value(), GameRuleValue::Boolean(true));
    assert_eq!(random_tick_speed_rule.get_key(), "random_tick_speed");
    assert_eq!(random_tick_speed_rule.get_id(), 41);
    assert_eq!(
        random_tick_speed_rule.get_default_value(),
        GameRuleValue::Integer(3)
    );
}

#[test]
fn game_rule_request_entry_preserves_requested_string_value() {
    let pvp_rule = GameRule::<bool>::PVP.erase();
    let request_entry = GameRuleRequestEntry::new(pvp_rule, "false");

    assert_eq!(request_entry.get_game_rule(), pvp_rule);
    assert_eq!(request_entry.get_value(), "false");
}

#[test]
fn unknown_static_game_rule_lookup_returns_none() {
    assert_eq!(get_static_game_rule("missing_rule"), None);
}
