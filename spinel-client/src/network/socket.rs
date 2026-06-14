use crate::events::network::packet_error::{PacketErrorEvent, PacketErrorStage};
use crate::instance::MinecraftClient;
use crate::network::server::instance::Server;
use spinel_network::DataType;
use spinel_network::Recipient;
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

    let stream = TcpStream::connect(&addr_str).await?;
    let addr = stream.peer_addr()?;

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
        let mut reader = read_stream;
        let mut decoder = spinel_network::decoder::PacketDecoder::new();

        loop {
            let packet_bytes = match decoder.read_frame(&mut reader) {
                Ok(b) => b,
                Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                    break;
                }
                Err(e) => {
                    if let Ok(mut server_lock) = server_arc.lock() {
                        if let Some(server) = server_lock.as_mut() {
                            let mut packet_error_event = PacketErrorEvent::new(
                                Recipient::Client,
                                PacketErrorStage::FrameRead,
                                server.state,
                                None,
                                None,
                                e.to_string(),
                            );
                            let mut client_handle = client.clone();
                            packet_error_event.dispatch(&mut client_handle, server);
                        }
                    }
                    break;
                }
            };

            let mut cursor = std::io::Cursor::new(&packet_bytes);
            let packet_id = match VarIntWrapper::decode(&mut cursor) {
                Ok(id) => id.0,
                Err(e) => {
                    if let Ok(mut server_lock) = server_arc.lock() {
                        if let Some(server) = server_lock.as_mut() {
                            let mut packet_error_event = PacketErrorEvent::new(
                                Recipient::Client,
                                PacketErrorStage::PacketIdDecode,
                                server.state,
                                None,
                                None,
                                e.to_string(),
                            );
                            let mut client_handle = client.clone();
                            packet_error_event.dispatch(&mut client_handle, server);
                        }
                    }
                    break;
                }
            };

            let payload_pos = cursor.position() as usize;
            let payload = packet_bytes[payload_pos..].to_vec();

            {
                let mut lock = server_arc.lock().unwrap();
                if let Some(s) = lock.as_mut() {
                    client.dispatch_packet(packet_id, s, payload);

                    if !s.is_connected() {
                        break;
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

        if let Ok(mut lock) = server_arc.lock() {
            *lock = None;
        }

        client.on_disconnect();
    });

    Ok(())
}
