use super::super::stop_sound::{NetworkSoundStop, StopSoundPacket};
use spinel_network::DataType;
use spinel_network::types::Identifier;

#[test]
fn stop_sound_packet_matches_minestom_optional_source_and_sound_shape() {
    let packet = StopSoundPacket::new(Some(3), Some(Identifier::minecraft("entity.arrow.hit")));
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = StopSoundPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(StopSoundPacket::get_id(), 0x75);
    assert_eq!(payload[0], 0x03);
    assert_eq!(decoded_packet.stop, packet.stop);
}

#[test]
fn stop_all_sounds_has_zero_flags_like_minestom() {
    let packet = StopSoundPacket {
        stop: NetworkSoundStop::default(),
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();

    assert_eq!(payload, vec![0]);
}
