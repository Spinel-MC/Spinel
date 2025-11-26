use crate as spinel;
use spinel_macros::packet_dispatcher;

#[packet_dispatcher(id: "hello", state: ConnectionState::Login)]
pub struct EncryptionRequestPacket {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub verify_token: Vec<u8>,
    pub should_authenticate: bool,
}

impl EncryptionRequestPacket {
    pub fn new(
        server_id: String,
        public_key: Vec<u8>,
        verify_token: Vec<u8>,
        should_authenticate: bool,
    ) -> Self {
        Self {
            server_id,
            public_key,
            verify_token,
            should_authenticate,
        }
    }
}
