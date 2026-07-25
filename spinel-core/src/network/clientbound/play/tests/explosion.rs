use super::super::explosion::{ExplosionBlockParticleInfo, ExplosionPacket};
use spinel_network::data_type::DataType;
use spinel_network::types::sound::SoundEvent;
use spinel_network::types::{Particle, ParticlePayload, Vector3d};

#[test]
fn explosion_packet_matches_reference_wire_shape() {
    let packet = ExplosionPacket {
        center: Vector3d {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
        radius: 0.0,
        block_count: 0,
        player_knockback: None,
        particle: Particle::new(23, ParticlePayload::Unit),
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
    assert_eq!(decoded.particle, Particle::new(23, ParticlePayload::Unit));
    assert_eq!(decoded.sound, SoundEvent::Id(668));
    assert!(decoded.block_particles.is_empty());
}

#[test]
fn explosion_packet_accepts_minestom_shaped_particle_payloads() {
    let packet = ExplosionPacket {
        center: Vector3d {
            x: 1.25,
            y: 64.5,
            z: -3.75,
        },
        radius: 1.2,
        block_count: 0,
        player_knockback: Some(Vector3d {
            x: 0.1,
            y: 0.2,
            z: 0.3,
        }),
        particle: Particle::new(27, ParticlePayload::Unit),
        sound: SoundEvent::Id(668),
        block_particles: vec![ExplosionBlockParticleInfo {
            particle: Particle::new(1, ParticlePayload::BlockState(1)),
            scaling: 0.5,
            speed: 1.0,
        }],
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded = ExplosionPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded.center, packet.center);
    assert_eq!(decoded.radius, packet.radius);
    assert_eq!(decoded.block_count, packet.block_count);
    assert_eq!(decoded.player_knockback, packet.player_knockback);
    assert_eq!(decoded.particle, packet.particle);
    assert_eq!(decoded.sound, packet.sound);
    assert_eq!(decoded.block_particles, packet.block_particles);
}
