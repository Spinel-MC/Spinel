use rsa::RsaPrivateKey;
use spinel_network::types::game_profile::GameProfile;

pub struct LoginMetadata {
    pub protocol_version: i32,
    pub game_profile: Option<GameProfile>,
    pub private_key: Option<RsaPrivateKey>,
    pub verify_token: Option<Vec<u8>>,
    pub pending_plugin_completion: Option<PendingPluginLoginCompletion>,
}

pub struct PendingPluginLoginCompletion {
    pub should_authenticate: bool,
    pub public_key_der: Vec<u8>,
    pub verify_token: Vec<u8>,
}

impl LoginMetadata {
    pub fn new(protocol_version: i32) -> Self {
        Self {
            protocol_version,
            game_profile: None,
            private_key: None,
            verify_token: None,
            pending_plugin_completion: None,
        }
    }
}
