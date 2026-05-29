use crate::entity::Player;
use crate::world::{Chunk, ChunkPosition};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use uuid::Uuid;

#[test]
fn player_send_chunk_queues_loaded_chunks_and_ignores_unloaded_chunks() {
    let mut player = Player::new(
        Uuid::nil(),
        "Player".to_string(),
        0,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565),
    );
    let loaded_chunk = Chunk::new(ChunkPosition::new(2, 3));
    let mut unloaded_chunk = Chunk::new(ChunkPosition::new(4, 5));

    unloaded_chunk.unload();

    assert!(loaded_chunk.send_chunk_to_player(&mut player));
    assert!(!unloaded_chunk.send_chunk_to_player(&mut player));
    assert_eq!(player.queued_chunk_count(), 1);
}
