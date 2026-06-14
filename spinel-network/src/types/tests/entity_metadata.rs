use crate::data_type::DataType;
use crate::types::entity_metadata::MetadataValue;
use crate::types::game_profile::GameProfileProperty;
use crate::types::{
    Identifier, MainHand, PartialGameProfile, Particle, ParticlePayload, PlayerSkinPatch,
    Quaternionf, ResolvableProfile, ResolvableProfileIdentity, Vector3f,
};
use std::io::Cursor;
use uuid::Uuid;

#[test]
fn current_metadata_serializer_ids_round_trip() {
    let values = [
        MetadataValue::ZombieNautilusVariant(3),
        MetadataValue::OptionalGlobalPos(None),
        MetadataValue::PaintingVariant(4),
        MetadataValue::SnifferState(5),
        MetadataValue::ArmadilloState(6),
        MetadataValue::CopperGolemState(7),
        MetadataValue::WeatherState(2),
        MetadataValue::Vector3f(Vector3f {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        }),
        MetadataValue::Quaternionf(Quaternionf {
            x: 0.0,
            y: 0.5,
            z: 1.0,
            w: -0.5,
        }),
        MetadataValue::ResolvableProfile(ResolvableProfile {
            identity: ResolvableProfileIdentity::Partial(PartialGameProfile {
                name: Some("Spinel".to_string()),
                uuid: Some(Uuid::from_u128(42)),
                properties: vec![GameProfileProperty {
                    name: "textures".to_string(),
                    value: "texture-value".to_string(),
                    signature: Some("texture-signature".to_string()),
                }],
            }),
            skin_patch: PlayerSkinPatch {
                body: Some(Identifier::minecraft("entity/player/wide/steve")),
                cape: None,
                elytra: None,
                is_slim: Some(false),
            },
        }),
        MetadataValue::MainHand(MainHand::Right),
    ];

    for (offset, value) in values.into_iter().enumerate() {
        let expected_type_id = 28 + offset as u8;
        let mut encoded = Vec::new();
        value.encode(&mut encoded).unwrap();

        assert_eq!(encoded[0], expected_type_id);
        assert_eq!(
            MetadataValue::decode(&mut Cursor::new(&encoded)).unwrap(),
            value
        );
    }
}

#[test]
fn main_hand_rejects_unknown_protocol_id() {
    let error = MainHand::decode(&mut Cursor::new([2])).unwrap_err();

    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
}

#[test]
fn particle_metadata_preserves_payload_bytes() {
    let particle = Particle::new(
        14,
        ParticlePayload::ColorScale {
            color: 0x112233,
            scale: 0.75,
        },
    );
    let metadata_value = MetadataValue::Particle(particle.clone());
    let mut encoded = Vec::new();

    metadata_value.encode(&mut encoded).unwrap();

    assert_eq!(encoded[0], 16);
    assert_eq!(
        MetadataValue::decode(&mut Cursor::new(&encoded)).unwrap(),
        metadata_value
    );
}

#[test]
fn particle_list_metadata_preserves_particle_count_and_payloads() {
    let metadata_value = MetadataValue::ParticleList(vec![
        Particle::new(0, ParticlePayload::Unit),
        Particle::new(38, ParticlePayload::Float(1.25)),
    ]);
    let mut encoded = Vec::new();

    metadata_value.encode(&mut encoded).unwrap();

    assert_eq!(encoded[0], 17);
    assert_eq!(
        MetadataValue::decode(&mut Cursor::new(&encoded)).unwrap(),
        metadata_value
    );
}

#[test]
fn particle_rejects_payload_shape_mismatch() {
    let mismatched_particle = Particle::new(14, ParticlePayload::Unit);
    let error = mismatched_particle.encode(&mut Vec::new()).unwrap_err();

    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
}
