use super::super::advancements::{AdvancementFrameType, AdvancementsPacket, Notification};
use spinel_network::DataType;
use spinel_registry::Material;
use spinel_utils::component::Component;

#[test]
fn notification_builds_minestom_advancement_add_and_remove_packets() {
    let notification = Notification::from_material(
        Component::text("Done").build(),
        AdvancementFrameType::Challenge,
        Material::DIAMOND,
    );
    let add_packet = notification.add_packet(42);
    let remove_packet = notification.remove_packet();
    let mut payload = Vec::new();

    add_packet.encode(&mut payload).unwrap();
    let decoded_packet = AdvancementsPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(AdvancementsPacket::get_id(), 0x80);
    assert_eq!(
        decoded_packet.advancement_mappings[0].key,
        "minestom:notification"
    );
    assert_eq!(
        decoded_packet.progress_mappings[0].progress.criteria[0]
            .progress
            .date_of_achieving,
        Some(42)
    );
    assert_eq!(
        remove_packet.identifiers_to_remove,
        vec!["minestom:notification"]
    );
}
