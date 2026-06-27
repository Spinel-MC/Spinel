use std::{
    borrow::Cow,
    fmt::{self, Display},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier {
    pub namespace: Cow<'static, str>,
    pub path: Cow<'static, str>,
}

impl Identifier {
    pub const VANILLA_NAMESPACE: &'static str = "minecraft";

    #[must_use]
    pub fn new(namespace: impl Into<String>, path: impl Into<String>) -> Self {
        Identifier {
            namespace: Cow::Owned(namespace.into()),
            path: Cow::Owned(path.into()),
        }
    }

    #[must_use]
    pub fn minecraft(path: impl Into<String>) -> Self {
        Self::new(Self::VANILLA_NAMESPACE, path)
    }

    #[must_use]
    pub fn vanilla(path: String) -> Self {
        Identifier {
            namespace: Cow::Borrowed(Self::VANILLA_NAMESPACE),
            path: Cow::Owned(path),
        }
    }

    #[must_use]
    pub const fn vanilla_static(path: &'static str) -> Self {
        Identifier {
            namespace: Cow::Borrowed(Self::VANILLA_NAMESPACE),
            path: Cow::Borrowed(path),
        }
    }

    #[must_use]
    pub fn valid_namespace_char(char: char) -> bool {
        char == '_'
            || char == '-'
            || char.is_ascii_lowercase()
            || char.is_ascii_digit()
            || char == '.'
    }

    #[must_use]
    pub fn valid_char(char: char) -> bool {
        Self::valid_namespace_char(char) || char == '/'
    }

    pub fn validate_namespace(namespace: &str) -> bool {
        namespace.chars().all(Self::valid_namespace_char)
    }

    pub fn validate_path(path: &str) -> bool {
        path.chars().all(Self::valid_char)
    }

    #[must_use]
    pub fn validate(namespace: &str, path: &str) -> bool {
        Self::validate_namespace(namespace) && Self::validate_path(path)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Self::minecraft("air")
    }
}

impl FromStr for Identifier {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        let (namespace, path) = match parts.as_slice() {
            [path] => (Self::VANILLA_NAMESPACE, *path),
            [namespace, path] => (*namespace, *path),
            _ => return Err(format!("Invalid resource location: {s}")),
        };

        if !Identifier::validate_namespace(namespace) {
            return Err(format!("Invalid namespace: {namespace}"));
        }

        if !Identifier::validate_path(path) {
            return Err(format!("Invalid path: {path}"));
        }

        Ok(Identifier {
            namespace: Cow::Owned(namespace.to_string()),
            path: Cow::Owned(path.to_string()),
        })
    }
}

impl Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Identifier::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BlockStateId(pub u16);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Axis::X => "x",
            Axis::Y => "y",
            Axis::Z => "z",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Todo;
