use crate::command::{ArgumentCallback, SuggestionCallback, SuggestionType};
use spinel_core::network::clientbound::play::commands::ArgumentParserType;
use spinel_nbt::NbtCompound;
use spinel_network::data_type::DataType;
use spinel_registry::{BlockState, EntityType};

#[derive(Clone)]
pub struct CommandArgument {
    id: String,
    kind: CommandArgumentKind,
    default_value: Option<CommandArgumentValue>,
    callback: Option<ArgumentCallback>,
    suggestion_callback: Option<SuggestionCallback>,
    suggestion_type: Option<SuggestionType>,
    registry_identifier: Option<String>,
    entity_selector_flags: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandArgumentKind {
    Literal,
    EntityType,
    RelativeVec3,
    RelativeBlockPosition,
    BlockState,
    NbtCompound,
    Parser {
        parser: ArgumentParserType,
        syntax_name: &'static str,
        allows_space: bool,
        uses_remaining_input: bool,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum CommandArgumentValue {
    EntityType(EntityType),
    RelativeVec3(RelativeVec3),
    BlockState(BlockState),
    NbtCompound(NbtCompound),
    String(String),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RelativeVec3 {
    x: RelativeCoordinate,
    y: RelativeCoordinate,
    z: RelativeCoordinate,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RelativeCoordinate {
    value: f64,
    coordinate_type: CoordinateType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoordinateType {
    Absolute,
    Relative,
    Local,
}

impl CommandArgument {
    pub fn literal(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            kind: CommandArgumentKind::Literal,
            default_value: None,
            callback: None,
            suggestion_callback: None,
            suggestion_type: None,
            registry_identifier: None,
            entity_selector_flags: 0,
        }
    }

    pub fn entity_type(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            kind: CommandArgumentKind::EntityType,
            default_value: None,
            callback: None,
            suggestion_callback: None,
            suggestion_type: Some(SuggestionType::SummonableEntities),
            registry_identifier: None,
            entity_selector_flags: 0,
        }
    }

    pub fn custom_parser(
        id: impl Into<String>,
        parser: ArgumentParserType,
        syntax_name: &'static str,
    ) -> Self {
        Self {
            id: id.into(),
            kind: CommandArgumentKind::Parser {
                parser,
                syntax_name,
                allows_space: false,
                uses_remaining_input: false,
            },
            default_value: None,
            callback: None,
            suggestion_callback: None,
            suggestion_type: None,
            registry_identifier: None,
            entity_selector_flags: 0,
        }
    }

    pub fn parser_with_space(
        id: impl Into<String>,
        parser: ArgumentParserType,
        syntax_name: &'static str,
    ) -> Self {
        Self {
            id: id.into(),
            kind: CommandArgumentKind::Parser {
                parser,
                syntax_name,
                allows_space: true,
                uses_remaining_input: false,
            },
            default_value: None,
            callback: None,
            suggestion_callback: None,
            suggestion_type: None,
            registry_identifier: None,
            entity_selector_flags: 0,
        }
    }

    pub fn parser_with_remaining(
        id: impl Into<String>,
        parser: ArgumentParserType,
        syntax_name: &'static str,
    ) -> Self {
        Self {
            id: id.into(),
            kind: CommandArgumentKind::Parser {
                parser,
                syntax_name,
                allows_space: true,
                uses_remaining_input: true,
            },
            default_value: None,
            callback: None,
            suggestion_callback: None,
            suggestion_type: None,
            registry_identifier: None,
            entity_selector_flags: 0,
        }
    }

    pub fn custom_registry_parser(
        id: impl Into<String>,
        parser: ArgumentParserType,
        syntax_name: &'static str,
        registry_identifier: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            kind: CommandArgumentKind::Parser {
                parser,
                syntax_name,
                allows_space: false,
                uses_remaining_input: false,
            },
            default_value: None,
            callback: None,
            suggestion_callback: None,
            suggestion_type: None,
            registry_identifier: Some(registry_identifier.into()),
            entity_selector_flags: 0,
        }
    }

    pub fn relative_vec3(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            kind: CommandArgumentKind::RelativeVec3,
            default_value: None,
            callback: None,
            suggestion_callback: None,
            suggestion_type: None,
            registry_identifier: None,
            entity_selector_flags: 0,
        }
    }

    pub fn relative_block_position(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            kind: CommandArgumentKind::RelativeBlockPosition,
            default_value: None,
            callback: None,
            suggestion_callback: None,
            suggestion_type: None,
            registry_identifier: None,
            entity_selector_flags: 0,
        }
    }

    pub fn block_state(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            kind: CommandArgumentKind::BlockState,
            default_value: None,
            callback: None,
            suggestion_callback: None,
            suggestion_type: None,
            registry_identifier: None,
            entity_selector_flags: 0,
        }
    }

