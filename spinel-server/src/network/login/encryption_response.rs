use crate::network::client::instance::Client;
use crate::network::login::session::MojangSessionVerifier;
use crate::server::MinecraftServer;
use rsa::Pkcs1v15Encrypt;
use rsa::RsaPrivateKey;
use spinel_core::network::serverbound::login::encryption_response::EncryptionResponsePacket;
use spinel_macros::fn_packet_listener;
use spinel_network::types::game_profile::GameProfile;
use spinel_utils::component::Component;

struct VerifiedLoginMetadata {
    private_key: RsaPrivateKey,
    expected_verify_token: Vec<u8>,
    public_key_der: Vec<u8>,
    game_profile: GameProfile,
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
            return self.kick_for_invalid_login_sequence();
        };

        let Some(shared_secret) =
            self.decrypt_payload(&login_metadata.private_key, &packet.keybytes)
        else {
            return true;
        };
        let Some(verify_token) =
            self.decrypt_payload(&login_metadata.private_key, &packet.encrypted_challenge)
        else {
            return true;
        };

        if verify_token != login_metadata.expected_verify_token {
            return self.kick_for_invalid_login_sequence();
        }

        self.complete_login(login_metadata, &shared_secret)
    }

    fn load_login_metadata(&mut self) -> Option<VerifiedLoginMetadata> {
        let login_metadata = self.client.login_metadata.as_ref()?;
        let private_key = login_metadata.private_key.clone()?;
        let expected_verify_token = login_metadata.verify_token.clone()?;
        let public_key_der = login_metadata.public_key_der.clone()?;
        let game_profile = login_metadata.game_profile.clone()?;

        Some(VerifiedLoginMetadata {
            private_key,
            expected_verify_token,
            public_key_der,
            game_profile,
        })
    }

    fn decrypt_payload(
        &mut self,
        private_key: &RsaPrivateKey,
        encrypted_bytes: &[i8],
    ) -> Option<Vec<u8>> {
        let encrypted_bytes: Vec<u8> = encrypted_bytes.iter().map(|&byte| byte as u8).collect();
        private_key
            .decrypt(Pkcs1v15Encrypt, &encrypted_bytes)
            .ok()
            .or_else(|| {
                self.kick_for_invalid_login_sequence();
                None
            })
    }

    fn complete_login(
        &mut self,
        login_metadata: VerifiedLoginMetadata,
        shared_secret: &[u8],
    ) -> bool {
        self.client.enable_encryption(shared_secret);
        let Some(verified_game_profile) = MojangSessionVerifier::verify_joined_profile(
            &login_metadata.game_profile.username,
            shared_secret,
            &login_metadata.public_key_der,
        ) else {
            return self.kick_for_failed_session_verification();
        };

        if let Some(login_metadata) = self.client.login_metadata.as_mut() {
            login_metadata.game_profile = Some(verified_game_profile.clone());
        }

        let Some(verified_game_profile) =
            self.dispatch_authenticated_pre_login_event(verified_game_profile)
        else {
            return true;
        };
        if self.client.has_pending_login_plugin_requests() {
            self.store_pending_plugin_completion();
            return true;
        }

        if self
            .client
            .transition_login_to_configuration(verified_game_profile)
            .is_err()
        {
            return false;
        }

        true
    }

    fn dispatch_authenticated_pre_login_event(
        &mut self,
        game_profile: GameProfile,
    ) -> Option<GameProfile> {
        let mut pre_login_event = crate::events::login::PreLoginEvent::new(
            game_profile.username.clone(),
            game_profile.uuid,
            false,
        );
        pre_login_event.set_game_profile(game_profile);
        pre_login_event.dispatch(self.server, self.client);
        if pre_login_event.cancelled {
            return None;
        }
        Some(pre_login_event.into_game_profile())
    }

    fn store_pending_plugin_completion(&mut self) {
        let Some(login_metadata) = self.client.login_metadata.as_mut() else {
            return;
        };
        login_metadata.pending_plugin_completion =
            Some(crate::network::client::metadata::PendingPluginLoginCompletion::Offline);
    }
    fn kick_for_failed_session_verification(&mut self) -> bool {
        let _ = self
            .server
            .kick(self.client, Component::text("Failed to verify username."));
        true
    }

    fn kick_for_invalid_login_sequence(&mut self) -> bool {
        let _ = self
            .server
            .kick(self.client, Component::text("Invalid login sequence."));
        true
    }
}

#[fn_packet_listener()]
fn on_encryption_response(
    client: &mut Client,
    packet: EncryptionResponsePacket,
    server: &mut MinecraftServer,
) -> bool {
    EncryptionResponseHandler::new(client, server).handle(packet)
}
