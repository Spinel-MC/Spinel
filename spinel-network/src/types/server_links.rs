use crate::data_type::DataType;
use crate::wrappers::JsonTextComponent;
use spinel_utils::component::text::TextComponent;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub enum ServerLinkLabel {
    Known(ServerLinkType),
    Custom(TextComponent),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerLinkType {
    BugReport,
    CommunityGuidelines,
    Support,
    Status,
    Feedback,
    Community,
    Website,
    Forums,
    News,
    Announcements,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ServerLinkEntry {
    pub label: ServerLinkLabel,
    pub link: String,
}

impl ServerLinkType {
    fn from_protocol_id(protocol_id: i32) -> io::Result<Self> {
        match protocol_id {
            0 => Ok(Self::BugReport),
            1 => Ok(Self::CommunityGuidelines),
            2 => Ok(Self::Support),
            3 => Ok(Self::Status),
            4 => Ok(Self::Feedback),
            5 => Ok(Self::Community),
            6 => Ok(Self::Website),
            7 => Ok(Self::Forums),
            8 => Ok(Self::News),
            9 => Ok(Self::Announcements),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unknown server link type protocol id {protocol_id}"),
            )),
        }
    }

    const fn protocol_id(self) -> i32 {
        match self {
            Self::BugReport => 0,
            Self::CommunityGuidelines => 1,
            Self::Support => 2,
            Self::Status => 3,
            Self::Feedback => 4,
            Self::Community => 5,
            Self::Website => 6,
            Self::Forums => 7,
            Self::News => 8,
            Self::Announcements => 9,
        }
    }
}

impl DataType for ServerLinkType {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.protocol_id().encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Self::from_protocol_id(i32::decode(reader)?)
    }
}

impl DataType for ServerLinkLabel {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            Self::Known(link_type) => {
                true.encode(writer)?;
                link_type.encode(writer)
            }
            Self::Custom(label) => {
                false.encode(writer)?;
                JsonTextComponent(label.clone()).encode(writer)
            }
        }
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        if bool::decode(reader)? {
            return ServerLinkType::decode(reader).map(Self::Known);
        }

        JsonTextComponent::decode(reader).map(|label| Self::Custom(label.0))
    }
}

impl DataType for ServerLinkEntry {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.label.encode(writer)?;
        self.link.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            label: ServerLinkLabel::decode(reader)?,
            link: String::decode(reader)?,
        })
    }
}
