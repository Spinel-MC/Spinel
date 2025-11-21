use crate as spinel;
use spinel::network::{Client, encoder::NetworkBuffer};

#[derive(Debug, Clone)]
pub struct Tag {
    pub tag_name: String,
    pub entries: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct TagRegistry {
    pub registry_name: String,
    pub tags: Vec<Tag>,
}

pub struct UpdateTagsPacket {
    pub registries: Vec<TagRegistry>,
}

impl UpdateTagsPacket {
    pub fn get_id() -> i32 {
        0x0D
    }

    pub fn vanilla_tags() -> Self {
        let registries = vec![
            TagRegistry {
                registry_name: "minecraft:worldgen/biome".to_string(),
                tags: vec![
                    Tag {
                        tag_name: "minecraft:is_overworld".to_string(),
                        entries: (0..65)
                            .filter(|&i| ![7, 16, 17, 18, 34, 44, 49, 56, 57, 59].contains(&i))
                            .collect(),
                    },
                    Tag {
                        tag_name: "minecraft:is_nether".to_string(),
                        entries: vec![2, 7, 34, 49, 59],
                    },
                    Tag {
                        tag_name: "minecraft:is_end".to_string(),
                        entries: vec![16, 17, 18, 44, 56],
                    },
                ],
            },
            TagRegistry {
                registry_name: "minecraft:dialog".to_string(),
                tags: vec![
                    Tag {
                        tag_name: "minecraft:pause_screen_additions".to_string(),
                        entries: vec![],
                    },
                    Tag {
                        tag_name: "minecraft:quick_actions".to_string(),
                        entries: vec![],
                    },
                ],
            },
        ];

        Self { registries }
    }

    pub fn encode(&self) -> NetworkBuffer {
        let mut buffer = NetworkBuffer::new();
        buffer.write_varint(self.registries.len() as i32);
        for registry in &self.registries {
            buffer.write_string(&registry.registry_name);
            buffer.write_varint(registry.tags.len() as i32);
            for tag in &registry.tags {
                buffer.write_string(&tag.tag_name);
                buffer.write_varint(tag.entries.len() as i32);
                for &entry_id in &tag.entries {
                    buffer.write_varint(entry_id);
                }
            }
        }
        buffer
    }

    pub fn dispatch(self, client: &mut Client) {
        let packet_id = Self::get_id();
        let payload_bytes = self.encode().into_buffer();
        client.send_packet(packet_id, &payload_bytes);
    }
}
