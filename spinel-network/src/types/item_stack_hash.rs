use crate::data_type::DataType;
use crate::types::ComponentChanges;
use crate::types::var_int::VarIntWrapper;
use spinel_registry::ItemStack;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemStackHash {
    Air,
    Item {
        item_id: i32,
        amount: i32,
        added_components: Vec<ItemStackHashComponent>,
        removed_components: Vec<i32>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemStackHashComponent {
    pub component_id: i32,
    pub hash: i32,
}

impl DataType for ItemStackHash {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            Self::Air => false.encode(writer),
            Self::Item {
                item_id,
                amount,
                added_components,
                removed_components,
            } => {
                true.encode(writer)?;
                VarIntWrapper(*item_id).encode(writer)?;
                VarIntWrapper(*amount).encode(writer)?;
                added_components.encode(writer)?;
                encode_component_ids(writer, removed_components)
            }
        }
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        if !bool::decode(reader)? {
            return Ok(Self::Air);
        }
        Ok(Self::Item {
            item_id: VarIntWrapper::decode(reader)?.0,
            amount: VarIntWrapper::decode(reader)?.0,
            added_components: Vec::<ItemStackHashComponent>::decode(reader)?,
            removed_components: decode_component_ids(reader)?,
        })
    }
}

fn encode_component_ids<W: Write>(writer: &mut W, component_ids: &[i32]) -> io::Result<()> {
    VarIntWrapper(component_ids.len() as i32).encode(writer)?;
    component_ids
        .iter()
        .try_for_each(|component_id| VarIntWrapper(*component_id).encode(writer))
}

fn decode_component_ids<R: Read>(reader: &mut R) -> io::Result<Vec<i32>> {
    let component_count = VarIntWrapper::decode(reader)?.0 as usize;
    (0..component_count)
        .map(|_| VarIntWrapper::decode(reader).map(|component_id| component_id.0))
        .collect()
}

impl DataType for ItemStackHashComponent {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        VarIntWrapper(self.component_id).encode(writer)?;
        self.hash.encode(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        Ok(Self {
            component_id: VarIntWrapper::decode(reader)?.0,
            hash: i32::decode(reader)?,
        })
    }
}

impl ItemStackHash {
    pub fn from_item_stack(item_stack: &ItemStack) -> Self {
        if item_stack.is_air() {
            return Self::Air;
        }
        let component_changes = ComponentChanges::from(item_stack.component_patch());
        Self::Item {
            item_id: item_stack.material().id(),
            amount: item_stack.amount(),
            added_components: component_changes
                .added
                .into_iter()
                .map(|component_entry| ItemStackHashComponent {
                    component_id: component_entry.type_id,
                    hash: crc32_hash(&component_entry.data),
                })
                .collect(),
            removed_components: component_changes.removed,
        }
    }
}

fn crc32_hash(data: &[u8]) -> i32 {
    let crc = data.iter().fold(0xffff_ffffu32, |crc, byte| {
        let crc = crc ^ u32::from(*byte);
        (0..8).fold(crc, |crc, _| {
            if crc & 1 == 1 {
                (crc >> 1) ^ 0xedb8_8320
            } else {
                crc >> 1
            }
        })
    });
    (!crc) as i32
}
