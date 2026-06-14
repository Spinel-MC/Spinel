use spinel_network::{DataType, VarIntWrapper};
use std::io::{self, Read, Write};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourcePackInfo {
    id: Uuid,
    url: String,
    hash: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourcePackStatus {
    SuccessfullyLoaded,
    Declined,
    FailedDownload,
    Accepted,
    Downloaded,
    InvalidUrl,
    FailedReload,
    Discarded,
}

impl ResourcePackInfo {
    pub fn new(id: Uuid, url: impl Into<String>, hash: impl Into<String>) -> Self {
        Self {
            id,
            url: url.into(),
            hash: hash.into(),
        }
    }

    pub const fn id(&self) -> Uuid {
        self.id
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }
}

impl ResourcePackStatus {
    pub const fn id(self) -> i32 {
        match self {
            Self::SuccessfullyLoaded => 0,
            Self::Declined => 1,
            Self::FailedDownload => 2,
            Self::Accepted => 3,
            Self::Downloaded => 4,
            Self::InvalidUrl => 5,
            Self::FailedReload => 6,
            Self::Discarded => 7,
        }
    }

    pub const fn from_id(id: i32) -> Option<Self> {
        match id {
            0 => Some(Self::SuccessfullyLoaded),
            1 => Some(Self::Declined),
            2 => Some(Self::FailedDownload),
            3 => Some(Self::Accepted),
            4 => Some(Self::Downloaded),
            5 => Some(Self::InvalidUrl),
            6 => Some(Self::FailedReload),
            7 => Some(Self::Discarded),
            _ => None,
        }
    }
}

impl DataType for ResourcePackStatus {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.id()).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let status_id = VarIntWrapper::decode(reader)?.0;
        Self::from_id(status_id).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unexpected resource pack status: {status_id}"),
            )
        })
    }
}
