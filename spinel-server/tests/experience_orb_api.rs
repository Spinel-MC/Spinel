use spinel_network::types::entity_metadata::MetadataValue;
use spinel_server::entity::ExperienceOrb;
use spinel_server::entity::metadata::definitions;

#[test]
fn experience_orb_count_api_matches_minestom_public_getter_and_setter() {
    let mut experience_orb = ExperienceOrb::new(3);

    experience_orb.set_experience_count(19);

    assert_eq!(experience_orb.experience_count(), 19);
    assert_eq!(
        experience_orb
            .metadata()
            .value(&definitions::experience_orb::value()),
        MetadataValue::VarInt(19)
    );
}