    pub fn nbt_compound(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            kind: CommandArgumentKind::NbtCompound,
            default_value: None,
            callback: None,
            suggestion_callback: None,
            suggestion_type: None,
            registry_identifier: None,
            entity_selector_flags: 0,
        }
    }

    pub fn with_default_value(mut self, default_value: CommandArgumentValue) -> Self {
        self.default_value = Some(default_value);
        self
    }

    pub fn set_default_value(&mut self, default_value: Option<CommandArgumentValue>) -> &mut Self {
        self.default_value = default_value;
        self
    }

    pub fn set_callback(&mut self, callback: Option<ArgumentCallback>) -> &mut Self {
        self.callback = callback;
        self
    }

    pub fn with_callback(mut self, callback: ArgumentCallback) -> Self {
        self.callback = Some(callback);
        self
    }

    pub fn callback(&self) -> Option<ArgumentCallback> {
        self.callback
    }

    pub fn has_error_callback(&self) -> bool {
        self.callback.is_some()
    }

    pub fn set_suggestion_callback(&mut self, callback: SuggestionCallback) -> &mut Self {
        self.suggestion_callback = Some(callback);
        self.suggestion_type = Some(SuggestionType::AskServer);
        self
    }

    pub fn set_entity_selector_flags(&mut self, flags: u8) -> &mut Self {
        self.entity_selector_flags = flags;
        self
    }

    pub fn suggestion_callback(&self) -> Option<SuggestionCallback> {
        self.suggestion_callback
    }

    pub fn has_suggestion(&self) -> bool {
        self.suggestion_type.is_some()
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub const fn kind(&self) -> CommandArgumentKind {
        self.kind
    }

    pub fn default_value(&self) -> Option<CommandArgumentValue> {
        self.default_value.clone()
    }

    pub const fn is_optional(&self) -> bool {
        self.default_value.is_some()
    }

    pub const fn allows_space(&self) -> bool {
        match self.kind {
            CommandArgumentKind::RelativeVec3
            | CommandArgumentKind::RelativeBlockPosition
            | CommandArgumentKind::BlockState
            | CommandArgumentKind::NbtCompound => true,
            CommandArgumentKind::Literal | CommandArgumentKind::EntityType => false,
            CommandArgumentKind::Parser { allows_space, .. } => allows_space,
        }
    }

    pub const fn uses_remaining_input(&self) -> bool {
        match self.kind {
            CommandArgumentKind::NbtCompound => true,
            CommandArgumentKind::Literal
            | CommandArgumentKind::EntityType
            | CommandArgumentKind::RelativeVec3
            | CommandArgumentKind::RelativeBlockPosition
            | CommandArgumentKind::BlockState => false,
            CommandArgumentKind::Parser {
                uses_remaining_input,
                ..
            } => uses_remaining_input,
        }
    }

    pub const fn parser(&self) -> ArgumentParserType {
        match self.kind {
            CommandArgumentKind::Literal => ArgumentParserType::String,
            CommandArgumentKind::EntityType => ArgumentParserType::ResourceLocation,
            CommandArgumentKind::RelativeVec3 => ArgumentParserType::Vec3,
            CommandArgumentKind::RelativeBlockPosition => ArgumentParserType::BlockPos,
            CommandArgumentKind::BlockState => ArgumentParserType::BlockState,
            CommandArgumentKind::NbtCompound => ArgumentParserType::NbtCompoundTag,
            CommandArgumentKind::Parser { parser, .. } => parser,
        }
    }

    pub fn suggestions_type(&self) -> Option<String> {
        self.suggestion_type
            .map(SuggestionType::identifier)
            .map(str::to_string)
    }

    pub fn protocol_properties(&self) -> Vec<u8> {
        match self.kind {
            CommandArgumentKind::Parser {
                parser: ArgumentParserType::String,
                allows_space,
                uses_remaining_input,
                ..
            } => vec![string_parser_mode(allows_space, uses_remaining_input)],
            CommandArgumentKind::Parser {
                parser:
                    ArgumentParserType::Float
                    | ArgumentParserType::Double
                    | ArgumentParserType::Integer
                    | ArgumentParserType::Long,
                ..
            } => vec![0],
            CommandArgumentKind::Parser {
                parser: ArgumentParserType::Entity,
                ..
            } => vec![self.entity_selector_flags],
            CommandArgumentKind::Parser {
                parser: ArgumentParserType::Time,
                ..
            } => 0i32.to_be_bytes().to_vec(),
            CommandArgumentKind::Parser {
                parser:
                    ArgumentParserType::ResourceOrTag
                    | ArgumentParserType::ResourceOrTagKey
                    | ArgumentParserType::Resource
                    | ArgumentParserType::ResourceKey,
                ..
            } => self.registry_identifier_properties(),
            _ => Vec::new(),
        }
    }

    fn registry_identifier_properties(&self) -> Vec<u8> {
        match &self.registry_identifier {
            Some(registry_identifier) => encoded_protocol_string(registry_identifier),
            None => Vec::new(),
        }
    }

    pub fn syntax_part(&self) -> String {
        match self.kind {
            CommandArgumentKind::Literal => format!("Literal<{}>", self.id),
            CommandArgumentKind::EntityType => format!("EntityType<{}>", self.id),
            CommandArgumentKind::RelativeVec3 => format!("RelativeVec3<{}>", self.id),
            CommandArgumentKind::RelativeBlockPosition => {
                format!("RelativeBlockPosition<{}>", self.id)
            }
            CommandArgumentKind::BlockState => format!("BlockState<{}>", self.id),
            CommandArgumentKind::NbtCompound => format!("NbtCompound<{}>", self.id),
            CommandArgumentKind::Parser { syntax_name, .. } => {
                format!("{syntax_name}<{}>", self.id)
            }
        }
    }
}

