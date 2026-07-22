mod any_argument;
mod argument;
mod argument_behavior;
mod argument_class;
pub(crate) mod default_value;
pub(crate) mod mapped_argument;
pub mod number;

pub use any_argument::{AnyArgument, AnyArgumentValue};
pub use argument::Argument;
pub use argument_behavior::ArgumentBehavior;
pub use argument_class::ArgumentClass;
pub use number::{ArgumentInteger, ArgumentNumber};
