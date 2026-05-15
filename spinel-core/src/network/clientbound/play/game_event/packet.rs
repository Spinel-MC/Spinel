use crate::network::clientbound::play::game_event::GameEvent;
use spinel_macros::packet;

#[packet(id: "game_event", state: ConnectionState::Play, recipient: Recipient::Client)]
pub struct GameEventPacket {
    pub event: u8,
    pub value: f32,
}

impl<T: Into<GameEvent>> From<T> for GameEventPacket {
    fn from(game_event: T) -> Self {
        let game_event = game_event.into();
        Self {
            event: game_event.event_id(),
            value: game_event.value(),
        }
    }
}
