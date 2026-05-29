use crate::command::CommandArgument;
use spinel_core::network::clientbound::play::commands::ArgumentParserType;

impl CommandArgument {
    pub fn boolean(id: impl Into<String>) -> Self {
        Self::custom_parser(id, ArgumentParserType::Bool, "Boolean")
    }

    pub fn float(id: impl Into<String>) -> Self {
        Self::custom_parser(id, ArgumentParserType::Float, "Float")
    }

    pub fn double(id: impl Into<String>) -> Self {
        Self::custom_parser(id, ArgumentParserType::Double, "Double")
    }

    pub fn integer(id: impl Into<String>) -> Self {
        Self::custom_parser(id, ArgumentParserType::Integer, "Integer")
    }

    pub fn long(id: impl Into<String>) -> Self {
        Self::custom_parser(id, ArgumentParserType::Long, "Long")
    }

    pub fn string(id: impl Into<String>) -> Self {
        Self::parser_with_space(id, ArgumentParserType::String, "String")
    }

    pub fn word(id: impl Into<String>) -> Self {
        Self::custom_parser(id, ArgumentParserType::String, "Word")
    }

    pub fn string_array(id: impl Into<String>) -> Self {
        Self::parser_with_remaining(id, ArgumentParserType::String, "StringArray")
    }

    pub fn command(id: impl Into<String>) -> Self {
        Self::parser_with_remaining(id, ArgumentParserType::String, "Command")
    }

    pub fn resource_location(id: impl Into<String>) -> Self {
        Self::custom_parser(id, ArgumentParserType::ResourceLocation, "ResourceLocation")
    }

    pub fn block_state(id: impl Into<String>) -> Self {
        Self::parser_with_space(id, ArgumentParserType::BlockState, "BlockState")
    }

    pub fn item_stack(id: impl Into<String>) -> Self {
        Self::parser_with_space(id, ArgumentParserType::ItemStack, "ItemStack")
    }

    pub fn component(id: impl Into<String>) -> Self {
        Self::parser_with_space(id, ArgumentParserType::Component, "Component")
    }

    pub fn entity(id: impl Into<String>) -> Self {
        Self::custom_parser(id, ArgumentParserType::Entity, "Entity")
    }

    pub fn uuid(id: impl Into<String>) -> Self {
        Self::custom_parser(id, ArgumentParserType::Uuid, "UUID")
    }

    pub fn color(id: impl Into<String>) -> Self {
        Self::custom_parser(id, ArgumentParserType::Color, "Color")
    }

    pub fn time(id: impl Into<String>) -> Self {
        Self::custom_parser(id, ArgumentParserType::Time, "Time")
    }

    pub fn particle(id: impl Into<String>) -> Self {
        Self::parser_with_space(id, ArgumentParserType::Particle, "Particle")
    }

    pub fn int_range(id: impl Into<String>) -> Self {
        Self::custom_parser(id, ArgumentParserType::IntRange, "IntRange")
    }

    pub fn float_range(id: impl Into<String>) -> Self {
        Self::custom_parser(id, ArgumentParserType::FloatRange, "FloatRange")
    }

    pub fn nbt(id: impl Into<String>) -> Self {
        Self::parser_with_space(id, ArgumentParserType::NbtTag, "NBT")
    }

    pub fn relative_block_position(id: impl Into<String>) -> Self {
        Self::parser_with_space(id, ArgumentParserType::BlockPos, "RelativeBlockPosition")
    }

    pub fn relative_vec2(id: impl Into<String>) -> Self {
        Self::parser_with_space(id, ArgumentParserType::Vec2, "RelativeVec2")
    }

    pub fn resource(id: impl Into<String>) -> Self {
        Self::custom_parser(id, ArgumentParserType::Resource, "Resource")
    }

    pub fn resource_or_tag(id: impl Into<String>) -> Self {
        Self::custom_parser(id, ArgumentParserType::ResourceOrTag, "ResourceOrTag")
    }
}
