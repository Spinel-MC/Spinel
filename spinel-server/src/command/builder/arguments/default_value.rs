use crate::command::{ArgumentError, CommandSender};
use std::sync::Arc;

type SenderDefaultValue<T> =
    dyn for<'sender> Fn(&CommandSender<'sender>) -> Result<T, ArgumentError> + Send + Sync;

#[derive(Clone)]
pub enum DefaultValue<T> {
    Value(T),
    Supplier(Arc<dyn Fn() -> T + Send + Sync>),
    SenderProvider(Arc<dyn for<'sender> Fn(&CommandSender<'sender>) -> T + Send + Sync>),
    FallibleSenderProvider(Arc<SenderDefaultValue<T>>),
}

impl<T: Clone> DefaultValue<T> {
    pub fn resolve(&self, sender: &CommandSender<'_>) -> Result<T, ArgumentError> {
        match self {
            Self::Value(value) => Ok(value.clone()),
            Self::Supplier(supplier) => Ok(supplier()),
            Self::SenderProvider(provider) => Ok(provider(sender)),
            Self::FallibleSenderProvider(provider) => provider(sender),
        }
    }
}
