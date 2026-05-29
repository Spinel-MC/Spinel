use spinel_macros::packet;
use spinel_network::data_type::DataType;
use std::io::{self, Read, Write};

#[packet(id: "move_vehicle", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct VehicleMovePacket {
    pub position: VehiclePosition,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VehiclePosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

impl DataType for VehiclePosition {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.x.encode(writer)?;
        self.y.encode(writer)?;
        self.z.encode(writer)?;
        self.yaw.encode(writer)?;
        self.pitch.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            x: f64::decode(reader)?,
            y: f64::decode(reader)?,
            z: f64::decode(reader)?,
            yaw: f32::decode(reader)?,
            pitch: f32::decode(reader)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{VehicleMovePacket, VehiclePosition};
    use spinel_network::DataType;

    #[test]
    fn vehicle_move_packet_matches_minestom_pos_shape() {
        let packet = VehicleMovePacket {
            position: VehiclePosition {
                x: 1.0,
                y: 2.0,
                z: 3.0,
                yaw: 90.0,
                pitch: 45.0,
            },
        };
        let mut payload = Vec::new();

        packet.encode(&mut payload).unwrap();
        let decoded_packet = VehicleMovePacket::decode(&mut payload.as_slice()).unwrap();

        assert_eq!(VehicleMovePacket::get_id(), 0x37);
        assert_eq!(decoded_packet.position, packet.position);
    }
}
