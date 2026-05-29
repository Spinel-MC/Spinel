use spinel_network::DataType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl GameMode {
    pub const fn id(self) -> u8 {
        match self {
            Self::Survival => 0,
            Self::Creative => 1,
            Self::Adventure => 2,
            Self::Spectator => 3,
        }
    }

    pub const fn from_id(game_mode_id: u8) -> Option<Self> {
        match game_mode_id {
            0 => Some(Self::Survival),
            1 => Some(Self::Creative),
            2 => Some(Self::Adventure),
            3 => Some(Self::Spectator),
            _ => None,
        }
    }

    pub const fn allows_flying(self) -> bool {
        matches!(self, Self::Creative | Self::Spectator)
    }

    pub const fn has_instant_break(self) -> bool {
        matches!(self, Self::Creative)
    }

    pub const fn is_invulnerable(self) -> bool {
        matches!(self, Self::Creative | Self::Spectator)
    }
}

impl DataType for GameMode {
    fn encode<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.id().encode(writer)
    }

    fn decode<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let game_mode_id = u8::decode(reader)?;
        Self::from_id(game_mode_id).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("invalid game mode id {game_mode_id}"),
            )
        })
    }
}
