use crate::command::{ArgumentError, CommandSender};

pub type ArgumentCallback = for<'sender> fn(&CommandSender<'sender>, &ArgumentError);
