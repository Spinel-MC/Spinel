use super::super::waypoint::{WaypointOperation, WaypointPacket};
use spinel_network::data_type::DataType;
use spinel_network::types::{
    Identifier, TrackedWaypoint, TrackedWaypointId, Vector3i, WaypointIcon,
};
use uuid::Uuid;

#[test]
fn waypoint_packet_roundtrips() {
    let packet = WaypointPacket {
        operation: WaypointOperation::Track,
        waypoint: TrackedWaypoint::Position {
            id: TrackedWaypointId::EntityUuid(Uuid::from_u128(0x00112233445566778899aabbccddeeff)),
            icon: WaypointIcon {
                style_asset: Identifier::minecraft("bowtie"),
                color: Some(0xff00ff),
            },
            position: Vector3i { x: 7, y: 8, z: 9 },
        },
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = WaypointPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet, packet);
}
