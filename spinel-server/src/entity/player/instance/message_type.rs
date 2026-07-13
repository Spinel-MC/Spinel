#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerMessageType {
    Chat,
    System,
    ActionBar,
}

impl PlayerMessageType {
    pub(super) const fn is_accepted_by_chat_mode(self, chat_mode: i32) -> bool {
        match (chat_mode, self) {
            (_, Self::ActionBar) => true,
            (0, Self::Chat | Self::System) => true,
            (1, Self::System) => true,
            _ => false,
        }
    }
}
