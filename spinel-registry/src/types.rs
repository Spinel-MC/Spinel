use std::{
    borrow::Cow,
    fmt::{self, Display},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

/// An identifier used by Minecraft.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    /// The namespace of the identifier.
    pub namespace: Cow<'static, str>,
    /// The path of the identifier.
    pub path: Cow<'static, str>,
}

impl Identifier {
    /// The vanilla namespace.
    pub const VANILLA_NAMESPACE: &'static str = "minecraft";

    /// Creates a new `Identifier` with the vanilla namespace.
    #[must_use]
    pub fn vanilla(path: String) -> Self {
        Identifier {
            namespace: Cow::Borrowed(Self::VANILLA_NAMESPACE),
            path: Cow::Owned(path),
        }
    }

    /// Creates a new `Identifier` with the vanilla namespace and a static path.
    #[must_use]
    pub const fn vanilla_static(path: &'static str) -> Self {
        Identifier {
            namespace: Cow::Borrowed(Self::VANILLA_NAMESPACE),
            path: Cow::Borrowed(path),
        }
    }

    /// Returns whether the character is a valid namespace character.
    #[must_use]
    pub fn valid_namespace_char(char: char) -> bool {
        char == '_'
            || char == '-'
            || char.is_ascii_lowercase()
            || char.is_ascii_digit()
            || char == '.'
    }

    /// Returns whether the character is a valid path character.
    #[must_use]
    pub fn valid_char(char: char) -> bool {
        Self::valid_namespace_char(char) || char == '/'
    }

    /// Returns whether the namespace is valid.
    pub fn validate_namespace(namespace: &str) -> bool {
        namespace.chars().all(Self::valid_namespace_char)
    }

    /// Returns whether the path is valid.
    pub fn validate_path(path: &str) -> bool {
        path.chars().all(Self::valid_char)
    }

    /// Returns whether the namespace and path are valid.
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

impl FromStr for Identifier {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid resource location: {s}"));
        }

        if !Identifier::validate_namespace(parts[0]) {
            return Err(format!("Invalid namespace: {}", parts[0]));
        }

        if !Identifier::validate_path(parts[1]) {
            return Err(format!("Invalid path: {}", parts[1]));
        }

        Ok(Identifier {
            namespace: Cow::Owned(parts[0].to_string()),
            path: Cow::Owned(parts[1].to_string()),
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

/// A raw block state id. Using the registry this id can be derived into a block and its current properties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct BlockStateId(pub u16);

/// An axis in 3D space.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Axis {
    /// X axis
    X,
    /// Y axis
    Y,
    /// Z axis
    Z,
}

impl Axis {
    /// Returns the axis as a string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Axis::X => "x",
            Axis::Y => "y",
            Axis::Z => "z",
        }
    }
}

/// Placeholder type for unimplemented data component values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Todo;

