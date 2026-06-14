use spinel_network::data_type::DataType;
use spinel_network::types::var_int::VarIntWrapper;
use spinel_network::{ConnectionState, PacketSender, PacketStruct};
use std::io::{self, Read, Write};

pub const COMMAND_NODE_TYPE_MASK: u8 = 0x03;
pub const COMMAND_NODE_IS_EXECUTABLE: u8 = 0x04;
pub const COMMAND_NODE_HAS_REDIRECT: u8 = 0x08;
pub const COMMAND_NODE_HAS_SUGGESTION_TYPE: u8 = 0x10;

pub struct CommandsPacket {
    pub nodes: Vec<CommandNode>,
    pub root_index: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandNode {
    pub flags: u8,
    pub children: Vec<i32>,
    pub redirected_node: Option<i32>,
    pub name: Option<String>,
    pub parser: Option<ArgumentParserType>,
    pub properties: Vec<u8>,
    pub suggestions_type: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandNodeType {
    Root,
    Literal,
    Argument,
}

impl CommandNode {
    pub fn root(children: Vec<i32>) -> Self {
        Self {
            flags: CommandNodeType::Root.flags(false),
            children,
            redirected_node: None,
            name: None,
            parser: None,
            properties: Vec::new(),
            suggestions_type: None,
        }
    }

    pub fn literal(name: impl Into<String>, children: Vec<i32>, is_executable: bool) -> Self {
        Self {
            flags: CommandNodeType::Literal.flags(is_executable),
            children,
            redirected_node: None,
            name: Some(name.into()),
            parser: None,
            properties: Vec::new(),
            suggestions_type: None,
        }
    }

    pub fn argument(
        name: impl Into<String>,
        parser: ArgumentParserType,
        children: Vec<i32>,
        is_executable: bool,
        suggestions_type: Option<String>,
    ) -> Self {
        let mut flags = CommandNodeType::Argument.flags(is_executable);
        if suggestions_type.is_some() {
            flags |= COMMAND_NODE_HAS_SUGGESTION_TYPE;
        }
        Self {
            flags,
            children,
            redirected_node: None,
            name: Some(name.into()),
            parser: Some(parser),
            properties: Vec::new(),
            suggestions_type,
        }
    }

    fn node_type(&self) -> Option<CommandNodeType> {
        match self.flags & COMMAND_NODE_TYPE_MASK {
            0 => Some(CommandNodeType::Root),
            1 => Some(CommandNodeType::Literal),
            2 => Some(CommandNodeType::Argument),
            _ => None,
        }
    }
}

impl CommandNodeType {
    const fn flags(self, is_executable: bool) -> u8 {
        let base_flags = match self {
            Self::Root => 0,
            Self::Literal => 1,
            Self::Argument => 2,
        };
        if is_executable {
            base_flags | COMMAND_NODE_IS_EXECUTABLE
        } else {
            base_flags
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArgumentParserType {
    Bool,
    Float,
    Double,
    Integer,
    Long,
    String,
    Entity,
    GameProfile,
    BlockPos,
    ColumnPos,
    Vec3,
    Vec2,
    BlockState,
    BlockPredicate,
    ItemStack,
    ItemPredicate,
    Color,
    HexColor,
    Component,
    Style,
    Message,
    NbtCompoundTag,
    NbtTag,
    NbtPath,
    Objective,
    ObjectiveCriteria,
    Operation,
    Particle,
    Angle,
    Rotation,
    ScoreboardSlot,
    ScoreHolder,
    Swizzle,
    Team,
    ItemSlot,
    ItemSlots,
    ResourceLocation,
    Function,
    EntityAnchor,
    IntRange,
    FloatRange,
    Dimension,
    GameMode,
    Time,
    ResourceOrTag,
    ResourceOrTagKey,
    Resource,
    ResourceKey,
    ResourceSelector,
    TemplateMirror,
    TemplateRotation,
    Heightmap,
    LootTable,
    LootPredicate,
    LootModifier,
    Dialog,
    Uuid,
}

impl ArgumentParserType {
    const fn protocol_id(self) -> i32 {
        match self {
            Self::Bool => 0,
            Self::Float => 1,
            Self::Double => 2,
            Self::Integer => 3,
            Self::Long => 4,
            Self::String => 5,
            Self::Entity => 6,
            Self::GameProfile => 7,
            Self::BlockPos => 8,
            Self::ColumnPos => 9,
            Self::Vec3 => 10,
            Self::Vec2 => 11,
            Self::BlockState => 12,
            Self::BlockPredicate => 13,
            Self::ItemStack => 14,
            Self::ItemPredicate => 15,
            Self::Color => 16,
            Self::HexColor => 17,
            Self::Component => 18,
            Self::Style => 19,
            Self::Message => 20,
            Self::NbtCompoundTag => 21,
            Self::NbtTag => 22,
            Self::NbtPath => 23,
            Self::Objective => 24,
            Self::ObjectiveCriteria => 25,
            Self::Operation => 26,
            Self::Particle => 27,
            Self::Angle => 28,
            Self::Rotation => 29,
            Self::ScoreboardSlot => 30,
            Self::ScoreHolder => 31,
            Self::Swizzle => 32,
            Self::Team => 33,
            Self::ItemSlot => 34,
            Self::ItemSlots => 35,
            Self::ResourceLocation => 36,
            Self::Function => 37,
            Self::EntityAnchor => 38,
            Self::IntRange => 39,
            Self::FloatRange => 40,
            Self::Dimension => 41,
            Self::GameMode => 42,
            Self::Time => 43,
            Self::ResourceOrTag => 44,
            Self::ResourceOrTagKey => 45,
            Self::Resource => 46,
            Self::ResourceKey => 47,
            Self::ResourceSelector => 48,
            Self::TemplateMirror => 49,
            Self::TemplateRotation => 50,
            Self::Heightmap => 51,
            Self::LootTable => 52,
            Self::LootPredicate => 53,
            Self::LootModifier => 54,
            Self::Dialog => 55,
            Self::Uuid => 56,
        }
    }

    fn from_protocol_id(protocol_id: i32) -> io::Result<Self> {
        match protocol_id {
            0 => Ok(Self::Bool),
            1 => Ok(Self::Float),
            2 => Ok(Self::Double),
            3 => Ok(Self::Integer),
            4 => Ok(Self::Long),
            5 => Ok(Self::String),
            6 => Ok(Self::Entity),
            7 => Ok(Self::GameProfile),
            8 => Ok(Self::BlockPos),
            9 => Ok(Self::ColumnPos),
            10 => Ok(Self::Vec3),
            11 => Ok(Self::Vec2),
            12 => Ok(Self::BlockState),
            13 => Ok(Self::BlockPredicate),
            14 => Ok(Self::ItemStack),
            15 => Ok(Self::ItemPredicate),
            16 => Ok(Self::Color),
            17 => Ok(Self::HexColor),
            18 => Ok(Self::Component),
            19 => Ok(Self::Style),
            20 => Ok(Self::Message),
            21 => Ok(Self::NbtCompoundTag),
            22 => Ok(Self::NbtTag),
            23 => Ok(Self::NbtPath),
            24 => Ok(Self::Objective),
            25 => Ok(Self::ObjectiveCriteria),
            26 => Ok(Self::Operation),
            27 => Ok(Self::Particle),
            28 => Ok(Self::Angle),
            29 => Ok(Self::Rotation),
            30 => Ok(Self::ScoreboardSlot),
            31 => Ok(Self::ScoreHolder),
            32 => Ok(Self::Swizzle),
            33 => Ok(Self::Team),
            34 => Ok(Self::ItemSlot),
            35 => Ok(Self::ItemSlots),
            36 => Ok(Self::ResourceLocation),
            37 => Ok(Self::Function),
            38 => Ok(Self::EntityAnchor),
            39 => Ok(Self::IntRange),
            40 => Ok(Self::FloatRange),
            41 => Ok(Self::Dimension),
            42 => Ok(Self::GameMode),
            43 => Ok(Self::Time),
            44 => Ok(Self::ResourceOrTag),
            45 => Ok(Self::ResourceOrTagKey),
            46 => Ok(Self::Resource),
            47 => Ok(Self::ResourceKey),
            48 => Ok(Self::ResourceSelector),
            49 => Ok(Self::TemplateMirror),
            50 => Ok(Self::TemplateRotation),
            51 => Ok(Self::Heightmap),
            52 => Ok(Self::LootTable),
            53 => Ok(Self::LootPredicate),
            54 => Ok(Self::LootModifier),
            55 => Ok(Self::Dialog),
            56 => Ok(Self::Uuid),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unknown command argument parser type",
            )),
        }
    }
}

impl DataType for ArgumentParserType {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.protocol_id()).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Self::from_protocol_id(VarIntWrapper::decode(reader)?.0)
    }
}

impl CommandsPacket {
    pub const fn get_id() -> i32 {
        0x10
    }

