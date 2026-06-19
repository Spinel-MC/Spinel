use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecipeBookType {
    Crafting,
    Furnace,
    BlastFurnace,
    Smoker,
}

impl DataType for RecipeBookType {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(match self {
            Self::Crafting => 0,
            Self::Furnace => 1,
            Self::BlastFurnace => 2,
            Self::Smoker => 3,
        })
        .encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        match VarIntWrapper::decode(reader)?.0 {
            0 => Ok(Self::Crafting),
            1 => Ok(Self::Furnace),
            2 => Ok(Self::BlastFurnace),
            3 => Ok(Self::Smoker),
            protocol_id => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unknown recipe book type protocol id {protocol_id}"),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecipeDisplayId {
    pub index: i32,
}

impl DataType for RecipeDisplayId {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.index).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            index: VarIntWrapper::decode(reader)?.0,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecipeBookTypeSettings {
    pub open: bool,
    pub filtering: bool,
}

impl DataType for RecipeBookTypeSettings {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.open.encode(writer)?;
        self.filtering.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            open: bool::decode(reader)?,
            filtering: bool::decode(reader)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecipeBookSettings {
    pub crafting: RecipeBookTypeSettings,
    pub furnace: RecipeBookTypeSettings,
    pub blast_furnace: RecipeBookTypeSettings,
    pub smoker: RecipeBookTypeSettings,
}

impl DataType for RecipeBookSettings {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.crafting.encode(writer)?;
        self.furnace.encode(writer)?;
        self.blast_furnace.encode(writer)?;
        self.smoker.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            crafting: RecipeBookTypeSettings::decode(reader)?,
            furnace: RecipeBookTypeSettings::decode(reader)?,
            blast_furnace: RecipeBookTypeSettings::decode(reader)?,
            smoker: RecipeBookTypeSettings::decode(reader)?,
        })
    }
}
