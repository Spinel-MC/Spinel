use spinel_macros::packet;
use spinel_network::data_type::DataType;
use spinel_network::types::Identifier;
use spinel_network::types::var_int::VarIntWrapper;
use std::io::{self, Read, Write};

#[packet(id: "stop_sound", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct StopSoundPacket {
    pub stop: NetworkSoundStop,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NetworkSoundStop {
    pub source: Option<i32>,
    pub sound: Option<Identifier>,
}

impl NetworkSoundStop {
    const SOURCE_FLAG: u8 = 0x01;
    const SOUND_FLAG: u8 = 0x02;

    pub fn new(source: Option<i32>, sound: Option<Identifier>) -> Self {
        Self { source, sound }
    }

    const fn flags(&self) -> u8 {
        let source_flag = match self.source {
            Some(_) => Self::SOURCE_FLAG,
            None => 0,
        };
        let sound_flag = match self.sound {
            Some(_) => Self::SOUND_FLAG,
            None => 0,
        };
        source_flag | sound_flag
    }
}

impl StopSoundPacket {
    pub fn new(source: Option<i32>, sound: Option<Identifier>) -> Self {
        Self {
            stop: NetworkSoundStop::new(source, sound),
        }
    }
}

impl DataType for NetworkSoundStop {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let flags = self.flags();
        flags.encode(writer)?;
        if let Some(source) = self.source {
            VarIntWrapper(source).encode(writer)?;
        }
        if let Some(sound) = &self.sound {
            sound.encode(writer)?;
        }
        Ok(())
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let flags = u8::decode(reader)?;
        let source = match flags & Self::SOURCE_FLAG != 0 {
            true => Some(VarIntWrapper::decode(reader)?.0),
            false => None,
        };
        let sound = match flags & Self::SOUND_FLAG != 0 {
            true => Some(Identifier::decode(reader)?),
            false => None,
        };
        Ok(Self { source, sound })
    }
}
