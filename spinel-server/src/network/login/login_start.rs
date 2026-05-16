use crate::events::intention::IntentionEvent;
use crate::events::login::PreLoginEvent;
use crate::network::client::instance::Client;
use crate::network::client::metadata::LoginMetadata;
use crate::server::MinecraftServer;

use rsa::RsaPrivateKey;
use rsa::pkcs8::EncodePublicKey;
use rsa::rand_core::{OsRng, RngCore};
use spinel_core::network::clientbound::login::encryption_request::EncryptionRequestPacket;
use spinel_core::network::serverbound::login::login_start::LoginStartPacket;
use spinel_macros::{event_listener, packet_listener};
use spinel_utils::component::Component;

struct LoginAuthenticationArtifacts {
    private_key: RsaPrivateKey,
    public_key_der: Vec<u8>,
    verify_token: Vec<u8>,
}

struct LoginStartHandler<'a> {
    client: &'a mut Client,
    server: &'a mut MinecraftServer,
}

impl<'a> LoginStartHandler<'a> {
    fn new(client: &'a mut Client, server: &'a mut MinecraftServer) -> Self {
        Self { client, server }
    }

    fn handle(mut self, packet: LoginStartPacket) -> bool {
        println!(
            "Login sequence started for user '{}' ({})",
            packet.name, packet.uuid
        );

        let pre_login_event = self.dispatch_pre_login_event(&packet);
        if pre_login_event.cancelled {
            return true;
        }

        let Some(authentication_artifacts) = self.create_authentication_artifacts() else {
            return false;
        };

        if !self.store_login_metadata(&packet, &authentication_artifacts) {
            return true;
        }

        self.dispatch_login_response(packet, pre_login_event, authentication_artifacts)
    }

    fn dispatch_pre_login_event(&mut self, packet: &LoginStartPacket) -> PreLoginEvent {
        let mut pre_login_event = PreLoginEvent::new(packet.name.clone(), packet.uuid, false);
        pre_login_event.dispatch(self.server, self.client);
        pre_login_event
    }

    fn create_authentication_artifacts(&self) -> Option<LoginAuthenticationArtifacts> {
        let private_key = RsaPrivateKey::new(&mut OsRng, 1024).ok()?;
        let public_key_der = private_key.to_public_key().to_public_key_der().ok()?;
        let verify_token = Self::create_verify_token();

        Some(LoginAuthenticationArtifacts {
            private_key,
            public_key_der: public_key_der.as_ref().to_vec(),
            verify_token,
        })
    }

    fn create_verify_token() -> Vec<u8> {
        let mut verify_token = [0u8; 16];
        OsRng.fill_bytes(&mut verify_token);
        verify_token.to_vec()
    }

    fn store_login_metadata(
        &mut self,
        packet: &LoginStartPacket,
        authentication_artifacts: &LoginAuthenticationArtifacts,
    ) -> bool {
        let Some(login_metadata) = &mut self.client.login_metadata else {
            println!(
                "Error: Client {} sent Login Start Packet without prior Intention Packet.",
                self.client.addr
            );
            if let Err(error) = self
                .server
                .disconnect(self.client, Component::text("Invalid login sequence."))
            {
                eprintln!(
                    "Failed to send disconnect packet to {}: {}",
                    self.client.addr, error
                );
            }
            return false;
        };

        login_metadata.username = Some(packet.name.clone());
        login_metadata.uuid = Some(packet.uuid);
        login_metadata.private_key = Some(authentication_artifacts.private_key.clone());
        login_metadata.verify_token = Some(authentication_artifacts.verify_token.clone());
        true
    }

    fn dispatch_login_response(
        &mut self,
        packet: LoginStartPacket,
        pre_login_event: PreLoginEvent,
        authentication_artifacts: LoginAuthenticationArtifacts,
    ) -> bool {
        if pre_login_event.should_authenticate {
            return self.dispatch_encryption_request(authentication_artifacts);
        }

        self.dispatch_login_success(packet)
    }

    fn dispatch_encryption_request(
        &mut self,
        authentication_artifacts: LoginAuthenticationArtifacts,
    ) -> bool {
        let encryption_request_packet = EncryptionRequestPacket::new(
            "".to_string(),
            authentication_artifacts.public_key_der,
            authentication_artifacts.verify_token,
            true,
        );

        if encryption_request_packet.dispatch(self.client).is_err() {
            return false;
        }

        println!("Sent encryption request to {}.", self.client.addr);
        true
    }

    fn dispatch_login_success(&mut self, packet: LoginStartPacket) -> bool {
        use spinel_core::network::clientbound::login::login_success::LoginSuccessPacket;

        if LoginSuccessPacket::new(packet.uuid, packet.name)
            .dispatch(self.client)
            .is_err()
        {
            return false;
        }

        println!("Sent Login Success to {} (offline mode).", self.client.addr);
        true
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
    LoginStartHandler::new(client, server).handle(packet)
}
