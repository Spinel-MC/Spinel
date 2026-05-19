use ::std::thread::sleep;
use ::std::time::Duration;

use crate::entity::{Entity, Player};
use crate::events::player_configuration::AsyncPlayerConfigurationEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
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
            .get_tag_packet()
            .clone()
            .dispatch(self.client)?;
        sleep(Duration::from_millis(100));
        FinishConfigurationPacket::new().dispatch(self.client)
    }

    fn configure_player(&mut self) -> bool {
        let Some(player) = create_player(self.client) else {
            let _ = self
                .server
                .disconnect(self.client, Component::text("Invalid login sequence."));
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

        place_player(
            self.client,
            self.server,
            spawning_world,
            event.into_player(),
        )
    }

    fn dispatch_registry_packets(&mut self) -> io::Result<()> {
        self.server
            .registry_cache
            .get_packets()
            .iter()
            .cloned()
            .try_for_each(|packet| packet.dispatch(self.client))
    }
}

fn create_player(client: &Client) -> Option<Player> {
    let login_metadata = client.login_metadata.as_ref()?;
    Some(Player::new(
        login_metadata.uuid?,
        login_metadata.username.clone()?,
        login_metadata.protocol_version,
        client.addr,
    ))
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
    let _ = server.disconnect(client, error_during_login());
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
