use crate::entity::EntityId;
use crate::network::client::metadata::LoginMetadata;
use crate::network::client::writer::OutboundPacketWriter;
use crate::network::login::plugin_message::{
    LoginPluginMessageProcessor, LoginPluginResponseFuture, UnexpectedLoginPluginResponseError,
};
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
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
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
    player_entity_id: Option<EntityId>,
    pub pending_encryption: Option<Vec<u8>>,
    pub pending_compression: Option<i32>,
    pub pending_client_settings: ClientInformation,
    login_plugin_message_processor: LoginPluginMessageProcessor,
    is_online: Arc<AtomicBool>,
    pub(super) alive_time: u64,
    pub(super) alive_pending: bool,
    pub(super) alive_id: u64,
    pub(super) latency_millis: u32,
    outbound_packet_queue: VecDeque<QueuedOutboundPacket>,
    outbound_packet_queue_enabled: bool,
    outbound_packet_writer: Option<OutboundPacketWriter>,
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
            player_entity_id: None,
            pending_encryption: None,
            pending_compression: None,
            pending_client_settings: ClientInformation::default(),
            login_plugin_message_processor: LoginPluginMessageProcessor::new(),
            is_online: Arc::new(AtomicBool::new(true)),
            alive_time: 0,
            alive_pending: false,
            alive_id: 0,
            latency_millis: 0,
            outbound_packet_queue: VecDeque::new(),
            outbound_packet_queue_enabled: false,
            outbound_packet_writer: None,
        }
    }

    pub fn close_connection(&mut self) {
        self.is_online.store(false, Ordering::SeqCst);
        self.outbound_packet_queue.clear();
        if let Some(writer) = &self.outbound_packet_writer {
            writer.close(Shutdown::Both);
        } else {
            let _ = self.stream.shutdown(Shutdown::Both);
        }
    }

    pub(crate) fn close_after_disconnect_packet(&mut self) {
        self.is_online.store(false, Ordering::SeqCst);
        self.outbound_packet_queue.clear();
        if let Some(writer) = &self.outbound_packet_writer {
            writer.close(Shutdown::Write);
        } else {
            let _ = self.stream.shutdown(Shutdown::Write);
        }
    }

    pub fn is_online(&self) -> bool {
        self.is_online.load(Ordering::SeqCst)
            && self
                .outbound_packet_writer
                .as_ref()
                .is_none_or(OutboundPacketWriter::is_online)
    }

    pub(crate) fn connection_status(&self) -> Arc<AtomicBool> {
        self.is_online.clone()
    }

    pub(crate) const fn player_entity_id(&self) -> Option<EntityId> {
        self.player_entity_id
    }

    pub(crate) fn set_player_entity_id(&mut self, player_entity_id: EntityId) {
        self.player_entity_id = Some(player_entity_id);
    }

    pub fn enable_encryption(&mut self, key: &[u8]) {
        if let Some(writer) = &self.outbound_packet_writer {
            writer.enable_encryption(key);
        } else {
            self.encoder.enable_encryption(key);
        }
        self.pending_encryption = Some(key.to_vec());
        println!("Encryption enabled for client {} (writer)", self.addr);
    }

    pub fn set_compression(&mut self, threshold: i32) {
        if let Some(writer) = &self.outbound_packet_writer {
            writer.set_compression(threshold);
        } else {
            self.encoder.set_compression(threshold);
        }
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
        if let Some(writer) = &self.outbound_packet_writer {
            return writer.send_raw_bytes(bytes.to_vec());
        }
        use std::io::Write;
        self.stream.write_all(bytes)
    }

    pub fn send_login_plugin_request(
        &mut self,
        channel: impl Into<String>,
        request_payload: Option<Vec<u8>>,
    ) -> io::Result<LoginPluginResponseFuture> {
        let mut processor = std::mem::take(&mut self.login_plugin_message_processor);
        let response_future = processor.request(self, channel, request_payload);
        self.login_plugin_message_processor = processor;
        response_future
    }

    pub fn complete_login_plugin_response(
        &mut self,
        message_id: i32,
        response_data: Option<Vec<u8>>,
    ) -> Result<(), UnexpectedLoginPluginResponseError> {
        self.login_plugin_message_processor
            .complete(message_id, response_data)
    }

    pub fn has_pending_login_plugin_requests(&self) -> bool {
        self.login_plugin_message_processor.has_pending_requests()
    }

    pub fn login_plugin_requests_have_timed_out(&self, now: Instant) -> bool {
        self.login_plugin_message_processor.has_timed_out(now)
    }

    #[cfg(test)]
    pub(crate) fn pending_login_plugin_message_ids(&self) -> Vec<i32> {
        self.login_plugin_message_processor.pending_message_ids()
    }

    pub(crate) fn enable_outbound_packet_queue(&mut self) {
        self.outbound_packet_queue_enabled = true;
        if self.outbound_packet_writer.is_none()
            && let Ok(writer_stream) = self.stream.try_clone()
        {
            self.outbound_packet_writer = Some(OutboundPacketWriter::new(
                writer_stream,
                self.is_online.clone(),
            ));
        }
    }

    pub(crate) fn outbound_packet_queue_enabled(&self) -> bool {
        self.outbound_packet_queue_enabled
    }

    #[cfg(test)]
    pub(crate) fn queued_outbound_packet_count(&self) -> usize {
        self.outbound_packet_queue.len()
    }

    #[cfg(test)]
    pub(crate) fn pending_outbound_packet_count(&self) -> usize {
        self.outbound_packet_queue.len()
            + self
                .outbound_packet_writer
                .as_ref()
                .map_or(0, OutboundPacketWriter::pending_packet_count)
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
        self.dispatch_outbound_write_errors();
        if !self.is_online() {
            self.outbound_packet_queue.clear();
            return Ok(0);
        }
        let mut flushed_packets = 0;
        while let Some(packet) = self.outbound_packet_queue.front().cloned() {
            self.state = packet.state;
            self.dispatch_queued_outbound_packet(packet.packet_id, &packet.payload)?;
            self.outbound_packet_queue.pop_front();
            flushed_packets += 1;
        }
        self.dispatch_outbound_write_errors();
        Ok(flushed_packets)
    }

    pub(crate) fn send_packet_immediately(
        &mut self,
        id: i32,
        payload: &[u8],
        packet_name: String,
    ) -> io::Result<()> {
        if !self.is_online() {
            return Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "connection is closed",
            ));
        }
        if let Some(writer) = &self.outbound_packet_writer {
            return writer.send_packet(id, packet_name, payload.to_vec());
        }
        let frame = self.encoder.encode_frame(id, payload)?;
        use std::io::Write;
        self.stream.write_all(&frame)
    }

    pub(crate) fn dispatch_outbound_write_errors(&mut self) {
        let Some(writer) = &self.outbound_packet_writer else {
            return;
        };
        writer.take_errors().into_iter().for_each(|error| {
            self.dispatch_packet_error(error.packet_id, error.packet_name, error.message);
        });
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
