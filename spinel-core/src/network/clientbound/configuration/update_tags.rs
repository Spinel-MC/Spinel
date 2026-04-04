use ::spinel_macros::packet;
use ::spinel_network::DataType;
use ::spinel_network::types::Array;
use ::spinel_network::types::var_int::VarIntWrapper;

#[derive(Debug, Clone)]
pub struct Tag {
    pub tag_name: String,
    pub entries: Array<VarIntWrapper>,
}

impl DataType for Tag {
    fn encode<W: ::std::io::Write>(&self, w: &mut W) -> ::std::io::Result<()> {
        <String as DataType>::encode(&self.tag_name, w)?;
        <Array<VarIntWrapper> as DataType>::encode(&self.entries, w)
    }
    fn decode<R: ::std::io::Read>(r: &mut R) -> ::std::io::Result<Self> {
        Ok(Self {
            tag_name: <String as DataType>::decode(r)?,
            entries: <Array<VarIntWrapper> as DataType>::decode(r)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct TagRegistry {
    pub registry_name: String,
    pub tags: Array<Tag>,
}

impl DataType for TagRegistry {
    fn encode<W: ::std::io::Write>(&self, w: &mut W) -> ::std::io::Result<()> {
        <String as DataType>::encode(&self.registry_name, w)?;
        <Array<Tag> as DataType>::encode(&self.tags, w)
    }
    fn decode<R: ::std::io::Read>(r: &mut R) -> ::std::io::Result<Self> {
        Ok(Self {
            registry_name: <String as DataType>::decode(r)?,
            tags: <Array<Tag> as DataType>::decode(r)?,
        })
    }
}

#[packet(
    id: "update_tags",
    state: ConnectionState::Configuration,
    recipient: Recipient::Client
)]
pub struct UpdateTagsPacket {
    pub registries: Array<TagRegistry>,
}

impl UpdateTagsPacket {
    pub fn vanilla_tags() -> Self {
        let registries = vec![
            TagRegistry {
                registry_name: "minecraft:worldgen/biome".to_string(),
                tags: Array(vec![
                    Tag {
                        tag_name: "minecraft:is_overworld".to_string(),
                        entries: Array(
                            (0..65)
                                .filter(|&i| ![7, 16, 17, 18, 34, 44, 49, 56, 57, 59].contains(&i))
                                .map(|i| VarIntWrapper(i as i32))
                                .collect(),
                        ),
                    },
                    Tag {
                        tag_name: "minecraft:is_nether".to_string(),
                        entries: Array(vec![
                            VarIntWrapper(2),
                            VarIntWrapper(7),
                            VarIntWrapper(34),
                            VarIntWrapper(49),
                            VarIntWrapper(59),
                        ]),
                    },
                    Tag {
                        tag_name: "minecraft:is_end".to_string(),
                        entries: Array(vec![
                            VarIntWrapper(16),
                            VarIntWrapper(17),
                            VarIntWrapper(18),
                            VarIntWrapper(44),
                            VarIntWrapper(56),
                        ]),
                    },
                ]),
            },
            TagRegistry {
                registry_name: "minecraft:dialog".to_string(),
                tags: Array(vec![
                    Tag {
                        tag_name: "minecraft:pause_screen_additions".to_string(),
                        entries: Array(vec![]),
                    },
                    Tag {
                        tag_name: "minecraft:quick_actions".to_string(),
                        entries: Array(vec![]),
                    },
                ]),
            },
            TagRegistry {
                registry_name: "minecraft:timeline".to_string(),
                tags: Array(vec![
                    Tag {
                        tag_name: "minecraft:in_overworld".to_string(),
                        entries: Array(vec![
                            VarIntWrapper(0),
                            VarIntWrapper(1),
                            VarIntWrapper(2),
                            VarIntWrapper(3),
                        ]),
                    },
                    Tag {
                        tag_name: "minecraft:in_nether".to_string(),
                        entries: Array(vec![]),
                    },
                    Tag {
                        tag_name: "minecraft:in_end".to_string(),
                        entries: Array(vec![]),
                    },
                    Tag {
                        tag_name: "minecraft:universal".to_string(),
                        entries: Array(vec![]),
                    },
                ]),
            },
        ];

        Self {
            registries: Array(registries),
        }
    }
}
