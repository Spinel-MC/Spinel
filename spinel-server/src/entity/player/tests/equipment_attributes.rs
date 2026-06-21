use crate::entity::{EquipmentSlot, Player};
use crate::network::client::instance::Client;
use spinel_core::network::clientbound::play::set_player_inventory::SetPlayerInventoryPacket;
use spinel_core::network::clientbound::play::update_attributes::UpdateAttributesPacket;
use spinel_network::ConnectionState;
use spinel_registry::data_components::vanilla_components::ATTRIBUTE_MODIFIERS;
use spinel_registry::data_components::{
    AttributeModifierDisplay, AttributeModifierEntry, AttributeOperation, EquipmentSlotGroup,
};
use spinel_registry::{Attribute, AttributeList, ItemStack, Material};
use std::net::TcpListener;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use uuid::Uuid;

fn player() -> Player {
    Player::new(
        Uuid::new_v4(),
        "living_attributes".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    )
}

#[test]
fn equipment_attributes_apply_vanilla_main_hand_modifier() {
    let mut player = player();

    assert!(player.set_equipment(
        EquipmentSlot::MainHand,
        ItemStack::of(Material::DIAMOND_PICKAXE),
    ));

    assert_eq!(
        player.get_attribute_value(Attribute::ATTACK_SPEED),
        1.2000000476837158
    );
}

#[test]
fn equipment_attributes_replace_old_modifier_for_changed_slot() {
    let mut player = player();
    let main_hand = |amount| {
        ItemStack::of(Material::STONE).with(
            ATTRIBUTE_MODIFIERS,
            AttributeList::new(vec![AttributeModifierEntry::new(
                Attribute::ATTACK_SPEED.identifier(),
                "minecraft:test_speed".parse().unwrap(),
                amount,
                AttributeOperation::AddValue,
                EquipmentSlotGroup::MainHand,
                AttributeModifierDisplay::Default,
            )]),
        )
    };

    assert!(player.set_equipment(EquipmentSlot::MainHand, main_hand(-1.0)));
    assert_eq!(player.get_attribute_value(Attribute::ATTACK_SPEED), 3.0);

    assert!(player.set_equipment(EquipmentSlot::MainHand, main_hand(-2.0)));
    assert_eq!(player.get_attribute_value(Attribute::ATTACK_SPEED), 2.0);
}

#[test]
fn login_attribute_sync_applies_preconfigured_selected_slot_item() {
    let mut player = player();
    let mut client = queued_client();
    assert!(
        player
            .get_inventory()
            .set_item_stack(0, ItemStack::of(Material::DIAMOND_PICKAXE))
    );

    player.sync_main_hand_attributes(&mut client).unwrap();

    assert_eq!(
        player.get_attribute_value(Attribute::ATTACK_SPEED),
        1.2000000476837158
    );
    assert_eq!(
        client.queued_outbound_packet_ids(),
        vec![UpdateAttributesPacket::get_id()]
    );
}

#[test]
fn active_equipment_change_syncs_inventory_slot_and_attributes() {
    let mut player = player();
    let mut client = queued_client();
    player.set_client(&mut client);

    assert!(player.set_equipment(
        EquipmentSlot::MainHand,
        ItemStack::of(Material::DIAMOND_PICKAXE),
    ));

    assert_eq!(
        player.get_attribute_value(Attribute::ATTACK_SPEED),
        1.2000000476837158
    );
    assert_eq!(
        client.queued_outbound_packet_ids(),
        vec![
            SetPlayerInventoryPacket::get_id(),
            UpdateAttributesPacket::get_id(),
        ]
    );
}

fn queued_client() -> Client {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let stream = std::net::TcpStream::connect(addr).unwrap();
    let _ = listener.accept().unwrap();
    let mut client = Client::new(stream, addr);
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();
    client
}