impl RelativeVec3 {
    pub const fn new(x: RelativeCoordinate, y: RelativeCoordinate, z: RelativeCoordinate) -> Self {
        Self { x, y, z }
    }

    pub const fn relative_origin() -> Self {
        Self {
            x: RelativeCoordinate::relative(0.0),
            y: RelativeCoordinate::relative(0.0),
            z: RelativeCoordinate::relative(0.0),
        }
    }

    pub fn resolve(self, origin_x: f64, origin_y: f64, origin_z: f64) -> (f64, f64, f64) {
        (
            self.x.resolve(origin_x),
            self.y.resolve(origin_y),
            self.z.resolve(origin_z),
        )
    }
}

impl RelativeCoordinate {
    pub const fn absolute(value: f64) -> Self {
        Self {
            value,
            coordinate_type: CoordinateType::Absolute,
        }
    }

    pub const fn relative(value: f64) -> Self {
        Self {
            value,
            coordinate_type: CoordinateType::Relative,
        }
    }

    pub const fn local(value: f64) -> Self {
        Self {
            value,
            coordinate_type: CoordinateType::Local,
        }
    }

    pub const fn coordinate_type(self) -> CoordinateType {
        self.coordinate_type
    }

    pub fn resolve(self, origin: f64) -> f64 {
        match self.coordinate_type {
            CoordinateType::Absolute => self.value,
            CoordinateType::Relative | CoordinateType::Local => origin + self.value,
        }
    }
}

impl From<RelativeVec3> for CommandArgumentValue {
    fn from(value: RelativeVec3) -> Self {
        Self::RelativeVec3(value)
    }
}
fn encoded_protocol_string(value: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    match value.to_string().encode(&mut bytes) {
        Ok(()) => bytes,
        Err(_) => Vec::new(),
    }
}

fn string_parser_mode(allows_space: bool, uses_remaining_input: bool) -> u8 {
    if uses_remaining_input {
        return 2;
    }
    if allows_space {
        return 1;
    }
    0
}
