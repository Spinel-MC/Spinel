use crate::{DynamicRegistry, RegistryKey};
use crate::chat_type::ChatType;
impl ChatType {
    pub const CHAT: RegistryKey<Self> = RegistryKey::vanilla_static("chat");
    pub const EMOTE_COMMAND: RegistryKey<Self> = RegistryKey::vanilla_static("emote_command");
    pub const MSG_COMMAND_INCOMING: RegistryKey<Self> = RegistryKey::vanilla_static("msg_command_incoming");
    pub const MSG_COMMAND_OUTGOING: RegistryKey<Self> = RegistryKey::vanilla_static("msg_command_outgoing");
    pub const SAY_COMMAND: RegistryKey<Self> = RegistryKey::vanilla_static("say_command");
    pub const TEAM_MSG_COMMAND_INCOMING: RegistryKey<Self> = RegistryKey::vanilla_static("team_msg_command_incoming");
    pub const TEAM_MSG_COMMAND_OUTGOING: RegistryKey<Self> = RegistryKey::vanilla_static("team_msg_command_outgoing");
}
pub fn register_chat_types(registry: &mut DynamicRegistry<ChatType>) {
    let _ = registry.register_vanilla(ChatType::CHAT, ChatType::default());
    let _ = registry.register_vanilla(ChatType::EMOTE_COMMAND, ChatType::default());
    let _ = registry.register_vanilla(ChatType::MSG_COMMAND_INCOMING, ChatType::default());
    let _ = registry.register_vanilla(ChatType::MSG_COMMAND_OUTGOING, ChatType::default());
    let _ = registry.register_vanilla(ChatType::SAY_COMMAND, ChatType::default());
    let _ = registry.register_vanilla(ChatType::TEAM_MSG_COMMAND_INCOMING, ChatType::default());
    let _ = registry.register_vanilla(ChatType::TEAM_MSG_COMMAND_OUTGOING, ChatType::default());
}
