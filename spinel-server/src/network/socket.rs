use crate::events::network::inbound_packet_error::{
    InboundPacketErrorEvent, InboundPacketErrorStage,
};
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::keep_alive::KeepAlivePacket;
use spinel_network::ConnectionState;
use spinel_network::DataType;
use spinel_network::VarIntWrapper;
use spinel_network::packet_names::PacketNameRegistry;
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::task;

pub async fn start_tcp_listener(
    server_arc: Arc<Mutex<MinecraftServer>>,
    address: &str,
    port: u16,
) -> Result<(), Error> {
    ServerSocketRuntime::start_listener(server_arc, address, port).await
}

struct ServerSocketRuntime;

impl ServerSocketRuntime {
    async fn start_listener(
        server_arc: Arc<Mutex<MinecraftServer>>,
        address: &str,
        port: u16,
    ) -> Result<(), Error> {
        let listener = TcpListener::bind(format!("{}:{}", address, port)).await?;
        println!("Listening on {}:{}", address, port);

        loop {
            let (stream, addr) = listener.accept().await?;
            println!("New connection from: {}", addr);
            tokio::spawn(Self::handle_client_connection(
                server_arc.clone(),
                stream,
                addr,
            ));
        }
    }

    async fn handle_client_connection(
        server_arc: Arc<Mutex<MinecraftServer>>,
        stream: TcpStream,
        addr: SocketAddr,
    ) {
        let Some((client, read_stream)) = Self::initialize_client(stream, addr) else {
            return;
        };

        if Self::connection_cancelled(&server_arc, client.clone(), addr) {
            return;
        }

        task::spawn_blocking(move || Self::run_client_loop(server_arc, client, read_stream, addr));
    }

    fn connection_cancelled(
        server_arc: &Arc<Mutex<MinecraftServer>>,
        client: Arc<Mutex<Client>>,
        addr: SocketAddr,
    ) -> bool {
        let Ok(mut server) = server_arc.lock() else {
            eprintln!("Failed to lock server during connection setup for {}", addr);
            return true;
        };
        if server.on_connection(client) {
            println!("Connection cancelled for {}", addr);
            return true;
        }

        false
    }

    fn dispatch_inbound_packet_error_event(
        server: &mut MinecraftServer,
        client: &mut Client,
        stage: InboundPacketErrorStage,
        packet_id: Option<i32>,
        message: String,
    ) {
        let packet_name =
            packet_id.map(|id| PacketNameRegistry::get_serverbound_packet_name(client.state, id));
        let mut inbound_packet_error_event =
            InboundPacketErrorEvent::new(stage, client.state, packet_id, packet_name, message);
        inbound_packet_error_event.dispatch(server, client);
    }

    fn initialize_client(
        stream: TcpStream,
        addr: SocketAddr,
    ) -> Option<(Arc<Mutex<Client>>, std::net::TcpStream)> {
        let _ = stream.set_nodelay(true);
        let std_stream = Self::into_blocking_std_stream(stream, addr)?;
        let read_stream = std_stream.try_clone().ok()?;
        let client = Arc::new(Mutex::new(Client::new(std_stream, addr)));
        Some((client, read_stream))
    }

    fn into_blocking_std_stream(
        stream: TcpStream,
        addr: SocketAddr,
    ) -> Option<std::net::TcpStream> {
        let std_stream = stream.into_std().ok()?;
        if let Err(error) = std_stream.set_nonblocking(false) {
            eprintln!("Failed to set stream to blocking for {}: {}", addr, error);
            return None;
        }

        Some(std_stream)
    }

    fn read_packet_id(
        client: &Arc<Mutex<Client>>,
        server_arc: &Arc<Mutex<MinecraftServer>>,
        packet_bytes: &[u8],
    ) -> Option<(i32, usize)> {
        let mut cursor = std::io::Cursor::new(packet_bytes);
        match VarIntWrapper::decode(&mut cursor) {
            Ok(packet_id) => Some((packet_id.0, cursor.position() as usize)),
            Err(error) => {
                let Ok(mut server) = server_arc.lock() else {
                    eprintln!("Failed to lock server while decoding packet id");
                    return None;
                };
                let Ok(mut client) = client.lock() else {
                    eprintln!("Failed to lock client while decoding packet id");
                    return None;
                };
                Self::dispatch_inbound_packet_error_event(
                    &mut server,
                    &mut client,
                    InboundPacketErrorStage::PacketIdDecode,
                    None,
                    error.to_string(),
                );
                None
            }
        }
    }

