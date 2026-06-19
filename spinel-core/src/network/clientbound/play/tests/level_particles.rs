use super::super::level_particles::LevelParticlesPacket;
use spinel_network::data_type::DataType;
use spinel_network::types::{Particle, Vector3d};

#[test]
fn level_particles_packet_roundtrips() {
    let packet = LevelParticlesPacket {
        override_limiter: true,
        always_show: false,
        position: Vector3d {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
        offset_x: 0.25,
        offset_y: 0.5,
        offset_z: 0.75,
        max_speed: 1.5,
        count: 8,
        particle: Particle::effect(),
    };
    let mut payload = Vec::new();

    packet.encode(&mut payload).unwrap();
    let decoded_packet = LevelParticlesPacket::decode(&mut payload.as_slice()).unwrap();

    assert_eq!(decoded_packet.override_limiter, packet.override_limiter);
    assert_eq!(decoded_packet.always_show, packet.always_show);
    assert_eq!(decoded_packet.position, packet.position);
    assert_eq!(decoded_packet.offset_x, packet.offset_x);
    assert_eq!(decoded_packet.offset_y, packet.offset_y);
    assert_eq!(decoded_packet.offset_z, packet.offset_z);
    assert_eq!(decoded_packet.max_speed, packet.max_speed);
    assert_eq!(decoded_packet.count, packet.count);
    assert_eq!(decoded_packet.particle, packet.particle);
}
