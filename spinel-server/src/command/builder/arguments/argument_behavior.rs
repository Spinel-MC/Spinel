use crate::command::{ArgumentClass, ArgumentError, CommandSender};
use spinel_core::network::clientbound::play::commands::ArgumentParserType;

pub trait ArgumentBehavior<T>: Send + Sync + 'static {
    fn parse_input(&self, sender: &CommandSender<'_>, input: &str) -> Result<T, ArgumentError>;
    fn get_parser(&self) -> ArgumentParserType;
    fn get_node_properties(&self) -> Option<Vec<u8>>;
    fn get_concrete_class(&self) -> ArgumentClass;
}
