use crate::data_type::DataType;
use crate::types::{BitSet, PackedMessageSignature, var_int::VarIntWrapper};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterMaskType {
    PassThrough,
    FullyFiltered,
    PartiallyFiltered,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterMask {
    pub filter_mask_type: FilterMaskType,
    pub mask: BitSet,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PackedLastSeenMessages {
    pub entries: Vec<PackedMessageSignature>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedMessageBodyPacked {
    pub content: String,
    pub timestamp_millis: i64,
    pub salt: i64,
    pub last_seen: PackedLastSeenMessages,
}

impl FilterMaskType {
    fn from_protocol_id(protocol_id: i32) -> io::Result<Self> {
        match protocol_id {
            0 => Ok(Self::PassThrough),
            1 => Ok(Self::FullyFiltered),
            2 => Ok(Self::PartiallyFiltered),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unknown filter mask type {protocol_id}"),
            )),
        }
    }

    const fn protocol_id(self) -> i32 {
        match self {
            Self::PassThrough => 0,
            Self::FullyFiltered => 1,
            Self::PartiallyFiltered => 2,
        }
    }
}

impl DataType for FilterMaskType {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.protocol_id()).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Self::from_protocol_id(VarIntWrapper::decode(reader)?.0)
    }
}

impl DataType for FilterMask {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.filter_mask_type.encode(writer)?;
        if self.filter_mask_type == FilterMaskType::PartiallyFiltered {
            self.mask.encode(writer)?;
        }
        Ok(())
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let filter_mask_type = FilterMaskType::decode(reader)?;
        let mask = if filter_mask_type == FilterMaskType::PartiallyFiltered {
            BitSet::decode(reader)?
        } else {
            BitSet::default()
        };

        Ok(Self {
            filter_mask_type,
            mask,
        })
    }
}

impl DataType for PackedLastSeenMessages {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.entries.len() as i32).encode(writer)?;
        self.entries
            .iter()
            .try_for_each(|signature| signature.encode(writer))
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let entry_count = VarIntWrapper::decode(reader)?.0;
        if entry_count < 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "last seen message entry count cannot be negative",
            ));
        }

        let entries = (0..entry_count)
            .map(|_| PackedMessageSignature::decode(reader))
            .collect::<io::Result<Vec<_>>>()?;

        Ok(Self { entries })
    }
}

impl DataType for SignedMessageBodyPacked {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        if self.content.len() > 256 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "signed message content exceeds maximum length",
            ));
        }

        self.content.encode(writer)?;
        self.timestamp_millis.encode(writer)?;
        self.salt.encode(writer)?;
        self.last_seen.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let content = String::decode(reader)?;
        if content.len() > 256 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "signed message content exceeds maximum length",
            ));
        }

        Ok(Self {
            content,
            timestamp_millis: i64::decode(reader)?,
            salt: i64::decode(reader)?,
            last_seen: PackedLastSeenMessages::decode(reader)?,
        })
    }
}
