use std::{
    io::{Error, ErrorKind},
    net::SocketAddr,
    sync::Arc,
};

use tokio::{
    net::{TcpListener, TcpStream as TokioTcpStream},
    sync::Mutex,
    task,
};

use crate::ServerContext;
use crate::client::instance::Client;

pub async fn start_tcp_listener<S>(
    server_arc: Arc<Mutex<S>>,
    address: &str,
    port: u16,
) -> Result<(), Error>
where
    S: ServerContext + Send + Sync + 'static,
{
    let listener = TcpListener::bind(format!("{}:{}", address, port)).await?;

    println!("Listening on {}:{}", address, port);

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                println!("New connection from: {}", addr);
                tokio::spawn(handle_client_connection(server_arc.clone(), stream, addr));
            }
            Err(e) => {
                eprintln!("Error accepting connection: {:?}", e);
                return Err(e);
            }
        }
    }
}

async fn handle_client_connection<S>(
    server_arc: Arc<Mutex<S>>,
    stream: TokioTcpStream,
    addr: SocketAddr,
) where
    S: ServerContext,
{
    stream.set_nodelay(true).unwrap_or_else(|_| ());
    let std_stream = match stream.into_std() {
        Ok(s) => {
            if let Err(e) = s.set_nonblocking(false) {
                eprintln!("Failed to set stream to blocking for {}: {}", addr, e);
                return;
            }
            s
        }
        Err(e) => {
            eprintln!(
                "Failed to convert tokio stream to std stream for {}: {}",
                addr, e
            );
            return;
        }
    };

    let stream_for_manager = match std_stream.try_clone() {
        Ok(cloned_stream) => cloned_stream,
        Err(e) => {
            eprintln!("Failed to clone stream for {}: {}", addr, e);
            return;
        }
    };

    {
        let mut server_guard = server_arc.lock().await;
        server_guard
            .connection_manager_mut()
            .register_connection(addr, stream_for_manager);
    }

    task::spawn_blocking(move || {
        println!("Client connection task started for: {}", addr);
        let mut client = Client::new(std_stream, addr);
        let mut server_guard = server_arc.blocking_lock();

        if server_guard.on_connection(&mut client) {
            println!("Client connection: {} was cancelled.", addr);
            client.disconnect();
            return;
        }
        drop(server_guard);

        let client_addr = client.addr;

        let mut first_byte = [0u8; 1];
        match client.stream.peek(&mut first_byte) {
            Ok(_) => {
                if first_byte[0] == 0xFE {
                    if client.read_exact(&mut first_byte).is_ok() {
                        let packet_id = 0xFE;
                        let mut server_guard = server_arc.blocking_lock();
                        server_guard.dispatch_packet(packet_id, &mut client, vec![]);
                    }
                } else {
                    loop {
                        let packet_id;
                        let packet_length;

                        match client.read_varint() {
                            Ok(len) => packet_length = len,
                            Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                                println!("Client {} disconnected.", client.addr);
                                break;
                            }
                            Err(e) => {
                                eprintln!(
                                    "Error reading packet length from {}: {}",
                                    client.addr, e
                                );
                                break;
                            }
                        }

                        match client.read_varint() {
                            Ok(id) => packet_id = id,
                            Err(e) => {
                                eprintln!("Error reading packet ID from {}: {}", client.addr, e);
                                break;
                            }
                        }

                        let packet_id_size = {
                            let mut buffer = crate::encoder::NetworkBuffer::new();
                            buffer.write_varint(packet_id);
                            buffer.into_buffer().len()
                        };

                        let payload_size = (packet_length as usize) - packet_id_size;
                        let mut payload_buffer = vec![0; payload_size];
                        if let Err(e) = client.read_exact(&mut payload_buffer) {
                            eprintln!(
                                "Error reading payload for packet from {}: {}",
                                client.addr, e
                            );
                            break;
                        }

                        let mut server_guard = server_arc.blocking_lock();

                        if server_guard.has_listener_for(packet_id, &client.state) {
                            let initial_client_state = client.state;
                            server_guard.dispatch_packet(packet_id, &mut client, payload_buffer);

                            println!(
                                "[{:?}, ID: {:#04X}] Packet handled!",
                                initial_client_state, packet_id
                            );
                        } else {
                            println!(
                                "[{:?}, ID: {:#04X}] Unhandled packet (no listener).",
                                client.state, packet_id
                            );
                        }
                    }
                }
            }
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                println!("Client {} disconnected before sending data.", client.addr);
            }
            Err(e) => {
                eprintln!("Error peeking first byte from {}: {}", client.addr, e);
            }
        }

        let mut server_guard = server_arc.blocking_lock();
        server_guard.on_disconnect(client_addr);
    });
}
