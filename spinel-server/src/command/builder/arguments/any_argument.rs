use crate::command::{ArgumentError, CommandSender};
use spinel_core::network::clientbound::play::commands::ArgumentParserType;
use std::any::Any;
use std::sync::Arc;

#[derive(Clone)]
pub struct AnyArgumentValue(Arc<dyn Any + Send + Sync>);

impl AnyArgumentValue {
    pub fn new<T: Any + Send + Sync>(value: T) -> Self {
        Self(Arc::new(value))
    }
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.0.downcast_ref()
    }
}

pub trait AnyArgument: Send + Sync {
    fn get_id(&self) -> &str;
    fn get_parser(&self) -> ArgumentParserType;
    fn get_node_properties(&self) -> Option<Vec<u8>>;
    fn parse_erased(
        &self,
        sender: &CommandSender<'_>,
        input: &str,
    ) -> Result<AnyArgumentValue, ArgumentError>;
}
