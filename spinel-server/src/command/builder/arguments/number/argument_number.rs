use crate::command::{Argument, ArgumentBehavior, ArgumentClass, ArgumentError, CommandSender};
use spinel_core::network::clientbound::play::commands::ArgumentParserType;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock};

pub struct ArgumentNumber<T> {
    argument: Argument<T>,
    state: Arc<RwLock<NumberState<T>>>,
}
struct NumberState<T> {
    min: Option<T>,
    max: Option<T>,
}
struct NumberBehavior<T> {
    parser: ArgumentParserType,
    state: Arc<RwLock<NumberState<T>>>,
    parse_decimal: fn(&str) -> Option<T>,
    parse_radix: fn(&str, u32) -> Option<T>,
    encode: fn(T) -> Vec<u8>,
    class: ArgumentClass,
}
impl<T: Copy + Ord + Send + Sync + 'static> ArgumentNumber<T> {
    pub const NOT_NUMBER_ERROR: i32 = 1;
    pub const TOO_LOW_ERROR: i32 = 2;
    pub const TOO_HIGH_ERROR: i32 = 3;
    pub(crate) fn new(
        id: impl Into<String>,
        parser: ArgumentParserType,
        parse_decimal: fn(&str) -> Option<T>,
        parse_radix: fn(&str, u32) -> Option<T>,
        encode: fn(T) -> Vec<u8>,
        class: ArgumentClass,
    ) -> Self {
        let state = Arc::new(RwLock::new(NumberState {
            min: None,
            max: None,
        }));
        let behavior = NumberBehavior {
            parser,
            state: Arc::clone(&state),
            parse_decimal,
            parse_radix,
            encode,
            class,
        };
        Self {
            argument: Argument::custom(id, behavior),
            state,
        }
    }
    pub fn min(&mut self, value: T) -> &mut Self {
        self.write_state().min = Some(value);
        self
    }
    pub fn max(&mut self, value: T) -> &mut Self {
        self.write_state().max = Some(value);
        self
    }
    pub fn between(&mut self, min: T, max: T) -> &mut Self {
        {
            let mut state = self.write_state();
            state.min = Some(min);
            state.max = Some(max);
        }
        self
    }
    pub fn get_number_properties(&self) -> u8 {
        u8::from(self.has_min()) | (u8::from(self.has_max()) << 1)
    }
    pub fn has_min(&self) -> bool {
        self.read_state().min.is_some()
    }
    pub fn get_min(&self) -> Option<T> {
        self.read_state().min
    }
    pub fn has_max(&self) -> bool {
        self.read_state().max.is_some()
    }
    pub fn get_max(&self) -> Option<T> {
        self.read_state().max
    }
    pub(crate) fn into_argument(self) -> Argument<T> {
        self.argument
    }
    fn read_state(&self) -> std::sync::RwLockReadGuard<'_, NumberState<T>> {
        match self.state.read() {
            Ok(state) => state,
            Err(error) => error.into_inner(),
        }
    }
    fn write_state(&self) -> std::sync::RwLockWriteGuard<'_, NumberState<T>> {
        match self.state.write() {
            Ok(state) => state,
            Err(error) => error.into_inner(),
        }
    }
}
impl<T> Deref for ArgumentNumber<T> {
    type Target = Argument<T>;
    fn deref(&self) -> &Self::Target {
        &self.argument
    }
}
impl<T> DerefMut for ArgumentNumber<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.argument
    }
}
impl<T: Copy + Ord + Send + Sync + 'static> ArgumentBehavior<T> for NumberBehavior<T> {
    fn parse_input(&self, _sender: &CommandSender<'_>, input: &str) -> Result<T, ArgumentError> {
        let (value, radix) = if let Some(value) = input.strip_prefix("0b") {
            (value, 2)
        } else if let Some(value) = input.strip_prefix("0x") {
            (value, 16)
        } else {
            (input, 10)
        };
        let value = if radix == 10 {
            (self.parse_decimal)(scientific_value(value).as_deref().unwrap_or(value))
        } else {
            (self.parse_radix)(value, radix)
        }
        .ok_or_else(|| {
            ArgumentError::new(
                "Input is not a number, or it's invalid for the given type",
                input,
                1,
            )
        })?;
        let state = match self.state.read() {
            Ok(state) => state,
            Err(error) => error.into_inner(),
        };
        if state.min.is_some_and(|min| value < min) {
            return Err(ArgumentError::new(
                "Input is lower than the minimum allowed value",
                input,
                2,
            ));
        }
        if state.max.is_some_and(|max| value > max) {
            return Err(ArgumentError::new(
                "Input is higher than the maximum allowed value",
                input,
                3,
            ));
        }
        Ok(value)
    }
    fn get_parser(&self) -> ArgumentParserType {
        self.parser
    }
    fn get_node_properties(&self) -> Option<Vec<u8>> {
        let state = match self.state.read() {
            Ok(state) => state,
            Err(error) => error.into_inner(),
        };
        let mut properties =
            vec![u8::from(state.min.is_some()) | (u8::from(state.max.is_some()) << 1)];
        state
            .min
            .map(self.encode)
            .into_iter()
            .for_each(|bytes| properties.extend(bytes));
        state
            .max
            .map(self.encode)
            .into_iter()
            .for_each(|bytes| properties.extend(bytes));
        Some(properties)
    }
    fn get_concrete_class(&self) -> ArgumentClass {
        self.class.clone()
    }
}
fn scientific_value(input: &str) -> Option<String> {
    if input.contains(['e', 'E']) {
        let value = input.parse::<f64>().ok()?;
        if value.fract() != 0.0 {
            return Some(input.to_string());
        }
        Some(format!("{value:.0}"))
    } else {
        None
    }
}
