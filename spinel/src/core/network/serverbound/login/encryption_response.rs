use rsa::Pkcs1v15Encrypt;
use spinel_macros::packet_listener;
use spinel_network::{Client, ConnectionState, Player};
use spinel_utils::component::Component;

use crate::core::network::clientbound::login::login_success::LoginSuccessPacket;
use crate::core::network::clientbound::login::set_compression::SetCompressionPacket;

use crate::core::server::MinecraftServer;
use crate as spinel;

fn disconnect_with_error(server: &mut MinecraftServer, client: &mut Client, log_message: &str) -> bool {
    println!("{}", log_message);
    server.disconnect(client, Component::text("Invalid login sequence."));
    true
}

#[packet_listener(
    id: 0x01, 
    state: ConnectionState::Login, 
    fields: (shared_secret: ByteArray, verify_token: ByteArray,),
    module: "login"
)]
fn on_encryption_response(client: &mut Client, packet: Packet, server: &mut MinecraftServer) -> bool {

    let owned_data = {
        let metadata = match client.login_metadata.as_ref() {
            Some(m) => m,
            None => {
                let log_msg = format!("Error: Login metadata not found for {}. Client state: {:?}", client.addr, client.state);
                return disconnect_with_error(server, client, &log_msg);
            }
        };

        let private_key = match metadata.private_key.clone() {
            Some(pk) => pk,
            None => return disconnect_with_error(server, client, &format!("Error: Private key missing in metadata for {}.", client.addr)),
        };

        let expected_verify_token = match metadata.verify_token.clone() {
            Some(vt) => vt,
            None => return disconnect_with_error(server, client, &format!("Error: Verify token missing in metadata for {}.", client.addr)),
        };

        let username = match metadata.username.clone() {
            Some(u) => u,
            None => return disconnect_with_error(server, client, &format!("Error: Username missing in metadata for {}.", client.addr)),
        };

        let uuid = match metadata.uuid {
            Some(u) => u,
            None => return disconnect_with_error(server, client, &format!("Error: UUID missing in metadata for {}.", client.addr)),
        };

        (private_key, expected_verify_token, metadata.protocol_version, username, uuid)
    };
    
    let (private_key, expected_verify_token, protocol_version, username, uuid) = owned_data;

    let decrypted_shared_secret = match private_key.decrypt(Pkcs1v15Encrypt, &packet.shared_secret) {
        Ok(s) => s,
        Err(e) => return disconnect_with_error(server, client, &format!("Failed to decrypt shared secret for {}: {}", client.addr, e)),
    };
    
    let decrypted_verify_token = match private_key.decrypt(Pkcs1v15Encrypt, &packet.verify_token) {
        Ok(t) => t,
        Err(e) => return disconnect_with_error(server, client, &format!("Failed to decrypt verify token for {}: {}", client.addr, e)),
    };

    if decrypted_verify_token != expected_verify_token {
        return disconnect_with_error(server, client, &format!("Client {} failed verification (verify token mismatch).", client.addr));
    }

    println!("Client {} successfully verified.", client.addr);
    
    client.enable_encryption(&decrypted_shared_secret);
    
    let set_compression_packet = SetCompressionPacket::new(-1); 
    set_compression_packet.dispatch(client);
    
    server.connection_manager.register_player(client.addr, Player::new(uuid, username.clone(), protocol_version, client.addr));
    
    let login_success_packet = LoginSuccessPacket::new(uuid, username);
    login_success_packet.dispatch(client);

    println!("Login success for client {}. Waiting for Login Acknowledge.", client.addr);
    
    true
}