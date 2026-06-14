use spinel_network::data_type::DataType;
use spinel_network::types::sound::SoundEvent;
use spinel_network::types::var_int::VarIntWrapper;
use spinel_network::types::{VarInt, Vector3d};
use spinel_network::{ConnectionState, PacketSender, PacketStruct};
use std::io::{self, Read, Write};

pub struct SoundEffectPacket {
    pub sound_event: NetworkPositionedSoundEvent,
    pub source_id: VarInt,
    pub position: Vector3d,
    pub volume: f32,
    pub pitch: f32,
    pub seed: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NetworkPositionedSoundEvent(pub SoundEvent);

impl SoundEffectPacket {
    pub const fn get_id() -> i32 {
        0x73
    }

    pub const fn get_id_const() -> i32 {
        0x73
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

impl DataType for SoundEffectPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.sound_event.encode(writer)?;
        VarIntWrapper(self.source_id).encode(writer)?;
        ((self.position.x * 8.0) as i32).encode(writer)?;
        ((self.position.y * 8.0) as i32).encode(writer)?;
        ((self.position.z * 8.0) as i32).encode(writer)?;
        self.volume.encode(writer)?;
        self.pitch.encode(writer)?;
        self.seed.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            sound_event: NetworkPositionedSoundEvent::decode(reader)?,
            source_id: VarIntWrapper::decode(reader)?.0,
            position: Vector3d {
                x: f64::from(i32::decode(reader)?) / 8.0,
                y: f64::from(i32::decode(reader)?) / 8.0,
                z: f64::from(i32::decode(reader)?) / 8.0,
            },
            volume: f32::decode(reader)?,
            pitch: f32::decode(reader)?,
            seed: i64::decode(reader)?,
        })
    }
}

impl PacketStruct for SoundEffectPacket {
    fn get_id() -> i32 {
        0x73
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}

impl DataType for NetworkPositionedSoundEvent {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.0.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self(SoundEvent::decode(reader)?))
    }
}
spinel_network::register_packet_codec!(SoundEffectPacket, spinel_network::Recipient::Client);
