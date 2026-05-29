use crate::command::CommandArgumentValue;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CommandData {
    values: HashMap<String, CommandArgumentValue>,
}

impl CommandData {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn set(mut self, key: impl Into<String>, value: CommandArgumentValue) -> Self {
        self.values.insert(key.into(), value);
        self
    }

    pub fn get(&self, key: &str) -> Option<&CommandArgumentValue> {
        self.values.get(key)
    }

    pub fn has(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }

    pub const fn values(&self) -> &HashMap<String, CommandArgumentValue> {
        &self.values
    }
}
