use crate::events::intention::IntentionEvent;
use crate::events::login::PreLoginEvent;
use crate::network::client::instance::Client;
use crate::network::client::metadata::{LoginMetadata, PendingPluginLoginCompletion};
use crate::server::{Auth, MinecraftServer};

use rsa::RsaPrivateKey;
use rsa::rand_core::{OsRng, RngCore};
use spinel_core::network::clientbound::login::encryption_request::EncryptionRequestPacket;
use spinel_core::network::serverbound::login::login_start::LoginStartPacket;
use spinel_macros::{fn_event_listener, fn_packet_listener};
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
        let pre_login_event = self.dispatch_pre_login_event(&packet);
        if pre_login_event.cancelled {
            return true;
        }

        let should_authenticate = pre_login_event.should_authenticate;
        let game_profile = pre_login_event.into_game_profile();
        let authentication_artifacts = if should_authenticate {
            let Some(authentication_artifacts) = self.create_authentication_artifacts() else {
                return self.kick_for_invalid_login_sequence();
            };
            Some(authentication_artifacts)
        } else {
            None
        };

        if !self.store_login_metadata(&game_profile, authentication_artifacts.as_ref()) {
            return true;
        }

        if self.client.has_pending_login_plugin_requests() {
            self.store_pending_plugin_completion(authentication_artifacts.as_ref());
            return true;
        }

        self.dispatch_login_response(game_profile, authentication_artifacts)
    }

    fn dispatch_pre_login_event(&mut self, packet: &LoginStartPacket) -> PreLoginEvent {
        let should_authenticate = matches!(self.server.auth(), Auth::Online(_));
        let mut pre_login_event =
            PreLoginEvent::new(packet.name.clone(), packet.uuid, should_authenticate);
        pre_login_event.dispatch(self.server, self.client);
        pre_login_event
    }

    fn create_authentication_artifacts(&self) -> Option<LoginAuthenticationArtifacts> {
        let Auth::Online(online_auth) = self.server.auth() else {
            return None;
        };
        let verify_token = Self::create_verify_token();

        Some(LoginAuthenticationArtifacts {
            private_key: online_auth.get_private_key().clone(),
            public_key_der: online_auth.get_public_key_der().to_vec(),
            verify_token,
        })
    }

    fn create_verify_token() -> Vec<u8> {
        let mut verify_token = [0u8; 4];
        OsRng.fill_bytes(&mut verify_token);
        verify_token.to_vec()
    }

    fn store_login_metadata(
        &mut self,
        game_profile: &spinel_network::types::game_profile::GameProfile,
        authentication_artifacts: Option<&LoginAuthenticationArtifacts>,
    ) -> bool {
        let Some(login_metadata) = &mut self.client.login_metadata else {
            return self.kick_for_invalid_login_sequence();
        };

        login_metadata.game_profile = Some(game_profile.clone());
        if let Some(authentication_artifacts) = authentication_artifacts {
            login_metadata.private_key = Some(authentication_artifacts.private_key.clone());
            login_metadata.public_key_der = Some(authentication_artifacts.public_key_der.clone());
            login_metadata.verify_token = Some(authentication_artifacts.verify_token.clone());
        }
        true
    }

    fn dispatch_login_response(
        &mut self,
        game_profile: spinel_network::types::game_profile::GameProfile,
        authentication_artifacts: Option<LoginAuthenticationArtifacts>,
    ) -> bool {
        let Some(authentication_artifacts) = authentication_artifacts else {
            return self.dispatch_login_success(game_profile);
        };

        self.dispatch_encryption_request(authentication_artifacts)
    }

    fn store_pending_plugin_completion(
        &mut self,
        authentication_artifacts: Option<&LoginAuthenticationArtifacts>,
    ) {
        let Some(login_metadata) = self.client.login_metadata.as_mut() else {
            return;
        };
        login_metadata.pending_plugin_completion = match authentication_artifacts {
            Some(authentication_artifacts) => Some(PendingPluginLoginCompletion::Online {
                public_key_der: authentication_artifacts.public_key_der.clone(),
                verify_token: authentication_artifacts.verify_token.clone(),
            }),
            None => Some(PendingPluginLoginCompletion::Offline),
        };
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

        true
    }

    fn kick_for_invalid_login_sequence(&mut self) -> bool {
        let _ = self
            .server
            .kick(self.client, Component::text("Invalid login sequence."));
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
    match pending_completion {
        PendingPluginLoginCompletion::Online {
            public_key_der,
            verify_token,
        } => EncryptionRequestPacket::new(String::new(), public_key_der, verify_token, true)
            .dispatch(client)
            .is_ok(),
        PendingPluginLoginCompletion::Offline => {
            if client
                .transition_login_to_configuration(game_profile)
                .is_err()
            {
                return false;
            }
            true
        }
    }
}

#[fn_event_listener()]
fn on_intention(event: &mut IntentionEvent, _server: &mut MinecraftServer) {
    event.client().login_metadata = Some(LoginMetadata::new(event.protocol_version));
}

#[fn_packet_listener()]
pub(super) fn on_login_start(
    client: &mut Client,
    packet: LoginStartPacket,
    server: &mut MinecraftServer,
) -> bool {
    LoginStartHandler::new(client, server).handle(packet)
}
