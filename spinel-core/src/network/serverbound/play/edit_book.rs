use spinel_network::data_type::DataType;
use spinel_network::types::var_int::VarIntWrapper;
use spinel_network::{ConnectionState, PacketStruct};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditBookPacket {
    pub slot: VarIntWrapper,
    pub pages: Vec<String>,
    pub title: Option<String>,
}

impl EditBookPacket {
    const MAX_PAGE_COUNT: usize = 100;
    const MAX_PAGE_LENGTH: usize = 1024;
    const MAX_TITLE_LENGTH: usize = 32;

    pub const fn get_id_const() -> i32 {
        0x17
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Play
    }
}

impl DataType for EditBookPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        if self.pages.len() > Self::MAX_PAGE_COUNT {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "book page count exceeds maximum",
            ));
        }

        self.slot.encode(writer)?;
        VarIntWrapper(self.pages.len() as i32).encode(writer)?;
        for page in &self.pages {
            encode_bounded_string(writer, page, Self::MAX_PAGE_LENGTH)?;
        }

        match &self.title {
            Some(title) => {
                true.encode(writer)?;
                encode_bounded_string(writer, title, Self::MAX_TITLE_LENGTH)?;
            }
            None => false.encode(writer)?,
        }

        Ok(())
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let slot = VarIntWrapper::decode(reader)?;
        let page_count = decode_bounded_len(reader, Self::MAX_PAGE_COUNT)?;
        let mut pages = Vec::with_capacity(page_count);
        for _ in 0..page_count {
            pages.push(decode_bounded_string(reader, Self::MAX_PAGE_LENGTH)?);
        }

        let title = if bool::decode(reader)? {
            Some(decode_bounded_string(reader, Self::MAX_TITLE_LENGTH)?)
        } else {
            None
        };

        Ok(Self { slot, pages, title })
    }
}

impl PacketStruct for EditBookPacket {
    fn get_id() -> i32 {
        Self::get_id_const()
    }

    fn get_state() -> ConnectionState {
        Self::get_state_const()
    }
}

spinel_network::register_packet_codec!(EditBookPacket, spinel_network::Recipient::Server);

fn encode_bounded_string<W: Write>(
    writer: &mut W,
    value: &str,
    max_length: usize,
) -> io::Result<()> {
    if value.len() > max_length {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "string exceeds packet-specific maximum length",
        ));
    }

    VarIntWrapper(value.len() as i32).encode(writer)?;
    writer.write_all(value.as_bytes())
}

fn decode_bounded_string<R: Read>(reader: &mut R, max_length: usize) -> io::Result<String> {
    let string_length = decode_bounded_len(reader, max_length)?;
    let mut string_bytes = vec![0; string_length];
    reader.read_exact(&mut string_bytes)?;
    String::from_utf8(string_bytes)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

fn decode_bounded_len<R: Read>(reader: &mut R, max_length: usize) -> io::Result<usize> {
    let length = VarIntWrapper::decode(reader)?.0;
    if length < 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "length cannot be negative",
        ));
    }

    let length = length as usize;
    if length > max_length {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "length exceeds packet-specific maximum",
        ));
    }

    Ok(length)
}
