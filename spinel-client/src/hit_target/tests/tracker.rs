use spinel_network::types::{
    Position, Vector3d,
    chunk::{ChunkData, ChunkSection, PalettedContainer},
};
use spinel_registry::EntityType;

use crate::hit_target::{BlockHitTarget, ClientHitTargetTracker, HitTarget, TrackedEntity};

#[test]
fn hit_target_selects_entity_in_view_ray() {
    let mut tracker = ClientHitTargetTracker::default();
    tracker.track_entity(TrackedEntity::new(
        7,
        EntityType::ZOMBIE.id(),
        Vector3d {
            x: 0.0,
            y: 64.0,
            z: 2.5,
        },
    ));

    let target = tracker.hit_target(
        Vector3d {
            x: 0.0,
            y: 65.62,
            z: 0.0,
        },
        Vector3d {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    );

    assert_eq!(target, HitTarget::Entity { entity_id: 7 });
}

#[test]
fn hit_target_selects_nearest_block_in_view_ray() {
    let mut tracker = ClientHitTargetTracker::default();
    tracker.set_block_state(Position { x: 0, y: 65, z: 2 }, 1);

    let target = tracker.hit_target(
        Vector3d {
            x: 0.5,
            y: 65.5,
            z: 0.5,
        },
        Vector3d {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    );

    assert_eq!(
        target,
        HitTarget::Block(BlockHitTarget {
            position: Position { x: 0, y: 65, z: 2 },
            face: 2,
        })
    );
}

#[test]
fn hit_target_selects_block_from_tracked_chunk() {
    let mut tracker = ClientHitTargetTracker::default();
    tracker.track_chunk(0, 0, chunk_with_single_block(0, 0, 2));

    let target = tracker.hit_target(
        Vector3d {
            x: 0.5,
            y: 64.5,
            z: 0.5,
        },
        Vector3d {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    );

    assert_eq!(
        target,
        HitTarget::Block(BlockHitTarget {
            position: Position { x: 0, y: 64, z: 2 },
            face: 2,
        })
    );
}

#[test]
fn block_update_overrides_tracked_chunk_without_expansion() {
    let mut tracker = ClientHitTargetTracker::default();
    let block_position = Position { x: 0, y: 64, z: 2 };
    tracker.track_chunk(0, 0, chunk_with_single_block(0, 0, 2));
    tracker.set_block_state(block_position, 0);

    let target = tracker.hit_target(
        Vector3d {
            x: 0.5,
            y: 64.5,
            z: 0.5,
        },
        Vector3d {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    );

    assert_eq!(target, HitTarget::Miss);
}

#[test]
fn tracked_chunk_count_in_square_reports_client_visible_coverage() {
    let mut tracker = ClientHitTargetTracker::default();
    let chunk_data = chunk_with_single_block(0, 0, 0);

    tracker.track_chunk(0, 0, chunk_data.clone());
    tracker.track_chunk(1, 0, chunk_data.clone());
    tracker.track_chunk(2, 0, chunk_data);

    assert_eq!(tracker.tracked_chunk_count_in_square(0, 0, 1), 2);
    assert_eq!(tracker.tracked_chunk_count_in_square(1, 0, 1), 3);
}

fn chunk_with_single_block(local_x: i32, local_y: i32, local_z: i32) -> ChunkData {
    let mut sections = (0..8)
        .map(|_| air_section())
        .chain(std::iter::once(section_with_single_block(
            local_x, local_y, local_z,
        )))
        .collect::<Vec<_>>();
    sections.extend((0..15).map(|_| air_section()));

    ChunkData {
        heightmaps: Vec::new(),
        sections,
        block_entities: Vec::new(),
    }
}

fn air_section() -> ChunkSection {
    ChunkSection {
        block_count: 0,
        block_states: PalettedContainer {
            bits_per_entry: 0,
            palette: Some(vec![0]),
            data: Vec::new(),
        },
        biomes: PalettedContainer {
            bits_per_entry: 0,
            palette: Some(vec![0]),
            data: Vec::new(),
        },
    }
}

fn section_with_single_block(local_x: i32, local_y: i32, local_z: i32) -> ChunkSection {
    let block_index = ((local_y as usize) << 8) | ((local_z as usize) << 4) | local_x as usize;
    let mut data = vec![0u64; 256];
    data[block_index / 16] = 1u64 << ((block_index % 16) * 4);

    ChunkSection {
        block_count: 1,
        block_states: PalettedContainer {
            bits_per_entry: 4,
            palette: Some(vec![0, 1]),
            data,
        },
        biomes: PalettedContainer {
            bits_per_entry: 0,
            palette: Some(vec![0]),
            data: Vec::new(),
        },
    }
}
