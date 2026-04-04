use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
use std::io::{self, Read, Write};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GameProfile {
    pub uuid: Uuid,
    pub username: String,
    pub properties: Vec<GameProfileProperty>,
}

#[derive(Debug, Clone)]
pub struct GameProfileProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

impl DataType for GameProfile {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
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

        let count = VarIntWrapper::decode(r)?.0 as usize;
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
