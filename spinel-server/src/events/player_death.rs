use crate::entity::Player;
use spinel_macros::event_dispatcher;
use spinel_utils::component::text::TextComponent;

#[event_dispatcher(with_client: true)]
pub struct PlayerDeathEvent {
    player: *mut Player,
    death_text: Option<TextComponent>,
    chat_message: Option<TextComponent>,
}

impl PlayerDeathEvent {
    pub fn new(
        player: *mut Player,
        death_text: Option<TextComponent>,
        chat_message: Option<TextComponent>,
    ) -> Self {
        Self {
            player,
            death_text,
            chat_message,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn death_text(&self) -> Option<&TextComponent> {
        self.death_text.as_ref()
    }

    pub fn set_death_text(&mut self, death_text: Option<TextComponent>) {
        self.death_text = death_text;
    }

    pub fn chat_message(&self) -> Option<&TextComponent> {
        self.chat_message.as_ref()
    }

    pub fn set_chat_message(&mut self, chat_message: Option<TextComponent>) {
        self.chat_message = chat_message;
    }

    pub fn into_messages(self) -> (Option<TextComponent>, Option<TextComponent>) {
        (self.death_text, self.chat_message)
    }
}
