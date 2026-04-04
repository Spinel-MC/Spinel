use crate::data_type::DataType;
use crate::types::{Identifier, Position};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GlobalPos {
    pub dimension: Identifier,
    pub position: Position,
}

impl DataType for GlobalPos {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.dimension.encode(w)?;
        self.position.encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        Ok(GlobalPos {
            dimension: Identifier::decode(r)?,
            position: Position::decode(r)?,
        })
    }
}
