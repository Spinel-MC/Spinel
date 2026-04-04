use crate::data_type::DataType;
use crate::types::component_changes::ComponentChanges;
use crate::types::var_int::VarIntWrapper;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Slot {
    pub count: i32,
    pub item_id: i32,
    pub components: ComponentChanges,
}

impl Slot {
    pub fn is_empty(&self) -> bool {
        self.count <= 0
    }
}

impl DataType for Slot {
    fn encode<W: Write>(&self, w: &mut W) -> io::Result<()> {
        if self.is_empty() {
            VarIntWrapper(0).encode(w)
        } else {
            VarIntWrapper(self.count).encode(w)?;
            VarIntWrapper(self.item_id).encode(w)?;
            self.components.encode(w)
        }
    }

    fn decode<R: Read>(r: &mut R) -> io::Result<Self> {
        let count = VarIntWrapper::decode(r)?.0;
        if count <= 0 {
            return Ok(Slot {
                count: 0,
                item_id: 0,
                components: ComponentChanges::default(),
            });
        }

        let item_id = VarIntWrapper::decode(r)?.0;
        let components = ComponentChanges::decode(r)?;

        Ok(Slot {
            count,
            item_id,
            components,
        })
    }
}
