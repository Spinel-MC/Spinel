use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
use std::io::{self, Read, Write};
use uuid::Uuid;

const MAXIMUM_PROFILE_NAME_LENGTH: usize = 16;
const MAXIMUM_PROFILE_PROPERTIES: usize = 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameProfile {
    pub uuid: Uuid,
    pub username: String,
    pub properties: Vec<GameProfileProperty>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameProfileProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

impl DataType for GameProfile {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        validate_profile_name(&self.username)?;
        validate_property_count(self.properties.len())?;
        self.uuid.encode(w)?;
        self.username.encode(w)?;
        VarIntWrapper(self.properties.len() as i32).encode(w)?;
        for prop in &self.properties {
            prop.encode(w)?;
        }
        Ok(())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let uuid = Uuid::decode(r)?;
        let username = String::decode(r)?;
        validate_profile_name(&username)?;

        let count = VarIntWrapper::decode(r)?.0;
        validate_property_count(count.try_into().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Profile property count out of bounds: {count}"),
            )
        })?)?;
        let count = count as usize;
        let mut properties = Vec::with_capacity(count);
        for _ in 0..count {
            properties.push(GameProfileProperty::decode(r)?);
        }

        Ok(GameProfile {
            uuid,
            username,
            properties,
        })
    }
}

fn validate_profile_name(profile_name: &str) -> io::Result<()> {
    if profile_name.len() <= MAXIMUM_PROFILE_NAME_LENGTH {
        return Ok(());
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!(
            "Profile name length exceeds {MAXIMUM_PROFILE_NAME_LENGTH}: {}",
            profile_name.len()
        ),
    ))
}

fn validate_property_count(property_count: usize) -> io::Result<()> {
    if property_count <= MAXIMUM_PROFILE_PROPERTIES {
        return Ok(());
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!("Profile property count exceeds {MAXIMUM_PROFILE_PROPERTIES}: {property_count}"),
    ))
}

impl DataType for GameProfileProperty {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.name.encode(w)?;
        self.value.encode(w)?;
        self.signature.encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let name = String::decode(r)?;
        let value = String::decode(r)?;
        let signature = Option::<String>::decode(r)?;
        Ok(GameProfileProperty {
            name,
            value,
            signature,
        })
    }
}
