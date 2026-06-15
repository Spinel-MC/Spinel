use spinel_network::encoder::PacketEncoder;
use std::collections::VecDeque;
use std::io::{self, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, mpsc};

pub(crate) struct OutboundPacketWriter {
    command_sender: mpsc::Sender<OutboundWriteCommand>,
    errors: Arc<Mutex<VecDeque<OutboundWriteError>>>,
    is_online: Arc<AtomicBool>,
    cancel_pending: Arc<AtomicBool>,
    pending_packet_count: Arc<AtomicUsize>,
}

pub(crate) struct OutboundWriteError {
    pub(crate) packet_id: i32,
    pub(crate) packet_name: String,
    pub(crate) message: String,
}

enum OutboundWriteCommand {
    Packet {
        packet_id: i32,
        packet_name: String,
        payload: Vec<u8>,
    },
    SetCompression(i32),
    EnableEncryption(Vec<u8>),
    Raw(Vec<u8>),
    Close(Shutdown),
}

impl OutboundPacketWriter {
    pub(crate) fn new(mut stream: TcpStream, is_online: Arc<AtomicBool>) -> Self {
        let (command_sender, command_receiver) = mpsc::channel();
        let errors = Arc::new(Mutex::new(VecDeque::new()));
        let writer_errors = errors.clone();
        let writer_is_online = is_online.clone();
        let cancel_pending = Arc::new(AtomicBool::new(false));
        let writer_cancel_pending = cancel_pending.clone();
        let pending_packet_count = Arc::new(AtomicUsize::new(0));
        let writer_pending_packet_count = pending_packet_count.clone();
        std::thread::spawn(move || {
            let mut encoder = PacketEncoder::new();
            while let Ok(command) = command_receiver.recv() {
                match command {
                    OutboundWriteCommand::Packet {
                        packet_id,
                        packet_name,
                        payload,
                    } => {
                        if writer_cancel_pending.load(Ordering::SeqCst) {
                            writer_pending_packet_count.fetch_sub(1, Ordering::SeqCst);
                            continue;
                        }
                        let write_result = encoder
                            .encode_frame(packet_id, &payload)
                            .and_then(|frame| stream.write_all(&frame));
                        if let Err(error) = write_result {
                            if let Ok(mut errors) = writer_errors.lock() {
                                errors.push_back(OutboundWriteError {
                                    packet_id,
                                    packet_name,
                                    message: error.to_string(),
                                });
                            }
                            writer_is_online.store(false, Ordering::SeqCst);
                            writer_pending_packet_count.store(0, Ordering::SeqCst);
                            let _ = stream.shutdown(Shutdown::Both);
                            return;
                        }
                        writer_pending_packet_count.fetch_sub(1, Ordering::SeqCst);
                    }
                    OutboundWriteCommand::SetCompression(threshold) => {
                        encoder.set_compression(threshold);
                    }
                    OutboundWriteCommand::EnableEncryption(key) => {
                        encoder.enable_encryption(&key);
                    }
                    OutboundWriteCommand::Raw(bytes) => {
                        if writer_cancel_pending.load(Ordering::SeqCst) {
                            writer_pending_packet_count.fetch_sub(1, Ordering::SeqCst);
                            continue;
                        }
                        if stream.write_all(&bytes).is_err() {
                            writer_is_online.store(false, Ordering::SeqCst);
                            writer_pending_packet_count.store(0, Ordering::SeqCst);
                            let _ = stream.shutdown(Shutdown::Both);
                            return;
                        }
                        writer_pending_packet_count.fetch_sub(1, Ordering::SeqCst);
                    }
                    OutboundWriteCommand::Close(shutdown) => {
                        let _ = stream.shutdown(shutdown);
                        writer_is_online.store(false, Ordering::SeqCst);
                        return;
                    }
                }
            }
            writer_is_online.store(false, Ordering::SeqCst);
            writer_pending_packet_count.store(0, Ordering::SeqCst);
        });
        Self {
            command_sender,
            errors,
            is_online,
            cancel_pending,
            pending_packet_count,
        }
    }

    pub(crate) fn send_packet(
        &self,
        packet_id: i32,
        packet_name: String,
        payload: Vec<u8>,
    ) -> io::Result<()> {
        if !self.is_online() {
            return Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "connection is closed",
            ));
        }
        self.pending_packet_count.fetch_add(1, Ordering::SeqCst);
        self.command_sender
            .send(OutboundWriteCommand::Packet {
                packet_id,
                packet_name,
                payload,
            })
            .map_err(|_| {
                self.pending_packet_count.fetch_sub(1, Ordering::SeqCst);
                io::Error::new(io::ErrorKind::BrokenPipe, "connection is closed")
            })
    }

    pub(crate) fn set_compression(&self, threshold: i32) {
        let _ = self
            .command_sender
            .send(OutboundWriteCommand::SetCompression(threshold));
    }

    pub(crate) fn enable_encryption(&self, key: &[u8]) {
        let _ = self
            .command_sender
            .send(OutboundWriteCommand::EnableEncryption(key.to_vec()));
    }

    pub(crate) fn send_raw_bytes(&self, bytes: Vec<u8>) -> io::Result<()> {
        if !self.is_online() {
            return Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "connection is closed",
            ));
        }
        self.pending_packet_count.fetch_add(1, Ordering::SeqCst);
        self.command_sender
            .send(OutboundWriteCommand::Raw(bytes))
            .map_err(|_| {
                self.pending_packet_count.fetch_sub(1, Ordering::SeqCst);
                io::Error::new(io::ErrorKind::BrokenPipe, "connection is closed")
            })
    }

    pub(crate) fn close(&self, shutdown: Shutdown, should_cancel_pending: bool) {
        self.cancel_pending
            .store(should_cancel_pending, Ordering::SeqCst);
        self.is_online.store(false, Ordering::SeqCst);
        let _ = self
            .command_sender
            .send(OutboundWriteCommand::Close(shutdown));
    }

    pub(crate) fn is_online(&self) -> bool {
        self.is_online.load(Ordering::SeqCst)
    }

    #[cfg(test)]
    pub(crate) fn pending_packet_count(&self) -> usize {
        self.pending_packet_count.load(Ordering::SeqCst)
    }

    pub(crate) fn take_errors(&self) -> Vec<OutboundWriteError> {
        self.errors
            .lock()
            .map(|mut errors| errors.drain(..).collect())
            .unwrap_or_default()
    }
}
