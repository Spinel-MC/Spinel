use crate::command::CommandSenderKind;

pub type CommandCondition = fn(CommandSenderKind, Option<&str>) -> bool;