    fn run_client_loop(
        server_arc: Arc<Mutex<MinecraftServer>>,
        client: Arc<Mutex<Client>>,
        mut read_stream: std::net::TcpStream,
        addr: SocketAddr,
    ) {
        println!("Client loop started for: {}", addr);
        let mut decoder = spinel_network::decoder::PacketDecoder::new();

        loop {
            if Self::client_is_offline(&client) {
                break;
            }
            let Ok(packet_bytes) =
                Self::read_frame(&server_arc, &client, &mut decoder, &mut read_stream)
            else {
                break;
            };
            let Some((packet_id, payload_pos)) =
                Self::read_packet_id(&client, &server_arc, &packet_bytes)
            else {
                break;
            };
            let payload = packet_bytes[payload_pos..].to_vec();
            Self::dispatch_packet(&server_arc, &client, &mut decoder, packet_id, payload);
        }

        if let Ok(mut server) = server_arc.lock() {
            server.on_disconnect(addr);
        }
    }

    fn read_frame(
        server_arc: &Arc<Mutex<MinecraftServer>>,
        client: &Arc<Mutex<Client>>,
        decoder: &mut spinel_network::decoder::PacketDecoder,
        read_stream: &mut std::net::TcpStream,
    ) -> Result<Vec<u8>, ()> {
        match decoder.read_frame(read_stream) {
            Ok(packet_bytes) => Ok(packet_bytes),
            Err(error) if error.kind() == ErrorKind::UnexpectedEof => Err(()),
            Err(error) => {
                let Ok(mut server) = server_arc.lock() else {
                    eprintln!("Failed to lock server while reading frame");
                    return Err(());
                };
                let Ok(mut client) = client.lock() else {
                    eprintln!("Failed to lock client while reading frame");
                    return Err(());
                };
                if !client.is_online() {
                    return Err(());
                }
                Self::dispatch_inbound_packet_error_event(
                    &mut server,
                    &mut client,
                    InboundPacketErrorStage::FrameRead,
                    None,
                    error.to_string(),
                );
                Err(())
            }
        }
    }

    fn dispatch_packet(
        server_arc: &Arc<Mutex<MinecraftServer>>,
        client: &Arc<Mutex<Client>>,
        decoder: &mut spinel_network::decoder::PacketDecoder,
        packet_id: i32,
        payload: Vec<u8>,
    ) {
        let Ok(mut server) = server_arc.lock() else {
            eprintln!("Failed to lock server while dispatching packet");
            return;
        };
        let Ok(mut client) = client.lock() else {
            eprintln!("Failed to lock client while dispatching packet");
            return;
        };

        let packet_has_listener = server.has_listener_for(packet_id, &client.state);
        if packet_has_listener && Self::packet_should_be_queued(&client, packet_id) {
            server.queue_player_packet(packet_id, &mut client, payload);
        } else if packet_has_listener {
            server.dispatch_packet(packet_id, &mut client, payload);
        }

        Self::sync_decoder_state(decoder, &mut client);
    }

    fn packet_should_be_queued(client: &Client, packet_id: i32) -> bool {
        client.state == ConnectionState::Play && packet_id != KeepAlivePacket::get_id()
    }

    fn sync_decoder_state(
        decoder: &mut spinel_network::decoder::PacketDecoder,
        client: &mut Client,
    ) {
        if let Some(key) = client.pending_encryption.take() {
            decoder.enable_encryption(&key);
        }

        if let Some(threshold) = client.pending_compression.take() {
            decoder.set_compression(threshold);
        }
    }

    fn client_is_offline(client: &Arc<Mutex<Client>>) -> bool {
        client
            .lock()
            .map(|client| !client.is_online())
            .unwrap_or(true)
    }
}
