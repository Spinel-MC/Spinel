use rsa::RsaPrivateKey;
use uuid::Uuid;

pub struct LoginMetadata {
    pub protocol_version: i32,
    pub username: Option<String>,
    pub uuid: Option<Uuid>,
    pub private_key: Option<RsaPrivateKey>,
    pub verify_token: Option<Vec<u8>>,
}

impl LoginMetadata {
    pub fn new(protocol_version: i32) -> Self {
        Self {
            protocol_version,
            username: None,
            uuid: None,
            private_key: None,
            verify_token: None,
        }
    }
}
