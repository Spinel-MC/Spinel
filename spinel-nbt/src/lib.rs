pub use compound::NbtCompound;
pub use serializer::to_bytes_unnamed;
use std::{
    error,
    fmt::{self, Display},
    io::{self, Read},
};
pub use tag::Nbt;

pub mod compound;
pub mod deserializer;
pub mod serializer;
pub mod tag;

pub const END_ID: u8 = 0x00;
pub const BYTE_ID: u8 = 0x01;
pub const SHORT_ID: u8 = 0x02;
pub const INT_ID: u8 = 0x03;
pub const LONG_ID: u8 = 0x04;
pub const FLOAT_ID: u8 = 0x05;
pub const DOUBLE_ID: u8 = 0x06;
pub const BYTE_ARRAY_ID: u8 = 0x07;
pub const STRING_ID: u8 = 0x08;
pub const LIST_ID: u8 = 0x09;
pub const COMPOUND_ID: u8 = 0x0A;
pub const INT_ARRAY_ID: u8 = 0x0B;
pub const LONG_ARRAY_ID: u8 = 0x0C;

#[derive(Debug)]
pub enum Error {
    NoRootCompound(u8),
    UnknownTagId(u8),
    Cesu8DecodingError,
    SerdeError(String),
    UnsupportedType(String),
    Incomplete(io::Error),
    NegativeLength(i32),
    LargeLength(usize),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NoRootCompound(id) => write!(
                f,
                "The root tag of the NBT file is not a compound tag. Received tag id: {}",
                id
            ),
            Error::UnknownTagId(id) => write!(f, "Encountered an unknown NBT tag id: {}.", id),
            Error::Cesu8DecodingError => write!(f, "Failed to Cesu 8 Decode"),
            Error::SerdeError(msg) => write!(f, "Serde error: {}", msg),
            Error::UnsupportedType(t) => write!(f, "NBT doesn't support this type: {}", t),
            Error::Incomplete(e) => write!(f, "NBT reading was cut short: {}", e),
            Error::NegativeLength(len) => write!(f, "Negative list length: {}", len),
            Error::LargeLength(len) => write!(f, "Length too large: {}", len),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Incomplete(io_err) => Some(io_err),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Incomplete(err)
    }
}

impl serde::ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::SerdeError(msg.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::SerdeError(msg.to_string())
    }
}

pub fn get_nbt_string<R: Read>(
    bytes: &mut deserializer::NbtReadHelper<R>,
) -> Result<String, Error> {
    let len = bytes.get_u16_be()? as usize;
    let string_bytes = bytes.read_boxed_slice(len)?;
    let string = cesu8::from_java_cesu8(&string_bytes).map_err(|_| Error::Cesu8DecodingError)?;
    Ok(string.to_string())
}
