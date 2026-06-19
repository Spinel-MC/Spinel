use crate::data_type::DataType;
use crate::types::component_changes::{ComponentChanges, ComponentEntry};
use std::io::{self, Read, Write};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExactDataComponentPredicate {
    pub expected_components: Vec<ComponentEntry>,
}

impl DataType for ExactDataComponentPredicate {
    fn encode<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        ComponentChanges {
            added: self.expected_components.clone(),
            removed: Vec::new(),
        }
        .encode_length_prefixed(writer)
    }

    fn decode<R: Read>(reader: &mut R) -> io::Result<Self> {
        let component_changes = ComponentChanges::decode(reader)?;
        if !component_changes.removed.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "exact data component predicate cannot contain removed components",
            ));
        }

        Ok(Self {
            expected_components: component_changes.added,
        })
    }
}
