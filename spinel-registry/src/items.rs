use crate::Identifier;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    pub key: Identifier,
}

impl Item {
    #[must_use]
    pub fn new(key: Identifier) -> Self {
        Self { key }
    }
}
