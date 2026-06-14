use crate::entity::game_mode::GameMode;
use spinel_network::VarIntWrapper;
use spinel_network::data_type::DataType;
use spinel_network::encoder::NetworkBuffer;
use spinel_network::types::Array;
use spinel_network::wrappers::JsonTextComponent;
use spinel_network::{ConnectionState, PacketSender, PacketStruct};
use spinel_utils::component::text::TextComponent;
use std::io::{self, Read, Write};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PlayerInfoUpdatePacket {
    pub actions: PlayerInfoActions,
    pub entries: Array<PlayerInfoEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerInfoActions(pub u8);

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerInfoEntry {
    pub uuid: Uuid,
    pub username: String,
    pub properties: Vec<PlayerInfoProperty>,
    pub listed: bool,
    pub latency: i32,
    pub game_mode: GameMode,
    pub display_name: Option<TextComponent>,
    pub list_order: i32,
    pub display_hat: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerInfoProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

impl PlayerInfoActions {
    pub const ADD_PLAYER: u8 = 1 << 0;
    pub const INITIALIZE_CHAT: u8 = 1 << 1;
    pub const UPDATE_GAME_MODE: u8 = 1 << 2;
    pub const UPDATE_LISTED: u8 = 1 << 3;
    pub const UPDATE_LATENCY: u8 = 1 << 4;
    pub const UPDATE_DISPLAY_NAME: u8 = 1 << 5;
    pub const UPDATE_LIST_ORDER: u8 = 1 << 6;
    pub const UPDATE_HAT: u8 = 1 << 7;

    pub const fn add_player_and_update_listed() -> Self {
        Self(Self::ADD_PLAYER | Self::UPDATE_LISTED)
    }

    pub const fn update_game_mode() -> Self {
        Self(Self::UPDATE_GAME_MODE)
    }

    pub const fn update_listed() -> Self {
        Self(Self::UPDATE_LISTED)
    }

    pub const fn update_latency() -> Self {
        Self(Self::UPDATE_LATENCY)
    }

    pub const fn update_display_name() -> Self {
        Self(Self::UPDATE_DISPLAY_NAME)
    }

    pub const fn update_list_order() -> Self {
        Self(Self::UPDATE_LIST_ORDER)
    }

    pub const fn update_hat() -> Self {
        Self(Self::UPDATE_HAT)
    }

    pub const fn contains(self, action: u8) -> bool {
        self.0 & action != 0
    }
}

impl PlayerInfoUpdatePacket {
    pub const fn get_id() -> i32 {
        0x44
    }

    pub const fn get_id_const() -> i32 {
        0x44
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Play
    }

    pub fn encode_to_buffer(&self) -> io::Result<NetworkBuffer> {
        let mut buffer = NetworkBuffer::new();
        self.encode(&mut buffer)?;
        Ok(buffer)
    }

    pub fn dispatch<S: PacketSender>(self, sender: &mut S) -> io::Result<()> {
        let payload_bytes = self.encode_to_buffer()?.into_buffer();
        sender.send_packet(Self::get_id(), &payload_bytes)
    }

    pub fn add_listed_player(uuid: Uuid, username: impl Into<String>) -> Self {
        Self::add_player(uuid, username, true)
    }

    pub fn add_player(uuid: Uuid, username: impl Into<String>, listed: bool) -> Self {
        Self {
            actions: PlayerInfoActions::add_player_and_update_listed(),
            entries: Array(vec![PlayerInfoEntry {
                uuid,
                username: username.into(),
                properties: Vec::new(),
                listed,
                latency: 0,
                game_mode: GameMode::Survival,
                display_name: None,
                list_order: 0,
                display_hat: true,
            }]),
        }
    }

    pub fn add_player_with_properties(
        uuid: Uuid,
        username: impl Into<String>,
        listed: bool,
        properties: Vec<PlayerInfoProperty>,
    ) -> Self {
        let mut packet = Self::add_player(uuid, username, listed);
        packet.entries.0[0].properties = properties;
        packet
    }

    pub fn update_game_mode(uuid: Uuid, game_mode: GameMode) -> Self {
        Self {
            actions: PlayerInfoActions::update_game_mode(),
            entries: Array(vec![PlayerInfoEntry {
                uuid,
                username: String::new(),
                properties: Vec::new(),
                listed: false,
                latency: 0,
                game_mode,
                display_name: None,
                list_order: 0,
                display_hat: true,
            }]),
        }
    }

    pub fn update_listed(uuid: Uuid, listed: bool) -> Self {
        Self {
            actions: PlayerInfoActions::update_listed(),
            entries: Array(vec![PlayerInfoEntry {
                uuid,
                username: String::new(),
                properties: Vec::new(),
                listed,
                latency: 0,
                game_mode: GameMode::Survival,
                display_name: None,
                list_order: 0,
                display_hat: true,
            }]),
        }
    }

    pub fn update_latency(uuid: Uuid, latency: i32) -> Self {
        Self {
            actions: PlayerInfoActions::update_latency(),
            entries: Array(vec![PlayerInfoEntry {
                uuid,
                username: String::new(),
                properties: Vec::new(),
                listed: false,
                latency,
                game_mode: GameMode::Survival,
                display_name: None,
                list_order: 0,
                display_hat: true,
            }]),
        }
    }

    pub fn update_display_name(uuid: Uuid, display_name: Option<TextComponent>) -> Self {
        Self {
            actions: PlayerInfoActions::update_display_name(),
            entries: Array(vec![PlayerInfoEntry {
                uuid,
                username: String::new(),
                properties: Vec::new(),
                listed: false,
                latency: 0,
                game_mode: GameMode::Survival,
                display_name,
                list_order: 0,
                display_hat: true,
            }]),
        }
    }

    pub fn update_list_order(uuid: Uuid, list_order: i32) -> Self {
        Self {
            actions: PlayerInfoActions::update_list_order(),
            entries: Array(vec![PlayerInfoEntry {
                uuid,
                username: String::new(),
                properties: Vec::new(),
                listed: false,
                latency: 0,
                game_mode: GameMode::Survival,
                display_name: None,
                list_order,
                display_hat: true,
            }]),
        }
    }

    pub fn update_hat(uuid: Uuid, display_hat: bool) -> Self {
        Self {
            actions: PlayerInfoActions::update_hat(),
            entries: Array(vec![PlayerInfoEntry {
                uuid,
                username: String::new(),
                properties: Vec::new(),
                listed: false,
                latency: 0,
                game_mode: GameMode::Survival,
                display_name: None,
                list_order: 0,
                display_hat,
            }]),
        }
    }
}

impl DataType for PlayerInfoActions {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.0.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self(u8::decode(reader)?))
    }
}

