use crate::data_type::DataType;
use crate::types::{Position, VarInt};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlockHitResult {
    pub position: Position,
    pub direction: VarInt,
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub cursor_z: f32,
    pub is_inside: bool,
    pub world_coordinate_index: Option<VarInt>,
}

impl DataType for BlockHitResult {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.position.encode(w)?;
        self.direction.encode(w)?;
        self.cursor_x.encode(w)?;
        self.cursor_y.encode(w)?;
        self.cursor_z.encode(w)?;
        self.is_inside.encode(w)?;
        if let Some(idx) = self.world_coordinate_index {
            true.encode(w)?;
            idx.encode(w)?;
        } else {
            false.encode(w)?;
        }
        Ok(())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let position = Position::decode(r)?;
        let direction = VarInt::decode(r)?;
        let cursor_x = f32::decode(r)?;
        let cursor_y = f32::decode(r)?;
        let cursor_z = f32::decode(r)?;
        let is_inside = bool::decode(r)?;
        let world_coordinate_index = if bool::decode(r)? {
            Some(VarInt::decode(r)?)
        } else {
            None
        };

        Ok(BlockHitResult {
            position,
            direction,
            cursor_x,
            cursor_y,
            cursor_z,
            is_inside,
            world_coordinate_index,
        })
    }
}
