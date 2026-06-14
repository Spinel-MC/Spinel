use crate::data_type::DataType;
use crate::types::component_changes::ComponentChanges;
use crate::types::slot::Slot;
use crate::types::var_int::VarIntWrapper;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UntrustedSlot(pub Slot);

impl UntrustedSlot {
    pub fn to_item_stack(&self) -> spinel_registry::ItemStack {
        self.0.to_item_stack()
    }
}

impl DataType for UntrustedSlot {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        if self.0.is_empty() {
            return VarIntWrapper(0).encode(writer);
        }

        VarIntWrapper(self.0.count).encode(writer)?;
        VarIntWrapper(self.0.item_id).encode(writer)?;
        self.0.components.encode_length_prefixed(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let count = VarIntWrapper::decode(reader)?.0;
        if count <= 0 {
            return Ok(Self(Slot::default()));
        }

        let item_id = VarIntWrapper::decode(reader)?.0;
        let components = ComponentChanges::decode_length_prefixed(reader)?;

        Ok(Self(Slot {
            count,
            item_id,
            components,
        }))
    }
}
