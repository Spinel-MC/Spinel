use crate::network::client::instance::Client;
use spinel_core::network::clientbound::login::login_success::LoginSuccessPacket;
use spinel_core::network::clientbound::login::set_compression::SetCompressionPacket;
use spinel_network::types::game_profile::GameProfile;
use std::io;

pub(crate) const DEFAULT_COMPRESSION_THRESHOLD: i32 = 256;

impl Client {
    pub(crate) fn start_compression(&mut self, threshold: i32) -> io::Result<()> {
        SetCompressionPacket::new(threshold).dispatch(self)?;
        self.set_compression(threshold);
        Ok(())
    }

    pub(crate) fn transition_login_to_configuration(
        &mut self,
        game_profile: GameProfile,
    ) -> io::Result<()> {
        self.start_compression(DEFAULT_COMPRESSION_THRESHOLD)?;
        LoginSuccessPacket {
            profile: game_profile,
        }
        .dispatch(self)
    }
}
