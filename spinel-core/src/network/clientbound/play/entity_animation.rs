use spinel_macros::packet;
use spinel_network::data_type::DataType;
use spinel_network::types::VarInt;
use std::io::{self, Read, Write};

#[packet(id: "animate", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct EntityAnimationPacket {
    pub entity_id: VarInt,
    pub animation: EntityAnimation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EntityAnimation {
    SwingMainArm = 0,
    TakeDamage = 1,
    LeaveBed = 2,
    SwingOffHand = 3,
    CriticalEffect = 4,
    MagicCriticalEffect = 5,
}

impl DataType for EntityAnimation {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        (*self as u8).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        match u8::decode(reader)? {
            0 => Ok(Self::SwingMainArm),
            1 => Ok(Self::TakeDamage),
            2 => Ok(Self::LeaveBed),
            3 => Ok(Self::SwingOffHand),
            4 => Ok(Self::CriticalEffect),
            5 => Ok(Self::MagicCriticalEffect),
            animation_id => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unknown entity animation id: {animation_id}"),
            )),
        }
    }
}
