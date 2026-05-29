use spinel_network::encoder::NetworkBuffer;
use spinel_network::{ConnectionState, DataType, PacketSender, PacketStruct, VarIntWrapper};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct InteractPacket {
    pub entity_id: i32,
    pub action: InteractAction,
    pub using_secondary_action: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InteractAction {
    Interact {
        hand: i32,
    },
    Attack,
    InteractAt {
        target_x: f32,
        target_y: f32,
        target_z: f32,
        hand: i32,
    },
}

impl InteractPacket {
    pub const fn get_id() -> i32 {
        0x19
    }

    pub const fn get_id_const() -> i32 {
        0x19
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Play
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

impl InteractAction {
    const INTERACT_ID: i32 = 0;
    const ATTACK_ID: i32 = 1;
    const INTERACT_AT_ID: i32 = 2;

    const fn id(&self) -> i32 {
        match self {
            Self::Interact { .. } => Self::INTERACT_ID,
            Self::Attack => Self::ATTACK_ID,
            Self::InteractAt { .. } => Self::INTERACT_AT_ID,
        }
    }
}

impl DataType for InteractPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.entity_id).encode(writer)?;
        self.action.encode(writer)?;
        self.using_secondary_action.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let entity_id = VarIntWrapper::decode(reader)?.0;
        let action = InteractAction::decode(reader)?;
        let using_secondary_action = bool::decode(reader)?;
        Ok(Self {
            entity_id,
            action,
            using_secondary_action,
        })
    }
}

impl DataType for InteractAction {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.id()).encode(writer)?;
        match self {
            Self::Interact { hand } => VarIntWrapper(*hand).encode(writer),
            Self::Attack => Ok(()),
            Self::InteractAt {
                target_x,
                target_y,
                target_z,
                hand,
            } => {
                target_x.encode(writer)?;
                target_y.encode(writer)?;
                target_z.encode(writer)?;
                VarIntWrapper(*hand).encode(writer)
            }
        }
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        match VarIntWrapper::decode(reader)?.0 {
            Self::INTERACT_ID => Ok(Self::Interact {
                hand: VarIntWrapper::decode(reader)?.0,
            }),
            Self::ATTACK_ID => Ok(Self::Attack),
            Self::INTERACT_AT_ID => Ok(Self::InteractAt {
                target_x: f32::decode(reader)?,
                target_y: f32::decode(reader)?,
                target_z: f32::decode(reader)?,
                hand: VarIntWrapper::decode(reader)?.0,
            }),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid interact action id",
            )),
        }
    }
}

impl PacketStruct for InteractPacket {
    fn get_id() -> i32 {
        Self::get_id()
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}

#[cfg(test)]
mod tests {
    use super::{InteractAction, InteractPacket};
    use spinel_network::{DataType, VarIntWrapper};
    use std::io::{Cursor, Read};

    #[test]
    fn interact_packet_decodes_minestom_interact_shape() {
        let mut payload = Vec::new();
        VarIntWrapper(10).encode(&mut payload).unwrap();
        VarIntWrapper(0).encode(&mut payload).unwrap();
        VarIntWrapper(1).encode(&mut payload).unwrap();
        true.encode(&mut payload).unwrap();

        let mut reader = Cursor::new(payload);
        let packet = InteractPacket::decode(&mut reader).unwrap();
        let mut remaining = Vec::new();
        reader.read_to_end(&mut remaining).unwrap();

        assert_eq!(packet.entity_id, 10);
        assert_eq!(packet.action, InteractAction::Interact { hand: 1 });
        assert!(packet.using_secondary_action);
        assert!(remaining.is_empty());
    }

    #[test]
    fn interact_packet_decodes_minestom_attack_shape() {
        let mut payload = Vec::new();
        VarIntWrapper(10).encode(&mut payload).unwrap();
        VarIntWrapper(1).encode(&mut payload).unwrap();
        false.encode(&mut payload).unwrap();

        let mut reader = Cursor::new(payload);
        let packet = InteractPacket::decode(&mut reader).unwrap();

        assert_eq!(packet.entity_id, 10);
        assert_eq!(packet.action, InteractAction::Attack);
        assert!(!packet.using_secondary_action);
    }

    #[test]
    fn interact_packet_decodes_minestom_interact_at_shape() {
        let mut payload = Vec::new();
        VarIntWrapper(10).encode(&mut payload).unwrap();
        VarIntWrapper(2).encode(&mut payload).unwrap();
        0.25f32.encode(&mut payload).unwrap();
        0.5f32.encode(&mut payload).unwrap();
        0.75f32.encode(&mut payload).unwrap();
        VarIntWrapper(0).encode(&mut payload).unwrap();
        false.encode(&mut payload).unwrap();

        let mut reader = Cursor::new(payload);
        let packet = InteractPacket::decode(&mut reader).unwrap();

        assert_eq!(packet.entity_id, 10);
        assert_eq!(
            packet.action,
            InteractAction::InteractAt {
                target_x: 0.25,
                target_y: 0.5,
                target_z: 0.75,
                hand: 0
            }
        );
        assert!(!packet.using_secondary_action);
    }

    #[test]
    fn interact_packet_rejects_unknown_action_id() {
        let mut payload = Vec::new();
        VarIntWrapper(10).encode(&mut payload).unwrap();
        VarIntWrapper(3).encode(&mut payload).unwrap();

        let error = InteractPacket::decode(&mut payload.as_slice()).unwrap_err();

        assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    }
}
