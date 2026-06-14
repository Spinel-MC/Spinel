use crate::entity::{Entity, GenericEntity, Player};
use crate::network::client::instance::Client;
use crate::world::World;
use spinel_core::network::clientbound::play::set_entity_data::SetEntityDataPacket;
use spinel_network::ConnectionState;
use spinel_network::types::Identifier;
use spinel_registry::{EntityType, Registries};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use uuid::Uuid;

#[test]
fn world_tick_automatically_broadcasts_dirty_generic_entity_metadata() {
    let mut world = World::new(Identifier::minecraft("overworld"));
    let mut viewer_client = queued_client();
    let viewer = entered_player(&mut viewer_client);
    let target = GenericEntity::new(EntityType::ZOMBIE);
    let target_id = target.entity_id();
    world.add_entity(Entity::Player(viewer));
    world.add_entity(Entity::Generic(target));
    world.process_pending_entity_visibility_refreshes().unwrap();
    viewer_client.discard_queued_outbound_packets();
    let Some(Entity::Generic(target)) = world.entity_by_id_mut(target_id) else {
        panic!("generic entity missing");
    };
    target.set_glowing(true);

    world.tick_with_registries(&Registries::new_vanilla());

    assert_eq!(
        viewer_client.queued_outbound_packet_ids(),
        vec![SetEntityDataPacket::get_id()]
    );
}

fn entered_player(client: &mut Client) -> Player {
    let mut player = Player::new(
        Uuid::new_v4(),
        "MetadataViewer".to_owned(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), client.addr.port()),
    );
    player.set_client(client);
    player.mark_entered_world();
    player
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
