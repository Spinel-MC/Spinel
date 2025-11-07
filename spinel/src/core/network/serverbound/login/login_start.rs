impl RngCore for CompatRng {
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }
    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.try_fill_bytes(dest).unwrap()
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rsa::rand_core::Error> {
        Ok(self.0.try_fill_bytes(dest).unwrap())
    }
}

impl CryptoRng for CompatRng {}
struct CompatRng(OsRng);

#[event_listener(event: "intention", module: "login")]
fn on_intention(event: &mut IntentionEvent, _server: &mut MinecraftServer) {
    event.client().login_metadata = Some(LoginMetadata::new(event.protocol_version));
}

use crate::core::{
    events::{intention::IntentionEvent, login::PreLoginEvent},
    network::clientbound::login::encryption_request::EncryptionRequestPacket,
    server::MinecraftServer,
};
use rsa::pkcs8::EncodePublicKey;
use rsa::{
    RsaPrivateKey,
    rand_core::{CryptoRng, OsRng, RngCore},
};
use spinel_macros::{event_listener, packet_listener};
use spinel_network::{Client, ConnectionState, client::metadata::LoginMetadata};
use spinel_utils::component::Component;

use crate as spinel;

#[packet_listener(
    id: 0x00,
    state: ConnectionState::Login,
    fields: (name: String, uuid: UUID),
    module: "login"
)]
fn on_login_start(client: &mut Client, packet: Packet, server: &mut MinecraftServer) -> bool {
    println!(
        "Login sequence started for user '{}' ({})",
        packet.name, packet.uuid
    );

    let mut pre_login_event = PreLoginEvent::new(packet.name.clone(), packet.uuid, false);

    pre_login_event.dispatch(server, client);

    if pre_login_event.cancelled {
        return true;
    }

    let mut rng = CompatRng(OsRng);
    let private_key = RsaPrivateKey::new(&mut rng, 1024).expect("Failed to generate a private key");
    let public_key_der = private_key.to_public_key().to_public_key_der().unwrap();

    let verify_token: [u8; 16] = rand::random();

    if let Some(metadata) = &mut client.login_metadata {
        metadata.username = Some(packet.name);
        metadata.uuid = Some(packet.uuid);
        metadata.private_key = Some(private_key);
        metadata.verify_token = Some(verify_token.to_vec());
    } else {
        println!(
            "Error: Client {} sent Login Start Packet without prior Intention Packet.",
            client.addr
        );
        server.disconnect(client, Component::text("Invalid login sequence."));
        return true;
    }

    let encryption_packet = EncryptionRequestPacket::new(
        "".to_string(),
        public_key_der.as_ref().to_vec(),
        verify_token.to_vec(),
        pre_login_event.should_authenticate,
    );

    encryption_packet.dispatch(client);

    println!("Sent encryption request to {}.", client.addr);
    true
}
