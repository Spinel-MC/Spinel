use spinel_network::data_type::DataType;
use spinel_network::wrappers::NbtTextComponent;
use spinel_network::{ConnectionState, PacketSender, PacketStruct, VarIntWrapper};
use spinel_utils::component::text::TextComponent;
use std::io::{self, Read, Write};
use uuid::Uuid;

pub struct BossBarPacket {
    pub id: Uuid,
    pub action: BossBarAction,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BossBarAction {
    Add {
        title: TextComponent,
        health: f32,
        color: BossBarColor,
        overlay: BossBarOverlay,
        flags: u8,
    },
    Remove,
    UpdateHealth {
        health: f32,
    },
    UpdateTitle {
        title: TextComponent,
    },
    UpdateStyle {
        color: BossBarColor,
        overlay: BossBarOverlay,
    },
    UpdateFlags {
        flags: u8,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BossBarColor {
    Pink,
    Blue,
    Red,
    Green,
    Yellow,
    Purple,
    White,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BossBarOverlay {
    Progress,
    Notched6,
    Notched10,
    Notched12,
    Notched20,
}

impl BossBarPacket {
    pub const fn get_id() -> i32 {
        0x09
    }

    pub const fn get_id_const() -> i32 {
        0x09
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

impl DataType for BossBarPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.id.encode(writer)?;
        VarIntWrapper(self.action.action_id()).encode(writer)?;
        self.action.encode_payload(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let id = Uuid::decode(reader)?;
        let action_id = VarIntWrapper::decode(reader)?.0;
        Ok(Self {
            id,
            action: BossBarAction::decode_payload(action_id, reader)?,
        })
    }
}

impl PacketStruct for BossBarPacket {
    fn get_id() -> i32 {
        0x09
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}

impl BossBarAction {
    fn action_id(&self) -> i32 {
        match self {
            Self::Add { .. } => 0,
            Self::Remove => 1,
            Self::UpdateHealth { .. } => 2,
            Self::UpdateTitle { .. } => 3,
            Self::UpdateStyle { .. } => 4,
            Self::UpdateFlags { .. } => 5,
        }
    }

    fn encode_payload<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            Self::Add {
                title,
                health,
                color,
                overlay,
                flags,
            } => {
                NbtTextComponent(title.clone()).encode(writer)?;
                health.encode(writer)?;
                VarIntWrapper(color.protocol_id()).encode(writer)?;
                VarIntWrapper(overlay.protocol_id()).encode(writer)?;
                flags.encode(writer)
            }
            Self::Remove => Ok(()),
            Self::UpdateHealth { health } => health.encode(writer),
            Self::UpdateTitle { title } => NbtTextComponent(title.clone()).encode(writer),
            Self::UpdateStyle { color, overlay } => {
                VarIntWrapper(color.protocol_id()).encode(writer)?;
                VarIntWrapper(overlay.protocol_id()).encode(writer)
            }
            Self::UpdateFlags { flags } => flags.encode(writer),
        }
    }

    fn decode_payload<R: Read>(action_id: i32, reader: &mut R) -> io::Result<Self> {
        match action_id {
            0 => Ok(Self::Add {
                title: NbtTextComponent::decode(reader)?.0,
                health: f32::decode(reader)?,
                color: BossBarColor::from_protocol_id(VarIntWrapper::decode(reader)?.0)?,
                overlay: BossBarOverlay::from_protocol_id(VarIntWrapper::decode(reader)?.0)?,
                flags: u8::decode(reader)?,
            }),
            1 => Ok(Self::Remove),
            2 => Ok(Self::UpdateHealth {
                health: f32::decode(reader)?,
            }),
            3 => Ok(Self::UpdateTitle {
                title: NbtTextComponent::decode(reader)?.0,
            }),
            4 => Ok(Self::UpdateStyle {
                color: BossBarColor::from_protocol_id(VarIntWrapper::decode(reader)?.0)?,
                overlay: BossBarOverlay::from_protocol_id(VarIntWrapper::decode(reader)?.0)?,
            }),
            5 => Ok(Self::UpdateFlags {
                flags: u8::decode(reader)?,
            }),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unknown boss bar action",
            )),
        }
    }
}

impl BossBarColor {
    pub const fn protocol_id(self) -> i32 {
        match self {
            Self::Pink => 0,
            Self::Blue => 1,
            Self::Red => 2,
            Self::Green => 3,
            Self::Yellow => 4,
            Self::Purple => 5,
            Self::White => 6,
        }
    }

    fn from_protocol_id(protocol_id: i32) -> io::Result<Self> {
        match protocol_id {
            0 => Ok(Self::Pink),
            1 => Ok(Self::Blue),
            2 => Ok(Self::Red),
            3 => Ok(Self::Green),
            4 => Ok(Self::Yellow),
            5 => Ok(Self::Purple),
            6 => Ok(Self::White),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unknown boss bar color",
            )),
        }
    }
}

impl BossBarOverlay {
    pub const fn protocol_id(self) -> i32 {
        match self {
            Self::Progress => 0,
            Self::Notched6 => 1,
            Self::Notched10 => 2,
            Self::Notched12 => 3,
            Self::Notched20 => 4,
        }
    }

    fn from_protocol_id(protocol_id: i32) -> io::Result<Self> {
        match protocol_id {
            0 => Ok(Self::Progress),
            1 => Ok(Self::Notched6),
            2 => Ok(Self::Notched10),
            3 => Ok(Self::Notched12),
            4 => Ok(Self::Notched20),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unknown boss bar overlay",
            )),
        }
    }
}
spinel_network::register_packet_codec!(BossBarPacket, spinel_network::Recipient::Client);
