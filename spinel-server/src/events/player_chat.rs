use crate::entity::Player;
use spinel_macros::event_dispatcher;
use spinel_utils::component::text::TextComponent;
use uuid::Uuid;

#[event_dispatcher(with_client: true)]
pub struct PlayerChatEvent {
    player: *mut Player,
    recipients: Vec<Uuid>,
    raw_message: String,
    formatted_message: TextComponent,
    cancelled: bool,
}

impl PlayerChatEvent {
    pub fn new(player: *mut Player, recipients: Vec<Uuid>, raw_message: String) -> Self {
        let formatted_message = default_formatted_chat_message(player, &raw_message);
        Self {
            player,
            recipients,
            raw_message,
            formatted_message,
            cancelled: false,
            connection_ptr: None,
        }
    }

    pub fn player(&mut self) -> &mut Player {
        unsafe { &mut *self.player }
    }

    pub fn recipients(&self) -> &[Uuid] {
        &self.recipients
    }

    pub fn recipients_mut(&mut self) -> &mut Vec<Uuid> {
        &mut self.recipients
    }

    pub fn raw_message(&self) -> &str {
        &self.raw_message
    }

    pub fn formatted_message(&self) -> &TextComponent {
        &self.formatted_message
    }

    pub fn set_formatted_message(&mut self, formatted_message: TextComponent) {
        self.formatted_message = formatted_message;
    }

    pub const fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }

    pub fn into_message(self) -> (Vec<Uuid>, TextComponent) {
        (self.recipients, self.formatted_message)
    }
}

fn default_formatted_chat_message(player: *mut Player, raw_message: &str) -> TextComponent {
    let username = unsafe { &*player }.get_username().to_owned();
    TextComponent::translatable("chat.type.text")
        .argument(
            TextComponent::text(username.clone())
                .insertion(username)
                .build(),
        )
        .argument(TextComponent::literal(raw_message))
        .build()
}
