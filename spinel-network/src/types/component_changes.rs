use crate::data_type::DataType;
use crate::types::var_int::VarIntWrapper;
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
