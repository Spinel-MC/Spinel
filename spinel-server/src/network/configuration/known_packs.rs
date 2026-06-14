use std::thread::sleep;
use std::time::Duration;

use crate::entity::{Entity, Player};
use crate::events::player_configuration::AsyncPlayerConfigurationEvent;
use crate::events::player_skin_init::PlayerSkinInitEvent;
use crate::network::client::instance::Client;
use crate::server::MinecraftServer;
use spinel_core::network::clientbound::configuration::features::FeaturesPacket;
use spinel_core::network::clientbound::configuration::finish_configuration::FinishConfigurationPacket;
use spinel_core::network::clientbound::configuration::reset_chat::ResetChatPacket;
use spinel_core::network::serverbound::configuration::known_packs::KnownPacksPacket;
use spinel_macros::packet_listener;
use spinel_network::types::{Array, Identifier};
use spinel_utils::component::Component;
use spinel_utils::component::color::{NamedTextColor, TextColor};
use spinel_utils::component::text::TextComponent;
use std::io;
use uuid::Uuid;

struct ConfigurationCompletion<'a, 'b> {
    client: &'a mut Client,
    server: &'b mut MinecraftServer,
}

struct ConfigurationPacketOptions {
    feature_flags: Vec<Identifier>,
    should_clear_chat: bool,
    should_send_registry_data: bool,
}

impl<'a, 'b> ConfigurationCompletion<'a, 'b> {
    fn dispatch(mut self) -> io::Result<()> {
        let Some(packet_options) = self.configure_player() else {
            return Ok(());
        };
        FeaturesPacket {
            features: Array(packet_options.feature_flags),
        }
        .dispatch(self.client)?;
        if packet_options.should_send_registry_data {
            self.dispatch_registry_packets()?;
            self.server
                .registry_cache
                .tag_packet()
                .clone()
                .dispatch(self.client)?;
        }
        if packet_options.should_clear_chat {
            ResetChatPacket.dispatch(self.client)?;
        }
        sleep(Duration::from_millis(100));
        FinishConfigurationPacket::new().dispatch(self.client)
    }

    fn configure_player(&mut self) -> Option<ConfigurationPacketOptions> {
        let Some(player) = create_player(self.client, self.server) else {
            let _ = self
                .server
                .kick(self.client, Component::text("Invalid login sequence."));
            return None;
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
            return None;
        };

        let is_hardcore = event.is_hardcore();
        let packet_options = ConfigurationPacketOptions {
            feature_flags: event.feature_flags().to_vec(),
            should_clear_chat: event.should_clear_chat(),
            should_send_registry_data: event.should_send_registry_data(),
        };
        let mut player = event.into_player();
        player.set_pending_options(spawning_world, is_hardcore);

        place_player(self.client, self.server, spawning_world, player).then_some(packet_options)
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

fn create_player(client: &mut Client, server: &mut MinecraftServer) -> Option<Player> {
    let login_metadata = client.login_metadata.as_ref()?;
    let game_profile = login_metadata.game_profile.as_ref()?;
    let mut player = Player::new(
        game_profile.uuid,
        game_profile.username.clone(),
        login_metadata.protocol_version,
        client.addr,
    );
    player.apply_skin(game_profile.properties.iter().find_map(|property| {
        (property.name == "textures").then(|| {
            crate::entity::player::PlayerSkin::new(
                property.value.clone(),
                property.signature.clone(),
            )
        })
    }));
    player.set_client(client);
    player.refresh_settings(client.pending_client_settings.clone());
    let player_pointer = &mut player as *mut Player;
    let mut skin_init_event = PlayerSkinInitEvent::new(player_pointer, player.skin().cloned());
    skin_init_event.dispatch(server, client);
    player.apply_skin(skin_init_event.into_skin());
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
    let _ = server.kick(client, error_during_login());
}

fn error_during_login() -> TextComponent {
    TextComponent::literal_with_color(
        "Error during login!",
        TextColor::from_named(NamedTextColor::Red),
    )
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
