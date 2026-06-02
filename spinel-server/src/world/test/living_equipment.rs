use crate::entity::player::Player;
use crate::entity::{Entity, EntityId, EntityPosition, EquipmentSlot, GenericEntity};
use crate::events::entity_equip::EntityEquipEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::clientbound::play::set_entity_data::SetEntityDataPacket;
use spinel_core::network::clientbound::play::set_equipment::SetEquipmentPacket;
use spinel_core::network::clientbound::play::update_attributes::UpdateAttributesPacket;
use spinel_macros::event_listener;
use spinel_network::ConnectionState;
use spinel_network::types::Identifier;
use spinel_registry::{EntityType, ItemStack, Material};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::sync::Mutex;
use uuid::Uuid;

static LIVING_EQUIPMENT_TEST_LOCK: Mutex<()> = Mutex::new(());
static LIVING_EQUIPMENT_TEST_ENTITY: Mutex<Option<EntityId>> = Mutex::new(None);

#[event_listener]
fn living_equipment_listener(event: &mut EntityEquipEvent, _server: &mut MinecraftServer) {
    if *LIVING_EQUIPMENT_TEST_ENTITY.lock().unwrap() != Some(event.entity_id()) {
        return;
    }
    event.set_equipped_item(ItemStack::of(Material::DIAMOND));
}

#[test]
fn world_set_entity_equipment_applies_event_mutation_and_sends_equipment_then_attributes_to_viewer()
{
    let _lock = LIVING_EQUIPMENT_TEST_LOCK.lock().unwrap();
    let mut server = MinecraftServer::new();
    let world_uuid = server
        .world_manager
        .create_world(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let mut viewer = Player::new(
        Uuid::new_v4(),
        "EquipmentViewer".to_owned(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), viewer_client.addr.port()),
    );
    viewer.set_client(&mut viewer_client);
    viewer.mark_entered_world();
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(1.0, 64.0, 1.0, 0.0, 0.0));
    let entity_id = entity.entity_id();
    entity.add_viewer(viewer.entity_id());
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.add_entity(Entity::Generic(entity));
    world.add_entity(Entity::Player(viewer));
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.use_server_event_dispatcher(server_ptr);
    *LIVING_EQUIPMENT_TEST_ENTITY.lock().unwrap() = Some(entity_id);

    assert!(
        world
            .set_entity_equipment(
                entity_id,
                EquipmentSlot::MainHand,
                ItemStack::of(Material::DIRT),
            )
            .unwrap()
    );

    assert_eq!(
        world
            .entity_by_id(entity_id)
            .and_then(|entity| match entity {
                Entity::Generic(entity) => Some(entity),
                _ => None,
            })
            .unwrap()
            .equipment(EquipmentSlot::MainHand),
        &ItemStack::of(Material::DIAMOND)
    );
    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![
            SetEquipmentPacket::get_id(),
            UpdateAttributesPacket::get_id(),
        ]
    );
    *LIVING_EQUIPMENT_TEST_ENTITY.lock().unwrap() = None;
}

#[test]
fn world_set_player_held_slot_updates_self_attributes_and_viewer_equipment() {
    let mut server = MinecraftServer::new();
    let world_uuid = server
        .world_manager
        .create_world(Identifier::minecraft("overworld"));
    let mut player_client = queued_client();
    let mut viewer_client = queued_client();
    let mut player = Player::new(
        Uuid::new_v4(),
        "EquipmentPlayer".to_owned(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), player_client.addr.port()),
    );
    let mut viewer = Player::new(
        Uuid::new_v4(),
        "EquipmentViewer".to_owned(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), viewer_client.addr.port()),
    );
    player.set_client(&mut player_client);
    viewer.set_client(&mut viewer_client);
    player.mark_entered_world();
    viewer.mark_entered_world();
    player
        .inventory()
        .set_item_stack(1, ItemStack::of(Material::DIAMOND_PICKAXE));
    let player_id = player.entity_id();
    player.view_mut().automatic_add(viewer.entity_id());

    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.add_entity(Entity::Player(player));
    world.add_entity(Entity::Player(viewer));
    let server_ptr = &mut server as *mut MinecraftServer;
    player_client.discard_queued_outbound_packets();
    viewer_client.discard_queued_outbound_packets();

    assert!(
        server
            .world_manager
            .world_mut(world_uuid)
            .unwrap()
            .set_player_held_slot(&mut player_client, 1, server_ptr)
            .unwrap()
    );

    assert_eq!(
        server
            .world_manager
            .world(world_uuid)
            .unwrap()
            .entity_by_id(player_id)
            .and_then(|entity| match entity {
                Entity::Player(player) =>
                    Some(player.attribute_value(spinel_registry::Attribute::ATTACK_SPEED)),
                _ => None,
            })
            .unwrap(),
        1.2000000476837158
    );
    assert_eq!(
        player_client.queued_outbound_packet_ids(),
        vec![UpdateAttributesPacket::get_id()]
    );
    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![SetEquipmentPacket::get_id(), SetEntityDataPacket::get_id(),]
    );
}

fn queued_client() -> Client {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let stream = std::net::TcpStream::connect(addr).unwrap();
    let (_peer_stream, _) = listener.accept().unwrap();
    let mut client = Client::new(stream, addr);
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();
    client
}
