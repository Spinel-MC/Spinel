use crate::instance::MinecraftClient;
use crate::network::server::instance::Server;
use spinel_network::DataType;
use std::io::{Error, ErrorKind};
use tokio::net::TcpStream;
use tokio::task;

use spinel_network::VarIntWrapper;

pub async fn connect_to_server(
    client: MinecraftClient,
    address: &str,
    port: u16,
) -> Result<(), Error> {
    let addr_str = format!("{}:{}", address, port);
    println!("Connecting to {}...", addr_str);

    let stream = TcpStream::connect(&addr_str).await?;
    let addr = stream.peer_addr()?;
    println!("Connected to {}", addr);

    stream.set_nodelay(true)?;

    let std_stream = stream.into_std()?;
    let read_stream = std_stream.try_clone()?;
    std_stream.set_nonblocking(false)?;
    read_stream.set_nonblocking(false)?;

    let server_instance = Server::new(std_stream, addr);

    {
        let mut server_lock = client.server.0.lock().unwrap();
        *server_lock = Some(server_instance);
    }

    let server_arc = client.server.0.clone();

    task::spawn_blocking(move || {
        println!("Network loop started for server connection: {}", addr);
        let mut reader = read_stream;
        let mut decoder = spinel_network::decoder::PacketDecoder::new();

        loop {
            let packet_bytes = match decoder.read_frame(&mut reader) {
                Ok(b) => b,
                Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                    println!("Disconnected from server.");
                    break;
                }
                Err(e) => {
                    eprintln!("Error reading packet: {}", e);
                    break;
                }
            };

            let mut cursor = std::io::Cursor::new(&packet_bytes);
            let packet_id = match VarIntWrapper::decode(&mut cursor) {
                Ok(id) => id.0,
                Err(e) => {
                    eprintln!("Error decoding packet ID: {}", e);
                    break;
                }
            };

            let payload_pos = cursor.position() as usize;
            let payload = packet_bytes[payload_pos..].to_vec();

            {
                let mut lock = server_arc.lock().unwrap();
                if let Some(s) = lock.as_mut() {
                    if client.has_listener_for(packet_id, &s.state) {
                        client.dispatch_packet(packet_id, s, payload);
                    }

                    if let Some(key) = s.pending_encryption.take() {
                        decoder.enable_encryption(&key);
                    }
                    if let Some(threshold) = s.pending_compression.take() {
                        decoder.set_compression(threshold);
                    }
                }
            }
        }

        client.on_disconnect();
    });

    Ok(())
}
