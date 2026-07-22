use crate::command::{Argument, ArgumentBehavior, ArgumentClass, ArgumentError, CommandSender};
use spinel_core::network::clientbound::play::commands::ArgumentParserType;
use std::sync::Arc;

type SenderMapper<I, O> =
    dyn for<'sender> Fn(&CommandSender<'sender>, I) -> Result<O, ArgumentError> + Send + Sync;

pub(crate) fn map_argument<I: Clone + Send + Sync + 'static, O: Clone + Send + Sync + 'static>(
    argument: Argument<I>,
    mapper: impl Fn(I) -> Result<O, ArgumentError> + Send + Sync + 'static,
) -> Argument<O> {
    map_argument_for_sender(argument, move |_sender, value| mapper(value))
}

pub(crate) fn map_argument_for_sender<
    I: Clone + Send + Sync + 'static,
    O: Clone + Send + Sync + 'static,
>(
    argument: Argument<I>,
    mapper: impl for<'sender> Fn(&CommandSender<'sender>, I) -> Result<O, ArgumentError>
    + Send
    + Sync
    + 'static,
) -> Argument<O> {
    let mapper: Arc<SenderMapper<I, O>> = Arc::new(mapper);
    let behavior = MappedArgumentBehavior {
        argument: argument.clone(),
        mapper: Arc::clone(&mapper),
    };
    let mut mapped = Argument::custom(argument.get_id(), behavior);
    mapped.set_allows_space(argument.allows_space());
    mapped.set_uses_remaining_input(argument.uses_remaining_input());
    if let Some(callback) = argument.get_callback() {
        mapped.set_callback(Some(callback));
    }
    if let Some(callback) = argument.get_suggestion_callback() {
        mapped.set_suggestion_callback(callback);
    }
    if argument.is_optional() {
        let source = argument.clone();
        mapped.set_default_value_result_provider(move |sender| {
            source
                .get_default_value(sender)?
                .ok_or_else(|| {
                    ArgumentError::new("Mapped default value is absent", source.get_id(), 555)
                })
                .and_then(|value| mapper(sender, value))
        });
    }
    mapped
}

pub(crate) fn filter_argument<T: Clone + Send + Sync + 'static>(
    argument: Argument<T>,
    predicate: impl Fn(&T) -> bool + Send + Sync + 'static,
) -> Argument<T> {
    let predicate: Arc<dyn Fn(&T) -> bool + Send + Sync> = Arc::new(predicate);
    let behavior = FilteredArgumentBehavior {
        argument: argument.clone(),
        predicate,
    };
    let mut filtered = Argument::custom(argument.get_id(), behavior);
    filtered.set_allows_space(argument.allows_space());
    filtered.set_uses_remaining_input(argument.uses_remaining_input());
    if let Some(callback) = argument.get_callback() {
        filtered.set_callback(Some(callback));
    }
    if let Some(callback) = argument.get_suggestion_callback() {
        filtered.set_suggestion_callback(callback);
    }
    if argument.is_optional() {
        argument.copy_default_value_to(&mut filtered);
    }
    filtered
}
struct MappedArgumentBehavior<I, O> {
    argument: Argument<I>,
    mapper: Arc<SenderMapper<I, O>>,
}
impl<I: Clone + Send + Sync + 'static, O: Clone + Send + Sync + 'static> ArgumentBehavior<O>
    for MappedArgumentBehavior<I, O>
{
    fn parse_input(&self, sender: &CommandSender<'_>, input: &str) -> Result<O, ArgumentError> {
        self.argument
            .parse_input(sender, input)
            .and_then(|value| (self.mapper)(sender, value))
    }
    fn get_parser(&self) -> ArgumentParserType {
        self.argument.get_parser()
    }
    fn get_node_properties(&self) -> Option<Vec<u8>> {
        self.argument.get_node_properties()
    }
    fn get_concrete_class(&self) -> ArgumentClass {
        ArgumentClass::new("ArgumentMap")
    }
}
struct FilteredArgumentBehavior<T> {
    argument: Argument<T>,
    predicate: Arc<dyn Fn(&T) -> bool + Send + Sync>,
}
impl<T: Clone + Send + Sync + 'static> ArgumentBehavior<T> for FilteredArgumentBehavior<T> {
    fn parse_input(&self, sender: &CommandSender<'_>, input: &str) -> Result<T, ArgumentError> {
        let value = self.argument.parse_input(sender, input)?;
        if (self.predicate)(&value) {
            Ok(value)
        } else {
            Err(ArgumentError::new("Predicate failed", input, 556))
        }
    }
    fn get_parser(&self) -> ArgumentParserType {
        self.argument.get_parser()
    }
    fn get_node_properties(&self) -> Option<Vec<u8>> {
        self.argument.get_node_properties()
    }
    fn get_concrete_class(&self) -> ArgumentClass {
        ArgumentClass::new("ArgumentFilter")
    }
}
