use crate::events::intention::IntentionEvent;
use crate::events::login::PreLoginEvent;
use crate::instance::MinecraftServer;
use crate::network::client::instance::Client;
use crate::network::client::metadata::LoginMetadata;

use rsa::pkcs8::EncodePublicKey;
use rsa::{
    RsaPrivateKey,
    rand_core::{CryptoRng, RngCore},
};
use spinel_core::network::clientbound::login::encryption_request::EncryptionRequestPacket;
use spinel_core::network::serverbound::login::login_start::LoginStartPacket;
use spinel_macros::{event_listener, packet_listener};
use spinel_utils::component::Component;

use rand::TryRngCore;
use rand::rngs::OsRng;

struct CompatRng(OsRng);

impl CryptoRng for CompatRng {}

impl RngCore for CompatRng {
    fn next_u32(&mut self) -> u32 {
        self.0.try_next_u32().expect("OsRng failed")
    }

    fn next_u64(&mut self) -> u64 {
        self.0.try_next_u64().expect("OsRng failed")
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.try_fill_bytes(dest).expect("OsRng failed")
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rsa::rand_core::Error> {
        self.0
            .try_fill_bytes(dest)
            .map_err(|e| rsa::rand_core::Error::new(e))
    }
}

#[event_listener(module: "login")]
fn on_intention(event: &mut IntentionEvent, _server: &mut MinecraftServer) {
    event.client().login_metadata = Some(LoginMetadata::new(event.protocol_version));
}

#[packet_listener(module: "login")]
fn on_login_start(
    client: &mut Client,
    packet: LoginStartPacket,
    server: &mut MinecraftServer,
) -> bool {
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

    let mut verify_token = [0u8; 16];
    rng.fill_bytes(&mut verify_token);

    if let Some(metadata) = &mut client.login_metadata {
        metadata.username = Some(packet.name.clone());
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

    if pre_login_event.should_authenticate {
        let encryption_packet = EncryptionRequestPacket::new(
            "".to_string(),
            public_key_der.as_ref().to_vec(),
            verify_token.to_vec(),
            true,
        );
        encryption_packet.dispatch(client);
        println!("Sent encryption request to {}.", client.addr);
    } else {
        use spinel_core::network::clientbound::login::login_success::LoginSuccessPacket;
        let success = LoginSuccessPacket::new(packet.uuid, packet.name);
        success.dispatch(client);
        println!("Sent Login Success to {} (offline mode).", client.addr);
    }
    true
}
