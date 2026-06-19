use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
use spinel_nbt::Nbt;
use std::io::{self, Cursor, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomClickActionPayload {
    tag_bytes: Option<Vec<u8>>,
}

impl CustomClickActionPayload {
    const MAX_LENGTH: usize = 65_536;

    pub const fn none() -> Self {
        Self { tag_bytes: None }
    }

    pub fn from_tag(tag: Nbt) -> io::Result<Self> {
        let mut tag_bytes = Vec::new();
        tag.write_unnamed(&mut tag_bytes)?;
        Self::from_tag_bytes(Some(tag_bytes))
    }

    pub fn from_tag_bytes(tag_bytes: Option<Vec<u8>>) -> io::Result<Self> {
        if let Some(tag_bytes) = &tag_bytes {
            Self::validate_tag_bytes(tag_bytes)?;
        }

        Ok(Self { tag_bytes })
    }

    pub fn bytes(&self) -> Option<&[u8]> {
        self.tag_bytes.as_deref()
    }

    pub fn tag(&self) -> io::Result<Option<Nbt>> {
        let Some(tag_bytes) = &self.tag_bytes else {
            return Ok(None);
        };

        let mut reader = Cursor::new(tag_bytes.as_slice());
        let tag = Nbt::read_unnamed(&mut reader)?;
        if reader.position() != tag_bytes.len() as u64 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "custom click action payload contains trailing bytes",
            ));
        }

        Ok(Some(tag))
    }

    fn validate_tag_bytes(tag_bytes: &[u8]) -> io::Result<()> {
        if tag_bytes.len() > Self::MAX_LENGTH {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "custom click action payload exceeds maximum length",
            ));
        }

        let mut reader = Cursor::new(tag_bytes);
        Nbt::read_unnamed(&mut reader)?;
        if reader.position() != tag_bytes.len() as u64 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "custom click action payload contains trailing bytes",
            ));
        }

        Ok(())
    }
}

impl Default for CustomClickActionPayload {
    fn default() -> Self {
        Self::none()
    }
}

impl DataType for CustomClickActionPayload {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let payload_bytes = self.tag_bytes.as_deref().unwrap_or(&[0]);
        if payload_bytes.len() > Self::MAX_LENGTH {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "custom click action payload exceeds maximum length",
            ));
        }

        VarIntWrapper(payload_bytes.len() as i32).encode(writer)?;
        writer.write_all(payload_bytes)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let payload_length = VarIntWrapper::decode(reader)?.0;
        if payload_length < 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "custom click action payload length cannot be negative",
            ));
        }

        let payload_length = payload_length as usize;
        if payload_length > Self::MAX_LENGTH {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "custom click action payload exceeds maximum length",
            ));
        }

        let mut payload_bytes = vec![0; payload_length];
        reader.read_exact(&mut payload_bytes)?;
        if payload_bytes == [0] {
            return Ok(Self::none());
        }

        Self::from_tag_bytes(Some(payload_bytes))
    }
}
