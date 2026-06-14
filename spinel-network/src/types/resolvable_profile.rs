use crate::data_type::DataType;
use crate::types::Identifier;
use crate::types::game_profile::{GameProfile, GameProfileProperty};
use crate::types::var_int::VarIntWrapper;
use std::io::{self, Read, Write};
use uuid::Uuid;

const MAXIMUM_PROFILE_PROPERTIES: usize = 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvableProfile {
    pub identity: ResolvableProfileIdentity,
    pub skin_patch: PlayerSkinPatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvableProfileIdentity {
    Complete(GameProfile),
    Partial(PartialGameProfile),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PartialGameProfile {
    pub name: Option<String>,
    pub uuid: Option<Uuid>,
    pub properties: Vec<GameProfileProperty>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PlayerSkinPatch {
    pub body: Option<Identifier>,
    pub cape: Option<Identifier>,
    pub elytra: Option<Identifier>,
    pub is_slim: Option<bool>,
}

impl Default for ResolvableProfile {
    fn default() -> Self {
        Self {
            identity: ResolvableProfileIdentity::Partial(PartialGameProfile::default()),
            skin_patch: PlayerSkinPatch::default(),
        }
    }
}

impl DataType for ResolvableProfile {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.identity.encode(writer)?;
        self.skin_patch.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            identity: ResolvableProfileIdentity::decode(reader)?,
            skin_patch: PlayerSkinPatch::decode(reader)?,
        })
    }
}

impl DataType for ResolvableProfileIdentity {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            Self::Complete(profile) => {
                true.encode(writer)?;
                profile.encode(writer)
            }
            Self::Partial(profile) => {
                false.encode(writer)?;
                profile.encode(writer)
            }
        }
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        match bool::decode(reader)? {
            true => Ok(Self::Complete(GameProfile::decode(reader)?)),
            false => Ok(Self::Partial(PartialGameProfile::decode(reader)?)),
        }
    }
}

impl DataType for PartialGameProfile {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.name.encode(writer)?;
        self.uuid.encode(writer)?;
        encode_profile_properties(&self.properties, writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            name: Option::<String>::decode(reader)?,
            uuid: Option::<Uuid>::decode(reader)?,
            properties: decode_profile_properties(reader)?,
        })
    }
}

impl DataType for PlayerSkinPatch {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.body.encode(writer)?;
        self.cape.encode(writer)?;
        self.elytra.encode(writer)?;
        self.is_slim.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            body: Option::<Identifier>::decode(reader)?,
            cape: Option::<Identifier>::decode(reader)?,
            elytra: Option::<Identifier>::decode(reader)?,
            is_slim: Option::<bool>::decode(reader)?,
        })
    }
}

fn encode_profile_properties<W: Write>(
    properties: &[GameProfileProperty],
    writer: &mut W,
) -> io::Result<()> {
    if properties.len() > MAXIMUM_PROFILE_PROPERTIES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "Profile property count exceeds {MAXIMUM_PROFILE_PROPERTIES}: {}",
                properties.len()
            ),
        ));
    }

    VarIntWrapper(properties.len() as i32).encode(writer)?;
    properties
        .iter()
        .try_for_each(|property| property.encode(writer))
}

fn decode_profile_properties<R: Read>(reader: &mut R) -> io::Result<Vec<GameProfileProperty>> {
    let property_count = VarIntWrapper::decode(reader)?.0;
    if !(0..=MAXIMUM_PROFILE_PROPERTIES as i32).contains(&property_count) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Profile property count out of bounds: {property_count}"),
        ));
    }

    (0..property_count)
        .map(|_| GameProfileProperty::decode(reader))
        .collect()
}
