use spinel_network::data_type::DataType;
use spinel_network::encoder::NetworkBuffer;
use spinel_network::types::var_int::VarIntWrapper;
use spinel_network::wrappers::JsonTextComponent;
use spinel_network::{ConnectionState, PacketSender, PacketStruct};
use spinel_registry::{ItemStack, Material};
use spinel_utils::component::Component;
use spinel_utils::component::text::TextComponent;
use std::io::{self, Read, Write};

const NOTIFICATION_IDENTIFIER: &str = "minestom:notification";
const NOTIFICATION_CRITERION: &str = "minestom:some_criteria";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdvancementFrameType {
    Task,
    Challenge,
    Goal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
    pub title: TextComponent,
    pub frame_type: AdvancementFrameType,
    pub icon: ItemStack,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AdvancementsPacket {
    pub reset: bool,
    pub advancement_mappings: Vec<AdvancementMapping>,
    pub identifiers_to_remove: Vec<String>,
    pub progress_mappings: Vec<ProgressMapping>,
    pub show_advancements: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AdvancementMapping {
    pub key: String,
    pub advancement: Advancement,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Advancement {
    pub parent_identifier: Option<String>,
    pub display_data: Option<AdvancementDisplayData>,
    pub requirements: Vec<AdvancementRequirement>,
    pub send_telemetry_data: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AdvancementDisplayData {
    pub title: TextComponent,
    pub description: TextComponent,
    pub icon: ItemStack,
    pub frame_type: AdvancementFrameType,
    pub flags: i32,
    pub background_texture: Option<String>,
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdvancementRequirement {
    pub requirements: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgressMapping {
    pub key: String,
    pub progress: AdvancementProgress,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AdvancementProgress {
    pub criteria: Vec<AdvancementCriteria>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AdvancementCriteria {
    pub criterion_identifier: String,
    pub progress: CriterionProgress,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CriterionProgress {
    pub date_of_achieving: Option<i64>,
}

impl Notification {
    pub fn new(title: TextComponent, frame_type: AdvancementFrameType, icon: ItemStack) -> Self {
        Self {
            title,
            frame_type,
            icon,
        }
    }

    pub fn from_material(
        title: TextComponent,
        frame_type: AdvancementFrameType,
        material: Material,
    ) -> Self {
        Self::new(title, frame_type, ItemStack::of(material))
    }

    pub fn add_packet(&self, timestamp_millis: i64) -> AdvancementsPacket {
        let criterion = AdvancementCriteria {
            criterion_identifier: NOTIFICATION_CRITERION.to_string(),
            progress: CriterionProgress {
                date_of_achieving: Some(timestamp_millis),
            },
        };

        AdvancementsPacket {
            reset: false,
            advancement_mappings: vec![AdvancementMapping {
                key: NOTIFICATION_IDENTIFIER.to_string(),
                advancement: Advancement {
                    parent_identifier: None,
                    display_data: Some(AdvancementDisplayData {
                        title: self.title.clone(),
                        description: Component::text("Articdive was here. #Minestom").build(),
                        icon: self.icon.clone(),
                        frame_type: self.frame_type,
                        flags: 0x6,
                        background_texture: None,
                        x: 0.0,
                        y: 0.0,
                    }),
                    requirements: vec![AdvancementRequirement {
                        requirements: vec![criterion.criterion_identifier.clone()],
                    }],
                    send_telemetry_data: false,
                },
            }],
            identifiers_to_remove: Vec::new(),
            progress_mappings: vec![ProgressMapping {
                key: NOTIFICATION_IDENTIFIER.to_string(),
                progress: AdvancementProgress {
                    criteria: vec![criterion],
                },
            }],
            show_advancements: true,
        }
    }

    pub fn remove_packet(&self) -> AdvancementsPacket {
        AdvancementsPacket {
            reset: false,
            advancement_mappings: Vec::new(),
            identifiers_to_remove: vec![NOTIFICATION_IDENTIFIER.to_string()],
            progress_mappings: Vec::new(),
            show_advancements: true,
        }
    }
}

impl AdvancementsPacket {
    pub const fn get_id() -> i32 {
        0x80
    }

    pub fn encode_to_buffer(&self) -> io::Result<NetworkBuffer> {
        let mut buffer = NetworkBuffer::new();
        self.encode(&mut buffer)?;
        Ok(buffer)
    }

    pub fn dispatch<S: PacketSender>(self, sender: &mut S) -> io::Result<()> {
        let payload_bytes = self.encode_to_buffer()?.into_buffer();
        sender.send_packet(Self::get_id(), &payload_bytes)
    }
}

impl AdvancementFrameType {
    const fn protocol_id(self) -> i32 {
        match self {
            Self::Task => 0,
            Self::Challenge => 1,
            Self::Goal => 2,
        }
    }

    const fn from_protocol_id(protocol_id: i32) -> Option<Self> {
        match protocol_id {
            0 => Some(Self::Task),
            1 => Some(Self::Challenge),
            2 => Some(Self::Goal),
            _ => None,
        }
    }
}

impl DataType for AdvancementFrameType {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.protocol_id()).encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Self::from_protocol_id(VarIntWrapper::decode(reader)?.0)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "unknown frame type"))
    }
}

impl DataType for AdvancementsPacket {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.reset.encode(writer)?;
        self.advancement_mappings.encode(writer)?;
        self.identifiers_to_remove.encode(writer)?;
        self.progress_mappings.encode(writer)?;
        self.show_advancements.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            reset: bool::decode(reader)?,
            advancement_mappings: Vec::<AdvancementMapping>::decode(reader)?,
            identifiers_to_remove: Vec::<String>::decode(reader)?,
            progress_mappings: Vec::<ProgressMapping>::decode(reader)?,
            show_advancements: bool::decode(reader)?,
        })
    }
}

impl DataType for AdvancementMapping {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.key.encode(writer)?;
        self.advancement.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            key: String::decode(reader)?,
            advancement: Advancement::decode(reader)?,
        })
    }
}

