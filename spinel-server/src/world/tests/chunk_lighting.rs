use crate::entity::{Entity, Player};
use crate::network::client::instance::Client;
use crate::world::{Block, BlockPosition, Chunk, ChunkPosition, World};
use spinel_core::network::clientbound::play::light_update::LightUpdatePacket;
use spinel_network::ConnectionState;
use spinel_network::types::Identifier;
use spinel_registry::dimension_type::DimensionType;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use uuid::Uuid;

#[test]
fn partial_light_data_contains_only_invalidated_sections_and_is_consumed_once() {
    let mut chunk = Chunk::new_lighting(ChunkPosition::new(0, 0));

    chunk.set_block(BlockPosition::new(1, 64, 1), Block::TORCH);
    let light_data = chunk.partial_light_data().unwrap();

    assert_eq!(light_data.sky_light_mask, vec![1 << 9]);
    assert_eq!(light_data.block_light_mask, vec![1 << 9]);
    assert_eq!(light_data.sky_light_arrays.len(), 1);
    assert_eq!(light_data.block_light_arrays.len(), 1);
    assert!(chunk.partial_light_data().is_none());
}

#[test]
fn chunk_storage_preserves_concrete_block_states() {
    let mut chunk = Chunk::new(ChunkPosition::new(0, 0));
    let position = BlockPosition::new(1, 64, 1);
    let east_stairs = Block::OAK_STAIRS
        .default_state()
        .with_property("facing", "east")
        .expect("oak stairs must expose an east-facing state");

    assert!(chunk.set_block_state(position, east_stairs));
    assert_eq!(chunk.block(position), Block::OAK_STAIRS);
    assert_eq!(chunk.block_state(position), east_stairs);
}

#[test]
fn block_light_propagates_with_minestom_attenuation() {
    let mut world = lighting_world("block_light_propagation");
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    world
        .set_block(BlockPosition::new(1, 64, 1), Block::TORCH)
        .unwrap();

    assert_eq!(world.block_light(BlockPosition::new(1, 64, 1)), 14);
    assert_eq!(world.block_light(BlockPosition::new(2, 64, 1)), 13);
    assert_eq!(world.block_light(BlockPosition::new(3, 64, 1)), 12);
}

#[test]
fn full_block_faces_stop_light_propagation() {
    let mut world = lighting_world("block_light_occlusion");
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    world
        .set_block(BlockPosition::new(1, 64, 1), Block::TORCH)
        .unwrap();
    world
        .set_block(BlockPosition::new(2, 64, 1), Block::STONE)
        .unwrap();

    assert_eq!(world.block_light(BlockPosition::new(2, 64, 1)), 0);
}

#[test]
fn block_light_crosses_loaded_chunk_boundaries() {
    let mut world = lighting_world("block_light_chunk_boundary");
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    world.load_chunk(ChunkPosition::new(1, 0)).unwrap();
    world
        .set_block(BlockPosition::new(15, 64, 1), Block::TORCH)
        .unwrap();

    assert_eq!(world.block_light(BlockPosition::new(16, 64, 1)), 13);
}

#[test]
fn skylight_uses_concrete_state_occlusion() {
    let mut world = lighting_world("sky_light_occlusion");
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    world
        .set_block(BlockPosition::new(1, 70, 1), Block::STONE)
        .unwrap();

    assert_eq!(world.sky_light(BlockPosition::new(1, 71, 1)), 15);
    assert_eq!(world.sky_light(BlockPosition::new(1, 70, 1)), 0);
}

#[test]
fn removing_a_source_removes_propagated_block_light() {
    let mut world = lighting_world("block_light_removal");
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    let source = BlockPosition::new(1, 64, 1);
    let neighbor = BlockPosition::new(2, 64, 1);
    world.set_block(source, Block::TORCH).unwrap();
    assert_eq!(world.block_light(neighbor), 13);

    world.set_block(source, Block::AIR).unwrap();

    assert_eq!(world.block_light(neighbor), 0);
}

#[test]
fn block_light_crosses_section_boundaries() {
    let mut world = lighting_world("block_light_section_boundary");
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    world
        .set_block(BlockPosition::new(1, 63, 1), Block::TORCH)
        .unwrap();

    assert_eq!(world.block_light(BlockPosition::new(1, 64, 1)), 13);
}

