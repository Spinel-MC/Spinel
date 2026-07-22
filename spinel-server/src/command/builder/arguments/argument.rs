use super::default_value::DefaultValue;
use super::mapped_argument;
use crate::command::{
    AnyArgument, AnyArgumentValue, ArgumentBehavior, ArgumentCallback, ArgumentError,
    CommandSender, SuggestionCallback, SuggestionType,
};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};

pub(crate) struct ArgumentState<T> {
    pub(crate) allows_space: bool,
    pub(crate) uses_remaining_input: bool,
    pub(crate) callback: Option<ArgumentCallback>,
    pub(crate) default_value: Option<DefaultValue<T>>,
    pub(crate) suggestion_callback: Option<SuggestionCallback>,
    pub(crate) suggestion_type: Option<SuggestionType>,
}

pub struct Argument<T> {
    pub(crate) id: String,
    pub(crate) state: Arc<RwLock<ArgumentState<T>>>,
    pub(crate) behavior: Arc<dyn ArgumentBehavior<T>>,
}

impl<T> Clone for Argument<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            state: Arc::clone(&self.state),
            behavior: Arc::clone(&self.behavior),
        }
    }
}

impl<T: Clone + Send + Sync + 'static> Argument<T> {
    pub fn custom(id: impl Into<String>, behavior: impl ArgumentBehavior<T>) -> Self {
        Self {
            id: id.into(),
            state: Arc::new(RwLock::new(ArgumentState {
                allows_space: false,
                uses_remaining_input: false,
                callback: None,
                default_value: None,
                suggestion_callback: None,
                suggestion_type: None,
            })),
            behavior: Arc::new(behavior),
        }
    }

    pub fn parse(sender: &CommandSender<'_>, argument: &Self) -> Result<T, ArgumentError> {
        argument.parse_input(sender, argument.get_id())
    }
    pub fn parse_input(&self, sender: &CommandSender<'_>, input: &str) -> Result<T, ArgumentError> {
        self.behavior.parse_input(sender, input)
    }
    pub fn set_allows_space(&mut self, allows_space: bool) {
        self.write_state().allows_space = allows_space;
    }
    pub fn set_uses_remaining_input(&mut self, uses_remaining_input: bool) {
        self.write_state().uses_remaining_input = uses_remaining_input;
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn allows_space(&self) -> bool {
        self.read_state().allows_space
    }
    pub fn uses_remaining_input(&self) -> bool {
        self.read_state().uses_remaining_input
    }
    pub fn get_parser(
        &self,
    ) -> spinel_core::network::clientbound::play::commands::ArgumentParserType {
        self.behavior.get_parser()
    }
    pub fn get_node_properties(&self) -> Option<Vec<u8>> {
        self.behavior.get_node_properties()
    }
    pub fn get_suggestion_type(&self) -> Option<SuggestionType> {
        self.read_state().suggestion_type
    }
    pub fn get_callback(&self) -> Option<ArgumentCallback> {
        self.read_state().callback
    }
    pub fn set_callback(&mut self, callback: Option<ArgumentCallback>) {
        self.write_state().callback = callback;
    }
    pub fn has_error_callback(&self) -> bool {
        self.read_state().callback.is_some()
    }
    pub fn is_optional(&self) -> bool {
        self.read_state().default_value.is_some()
    }
    pub fn get_default_value(
        &self,
        sender: &CommandSender<'_>,
    ) -> Result<Option<T>, ArgumentError> {
        match self.read_state().default_value.as_ref() {
            Some(default_value) => default_value.resolve(sender).map(Some),
            None => Ok(None),
        }
    }
    pub fn set_default_value(&mut self, value: T) -> &mut Self {
        self.write_state().default_value = Some(DefaultValue::Value(value));
        self
    }
    pub fn set_default_value_supplier(
        &mut self,
        supplier: impl Fn() -> T + Send + Sync + 'static,
    ) -> &mut Self {
        self.write_state().default_value = Some(DefaultValue::Supplier(Arc::new(supplier)));
        self
    }
    pub fn set_default_value_for_sender(
        &mut self,
        provider: impl for<'sender> Fn(&CommandSender<'sender>) -> T + Send + Sync + 'static,
    ) -> &mut Self {
        self.write_state().default_value = Some(DefaultValue::SenderProvider(Arc::new(provider)));
        self
    }
    pub(crate) fn copy_default_value_to(&self, target: &mut Self) {
        target.write_state().default_value = self.read_state().default_value.clone();
    }
    pub(crate) fn set_default_value_result_provider(
        &mut self,
        provider: impl for<'sender> Fn(&CommandSender<'sender>) -> Result<T, ArgumentError>
        + Send
        + Sync
        + 'static,
    ) {
        self.write_state().default_value =
            Some(DefaultValue::FallibleSenderProvider(Arc::new(provider)));
    }
    pub fn get_suggestion_callback(&self) -> Option<SuggestionCallback> {
        self.read_state().suggestion_callback
    }
    pub fn set_suggestion_callback(&mut self, callback: SuggestionCallback) -> &mut Self {
        {
            let mut state = self.write_state();
            state.suggestion_callback = Some(callback);
            state.suggestion_type = Some(SuggestionType::AskServer);
        }
        self
    }
    pub fn has_suggestion(&self) -> bool {
        self.read_state().suggestion_type.is_some()
    }
    pub fn map<O: Clone + Send + Sync + 'static>(
        &self,
        mapper: impl Fn(T) -> Result<O, ArgumentError> + Send + Sync + 'static,
    ) -> Argument<O> {
        mapped_argument::map_argument(self.clone(), mapper)
    }
    pub fn map_for_sender<O: Clone + Send + Sync + 'static>(
        &self,
        mapper: impl for<'sender> Fn(&CommandSender<'sender>, T) -> Result<O, ArgumentError>
        + Send
        + Sync
        + 'static,
    ) -> Argument<O> {
        mapped_argument::map_argument_for_sender(self.clone(), mapper)
    }
    pub fn filter(&self, predicate: impl Fn(&T) -> bool + Send + Sync + 'static) -> Self {
        mapped_argument::filter_argument(self.clone(), predicate)
    }
    fn read_state(&self) -> std::sync::RwLockReadGuard<'_, ArgumentState<T>> {
        match self.state.read() {
            Ok(state) => state,
            Err(error) => error.into_inner(),
        }
    }
    fn write_state(&self) -> std::sync::RwLockWriteGuard<'_, ArgumentState<T>> {
        match self.state.write() {
            Ok(state) => state,
            Err(error) => error.into_inner(),
        }
    }
}

impl<T: Clone + Send + Sync + 'static> AnyArgument for Argument<T> {
    fn get_id(&self) -> &str {
        self.get_id()
    }
    fn get_parser(&self) -> spinel_core::network::clientbound::play::commands::ArgumentParserType {
        self.get_parser()
    }
    fn get_node_properties(&self) -> Option<Vec<u8>> {
        self.get_node_properties()
    }
    fn parse_erased(
        &self,
        sender: &CommandSender<'_>,
        input: &str,
    ) -> Result<AnyArgumentValue, ArgumentError> {
        self.parse_input(sender, input).map(AnyArgumentValue::new)
    }
}

impl<T: Clone + Send + Sync + 'static, U: Clone + Send + Sync + 'static> PartialEq<Argument<U>>
    for Argument<T>
{
    fn eq(&self, other: &Argument<U>) -> bool {
        self.behavior.get_concrete_class() == other.behavior.get_concrete_class()
            && self.get_id() == other.get_id()
    }
}
impl<T: Clone + Send + Sync + 'static> Eq for Argument<T> {}
impl<T: Clone + Send + Sync + 'static> Hash for Argument<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.behavior.get_concrete_class().hash(state);
        self.get_id().hash(state);
    }
}
