use spinel_network::types::var_int::VarIntWrapper;
use spinel_network::wrappers::JsonTextComponent;
use spinel_network::{ConnectionState, DataType, PacketSender, PacketStruct};
use spinel_utils::component::text::TextComponent;
use std::io::{self, Read, Write};

#[derive(Clone, Debug, PartialEq)]
pub struct SetPlayerTeamPacket {
    pub team_name: String,
    pub action: TeamAction,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TeamAction {
    Create(TeamParameters),
    Remove,
    Update(TeamParameters),
    AddEntities(Vec<String>),
    RemoveEntities(Vec<String>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TeamParameters {
    pub display_name: TextComponent,
    pub friendly_flags: u8,
    pub name_tag_visibility: TeamNameTagVisibility,
    pub collision_rule: TeamCollisionRule,
    pub color: i32,
    pub prefix: TextComponent,
    pub suffix: TextComponent,
    pub entities: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TeamNameTagVisibility {
    Always,
    Never,
    HideForOtherTeams,
    HideForOwnTeam,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TeamCollisionRule {
    Always,
    Never,
    PushOtherTeams,
    PushOwnTeam,
}

impl SetPlayerTeamPacket {
    pub const fn get_id() -> i32 {
        0x6b
    }

    pub const fn get_id_const() -> i32 {
        Self::get_id()
    }

    pub const fn get_state_const() -> ConnectionState {
        ConnectionState::Play
    }

    pub fn dispatch<S: PacketSender>(self, sender: &mut S) -> io::Result<()> {
        let mut data = Vec::new();
        self.encode(&mut data)?;
        sender.send_packet(Self::get_id(), &data)
    }
}

impl TeamParameters {
    pub fn new(entities: Vec<String>) -> Self {
        Self {
            display_name: TextComponent::empty(),
            friendly_flags: 0,
            name_tag_visibility: TeamNameTagVisibility::Always,
            collision_rule: TeamCollisionRule::Always,
            color: 15,
            prefix: TextComponent::empty(),
            suffix: TextComponent::empty(),
            entities,
        }
    }

    fn encode_without_entities<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        JsonTextComponent(self.display_name.clone()).encode(writer)?;
        self.friendly_flags.encode(writer)?;
        self.name_tag_visibility.encode(writer)?;
        self.collision_rule.encode(writer)?;
        VarIntWrapper(self.color).encode(writer)?;
        JsonTextComponent(self.prefix.clone()).encode(writer)?;
        JsonTextComponent(self.suffix.clone()).encode(writer)
    }

    fn decode_without_entities<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            display_name: JsonTextComponent::decode(reader)?.0,
            friendly_flags: u8::decode(reader)?,
            name_tag_visibility: TeamNameTagVisibility::decode(reader)?,
            collision_rule: TeamCollisionRule::decode(reader)?,
            color: VarIntWrapper::decode(reader)?.0,
            prefix: JsonTextComponent::decode(reader)?.0,
            suffix: JsonTextComponent::decode(reader)?.0,
            entities: Vec::new(),
        })
    }
}

impl DataType for SetPlayerTeamPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.team_name.encode(writer)?;
        match &self.action {
            TeamAction::Create(parameters) => {
                0i8.encode(writer)?;
                parameters.encode_without_entities(writer)?;
                parameters.entities.encode(writer)
            }
            TeamAction::Remove => 1i8.encode(writer),
            TeamAction::Update(parameters) => {
                2i8.encode(writer)?;
                parameters.encode_without_entities(writer)
            }
            TeamAction::AddEntities(entities) => {
                3i8.encode(writer)?;
                entities.encode(writer)
            }
            TeamAction::RemoveEntities(entities) => {
                4i8.encode(writer)?;
                entities.encode(writer)
            }
        }
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let team_name = String::decode(reader)?;
        let action_id = i8::decode(reader)?;
        let action = match action_id {
            0 => {
                let mut parameters = TeamParameters::decode_without_entities(reader)?;
                parameters.entities = Vec::<String>::decode(reader)?;
                TeamAction::Create(parameters)
            }
            1 => TeamAction::Remove,
            2 => TeamAction::Update(TeamParameters::decode_without_entities(reader)?),
            3 => TeamAction::AddEntities(Vec::<String>::decode(reader)?),
            4 => TeamAction::RemoveEntities(Vec::<String>::decode(reader)?),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Unknown team action.",
                ));
            }
        };
        Ok(Self { team_name, action })
    }
}

impl DataType for TeamNameTagVisibility {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.identifier().to_owned().encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        match String::decode(reader)?.as_str() {
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            "hideForOtherTeams" => Ok(Self::HideForOtherTeams),
            "hideForOwnTeam" => Ok(Self::HideForOwnTeam),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unknown name tag visibility.",
            )),
        }
    }
}

impl TeamNameTagVisibility {
    const fn identifier(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::Never => "never",
            Self::HideForOtherTeams => "hideForOtherTeams",
            Self::HideForOwnTeam => "hideForOwnTeam",
        }
    }
}

impl DataType for TeamCollisionRule {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.identifier().to_owned().encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        match String::decode(reader)?.as_str() {
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            "pushOtherTeams" => Ok(Self::PushOtherTeams),
            "pushOwnTeam" => Ok(Self::PushOwnTeam),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unknown team collision rule.",
            )),
        }
    }
}

impl TeamCollisionRule {
    const fn identifier(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::Never => "never",
            Self::PushOtherTeams => "pushOtherTeams",
            Self::PushOwnTeam => "pushOwnTeam",
        }
    }
}

impl PacketStruct for SetPlayerTeamPacket {
    fn get_id() -> i32 {
        Self::get_id()
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}
spinel_network::register_packet_codec!(SetPlayerTeamPacket, spinel_network::Recipient::Client);
