use crate::network::client::metadata::LoginMetadata;
use spinel_network::ConnectionState;
use spinel_network::DataType;
use spinel_network::VarIntWrapper;
use spinel_network::encoder::PacketEncoder;
use spinel_network::types::{ClientInformation, Position, Slot};
use spinel_network::wrappers::JsonTextComponent;
use spinel_utils::component::text::TextComponent;
use std::collections::VecDeque;
use std::io::{self, Cursor, Error, ErrorKind, Read};
use std::net::{Shutdown, SocketAddr, TcpStream};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct QueuedOutboundPacket {
    pub(crate) state: ConnectionState,
    pub(crate) packet_id: i32,
    pub(crate) payload: Vec<u8>,
}

pub struct Client {
    pub stream: TcpStream,
    pub encoder: PacketEncoder,
    pub addr: SocketAddr,
    pub state: ConnectionState,
    pub login_metadata: Option<LoginMetadata>,
    pub payload_cursor: Option<Cursor<Vec<u8>>>,
    pub server_ptr: Option<usize>,
    pub pending_encryption: Option<Vec<u8>>,
    pub pending_compression: Option<i32>,
    pub pending_client_settings: ClientInformation,
    is_online: bool,
    pub(super) alive_time: u64,
    pub(super) alive_pending: bool,
    pub(super) alive_id: u64,
    pub(super) latency_millis: u32,
    outbound_packet_queue: VecDeque<QueuedOutboundPacket>,
    outbound_packet_queue_enabled: bool,
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
            server_ptr: None,
            pending_encryption: None,
            pending_compression: None,
            pending_client_settings: ClientInformation::default(),
            is_online: true,
            alive_time: 0,
            alive_pending: false,
            alive_id: 0,
            latency_millis: 0,
            outbound_packet_queue: VecDeque::new(),
            outbound_packet_queue_enabled: false,
        }
    }

    pub fn disconnect(&mut self) {
        let _ = self.flush_outbound_packets();
        self.is_online = false;
        let _ = self.stream.shutdown(Shutdown::Both);
    }

    pub fn finish_disconnect_packet(&mut self) {
        let _ = self.flush_outbound_packets();
        self.is_online = false;
        let _ = self.stream.shutdown(Shutdown::Write);
    }

    pub const fn is_online(&self) -> bool {
        self.is_online
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

    pub(crate) fn enable_outbound_packet_queue(&mut self) {
        self.outbound_packet_queue_enabled = true;
    }

    pub(crate) fn outbound_packet_queue_enabled(&self) -> bool {
        self.outbound_packet_queue_enabled
    }

    #[cfg(test)]
    pub(crate) fn queued_outbound_packet_count(&self) -> usize {
        self.outbound_packet_queue.len()
    }

    #[cfg(test)]
    pub(crate) fn queued_outbound_packet_ids(&self) -> Vec<i32> {
        self.outbound_packet_queue
            .iter()
            .map(|packet| packet.packet_id)
            .collect()
    }

    #[cfg(test)]
    pub(crate) fn queued_outbound_packet_payloads(&self) -> Vec<(i32, Vec<u8>)> {
        self.outbound_packet_queue
            .iter()
            .map(|packet| (packet.packet_id, packet.payload.clone()))
            .collect()
    }

    #[cfg(test)]
    pub(crate) fn discard_queued_outbound_packets(&mut self) {
        self.outbound_packet_queue.clear();
    }

    pub(crate) fn enqueue_outbound_packet(&mut self, packet_id: i32, payload: Vec<u8>) {
        self.outbound_packet_queue.push_back(QueuedOutboundPacket {
            state: self.state,
            packet_id,
            payload,
        });
    }

    pub(crate) fn flush_outbound_packets(&mut self) -> io::Result<usize> {
        let mut flushed_packets = 0;
        while let Some(packet) = self.outbound_packet_queue.pop_front() {
            self.state = packet.state;
            self.send_packet_immediately(packet.packet_id, &packet.payload)?;
            flushed_packets += 1;
        }
        Ok(flushed_packets)
    }

    pub(crate) fn send_packet_immediately(&mut self, id: i32, payload: &[u8]) -> io::Result<()> {
        self.encoder.write_frame(&mut self.stream, id, payload)
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

impl Read for Client {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if let Some(cursor) = &mut self.payload_cursor {
            cursor.read(buf)
        } else {
            Ok(0)
        }
    }
}
