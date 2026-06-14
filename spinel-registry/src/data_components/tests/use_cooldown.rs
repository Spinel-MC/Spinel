use super::super::use_cooldown::UseCooldown;
use crate::DataComponentValue;
use spinel_nbt::{Nbt, NbtCompound};

#[test]
fn use_cooldown_decodes_seconds_and_optional_group_from_extracted_component_nbt() {
    let mut compound = NbtCompound::new();
    compound.insert("seconds".to_string(), Nbt::Double(1.5));
    compound.insert(
        "cooldown_group".to_string(),
        Nbt::String("minecraft:ender_pearl".to_string()),
    );

    let use_cooldown = UseCooldown::from_component_nbt(&Nbt::Compound(compound)).unwrap();

    assert_eq!(use_cooldown.seconds(), 1.5);
    assert_eq!(use_cooldown.cooldown_group(), Some("minecraft:ender_pearl"));
    assert_eq!(use_cooldown.ticks(), 30);
}
