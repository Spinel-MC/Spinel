use spinel_macros::packet;

#[packet(
    id: "player_abilities",
    state: ConnectionState::Play,
    recipient: Recipient::Client
)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerAbilitiesPacket {
    pub flags: u8,
    pub flying_speed: f32,
    pub field_view_modifier: f32,
}

impl PlayerAbilitiesPacket {
    pub const INVULNERABLE: u8 = 0x01;
    pub const FLYING: u8 = 0x02;
    pub const ALLOW_FLYING: u8 = 0x04;
    pub const INSTANT_BREAK: u8 = 0x08;

    pub const fn new(flags: u8, flying_speed: f32, field_view_modifier: f32) -> Self {
        Self {
            flags,
            flying_speed,
            field_view_modifier,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PlayerAbilitiesPacket;
    use spinel_network::DataType;

    #[test]
    fn player_abilities_packet_matches_minestom_shape() {
        let packet = PlayerAbilitiesPacket::new(
            PlayerAbilitiesPacket::INVULNERABLE
                | PlayerAbilitiesPacket::ALLOW_FLYING
                | PlayerAbilitiesPacket::INSTANT_BREAK,
            0.05,
            0.1,
        );
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();
        let decoded_packet = PlayerAbilitiesPacket::decode(&mut payload.as_slice()).unwrap();

        assert_eq!(PlayerAbilitiesPacket::get_id(), 0x3e);
        assert_eq!(payload.len(), 9);
        assert_eq!(decoded_packet.flags, 0x0d);
        assert_eq!(decoded_packet.flying_speed, 0.05);
        assert_eq!(decoded_packet.field_view_modifier, 0.1);
    }
}
