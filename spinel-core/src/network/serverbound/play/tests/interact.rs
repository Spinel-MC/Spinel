use super::super::interact::{InteractAction, InteractPacket};
use spinel_network::{DataType, VarIntWrapper};
use std::io::{Cursor, Read};

#[test]
fn interact_packet_decodes_minestom_interact_shape() {
    let mut payload = Vec::new();
    VarIntWrapper(10).encode(&mut payload).unwrap();
    VarIntWrapper(0).encode(&mut payload).unwrap();
    VarIntWrapper(1).encode(&mut payload).unwrap();
    true.encode(&mut payload).unwrap();

    let mut reader = Cursor::new(payload);
    let packet = InteractPacket::decode(&mut reader).unwrap();
    let mut remaining = Vec::new();
    reader.read_to_end(&mut remaining).unwrap();

    assert_eq!(packet.entity_id, 10);
    assert_eq!(packet.action, InteractAction::Interact { hand: 1 });
    assert!(packet.using_secondary_action);
    assert!(remaining.is_empty());
}

#[test]
fn interact_packet_decodes_minestom_attack_shape() {
    let mut payload = Vec::new();
    VarIntWrapper(10).encode(&mut payload).unwrap();
    VarIntWrapper(1).encode(&mut payload).unwrap();
    false.encode(&mut payload).unwrap();

    let mut reader = Cursor::new(payload);
    let packet = InteractPacket::decode(&mut reader).unwrap();

    assert_eq!(packet.entity_id, 10);
    assert_eq!(packet.action, InteractAction::Attack);
    assert!(!packet.using_secondary_action);
}

#[test]
fn interact_packet_decodes_minestom_interact_at_shape() {
    let mut payload = Vec::new();
    VarIntWrapper(10).encode(&mut payload).unwrap();
    VarIntWrapper(2).encode(&mut payload).unwrap();
    0.25f32.encode(&mut payload).unwrap();
    0.5f32.encode(&mut payload).unwrap();
    0.75f32.encode(&mut payload).unwrap();
    VarIntWrapper(0).encode(&mut payload).unwrap();
    false.encode(&mut payload).unwrap();

    let mut reader = Cursor::new(payload);
    let packet = InteractPacket::decode(&mut reader).unwrap();

    assert_eq!(packet.entity_id, 10);
    assert_eq!(
        packet.action,
        InteractAction::InteractAt {
            target_x: 0.25,
            target_y: 0.5,
            target_z: 0.75,
            hand: 0
        }
    );
    assert!(!packet.using_secondary_action);
}

#[test]
fn interact_packet_rejects_unknown_action_id() {
    let mut payload = Vec::new();
    VarIntWrapper(10).encode(&mut payload).unwrap();
    VarIntWrapper(3).encode(&mut payload).unwrap();

    let error = InteractPacket::decode(&mut payload.as_slice()).unwrap_err();

    assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
}
