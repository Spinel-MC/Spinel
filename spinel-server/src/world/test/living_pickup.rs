use crate::entity::player::{Player, PlayerChunk};
use crate::entity::{Entity, EntityPosition, GenericEntity, ItemEntity};
use crate::events::pickup_item::PickupItemEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use crate::world::World;
use spinel_core::network::clientbound::play::take_item_entity::TakeItemEntityPacket;
use spinel_macros::event_listener;
use spinel_network::types::Identifier;
use spinel_network::{ConnectionState, DataType};
use spinel_registry::{EntityType, ItemStack, Material, Registries};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use uuid::Uuid;

static LIVING_PICKUP_TEST_LOCK: Mutex<()> = Mutex::new(());
static LIVING_PICKUP_EVENT_CANCELLED: AtomicBool = AtomicBool::new(false);

#[event_listener]
fn living_pickup_listener(event: &mut PickupItemEvent, _server: &mut MinecraftServer) {
    event.set_cancelled(LIVING_PICKUP_EVENT_CANCELLED.load(Ordering::SeqCst));
}

#[test]
fn generic_living_pickup_refreshes_cooldown_sends_removal_and_unregisters_item() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let living_entity = positioned_living_entity();
    let living_entity_id = living_entity.entity_id();
    let item_entity = positioned_item_entity();
    let item_entity_id = item_entity.entity_id();
    world.add_entity(Entity::Generic(living_entity));
    world.add_entity(Entity::Item(item_entity));

    world.tick_with_registries(&Registries::new_vanilla());

    assert!(world.entity_by_id(item_entity_id).is_none());
    assert!(
        world
            .entity_tracker()
            .entity_by_id(item_entity_id)
            .is_none()
    );
    assert_eq!(
        world
            .entity_by_id(living_entity_id)
            .and_then(generic_entity)
            .unwrap()
            .item_pickup_cooldown(),
        5
    );
}

#[test]
fn cancelled_living_pickup_preserves_item_entity() {
    let _lock = LIVING_PICKUP_TEST_LOCK.lock().unwrap();
    LIVING_PICKUP_EVENT_CANCELLED.store(true, Ordering::SeqCst);
    let mut server = MinecraftServer::new();
    let world_uuid = server
        .world_manager
        .create_world(Identifier::minecraft("overworld"));
    let server_ptr = &mut server as *mut MinecraftServer as usize;
    let world = server.world_manager.world_mut(world_uuid).unwrap();
    world.use_server_event_dispatcher(server_ptr);
    let living_entity = positioned_living_entity();
    let item_entity = positioned_item_entity();
    let item_entity_id = item_entity.entity_id();
    world.add_entity(Entity::Generic(living_entity));
    world.add_entity(Entity::Item(item_entity));

    world.tick_with_registries(&Registries::new_vanilla());

    assert!(world.entity_by_id(item_entity_id).is_some());
    LIVING_PICKUP_EVENT_CANCELLED.store(false, Ordering::SeqCst);
}

#[test]
fn player_pickup_rejects_item_not_visible_to_player() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let player = positioned_player(25565);
    let item_entity = positioned_item_entity();
    let item_entity_id = item_entity.entity_id();
    world.add_entity(Entity::Player(player));
    world.add_entity(Entity::Item(item_entity));

    world.tick_with_registries(&Registries::new_vanilla());

    assert!(world.entity_by_id(item_entity_id).is_some());
}

#[test]
fn player_pickup_sends_collect_packet_and_removes_visible_item() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let mut player = positioned_player(viewer_client.addr.port());
    player.set_client(&mut viewer_client);
    player.mark_entered_world();
    player.mark_chunk_sent_to_client(PlayerChunk::new(0, 0));
    let player_id = player.entity_id();
    let mut item_entity = positioned_item_entity();
    let item_entity_id = item_entity.entity_id();
    item_entity.add_viewer(player_id);
    world.add_entity(Entity::Player(player));
    world.add_entity(Entity::Item(item_entity));

    world.tick_with_registries(&Registries::new_vanilla());

    let payload = viewer_client
        .queued_outbound_packet_payloads()
        .into_iter()
        .find_map(|(packet_id, payload)| {
            (packet_id == TakeItemEntityPacket::get_id()).then_some(payload)
        })
        .unwrap();
    let packet = TakeItemEntityPacket::decode(&mut payload.as_slice()).unwrap();
    assert_eq!(packet.collected_entity_id, item_entity_id.value());
    assert_eq!(packet.collector_entity_id, player_id.value());
    assert_eq!(packet.pickup_item_count, 3);
    assert!(world.entity_by_id(item_entity_id).is_none());
}

fn positioned_living_entity() -> GenericEntity {
    let mut entity = GenericEntity::new(EntityType::ZOMBIE);
    entity.set_position(EntityPosition::new(1.0, 64.0, 1.0, 0.0, 0.0));
    entity
}

fn positioned_item_entity() -> ItemEntity {
    let mut entity = ItemEntity::new(ItemStack::of(Material::DIAMOND).with_amount(3));
    entity.spawn(EntityPosition::new(1.0, 64.0, 1.0, 0.0, 0.0));
    entity
}

fn positioned_player(port: u16) -> Player {
    let mut player = Player::new(
        Uuid::new_v4(),
        "PickupPlayer".to_owned(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port),
    );
    player.set_position(EntityPosition::new(1.0, 64.0, 1.0, 0.0, 0.0));
    player
}

fn generic_entity(entity: &Entity) -> Option<&GenericEntity> {
    match entity {
        Entity::Generic(entity) => Some(entity),
        _ => None,
    }
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
