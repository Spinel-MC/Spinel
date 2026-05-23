use crate::Identifier;
use crate::data_components::DataComponentType;
use spinel_nbt::NbtCompound;
use spinel_utils::component::text::TextComponent;

pub const CUSTOM_DATA: DataComponentType<NbtCompound> =
    DataComponentType::new(0, Identifier::vanilla_static("custom_data"));
pub const MAX_STACK_SIZE: DataComponentType<i32> =
    DataComponentType::new(1, Identifier::vanilla_static("max_stack_size"));
pub const MAX_DAMAGE: DataComponentType<i32> =
    DataComponentType::new(2, Identifier::vanilla_static("max_damage"));
pub const DAMAGE: DataComponentType<i32> =
    DataComponentType::new(3, Identifier::vanilla_static("damage"));
pub const CUSTOM_NAME: DataComponentType<TextComponent> =
    DataComponentType::new(6, Identifier::vanilla_static("custom_name"));
pub const ITEM_MODEL: DataComponentType<String> =
    DataComponentType::new(10, Identifier::vanilla_static("item_model"));
pub const LORE: DataComponentType<Vec<TextComponent>> =
    DataComponentType::new(11, Identifier::vanilla_static("lore"));
pub const ENCHANTMENT_GLINT_OVERRIDE: DataComponentType<bool> =
    DataComponentType::new(21, Identifier::vanilla_static("enchantment_glint_override"));
