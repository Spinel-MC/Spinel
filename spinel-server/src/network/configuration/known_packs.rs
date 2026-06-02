use ::std::thread::sleep;
use ::std::time::Duration;

use crate::entity::{Entity, Player};
use crate::events::player_configuration::AsyncPlayerConfigurationEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use ::spinel_core::network::clientbound::configuration::disconnect::ConfigurationDisconnectPacket;
use ::spinel_core::network::clientbound::configuration::finish_configuration::FinishConfigurationPacket;
use ::spinel_core::network::serverbound::configuration::known_packs::KnownPacksPacket;
use ::spinel_macros::packet_listener;
use ::spinel_utils::component::Component;
use ::spinel_utils::component::color::{NamedTextColor, TextColor};
use ::spinel_utils::component::text::TextComponent;
use std::io;
use uuid::Uuid;

struct ConfigurationCompletion<'a, 'b> {
    client: &'a mut Client,
    server: &'b mut MinecraftServer,
}

impl<'a, 'b> ConfigurationCompletion<'a, 'b> {
    fn dispatch(mut self) -> io::Result<()> {
        if !self.configure_player() {
            return Ok(());
        }
        self.dispatch_registry_packets()?;
        self.server
            .registry_cache
            .tag_packet()
            .clone()
            .dispatch(self.client)?;
        sleep(Duration::from_millis(100));
        FinishConfigurationPacket::new().dispatch(self.client)
    }

    fn configure_player(&mut self) -> bool {
        let Some(player) = create_player(self.client) else {
            let _ = ConfigurationDisconnectPacket::new(Component::text("Invalid login sequence."))
                .dispatch(self.client);
            let client_address = self.client.addr;
            self.client.disconnect();
            self.server.on_disconnect(client_address);
            return false;
        };

        let mut event = AsyncPlayerConfigurationEvent::new(player, true);
        tokio::runtime::Handle::current().block_on(event.dispatch(self.server, self.client));
        let username = event.player().username.clone();
        let Some(spawning_world) = event.spawning_world() else {
            fail_player_join(
                self.client,
                self.server,
                format!("{username} failed to join because no spawning world was set."),
            );
            return false;
        };

        let is_hardcore = event.is_hardcore();
        let mut player = event.into_player();
        player.set_pending_options(spawning_world, is_hardcore);

        place_player(self.client, self.server, spawning_world, player)
    }

    fn dispatch_registry_packets(&mut self) -> io::Result<()> {
        self.server
            .registry_cache
            .packets()
            .iter()
            .cloned()
            .try_for_each(|packet| packet.dispatch(self.client))
    }
}

fn create_player(client: &mut Client) -> Option<Player> {
    let login_metadata = client.login_metadata.as_ref()?;
    let mut player = Player::new(
        login_metadata.uuid?,
        login_metadata.username.clone()?,
        login_metadata.protocol_version,
        client.addr,
    );
    player.set_client(client);
    player.refresh_settings(client.pending_client_settings.clone());
    Some(player)
}

fn place_player(
    client: &mut Client,
    server: &mut MinecraftServer,
    spawning_world: Uuid,
    player: Player,
) -> bool {
    if server.world_manager.world(spawning_world).is_none() {
        fail_player_join(
            client,
            server,
            format!(
                "{} failed to join because the configured spawning world was not registered.",
                player.username
            ),
        );
        return false;
    }

    server
        .world_manager
        .add_entity(spawning_world, Entity::Player(player))
}

fn fail_player_join(client: &mut Client, server: &mut MinecraftServer, log_message: String) {
    eprintln!("{log_message}");
    let _ = ConfigurationDisconnectPacket::new(error_during_login()).dispatch(client);
    let client_address = client.addr;
    client.disconnect();
    server.on_disconnect(client_address);
}

fn error_during_login() -> TextComponent {
    TextComponent::literal_with_color(
        "Error during login!",
        TextColor::from_named(NamedTextColor::Red),
    )
}

#[cfg(test)]
mod tests {
    use super::error_during_login;

    #[test]
    fn error_during_login_is_red() {
        let json = error_during_login().to_json_string();

        assert!(json.contains("\"text\":\"Error during login!\""));
        assert!(json.contains("\"color\":\"red\""));
    }
}

#[packet_listener(module: "login")]
fn on_known_packs(
    client: &mut Client,
    _packet: KnownPacksPacket,
    server: &mut MinecraftServer,
) -> bool {
    ConfigurationCompletion { client, server }
        .dispatch()
        .is_ok()
}
