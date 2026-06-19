use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
use std::io::{self, Read, Write};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfilePublicKeyData {
    pub expires_at_millis: i64,
    pub encoded_public_key: Vec<u8>,
    pub key_signature: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteChatSessionData {
    pub session_id: Uuid,
    pub profile_public_key: ProfilePublicKeyData,
}

impl ProfilePublicKeyData {
    const MAX_PUBLIC_KEY_LENGTH: usize = 512;
    const MAX_KEY_SIGNATURE_LENGTH: usize = 4096;

    fn encode_bounded_bytes<W: Write>(
        writer: &mut W,
        bytes: &[u8],
        maximum_length: usize,
    ) -> io::Result<()> {
        if bytes.len() > maximum_length {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("byte array exceeds maximum length {maximum_length}"),
            ));
        }

        VarIntWrapper(bytes.len() as i32).encode(writer)?;
        writer.write_all(bytes)
    }

    fn decode_bounded_bytes<R: Read>(reader: &mut R, maximum_length: usize) -> io::Result<Vec<u8>> {
        let byte_count = VarIntWrapper::decode(reader)?.0;
        if byte_count < 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "byte array length cannot be negative",
            ));
        }

        let byte_count = byte_count as usize;
        if byte_count > maximum_length {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("byte array exceeds maximum length {maximum_length}"),
            ));
        }

        let mut bytes = vec![0; byte_count];
        reader.read_exact(&mut bytes)?;
        Ok(bytes)
    }
}

impl DataType for ProfilePublicKeyData {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.expires_at_millis.encode(writer)?;
        Self::encode_bounded_bytes(
            writer,
            &self.encoded_public_key,
            Self::MAX_PUBLIC_KEY_LENGTH,
        )?;
        Self::encode_bounded_bytes(writer, &self.key_signature, Self::MAX_KEY_SIGNATURE_LENGTH)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            expires_at_millis: i64::decode(reader)?,
            encoded_public_key: Self::decode_bounded_bytes(reader, Self::MAX_PUBLIC_KEY_LENGTH)?,
            key_signature: Self::decode_bounded_bytes(reader, Self::MAX_KEY_SIGNATURE_LENGTH)?,
        })
    }
}

impl DataType for RemoteChatSessionData {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.session_id.encode(writer)?;
        self.profile_public_key.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            session_id: Uuid::decode(reader)?,
            profile_public_key: ProfilePublicKeyData::decode(reader)?,
        })
    }
}