impl DataType for Advancement {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.parent_identifier.encode(writer)?;
        self.display_data.encode(writer)?;
        self.requirements.encode(writer)?;
        self.send_telemetry_data.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            parent_identifier: Option::<String>::decode(reader)?,
            display_data: Option::<AdvancementDisplayData>::decode(reader)?,
            requirements: Vec::<AdvancementRequirement>::decode(reader)?,
            send_telemetry_data: bool::decode(reader)?,
        })
    }
}

impl DataType for AdvancementDisplayData {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        JsonTextComponent(self.title.clone()).encode(writer)?;
        JsonTextComponent(self.description.clone()).encode(writer)?;
        Slot::from_item_stack(&self.icon).encode(writer)?;
        self.frame_type.encode(writer)?;
        self.flags.encode(writer)?;
        if self.flags & 0x1 != 0 {
            self.background_texture
                .clone()
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "background texture is required",
                    )
                })?
                .encode(writer)?;
        }
        self.x.encode(writer)?;
        self.y.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let title = JsonTextComponent::decode(reader)?.0;
        let description = JsonTextComponent::decode(reader)?.0;
        let icon = Slot::decode(reader)?.to_item_stack();
        let frame_type = AdvancementFrameType::decode(reader)?;
        let flags = i32::decode(reader)?;
        let background_texture = match flags & 0x1 != 0 {
            true => Some(String::decode(reader)?),
            false => None,
        };
        Ok(Self {
            title,
            description,
            icon,
            frame_type,
            flags,
            background_texture,
            x: f32::decode(reader)?,
            y: f32::decode(reader)?,
        })
    }
}

impl DataType for AdvancementRequirement {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.requirements.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            requirements: Vec::<String>::decode(reader)?,
        })
    }
}

impl DataType for ProgressMapping {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.key.encode(writer)?;
        self.progress.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            key: String::decode(reader)?,
            progress: AdvancementProgress::decode(reader)?,
        })
    }
}

impl DataType for AdvancementProgress {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.criteria.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            criteria: Vec::<AdvancementCriteria>::decode(reader)?,
        })
    }
}

impl DataType for AdvancementCriteria {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.criterion_identifier.encode(writer)?;
        self.progress.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            criterion_identifier: String::decode(reader)?,
            progress: CriterionProgress::decode(reader)?,
        })
    }
}

impl DataType for CriterionProgress {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.date_of_achieving.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            date_of_achieving: Option::<i64>::decode(reader)?,
        })
    }
}

impl PacketStruct for AdvancementsPacket {
    fn get_id() -> i32 {
        Self::get_id()
    }

    fn get_state() -> ConnectionState {
        ConnectionState::Play
    }
}

use spinel_network::types::Slot;

#[cfg(test)]
mod tests {
    use super::{AdvancementFrameType, AdvancementsPacket, Notification};
    use spinel_network::DataType;
    use spinel_registry::Material;
    use spinel_utils::component::Component;

    #[test]
    fn notification_builds_minestom_advancement_add_and_remove_packets() {
        let notification = Notification::from_material(
            Component::text("Done").build(),
            AdvancementFrameType::Challenge,
            Material::DIAMOND,
        );
        let add_packet = notification.add_packet(42);
        let remove_packet = notification.remove_packet();
        let mut payload = Vec::new();

        add_packet.encode(&mut payload).unwrap();
        let decoded_packet = AdvancementsPacket::decode(&mut payload.as_slice()).unwrap();

        assert_eq!(AdvancementsPacket::get_id(), 0x80);
        assert_eq!(
            decoded_packet.advancement_mappings[0].key,
            "minestom:notification"
        );
        assert_eq!(
            decoded_packet.progress_mappings[0].progress.criteria[0]
                .progress
                .date_of_achieving,
            Some(42)
        );
        assert_eq!(
            remove_packet.identifiers_to_remove,
            vec!["minestom:notification"]
        );
    }
}
