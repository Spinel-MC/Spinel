use super::super::sound_effect::{NetworkPositionedSoundEvent, SoundEffectPacket};
use spinel_network::DataType;
use spinel_network::types::Vector3d;
use spinel_network::types::sound::SoundEvent;

#[test]
fn sound_effect_packet_source_is_var_int_like_minestom() {
    let packet = SoundEffectPacket {
        sound_event: NetworkPositionedSoundEvent(SoundEvent::Id(2)),
        source_id: 0,
        position: Vector3d {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
        volume: 1.0,
        pitch: 1.0,
        seed: 42,
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = SoundEffectPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(payload[1], 0);
    assert_eq!(payload.len(), 30);
    assert_eq!(decoded_packet.source_id, 0);
    assert_eq!(decoded_packet.seed, 42);
}
