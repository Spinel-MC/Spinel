use spinel_macros::packet;
use spinel_network::types::VarInt;

#[packet(id: "cooldown", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct CooldownPacket {
    pub cooldown_group: String,
    pub cooldown_ticks: VarInt,
}

impl CooldownPacket {
    pub fn new(cooldown_group: impl Into<String>, cooldown_ticks: i32) -> Self {
        Self {
            cooldown_group: cooldown_group.into(),
            cooldown_ticks,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CooldownPacket;
    use spinel_network::DataType;

    #[test]
    fn cooldown_packet_uses_play_packet_id_string_group_and_var_int_ticks() {
        let packet = CooldownPacket::new("minecraft:ender_pearl", 20);
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();

        assert_eq!(CooldownPacket::get_id(), 0x16);
        assert_eq!(payload.last().copied(), Some(20));
    }
}
