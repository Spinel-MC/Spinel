use crate::command::CommandSenderKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandConditionContext {
    sender_kind: CommandSenderKind,
    permission_level: i32,
}

pub type CommandCondition = fn(CommandConditionContext, Option<&str>) -> bool;

impl CommandConditionContext {
    pub const fn console() -> Self {
        Self {
            sender_kind: CommandSenderKind::Console,
            permission_level: 4,
        }
    }

    pub const fn server() -> Self {
        Self {
            sender_kind: CommandSenderKind::Server,
            permission_level: 4,
        }
    }

    pub const fn player(permission_level: i32) -> Self {
        Self {
            sender_kind: CommandSenderKind::Player,
            permission_level,
        }
    }

    pub const fn sender_kind(self) -> CommandSenderKind {
        self.sender_kind
    }

    pub const fn permission_level(self) -> i32 {
        self.permission_level
    }
}

impl From<CommandSenderKind> for CommandConditionContext {
    fn from(sender_kind: CommandSenderKind) -> Self {
        match sender_kind {
            CommandSenderKind::Player => Self::player(0),
            CommandSenderKind::Server => Self::server(),
            CommandSenderKind::Console => Self::console(),
        }
    }
}
