use spinel_network::data_type::DataType;
use spinel_network::types::var_int::VarIntWrapper;
use spinel_network::{ConnectionState, PacketStruct};
use std::collections::HashMap;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomReportDetailsPacket {
    pub details: HashMap<String, String>,
}

impl CustomReportDetailsPacket {
    const MAX_DETAIL_COUNT: usize = 32;
    const MAX_DETAIL_KEY_LENGTH: usize = 128;
    const MAX_DETAIL_VALUE_LENGTH: usize = 4096;

    pub const fn get_id_const() -> i32 {
        0x0f
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Configuration
    }
}

impl DataType for CustomReportDetailsPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        if self.details.len() > Self::MAX_DETAIL_COUNT {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "custom report details exceed maximum entry count",
            ));
        }

        VarIntWrapper(self.details.len() as i32).encode(writer)?;
        for (detail_key, detail_value) in &self.details {
            encode_bounded_string(writer, detail_key, Self::MAX_DETAIL_KEY_LENGTH)?;
            encode_bounded_string(writer, detail_value, Self::MAX_DETAIL_VALUE_LENGTH)?;
        }

        Ok(())
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let detail_count = decode_bounded_len(reader, Self::MAX_DETAIL_COUNT)?;
        let mut details = HashMap::with_capacity(detail_count);
        for _ in 0..detail_count {
            let detail_key = decode_bounded_string(reader, Self::MAX_DETAIL_KEY_LENGTH)?;
            let detail_value = decode_bounded_string(reader, Self::MAX_DETAIL_VALUE_LENGTH)?;
            details.insert(detail_key, detail_value);
        }

        Ok(Self { details })
    }
}

impl PacketStruct for CustomReportDetailsPacket {
    fn get_id() -> i32 {
        Self::get_id_const()
    }

    fn get_state() -> ConnectionState {
        Self::get_state_const()
    }
}

spinel_network::register_packet_codec!(
    CustomReportDetailsPacket,
    spinel_network::Recipient::Client
);

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
