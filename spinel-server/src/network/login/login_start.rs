use crate::events::intention::IntentionEvent;
use crate::events::login::PreLoginEvent;
use crate::network::client::instance::Client;
use crate::network::client::metadata::{LoginMetadata, PendingPluginLoginCompletion};
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

        let should_authenticate = pre_login_event.should_authenticate;
        let game_profile = pre_login_event.into_game_profile();
        if !self.store_login_metadata(&game_profile, &authentication_artifacts) {
            return true;
        }

        if self.client.has_pending_login_plugin_requests() {
            self.store_pending_plugin_completion(should_authenticate, &authentication_artifacts);
            return true;
        }

        self.dispatch_login_response(game_profile, should_authenticate, authentication_artifacts)
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
        game_profile: &spinel_network::types::game_profile::GameProfile,
        authentication_artifacts: &LoginAuthenticationArtifacts,
    ) -> bool {
        let Some(login_metadata) = &mut self.client.login_metadata else {
            println!(
                "Error: Client {} sent Login Start Packet without prior Intention Packet.",
                self.client.addr
            );
            if let Err(error) = self
                .server
                .kick(self.client, Component::text("Invalid login sequence."))
            {
                eprintln!(
                    "Failed to send disconnect packet to {}: {}",
                    self.client.addr, error
                );
            }
            return false;
        };

        login_metadata.game_profile = Some(game_profile.clone());
        login_metadata.private_key = Some(authentication_artifacts.private_key.clone());
        login_metadata.verify_token = Some(authentication_artifacts.verify_token.clone());
        true
    }

    fn dispatch_login_response(
        &mut self,
        game_profile: spinel_network::types::game_profile::GameProfile,
        should_authenticate: bool,
        authentication_artifacts: LoginAuthenticationArtifacts,
    ) -> bool {
        if should_authenticate {
            return self.dispatch_encryption_request(authentication_artifacts);
        }

        self.dispatch_login_success(game_profile)
    }

    fn store_pending_plugin_completion(
        &mut self,
        should_authenticate: bool,
        authentication_artifacts: &LoginAuthenticationArtifacts,
    ) {
        let Some(login_metadata) = self.client.login_metadata.as_mut() else {
            return;
        };
        login_metadata.pending_plugin_completion = Some(PendingPluginLoginCompletion {
            should_authenticate,
            public_key_der: authentication_artifacts.public_key_der.clone(),
            verify_token: authentication_artifacts.verify_token.clone(),
        });
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

    fn dispatch_login_success(
        &mut self,
        game_profile: spinel_network::types::game_profile::GameProfile,
    ) -> bool {
        if self
            .client
            .transition_login_to_configuration(game_profile)
            .is_err()
        {
            return false;
        }

        println!("Sent Login Success to {} (offline mode).", self.client.addr);
        true
    }
}

pub(crate) fn resume_login_after_plugin_responses(client: &mut Client) -> bool {
    if client.has_pending_login_plugin_requests() {
        return true;
    }
    let Some(login_metadata) = client.login_metadata.as_mut() else {
        return false;
    };
    let Some(pending_completion) = login_metadata.pending_plugin_completion.take() else {
        return true;
    };
    let Some(game_profile) = login_metadata.game_profile.clone() else {
        return false;
    };
    if pending_completion.should_authenticate {
        return EncryptionRequestPacket::new(
            String::new(),
            pending_completion.public_key_der,
            pending_completion.verify_token,
            true,
        )
        .dispatch(client)
        .is_ok();
    }
    if client
        .transition_login_to_configuration(game_profile)
        .is_err()
    {
        return false;
    }
    true
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
