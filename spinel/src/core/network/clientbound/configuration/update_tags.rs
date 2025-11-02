use crate as spinel;
use spinel::network::{encoder::NetworkBuffer, Client};

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
                registry_name: "minecraft:block".to_string(),
                tags: vec![
                    Tag { tag_name: "minecraft:mob_interactable_doors".to_string(), entries: vec![207, 614, 615, 616, 617, 619, 620, 858, 859, 621, 622, 618, 1008, 1009, 1011, 1010, 1012, 1013, 1015, 1014] },
                    Tag { tag_name: "minecraft:campfires".to_string(), entries: vec![818, 819] },
                    Tag { tag_name: "minecraft:soul_fire_base_blocks".to_string(), entries: vec![273, 274] },
                    Tag { tag_name: "minecraft:infiniburn_nether".to_string(), entries: vec![272, 639] },
                    Tag { tag_name: "minecraft:wooden_slabs".to_string(), entries: vec![567, 568, 569, 570, 571, 573, 574, 844, 845, 575, 576, 572] },
                    Tag { tag_name: "minecraft:snaps_goat_horn".to_string(), entries: vec![53, 51, 49, 52, 50, 55, 56, 57, 54, 1, 524, 44, 46, 970, 367] },
                    Tag { tag_name: "minecraft:coal_ores".to_string(), entries: vec![46, 47] },
                    Tag { tag_name: "minecraft:occludes_vibration_signals".to_string(), entries: (140..=155).collect() },
                    Tag { tag_name: "minecraft:wool".to_string(), entries: (140..=155).collect() },
                ],
            },
            TagRegistry {
                registry_name: "minecraft:worldgen/biome".to_string(),
                tags: vec![
                    Tag { tag_name: "minecraft:is_overworld".to_string(), entries: (0..=65).filter(|&i| ![7, 16, 17, 18, 34, 44, 49, 56, 57, 59].contains(&i)).collect() },
                    Tag { tag_name: "minecraft:is_nether".to_string(), entries: vec![2, 7, 34, 49, 59] },
                    Tag { tag_name: "minecraft:is_end".to_string(), entries: vec![16, 17, 18, 44, 56] },
                ],
            },
            TagRegistry {
                registry_name: "minecraft:enchantment".to_string(),
                tags: vec![
                    Tag { tag_name: "minecraft:tooltip_order".to_string(), entries: (0..=41).collect() },
                    Tag { tag_name: "minecraft:double_trade_price".to_string(), entries: vec![2, 40, 37, 35, 14, 22, 41] },
                    Tag { tag_name: "minecraft:treasure".to_string(), entries: vec![2, 40, 37, 35, 14, 22, 41] },
                ],
            },
            TagRegistry {
                registry_name: "minecraft:damage_type".to_string(),
                tags: vec![
                    Tag { tag_name: "minecraft:bypasses_armor".to_string(), entries: vec![2, 4, 5, 6, 7, 8, 10, 16, 17, 18, 19, 22, 23, 27, 31, 32, 33, 36, 38, 39, 47] },
                    Tag { tag_name: "minecraft:is_fire".to_string(), entries: vec![3, 14, 20, 21, 24, 31, 45] },
                ],
            },
             TagRegistry {
                registry_name: "minecraft:banner_pattern".to_string(),
                tags: vec![
                    Tag { tag_name: "minecraft:no_item_required".to_string(), entries: (1..=42).filter(|&i| ![22, 2, 12, 11, 4, 13, 6, 16, 21, 24].contains(&i)).collect() },
                ],
            },
             TagRegistry {
                registry_name: "minecraft:point_of_interest_type".to_string(),
                tags: vec![
                    Tag { tag_name: "minecraft:acquirable_job_site".to_string(), entries: (0..=12).collect() },
                ],
            },
            TagRegistry {
                registry_name: "minecraft:entity_type".to_string(),
                tags: vec![
                    Tag { tag_name: "minecraft:skeletons".to_string(), entries: vec![16, 109, 110, 122, 140] },
                ],
            },
            TagRegistry {
                registry_name: "minecraft:instrument".to_string(),
                tags: vec![
                    Tag { tag_name: "minecraft:goat_horns".to_string(), entries: (0..=7).collect() },
                ],
            },
            TagRegistry {
                registry_name: "minecraft:game_event".to_string(),
                tags: vec![
                    Tag { tag_name: "minecraft:vibrations".to_string(), entries: (0..=59).collect() },
                ],
            },
            TagRegistry {
                registry_name: "minecraft:fluid".to_string(),
                tags: vec![
                    Tag { tag_name: "minecraft:lava".to_string(), entries: vec![3, 4] },
                    Tag { tag_name: "minecraft:water".to_string(), entries: vec![1, 2] },
                ],
            },
            // Declare other necessary registries even if their tag lists are empty
            TagRegistry { registry_name: "minecraft:item".to_string(), tags: vec![] },
            TagRegistry { registry_name: "minecraft:dialog".to_string(), tags: vec![] },
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