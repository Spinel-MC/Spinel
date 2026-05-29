use crate::command::{CommandArgument, CommandArgumentValue, CommandData, RelativeVec3};
use spinel_nbt::NbtCompound;
use spinel_registry::EntityType;
use std::collections::HashMap;

pub struct CommandContext {
    input: String,
    command_name: String,
    arguments: HashMap<String, CommandArgumentValue>,
    raw_arguments: HashMap<String, String>,
    return_data: Option<CommandData>,
}

impl CommandContext {
    pub fn new(
        input: impl Into<String>,
        arguments: HashMap<String, CommandArgumentValue>,
        raw_arguments: HashMap<String, String>,
    ) -> Self {
        let input = input.into();
        let command_name = input
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_string();
        Self {
            input,
            command_name,
            arguments,
            raw_arguments,
            return_data: None,
        }
    }

    pub fn empty(input: impl Into<String>) -> Self {
        Self::new(input, HashMap::new(), HashMap::new())
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn command_name(&self) -> &str {
        &self.command_name
    }

    pub fn entity_type(&self, argument_id: &str) -> Option<EntityType> {
        match self.arguments.get(argument_id) {
            Some(CommandArgumentValue::EntityType(entity_type)) => Some(*entity_type),
            _ => None,
        }
    }

    pub fn argument_is_present(&self, argument_id: &str) -> bool {
        self.arguments.contains_key(argument_id)
    }

    pub fn has(&self, argument: &CommandArgument) -> bool {
        self.arguments.contains_key(argument.id())
    }

    pub fn raw(&self, argument_id: &str) -> Option<&str> {
        self.raw_arguments.get(argument_id).map(String::as_str)
    }

    pub fn relative_vec3(&self, argument_id: &str) -> Option<RelativeVec3> {
        match self.arguments.get(argument_id) {
            Some(CommandArgumentValue::RelativeVec3(position)) => Some(*position),
            _ => None,
        }
    }

    pub fn nbt_compound(&self, argument_id: &str) -> Option<&NbtCompound> {
        match self.arguments.get(argument_id) {
            Some(CommandArgumentValue::NbtCompound(compound)) => Some(compound),
            _ => None,
        }
    }

    pub const fn return_data(&self) -> Option<&CommandData> {
        self.return_data.as_ref()
    }

    pub fn set_return_data(&mut self, return_data: Option<CommandData>) {
        self.return_data = return_data;
    }
}
