use crate::network::serverbound::play::set_creative_mode_slot::SetCreativeModeSlotPacket;
use spinel_nbt::{Nbt, NbtCompound};
use spinel_network::DataType;
use spinel_network::types::Slot;
use spinel_network::types::var_int::VarIntWrapper;
use spinel_registry::data_components::vanilla_components::{CUSTOM_DATA, CUSTOM_NAME};
use spinel_registry::{ItemStack, Material};
use spinel_utils::component::Component;

#[test]
fn creative_mode_slot_decodes_length_prefixed_item_stack_and_preserves_custom_data() {
    let mut custom_data = NbtCompound::new();
    custom_data.insert("spinel:pathfinder".to_string(), Nbt::Int(1));
    let item_stack = ItemStack::of(Material::STICK)
        .with(CUSTOM_DATA, custom_data.clone())
        .with_custom_name(Component::text("Zombie Pathfinder").build());
    let mut slot = Slot::from_item_stack(&item_stack);
    let custom_name = slot
        .components
        .added
        .iter_mut()
        .find(|component| component.type_id == CUSTOM_NAME.id())
        .unwrap();
    custom_name.data.clear();
    Nbt::String("Zombie Pathfinder".to_string())
        .write_unnamed(&mut custom_name.data)
        .unwrap();

    let mut packet_payload = Vec::new();
    36_i16.encode(&mut packet_payload).unwrap();
    VarIntWrapper(slot.count)
        .encode(&mut packet_payload)
        .unwrap();
    VarIntWrapper(slot.item_id)
        .encode(&mut packet_payload)
        .unwrap();
    VarIntWrapper(slot.components.added.len() as i32)
        .encode(&mut packet_payload)
        .unwrap();
    VarIntWrapper(slot.components.removed.len() as i32)
        .encode(&mut packet_payload)
        .unwrap();
    slot.components.added.iter().for_each(|component| {
        VarIntWrapper(component.type_id)
            .encode(&mut packet_payload)
            .unwrap();
        VarIntWrapper(component.data.len() as i32)
            .encode(&mut packet_payload)
            .unwrap();
        packet_payload.extend_from_slice(&component.data);
    });
    slot.components.removed.iter().for_each(|component_id| {
        VarIntWrapper(*component_id)
            .encode(&mut packet_payload)
            .unwrap();
    });

    let packet = SetCreativeModeSlotPacket::decode(&mut packet_payload.as_slice()).unwrap();
    let decoded_item_stack = packet.clicked_item.to_item_stack();

    assert_eq!(*decoded_item_stack.material(), Material::STICK);
    assert_eq!(decoded_item_stack.get(CUSTOM_DATA), Some(custom_data));
    assert_eq!(
        decoded_item_stack
            .get(CUSTOM_NAME)
            .map(|component| component.to_plain_string()),
        Some("Zombie Pathfinder".to_string())
    );
}
