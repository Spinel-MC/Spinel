use super::super::set_player_inventory::SetPlayerInventoryPacket;
use spinel_network::DataType;
use spinel_network::types::Slot;
use spinel_registry::{ItemStack, Material};
use spinel_utils::component::Component;

#[test]
fn configured_inventory_items_match_vanilla_optional_item_stack_shape() {
    let configured_items = [
        (0, Material::DIAMOND_PICKAXE),
        (1, Material::DIAMOND_HELMET),
        (2, Material::DIRT),
    ];

    configured_items.into_iter().for_each(|(slot, material)| {
        let packet = SetPlayerInventoryPacket {
            slot,
            item: Slot::from_item_stack(&ItemStack::of(material.clone())),
        };
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();

        let item_id = material.id();
        let expected_item_id = encode_var_int(item_id);
        let mut expected_payload = vec![slot as u8, 1];
        expected_payload.extend(expected_item_id);
        expected_payload.extend([0, 0]);

        assert_eq!(payload, expected_payload);
    });
}

#[test]
fn named_inventory_item_uses_registry_nbt_text_component_payload() {
    let named_item = ItemStack::of(Material::STICK)
        .with_custom_name(Component::text("Zombie Pathfinder").build());
    let packet = SetPlayerInventoryPacket {
        slot: 0,
        item: Slot::from_item_stack(&named_item),
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();

    let component_count_position = 2 + encode_var_int(Material::STICK.id()).len();
    assert_eq!(payload[component_count_position], 1);
    assert_eq!(payload[component_count_position + 1], 0);
    assert_eq!(payload[component_count_position + 2], 6);
    assert_eq!(payload[component_count_position + 3], 10);
}

fn encode_var_int(value: i32) -> Vec<u8> {
    let mut encoded = Vec::new();
    spinel_network::types::var_int::VarIntWrapper(value)
        .encode(&mut encoded)
        .unwrap();
    encoded
}
