use spinel_network::data_type::DataType;
use spinel_network::types::VarInt;
use spinel_network::types::sound::SoundEvent;
use spinel_network::types::var_int::VarIntWrapper;
use spinel_network::{ConnectionState, PacketSender, PacketStruct};
use std::io::{self, Read, Write};

pub struct EntitySoundEffectPacket {
    pub sound_event: NetworkSoundEvent,
    pub source_id: VarInt,
    pub entity_id: VarInt,
    pub volume: f32,
    pub pitch: f32,
    pub seed: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NetworkSoundEvent(pub SoundEvent);

impl EntitySoundEffectPacket {
    pub const fn get_id() -> i32 {
        0x72
    }

    pub const fn get_id_const() -> i32 {
        0x72
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Play
    }

    pub fn dispatch<S: PacketSender>(self, sender: &mut S) -> io::Result<()> {
        let mut payload_bytes = Vec::new();
        self.encode(&mut payload_bytes)?;
        sender.send_packet(Self::get_id(), &payload_bytes)
    }
}

impl DataType for EntitySoundEffectPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.sound_event.encode(writer)?;
        VarIntWrapper(self.source_id).encode(writer)?;
        VarIntWrapper(self.entity_id).encode(writer)?;
        self.volume.encode(writer)?;
        self.pitch.encode(writer)?;
        self.seed.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            sound_event: NetworkSoundEvent::decode(reader)?,
            source_id: VarIntWrapper::decode(reader)?.0,
            entity_id: VarIntWrapper::decode(reader)?.0,
            volume: f32::decode(reader)?,
            pitch: f32::decode(reader)?,
            seed: i64::decode(reader)?,
        })
    }
}

impl PacketStruct for EntitySoundEffectPacket {
    fn get_id() -> i32 {
        0x72
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}

impl DataType for NetworkSoundEvent {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.0.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self(SoundEvent::decode(reader)?))
    }
}
spinel_network::register_packet_codec!(EntitySoundEffectPacket, spinel_network::Recipient::Client);
