use crate::network::client::instance::Client;
use spinel_core::network::clientbound::play::keep_alive::KeepAlivePacket;
use spinel_core::network::serverbound::play::keep_alive::KeepAlivePacket as KeepAliveResponsePacket;
use spinel_network::{DataType, VarIntWrapper};
use std::io::{Cursor, Read};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::time::Duration;

#[test]
fn valid_keep_alive_payload_is_acknowledged_without_server_access() {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0)).unwrap();
    let address = listener.local_addr().unwrap();
    let stream = TcpStream::connect(address).unwrap();
    let (mut peer_stream, _) = listener.accept().unwrap();
    peer_stream
        .set_read_timeout(Some(Duration::from_millis(100)))
        .unwrap();
    let mut client = Client::new(stream, address);
    client.enter_play();

    assert!(client.tick());

    let keep_alive_id = read_keep_alive_id(&mut peer_stream);
    let mut payload = Vec::new();
    KeepAliveResponsePacket { id: keep_alive_id }
        .encode(&mut payload)
        .unwrap();

    assert!(client.handle_keep_alive_payload(&payload).unwrap());
    assert!(client.tick());
}

fn read_keep_alive_id(stream: &mut TcpStream) -> i64 {
    let frame_length = VarIntWrapper::decode(stream).unwrap().0 as usize;
    let mut frame = vec![0; frame_length];
    stream.read_exact(&mut frame).unwrap();
    let mut frame = Cursor::new(frame);

    assert_eq!(
        VarIntWrapper::decode(&mut frame).unwrap().0,
        KeepAlivePacket::get_id()
    );
    KeepAlivePacket::decode(&mut frame).unwrap().id
}
