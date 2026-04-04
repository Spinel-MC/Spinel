use crate::data_type::DataType;
use crate::types::{GlobalPos, Identifier, VarInt, var_int::VarIntWrapper};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommonPlayerSpawnInfo {
    pub dimension_type: VarInt,
    pub dimension: Identifier,
    pub seed: i64,
    pub game_mode: u8,
    pub previous_game_mode: i8,
    pub is_debug: bool,
    pub is_flat: bool,
    pub last_death_location: Option<GlobalPos>,
    pub portal_cooldown: VarInt,
    pub sea_level: VarInt,
}

impl DataType for CommonPlayerSpawnInfo {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        VarIntWrapper(self.dimension_type).encode(w)?;
        self.dimension.encode(w)?;
        self.seed.encode(w)?;
        self.game_mode.encode(w)?;
        self.previous_game_mode.encode(w)?;
        self.is_debug.encode(w)?;
        self.is_flat.encode(w)?;
        self.last_death_location.encode(w)?;
        VarIntWrapper(self.portal_cooldown).encode(w)?;
        VarIntWrapper(self.sea_level).encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        Ok(Self {
            dimension_type: VarIntWrapper::decode(r)?.0,
            dimension: Identifier::decode(r)?,
            seed: i64::decode(r)?,
            game_mode: u8::decode(r)?,
            previous_game_mode: i8::decode(r)?,
            is_debug: bool::decode(r)?,
            is_flat: bool::decode(r)?,
            last_death_location: Option::<GlobalPos>::decode(r)?,
            portal_cooldown: VarIntWrapper::decode(r)?.0,
            sea_level: VarIntWrapper::decode(r)?.0,
        })
    }
}
