use spinel_network::data_type::DataType;
use spinel_network::encoder::NetworkBuffer;
use spinel_network::types::var_int::VarIntWrapper;
use spinel_network::{ConnectionState, PacketSender, PacketStruct};
use spinel_registry::RegistryKey;
use spinel_registry::dialog::Dialog;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShowDialogPacket {
    pub dialog_id: i32,
}

impl ShowDialogPacket {
    pub const fn get_id() -> i32 {
        0x8a
    }

    pub const fn get_id_const() -> i32 {
        0x8a
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Play
    }

    pub fn from_vanilla_dialog(dialog: &RegistryKey<Dialog>) -> Option<Self> {
        let dialog_id = if dialog == &Dialog::CUSTOM_OPTIONS {
            0
        } else if dialog == &Dialog::QUICK_ACTIONS {
            1
        } else if dialog == &Dialog::SERVER_LINKS {
            2
        } else {
            return None;
        };

        Some(Self {
            dialog_id: dialog_id + 1,
        })
    }

    pub fn encode_to_buffer(&self) -> io::Result<NetworkBuffer> {
        let mut buffer = NetworkBuffer::new();
        self.encode(&mut buffer)?;
        Ok(buffer)
    }

    pub fn dispatch<S: PacketSender>(self, sender: &mut S) -> io::Result<()> {
        let payload_bytes = self.encode_to_buffer()?.into_buffer();
        sender.send_packet(Self::get_id(), &payload_bytes)
    }
}

impl DataType for ShowDialogPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.dialog_id).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            dialog_id: VarIntWrapper::decode(reader)?.0,
        })
    }
}

impl PacketStruct for ShowDialogPacket {
    fn get_id() -> i32 {
        Self::get_id()
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}

#[cfg(test)]
mod tests {
    use super::ShowDialogPacket;
    use spinel_network::DataType;
    use spinel_registry::dialog::Dialog;

    #[test]
    fn show_dialog_packet_uses_minestom_holder_id_shape() {
        let packet = ShowDialogPacket::from_vanilla_dialog(&Dialog::QUICK_ACTIONS).unwrap();
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();
        let decoded_packet = ShowDialogPacket::decode(&mut payload.as_slice()).unwrap();

        assert_eq!(ShowDialogPacket::get_id(), 0x8a);
        assert_eq!(decoded_packet.dialog_id, 2);
    }
}
