use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
use aes::Aes128;
use cfb8::Encryptor;
use cfb8::cipher::{BlockEncryptMut, KeyIvInit, generic_array::GenericArray};
use flate2::{Compress, Compression, FlushCompress, Status};
use std::io::{self, Write};
use std::net::TcpStream;

pub use crate::network_buffer::NetworkBuffer;

pub struct PacketEncoder {
    encryptor: Option<Encryptor<Aes128>>,
    compression_threshold: Option<i32>,
    compressor: Compress,
}

impl PacketEncoder {
    pub fn new() -> Self {
        Self {
            encryptor: None,
            compression_threshold: None,
            compressor: Compress::new(Compression::fast(), true),
        }
    }

    pub fn enable_encryption(&mut self, key: &[u8]) {
        let key_arr = GenericArray::from_slice(key);
        let iv_arr = GenericArray::from_slice(key);
        self.encryptor = Some(Encryptor::new(key_arr, iv_arr));
    }

    pub fn set_compression(&mut self, threshold: i32) {
        if threshold >= 0 {
            self.compression_threshold = Some(threshold);
        } else {
            self.compression_threshold = None;
        }
    }

    pub fn write_frame(
        &mut self,
        stream: &mut TcpStream,
        packet_id: i32,
        payload: &[u8],
    ) -> io::Result<()> {
        let output_buffer = self.encode_frame(packet_id, payload)?;
        stream.write_all(&output_buffer)
    }

    pub fn encode_frame(&mut self, packet_id: i32, payload: &[u8]) -> io::Result<Vec<u8>> {
        let mut raw_packet = Vec::new();
        VarIntWrapper(packet_id).encode(&mut raw_packet)?;
        raw_packet.extend_from_slice(payload);

        let final_data = if let Some(threshold) = self.compression_threshold {
            let data_len = raw_packet.len();
            if data_len >= threshold as usize {
                let mut comp_frame = Vec::new();
                VarIntWrapper(data_len as i32).encode(&mut comp_frame)?;

                let mut compressed =
                    Vec::with_capacity(data_len + data_len.saturating_div(100) + 64);
                self.compressor.reset();
                let status = self
                    .compressor
                    .compress_vec(&raw_packet, &mut compressed, FlushCompress::Finish)
                    .map_err(io::Error::other)?;
                if status != Status::StreamEnd {
                    return Err(io::Error::other(
                        "packet compression output buffer was too small",
                    ));
                }
                comp_frame.extend_from_slice(&compressed);

                comp_frame
            } else {
                let mut comp_frame = Vec::new();
                VarIntWrapper(0).encode(&mut comp_frame)?;
                comp_frame.extend_from_slice(&raw_packet);
                comp_frame
            }
        } else {
            raw_packet
        };

        let mut output_buffer = Vec::new();
        VarIntWrapper(final_data.len() as i32).encode(&mut output_buffer)?;
        output_buffer.extend_from_slice(&final_data);

        if let Some(encryptor) = &mut self.encryptor {
            for byte in &mut output_buffer {
                let mut block = cfb8::cipher::generic_array::GenericArray::<
                    u8,
                    cfb8::cipher::typenum::U1,
                >::from_mut_slice(std::slice::from_mut(byte));
                encryptor.encrypt_block_mut(&mut block);
            }
        }

        Ok(output_buffer)
    }
}

impl Default for PacketEncoder {
    fn default() -> Self {
        Self::new()
    }
}
