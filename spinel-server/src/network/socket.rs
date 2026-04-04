use crate::instance::MinecraftServer;
use crate::network::client::instance::Client;
use spinel_network::DataType;
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};
use tokio::task;

// Import the wrapper for manual decoding
use spinel_network::VarIntWrapper;

pub async fn start_tcp_listener(
    server_arc: Arc<Mutex<MinecraftServer>>,
    address: &str,
    port: u16,
) -> Result<(), Error> {
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

async fn handle_client_connection(
    server_arc: Arc<Mutex<MinecraftServer>>,
    stream: TcpStream,
    addr: SocketAddr,
) {
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

    let read_stream = match std_stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to clone stream for {}: {}", addr, e);
            return;
        }
    };

    let client = Arc::new(Mutex::new(Client::new(std_stream, addr)));

    {
        // Still using tokio lock here as it's an async context, but we don't hold it long
        let mut server = server_arc.lock().unwrap();
        // WARNING: on_connection currently uses blocking_lock() internally.
        // We should really fix that too, but let's see if this part works first.
        if server.on_connection(client.clone()) {
            println!("Connection cancelled for {}", addr);
            return;
        }
    }

    task::spawn_blocking(move || {
        println!("Client loop started for: {}", addr);
        let mut reader = read_stream;
        let mut decoder = spinel_network::decoder::PacketDecoder::new();

        loop {
            // BLOCKING READ (No lock held)
            let packet_bytes = match decoder.read_frame(&mut reader) {
                Ok(b) => b,
                Err(e) if e.kind() == ErrorKind::UnexpectedEof => {
                    println!("Client {} disconnected.", addr);
                    break;
                }
                Err(e) => {
                    eprintln!("Error reading packet from {}: {}", addr, e);
                    break;
                }
            };

            let mut cursor = std::io::Cursor::new(&packet_bytes);
            let packet_id = match VarIntWrapper::decode(&mut cursor) {
                Ok(id) => id.0,
                Err(e) => {
                    eprintln!("Error decoding packet ID from {}: {}", addr, e);
                    break;
                }
            };

            let payload_pos = cursor.position() as usize;
            let payload = packet_bytes[payload_pos..].to_vec();

            {
                // Still using std lock here as we moved the decoder out of the locked part
                let mut server = server_arc.lock().unwrap();
                let mut c = client.lock().unwrap();

                if server.has_listener_for(packet_id, &c.state) {
                    server.dispatch_packet(packet_id, &mut c, payload);
                }

                // Sync pending state
                if let Some(key) = c.pending_encryption.take() {
                    decoder.enable_encryption(&key);
                }
                if let Some(threshold) = c.pending_compression.take() {
                    decoder.set_compression(threshold);
                }
            }
        }

        let mut server = server_arc.lock().unwrap();
        server.on_disconnect(addr);
    });
}
