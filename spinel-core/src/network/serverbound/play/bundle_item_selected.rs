use spinel_network::data_type::DataType;
use spinel_network::types::var_int::VarIntWrapper;
use spinel_network::{ConnectionState, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleItemSelectedPacket {
    pub slot_id: i32,
    pub selected_item_index: i32,
}

impl BundleItemSelectedPacket {
    pub const fn get_id_const() -> i32 {
        0x02
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Play
    }
}

impl DataType for BundleItemSelectedPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.slot_id).encode(writer)?;
        VarIntWrapper(self.selected_item_index).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let slot_id = VarIntWrapper::decode(reader)?.0;
        let selected_item_index = VarIntWrapper::decode(reader)?.0;
        if selected_item_index < -1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid selected_item_index {selected_item_index}"),
            ));
        }

        Ok(Self {
            slot_id,
            selected_item_index,
        })
    }
}

impl PacketStruct for BundleItemSelectedPacket {
    fn get_id() -> i32 {
        Self::get_id_const()
    }

    fn get_state() -> ConnectionState {
        Self::get_state_const()
    }
}

spinel_network::register_packet_codec!(BundleItemSelectedPacket, spinel_network::Recipient::Server);
