use spinel_macros::packet;
use uuid::Uuid;

#[packet(
    id: "teleport_to_entity",
    state: ConnectionState::Play,
    recipient: Recipient::Server
)]
pub struct TeleportToEntityPacket {
    pub target: Uuid,
}

#[cfg(test)]
mod tests {
    use super::TeleportToEntityPacket;
    use spinel_network::DataType;
    use std::io::{Cursor, Read};
    use uuid::Uuid;

    #[test]
    fn teleport_to_entity_decodes_minestom_spectate_packet_shape() {
        let target = Uuid::from_u128(12);
        let mut payload = Vec::new();
        target.encode(&mut payload).unwrap();

        let mut reader = Cursor::new(payload);
        let packet = TeleportToEntityPacket::decode(&mut reader).unwrap();
        let mut remaining = Vec::new();
        reader.read_to_end(&mut remaining).unwrap();

        assert_eq!(packet.target, target);
        assert!(remaining.is_empty());
        assert_eq!(TeleportToEntityPacket::get_id(), 0x3d);
    }
}
