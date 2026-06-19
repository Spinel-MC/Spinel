use crate::data_type::DataType;
use crate::types::{
    ChunkPos, Identifier, TrackedWaypoint, TrackedWaypointId, Vector3i, WaypointIcon,
};
use uuid::Uuid;

#[test]
fn tracked_waypoint_position_roundtrips() {
    let waypoint = TrackedWaypoint::Position {
        id: TrackedWaypointId::EntityUuid(Uuid::from_u128(0x00112233445566778899aabbccddeeff)),
        icon: WaypointIcon {
            style_asset: Identifier::minecraft("default"),
            color: Some(0x00ff00),
        },
        position: Vector3i { x: 1, y: 2, z: 3 },
    };
    let mut payload = Vec::new();

    waypoint.encode(&mut payload).unwrap();
    let decoded_waypoint = TrackedWaypoint::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_waypoint, waypoint);
}

#[test]
fn tracked_waypoint_chunk_roundtrips() {
    let waypoint = TrackedWaypoint::Chunk {
        id: TrackedWaypointId::Named("spawn".to_owned()),
        icon: WaypointIcon::null(),
        chunk_pos: ChunkPos { x: 9, z: 10 },
    };
    let mut payload = Vec::new();

    waypoint.encode(&mut payload).unwrap();
    let decoded_waypoint = TrackedWaypoint::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_waypoint, waypoint);
}