impl DataType for PlayerInfoEntry {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.encode_with_actions(writer, PlayerInfoActions::add_player_and_update_listed())
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Self::decode_with_actions(reader, PlayerInfoActions::add_player_and_update_listed())
    }
}

impl PlayerInfoEntry {
    fn encode_with_actions<W: Write>(
        &self,
        writer: &mut W,
        actions: PlayerInfoActions,
    ) -> io::Result<()> {
        self.uuid.encode(writer)?;
        if actions.contains(PlayerInfoActions::ADD_PLAYER) {
            self.username.encode(writer)?;
            Array(self.properties.clone()).encode(writer)?;
        }
        if actions.contains(PlayerInfoActions::INITIALIZE_CHAT) {
            false.encode(writer)?;
        }
        if actions.contains(PlayerInfoActions::UPDATE_GAME_MODE) {
            VarIntWrapper(self.game_mode.id() as i32).encode(writer)?;
        }
        if actions.contains(PlayerInfoActions::UPDATE_LISTED) {
            self.listed.encode(writer)?;
        }
        if actions.contains(PlayerInfoActions::UPDATE_LATENCY) {
            VarIntWrapper(self.latency).encode(writer)?;
        }
        if actions.contains(PlayerInfoActions::UPDATE_DISPLAY_NAME) {
            self.display_name
                .clone()
                .map(JsonTextComponent)
                .encode(writer)?;
        }
        if actions.contains(PlayerInfoActions::UPDATE_LIST_ORDER) {
            VarIntWrapper(self.list_order).encode(writer)?;
        }
        if actions.contains(PlayerInfoActions::UPDATE_HAT) {
            self.display_hat.encode(writer)?;
        }
        Ok(())
    }

