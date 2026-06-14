use super::super::explosion::{ExplosionPacket, ExplosionParticle};
use spinel_network::data_type::DataType;
use spinel_network::types::Vector3d;
use spinel_network::types::sound::SoundEvent;

#[test]
fn explosion_packet_matches_minestom_wire_shape() {
    let packet = ExplosionPacket {
        center: Vector3d {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
        radius: 0.0,
        block_count: 0,
        player_knockback: None,
        particle: ExplosionParticle::Explosion,
        sound: SoundEvent::Id(668),
        block_particles: Vec::new(),
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded = ExplosionPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(ExplosionPacket::get_id(), 0x24);
    assert_eq!(decoded.center, packet.center);
    assert_eq!(decoded.radius, 0.0);
    assert_eq!(decoded.block_count, 0);
    assert_eq!(decoded.player_knockback, None);
    assert_eq!(decoded.particle, ExplosionParticle::Explosion);
    assert_eq!(decoded.sound, SoundEvent::Id(668));
    assert!(decoded.block_particles.is_empty());
}
