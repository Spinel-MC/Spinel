use crate::{Identifier, VillagerProfession, VillagerType};

#[test]
fn extracted_villager_static_protocol_values_match_vanilla_ids() {
    assert_eq!(VillagerType::DESERT.protocol_id(), 0);
    assert_eq!(VillagerType::TAIGA.protocol_id(), 6);
    assert_eq!(VillagerProfession::NONE.protocol_id(), 0);
    assert_eq!(VillagerProfession::WEAPONSMITH.protocol_id(), 14);
    assert_eq!(
        VillagerType::from_key(&Identifier::vanilla_static("plains")),
        Some(VillagerType::PLAINS)
    );
    assert_eq!(
        VillagerProfession::from_protocol_id(9),
        Some(VillagerProfession::LIBRARIAN)
    );
}
