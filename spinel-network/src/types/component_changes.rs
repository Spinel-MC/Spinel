use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
use spinel_nbt::Nbt;
use spinel_registry::DataComponentMap;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ComponentChanges {
    pub added: Vec<ComponentEntry>,
    pub removed: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentEntry {
    pub type_id: i32,
    pub data: Vec<u8>,
}

impl DataType for ComponentChanges {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        VarIntWrapper(self.added.len() as i32).encode(w)?;
        VarIntWrapper(self.removed.len() as i32).encode(w)?;

        for entry in &self.added {
            VarIntWrapper(entry.type_id).encode(w)?;
            w.write_all(&entry.data)?;
        }

        for type_id in &self.removed {
            VarIntWrapper(*type_id).encode(w)?;
        }

        Ok(())
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let added_count = VarIntWrapper::decode(r)?.0 as usize;
        let removed_count = VarIntWrapper::decode(r)?.0 as usize;

        if added_count > 0 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Decoding non-empty ComponentChanges is not fully supported without a component registry.",
            ));
        }

        let mut removed = Vec::with_capacity(removed_count);
        for _ in 0..removed_count {
            removed.push(VarIntWrapper::decode(r)?.0);
        }

        Ok(ComponentChanges {
            added: vec![],
            removed,
        })
    }
}

impl From<&DataComponentMap> for ComponentChanges {
    fn from(component_patch: &DataComponentMap) -> Self {
        Self {
            added: component_patch
                .entries()
                .into_iter()
                .filter_map(|component| {
                    encode_component_nbt(&component.component_nbt).map(|data| ComponentEntry {
                        type_id: component.component_id,
                        data,
                    })
                })
                .collect(),
            removed: component_patch.removed_component_ids(),
        }
    }
}

fn encode_component_nbt(component_nbt: &Nbt) -> Option<Vec<u8>> {
    let mut data = Vec::new();
    match component_nbt {
        Nbt::Int(value) => VarIntWrapper(*value).encode(&mut data).ok()?,
        Nbt::Byte(value) => data.write_all(&value.to_be_bytes()).ok()?,
        Nbt::String(value) => value.encode(&mut data).ok()?,
        Nbt::List(values) => {
            VarIntWrapper(values.len() as i32).encode(&mut data).ok()?;
            for value in values {
                let Nbt::String(value) = value else {
                    return None;
                };
                value.encode(&mut data).ok()?;
            }
        }
        Nbt::Compound(_) => return None,
        _ => return None,
    }
    Some(data)
}
