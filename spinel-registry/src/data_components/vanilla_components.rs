use crate::data_components::DataComponentType;
use crate::{Identifier, Todo};

pub const CUSTOM_DATA: DataComponentType<Todo> =
    DataComponentType::new(Identifier::vanilla_static("custom_data"));
pub const MAX_STACK_SIZE: DataComponentType<i32> =
    DataComponentType::new(Identifier::vanilla_static("max_stack_size"));
pub const MAX_DAMAGE: DataComponentType<i32> =
    DataComponentType::new(Identifier::vanilla_static("max_damage"));
