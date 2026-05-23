use spinel_macros::packet;
use spinel_network::VarInt;
use spinel_network::types::{Array, ItemStackHash};

#[packet(id: "container_click", state: ConnectionState::Play, recipient: Recipient::Server)]
pub struct ContainerClickPacket {
    pub container_id: VarInt,
    pub state_id: VarInt,
    pub slot: i16,
    pub button: i8,
    pub click_type: VarInt,
    pub changed_slots: Array<ChangedSlot>,
    pub carried_item: ItemStackHash,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangedSlot {
    pub slot: i16,
    pub item: ItemStackHash,
}

impl spinel_network::data_type::DataType for ChangedSlot {
    fn encode<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.slot.encode(writer)?;
        self.item.encode(writer)
    }

    fn decode<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        Ok(Self {
            slot: i16::decode(reader)?,
            item: ItemStackHash::decode(reader)?,
        })
    }
}
