use crate::network::client::metadata::LoginMetadata;
use spinel_network::ConnectionState;
use spinel_network::DataType;
use spinel_network::PacketSender;
use spinel_network::VarIntWrapper;
use spinel_network::encoder::PacketEncoder;
use spinel_network::types::{Position, Slot};
use spinel_network::wrappers::JsonTextComponent;
use spinel_utils::component::text::TextComponent;
use std::io::{self, Cursor, Error, ErrorKind, Read};
use std::net::{Shutdown, SocketAddr, TcpStream};
use uuid::Uuid;

/// Represents a connected player on the server.
pub struct Client {
    pub stream: TcpStream,
    pub encoder: PacketEncoder,
    pub addr: SocketAddr,
    pub state: ConnectionState,
    pub login_metadata: Option<LoginMetadata>,
    pub payload_cursor: Option<Cursor<Vec<u8>>>,
    // Pending updates for the reader (network loop has the decoder)
    pub pending_encryption: Option<Vec<u8>>,
    pub pending_compression: Option<i32>,
}

impl Client {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> Self {
        Self {
            stream,
            encoder: PacketEncoder::new(),
            addr,
            state: ConnectionState::Handshaking,
            login_metadata: None,
            payload_cursor: None,
            pending_encryption: None,
            pending_compression: None,
        }
    }

    pub fn disconnect(&mut self) {
        let _ = self.stream.shutdown(Shutdown::Both);
    }

    pub fn enable_encryption(&mut self, key: &[u8]) {
        self.encoder.enable_encryption(key);
        self.pending_encryption = Some(key.to_vec());
        println!("Encryption enabled for client {} (writer)", self.addr);
    }

    pub fn set_compression(&mut self, threshold: i32) {
        self.encoder.set_compression(threshold);
        self.pending_compression = Some(threshold);
        println!(
            "Compression set to {} for client {} (writer)",
            threshold, self.addr
        );
    }

    pub fn decode<T: DataType>(&mut self) -> io::Result<T> {
        if let Some(cursor) = &mut self.payload_cursor {
            T::decode(cursor)
        } else {
            Err(Error::new(ErrorKind::NotConnected, "No payload cursor set"))
        }
    }

    pub fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        if let Some(cursor) = &mut self.payload_cursor {
            cursor.read_exact(buf)
        } else {
            Err(Error::new(ErrorKind::NotConnected, "No payload cursor set"))
        }
    }

    pub fn read_varint(&mut self) -> io::Result<i32> {
        Ok(self.decode::<VarIntWrapper>()?.0)
    }

    pub fn send_raw_bytes(&mut self, bytes: &[u8]) -> io::Result<()> {
        use std::io::Write;
        self.stream.write_all(bytes)
    }

    pub fn read_byte(&mut self) -> io::Result<i8> {
        self.decode()
    }
    pub fn read_unsigned_byte(&mut self) -> io::Result<u8> {
        self.decode()
    }
    pub fn read_short(&mut self) -> io::Result<i16> {
        self.decode()
    }
    pub fn read_unsigned_short(&mut self) -> io::Result<u16> {
        self.decode()
    }
    pub fn read_int(&mut self) -> io::Result<i32> {
        self.decode()
    }
    pub fn read_long(&mut self) -> io::Result<i64> {
        self.decode()
    }
    pub fn read_float(&mut self) -> io::Result<f32> {
        self.decode()
    }
    pub fn read_double(&mut self) -> io::Result<f64> {
        self.decode()
    }
    pub fn read_bool(&mut self) -> io::Result<bool> {
        self.decode()
    }
    pub fn read_string(&mut self) -> io::Result<String> {
        self.decode()
    }
    pub fn read_string_with_limit(&mut self, max_len: usize) -> io::Result<String> {
        let s = self.read_string()?;
        if s.len() > max_len {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "String limit exceeded",
            ));
        }
        Ok(s)
    }
    pub fn read_uuid(&mut self) -> io::Result<Uuid> {
        self.decode()
    }
    pub fn read_position(&mut self) -> io::Result<Position> {
        self.decode()
    }
    pub fn read_byte_array(&mut self) -> io::Result<Vec<u8>> {
        self.decode()
    }
    pub fn read_slot(&mut self) -> io::Result<Slot> {
        self.decode()
    }
    pub fn read_json_text_component(&mut self) -> io::Result<TextComponent> {
        Ok(self.decode::<JsonTextComponent>()?.0)
    }
}

impl PacketSender for Client {
    fn send_packet(&mut self, id: i32, payload: &[u8]) {
        if let Err(e) = self.encoder.write_frame(&mut self.stream, id, payload) {
            eprintln!("Failed to send packet to {}: {}", self.addr, e);
        }
        let packet_name = spinel_network::packet_names::get_clientbound_packet_name(self.state, id);
        println!(
            "[Clientbound]: State={:?}, ID={:#04X}, resource=\"{}\", PayloadSize={}",
            self.state,
            id,
            packet_name,
            payload.len()
        );
    }
}

impl Read for Client {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if let Some(cursor) = &mut self.payload_cursor {
            cursor.read(buf)
        } else {
            Ok(0)
        }
    }
}