#[test]
fn block_light_crosses_negative_chunk_boundaries() {
    let mut world = lighting_world("block_light_negative_boundary");
    world.load_chunk(ChunkPosition::new(-1, 0)).unwrap();
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    world
        .set_block(BlockPosition::new(-1, 64, 1), Block::TORCH)
        .unwrap();

    assert_eq!(world.block_light(BlockPosition::new(0, 64, 1)), 13);
}

#[test]
fn light_does_not_propagate_into_unloaded_chunks() {
    let mut world = lighting_world("block_light_unloaded_neighbor");
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    world
        .set_block(BlockPosition::new(15, 64, 1), Block::TORCH)
        .unwrap();

    assert_eq!(world.block_light(BlockPosition::new(16, 64, 1)), 0);
}

#[test]
fn opaque_block_removal_restores_propagated_light() {
    let mut world = lighting_world("block_light_blocker_removal");
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    let blocker = BlockPosition::new(2, 64, 1);
    world
        .set_block(BlockPosition::new(1, 64, 1), Block::TORCH)
        .unwrap();
    world.set_block(blocker, Block::STONE).unwrap();
    assert_eq!(world.block_light(blocker), 0);

    world.set_block(blocker, Block::AIR).unwrap();

    assert_eq!(world.block_light(blocker), 13);
}

#[test]
fn skylight_shaft_reopens_after_occluder_removal() {
    let mut world = lighting_world("sky_light_shaft");
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    let occluder = BlockPosition::new(8, 70, 8);
    world.set_block(occluder, Block::STONE).unwrap();
    assert_eq!(world.sky_light(occluder), 0);

    world.set_block(occluder, Block::AIR).unwrap();

    assert_eq!(world.sky_light(occluder), 15);
}

#[test]
fn dimensions_without_skylight_keep_sky_light_empty() {
    let dimension = DimensionType::builder().skylight(false).build();
    let mut world = World::new_with_dimension(
        Identifier::minecraft("no_skylight"),
        DimensionType::OVERWORLD,
        dimension,
    );
    world.set_chunk_supplier(Chunk::new_lighting);
    world.load_chunk(ChunkPosition::new(0, 0)).unwrap();
    world.invalidate_section(0, 4, 0);

    assert_eq!(world.sky_light(BlockPosition::new(1, 64, 1)), 0);
}

#[test]
fn chunk_occlusion_map_tracks_highest_occluding_state() {
    let mut chunk = Chunk::new_lighting(ChunkPosition::new(0, 0));
    let position = BlockPosition::new(3, 80, 4);
    chunk.set_block(position, Block::STONE);

    assert_eq!(chunk.sky_occlusion_map()[4 * 16 + 3], 81);
    assert_eq!(chunk.highest_block(), 80);

    chunk.set_block(position, Block::AIR);

    assert_eq!(chunk.sky_occlusion_map()[4 * 16 + 3], -64);
    assert_eq!(chunk.highest_block(), -65);
}

#[test]
fn bulk_relight_returns_all_loaded_affected_chunks() {
    let mut world = lighting_world("bulk_relight");
    let first = ChunkPosition::new(0, 0);
    let second = ChunkPosition::new(1, 0);
    let distant = ChunkPosition::new(4, 4);
    world.load_chunk(first).unwrap();
    world.load_chunk(second).unwrap();
    world.load_chunk(distant).unwrap();

    let mut affected = world.relight_chunks(&[first]);
    affected.sort_by_key(|position| (position.x, position.z));

    assert_eq!(affected, vec![first, second]);
    assert!(world.relight_chunks(&[ChunkPosition::new(5, 5)]).is_empty());
}

