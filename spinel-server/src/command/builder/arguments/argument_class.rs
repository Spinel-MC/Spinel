use std::hash::Hash;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ArgumentClass(String);

impl ArgumentClass {
    pub fn new(identifier: impl Into<String>) -> Self {
        Self(identifier.into())
    }
}
