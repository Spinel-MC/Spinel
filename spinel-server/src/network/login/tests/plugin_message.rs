use crate::network::client::instance::Client;
use spinel_network::ConnectionState;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};

#[test]
fn login_plugin_requests_correlate_response_id_channel_and_payload() {
    let mut client = connected_client();
    client.state = ConnectionState::Login;
    let mut response_future = client
        .send_login_plugin_request("velocity:player_info", Some(vec![1, 2, 3]))
        .unwrap();
    let message_ids = client.pending_login_plugin_message_ids();

    assert_eq!(message_ids.len(), 1);
    assert!(client.has_pending_login_plugin_requests());
    client
        .complete_login_plugin_response(message_ids[0], Some(vec![4, 5, 6]))
        .unwrap();
    let response = response_future.try_recv().unwrap();

    assert_eq!(response.channel(), "velocity:player_info");
    assert_eq!(response.data(), Some([4, 5, 6].as_slice()));
    assert!(!client.has_pending_login_plugin_requests());
}

#[test]
fn login_plugin_processor_rejects_unexpected_response_ids() {
    let mut client = connected_client();

    assert!(
        client
            .complete_login_plugin_response(42, Some(Vec::new()))
            .is_err()
    );
}

fn connected_client() -> Client {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
    let server_address = listener.local_addr().unwrap();
    let client_stream = std::net::TcpStream::connect(server_address).unwrap();
    let (_server_stream, client_address) = listener.accept().unwrap();
    Client::new(
        client_stream,
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), client_address.port()),
    )
}
