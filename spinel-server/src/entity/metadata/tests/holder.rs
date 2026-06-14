use super::super::holder::MetadataHolder;
use crate::entity::metadata::{MetadataByteMaskDefinition, MetadataDefinition, definitions};
use spinel_network::types::entity_metadata::MetadataValue;

#[test]
fn metadata_holder_returns_minestom_default_when_entry_is_unset() {
    let metadata = MetadataHolder::default();

    assert_eq!(
        metadata.value(&definitions::air_ticks()),
        MetadataValue::VarInt(300)
    );
}

#[test]
fn metadata_holder_tracks_dirty_entries_until_drain() {
    let mut metadata = MetadataHolder::default();

    metadata.set(&definitions::air_ticks(), MetadataValue::VarInt(10));
    let dirty_entries = metadata.drain_dirty_entries();

    assert_eq!(dirty_entries.len(), 1);
    assert_eq!(dirty_entries[0].index, 1);
    assert!(metadata.drain_dirty_entries().is_empty());
}

#[test]
fn metadata_holder_retains_latest_changes_while_notifications_are_disabled() {
    let mut metadata = MetadataHolder::default();

    metadata.set_change_notifications_enabled(false);
    metadata.set(&definitions::air_ticks(), MetadataValue::VarInt(10));
    metadata.set(&definitions::air_ticks(), MetadataValue::VarInt(20));

    assert!(!metadata.change_notifications_are_enabled());
    assert!(metadata.drain_dirty_entries().is_empty());

    metadata.set_change_notifications_enabled(true);
    let dirty_entries = metadata.drain_dirty_entries();

    assert_eq!(dirty_entries.len(), 1);
    assert_eq!(dirty_entries[0].index, definitions::air_ticks().index());
    assert_eq!(dirty_entries[0].value, MetadataValue::VarInt(20));
}

#[test]
fn metadata_holder_bitmask_entries_share_the_base_byte() {
    let mut metadata = MetadataHolder::default();

    metadata.set_flag(&definitions::is_on_fire(), true);
    metadata.set_flag(&definitions::is_crouching(), true);

    assert!(metadata.flag(&definitions::is_on_fire()));
    assert!(metadata.flag(&definitions::is_crouching()));
    assert_eq!(
        metadata.value(&definitions::entity_flags()),
        MetadataValue::Byte(0x03)
    );
}

#[test]
fn metadata_holder_byte_mask_preserves_neighboring_bits() {
    let alignment = MetadataByteMaskDefinition::new(4, 0x18, 3, 0);
    let flags = MetadataDefinition::new(4, MetadataValue::Byte(0));
    let mut metadata = MetadataHolder::default();
    metadata.set(&flags, MetadataValue::Byte(0x21));

    metadata.set_byte(&alignment, 2);

    assert_eq!(metadata.byte(&alignment), 2);
    assert_eq!(metadata.value(&flags), MetadataValue::Byte(0x31));
}

#[test]
fn player_metadata_definitions_include_avatar_and_living_offsets() {
    assert_eq!(definitions::living_entity::health().index(), 9);
    assert_eq!(definitions::avatar::main_hand().index(), 15);
    assert_eq!(definitions::player::additional_hearts().index(), 17);
    assert_eq!(
        definitions::player::right_shoulder_entity_data().index(),
        20
    );
}

#[test]
fn inherited_animal_metadata_indexes_match_minestom() {
    assert_eq!(definitions::ageable_mob::is_baby().index(), 16);
    assert_eq!(definitions::tameable_animal::owner().index(), 18);
    assert_eq!(definitions::wolf::sound_variant().index(), 23);
    assert_eq!(definitions::zombie_nautilus::variant().index(), 20);
}

#[test]
fn display_and_vehicle_metadata_include_superclass_offsets() {
    assert_eq!(definitions::display::translation().index(), 11);
    assert_eq!(definitions::text_display::alignment().index(), 27);
    assert_eq!(definitions::boat::splash_timer().index(), 13);
    assert_eq!(
        definitions::command_block_minecart::last_output().index(),
        14
    );
}

#[test]
fn golem_and_monster_metadata_include_superclass_offsets() {
    assert_eq!(definitions::copper_golem::state().index(), 17);
    assert_eq!(definitions::piglin::is_dancing().index(), 19);
    assert_eq!(definitions::pillager::is_charging().index(), 17);
    assert_eq!(definitions::zombie::is_becoming_drowned().index(), 18);
}
