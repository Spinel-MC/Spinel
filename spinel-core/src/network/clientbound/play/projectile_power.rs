use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "projectile_power", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct ProjectilePowerPacket {
    pub entity_id: VarInt,
    pub acceleration_power: f64,
}

#[cfg(test)]
mod tests {
    use super::ProjectilePowerPacket;
    use spinel_network::DataType;

    #[test]
    fn projectile_power_packet_matches_minestom_var_int_then_double_shape() {
        let packet = ProjectilePowerPacket {
            entity_id: 7,
            acceleration_power: 1.5,
        };
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();
        let decoded_packet = ProjectilePowerPacket::decode(&mut payload.as_slice()).unwrap();

        assert_eq!(ProjectilePowerPacket::get_id(), 0x85);
        assert_eq!(decoded_packet.entity_id, 7);
        assert_eq!(decoded_packet.acceleration_power, 1.5);
    }
}
