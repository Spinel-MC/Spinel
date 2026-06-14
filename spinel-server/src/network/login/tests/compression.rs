use crate::network::client::instance::Client;
use spinel_core::network::clientbound::login::set_compression::SetCompressionPacket;
use spinel_network::{DataType, PacketDecoder, VarIntWrapper};
use std::io::Cursor;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};

#[test]
fn login_compression_starts_after_set_compression_packet() {
    let (mut client, mut peer_stream) = connected_client();
    client.state = spinel_network::ConnectionState::Login;
    client.enable_outbound_packet_queue();

    client.start_compression(256).unwrap();
    client
        .send_packet_immediately(1, &[7; 512], "compressed_test_packet".to_string())
        .unwrap();

    let mut decoder = PacketDecoder::new();
    let compression_packet_frame = decoder.read_frame(&mut peer_stream).unwrap();
    let mut compression_packet_cursor = Cursor::new(compression_packet_frame);

    assert_eq!(
        VarIntWrapper::decode(&mut compression_packet_cursor)
            .unwrap()
            .0,
        SetCompressionPacket::get_id()
    );
    assert_eq!(
        VarIntWrapper::decode(&mut compression_packet_cursor)
            .unwrap()
            .0,
        256
    );

    decoder.set_compression(256);
    let compressed_packet_frame = decoder.read_frame(&mut peer_stream).unwrap();
    let mut compressed_packet_cursor = Cursor::new(compressed_packet_frame);

    assert_eq!(
        VarIntWrapper::decode(&mut compressed_packet_cursor)
            .unwrap()
            .0,
        1
    );
    assert_eq!(compressed_packet_cursor.into_inner()[1..], [7; 512]);
}

fn connected_client() -> (Client, TcpStream) {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
    let server_address = listener.local_addr().unwrap();
    let client_stream = TcpStream::connect(server_address).unwrap();
    let (peer_stream, client_address) = listener.accept().unwrap();
    let client = Client::new(
        client_stream,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), client_address.port()),
    );
    (client, peer_stream)
}