    fn decode_with_actions<R: Read>(
        reader: &mut R,
        actions: PlayerInfoActions,
    ) -> io::Result<Self> {
        let uuid = Uuid::decode(reader)?;
        let username = match actions.contains(PlayerInfoActions::ADD_PLAYER) {
            true => String::decode(reader)?,
            false => String::new(),
        };
        let properties = match actions.contains(PlayerInfoActions::ADD_PLAYER) {
            true => Array::<PlayerInfoProperty>::decode(reader)?.0,
            false => Vec::new(),
        };
        if actions.contains(PlayerInfoActions::INITIALIZE_CHAT) {
            let _chat_session_is_present = bool::decode(reader)?;
        }
        let game_mode = match actions.contains(PlayerInfoActions::UPDATE_GAME_MODE) {
            true => {
                let game_mode_id = VarIntWrapper::decode(reader)?.0 as u8;
                GameMode::from_id(game_mode_id).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Invalid game mode id")
                })?
            }
            false => GameMode::Survival,
        };
        let listed = match actions.contains(PlayerInfoActions::UPDATE_LISTED) {
            true => bool::decode(reader)?,
            false => false,
        };
        let latency = match actions.contains(PlayerInfoActions::UPDATE_LATENCY) {
            true => VarIntWrapper::decode(reader)?.0,
            false => 0,
        };
        let display_name = match actions.contains(PlayerInfoActions::UPDATE_DISPLAY_NAME) {
            true => Option::<JsonTextComponent>::decode(reader)?.map(|wrapper| wrapper.0),
            false => None,
        };
        let list_order = match actions.contains(PlayerInfoActions::UPDATE_LIST_ORDER) {
            true => VarIntWrapper::decode(reader)?.0,
            false => 0,
        };
        let display_hat = match actions.contains(PlayerInfoActions::UPDATE_HAT) {
            true => bool::decode(reader)?,
            false => true,
        };
        Ok(Self {
            uuid,
            username,
            properties,
            listed,
            latency,
            game_mode,
            display_name,
            list_order,
            display_hat,
        })
    }
}

impl DataType for PlayerInfoProperty {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.name.encode(writer)?;
        self.value.encode(writer)?;
        self.signature.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            name: String::decode(reader)?,
            value: String::decode(reader)?,
            signature: Option::<String>::decode(reader)?,
        })
    }
}

impl DataType for PlayerInfoUpdatePacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.actions.encode(writer)?;
        VarIntWrapper(self.entries.0.len() as i32).encode(writer)?;
        self.entries
            .0
            .iter()
            .try_for_each(|entry| entry.encode_with_actions(writer, self.actions))
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let actions = PlayerInfoActions::decode(reader)?;
        let entry_count = VarIntWrapper::decode(reader)?.0 as usize;
        let entries = (0..entry_count)
            .map(|_| PlayerInfoEntry::decode_with_actions(reader, actions))
            .collect::<io::Result<Vec<_>>>()?;
        Ok(Self {
            actions,
            entries: Array(entries),
        })
    }
}

impl PacketStruct for PlayerInfoUpdatePacket {
    fn get_id() -> i32 {
        Self::get_id()
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}
spinel_network::register_packet_codec!(PlayerInfoUpdatePacket, spinel_network::Recipient::Client);
