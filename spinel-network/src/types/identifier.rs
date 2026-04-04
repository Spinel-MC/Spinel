use crate::data_type::DataType;
use std::fmt;
use std::io::{self, Read, Write};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub namespace: String,
    pub path: String,
}

impl Identifier {
    pub fn new(namespace: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            path: path.into(),
        }
    }

    pub fn minecraft(path: impl Into<String>) -> Self {
        Self::new("minecraft", path)
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Self::minecraft("air")
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

impl FromStr for Identifier {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        match parts.len() {
            1 => Ok(Identifier::minecraft(parts[0])),
            2 => Ok(Identifier::new(parts[0], parts[1])),
            _ => Err(format!("Invalid identifier: {}", s)),
        }
    }
}

impl DataType for Identifier {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.to_string().encode(w)
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let s = String::decode(r)?;
        s.parse()
            .map_err(|e: String| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}
