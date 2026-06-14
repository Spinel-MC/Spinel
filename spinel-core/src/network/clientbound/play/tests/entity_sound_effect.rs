use super::super::entity_sound_effect::{EntitySoundEffectPacket, NetworkSoundEvent};
use spinel_network::DataType;
use spinel_network::types::sound::SoundEvent;

#[test]
fn entity_sound_effect_packet_matches_minestom_builtin_sound_branch_shape() {
    let packet = EntitySoundEffectPacket {
        sound_event: NetworkSoundEvent(SoundEvent::Id(2)),
        source_id: 2,
        entity_id: 7,
        volume: 1.0,
        pitch: 0.5,
        seed: 42,
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = EntitySoundEffectPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(EntitySoundEffectPacket::get_id(), 0x72);
    assert_eq!(payload.len(), 19);
    assert_eq!(
        decoded_packet.sound_event,
        NetworkSoundEvent(SoundEvent::Id(2))
    );
    assert_eq!(decoded_packet.source_id, 2);
    assert_eq!(decoded_packet.entity_id, 7);
    assert_eq!(decoded_packet.seed, 42);
}

#[test]
fn entity_sound_effect_packet_matches_minestom_named_sound_branch_shape() {
    let packet = EntitySoundEffectPacket {
        sound_event: NetworkSoundEvent(SoundEvent::Named {
            name: "minecraft:custom.sound".to_string(),
            fixed_range: Some(12.0),
        }),
        source_id: 2,
        entity_id: 7,
        volume: 1.0,
        pitch: 0.5,
        seed: 42,
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = EntitySoundEffectPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet.sound_event, packet.sound_event);
    assert_eq!(decoded_packet.source_id, 2);
    assert_eq!(decoded_packet.entity_id, 7);
}
