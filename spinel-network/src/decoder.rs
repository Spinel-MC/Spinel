use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
use aes::Aes128;
use cfb8::Decryptor;
use cfb8::cipher::{BlockDecryptMut, KeyIvInit, generic_array::GenericArray};
use flate2::read::ZlibDecoder;
use std::io::{self, Read};
use std::net::TcpStream;

pub struct PacketDecoder {
    decryptor: Option<Decryptor<Aes128>>,
    compression_threshold: Option<i32>,
}

impl PacketDecoder {
    pub fn new() -> Self {
        Self {
            decryptor: None,
            compression_threshold: None,
        }
    }

    pub fn enable_encryption(&mut self, key: &[u8]) {
        let key_arr = GenericArray::from_slice(key);
        let iv_arr = GenericArray::from_slice(key);
        self.decryptor = Some(Decryptor::new(key_arr, iv_arr));
    }

    pub fn set_compression(&mut self, threshold: i32) {
        if threshold >= 0 {
            self.compression_threshold = Some(threshold);
        } else {
            self.compression_threshold = None;
        }
    }

    pub fn read_frame(&mut self, stream: &mut TcpStream) -> io::Result<Vec<u8>> {
        let mut read_byte = |stream: &mut TcpStream| -> io::Result<u8> {
            let mut buf = [0u8; 1];
            stream.read_exact(&mut buf)?;
            if let Some(decryptor) = &mut self.decryptor {
                decryptor.decrypt_block_mut(GenericArray::from_mut_slice(&mut buf));
            }
            Ok(buf[0])
        };

        let mut read_varint = |stream: &mut TcpStream| -> io::Result<i32> {
            let mut num_read = 0;
            let mut result = 0;
            loop {
                let read = read_byte(stream)?;
                let value = (read & 0x7F) as i32;
                result |= value << (7 * num_read);
                if (read & 0x80) == 0 {
                    break;
                }
                num_read += 1;
                if num_read >= 5 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "VarInt is too big",
                    ));
                }
            }
            Ok(result)
        };

        let packet_length = read_varint(stream)?;

        let mut data = vec![0u8; packet_length as usize];
        stream.read_exact(&mut data)?;
        if let Some(decryptor) = &mut self.decryptor {
            for byte in &mut data {
                let mut block = cfb8::cipher::generic_array::GenericArray::<
                    u8,
                    cfb8::cipher::typenum::U1,
                >::from_mut_slice(std::slice::from_mut(byte));
                decryptor.decrypt_block_mut(&mut block);
            }
        }

        if let Some(_) = self.compression_threshold {
            let mut cursor = io::Cursor::new(&data);
            let data_length = VarIntWrapper::decode(&mut cursor)?.0;

            if data_length == 0 {
                let position = cursor.position() as usize;
                Ok(data[position..].to_vec())
            } else {
                let position = cursor.position() as usize;
                let compressed_data = &data[position..];
                let mut decoder = ZlibDecoder::new(compressed_data);
                let mut decompressed = Vec::with_capacity(data_length as usize);
                decoder.read_to_end(&mut decompressed)?;
                if decompressed.len() != data_length as usize {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Decompressed length mismatch",
                    ));
                }
                Ok(decompressed)
            }
        } else {
            Ok(data)
        }
    }
}

impl Default for PacketDecoder {
    fn default() -> Self {
        Self::new()
    }
}
