use spinel_network::VarIntWrapper;
use spinel_network::data_type::DataType;
use spinel_network::types::Identifier;
use spinel_network::{ConnectionState, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeenAdvancementsPacket {
    pub action: i32,
    pub tab_identifier: Option<Identifier>,
}

impl SeenAdvancementsPacket {
    pub const OPENED_TAB: i32 = 0;
    pub const CLOSED_SCREEN: i32 = 1;

    pub const fn get_id_const() -> i32 {
        0x31
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Play
    }
}

impl PacketStruct for SeenAdvancementsPacket {
    fn get_id() -> i32 {
        0x31
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}

impl DataType for SeenAdvancementsPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.action).encode(writer)?;
        if self.action == Self::OPENED_TAB {
            let Some(tab_identifier) = self.tab_identifier.as_ref() else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "opened advancement tab requires identifier",
                ));
            };
            tab_identifier.encode(writer)?;
        }
        Ok(())
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let action = VarIntWrapper::decode(reader)?.0;
        let tab_identifier = if action == Self::OPENED_TAB {
            Some(Identifier::decode(reader)?)
        } else {
            None
        };
        Ok(Self {
            action,
            tab_identifier,
        })
    }
}
