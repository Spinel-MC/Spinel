use crate::entity::{Player, PlayerHand};
use crate::world::Block;
use spinel_registry::data_components::vanilla_components::TOOL;
use spinel_registry::mob_effect::MobEffect;
use spinel_registry::{
    Attribute, Identifier, Material, Registries, RegistryTagReference, Tool, ToolRule,
};

pub(crate) const UNBREAKABLE: i32 = -1;

pub(crate) fn break_ticks(
    block: Block,
    player: &Player,
    registries: &Registries,
    player_is_in_water: bool,
) -> i32 {
    if player.get_game_mode() == spinel_core::entity::game_mode::GameMode::Creative {
        return 0;
    }

    let block_hardness = block.hardness();
    if block_hardness == -1.0 {
        return UNBREAKABLE;
    }

    let item_stack = player.item_in_hand(PlayerHand::Main);
    let material = item_stack.material();
    if bamboo_is_instant_with_sword(block, material, registries) {
        return 0;
    }

    let tool = item_stack.get::<Tool>(TOOL);
    let is_best_tool = can_break_block(tool.as_ref(), block, registries);
    let speed_multiplier = speed_multiplier(
        block,
        player,
        registries,
        tool.as_ref(),
        is_best_tool,
        player_is_in_water,
    );
    if speed_multiplier == 0.0 {
        return UNBREAKABLE;
    }
    if block_hardness == 0.0 {
        return 0;
    }

    let divisor = if is_best_tool { 30.0 } else { 100.0 };
    let damage = f64::from(speed_multiplier) / f64::from(block_hardness) / divisor;
    if damage >= 1.0 {
        return 0;
    }
    (1.0 / damage).ceil() as i32
}

fn bamboo_is_instant_with_sword(
    block: Block,
    material: &Material,
    registries: &Registries,
) -> bool {
    if block != Block::BAMBOO && block != Block::BAMBOO_SAPLING {
        return false;
    }
    registries.item_tag_contains(&Identifier::vanilla_static("swords"), material)
}

fn speed_multiplier(
    block: Block,
    player: &Player,
    registries: &Registries,
    tool: Option<&Tool>,
    is_best_tool: bool,
    player_is_in_water: bool,
) -> f32 {
    let mut speed_multiplier = if is_best_tool {
        mining_speed(tool, block, registries)
    } else {
        1.0
    };

    if is_best_tool && speed_multiplier > 1.0 {
        speed_multiplier += player.get_attribute_value(Attribute::MINING_EFFICIENCY) as f32;
    }
    if player_has_effect(player, registries, &MobEffect::HASTE)
        || player_has_effect(player, registries, &MobEffect::CONDUIT_POWER)
    {
        speed_multiplier *= haste_multiplier(player, registries);
    }
    if player_has_effect(player, registries, &MobEffect::MINING_FATIGUE) {
        speed_multiplier *= mining_fatigue_multiplier(player, registries);
    }

    speed_multiplier *= player.get_attribute_value(Attribute::BLOCK_BREAK_SPEED) as f32;
    if player_is_in_water {
        speed_multiplier *= player.get_attribute_value(Attribute::SUBMERGED_MINING_SPEED) as f32;
    }
    if player.is_on_ground() {
        return speed_multiplier;
    }
    speed_multiplier / 5.0
}

fn mining_speed(tool: Option<&Tool>, block: Block, registries: &Registries) -> f32 {
    let Some(tool) = tool else {
        return 1.0;
    };
    matching_tool_rule(tool, block, registries)
        .and_then(ToolRule::speed)
        .unwrap_or_else(|| tool.default_mining_speed())
}

fn can_break_block(tool: Option<&Tool>, block: Block, registries: &Registries) -> bool {
    if !block.requires_tool() {
        return true;
    }
    is_effective(tool, block, registries)
}

fn is_effective(tool: Option<&Tool>, block: Block, registries: &Registries) -> bool {
    matching_tool_rule_option(tool, block, registries)
        .and_then(ToolRule::correct_for_drops)
        .unwrap_or(false)
}

fn matching_tool_rule_option<'a>(
    tool: Option<&'a Tool>,
    block: Block,
    registries: &Registries,
) -> Option<&'a ToolRule> {
    tool.and_then(|tool| matching_tool_rule(tool, block, registries))
}

fn matching_tool_rule<'a>(
    tool: &'a Tool,
    block: Block,
    registries: &Registries,
) -> Option<&'a ToolRule> {
    tool.rules()
        .iter()
        .find(|rule| tool_rule_contains_block(rule, block, registries))
}

fn tool_rule_contains_block(rule: &ToolRule, block: Block, registries: &Registries) -> bool {
    match rule.blocks() {
        RegistryTagReference::Backed(_) | RegistryTagReference::Direct(_) => {
            rule.blocks().contains_block(block, registries)
        }
        RegistryTagReference::Empty => false,
    }
}

fn player_has_effect(
    player: &Player,
    registries: &Registries,
    effect: &spinel_registry::RegistryKey<MobEffect>,
) -> bool {
    registries
        .mob_effect_id(effect)
        .is_some_and(|effect_id| player.has_effect(effect_id))
}

fn player_effect_level(
    player: &Player,
    registries: &Registries,
    effect: &spinel_registry::RegistryKey<MobEffect>,
) -> i32 {
    registries
        .mob_effect_id(effect)
        .and_then(|effect_id| player.get_effect_level(effect_id))
        .unwrap_or(-1)
}

fn haste_multiplier(player: &Player, registries: &Registries) -> f32 {
    let haste_level = player_effect_level(player, registries, &MobEffect::HASTE);
    let conduit_level = player_effect_level(player, registries, &MobEffect::CONDUIT_POWER);
    1.0 + 0.2 * ((haste_level.max(conduit_level) + 1) as f32)
}

fn mining_fatigue_multiplier(player: &Player, registries: &Registries) -> f32 {
    match player_effect_level(player, registries, &MobEffect::MINING_FATIGUE) + 1 {
        0 => 0.0,
        1 => 0.3,
        2 => 0.09,
        3 => 0.027,
        _ => 0.0081,
    }
}
