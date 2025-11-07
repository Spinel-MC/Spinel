#[derive(Debug, Clone, Default)]
pub struct Array<T>(pub Vec<T>);
pub type Identifier = String;
pub type Optional<T> = Option<T>;
