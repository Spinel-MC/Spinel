use spinel_macros::event_dispatcher;
use spinel_network::ConnectionState;
use spinel_utils::component::text::TextComponent;

#[event_dispatcher(with_server: true)]
pub struct DisconnectEvent {
    pub state: ConnectionState,
    pub reason: TextComponent,
}

impl DisconnectEvent {
    pub fn new(state: ConnectionState, reason: TextComponent) -> Self {
        Self {
            state,
            reason,
            connection_ptr: None,
        }
    }
}
