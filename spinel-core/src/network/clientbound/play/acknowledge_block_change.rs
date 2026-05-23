use spinel_macros::packet;
use spinel_network::VarInt;

#[packet(id: "block_changed_ack", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct AcknowledgeBlockChangePacket {
    pub sequence: VarInt,
}

#[cfg(test)]
mod tests {
    use super::AcknowledgeBlockChangePacket;
    use spinel_network::DataType;

    #[test]
    fn block_changed_ack_uses_play_packet_id_and_var_int_sequence() {
        let mut payload = Vec::new();

        AcknowledgeBlockChangePacket { sequence: 300 }
            .encode(&mut payload)
            .unwrap();

        assert_eq!(AcknowledgeBlockChangePacket::get_id(), 0x04);
        assert_eq!(payload, vec![0xAC, 0x02]);
    }
}
