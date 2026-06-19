use super::super::debug_sample::{DebugSamplePacket, RemoteDebugSampleType};
use spinel_network::data_type::DataType;

#[test]
fn debug_sample_packet_roundtrips() {
    let packet = DebugSamplePacket {
        sample: vec![11, 12, 13],
        debug_sample_type: RemoteDebugSampleType::TickTime,
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = DebugSamplePacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet, packet);
}