    pub const fn get_id_const() -> i32 {
        0x10
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Play
    }

    pub fn encode_to_buffer(&self) -> io::Result<spinel_network::encoder::NetworkBuffer> {
        let mut buffer = spinel_network::encoder::NetworkBuffer::new();
        self.encode(&mut buffer)?;
        Ok(buffer)
    }

    pub fn dispatch<S: PacketSender>(self, sender: &mut S) -> io::Result<()> {
        let payload_bytes = self.encode_to_buffer()?.into_buffer();
        sender.send_packet(Self::get_id(), &payload_bytes)
    }
}

impl DataType for CommandsPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.nodes.encode(writer)?;
        VarIntWrapper(self.root_index).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            nodes: Vec::<CommandNode>::decode(reader)?,
            root_index: VarIntWrapper::decode(reader)?.0,
        })
    }
}

impl PacketStruct for CommandsPacket {
    fn get_id() -> i32 {
        0x10
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}

impl DataType for CommandNode {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.flags.encode(writer)?;
        VarIntWrapper(self.children.len() as i32).encode(writer)?;
        self.children
            .iter()
            .try_for_each(|child| VarIntWrapper(*child).encode(writer))?;
        if self.flags & COMMAND_NODE_HAS_REDIRECT != 0 {
            VarIntWrapper(self.redirected_node.unwrap_or_default()).encode(writer)?;
        }
        match self.node_type() {
            Some(CommandNodeType::Literal | CommandNodeType::Argument) => {
                self.name.clone().unwrap_or_default().encode(writer)?;
            }
            Some(CommandNodeType::Root) => {}
            None => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unknown command node type",
            ))?,
        };
        if self.node_type() == Some(CommandNodeType::Argument) {
            self.parser
                .unwrap_or(ArgumentParserType::String)
                .encode(writer)?;
            writer.write_all(&self.properties)?;
        }
        if self.flags & COMMAND_NODE_HAS_SUGGESTION_TYPE != 0 {
            self.suggestions_type
                .clone()
                .unwrap_or_default()
                .encode(writer)?;
        }
        Ok(())
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let flags = u8::decode(reader)?;
        let children_count = VarIntWrapper::decode(reader)?.0 as usize;
        let children = (0..children_count)
            .map(|_| VarIntWrapper::decode(reader).map(|child| child.0))
            .collect::<io::Result<Vec<_>>>()?;
        let redirected_node = if flags & COMMAND_NODE_HAS_REDIRECT != 0 {
            Some(VarIntWrapper::decode(reader)?.0)
        } else {
            None
        };
        let node_type = match flags & COMMAND_NODE_TYPE_MASK {
            0 => CommandNodeType::Root,
            1 => CommandNodeType::Literal,
            2 => CommandNodeType::Argument,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Unknown command node type",
                ));
            }
        };
        let name = match node_type {
            CommandNodeType::Root => None,
            CommandNodeType::Literal | CommandNodeType::Argument => Some(String::decode(reader)?),
        };
        let parser = match node_type {
            CommandNodeType::Argument => Some(ArgumentParserType::decode(reader)?),
            CommandNodeType::Root | CommandNodeType::Literal => None,
        };
        let suggestions_type = if flags & COMMAND_NODE_HAS_SUGGESTION_TYPE != 0 {
            Some(String::decode(reader)?)
        } else {
            None
        };
        Ok(Self {
            flags,
            children,
            redirected_node,
            name,
            parser,
            properties: Vec::new(),
            suggestions_type,
        })
    }
}
spinel_network::register_packet_codec!(CommandsPacket, spinel_network::Recipient::Client);
