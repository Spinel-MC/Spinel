use spinel_network::data_type::DataType;
use spinel_network::types::{MerchantOffers, var_int::VarIntWrapper};
use spinel_network::{ConnectionState, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct MerchantOffersPacket {
    pub container_id: i32,
    pub offers: MerchantOffers,
    pub villager_level: i32,
    pub villager_experience: i32,
    pub show_progress: bool,
    pub can_restock: bool,
}

impl DataType for MerchantOffersPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.container_id).encode(writer)?;
        self.offers.encode(writer)?;
        VarIntWrapper(self.villager_level).encode(writer)?;
        VarIntWrapper(self.villager_experience).encode(writer)?;
        self.show_progress.encode(writer)?;
        self.can_restock.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            container_id: VarIntWrapper::decode(reader)?.0,
            offers: MerchantOffers::decode(reader)?,
            villager_level: VarIntWrapper::decode(reader)?.0,
            villager_experience: VarIntWrapper::decode(reader)?.0,
            show_progress: bool::decode(reader)?,
            can_restock: bool::decode(reader)?,
        })
    }
}

impl PacketStruct for MerchantOffersPacket {
    fn get_id() -> i32 {
        0x32
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}
spinel_network::register_packet_codec!(MerchantOffersPacket, spinel_network::Recipient::Client);