#[test]
fn block_change_coalesces_delayed_light_updates_for_loaded_neighbor_chunks() {
    let mut world = lighting_world("overworld");
    let affected_positions = (-1..=1)
        .flat_map(|x| (-1..=1).map(move |z| ChunkPosition::new(x, z)))
        .collect::<Vec<_>>();
    let unrelated_position = ChunkPosition::new(2, 2);
    affected_positions
        .iter()
        .copied()
        .chain([unrelated_position])
        .for_each(|position| {
            world.load_chunk(position).unwrap();
        });
    (0..20).for_each(|tick| {
        world.tick_chunks(tick);
    });
    let mut affected_clients = (0..affected_positions.len())
        .map(|_| queued_client())
        .collect::<Vec<_>>();
    let mut unrelated_client = queued_client();
    affected_positions
        .iter()
        .zip(affected_clients.iter_mut())
        .for_each(|(position, client)| {
            let player = entered_player(client);
            let player_id = player.get_entity_id();
            world.add_entity(Entity::Player(player));
            world.load_chunk(*position).unwrap().add_viewer(player_id);
        });
    let unrelated_player = entered_player(&mut unrelated_client);
    let unrelated_player_id = unrelated_player.get_entity_id();
    world.add_entity(Entity::Player(unrelated_player));
    world
        .load_chunk(unrelated_position)
        .unwrap()
        .add_viewer(unrelated_player_id);

    world
        .set_block(BlockPosition::new(1, 64, 1), Block::TORCH)
        .unwrap();

    affected_clients.iter().for_each(|client| {
        assert!(
            !client
                .queued_outbound_packet_ids()
                .contains(&LightUpdatePacket::get_id())
        );
    });
    (0..99).for_each(|tick| {
        world.tick_chunks(tick);
    });
    affected_clients.iter().for_each(|client| {
        assert!(
            !client
                .queued_outbound_packet_ids()
                .contains(&LightUpdatePacket::get_id())
        );
    });
    world.tick_chunks(99);
    affected_clients.iter().for_each(|client| {
        assert!(
            client
                .queued_outbound_packet_ids()
                .contains(&LightUpdatePacket::get_id())
        );
    });
    assert!(
        !unrelated_client
            .queued_outbound_packet_ids()
            .contains(&LightUpdatePacket::get_id())
    );
}

#[test]
fn frozen_lighting_invalidation_skips_neighbor_updates_until_unfrozen() {
    let mut world = lighting_world("frozen_lighting");
    let changed_position = ChunkPosition::new(0, 0);
    let neighbor_position = ChunkPosition::new(1, 0);
    world.load_chunk(changed_position).unwrap();
    world.load_chunk(neighbor_position).unwrap();
    (0..20).for_each(|tick| {
        world.tick_chunks(tick);
    });
    world
        .load_chunk(changed_position)
        .unwrap()
        .set_freeze_lighting_invalidation(true);

    world
        .set_block(BlockPosition::new(1, 64, 1), Block::TORCH)
        .unwrap();

    assert!(
        !world
            .chunk(neighbor_position)
            .unwrap()
            .section(4)
            .unwrap()
            .block_light_is_invalidated()
    );

    world
        .load_chunk(changed_position)
        .unwrap()
        .set_freeze_lighting_invalidation(false);
    world
        .set_block(BlockPosition::new(1, 64, 1), Block::AIR)
        .unwrap();

    assert!(
        world
            .chunk(neighbor_position)
            .unwrap()
            .section(4)
            .unwrap()
            .block_light_is_invalidated()
    );
}

#[test]
fn generated_chunk_invalidates_neighbors_and_resends_after_twenty_ticks() {
    let mut world = lighting_world("generated_lighting");
    let generated_position = ChunkPosition::new(0, 0);
    let neighbor_position = ChunkPosition::new(1, 0);
    world.load_chunk(neighbor_position).unwrap();
    let mut neighbor_client = queued_client();
    let neighbor_player = entered_player(&mut neighbor_client);
    let neighbor_player_id = neighbor_player.get_entity_id();
    world.add_entity(Entity::Player(neighbor_player));
    world
        .load_chunk(neighbor_position)
        .unwrap()
        .add_viewer(neighbor_player_id);
    world.set_generator(|_| {});

    world.load_chunk(generated_position).unwrap();

    (0..19).for_each(|tick| {
        world.tick_chunks(tick);
    });
    assert!(
        !neighbor_client
            .queued_outbound_packet_ids()
            .contains(&LightUpdatePacket::get_id())
    );

    world.tick_chunks(19);

    assert!(
        neighbor_client
            .queued_outbound_packet_ids()
            .contains(&LightUpdatePacket::get_id())
    );
}

fn entered_player(client: &mut Client) -> Player {
    let mut player = Player::new(Uuid::new_v4(), "Viewer".to_owned(), 0, client.addr);
    player.set_client(client);
    player.mark_entered_world();
    player
}

fn queued_client() -> Client {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let addr = listener.local_addr().unwrap();
    let stream = std::net::TcpStream::connect(addr).unwrap();
    let _peer = listener.accept().unwrap();
    let mut client = Client::new(stream, addr);
    client.state = ConnectionState::Play;
    client.enable_outbound_packet_queue();
    client
}

fn lighting_world(identifier: &str) -> World {
    let mut world = World::new(Identifier::minecraft(identifier));
    world.set_chunk_supplier(Chunk::new_lighting);
    world
}
