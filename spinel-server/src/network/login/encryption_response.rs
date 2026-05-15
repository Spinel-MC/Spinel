use crate::entity::player::Player;
use crate::server::MinecraftServer;
use crate::network::client::instance::Client;
use ::rsa::Pkcs1v15Encrypt;
use ::spinel_core::network::clientbound::login::login_success::LoginSuccessPacket;
use ::spinel_core::network::clientbound::login::set_compression::SetCompressionPacket;
use ::spinel_core::network::serverbound::login::encryption_response::EncryptionResponsePacket;
use ::spinel_macros::packet_listener;
use ::spinel_utils::component::Component;
use rsa::RsaPrivateKey;
use uuid::Uuid;

struct VerifiedLoginMetadata {
    private_key: RsaPrivateKey,
    expected_verify_token: Vec<u8>,
    protocol_version: i32,
    username: String,
    uuid: Uuid,
}

struct EncryptionResponseHandler<'a> {
    client: &'a mut Client,
    server: &'a mut MinecraftServer,
}

impl<'a> EncryptionResponseHandler<'a> {
    fn new(client: &'a mut Client, server: &'a mut MinecraftServer) -> Self {
        Self { client, server }
    }

    fn handle(mut self, packet: EncryptionResponsePacket) -> bool {
        let Some(login_metadata) = self.load_login_metadata() else {
            return self.disconnect_invalid_login_sequence(format!(
                "Error: Login metadata not found for {}. Client state: {:?}",
                self.client.addr, self.client.state
            ));
        };

        let Some(shared_secret) = self.decrypt_payload(
            &login_metadata.private_key,
            &packet.keybytes,
            "shared secret",
        ) else {
            return true;
        };
        let Some(verify_token) = self.decrypt_payload(
            &login_metadata.private_key,
            &packet.encrypted_challenge,
            "verify token",
        ) else {
            return true;
        };

        if verify_token != login_metadata.expected_verify_token {
            return self.disconnect_invalid_login_sequence(format!(
                "Client {} failed verification (verify token mismatch).",
                self.client.addr
            ));
        }

        self.complete_login(login_metadata, &shared_secret)
    }

    fn load_login_metadata(&mut self) -> Option<VerifiedLoginMetadata> {
        let login_metadata = self.client.login_metadata.as_ref()?;
        let private_key = login_metadata.private_key.clone()?;
        let expected_verify_token = login_metadata.verify_token.clone()?;
        let username = login_metadata.username.clone()?;
        let uuid = login_metadata.uuid?;

        Some(VerifiedLoginMetadata {
            private_key,
            expected_verify_token,
            protocol_version: login_metadata.protocol_version,
            username,
            uuid,
        })
    }

    fn decrypt_payload(
        &mut self,
        private_key: &RsaPrivateKey,
        encrypted_bytes: &[i8],
        payload_name: &str,
    ) -> Option<Vec<u8>> {
        let encrypted_bytes: Vec<u8> = encrypted_bytes.iter().map(|&byte| byte as u8).collect();
        private_key
            .decrypt(Pkcs1v15Encrypt, &encrypted_bytes)
            .ok()
            .or_else(|| {
                self.disconnect_invalid_login_sequence(format!(
                    "Failed to decrypt {} for {}.",
                    payload_name, self.client.addr
                ));
                None
            })
    }

    fn complete_login(
        &mut self,
        login_metadata: VerifiedLoginMetadata,
        shared_secret: &[u8],
    ) -> bool {
        println!("Client {} successfully verified.", self.client.addr);
        self.client.enable_encryption(shared_secret);

        if SetCompressionPacket::new(-1).dispatch(self.client).is_err() {
            return false;
        }

        self.server.connection_manager.register_player(
            self.client.addr,
            Player::new(
                login_metadata.uuid,
                login_metadata.username.clone(),
                login_metadata.protocol_version,
                self.client.addr,
            ),
        );

        if LoginSuccessPacket::new(login_metadata.uuid, login_metadata.username)
            .dispatch(self.client)
            .is_err()
        {
            return false;
        }

        println!(
            "Login success for client {}. Waiting for Login Acknowledge.",
            self.client.addr
        );
        true
    }

    fn disconnect_invalid_login_sequence(&mut self, log_message: String) -> bool {
        println!("{}", log_message);
        if let Err(error) = self
            .server
            .disconnect(self.client, Component::text("Invalid login sequence."))
        {
            eprintln!(
                "Failed to send disconnect packet to {}: {}",
                self.client.addr, error
            );
        }
        true
    }
}

#[packet_listener(module: "login")]
fn on_encryption_response(
    client: &mut Client,
    packet: EncryptionResponsePacket,
    server: &mut MinecraftServer,
) -> bool {
    EncryptionResponseHandler::new(client, server).handle(packet)
}
