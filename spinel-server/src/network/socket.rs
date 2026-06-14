use crate::events::network::packet_error::{PacketErrorEvent, PacketErrorStage};
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::serverbound::play::keep_alive::KeepAlivePacket;
use spinel_network::DataType;
use spinel_network::VarIntWrapper;
use spinel_network::packet_names::PacketNameRegistry;
use spinel_network::{ConnectionState, Recipient};
use std::io::{Error, ErrorKind};
use std::net::{Shutdown, SocketAddr};
use std::sync::atomic::{AtomicBool, Ordering};
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

        let Some(connection_status) = client.lock().ok().map(|client| client.connection_status())
        else {
            return;
        };
        task::spawn_blocking(move || {
            Self::run_client_loop(server_arc, client, read_stream, connection_status, addr)
        });
    }

    fn connection_cancelled(
        server_arc: &Arc<Mutex<MinecraftServer>>,
        client: Arc<Mutex<Client>>,
        addr: SocketAddr,
    ) -> bool {
        let Ok(mut server) = server_arc.lock() else {
            return true;
        };
        if server.on_connection(client) {
            println!("Connection cancelled for {}", addr);
            return true;
        }

        false
    }

    fn dispatch_packet_error_event(
        server: &mut MinecraftServer,
        client: &mut Client,
        stage: PacketErrorStage,
        packet_id: Option<i32>,
        message: String,
    ) {
        let packet_name =
            packet_id.map(|id| PacketNameRegistry::get_serverbound_packet_name(client.state, id));
        let mut packet_error_event = PacketErrorEvent::new(
            Recipient::Server,
            stage,
            client.state,
            packet_id,
            packet_name,
            message,
        );
        packet_error_event.dispatch(server, client);
    }

    fn initialize_client(
        stream: TcpStream,
        addr: SocketAddr,
    ) -> Option<(Arc<Mutex<Client>>, std::net::TcpStream)> {
        let _ = stream.set_nodelay(true);
        let std_stream = Self::into_blocking_std_stream(stream)?;
        let read_stream = std_stream.try_clone().ok()?;
        let client = Arc::new(Mutex::new(Client::new(std_stream, addr)));
        Some((client, read_stream))
    }

    fn into_blocking_std_stream(stream: TcpStream) -> Option<std::net::TcpStream> {
        let std_stream = stream.into_std().ok()?;
        if std_stream.set_nonblocking(false).is_err() {
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
                    return None;
                };
                let Ok(mut client) = client.lock() else {
                    return None;
                };
                Self::dispatch_packet_error_event(
                    &mut server,
                    &mut client,
                    PacketErrorStage::PacketIdDecode,
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
        connection_status: Arc<AtomicBool>,
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

        connection_status.store(false, Ordering::SeqCst);
        let _ = read_stream.shutdown(Shutdown::Both);
        if let Ok(mut server) = server_arc.lock() {
            server.handle_connection_closed(addr);
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
                    return Err(());
                };
                let Ok(mut client) = client.lock() else {
                    return Err(());
                };
                if !client.is_online() {
                    return Err(());
                }
                Self::dispatch_packet_error_event(
                    &mut server,
                    &mut client,
                    PacketErrorStage::FrameRead,
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
        if Self::handle_keep_alive_without_server(client, packet_id, &payload) {
            return;
        }
        let Ok(mut server) = server_arc.lock() else {
            return;
        };
        let Ok(mut client) = client.lock() else {
            return;
        };

        let packet_has_codec = server.has_codec_for(packet_id, client.state);
        if packet_has_codec && Self::packet_should_be_queued(&client, packet_id) {
            server.queue_player_packet(packet_id, &mut client, payload);
        } else {
            server.dispatch_packet(packet_id, &mut client, payload);
        }

        Self::sync_decoder_state(decoder, &mut client);
    }

    fn handle_keep_alive_without_server(
        client: &Arc<Mutex<Client>>,
        packet_id: i32,
        payload: &[u8],
    ) -> bool {
        let Ok(mut client) = client.lock() else {
            return false;
        };
        if client.state != ConnectionState::Play || packet_id != KeepAlivePacket::get_id() {
            return false;
        }
        client.handle_keep_alive_payload(payload).unwrap_or(false)
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
