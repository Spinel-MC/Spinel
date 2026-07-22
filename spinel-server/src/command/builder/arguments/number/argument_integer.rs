use crate::command::{ArgumentClass, ArgumentNumber};
use spinel_core::network::clientbound::play::commands::ArgumentParserType;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

pub struct ArgumentInteger(ArgumentNumber<i32>);
impl ArgumentInteger {
    pub fn new(id: impl Into<String>) -> Self {
        Self(ArgumentNumber::new(
            id,
            ArgumentParserType::Integer,
            |input| input.parse().ok(),
            |input, radix| i32::from_str_radix(input, radix).ok(),
            |value| value.to_be_bytes().to_vec(),
            ArgumentClass::new("ArgumentInteger"),
        ))
    }
}
impl Deref for ArgumentInteger {
    type Target = ArgumentNumber<i32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ArgumentInteger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl ArgumentInteger {
    pub(crate) fn into_argument(self) -> crate::command::Argument<i32> {
        self.0.into_argument()
    }
}
impl Display for ArgumentInteger {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Integer<{}>", self.get_id())
    }
}

impl From<ArgumentInteger> for crate::command::CommandArgument {
    fn from(argument: ArgumentInteger) -> Self {
        crate::command::CommandArgument::from_integer(argument)
    }
}
