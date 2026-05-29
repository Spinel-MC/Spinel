use spinel_network::data_type::DataType;
use spinel_network::types::Vector3d;
use spinel_network::types::sound::SoundEvent;
use spinel_network::{ConnectionState, PacketSender, PacketStruct, VarIntWrapper};
use std::io::{self, Read, Write};

pub struct ExplosionPacket {
    pub center: Vector3d,
    pub radius: f32,
    pub block_count: i32,
    pub player_knockback: Option<Vector3d>,
    pub particle: ExplosionParticle,
    pub sound: SoundEvent,
    pub block_particles: Vec<ExplosionBlockParticleInfo>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExplosionParticle {
    Explosion,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExplosionBlockParticleInfo {
    pub particle: ExplosionParticle,
    pub scaling: f32,
    pub speed: f32,
}

impl ExplosionPacket {
    pub const fn get_id() -> i32 {
        0x24
    }

    pub const fn get_id_const() -> i32 {
        0x24
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

impl DataType for ExplosionPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.center.encode(writer)?;
        self.radius.encode(writer)?;
        self.block_count.encode(writer)?;
        self.player_knockback.encode(writer)?;
        self.particle.encode(writer)?;
        self.sound.encode(writer)?;
        self.block_particles.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            center: Vector3d::decode(reader)?,
            radius: f32::decode(reader)?,
            block_count: i32::decode(reader)?,
            player_knockback: Option::<Vector3d>::decode(reader)?,
            particle: ExplosionParticle::decode(reader)?,
            sound: SoundEvent::decode(reader)?,
            block_particles: Vec::<ExplosionBlockParticleInfo>::decode(reader)?,
        })
    }
}

impl PacketStruct for ExplosionPacket {
    fn get_id() -> i32 {
        0x24
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}

impl DataType for ExplosionParticle {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.protocol_id()).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        match VarIntWrapper::decode(reader)?.0 {
            15 => Ok(Self::Explosion),
            particle_id => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unknown explosion particle type: {particle_id}"),
            )),
        }
    }
}

impl ExplosionParticle {
    pub const fn protocol_id(self) -> i32 {
        match self {
            Self::Explosion => 15,
        }
    }
}

impl DataType for ExplosionBlockParticleInfo {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.particle.encode(writer)?;
        self.scaling.encode(writer)?;
        self.speed.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            particle: ExplosionParticle::decode(reader)?,
            scaling: f32::decode(reader)?,
            speed: f32::decode(reader)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{ExplosionPacket, ExplosionParticle};
    use spinel_network::data_type::DataType;
    use spinel_network::types::Vector3d;
    use spinel_network::types::sound::SoundEvent;

    #[test]
    fn explosion_packet_matches_minestom_wire_shape() {
        let packet = ExplosionPacket {
            center: Vector3d {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            radius: 0.0,
            block_count: 0,
            player_knockback: None,
            particle: ExplosionParticle::Explosion,
            sound: SoundEvent::Id(668),
            block_particles: Vec::new(),
        };
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();
        let decoded = ExplosionPacket::decode(&mut payload.as_slice()).unwrap();

        assert_eq!(ExplosionPacket::get_id(), 0x24);
        assert_eq!(decoded.center, packet.center);
        assert_eq!(decoded.radius, 0.0);
        assert_eq!(decoded.block_count, 0);
        assert_eq!(decoded.player_knockback, None);
        assert_eq!(decoded.particle, ExplosionParticle::Explosion);
        assert_eq!(decoded.sound, SoundEvent::Id(668));
        assert!(decoded.block_particles.is_empty());
    }
}
