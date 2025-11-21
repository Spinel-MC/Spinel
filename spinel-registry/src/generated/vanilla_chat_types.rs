use crate::chat_type::{ChatStyle, ChatType, ChatTypeDecoration, ChatTypeRegistry};
use crate::types::Identifier;
pub const CHAT: &ChatType = &ChatType {
    key: Identifier::vanilla_static("chat"),
    chat: ChatTypeDecoration {
        translation_key: "chat.type.text",
        parameters: &["sender", "content"],
        style: None,
    },
    narration: ChatTypeDecoration {
        translation_key: "chat.type.text.narrate",
        parameters: &["sender", "content"],
        style: None,
    },
};
pub const EMOTE_COMMAND: &ChatType = &ChatType {
    key: Identifier::vanilla_static("emote_command"),
    chat: ChatTypeDecoration {
        translation_key: "chat.type.emote",
        parameters: &["sender", "content"],
        style: None,
    },
    narration: ChatTypeDecoration {
        translation_key: "chat.type.emote",
        parameters: &["sender", "content"],
        style: None,
    },
};
pub const MSG_COMMAND_INCOMING: &ChatType = &ChatType {
    key: Identifier::vanilla_static("msg_command_incoming"),
    chat: ChatTypeDecoration {
        translation_key: "commands.message.display.incoming",
        parameters: &["sender", "content"],
        style: Some(ChatStyle {
            color: Some("gray"),
            bold: None,
            italic: Some(true),
            underlined: None,
            strikethrough: None,
            obfuscated: None,
        }),
    },
    narration: ChatTypeDecoration {
        translation_key: "chat.type.text.narrate",
        parameters: &["sender", "content"],
        style: None,
    },
};
pub const MSG_COMMAND_OUTGOING: &ChatType = &ChatType {
    key: Identifier::vanilla_static("msg_command_outgoing"),
    chat: ChatTypeDecoration {
        translation_key: "commands.message.display.outgoing",
        parameters: &["target", "content"],
        style: Some(ChatStyle {
            color: Some("gray"),
            bold: None,
            italic: Some(true),
            underlined: None,
            strikethrough: None,
            obfuscated: None,
        }),
    },
    narration: ChatTypeDecoration {
        translation_key: "chat.type.text.narrate",
        parameters: &["sender", "content"],
        style: None,
    },
};
pub const SAY_COMMAND: &ChatType = &ChatType {
    key: Identifier::vanilla_static("say_command"),
    chat: ChatTypeDecoration {
        translation_key: "chat.type.announcement",
        parameters: &["sender", "content"],
        style: None,
    },
    narration: ChatTypeDecoration {
        translation_key: "chat.type.text.narrate",
        parameters: &["sender", "content"],
        style: None,
    },
};
pub const TEAM_MSG_COMMAND_INCOMING: &ChatType = &ChatType {
    key: Identifier::vanilla_static("team_msg_command_incoming"),
    chat: ChatTypeDecoration {
        translation_key: "chat.type.team.text",
        parameters: &["target", "sender", "content"],
        style: None,
    },
    narration: ChatTypeDecoration {
        translation_key: "chat.type.text.narrate",
        parameters: &["sender", "content"],
        style: None,
    },
};
pub const TEAM_MSG_COMMAND_OUTGOING: &ChatType = &ChatType {
    key: Identifier::vanilla_static("team_msg_command_outgoing"),
    chat: ChatTypeDecoration {
        translation_key: "chat.type.team.sent",
        parameters: &["target", "sender", "content"],
        style: None,
    },
    narration: ChatTypeDecoration {
        translation_key: "chat.type.text.narrate",
        parameters: &["sender", "content"],
        style: None,
    },
};
pub fn register_chat_types(registry: &mut ChatTypeRegistry) {
    registry.register(CHAT);
    registry.register(EMOTE_COMMAND);
    registry.register(MSG_COMMAND_INCOMING);
    registry.register(MSG_COMMAND_OUTGOING);
    registry.register(SAY_COMMAND);
    registry.register(TEAM_MSG_COMMAND_INCOMING);
    registry.register(TEAM_MSG_COMMAND_OUTGOING);
}
