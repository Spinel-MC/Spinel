use crate::events::block_interact::BlockInteractListener;
use crate::events::game_mode_change::GameModeChangeListener;
use crate::events::info::inbound_packet::InboundPacketInfoListener;
use crate::events::info::outbound_packet::OutboundPacketInfoListener;
use crate::events::info::packet_error::PacketErrorInfoListener;
use crate::events::info::pre_login::PreLoginInfoListener;
use crate::events::info::signal::SignalInfoListener;
use crate::events::info::startup::StartupInfoListener;
use crate::events::player_configuration::PlayerConfigurationListener;
use crate::events::player_join::PlayerJoinListener;
use crate::events::player_quit::PlayerQuitListener;
use crate::events::server_list_ping::event::ServerListPingListener;
use spinel::server::MinecraftServer;

pub struct TestServerEventHandlers;

impl TestServerEventHandlers {
    pub fn register(server: &mut MinecraftServer) {
        server.register_event_handler(StartupInfoListener);
        server.register_event_handler(SignalInfoListener);
        server.register_event_handler(PreLoginInfoListener);
        server.register_event_handler(InboundPacketInfoListener);
        server.register_event_handler(OutboundPacketInfoListener);
        server.register_event_handler(PacketErrorInfoListener);
        server.register_event_handler(ServerListPingListener);
        server.register_event_handler(PlayerJoinListener);
        server.register_event_handler(PlayerQuitListener);
        server.register_event_handler(PlayerConfigurationListener);
        server.register_event_handler(BlockInteractListener);
        server.register_event_handler(GameModeChangeListener);
    }
}
