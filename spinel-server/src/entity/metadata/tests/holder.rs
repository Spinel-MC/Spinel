use super::super::holder::MetadataHolder;
use crate::entity::metadata::{MetadataByteMaskDefinition, MetadataDefinition, definitions};
use spinel_network::types::entity_metadata::MetadataValue;

#[test]
fn metadata_holder_returns_minestom_default_when_entry_is_unset() {
    let metadata = MetadataHolder::default();

    assert_eq!(
        metadata.get_value(&definitions::get_air_ticks()),
        MetadataValue::VarInt(300)
    );
}

#[test]
fn metadata_holder_tracks_dirty_entries_until_drain() {
    let mut metadata = MetadataHolder::default();

    metadata.set(&definitions::get_air_ticks(), MetadataValue::VarInt(10));
    let dirty_entries = metadata.drain_dirty_entries();

    assert_eq!(dirty_entries.len(), 1);
    assert_eq!(dirty_entries[0].index, 1);
    assert!(metadata.drain_dirty_entries().is_empty());
}

#[test]
fn metadata_holder_retains_latest_changes_while_notifications_are_disabled() {
    let mut metadata = MetadataHolder::default();

    metadata.set_change_notifications_enabled(false);
    metadata.set(&definitions::get_air_ticks(), MetadataValue::VarInt(10));
    metadata.set(&definitions::get_air_ticks(), MetadataValue::VarInt(20));

    assert!(!metadata.has_change_notifications_enabled());
    assert!(metadata.drain_dirty_entries().is_empty());

    metadata.set_change_notifications_enabled(true);
    let dirty_entries = metadata.drain_dirty_entries();

    assert_eq!(dirty_entries.len(), 1);
    assert_eq!(
        dirty_entries[0].index,
        definitions::get_air_ticks().get_index()
    );
    assert_eq!(dirty_entries[0].value, MetadataValue::VarInt(20));
}

#[test]
fn metadata_holder_bitmask_entries_share_the_base_byte() {
    let mut metadata = MetadataHolder::default();

    metadata.set_flag(&definitions::is_on_fire(), true);
    metadata.set_flag(&definitions::is_crouching(), true);

    assert!(metadata.get_flag(&definitions::is_on_fire()));
    assert!(metadata.get_flag(&definitions::is_crouching()));
    assert_eq!(
        metadata.get_value(&definitions::entity_flags()),
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

    assert_eq!(metadata.get_byte(&alignment), 2);
    assert_eq!(metadata.get_value(&flags), MetadataValue::Byte(0x31));
}

#[test]
fn player_metadata_definitions_include_avatar_and_living_offsets() {
    assert_eq!(definitions::living_entity::get_health().get_index(), 9);
    assert_eq!(definitions::avatar::get_main_hand().get_index(), 15);
    assert_eq!(definitions::player::get_additional_hearts().get_index(), 17);
    assert_eq!(
        definitions::player::get_right_shoulder_entity_data().get_index(),
        20
    );
}

#[test]
fn inherited_animal_metadata_indexes_match_minestom() {
    assert_eq!(definitions::ageable_mob::is_baby().get_index(), 16);
    assert_eq!(definitions::tameable_animal::get_owner().get_index(), 18);
    assert_eq!(definitions::wolf::get_sound_variant().get_index(), 23);
    assert_eq!(definitions::zombie_nautilus::get_variant().get_index(), 20);
}

#[test]
fn display_and_vehicle_metadata_include_superclass_offsets() {
    assert_eq!(definitions::display::get_translation().get_index(), 11);
    assert_eq!(definitions::text_display::get_alignment().get_index(), 27);
    assert_eq!(definitions::boat::get_splash_timer().get_index(), 13);
    assert_eq!(
        definitions::command_block_minecart::get_last_output().get_index(),
        14
    );
}

#[test]
fn golem_and_monster_metadata_include_superclass_offsets() {
    assert_eq!(definitions::copper_golem::get_state().get_index(), 17);
    assert_eq!(definitions::piglin::is_dancing().get_index(), 19);
    assert_eq!(definitions::pillager::is_charging().get_index(), 17);
    assert_eq!(definitions::zombie::is_becoming_drowned().get_index(), 18);
}
